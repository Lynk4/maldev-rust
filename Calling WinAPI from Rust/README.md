# Calling WinAPI from Rust

---


## Cross-Compilation Setup (Linux/macOS → Windows)

```bash
# 1. Install Windows target
rustup target add x86_64-pc-windows-gnu   # MinGW (no MSVC needed)
# or: rustup target add x86_64-pc-windows-msvc  # if you have MSVC

# 2. Install linker (Linux example)
sudo apt install gcc-mingw-w64-x86-64

# 3. Build
cargo build --target x86_64-pc-windows-gnu --release

```

---

