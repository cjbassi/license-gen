# license-gen

Tiny Rust program to generate license files.

Install with:

```
cargo install --git https://github.com/cjbassi/license-gen
```

Takes a required license name and an optional author name. If no author name is provided, retrieves name from `gitconfig` using `git config user.name`.

Available licenses can be found in the [templates](./templates) folder.
