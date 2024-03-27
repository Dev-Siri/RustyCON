# RCON Protocol

Basic RCON Protocol implementation in Rust.

## Usage

Run the program with `cargo run` and provide a peer connection address

```sh
$ cargo run --release -- 127.0.0.1:8080
```

You can then type your commands

```sh
<pak_id> <type> say <body>

# example
69420 SERVER_DATA_EXEC_COMMAND say hello_world
```

## Getting Started

- Clone.

```sh
$ gcl https://github.com/Dev-Siri/RustyCON
```

- Compile/Install

```sh
$ cargo install --path .
```

- Run (see the usage section)

```
$ rscon <conn-url>
```

## License

This project is MIT Licensed. See [LICENSE](LICENSE)
