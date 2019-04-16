# license-gen

Tiny Rust program to generate license files.

Takes a required license name and an optional author name. If no author name is provided, retrieves name from `gitconfig` using `git config user.name`.

Available licenses can be found in the [templates](./templates) folder.

## Installation

### Prebuilt binaries:

Note: (currently only a binary for Linux-x86_64 is available)

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`, courtesy of [japaric/trust](https://github.com/japaric/trust):

```bash
bash <(curl -LSfs https://japaric.github.io/trust/install.sh) \
  -f --git cjbassi/license-gen
```

### From source:

```bash
cargo install --git https://github.com/cjbassi/license-gen
```
