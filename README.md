# Rust Program Template #

This is a template for a program written in Rust.

## Usage ##

### Things you need to do ###

Suppose your program is called "ethtool".

 1. `sed -i "s/foobar/ethtool/gI" Cargo.toml`
 2. `sed -i "s/Foobar/EthTool/gI" src/*.rs`
 3. Manually populate `LICENSE` (optionally replacing it altogether)
 4. Manually populate `Cargo.toml` with [the necessary fields](https://doc.rust-lang.org/cargo/reference/manifest.html)
 5. `rm -rf .git && git init` (reinitialise Git repository for your project; **ignore if you're using the GitHub template feature**)
 6. ???
 7. Profit

## Features ##

 - Global error type allowing ready use of the question mark operator
 - Derived Clap-v3-style CLI
 - Asynchronous by default
 - GitHub Actions workflow for CI/CD

## Motivation ##

For most applications, I would wager boilerplate obeys a [power law](https://en.wikipedia.org/wiki/Power_law). This template aims to capture the vast majority of such boilerplate by targeting the least common denominator of functionality amongst the various domains I work on.

