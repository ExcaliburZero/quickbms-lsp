* Update the version number in `Cargo.toml`
* Update `CHANGELOG.md`
* Create a package file for the crate:

```bash
cargo publish --dry-run
```

* Create a binary for the language server:

```bash
cargo clean
cargo build --release
```

* Create a new release on GitHub
    * Set a tag name (ex. `0.0.1`)
    * Set the release title (ex. `0.0.1`)
    * Attach the generated binary and package file
    * Click publish
* Create a new release on Crates.io

```bash
cargo publish
```
