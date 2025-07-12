# heystupid

A simple command-line tool that sends prompts to OpenAI's API for quick answers and explanations. Perfect for getting concise help with command outputs, error messages, or general questions directly from your terminal.

## Features

- Send text prompts directly to OpenAI
- Pipe command output for analysis and explanation
- Configurable OpenAI model and base prompt selection
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

## Configuration

Create a configuration file named `.heystupid.config` in your home directory.
Use the following key=value format (no quotes, no spaces around `=`):

```
openai_api_key=your_openai_api_key_here
model=gpt-4o-mini
base_prompt=Your custom base prompt here
```

- `openai_api_key` (required): Your OpenAI API key.
- `model` (optional): OpenAI model to use. Defaults to `gpt-4.1-mini` if unspecified.
- `base_prompt` (optional): Base prompt providing context to OpenAI.
  If not set, a default prompt that instructs concise and clean terminal-friendly responses
  is used.

Get your OpenAPI key from: https://platform.openai.com/api-keys

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

- `--model`: Specify OpenAI model (overrides config)
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

## License

MIT License - see LICENSE file for details

## Contributing

Issues and pull requests welcome! Please feel free to contribute improvements or report bugs.
