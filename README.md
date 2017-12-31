# `ds4`

This library implements a userspace DualShock 4 driver on top of [signal11's hidapi](https://github.com/signal11/hidapi) project.

---

### Project status

In development. Unusable.

---

### Project goals

The main goals for this project are as follows:

1) Learn to write a userspace driver over [HID](https://en.wikipedia.org/wiki/Human_interface_device)
2) ???
3) Profit!

---

### Building

You will need the [Rust](https://www.rust-lang.org/en-US/) toolchain installed (which you can easily get via [rustup](https://rustup.rs)).

Clone the source code like so:

```
$ git clone git@github.com:Phrohdoh/ds4-rs.git
```

Then `cd` into the newly-created directory and build the project via `cargo` (read about `cargo` [here](http://doc.crates.io/index.html)):

```
$ cd ds4-rs
$ cargo build
```

---

### Running

Currently the project contains both a `lib` and `bin` target which means you can use `ds4` as a library or run it as an executable.

```
$ cargo run
```

---

### Running tests

```
$ cargo test
```