# license-gen

Tiny Rust program to generate license files.

Takes a required license name and an optional author name. If no author name is provided, retrieves name from `gitconfig` using `git config user.name`.

Available licenses can be found in the [templates](./templates) folder.

## Installation

### Prebuilt binaries:

Run the following to download the correct binary for your system from the releases tab into `$CARGO_HOME/bin`: (currently only Linux-x86_64 is available)

```
bash <(curl https://raw.githubusercontent.com/japaric/trust/c268696ab9f054e1092f195dddeead2420c04261/install.sh) -f --git cjbassi/license-gen
```

### From source:

```
cargo install --git https://github.com/cjbassi/license-gen
```
