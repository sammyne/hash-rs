# hash

![build status](https://github.com/sammyne/hash-rs/workflows/build/badge.svg)
[![docs badge](https://img.shields.io/badge/docs-0.2.0-blue)][doc-page]
![minimum rustc](https://img.shields.io/badge/rustc-1.65.0%2B-blue)

This repository tries to implement a Go-like hash library in Rust.

## Overview

Supported hashes go as follow

- [x] [crc32][crc32-doc-page]
- [x] [crc64][crc64-doc-page]
- [ ] adler32
- [ ] fnv
- [ ] maphash

## Reference

- [Go's hash package](https://pkg.go.dev/hash)

[doc-page]: https://sammyne.github.io/hash-rs/hash/
[crc32-doc-page]: https://sammyne.github.io/hash-rs/hash/crc32/
[crc64-doc-page]: https://sammyne.github.io/hash-rs/hash/crc64/
