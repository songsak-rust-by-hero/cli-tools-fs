use crate::file_ops::my_prelude::*;

pub fn count_lines(path: &Path) -> Result<()> {
    let content = read_file(path).with_context(|| format!("ไม่สามารอ่านไฟล์นนี้ได้: {:?}", path))?;
    println!("{}", content.lines().count());
    Ok(())
}
pub fn search_file(path: &Path, keywold: &str) -> Result<()> {
    let content = read_file(path).with_context(|| format!("ไม่สามารอ่านไฟล์นนี้ได้: {:?}", path))?;
    for line in content.lines() {
        if line.contains(keywold) {
            println!("{line}");
        }
    }
    Ok(())
}
