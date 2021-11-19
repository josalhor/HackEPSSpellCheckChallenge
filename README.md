Tested with rustc 1.56.1

compile on windows with:

```
cargo build --release
```

compile on linux with:
```
RUSTFLAGS="-C opt-level=3 -C debuginfo=0 -C target-cpu=native" cargo build --release
```

Run the binary `RustCorrect` on `target/release/`