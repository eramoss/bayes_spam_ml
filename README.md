# Bayesian Spam Filter CLI

This project implements a simple Bayesian spam filter in Rust, allowing users to train the filter with labeled data and predict whether a given message is spam or not. The filter uses a basic probability-based approach to classify messages as spam or ham.


### Example

Here's an example of a training file:

```plaintext
Hello, this is a normal message.0
Buy our amazing products now!1
Another regular message.0
Click here for exclusive offers.1
```

## Dependencies

This project uses the following external crate:

- [clap](https://crates.io/crates/clap): Command-line argument parsing for Rust.

## Build and Run

To build and run the project, make sure you have Rust installed. Then, use the following commands:

```bash
cargo build
cargo run -- [arguments]
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---