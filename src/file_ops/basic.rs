use crate::file_ops::my_prelude::*;

pub fn read_file(path: &Path) -> Result<String> {
    let content =
        fs::read_to_string(path).with_context(|| format!("ไม่สามารถอ่านไฟล์: {:?}", path))?;
    Ok(content)
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, format!("{content}\n"))
        .with_context(|| format!("ไม่สามาเขียนไฟล์ได้: {:?}", path))?;
    Ok(())
}

pub fn delete_file(path: &Path) -> Result<()> {
    fs::remove_file(path).with_context(|| format!("ไม่สามารถลบไฟล์ได้: {:?}", path))?;
    Ok(())
}

pub fn delete_dir(path: &Path) -> Result<()> {
    fs::remove_dir(path).with_context(|| format!("ไม่สามารถลบโฟลเดอร์ได้: {:?}", path))?;
    Ok(())
}

pub fn create_file(path: &Path) -> Result<()> {
    File::create(path).with_context(|| format!("ไม่สามารถสร้างไฟล์ได้: {:?}", path))?;
    Ok(())
}

pub fn file_exists(path: &PathBuf) {
    if path.exists() {
        println!("เจอไฟล์: {:?}", path);
    } else {
        println!("ไม่มีไฟล์: {:?}", path);
    }
}

pub fn append_file(path: &Path, content: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open(path)
        .with_context(|| format!("ไม่สามารถเปิดไฟล์ได้: {:?}", path))?;

    writeln!(file, "{content}").with_context(|| format!("ไม่สามารถเขียนไฟล์ได้: {:?}", path))?;
    Ok(())
}
