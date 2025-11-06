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

## Execution........
<div align=center />
<img width="534" height="301" alt="Screenshot 2025-11-06 at 10 40 09 AM" src="https://github.com/user-attachments/assets/1a17faf4-dbdf-4868-a1cb-af98687db36b" />


