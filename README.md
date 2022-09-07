# openconnect-sys

This crate contains auto-generated FFI bindings for libopenconnect.

## Dependencies

At the moment, this crate links openconnect to your system-provided versions of
* libxml2
* zlib
* OpenSSL or GnuTLS *(>=3.2.10)*.

This may change in future versions.

### macOS

Install GnuTLS using homebrew:
```sh
brew install gnutls
```
The rest is included with the developer tools.

### Debian-based Linux

Install using apt:
```sh
sudo apt install libxml2-dev zlib1g-dev libssl-dev
```

## License

Openconnect is licensed under [GNU LGPL v2.1](https://www.gnu.org/licenses/old-licenses/lgpl-2.1-standalone.html).

## Credits

All credits go to [OpenConnect](https://www.infradead.org/openconnect).
