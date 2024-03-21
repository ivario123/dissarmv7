<h1 align="center">
  Assembly language for Disarmv7
</h1>

This crate defines the high-level representation of the parsed instructions from [`disarmv7`](../).
Moreover, it exports Builders and what we are calling a consumer which ensures that all fields of a struct are extracted,
this is probably not generally useable but in this case, it ensures that the translators are explicit about what fields are not used.

## Syntax

To extend or modify the intermediate language it is important to note the syntax

```rust
operation!(
    AdcImmediate {s:bool}, {rd: Register}, <rn: Register>, <imm:u32>
);
```

Take the `AdcImmediate` instruction above, it defines 3 structs and adds `AdcImmediate` to the `Operation` enum.

```rust
struct AdcImmediate {
    s: Option<bool>,
    rd: Option<Register>,
    rn: Register,
    imm: u32
}

struct AdcImmediateBuilder<const sSET: bool, const rdSET: bool, const rnSET: bool, const immSET: bool> {
    s: Option<Option<bool>>,
    rd: Option<Option<Register>>,
    rn: Option<Register>,
    imm: Option<u32>
}

pub struct AdcImmediateConsumer<const sSET: bool, const rdSET: bool, const rnSET: bool, const immSET: bool> {
    s: Option<Option<bool>>,
    rd: Option<Option<Register>>,
    rn: Option<Register>,
    imm: Option<u32>
}
```

Where the Builder and Consumers allow the fields to be set/consumed once and only once using the const flags.
The fields initiated with `{<id>: <ty>}` are optional and the fields initiated with `<<id>: <ty>>`
are mandatory.

## Future improvements

These are not planned, but any contributions are welcome.

- [ ] Add in floating point instructions.
- [ ] Add documentation for the instructions.

## License

This repository is licensed under the [`MIT`](../LICENSE) license and any contributions shall be licensed under the same license unless explicitly stated otherwise.
