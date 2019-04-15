
# prepare
cargo install cargo-xbuild
# how to build
```bash
# build driver for x64 
cargo xbuild --target x86_64-kernel-windows-msvc.json 
# build driver for x86
cargo xbuild --target i686-kernel-windows-msvc.json 
```
