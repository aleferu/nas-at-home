# nas-at-home

**HIGHLY** inspired by [TheWaWaR](https://github.com/TheWaWaR)'s version of the software: [simple-http-server](https://github.com/TheWaWaR/simple-http-server). Please make sure to take a look at his work before looking more into mine.

My goal is to build something similar without taking a look at his code (I can look at the html my browser receives). Bare in mind this is my first time using http (or https if I end up implementing it) and/or html + css...

## Usable, but not finished

There are things I want to add, but it works.

## Dependencies

- [Rust](https://www.rust-lang.org/)
- [chrono](https://docs.rs/chrono/latest/chrono/) crate.

## Installation

```sh
$ cargo build --release
```

## Usage

You can run the program using the following command:
```sh
$ cargo run --release
```

Help text:
```sh
$ nas-at-home --help
Usage: nas-at-home [FLAGS] [OPTIONS]

FLAGS:
    --help     Prints this, nothing else happens.
OPTIONS
    -ip        Sets the ip for the TCP Listener, 127.0.0.1 is the default value.
               Example: -ip 127.0.0.1
    -port      Sets the port for the TCP Listener, 8080 is the default value
               Example: -port 8080
```

An example would be:
```sh
$ nas-at-home -ip 127.0.0.1 -port 8080
```

The idea is to move the binary to the folder where you want the server (at least for now).

## TODOs

- Specify start folder.
- Maybe https.
- I don't know hot to setup passwords but it's an idea.
- Upload?

## Contact

Feel free to contact me if you encounter any issue.
