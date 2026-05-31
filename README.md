Fair enough. Here's a draft - you write it up yourself though:

---

# MOS6502

A cycle-accurate emulator of the MOS Technology 6502 processor, written in Rust.

## What

An implementation of the 6502 CPU that powered the Apple II, Commodore 64, NES, and BBC Micro among others. The project covers the full instruction set, all addressing modes, and accurate memory layout including zero page, stack, and interrupt vectors.

## Why

This is a learning project. The goal is to get comfortable with Rust by working on something with well-documented, verifiable behaviour. The 6502 is small enough to be tractable but complex enough to stress the language properly - ownership comes up immediately with the CPU/memory relationship, the type system earns its keep with things like address arithmetic, and there is no shortage of edge cases to write tests for.

### Rules

- No AI-generated code. AI is used as a tutor only - to explain concepts, answer questions, and review reasoning, not to write code that gets copied in.
- No copy-pasting from existing emulator implementations.

> AI may be used to guide on broken code and missing logic, but only by explanation - not by providing the solution directly. AI may also review working code for idiomatic Rust, improving style and language use, but the code must already be the developer's own work.

## Structure

The project is a Cargo workspace with two crates:

- `cpu` - the emulator library, containing the CPU struct, memory, instruction set, addressing modes, and status flags
- `main` - the binary entry point

## Running

```bash
cargo run
```

```bash
cargo test
```
