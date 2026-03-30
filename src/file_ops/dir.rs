use crate::file_ops::my_prelude::*;

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
