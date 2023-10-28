use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::PathBuf,
};

use cmd_lib::run_cmd;

use crate::res::Asset;

pub fn install() -> eyre::Result<()> {
    // ensure dirs
    for i in [
        "/etc/systemd/logind.conf.d",
        "/etc/systemd/sleep.conf.d",
        "/etc/systemd/system/systemd-hibernate.service.d",
        "/etc/systemd/system/systemd-logind.service.d",
        "/etc/systemd/system/systemd-suspend-then-hibernate.service.d",
    ] {
        create_dir_all(i)?;
    }

    for asset in Asset::iter() {
        let asset_content = Asset::get(&asset).expect("TODO");
        let asset_path = PathBuf::from(asset.to_string());
        let mut target_path = PathBuf::from("/");
        target_path.push(asset_path);
        let mut target_file = File::create(target_path)?;
        target_file.write_all(&asset_content.data)?;
        target_file.sync_all()?;
    }

    Ok(())
}

pub fn enable_services() -> eyre::Result<()> {
    run_cmd!(
        systemctl enable hibernate-preparation.service;
        systemctl enable hibernate-resume.service;
    )?;
    Ok(())
}
