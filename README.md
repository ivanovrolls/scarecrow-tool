# Scarecrow 🐦‍⬛
A lightweight, fast CLI version manager for Node.js, Go, and Python. Built in Rust.

Scarecrow lets you install, switch, and manage multiple versions of development tools with minimal setup. Drop a `.scarecrow` file in any project directory and Scarecrow automatically switches to the right versions when you `cd` in.

---

## Installation

### Prerequisites
- Rust and Cargo installed ([rustup.rs](https://rustup.rs))

### Build from source
```bash
git clone https://github.com/ivanovrolls/scarecrow-tool
cd scarecrow-tool
cargo install --path .
```

Then run:
```bash
crow init
source ~/.zshrc  #or ~/.bashrc on Linux
```

---

## Quick Start
```bash
#install a tool
crow fetch node@18.16.0

#switch to it
crow perch node@18.16.0

#set up automatic version switching for a project
cd my-project
crow field node@18.16.0

#anyone who cds into this directory will automatically switch to node@18.16.0
```

---

## Commands

| Command | Description | Example |
|---|---|---|
| `crow fetch` | Install a tool version | `crow fetch node@18.16.0` |
| `crow perch` | Switch to an installed version | `crow perch node@20.0.0` |
| `crow drop` | Uninstall a version | `crow drop node@18.16.0` |
| `crow field` | Add a tool to the current project's `.scarecrow` file | `crow field python@3.11.15` |
| `crow refresh` | Re-apply the current directory's `.scarecrow` file | `crow refresh` |
| `crow list-all` | List all installed versions | `crow list-all` |
| `crow init` | Set up shell integration (run once) | `crow init` |

---

## Supported Tools

| Tool | Example |
|---|---|
| Node.js | `crow fetch node@18.16.0` |
| Go | `crow fetch go@1.21.0` |
| Python | `crow fetch python@3.11.15` |

---

## Shell Integration
Running `crow init` installs a shell hook into your `.zshrc` or `.bashrc`. After restarting your terminal, Scarecrow automatically switches tool versions whenever you `cd` into a directory containing a `.scarecrow` file.

To set up a project:
```bash
cd my-project
crow field node@18.16.0
crow field python@3.11.15
```

Your `.scarecrow` file will look like:
```
node@18.16.0
python@3.11.15
```

Now anyone with Scarecrow installed who `cd`s into this directory will automatically switch to these versions.

---

## How It Works
Scarecrow stores all tool versions under `~/.scarecrow/versions/` and manages a `~/.scarecrow/bin/` directory of symlinks pointing to the active version of each tool. Add `~/.scarecrow/bin` to your `$PATH` (done automatically by `crow init`) and Scarecrow handles the rest.

---

## Adding New Tools
Scarecrow is designed to be easily extensible. To add support for a new tool, add arms to `build_url`, `map_os`, `map_arch`, and `symlink` in `src/main.rs`. Pull requests welcome!

---

## Roadmap
- [ ] Checksum verification for downloads
- [ ] Version aliasing (`node@lts`)
- [ ] SDK/Java support
- [ ] Progress bar for downloads
- [ ] `crow update` command

---

## License
MIT