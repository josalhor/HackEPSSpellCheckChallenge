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

```
./target/release/<RustCorrect binary>
```

Note: The binary gets the dictionary.txt and original.txt files from the current working directory (CWD). So do not change directories!

## Notes

This program is as correct as one can get.

There may be a tie between some words, in which case the first match will be returned.

This means there isn't a *single* possible solution.

## Performance Note

This implementation... Is crazy fast.

It is so fast I have trouble measuring how much the executions last. lol.