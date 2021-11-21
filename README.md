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

## Notes

This program is as correct as one can get.
There may be a tie between some words, in which case the first match will be returned.
This means there isn't a *single* possible solution.