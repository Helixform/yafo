# Yafo (Yet Another File Obfuscator)

## Introduction

Yafo is a minimalist file obfuscator, which can "encrypt" your file with the key derived from a given mnemonic phrase. It provides both CLI and library, so you can use it independently or embedded in your own apps. Yafo uses its own [algorithm](./docs/algorithm-design.md), and the key difference from other encryption algorithms (like AES) is that it's super fast.

## Getting Started

If you want to use it as a CLI program, you can install it on the supported platforms. Yafo supports various of platforms, and it's currently tested and distributed on the following platforms:

-   macOS
-   Linux
-   Windows

### Install with Cargo

```shell
cargo install yafo --features=cli
```

### From binaries

Download the prebuilt binary from [releases](https://github.com/Helixform/yafo/releases) and execute it directly. You can append the executable path to environment variables manually if you want.

### From source

If you want to build it from source locally, you need Rust installed. Clone the repository and build everything with `cargo`:

```shell
cargo install --locked --features=cli --path .
```

If you don't need the CLI binary, you can just compile the library itself:

```shell
cargo build --release
```

## CLI Usage

Encrypt a file with the given mnemonic phrase:

```shell
yafo encrypt --key <YOUR_KEY> /path/to/file-to-encrypt
```

The file will be encrypted **in-place**. And after encryption, a `.yafo` extension will be appended to the filename of the given file.

To decrypt it:

```shell
yafo decrypt --key <YOUR_KEY> /path/to/file-to-decrypt
```

Note that any file will be treated transparently, whether it's encrypted or not. It means `yafo` will not check whether the given file is ever encrypted when you execute `yafo decrypt`. And you can also encrypt the same file multiple times with `yafo encrypt`.

For better performance, you can use `--silent` option to run it without displaying the progress bar.

## FAQ

### What are the possible use cases of it?

The use cases of Yafo can be various. You can use it to make your messages hard to recognize, prevent your files against hash matching, and bypass the Internet censorship. However, Yafo as a tool is not responsible for your illegal usages. You should use it at your own risk.

### Why not using AES or other algorithms?

The main purpose of Yafo is data obfuscation, not encryption. AES and other algorithms can be complex and less efficient. The algorithm of Yafo is designed to be fast, and security is not the top priority.

### Is it secure to encrypt important data?

No, the algorithm of Yafo is not designed for strong encryption, and its security is also not validated. Additionally, you should regard the key as a seed, which is used to add randomness to the algorithm. It's still possible to decrypt a file using a key other than the original one for encryption.

### What if I forgot my key?

Unfortunately, there is no metadata attached to the encrypted file. Thus the key is dropped once the encryption process is finished. It's also impossible to test whether the decrypted file is valid. Simply put, if you lose your key, you lose your data.

## License

Copyright (c) 2023 Yafo Developers.

Source code and its algorithm are available under the terms of GPLv3 license.
