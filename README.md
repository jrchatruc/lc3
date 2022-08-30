# LC-3

A basic implementation of an [LC-3](https://en.wikipedia.org/wiki/Little_Computer_3) VM in Rust, following [Write your own virtual machine](https://justinmeiners.github.io/lc3-vm).

## Examples

To create a `Hello world` object file, run

```
cargo run --example hello_world
```
You should now have a `hello_world.obj` file under `./examples`, which you can then run on the VM with

```
cargo run --release -- ./examples/hello_world.obj
```

and get a

```
Hello world!
```

For more interesting examples, you can download the assembled versions of [2048](https://justinmeiners.github.io/lc3-vm/supplies/2048.obj) and [Rogue](https://justinmeiners.github.io/lc3-vm/supplies/rogue.obj) and run them in the same way, e.g., assuming you download `2048.obj` to the repository's root directory:

```
cargo run --release -- 2048.obj
```

## Disassembly

You can disassemble code by passing `--disassemble` as the first argument:

```
cargo run -- --disassemble ./examples/hello_world.obj
```

which, in this example, should output the following:

```
0x3000 1110 0000 0000 0010 LEA R0 0x3003
0x3001 1111 0000 0010 0100 TRAP PUTSP
0x3002 1111 0000 0010 0101 TRAP HALT
0x3003 0110 0101 0100 1000 LDR R2 R5 0x8
0x3004 0110 1100 0110 1100 LDR R6 R1 0xFFEC
0x3005 0010 0000 0110 1111 LD R0 0x3075
0x3006 0110 1111 0111 0111 LDR R7 R5 0xFFF7
0x3007 0110 1100 0111 0010 LDR R6 R1 0xFFF2
0x3008 0010 0001 0110 0100 LD R0 0x2F6D
0x3009 0000 0000 0000 1010 BR 0x3014
```

Do note, however, that this disassembler does not distinguish between code and data. In the example above, everything below
```
0x3002 1111 0000 0010 0101 TRAP HALT
```
is actually just data (it's the null terminated string `Hello world!\n`), but because (as far as I know) there are no agreed upon code and data sections in an LC3 object file, the disassembler will just interpret everything as code.

## Other references

- [Writing a simple 16 bit VM in less than 125 lines of C](https://www.andreinc.net/2021/12/01/writing-a-simple-vm-in-less-than-125-lines-of-c)
- [LC-3 ISA](https://justinmeiners.github.io/lc3-vm/supplies/lc3-isa.pdf)
