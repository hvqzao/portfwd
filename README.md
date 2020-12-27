# TCP Port Fwd

Simple TCP port forwarded written in Rust.

Uses Tokio async runtime.

## Usage

```
target\debug\portfwd.exe [-q|--quiet] [<bind_addr>:]<bind_port>:<dst_addr>:<dst_port> [...]
```

## Ready to use binaries

Pre-build releases are located in `build` directory.

Stripped / packed with upx.

bin/windows/x86_64:

```
cargo build --target=x86_64-pc-windows-msvc --release
upx --best --lzma target/x86_64-pc-windows-msvc/release/portfwd.exe
copy target/x86_64-pc-windows-msvc/release/portfwd.exe bin/windows/x86_64/
```

bin/windows/i686:

```
cargo build --target=i686-pc-windows-msvc --release
upx --best --lzma target/i686-pc-windows-msvc/release/portfwd.exe
copy target/i686-pc-windows-msvc/release/portfwd.exe bin/windows/i686/
```

bin/linux/x86_64:

```
cargo build --target=x86_64-unknown-linux-gnu --release
strip target/x86_64-unknown-linux-gnu/release/portfwd
cp target/x86_64-unknown-linux-gnu/release/portfwd bin/linux/x86_64/
```

bin/linux/i686:

```
sudo apt install gcc-multilib
#rustup toolchain install stable-i686-unknown-linux-gnu
rustup target add i686-unknown-linux-gnu
cargo build --target=i686-unknown-linux-gnu --release
strip target/i686-unknown-linux-gnu/release/portfwd
cp target/i686-unknown-linux-gnu/release/portfwd bin/linux/i686/
```

## License

[MIT License](LICENSE)
