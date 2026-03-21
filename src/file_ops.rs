use anyhow::{Context, Result};
use std::fs;
use std::fs::File;
use std::path::{Path,PathBuf};


pub fn read_file(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path).context("ไม่มีไฟล์ที่ต้องการ")?;
    Ok(content)
}

pub fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    fs::write(path, content).context("เขียนไฟล์ไม่ได้")?;
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

