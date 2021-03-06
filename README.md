# TCP Port Fwd

Simple TCP port forwarder written in Rust.

Uses Tokio async runtime.

## Usage

```
target\debug\portfwd.exe [-q|--quiet] [<bind_addr>:]<bind_port>:<dst_addr>:<dst_port> [...]
```

## Prebuilt binaries

Ready to use, stripped / packed with upx binaries are located in `bin` directory.

### bin/windows/x86_64:

Download link: [portfwd.exe](https://github.com/hvqzao/portfwd/raw/master/bin/windows/x86_64/portfwd.exe)

```
cargo build --target=x86_64-pc-windows-msvc --release
upx --best --lzma target/x86_64-pc-windows-msvc/release/portfwd.exe
copy target/x86_64-pc-windows-msvc/release/portfwd.exe bin/windows/x86_64/
```

### bin/windows/i686:

Download link: [portfwd.exe](https://github.com/hvqzao/portfwd/raw/master/bin/windows/i686/portfwd.exe)

```
cargo build --target=i686-pc-windows-msvc --release
upx --best --lzma target/i686-pc-windows-msvc/release/portfwd.exe
copy target/i686-pc-windows-msvc/release/portfwd.exe bin/windows/i686/
```

### bin/linux/x86_64:

Download link: [portfwd](https://github.com/hvqzao/portfwd/raw/master/bin/linux/x86_64/portfwd)

```
cargo build --target=x86_64-unknown-linux-gnu --release
strip target/x86_64-unknown-linux-gnu/release/portfwd
cp target/x86_64-unknown-linux-gnu/release/portfwd bin/linux/x86_64/
```

### bin/linux/i686:

Download link: [portfwd](https://github.com/hvqzao/portfwd/raw/master/bin/linux/i686/portfwd)

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
