use crate::file_ops::my_prelude::*;

pub fn read_file(path: &Path) -> Result<String> {
    let content =
        fs::read_to_string(path).with_context(|| format!("ไม่สามารถอ่านไฟล์: {:?}", path))?;
    Ok(content)
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    fs::write(path, format!("{content}\n")).with_context(|| format!("เขียนไฟล์ไม่ได้: {:?}", path))?;
    Ok(())
}

pub fn delete_file<P: AsRef<Path>>(paths: &[P], force: bool) -> Result<()> {
    for path in paths {
        let path = path.as_ref();

        if let Err(e) = fs::remove_file(path)
            && !(force && e.kind() == std::io::ErrorKind::NotFound)
        {
            return Err(e).with_context(|| format!("ไม่สามารถลบไฟล์ได้: {:?}", path));
        }
    }
    Ok(())
}

pub fn delete_dir(path: &Path) -> Result<()> {
    fs::remove_dir(path).with_context(|| format!("ไม่สามารถลบโฟลเดอร์ได้: {:?}", path))?;
    Ok(())
}

pub fn create_file<P: AsRef<Path>>(paths: &[P]) -> Result<()> {
    for path in paths {
        let path = path.as_ref();

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| format!("สร้างโฟร์เด้อไม่ได้: {:?}", parent))?;
        }
        File::create(path).with_context(|| format!("สร้างไฟล์ไม่ได้: {:?}", path))?;
    }
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
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("ไม่สามารถเปิดไฟล์ได้: {:?}", path))?;

    writeln!(file, "{content}").with_context(|| format!("ไม่สามารถเขียนไฟล์ได้: {:?}", path))?;
    Ok(())
}
