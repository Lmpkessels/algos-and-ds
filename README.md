# Algos and DS

//TODO: Add under development status, add issues, add benchmarks etc.
![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Built with C](https://img.shields.io/badge/Built%20with-C-blue.svg)
![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-red.svg)

This repository will cover multiple algorithms, hash functions, and data-structures during the process of maintaining it. It's purpose is to solve complex problems and give computers step-by-step instructions to accomplish specific tests.

## Index

| Algorithm        | Language | Status | Tests        |
| ---------------- | -------- | ------ | ------------ |
| SHA1             | Rust     | Done   | Included     |
| SHA256           | Rust     | Done   | Included     |
| SHA512           | Rust     | Done   | Included     |
| SHA3             | Rust     | Done   | Included     |
| MD4              | Rust     | Done   | Included     |
| MD5              | Rust     | Done   | Included     |
| RIPEMD-160       | Rust     | Done   | Included     |
| Binary Search    | C        | Done   | Included     |
| Quick Sort       | C        | Done   | Included     |
| Insertion Sort   | Rust     | Done   | Not Included |
| Merge Sort       | Rust     | Done   | Not Included |
| Roman to integer | Rust     | Done   | Not Included |

## Modules

- [Hash Algorithms in Rust](./rust/crypto)
- [Sorting Algorithms in C](./c/algorithms)

## Setup

```bash
git clone https://github.com/Lmpkessels/algos-and-ds.git
cd algos-and-ds

# For C
cd c
gcc tests/test_name.c directory_name/name.c -o build/test_name
./build/name

# For Rust
cd rust
cargo test
```

Navigate through folders by topic.

## Contribution

Pull requests are welcome.
For major changes, please open an issue first to discuss what you’d like to improve or add.

## License

Licensed under [MIT License](./LICENSE-MIT). <br/>
© 2026 Luuk Kessels
