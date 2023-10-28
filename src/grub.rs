use envfile::EnvFile;
use std::{collections::HashMap, path::Path};

pub fn set_variable(key: String, value: String) -> eyre::Result<()> {
    let mut envfile = EnvFile::new(&Path::new("/etc/default/grub"))?;
    let current_value = envfile.get("GRUB_CMDLINE_LINUX").unwrap_or("");
    let mut current_kv: HashMap<String, String> = HashMap::new();
    for item in current_value.split_whitespace() {
        let mut kv_iter = item.splitn(2, "=");
        let k = kv_iter.next().expect("TODO").to_string();
        let v = kv_iter.next().expect("TODO").to_string();
        current_kv.insert(k, v);
    }

    current_kv.insert(key, value);

    let new_values: Vec<String> = current_kv
        .iter()
        .map(|(k, v)| format!("{}={}", k, v))
        .collect();
    let new_value = new_values.join(" ");
    envfile.update("GRUB_CMDLINE_LINUX", &new_value);
    envfile.write()?;
    Ok(())
}
