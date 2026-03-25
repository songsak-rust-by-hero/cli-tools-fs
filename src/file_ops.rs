use anyhow::{Context, Result};
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn read_file(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path).context("ไม่มีไฟล์ที่ต้องการ")?;
    Ok(content)
}

pub fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    fs::write(path, format!("{content}\n")).context("เขียนไฟล์ไม่ได้")?;
    Ok(())
}

pub fn delete_file(path: &PathBuf) -> Result<()> {
    fs::remove_file(path).context("ไม่สามารถลบไฟล์ได้")?;
    Ok(())
}

pub fn create_file(path: &PathBuf) -> Result<()> {
    File::create(path).context("สร้างไฟล์ไม่ได้")?;
    Ok(())
}

pub fn file_exists(path: &PathBuf) {
    if Path::new(path).exists() {
        println!("เจอไฟล์");
    } else {
        println!("ไม่มีไฟล์");
    }
}

pub fn append_file(path: &PathBuf, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .context("เปิดไฟล์ไม่ได้")?;

    writeln!(file, "{content}").context("เขียนไฟล์ไม่ได้")?;
    Ok(())
}

pub fn file_size(path: &PathBuf) -> Result<()> {
    let meta = fs::metadata(path).context("อ่านข้อมูลไม่ได้")?;
    let size = meta.len() as f64;

    if size < 1024.0 {
        println!("ไฟล์นี้ขนาด {} bytes", size);
    } else if size < 1024.0 * 1024.0 {
        println!("ไฟล์นี้ขนาด {:.2} KB", size / 1024.0);
    } else {
        println!("ไฟล์นี้ขนาด {:.2} MB", size / (1024.0 * 1024.0));
    }
    Ok(())
}

pub fn copy_file(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::copy(src, dst).context("ไฟล์ไม่มีอยู่จริง")?;
    Ok(())
}
pub fn move_file(src: &PathBuf, dst: &PathBuf) -> Result<()> {
    fs::rename(src, dst).context("ย้ายไฟล์ไม่ได้")?;
    Ok(())
}
pub fn list_dir(path: &PathBuf) -> Result<()> {
    let entries = fs::read_dir(path).context("เปิดโฟลเด้อไม่ได้")?;
    for entry in entries {
        let entry = entry.context("อ่านไฟล์ไม่ได้")?;
        println!("{}", entry.file_name().to_string_lossy());
    }
    Ok(())
}

pub fn make_dir(path: &PathBuf) -> Result<()> {
    fs::create_dir(path).context("สร้างไฟล์ไม่ได้")?;
    Ok(())
}

pub fn count_lines(path: &PathBuf) -> Result<()> {
    let content = read_file(path)?;
    println!("{}", content.lines().count());
    Ok(())
}
pub fn search_file(path: &PathBuf, keywold: &str) -> Result<()> {
    let content = read_file(path)?;
    for line in content.lines() {
        if line.contains(keywold) {
            println!("{line}");
        }
    }
    Ok(())
}
pub fn run_tree(path: &PathBuf, indent: String, is_last: bool, all: bool) -> Result<()> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_else(|| ".".into());

    let marker = if is_last { "└── " } else { "├── " };

    println!("{}{}{}", indent, marker, name);

    if path.is_dir() {
        let entries: Vec<_> = fs::read_dir(path)
            .with_context(|| format!("ไม่สามารถอ่านโฟลเดอร์: {:?}", path))?
            .filter_map(|e| e.ok())
            .filter(|entry| {
                if all {
                    return true;
                }
                let name = entry.file_name().to_string_lossy().to_string();

                !name.starts_with(".") && name != "target"
            })
            .collect();

        let count = entries.len();

        for (i, enty) in entries.into_iter().enumerate() {
            let is_last_enty = i == count - 1;

            let new_indent = format!("{}{}", indent, if is_last { "    " } else { "│   " });

            run_tree(&enty.path(), new_indent, is_last_enty, all)?;
        }
    }
    Ok(())
}
