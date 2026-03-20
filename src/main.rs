use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;

#[derive(Parser)]
struct Cil {
    path: PathBuf,
}

fn read_file(path: &PathBuf) -> Result<String> {
    let content = fs::read_to_string(path).context("ไม่มีไฟล์ที่ต้องการ")?;
    Ok(content)
}

fn write_file(path: &PathBuf, content: &str) -> Result<()> {
    fs::write(path, content).context("เขียนไฟล์ไม่ได้")?;
    Ok(())
}

fn delete_file(path: &PathBuf) -> Result<()> {
    fs::remove_file(path).context("ไม่สามารถลบไฟล์ได้")?;
    Ok(())
}

fn create_file(path: &PathBuf) -> Result<()> {
    File::create(path).context("สร้างไฟล์ไม่ได้")?;
    Ok(())
}

fn file_exists(path: &PathBuf) {
   if Path::new(path).exists(){
       println!("เจอไฟล์");
   } else {
       println!("ไม่มีไฟล์");
   }
}

fn main() -> Result<()> {
    let arge = Cil::parse();
    create_file(&arge.path)?;
    println!("สร้างไฟล์สำเร็จ");
    let content = read_file(&arge.path)?;
    let new_content = format!("{}\n -- username --", content);
    write_file(&arge.path, &new_content)?;
    println!("เขียนเพิ่มสำเร็จ");
    delete_file(&arge.path)?;
    println!("ลบไฟล์สำเร็จ");
    file_exists(&arge.path);
    Ok(())
}
