# Ember OS

> ## ❤️ Please give me a `Star` / `Follow` if you like this project! ❤️

## Brief

A simple OS implemented in rust, referenced `Philipp Oppermann`'s `Writing an OS in Rust blog`.

## Referencing Info

This project includes(referenced) code from [phil-opp](https://github.com/phil-opp)'s [blog_os](https://github.com/phil-opp/blog_os), which is available under the [MIT LICENSE](https://github.com/phil-opp/blog_os/blob/main/LICENSE-MIT) and the [APACHE LICENSE](https://github.com/phil-opp/blog_os/blob/main/LICENSE-APACHE). The original code can be found at [phil-opp/blog_os](https://github.com/phil-opp/blog_os). Main extensions (differences between `this` project and `phil-opp`'s) are as follows:

1. Full implementation of the `Asynchronous Task Manager`
2. Simple implementation of the `Shell`
3. Fully transplanted `benchmarks` from `NJU-OS-Experiment`

## Build

With the reliance on a bunch of `unstable features`, `nightly` channel of `rust-toolchain` is in need. A simple way is to run `rustup update nightly --force`.

Obviously, you should have `qemu` installed first. You could do that with the help of `brew`:

```bash
brew install qemu
```

Last but not least, install [`bootimage`](https://crates.io/crates/bootimage) so that you could create a legal boot disk image from the complied kernel:

```bash
cargo install bootimage
```

Finally, you could build the project by running:

```bash
cargo build
```

And then, you could run it:

```bash
cargo run
```

Or, run some given tests:

```bash
cargo test
```

(Yes, `cargo bootimage` is not a necessary step, as the build behavior under this project has been adjusted to do that automatically before `running`)

## Acknowledgements

- [https://github.com/phil-opp/blog_os]
- [https://os.phil-opp.com/]
