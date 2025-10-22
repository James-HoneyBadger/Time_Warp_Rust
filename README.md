# Time Warp IDE

A modern, educational programming environment built in Rust using the egui framework.

## Features

- **Multi-Language Support**: Execute code in multiple educational programming languages:
  - Time Warp (unified educational language)
  - BASIC (line-numbered programming)
  - Pascal (structured programming)
  - Prolog (logic programming)
  - Logo (turtle graphics)
  - PILOT (frame-based programming)

- **Turtle Graphics**: Visual programming with Logo-style turtle graphics
- **Code Editor**: Full-featured editor with:
  - Line numbers
  - Find/Replace functionality
  - Syntax checking
  - Undo/Redo support

- **Educational Focus**: Designed for teaching programming concepts with clear error messages and visual feedback

## Building and Running

### Prerequisites
- Rust 1.70 or later
- System dependencies for egui (varies by platform)

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
```

## Supported Languages

### Time Warp
A unified educational language combining features from multiple programming paradigms.

Example:
```
PRINT "Hello, Time Warp!"
```

### BASIC
Classic line-numbered BASIC programming.

Example:
```
10 PRINT "Hello, BASIC!"
20 LET X = 42
30 PRINT X
```

### Logo
Turtle graphics programming.

Example:
```
FORWARD 100
RIGHT 90
FORWARD 50
```

### Pascal
Structured programming with clear syntax.

Example:
```
writeln('Hello, Pascal!');
```

## Architecture

The IDE is built using:
- **egui**: Immediate mode GUI framework
- **eframe**: App framework for egui
- **rfd**: Native file dialogs

The interpreter is implemented as a native Rust module with separate execution logic for each supported language.

## Contributing

This is an educational project focused on teaching programming concepts through multiple language paradigms.

## License

Educational use encouraged.