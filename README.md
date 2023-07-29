# epub_to_txt

A simple (but really fast!) CLI tool written in Rust to convert an epub into a text file (in Markdown format).

## Usage

    epub_to_txt filename.epub > output.txt

## Build

To build the tool, use [cargo](https://doc.rust-lang.org/stable/cargo/).

    cargo build --release

The resulting binary will be in `./target/release/epub_to_txt` (~ 4.2M).

## Benchmarks

Time taken to convert some example epub books (on my MacBook Pro):

- Simple 248K text only epub = 0.02s
- 1.9M software engineering book = 0.03s
- More complex 37.1M computer science textbook = 0.15s
