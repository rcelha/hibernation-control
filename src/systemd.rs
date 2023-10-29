use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cmd_lib::run_cmd;

use crate::res::Asset;

pub fn install() -> eyre::Result<()> {
    // ensure folders exist
    for i in ["/etc/systemd/logind.conf.d", "/etc/systemd/sleep.conf.d"] {
        run_cmd!(info Creating folder ${i};)?;
        create_dir_all(i)?;
    }

    for asset in Asset::iter() {
        let asset_content = Asset::get(&asset).expect("unreacheable"); // unwrap should never fail
        let asset_path = PathBuf::from(asset.to_string());
        let mut target_path = PathBuf::from("/");
        target_path.push(asset_path);
        run_cmd!(info Install systemd file ${target_path};)?;
        let mut target_file = File::create(target_path)?;
        target_file.write_all(&asset_content.data)?;
        target_file.sync_all()?;
    }

    Ok(())
}
