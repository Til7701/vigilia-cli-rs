# Vigilia CLI

Vigilia CLI is a command-line interface for the Vigilia toolbox.
The service, which contains the logic for the Vigilia toolbox, is located [here](https://github.com/schlunzis/vigilia).

## Build

To build the Vigilia CLI, you need to have Rust and Cargo installed.

You can install those using `rustup`:

```bash
rustup toolchain install stable
```

Then, you can build the CLI using the following command:

```bash
cargo build
```

This will build the CLI and create an executable in the `target/debug` directory.

## Run

To run the CLI, you can use the following command:

```bash
cargo run -- <args>
```

Where `<args>` are the args to pass to the CLI.
