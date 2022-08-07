# Running

There are two ways to run the server.

## Download Binary

TODO: Download Binary from github actions

## Build from Source

Building from source requires a few dependencies

- [Git](https://git-scm.com)
- [The Rust Toolchain](https://www.rust-lang.org/tools/install)

First, get the source code from the [Github repo](https://github.com/KennanHunter/photo).

With the Cli, run:

```sh
git clone https://github.com/KennanHunter/photo
```

Then run the code with the command.

```sh
cargo run --release
```

And thats it!
The entire server is written in pure rust, allowing us to have a very easy build process.
