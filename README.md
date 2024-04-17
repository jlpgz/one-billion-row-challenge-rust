# One Billion Rows Challenge in Rust

This repository contains my solution to the One Billion Rows Challenge, implemented in Rust. The challenge is a common
benchmark used to test the performance of different programming languages and systems when processing large amounts of
data.

## About the Code

The code in this repository is written in Rust, a systems programming language that runs blazingly fast, prevents
segfaults, and guarantees thread safety. One of the key features of Rust is its emphasis on safety. This code does not
make use of `unsafe`.

I have used some optimizations to improve the performance of the code. Specifically, I have used the AHash hashing
algorithm, which is a high-speed, deterministic, keyed hash function. It provides a high-speed hash function for use in
in-memory hash maps.

In addition, the code runs in parallel using as many cores as possible via Rayon, a data-parallelism library for Rust.
This allows the code to process large amounts of data more efficiently.

While I have strived to write efficient and idiomatic Rust code, I am still learning the language. Therefore, there may
be better ways to implement the solution. I welcome any suggestions or corrections. Please feel free to open an issue if
you spot any areas for improvement.

## Generating Data

The data needed for this project can be generated according to the instructions provided in the original repository of
the One Billion Rows Challenge. You can find these instructions at the following URL:

[https://github.com/gunnarmorling/1brc](https://github.com/gunnarmorling/1brc)

Please follow the instructions in the `README.md` file of the original repository to generate the data file. Once you
have generated the data file, you can use it as input for this Rust implementation of the challenge.

## Usage

To run the code, you will need to have Rust and Cargo installed on your machine. Once you have these prerequisites, you
can clone the repository and run the code using the following commands:

```bash
git clone https://github.com/yourusername/one-billion-rows-challenge-rust.git
cd one-billion-rows-challenge-rust
cargo run -d -- /path/to/datafile.txt
```

## Contributing

This code is public and reusable. If you have any suggestions or corrections, please open an issue. I am open to all
kinds of improvements and contributions. Thank you for your interest in this project!