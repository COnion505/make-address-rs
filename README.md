
## build on mac for linux
```terminal
rustup target add x86_64-unknown-linux-gnu
brew install SergioBenitez/osxct/x86_64-unknown-linux-gnu
```

~/.cargo/config.toml
```toml
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-unknown-linux-gnu-gcc"
```

```terminal
TARGET_CC=x86_64-unknown-linux-gnu cargo build --release --target x86_64-unknown-linux-gnu
```