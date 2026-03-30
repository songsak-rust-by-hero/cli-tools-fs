use crate::file_ops::my_prelude::*;

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
