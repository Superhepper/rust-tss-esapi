# TPM2 Software Stack Rust Wrapper

<p align="center">
  <a href="https://crates.io/crates/tss-esapi-sys"><img alt="Crates.io" src="https://img.shields.io/crates/v/tss-esapi-sys"></a>
  <a href="https://docs.rs/tss-esapi-sys"><img src="https://docs.rs/tss-esapi-sys/badge.svg" alt="Code documentation"/></a>
  <a href="https://codecov.io/gh/parallaxsecond/rust-tss-esapi"><img src="https://codecov.io/gh/parallaxsecond/rust-tss-esapi/branch/main/graph/badge.svg?token=5T7SVCHWFE"/></a>
</p>

This is the lower-level wrapper that exposes a minimal, low-level C
interface to Rust to [TSS](https://github.com/tpm2-software/tpm2-tss).

## Dependencies

This crate exposes an interface for the TSS Enhanced System API and thus
links to libraries that expose this interface. In order to allow proper use
of the ESAPI, this FFI layer includes bindings to TCTI and MU headers, and
must therefore link to all of them at build time.

The paths to the libraries are discovered using `pkg-config` - make sure they
are discoverable in this way on your system. Our build script looks for
`tss2-esys`, `tss2-tctildr` and `tss2-mu`. A minimum version of `4.0.1` is
required for all of them.

Having installed the open-source implementation libraries at `/usr/local/lib` (by default), it
might happen that `pkg-config` can not find them. Run the following command if that is the
case:
```bash
$ export PKG_CONFIG_PATH=/usr/local/lib/pkgconfig
```

The FFI bindings presented by this crate can be either those committed in the
crate under `src/bindings` or generated on the fly from the library headers
found on the system, at build time. For generating the bindings at build time
please enable the `generate-bindings` feature, as it is not enabled by default.
The build script will then identify the header files using `pkg-config` and
generate fresh bindings from them.

NOTE: Only a limited set of bindings are committed and their target triplet
is included in the name of the file - if the triplet you require is not
available, feel free to raise a Pull Request to add it or to use build-time
generation of bindings. All the committed bindings **MUST** be generated from
the library version found under the `vendor` submodule.

## Bundling TPM-TSS

tpm-tss is used by this library to communicate with TPMs. If this library
is not available on your system you may optionally bundle (vendor) tpm-tss
during builds. tpm-tss can be provided from a local source path with the
environment variable `TPM_TSS_SOURCE_PATH` or it will be retrieved from
github during the build. The version to retrieve can be controlled by setting
the `TPM2_TSS_SOURCE_VERSION` environment variable.
N.B. On windows it might be necessary to manually create the VERSION file
when a local source is being used.

To enable this feature:

```bash
cargo build --features=bundled
```

```bash
TPM2_TSS_VERSION="4.1.3" cargo build --features=bundled
```

```bash
TPM_TSS_SOURCE_PATH=/path/to/tpm-tss cargo build --features=bundled
```

If using this feature from an external project

```
tss-esapi-sys = { version = "...", features = "bundled" }
```

### Windows

Compiling for windows requires a bit of setup to work with the bundled feature.

* Openssl must be installed to a non-standard location at C:\OpenSSL-v11-Win64
* Visual studio 2019 must be installed with the Clang/C2 experimental component,
  and windows sdk 10.0 (Other versions of Visual Studio may work but are untested 
  at this point).

### MacOS

Compiling on MacOS requires the bundling feature. This requires dependencies
from brew.

```bashbre
brew install autoconf autoconf-archive automake json-c libtool m4 pkg-config
```

Optionally you may require these libraries for certain classes of TPM transport

```
brew install libftdi
```

### OpenSUSE / SUSE

```
sudo zypper in autoconf autoconf-archive automake libjson-c-devel libtool libtpms-devel gawk make
```

## Cross compiling

Cross-compilation can be done as long as you have on your build system the TSS
libraries compiled for your target system of choice. We rely on `pkg-config` to
identify the libraries which we link against. Installing `tpm2-tss` does yield
`.pc` files which can be used for this purpose, but depending on the exact build
environment setup, the configuration and compilation of `tpm2-tss` could require
some special tailoring.

We include cross-compilation builds as a nightly check in Github Actions - you
can find them
[here](https://github.com/parallaxsecond/rust-tss-esapi/blob/main/tss-esapi/tests/cross-compile.sh)
as an example of the steps needed. You can find more information on using
`pkg-config` when cross-compiling
[here](https://github.com/parallaxsecond/rust-tss-esapi/issues/204). Our
wrapper script around `pkg-config` can be seen
[here](https://github.com/parallaxsecond/rust-tss-esapi/blob/main/tss-esapi/tests/pkg-config).

Be advised that in some cases the linker used might need to be set manually in
`.cargo/config`.

## Locally built tpm2-tss
It is now possible to specify an installation path when building the crate. This will
make the build process trying to find all the libraries and header files it needs from
installation path instead of using `pkg-config`.

The `TPM2_TSS_PATH` environment variable name is used to specify the path to the installation.
The installation is required to have a specific layout.

```md
Installation folder
├── bin (Optional)
│   ├── tss2-*.dll (Windows)
├── include (Required)
│   ├── tss2
│   │   ├── tss2_*.h
├── lib (Required)
│   ├── tss2-*.lib (Windows)
│   ├── tss2-*.so  (Nix)
│   ├── tss2-*.pdb (Windows)
└── VERSION (Required)
```

*Copyright 2021 Contributors to the Parsec project.*
