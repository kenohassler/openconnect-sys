# openconnect-sys

This crate provides auto-generated FFI bindings for [OpenConnect](https://www.infradead.org/openconnect),
the open client for Cisco AnyConnect.

## Dependencies

This crate links dynamically to your local version of `libopenconnect`.
In the build process, `pkg-config` is used to find the library.

### macOS

Install using [Homebrew](https://brew.sh):
```sh
brew install openconnect pkg-config
```

### Debian-based Linux

Install using apt:
```sh
sudo apt install libopenconnect-dev pkg-config
```

## License

OpenConnect is licensed under [GNU LGPL v2.1](https://www.gnu.org/licenses/old-licenses/lgpl-2.1-standalone.html).
This crate links dynamically to the library,
allowing the crate to be permissively licensed (MIT OR Apache-2.0).
