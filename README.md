parselnk
========

Parse Windows `.lnk` files in pure Rust!

## Usage

#### In your `Cargo.toml`:

From `crates.io`:
```toml
[dependencies]
parselnk = "0.1"
```

To use master branch:
```toml
[dependencies]
parselnk = { git = "https://github.com/rustysec/parselnk-rs" }
```

#### In your code:

```rust
let lnk_path = std::path::Path::new(r"c:\users\me\desktop\slack.lnk");
let lnk = parselnk::Lnk::from(lnk_path).unwrap();

println!("Lnk relative path: {:?}", lnk.relative_path());
```


## Features
These features are enabled by default and can be toggled off 
by specifying `default-features = false` in your `Cargo.toml`:

```toml
# Disable optional features
[dependencies]
parselnk = { version = "0.1", default-features = false }
```

- `chrono` - exposes convenience methods for parsing windows `FileTime` structures
- `serde` - enables serialization of `parselnk` types

## Helping Out
Issues and pull requests are welcome!
