# BackupSync 🛡️📂
## A fast, efficient, and cross-platform backup & synchronization tool written in Rust!
<br>

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
<br>

## 💡 Why Use BackupSync?
Unlike high-level backup software, BackupSync is:
#### ✅ Lightweight — No unnecessary UI, just raw speed.
#### ✅ Optimized for Performance — Uses multi-threading and recursive traversal for fast backups.
#### ✅ Cross-Platform — Works on Windows, Linux, macOS without dependencies.
#### ✅ Open Source — Modify, extend, and contribute!

<br>

## 🚀 Installation

### 📌 Method 1: Download Prebuilt Binary (Recommended for Linux/macOS)
1. Download the latest backupsync binary from Releases.
Move it to a system-wide location:

```bash
sudo mv backupsync /usr/local/bin/
```

2. Now, you can run backupsync from anywhere!
<br>

### 📌 Method 2: Build from Source (Requires Rust)

1. Install Rust:

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

3. Move the compiled binary to /usr/local/bin/ (Linux/macOS).
<br>
<br>

### 📌 Method 3: For Windows

1. Install Rust (if not installed):
  - Download and install Rust for Windows.
 - Restart your terminal after installation. <br>
<br>

2. Clone the repository: Open PowerShell and run: <br>

```bash
git clone https://github.com/lakshyajn/BackupSync.git
cd BackupSync
```

3. Build the project:<br>
```bash
cargo build --release
```

4. Move the binary to system PATH (**Optional**, for global usage): <br>
```bash
$env:Path += ";$PWD\target\release"
```

<br>

### 🛠️ Usage

#### 🔹 Basic Backup (CLI)
Run the following command (For Linux,macOS):

```bash
backupsync <source_folder> <backup_folder>
```

<br>

### 📌 Example:

```bash
backupsync ~/Documents ~/Backup
This will incrementally copy all files from ~/Documents to ~/Backup.
```
<br>

Run the following command (For Windows):<br>
  - Once installed, you can run it from the terminal.<br>

```bash
backupsync C:\Users\YourUsername\Documents C:\Users\YourUsername\Backup
```
<br>

  - If you didn't moved it to your parent working directory (For all OSs):
```bash
cd backupsync
./target/release/backupsync C:\Users\YourUsername\Documents C:\Users\YourUsername\Backup
```

<br>

#### 🔹 Enable Compression
  - To create a .tar.gz backup instead of raw files:

```bash
backupsync ~/Documents ~/Backup tar.gz
```

  - To create a .zip backup:

```bash
backupsync  ~/Documents ~/Backup zip
```
<br>

#### 🔹 Using a Config File (config.toml)
  - Instead of typing paths every time, create a config.toml file:

```bash
source = "/home/user/Documents"
backup = "/home/user/Backup"
```

  - Then, just run:

```bash
backupsync
```
  - It will read paths from config.toml automatically.

<br>

### 📜 Backup Log (backup_log.txt)
All backup operations are logged in backup_log.txt, including copied files and errors.

```txt
[2025-02-06 12:34:56] Copied /home/user/Documents/file1.txt -> /home/user/Backup/file1.txt
[2025-02-06 12:34:58] Copied /home/user/Documents/file2.jpg -> /home/user/Backup/file2.jpg
```
<br>

### 🤖 Automating Backups
#### 🔹 Linux/macOS (crontab)
  - Open crontab:

```bash
crontab -e
```

  - Add this line to schedule a daily backup at 2 AM:

```bash
0 2 * * * /usr/local/bin/backupsync ~/Documents ~/Backup
```
<br>

#### 🔹 Windows (Task Scheduler)
1. Open Task Scheduler (Win + R, type taskschd.msc, press Enter).<br>
2. Click Create Basic Task → Name it "BackupSync".<br>
3. Set Trigger → Choose Daily or your preferred schedule.
4. Set Action → Select Start a program.
5. Browse for the backupsync.exe file in your target/release/ folder.
6. Add arguments:

```bash
"C:\Users\YourName\Documents" "D:\Backup"
```
<br>

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
<br>

### 🛠️ Contributing
Want to improve BackupSync? Feel free to:

Open an issue for feature requests.
Submit a pull request with improvements.

### Happy Backing Up! 🎉
