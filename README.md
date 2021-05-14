# sfetch

A minimalistic and customizable fetching program written in Rust.

## The Goal

Create a fetching program, similar to `neofetch`, `afetch`, and others, while maintaining as
much functionality as possible with a maximum of 80 lines of code (SLOC).

## Name Choice

One letter followed by 'fetch' is a pretty common naming scheme for these fetching programs.
I chose 's' because this project is really stupid.

## Configuration

Configuration is stored in the `src/config.rs` file, and is used at compile-time. You must
rebuild to see changes.
