use crate::file_ops::my_prelude::*;

pub fn file_size(path: &Path) -> Result<()> {
    let meta = fs::metadata(path).with_context(|| format!("ไม่สามารถอ่านข้อมูลได้: {:?}", path))?;
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

pub fn copy_file(src: &Path, dst: &Path) -> Result<()> {
    fs::copy(src, dst).with_context(|| format!("ไม่สามารถ copy ไฟล์จาก {:?} ไปยัง {:?}", src, dst))?;
    Ok(())
}
pub fn move_file(src: &Path, dst: &Path) -> Result<()> {
    fs::rename(src, dst)
        .with_context(|| format!("ไม่สามารถ move ไฟล์จาก {:?} ไปยัง {:?}", src, dst))?;
    Ok(())
}
