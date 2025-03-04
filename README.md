# fish-hist-import

`fish-hist-import` is a tool that helps you import your ZSH history into FISH. It reads your ZSH history file, converts the commands to FISH syntax, expands aliases, and converts ZSH-specific syntax to FISH syntax.

## Prerequisites

Before you can use `fish-hist-import`, you need to have the Rust toolchain installed. You can install it via [rustup](https://rustup.rs/).

## Installation

To install `fish-hist-import`, clone the repository and build the project using Cargo:

```sh
git clone git@github.com:scnewma/fish-hist-import.git
cd fish-hist-import
cargo install --path .
```

Replace `<repository-url>` with the URL of your repository.

## Usage

Once installed, you can run the `fish-hist-import` binary to import your ZSH history into FISH:

```sh
fish-hist-import
```

The tool will read the `HISTFILE` from ZSH, convert the commands, and append them to your FISH history file located at `~/.local/share/fish/fish_history`.

## Configuration

`fish-hist-import` reads the `HISTFILE` environment variable from ZSH to locate your ZSH history file. Make sure this variable is set correctly in your ZSH configuration.

## Limitations

- The conversion from ZSH to FISH is basic and uses string replacement. This may lead to incorrect replacements, such as replacing `&&` in the middle of a string.
- The tool does not handle all edge cases and may not work perfectly for all ZSH commands.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Credits

This project was inspired by [rsalmei/zsh-history-to-fish](https://github.com/rsalmei/zsh-history-to-fish).
