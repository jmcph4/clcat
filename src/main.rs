use clap::Parser;
use libp2p::futures::StreamExt;
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::OrTransport, upgrade},
    futures::future::Either,
    gossipsub, identity, mdns, noise, quic,
    swarm::NetworkBehaviour,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};
use lighthouse_network::types::GossipEncoding;
use lighthouse_network::GossipTopic;
use log::{error, info};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;
use tokio::{
    io::{self, AsyncBufReadExt},
    select,
};
use tokio_stream::wrappers::LinesStream;
use types::{ForkContext, ForkName};

use crate::{cli::Opts, error::ClCatError};

mod cli;
mod error;

pub struct ForkDigest(pub [u8; 4]);

impl From<ForkName> for ForkDigest {
    fn from(value: ForkName) -> Self {
        ForkDigest(match value {
            ForkName::Base => [181, 48, 63, 42],
            ForkName::Altair => [175, 202, 171, 160],
            ForkName::Merge => [74, 38, 197, 139],
            ForkName::Capella => [187, 164, 218, 150],
        })
    }
}

impl Default for ForkDigest {
    fn default() -> Self {
        default_fork().into()
    }
}

fn default_fork() -> ForkName {
    ForkName::Capella
}

#[derive(NetworkBehaviour)]
struct DefaultBehaviour {
    gossipsub: gossipsub::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

fn gossipsub_topics() -> Vec<GossipTopic> {
    lighthouse_network::types::core_topics_to_subscribe(default_fork())
        .iter()
        .cloned()
        .map(|kind| {
            GossipTopic::new(
                kind,
                GossipEncoding::default(),
                ForkDigest::default().0,
            )
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), ClCatError> {
    pretty_env_logger::init();

    let opts: Opts = Opts::parse();

    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(id_keys.public());

    info!("Local peer id: {local_peer_id}");

    // Set up an encrypted DNS-enabled TCP Transport over the yamux protocol.
    let tcp_transport =
        tcp::tokio::Transport::new(tcp::Config::default().nodelay(true))
            .upgrade(upgrade::Version::V1Lazy)
            .authenticate((noise::Config::new(&id_keys))?)
            .multiplex(yamux::Config::default())
            .timeout(std::time::Duration::from_secs(20))
            .boxed();
    let quic_transport =
        quic::tokio::Transport::new(quic::Config::new(&id_keys));
    let transport = OrTransport::new(quic_transport, tcp_transport)
        .map(|either_output, _| match either_output {
            Either::Left((peer_id, muxer)) => {
                (peer_id, StreamMuxerBox::new(muxer))
            }
            Either::Right((peer_id, muxer)) => {
                (peer_id, StreamMuxerBox::new(muxer))
            }
        })
        .boxed();

    // To content-address message, we can take the hash of message and use it as an ID.
    let message_id_fn = |message: &gossipsub::Message| {
        let mut s = DefaultHasher::new();
        message.data.hash(&mut s);
        gossipsub::MessageId::from(s.finish().to_string())
    };

    // Set a custom gossipsub configuration
    let gossipsub_config = gossipsub::ConfigBuilder::default()
        .heartbeat_interval(Duration::from_secs(10)) // This is set to aid debugging by not cluttering the log space
        .validation_mode(gossipsub::ValidationMode::Strict) // This sets the kind of message validation. The default is Strict (enforce message signing)
        .message_id_fn(message_id_fn) // content-address messages. No two messages of the same content will be propagated.
        .build()?;

    // build a gossipsub network behaviour
    let mut gossipsub = gossipsub::Behaviour::new(
        gossipsub::MessageAuthenticity::Signed(id_keys),
        gossipsub_config,
    )?;

    for topic in gossipsub_topics() {
        info!("Subscribing to gossipsub topic: {}...", &topic);
        gossipsub.subscribe(&topic.into())?;
    }

    // Create a Swarm to manage peers and events
    let mut swarm = {
        let mdns = mdns::tokio::Behaviour::new(
            mdns::Config::default(),
            local_peer_id,
        )?;
        let behaviour = DefaultBehaviour { gossipsub, mdns };
        SwarmBuilder::with_tokio_executor(transport, behaviour, local_peer_id)
            .build()
    };

    // Read full lines from stdin
    let mut stdin =
        LinesStream::new(io::BufReader::new(io::stdin()).lines()).fuse();

    if opts.listen_address.is_empty() && opts.dial_address.is_empty() {
        swarm.listen_on("/ip4/0.0.0.0/udp/0/quic-v1".parse()?)?;
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
    } else {
        for addr in opts.listen_address {
            info!("Listening on {}...", &addr);
            swarm.listen_on(addr)?;
        }

        for addr in opts.dial_address {
            info!("Dialling {}...", &addr);
            swarm.dial(addr)?;
        }
    }

    // Kick it off
    loop {
        select! {
            line = stdin.select_next_some() => {
                for topic in gossipsub_topics() {
                    if let Err(e) = swarm
                        .behaviour_mut().gossipsub
                        .publish(gossipsub::Topic::from(topic.clone()), line.as_ref()?.as_bytes()) {
                        error!("Publish error: {e:?}");
                    }
                }
            },
            event = swarm.select_next_some() => match event {
                SwarmEvent::Behaviour(DefaultBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _multiaddr) in list {
                        info!("mDNS discovered a new peer: {peer_id}");
                        swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(DefaultBehaviourEvent::Mdns(mdns::Event::Expired(list))) => {
                    for (peer_id, _multiaddr) in list {
                        info!("mDNS discover peer has expired: {peer_id}");
                        swarm.behaviour_mut().gossipsub.remove_explicit_peer(&peer_id);
                    }
                },
                SwarmEvent::Behaviour(DefaultBehaviourEvent::Gossipsub(gossipsub::Event::Message {
                    propagation_source: peer_id,
                    message_id: id,
                    message,
                })) => info!(
                        "Got message: '{}' with id: {id} from peer: {peer_id}",
                        String::from_utf8_lossy(&message.data),
                    ),
                SwarmEvent::NewListenAddr { address, .. } => {
                    info!("Local node is listening on {address}");
                }
                _ => {}
            }
        }
    }
}
