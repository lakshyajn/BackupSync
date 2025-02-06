# BackupSync 🛡️📂
## A fast, efficient, and cross-platform backup & synchronization tool written in Rust!
<br><br>

## ✨ Features
### ✅ Backup & Sync — Supports incremental backup, only copying files if they are newer.
### ✅ Multi-threading — Uses multiple threads for faster file copying.
### ✅ Compression Support — Backup as .tar.gz (default) or .zip (optional).
### ✅ Progress Bar — See live progress with indicatif.
### ✅ Logging — Saves backup history in backup_log.txt.
### ✅ Config File Support — Define paths in config.toml for automation.
### ✅ Cross-Platform — Works on Windows, Linux, macOS.
### ✅ CLI Friendly — Run via terminal:

<br>

```bash
backupsync ~/Documents ~/Backup 
```
<br>

### ✅ Automation Support — Schedule backups with crontab (Linux/macOS) or Task Scheduler (Windows).
<br><br>

## 🚀 Installation
<br>

### 📌 Method 1: Download Prebuilt Binary (Recommended)
1. Download the latest backupsync binary from Releases.
Move it to a system-wide location:
<br>

```bash
sudo mv backupsync /usr/local/bin/
```

<br>

2. Now, you can run backupsync from anywhere!
<br>

### 📌 Method 2: Build from Source (Requires Rust)
1. Install Rust:
<br>

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<br>

2. Clone the repository:
```bash
git clone https://github.com/YOUR_GITHUB/backupsync.git  
cd backupsync  
cargo build --release
```
<br>  

3. Move the compiled binary to /usr/local/bin/ (Linux/macOS) or C:\Windows\System32\ (Windows).
### 🛠️ Usage
<br>

#### 🔹 Basic Backup (CLI)
Run the following command:

```bash
backupsync <source_folder> <backup_folder>
```

### 📌 Example:

```bash
backupsync ~/Documents ~/Backup
This will incrementally copy all files from ~/Documents to ~/Backup.
```
<br>

#### 🔹 Enable Compression
To create a .tar.gz backup instead of raw files:

```bash
backupsync --tar ~/Documents ~/Backup
```

To create a .zip backup:

```bash
backupsync --zip ~/Documents ~/Backup
```

#### 🔹 Using a Config File (config.toml)
Instead of typing paths every time, create a config.toml file:

```bash
source = "/home/user/Documents"
backup = "/home/user/Backup"
```

Then, just run:

```bash
backupsync
```
<br>
It will read paths from config.toml automatically.

### 📜 Backup Log (backup_log.txt)
<br>
All backup operations are logged in backup_log.txt, including copied files and errors.

```txt
[2025-02-06 12:34:56] Copied /home/user/Documents/file1.txt -> /home/user/Backup/file1.txt
[2025-02-06 12:34:58] Copied /home/user/Documents/file2.jpg -> /home/user/Backup/file2.jpg
```

### 🤖 Automating Backups
#### 🔹 Linux/macOS (crontab)
<br>
Open crontab:

```bash
crontab -e
```

<br>
Add this line to schedule a daily backup at 2 AM:

```bash
0 2 * * * /usr/local/bin/backupsync ~/Documents ~/Backup
```

#### 🔹 Windows (Task Scheduler)
Open Task Scheduler → Create Basic Task.
Choose Daily and set a time (e.g., 2 AM).
Set Action to Start a program and browse to backupsync.exe.
Add arguments:
arduino

```bash
"C:\Users\YourName\Documents" "D:\Backup"
```

### 📦 Using BackupSync as a Rust Crate
If you want to integrate backupsync into your Rust project, add it to your ```Cargo.toml```:

```bash
[dependencies]
backupsync = "0.1.0"
Example Rust Code:
rust
Copy
use backupsync::backup;

fn main() {
    backup("/home/user/Documents", "/home/user/Backup").unwrap();
}

```

### 💡 Why Use BackupSync?
Unlike high-level backup software, BackupSync is:
#### ✅ Lightweight — No unnecessary UI, just raw speed.
#### ✅ Optimized for Performance — Uses multi-threading and recursive traversal for fast backups.
#### ✅ Cross-Platform — Works on Windows, Linux, macOS without dependencies.
#### ✅ Open Source — Modify, extend, and contribute!

### 🛠️ Contributing
Want to improve BackupSync? Feel free to:

Open an issue for feature requests.
Submit a pull request with improvements.

### Happy Backing Up! 🎉