use crate::file_ops::my_prelude::*;

pub fn count_lines(path: &PathBuf) -> Result<()> {
    let content = read_file(path)?;
    println!("{}", content.lines().count());
    Ok(())
}
pub fn search_file(path: &PathBuf, keywold: &str) -> Result<()> {
    let content = read_file(path)?;
    for line in content.lines() {
        if line.contains(keywold) {
            println!("{line}");
        }
    }
    Ok(())
}
