<div align="center">
  <h1>⏱️ zeitr ⏰</h1>
  <p>
    A lightweight time calculation utility for the command line
  </p>
  <br/>
  <a href="https://github.com/PlexSheep/zeitr/actions/workflows/cargo.yaml">
    <img src="https://img.shields.io/github/actions/workflow/status/PlexSheep/zeitr/cargo.yaml?label=Rust%20CI" alt="Rust CI"/>
  </a>
  <a href="https://github.com/PlexSheep/zeitr/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/zeitr" alt="License"/>
  </a>
  <a href="https://github.com/PlexSheep/zeitr/releases">
    <img src="https://img.shields.io/github/v/release/PlexSheep/zeitr" alt="Release"/>
  </a>
  <br/>
  <a href="https://rust-lang.org">
    <img src="https://img.shields.io/badge/language-Rust-blue.svg" alt="Rust"/>
  </a>
  <a href="https://crates.io/crates/zeitr">
    <img alt="Crates.io" src="https://img.shields.io/crates/v/zeitr">
    <img alt="Crates.io Downloads" src="https://img.shields.io/crates/d/zeitr">
  </a>
</div>

# zeitr

* [GitHub](https://github.com/PlexSheep/zeitr)
* [crates.io](https://crates.io/crates/zeitr)

A simple command-line utility for time calculations, tracking work hours, and performing time arithmetic operations. The name "zeitr" comes from the German word "Zeit" (meaning "time").

## Features

- **Time Span Calculation**: Calculate the duration between start and end times with optional pause deduction
- **Time Arithmetic**: Perform addition and subtraction with time values
- **Simple CLI Interface**: Easy-to-use command line arguments for different operations
- **Flexible Time Formats**: Support for HH:MM and HH:MM:SS formats

## Installation

### From crates.io

```bash
cargo install zeitr
```

### From source

```bash
git clone https://github.com/PlexSheep/zeitr.git
cd zeitr
cargo build --release
```

The binary will be available at `./target/release/zeitr`.

## Usage

### Calculate time span

```bash
# Calculate the time span between 9:00 and 17:30
zeitr span 09:00 17:30

# Calculate time span with a 1 hour lunch break
zeitr span 09:00 17:30 -p 01:00

# Short alias
zeitr s 09:00 17:30
```

### Time arithmetic

```bash
# Add 2 hours and 30 minutes to 13:45
zeitr calc 13:45 + 02:30

# Subtract 1 hour and 15 minutes from 17:00
zeitr calc 17:00 - 01:15

# Short alias
zeitr c 09:30 + 01:45
```

## Examples

```bash
$ zeitr span 09:00 17:30 -p 01:00
Start:    09:00:00        
End:      17:30:00        
Pause:    01:00:00        
================================================================================
Span:     07:30:00        

$ zeitr calc 13:45 + 02:30
16:15:00

$ zeitr calc 13:45 + 02:30 + 1:30 + 20:25 + 19:30
09:40:00
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

Distributed under the GPL-3.0 License. See `LICENSE` for more information.
