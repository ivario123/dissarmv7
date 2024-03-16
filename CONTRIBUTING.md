<h1 align="center">
  Contributing to Disarmv7
</h1>

## Crate structure

This project is split in to three main crates

```md
| disarmv7
|- arch
|- operation
```

Where the `operation` crate defines the assembly language that the binary code is lifted in to.
The `arch` crate defines the data types specific to the ArmV7 ISA.
The main crate `disarmv7` defines parsing rules and some helpers that simplify the parsing rules.


## Main crate

The main crate is divided in to a main files

```md
|- src
|  |- lib.rs        # Defines the re-exports and some high level traits and structs.
|  |- decoder.rs    # This should probably be moved and will be moved after som Symex restructure.
|  |- buffer.rs     # Defines a peekable buffer.
|  |- helpers.rs    # Defines a few helpers for internal use, these are macros that hide implementation details.
|  |- asm           # Defines the parser details
```

### Parser structure

The [`parser`](./src/asm/) defines the parsing rules for the [`Operation`](./operation) crate, these parsing rules are split in to which table they belong
to in the [`documentation`](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwjc6YCk0fiEAxUSLhAIHU-1BY8QFnoECBQQAQ&url=https%3A%2F%2Fdocumentation-service.arm.com%2Fstatic%2F5f8fef3af86e16515cdbf816%3Ftoken%3D&usg=AOvVaw1Pwok2Ulie5wtDRP5IwyNw&opi=89978449). Moreover these parsing rules are split in to wether the instruction is a 16 bit instruction 32 bit instruction.
 
## Testing

Each positive case in the [`parser`](./src/lib.rs) needs to be tested, as there are too many negative cases they cannot, in a useful manner, be tested
therefore the negative cases can be omitted in some cases.
Note that since there are way to many combinations of positive cases one cannot test all combinations of positive cases, so focus on the edge cases.

## Pull request format

This is not a required format but for people who are not used to writing pull requests it might be a nice to have.

```md
# A concise title that describes what you did

A brief introduction to why the change is needed

## How did I test this solution


## What is left to do ( if any )

- [ ] This should preferably be a list of checkboxes

## Side-effects if any

### Codebase wise

Did this have any side-effect on the functionality in the repository? Have these been tested?
Why should these be accepted?

### Project wise

The pull request can auto close issues for example

Closes #10
```
