# Soma

[![Build Status](https://dev.azure.com/plus-postech/soma/_apis/build/status/PLUS-POSTECH.soma?branchName=master)](https://dev.azure.com/plus-postech/soma/_build/latest?definitionId=1?branchName=master)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FPLUS-POSTECH%2Fsoma.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2FPLUS-POSTECH%2Fsoma?ref=badge_shield)

Your one-stop CTF problem management tool


## Testing, Building, and Running

soma is written with Rust, and utilizes Cargo as a building and testing system.

You can test, build, run using the following command:

```
cargo test
cargo build
cargo run
```


## Development

* Install Rust stable toolchain.
* Install `openssl` (Required by `openssl-sys` crate).
* Install `rustfmt`.
    * `rustup component add rustfmt`
* Copy files in `hooks` directory to `.git/hooks`.


## License

Licensed under either of
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.



[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FPLUS-POSTECH%2Fsoma.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FPLUS-POSTECH%2Fsoma?ref=badge_large)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.