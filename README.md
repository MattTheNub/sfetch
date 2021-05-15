# sfetch

A minimalistic and customizable fetching program written in Rust.

## Installation

The easiest way to install `sfetch` is to run `cargo install sfetch`. However, if you want to
customize your installation, you may instead build from source:

```sh
git clone https://github.com/MattTheNub/sfetch.git
cd sfetch
cargo build --release
```

You may now [configure](#configuration) and rebuild `sfetch`. The binary will be located at
`target/release/sfetch`.

## The Goal

Create a fetching program, similar to `neofetch`, `afetch`, and others, while maintaining as
much functionality as possible with a maximum of 80 lines of code (SLOC).

## Name Choice

One letter followed by 'fetch' is a pretty common naming scheme for these fetching programs.
I chose 's' because this project is really stupid.

## Configuration

Configuration is stored in the `src/config.rs` file, and is used at compile-time. You must
rebuild (`cargo build --release`) to see changes.
