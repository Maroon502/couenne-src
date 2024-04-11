# Couenne-src

[![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![License][license-img]][license-url]

## description

`Couenne-src` crate is a *-src crate. This links [Couenne] libraries to executable build by cargo, but does not provide Rust bindings. [Couenne] is built with [CoinUtils] ([CoinUtils-src]), [Osi] ([Osi-src]), [Cgl] ([Cgl-src]), [Clp] ([Clp-src]), [Cbc] ([Cbc-src]), [Ipopt] ([Ipopt-src]), [Mumps] ([Mumps-src])(Optional), [OpenBLAS] ([OpenBLAS-src])(Optional), [Intel-mkl] ([intel-mkl-src])(Optional) and [Bonmin] ([Bonmin-src]).

By this package, you don't need to worry about installing Couenne in the system, and it's a package for **all platforms**.

[Couenne] (Convex Over and Under ENvelopes for Nonlinear Estimation) is a branch&bound algorithm to solve Mixed-Integer Nonlinear Programming (MINLP) problems

## Usage

1. add the following to your `Cargo.toml`:

    ```toml
    [dependencies]
    couenne-src = "\*"
    ```

2. add the following to your `lib.rs`:

    ```toml
    extern crate couenne_src;
    ```

This package does not provide bindings. There's no C interface now.

## Configuration

### Environment

The package build from the source and link statically by default. It also provide the following environment variables to allow users to link to system library customly:

* `CARGO_COUENNE_STATIC` to link to Couenne statically;
* `CARGO_COUENNE_SYSTEM` to link to Couenne system library;
* `CARGO_BONMIN_STATIC` to link to Bonmin statically;
* `CARGO_BONMIN_SYSTEM` to link to Bonmin system library;
* `CARGO_IPOPT_STATIC` to link to Ipopt statically;
* `CARGO_IPOPT_SYSTEM` to link to Ipopt system library;
* `CARGO_MUMPS_STATIC` to link to Mumps statically;
* `CARGO_MUMPS_SYSTEM` to link to Mumps system library;
* `CARGO_COINUTILS_STATIC` to link to CoinUtils statically;
* `CARGO_COINUTILS_SYSTEM` to link to CoinUtils system library;
* `CARGO_OSI_STATIC` to link to Osi statically;
* `CARGO_OSI_SYSTEM` to link to Osi system library;
* `CARGO_CLP_STATIC` to link to Clp statically;
* `CARGO_CLP_SYSTEM` to link to Clp system library;
* `CARGO_CGL_STATIC` to link to Cgl statically;
* `CARGO_CGL_SYSTEM` to link to Cgl system library;
* `CARGO_CBC_STATIC` to link to Cbc statically;
* `CARGO_CBC_SYSTEM` to link to Cbc system library;

Set the environment variable to `1` to enable the feature. For example, to link to system library dynamically, set `CARGO_${LIB_NAME}_SYSTEM` to `1`; to link to system library statically, set both `CARGO_${LIB_NAME}_SYSTEM` and `CARGO_${LIB_NAME}_STATIC` to `1`.

If you enable OpenBLAS([openblas-src]), you can also pass env to `make` by `OPENBLAS_*`. Read more at [here](#cross-compilation)

### Others

```toml
bonmin-src = { version = "\*", default-features = no }
ipopt-src = { version = "\*", features = ["intel-mkl", "intel-mkl-lp64-seq"] }
```

ref: [intel-mkl-src], [intel-mkl-src], [ipopt-src], [mumps-src], [bonmin-src]

## Windows and vcpkg

On Windows, openblas need [vcpkg] to find Couenne. Before building, you must have the correct Couenne installed for your target triplet and kind of linking. For instance, to link dynamically for the `x86_64-pc-windows-msvc` toolchain, install  `couenne` for the `x64-windows` triplet:

```sh
vcpkg install couenne --triplet x64-windows
```

To link Couenne statically, install `couenne` for the `x64-windows-static-md` triplet:

```sh
vcpkg install couenne --triplet x64-windows-static-md
```

To link Couenne and C Runtime (CRT) statically, install `couenne` for the `x64-windows-static` triplet:

```sh
vcpkg install couenne --triplet x64-windows-static
```

and build with `+crt-static` option

```sh
RUSTFLAGS='-C target-feature=+crt-static' cargo build --target x86_64-pc-windows-msvc
```

Please see the ["Static and dynamic C runtimes" in The Rust reference](https://doc.rust-lang.org/reference/linkage.html#static-and-dynamic-c-runtimes) for detail.

## Cross Compilation

If you use OpenBLAS([openblas-src]), you need to set `OPENBLAS_CC`, `OPENBLAS_FC`, `OPENBLAS_HOSTCC`, and `OPENBLAS_TARGET` to pass env to [OpenBLAS], ref:[openblas-src] and [OpenBLAS]. For example:

```sh
export OPENBLAS_TARGET=ARMV8
export OPENBLAS_HOSTCC=gcc
export OPENBLAS_CC=aarch64-linux-gnu-gcc
export OPENBLAS_FC=aarch64-linux-gnu-gfortran
```

you can compile it for the other target by providing the `--target` option to `cargo build`.

| Target                               |  supported  |
|--------------------------------------|:-----------:|
| `arm-unknown-linux-gnueabi`          | ✓   |
| `arm-unknown-linux-gnueabihf`        | ✓   |
| `armv7-unknown-linux-gnueabi`        | ✓   |
| `armv7-unknown-linux-gnueabihf`      | ✓   |
| `armv7-unknown-linux-musleabi`       | ✓   |
| `armv7-unknown-linux-musleabihf`     | ✓   |
| `aarch64-unknown-linux-gnu`          | ✓   |
| `aarch64-unknown-linux-musl`         | ✓   |
| `riscv64gc-unknown-linux-gnu`        | ✓   |
| `x86_64-pc-windows-msvc`             | ✓   |
| `x86_64-unknown-linux-gnu`           | ✓   |
| others                               | not test   |

Note: `intel-mkl-*` can only be used for `x86_64-*`.`openblas-static` can only be used for `linux`.

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE](license-url).

[CoinUtils]: https://github.com/coin-or/CoinUtils
[Osi]: https://github.com/coin-or/Osi
[Cgl]: https://github.com/coin-or/Cgl
[Clp]: https://github.com/coin-or/Clp
[Cbc]: https://github.com/coin-or/Cbc
[Ipopt]: https://github.com/coin-or/Ipopt
[Bonmin]: https://github.com/coin-or/Bonmin
[Couenne]: https://github.com/coin-or/Couenne
[Mumps]: https://mumps-solver.org/
[OpenBLAS]: https://github.com/OpenMathLib/OpenBLAS
[intel-mkl]: https://www.intel.com/content/www/us/en/developer/tools/oneapi/onemkl.html

[CoinUtils-src]: https://github.com/Maroon502/coinutils-src
[Cgl-src]: https://github.com/Maroon502/cgl-src
[Clp-src]: https://github.com/Maroon502/clp-src
[Osi-src]: https://github.com/Maroon502/osi-src
[Ipopt-src]: https://github.com/Maroon502/ipopt-src
[Mumps-src]: https://github.com/Maroon502/mumps-src
[Bonmin-src]: https://github.com/Maroon502/bonmin-src
[OpenBLAS-src]: https://github.com/blas-lapack-rs/openblas-src
[intel-mkl-src]: https://github.com/rust-math/intel-mkl-src

[vcpkg]: https://github.com/Microsoft/vcpkg

[documentation-img]: https://docs.rs/couenne-src/badge.svg
[documentation-url]: https://docs.rs/couenne-src
[package-img]: https://img.shields.io/crates/v/couenne-src.svg
[package-url]: https://crates.io/crates/couenne-src
[license-img]: https://img.shields.io/crates/l/couenne-src.svg
[license-url]: https://github.com/Maroon502/couenne-src/blob/master/LICENSE.md
