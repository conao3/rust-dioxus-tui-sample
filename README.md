# Dioxus TUI Sample

A collection of examples demonstrating [Dioxus](https://dioxuslabs.com/) for building terminal user interfaces in Rust.

## Overview

This project provides practical examples for learning Dioxus TUI, covering RSX syntax, component patterns, state management with hooks, and event handling in the terminal.

## Requirements

- Rust 1.60 or later
- Cargo

## Getting Started

Clone the repository and run the main application:

```bash
git clone https://github.com/conao3/rust-dioxus-tui-sample.git
cd rust-dioxus-tui-sample
cargo run
```

## Examples

### RSX Basics

**Simple RSX** - Basic RSX syntax with string interpolation and conditionals:

```bash
cargo run --example rsx-simple
```

**Components** - Creating reusable components with props:

```bash
cargo run --example rsx-components
```

**Iteration** - Rendering lists with iterators:

```bash
cargo run --example rsx-iter
```

### Hooks

**useState** - Managing component state:

```bash
cargo run --example hooks-usestate
```

**useFuture** - Handling async operations with automatic updates:

```bash
cargo run --example hooks-usefuture
```

### Terminal Events

**Event Handling** - Mouse, keyboard, wheel, and focus events:

```bash
cargo run --example tui-events
```

## Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| dioxus | 0.2.4 | Core framework |
| dioxus-html | 0.2.1 | HTML elements |
| dioxus-tui | 0.2.2 | Terminal renderer |
| tokio | 1.23.0 | Async runtime |

## Resources

- [Dioxus Documentation](https://dioxuslabs.com/docs/0.3/guide/en/)
- [Dioxus GitHub](https://github.com/DioxusLabs/dioxus)

## License

MIT
