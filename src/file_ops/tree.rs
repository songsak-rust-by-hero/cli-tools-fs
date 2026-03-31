use crate::file_ops::my_prelude::*;

pub fn run_tree(path: &Path, indent: String, is_last: bool, all: bool) -> Result<()> {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy())
        .unwrap_or_else(|| ".".into());

    let marker = if is_last { "└── " } else { "├── " };

    println!("{}{}{}", indent, marker, name);

    if path.is_dir() {
        let entries: Vec<_> = fs::read_dir(path)
            .with_context(|| format!("ไม่สามารถอ่านโฟลเดอร์: {:?}", path))?
            .filter_map(|e| e.ok())
            .filter(|entry| {
                if all {
                    return true;
                }
                let name = entry.file_name().to_string_lossy().to_string();

                !name.starts_with(".") && name != "target"
            })
            .collect();

        let count = entries.len();

        for (i, enty) in entries.into_iter().enumerate() {
            let is_last_enty = i == count - 1;

            let new_indent = format!("{}{}", indent, if is_last { "    " } else { "│   " });

            run_tree(&enty.path(), new_indent, is_last_enty, all)?;
        }
    }
    Ok(())
}
