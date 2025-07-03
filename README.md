# heystupid

A simple command-line tool that sends prompts to OpenAI's API for quick answers and explanations. Perfect for getting concise help with command outputs, error messages, or general questions directly from your terminal.

## Features

- Send text prompts directly to OpenAI
- Pipe command output for analysis and explanation
- Configurable OpenAI model selection
- Designed for concise, terminal-friendly responses
- No special formatting - just clean, readable output

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [releases page](https://github.com/krets/heystupid/releases):

- **Linux x86_64**: `linux-x86_64/heystupid`
- **Windows x86_64**: `windows-x86_64/heystupid.exe`

Make the binary executable (Linux/macOS):
```bash
chmod +x heystupid
```

Move to your PATH:
```bash
# Linux/macOS
sudo mv heystupid /usr/local/bin/

# Or add to your user bin directory
mkdir -p ~/.local/bin
mv heystupid ~/.local/bin/
```

### Build from Source

Requirements:
- Rust 1.70+
- Cargo

```bash
git clone https://github.com/krets/heystupid.git
cd heystupid
cargo build --release
```

The binary will be available at `target/release/heystupid`

## Configuration

Create a configuration file in your home directory:

```bash
echo 'OPENAI_API_KEY=your_openai_api_key_here' > ~/.heystupid
```

Get your OpenAI API key from: https://platform.openai.com/api-keys

## Usage

### Basic Usage

```bash
# Ask a direct question
heystupid "What is the difference between TCP and UDP?"

# Use a different model
heystupid --model gpt-4 "Explain quantum computing"
```

### Piping Command Output

```bash
# Analyze command output
ls -la | heystupid "What files are taking up the most space?"

# Debug error messages
make 2>&1 | heystupid "What's wrong with this build?"

# System information
cat /etc/os-release | heystupid "What OS is this?"

# Process analysis
ps aux | heystupid "Are there any concerning processes?"
```

### Combined Input

```bash
# Combine piped input with additional context
dmesg | tail -20 | heystupid "Are there any hardware issues?"
```

## Options

- `--model`: Specify OpenAI model (default: gpt-4o-mini)
- `--help`: Show help information
- `--version`: Show version information

## Examples

```bash
# Quick explanations
heystupid "How do I find large files in Linux?"

# Error analysis
gcc program.c 2>&1 | heystupid "Help me fix these compilation errors"

# System diagnostics
df -h | heystupid "Do I have disk space issues?"

# Log analysis
tail -n 50 /var/log/syslog | heystupid "Any concerning log entries?"
```

## Dependencies

- OpenAI API access and valid API key
- Internet connection for API requests

## Building for Distribution

```bash
# Linux x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Windows x86_64 (from Linux with cross-compilation)
cargo build --release --target x86_64-pc-windows-gnu
```

## License

MIT License - see LICENSE file for details

## Contributing

Issues and pull requests welcome! Please feel free to contribute improvements or report bugs.
