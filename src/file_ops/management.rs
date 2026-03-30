use crate::file_ops::my_prelude::*;

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
