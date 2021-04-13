![Build](https://github.com/miried/mohrust/workflows/Rust/badge.svg)
# mohrust

Rust implementations for game libraries running with ioq3.

Currently, only supporting the UI lib.

![Screenshot](screenshot/openmohaa.png)

## Getting Started

Just run `cargo build` to get the library built.

### Prerequisites

```
ioquake3
```
See [ioquake3](https://github.com/ioquake/ioq3) for further instructions. You don't need to build the game libraries, the standalone client is enough.

It also expects the game files from MOHAA in the `main` directory.


### Installing

Currently, must be done manually. Copy the built shared library to the game's `main` folder so they are found.

## Built With

* [Rust](https://www.rust-lang.org/) - Rust programming language

## Contributing

Please read [CONTRIBUTING.md]() for details on our code of conduct, and the process for submitting pull requests to us.

## Authors

* **Michael Rieder** - *Initial work* - [miried](https://github.com/miried)

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* The Rust community
* John Carmack
