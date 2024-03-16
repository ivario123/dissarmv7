<h1 align="center">
  Disarmv7
</h1>

Disarmv7 is a disassembler for the Arm®V7-M instruction set. It provides a fast enough disassembler that lifts [most$^1$](#foot-notes) of the Arm®V7-m instructions to a rust `enum` which is ideal if you want to do semantic analysis, [`symbolic execution`](https://github.com/ivario123/symex) or similar on
the assembly/machine code level. As of now it does not provide a textual representation of the assembly instructions.
This project is mainly written as a support project for the [`Symex`](https://github.com/ivario123/symex) project which is a symbolic execution engine.

## Usage

### Prerequisites

- As this program is written in rust, you do need to install rust and cargo a fast way of doing this is by using [`rustup`](https://rustup.rs/).
- A suitable binary from which the code sections have been extracted [in some manner$^2$](#foot-notes).

Finally add the dependency to your `Cargo.toml`

```toml
[dependencies]
disarmv7 = {git = "https://github.com/ivario123/disarmv7"}
```

### Using the library

Assuming that you have placed the instructions in an slice of `u8`s (buff) you can call the disassembler like this

```rust
use disarmv7::prelude::*;

let mut buff: disarmv7::buffer::PeekableBuffer<u8, _> = buff.iter().cloned().into();
let asm = Asm::parse(&mut buff).map_err(|e| ArchError::ParsingError(e.into()))?;
println!("Assembly : {asm:?}");
```

## Limitations

This project does not load binaries, nor does it generate a textual representation of the assembly, this is outside of the scope for the project,
and for things that require textual representations of the program we refer the user to projects like [`Capstone`](https://github.com/capstone-engine/capstone) or [`Binary Ninja`](https://binary.ninja/) which provide a more complete experience.

## License



## Foot notes

- [1] The disassembler does not support the floating point extensions as of now.
- [2] For further documentation on this please read the documentation for [`Object`](https://docs.rs/object/latest/object/).
