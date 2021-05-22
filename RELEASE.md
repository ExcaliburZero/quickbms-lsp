* Update the version number in `Cargo.toml`
* Update `CHANGELOG.md`

```bash
git push origin master
```

* Create a package file for the crate:

```bash
rm tree-sitter-quickbms/Cargo.toml
cargo publish --dry-run
cargo publish --dry-run --allow-dirty
```

* Create a binary for the language server:

```bash
cargo clean
cargo build --release
cargo build --release --target x86_64-pc-windows-gnu
```

* Rename the Linux and Windows binaries.

* Create a new release on GitHub
    * Set a tag name (ex. `0.0.1`)
    * Set the release title (ex. `0.0.1`)
    * Attach the generated binaries and package file
    * Click publish
* Create a new release on Crates.io

```bash
cargo publish
```
