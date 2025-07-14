# Dir Info 📁

A colorful Rust CLI tool that provides detailed information about directories including file sizes, permissions, creation dates, and file type statistics.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![CLI](https://img.shields.io/badge/CLI-Tool-blue?style=for-the-badge)

## ✨ Features

- 🎨 **Colorful Terminal Interface** - Files and folders displayed with different colors and icons
- 📏 **Configurable Search Depth** - Control how deep to search in subdirectories
- 📊 **File Information** - Shows file sizes, permissions (read-only/read-write), and creation dates
- 📈 **Summary Statistics** - Displays total size, folder count, and file type breakdown
- 🚀 **Fast Performance** - Built with efficient Rust libraries for quick directory traversal

## 🛠️ Installation

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable version)

### Build from Source
```bash
git clone https://github.com/mapcrafter2048/dir_info.git
cd dir_info
cargo build --release
```

The executable will be available at `target/release/dir_info` (or `dir_info.exe` on Windows).

## 🚀 Usage

### Basic Usage
```bash
# Analyze current directory (default depth 1)
./dir_info

# Analyze current directory with depth 2
./dir_info --depth 2

# Analyze specific directory
./dir_info /path/to/directory

# Analyze specific directory with custom depth
./dir_info /path/to/directory --depth 3
```

### Command Line Options
```
Usage: dir_info [OPTIONS] [PATH]

Arguments:
  [PATH]  The path to the directory to inspect [default: .]

Options:
  -d, --depth <DEPTH>  The depth of subdirectories to search [default: 1]
  -h, --help           Print help
  -V, --version        Print version
```

## 📋 Example Output

```
Inspecting directory: '.'
Search depth: 2
--------------------------------------------------
📁 src
  📄 main.rs 3.5 KB (rw) 2025-07-14 22:26 [rs]
📁 target
  📄 .rustc_info.json 1.1 KB (rw) 2025-07-14 22:30 [json]
  📄 CACHEDIR.TAG 177 B (rw) 2025-07-14 22:30 [TAG]
📄 Cargo.toml 185 B (rw) 2025-07-14 22:26 [toml]
📄 Cargo.lock 14.2 KB (rw) 2025-07-14 22:30 [lock]
--------------------------------------------------
Total Size: 2.4 MB
Total Folders: 17
File Types:
  - rs: 1
  - json: 1
  - toml: 1
  - lock: 1
  - TAG: 1
```

## 🏗️ Built With

- **[clap](https://crates.io/crates/clap)** - Command line argument parsing
- **[colored](https://crates.io/crates/colored)** - Terminal color support
- **[chrono](https://crates.io/crates/chrono)** - Date and time handling
- **[walkdir](https://crates.io/crates/walkdir)** - Efficient directory traversal

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is open source and available under the [MIT License](LICENSE).

## 👤 Author

**Aadish Jain**
- GitHub: [@mapcrafter2048](https://github.com/mapcrafter2048)
- Email: aadish.jinesh.jain@gmail.com

---

⭐ If you found this project useful, please consider giving it a star!
