# Hibernation Control

:warning:

This little tool only works on Ubuntu. There is a lot of hard-coded paths, and it will
override the Ubuntu's standard swapfile.

Eventually, I might add some more configurations options and enable the systemd
integration (see `./src/systemd.rs`).

:warning:

## Usage

```sh
cargo build --release
sudo ./target/release/hibernation-control enable
```

## TODO

- [x] Basic `enable` command
- [x] EFI support
- [ ] `disable` command
- [ ] EFI configuration
- [ ] Test user is `root`
- [ ] Test dependencies before running it
