use crate::file_ops::my_prelude::*;

pub fn safe_write(path: &Path, content: &str) -> Result<()> {
    let temp_path = path.with_extension("tmp");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&temp_path)
        .with_context(|| format!("เปิด temp file ไม่ได้: {:?}", temp_path))?;

    writeln!(file, "{content}").with_context(|| "เขียน temp file ไม่ได้")?;

    file.sync_all().with_context(|| "sync ไม่สำเร็จ")?;

    if path.exists() {
        let backup_path = path.with_extension("bak");
        fs::copy(path, &backup_path).with_context(|| format!("backup ไม่ได้: {:?}", backup_path))?;
    }

    fs::rename(&temp_path, path).with_context(|| format!("replace ไม่ได้: {:?}", path))?;

    Ok(())
}
