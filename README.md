# LC-3

A basic implementation of an [LC-3](https://en.wikipedia.org/wiki/Little_Computer_3) VM in Rust, following [Write your own virtual machine](https://justinmeiners.github.io/lc3-vm).

## Examples

To create a `Hello world` object file, run

```
$ cargo run --example hello_world
```
You should now have a `hello_world.obj` file under `./examples`, which you can then run on the VM with

```
$ cargo run --release ./examples/hello_world.obj
Hello world!
```

For more interesting examples, you can download the assembled versions of [2048](https://justinmeiners.github.io/lc3-vm/supplies/2048.obj) and [Rogue](https://justinmeiners.github.io/lc3-vm/supplies/rogue.obj) and run them in the same way, e.g., assuming you download `2048.obj` to the repository's root directory:

```
$ cargo run --release 2048.obj
```

Caveat:
- The VM needs to be run with the `--release` flag because it relies on integer overflow, running it in `debug` mode can panic with messages like `thread 'main' panicked at 'attempt to add with overflow'`. This should be resolved soon™.

## Other references

- [Writing a simple 16 bit VM in less than 125 lines of C](https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c)
- [LC-3 ISA](https://justinmeiners.github.io/lc3-vm/supplies/lc3-isa.pdf)