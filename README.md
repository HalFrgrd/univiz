# Univiz

A command-line tool for analyzing Unicode strings, providing detailed information about graphemes, code points, and UTF-8 byte sequences.

## Installation

### Quick install: `install.sh`

> [!TIP]
> Run the following command to download univiz. No need for `sudo`!
```bash
curl -sSfL https://github.com/halfrgrd/univiz/releases/latest/download/install.sh | sh
```

### Via Cargo

```bash
cargo install univiz
```

## Usage

```bash
univiz "your string here"
```

## Example Output

![Univiz Demo](demo/univiz-demo.gif)

## Features

- Grapheme cluster analysis
- Code point information with Unicode values
- UTF-8 byte sequence breakdown
- Display width calculation
- Detailed byte indexing (local and global)
