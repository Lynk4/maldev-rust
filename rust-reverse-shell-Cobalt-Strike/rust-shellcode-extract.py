import sys

fn = sys.argv[1] if len(sys.argv) > 1 else "payload_x64.bin"
with open(fn, "rb") as f:
    data = f.read()

print("let payload: &[u8] = &[", end='')
print(', '.join(f"0x{b:02x}" for b in data), end='')
print("];")
