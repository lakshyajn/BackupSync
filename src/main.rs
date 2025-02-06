use std::env;
use std::fs::{self, OpenOptions, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use walkdir::WalkDir;
use indicatif::{ProgressBar, ProgressStyle};
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::{Builder, Header};
use zip::write::FileOptions;
use zip::ZipWriter;
use toml::Value;

/// Log messages to a file
fn log_message(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("backup_log.txt")
        .unwrap();
    writeln!(file, "{}", message).unwrap();
}

/// Read source and backup paths from `config.toml`
fn get_paths_from_config() -> Option<(PathBuf, PathBuf, String)> {
    if let Ok(config_str) = fs::read_to_string("config.toml") {
        if let Ok(config) = config_str.parse::<Value>() {
            if let (Some(source), Some(backup), Some(format)) = (
                config.get("source")?.as_str(),
                config.get("backup")?.as_str(),
                config.get("format")?.as_str(),
            ) {
                return Some((PathBuf::from(source), PathBuf::from(backup), format.to_string()));
            }
        }
    }
    None
}

/// Multi-threaded incremental backup with progress bar
fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    let (tx, rx) = mpsc::channel::<(PathBuf, PathBuf)>(); 
    let rx = Arc::new(Mutex::new(rx));

    let num_threads = 4;
    let mut handles = vec![];

    // Progress bar
    let total_files = WalkDir::new(src)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .count();

    let progress_bar = Arc::new(ProgressBar::new(total_files as u64));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {wide_bar} {percent}% ({pos}/{len})")
            .unwrap()
            .progress_chars("#>-"),
    );

    // Worker threads
    for _ in 0..num_threads {
        let rx = Arc::clone(&rx);
        let progress_bar = Arc::clone(&progress_bar);

        let handle = thread::spawn(move || {
            while let Ok((src_file, dst_file)) = rx.lock().unwrap().recv() {
                if should_copy_file(&src_file, &dst_file) {
                    if let Err(e) = fs::copy(&src_file, &dst_file) {
                        eprintln!("Failed to copy {:?} -> {:?}: {}", src_file, dst_file, e);
                    } else {
                        log_message(&format!("Copied {:?} -> {:?}", src_file, dst_file));
                    }
                }
                progress_bar.inc(1);
            }
        });
        handles.push(handle);
    }

    // Traverse directory and send copy tasks
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let rel_path = entry.path().strip_prefix(src).unwrap();
        let target_path = dst.join(rel_path);

        if entry.path().is_dir() {
            fs::create_dir_all(&target_path)?;
        } else {
            tx.send((entry.path().to_path_buf(), target_path)).unwrap();
        }
    }

    drop(tx);
    for handle in handles {
        handle.join().unwrap();
    }

    progress_bar.finish_with_message("Backup completed successfully!");
    Ok(())
}

/// Incremental backup: only copy if source file is newer
fn should_copy_file(src: &Path, dst: &Path) -> bool {
    if !dst.exists() {
        return true;
    }

    let src_metadata = fs::metadata(src).ok();
    let dst_metadata = fs::metadata(dst).ok();

    if let (Some(src_meta), Some(dst_meta)) = (src_metadata, dst_metadata) {
        if let (Ok(src_mod), Ok(dst_mod)) = (src_meta.modified(), dst_meta.modified()) {
            return src_mod > dst_mod;
        }
    }
    true
}

/// Compress backup as `.tar.gz`
fn backup_as_tar_gz(source: &PathBuf, backup: &PathBuf) -> io::Result<()> {
    let tar_path = backup.join("backup.tar.gz");
    let tar_gz = File::create(&tar_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);

    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(source)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        if path.is_file() {
            let mut file = File::open(path)?;
            let mut header = Header::new_gnu();
            header.set_size(entry.metadata()?.len());
            header.set_mode(0o644);
            tar.append_file(relative_path, &mut file)?;
        }
    }

    tar.finish()?;
    println!("Backup saved as {:?}", tar_path);
    Ok(())
}

/// Compress backup as `.zip`
fn backup_as_zip(source: &PathBuf, backup: &PathBuf) -> io::Result<()> {
    let zip_path = backup.join("backup.zip");
    let zip_file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        let relative_path = path
            .strip_prefix(source)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?
            .to_str()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid UTF-8 in file path"))?;

        if path.is_file() {
            let options: FileOptions<()> = FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o644);

            zip.start_file(relative_path, options)?;
            let mut file = File::open(path)?;
            std::io::copy(&mut file, &mut zip)?;
        }
    }

    zip.finish()?;
    println!("Backup saved as {:?}", zip_path);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (source, backup, format) = if args.len() >= 3 {
        (PathBuf::from(&args[1]), PathBuf::from(&args[2]), args.get(3).cloned().unwrap_or_else(|| "tar.gz".to_string()))
    } else if let Some(config) = get_paths_from_config() {
        config
    } else {
        eprintln!("Usage: {} <source_folder> <backup_folder> [tar.gz|zip]", args[0]);
        std::process::exit(1);
    };

    println!("Backing up {:?} to {:?}", source, backup);
    fs::create_dir_all(&backup).expect("Failed to create backup directory");

    match format.as_str() {
        "tar.gz" => backup_as_tar_gz(&source, &backup).unwrap(),
        "zip" => backup_as_zip(&source, &backup).unwrap(),
        _ => {
            println!("No format specified. Performing incremental backup...");
            copy_dir_recursive(&source, &backup).unwrap();
        }
    }
}