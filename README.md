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

In order to avoid licensing problems (OpenConnect is licensed under LGPL v2.1),
we only allow dynamic linking.
This ensures that the crate can be permissively licensed (MIT+Apache2).
