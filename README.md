# Prepare build environment
```bash
cargo install cargo-xbuild
```

# Build drivers
```bash
# build driver for x64
cargo xbuild --target x86_64-kernel-windows-msvc.json
# build driver for x86
cargo xbuild --target i686-kernel-windows-msvc.json
```

# Acknowledges
* [winapi-kmd-rs](https://github.com/pravic/winapi-kmd-rs)
* [blog_os](https://os.phil-opp.com/)
* [cargo-xbuild](https://github.com/rust-osdev/cargo-xbuild)
