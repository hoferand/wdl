# Workflow Definition Language (WDL)

**This project was developed as part of my bachelor's thesis.**

## Interpreter

The key component of this repository is the interpreter, fully written in Rust. The interpreter, along with other tools for its use, is provided as libraries. Additionally, we provide a CLI to use it. All source files for these components can be found in `/crates`.

### Requirements

The only requirement is the [Rust](https://www.rust-lang.org/) toolchain, including `cargo`, and permission for port `3003` on `0.0.0.0` for the gRPC communication between the interpreter and the router.

### Usage

The CLI can be used with `cargo run`. Currently, the CLI supports 4 subcommands:

- `check`: checks if the syntax of a program is valid
  - e.g., `cargo run -- check examples/station2station.wdl`
- `compile`: checks the program and saves the AST as JSON
  - e.g., `cargo run -- compile examples/station2station.wdl`, outputs to `examples/station2station.wdl.compiled`
- `run`: checks the program and if it's valid, runs it
  - e.g., `cargo run -- run examples/station2station.wdl`
- `router`: can be used to simulate the router, which executes the actions physically
  - e.g., `cargo run -- router`

## Language Support

For easier usage, we provide a minimal [Visual Studio Code](https://code.visualstudio.com/) extension that offers syntax highlighting for the source files. All source files for this extension can be found in `lang-support`. For easy usage, we provide an NPM command, just run `npm run deploy` inside the `lang-support/` folder. Alternatively, the source files can be copied to `~/.vscode/extensions/wdl-lang-support/` manually.

## Language Documentation

Details on the language design and standard library can be found in the language documentation. This is provided by an [mdbook](http://rust-lang.github.io/mdBook/). All source files can be found in `lang-doc`.

### Requirements

The only requirement is the `mdbook` CLI, which can be installed with `cargo install mdbook`.

### Usage

To open the book in your browser, just run `mdbook serve --open` inside of `lang-doc/`.
