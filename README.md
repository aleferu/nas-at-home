# NAS-at-home

**HIGHLY** inspired by [TheWaWaR](https://github.com/TheWaWaR)'s version of the software: [simple-http-server](https://github.com/TheWaWaR/simple-http-server). Please make sure to take a look at his work before looking more into mine.

My goal is to build something similar without taking a look at his code (I can look at the html my browser receives). Bare in mind this is my first time using http (or https if I end up implementing it) and/or html + css...

## Usable, but not finished

There are things I want to add, but it works.

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [chrono](https://docs.rs/chrono/latest/chrono/) crate.

## Installation

```sh
cargo build --release
```

## Usage

You either run the program using the following command:
```sh
cargo run --release
```

But the idea is to move the binary to the folder where you want the server (at least for now).

## TODO

- Previsualize images.
- Previsualize text files.
- Previsualize pdfs.
- IP + port.
- Specify start folder.
- Maybe https.
- I don't know hot to setup passwords but it's an idea.
