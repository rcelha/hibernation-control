use std::fs::File;
use std::io::Write;
use std::{fs::copy, path::PathBuf};

use cmd_lib::{run_cmd, run_fun};
use sys_info::mem_info;

use crate::grub;

pub fn run() -> eyre::Result<()> {
    sanity_check()?;

    run_cmd!(info "Enabling hibernation")?;
    backup_files()?;

    create_swapfile()?;

    let uuid = get_uuid()?;
    run_cmd!(info /swapfile uuid=$uuid)?;

    let offset = get_offset()?;
    run_cmd!(info /swapfile offset=$offset)?;

    set_grub_options(uuid.clone(), offset)?;
    set_initramfs_options(uuid, offset)?;

    // systemd::install()?;
    // systemd::enable_services()?;

    run_cmd!(info you can install the extension "https://extensions.gnome.org/extension/755/hibernate-status-button/" to add hibernate option to your power menu)?;

    Ok(())
}

/// Test for OS dependencies and users' permissions
pub fn sanity_check() -> eyre::Result<()> {
    // TODO
    Ok(())
}

pub fn backup_files() -> eyre::Result<()> {
    let suffix = "hibernation.bk";
    let file_list = vec![
        "/etc/default/grub",
        // "/etc/systemd/sleep.conf.d/sleep.conf",
        // "/etc/systemd/system/hibernate-preparation.service",
        // "/etc/systemd/system/hibernate-resume.service",
        // "/etc/systemd/system/systemd-logind.service.d/override.conf",
        // "/etc/systemd/system/systemd-hibernate.service.d/override.conf",
        // "/etc/systemd/system/systemd-suspend-then-hibernate.service.d/override.conf",
    ];

    for i in file_list {
        let mut file_path = PathBuf::from(i);
        file_path.set_extension(suffix);
        copy(i, file_path).ok();
    }
    Ok(())
}

pub fn create_swapfile() -> eyre::Result<()> {
    run_cmd!(
        info "Creating swapfile (/swapfile)";
        bash -c "swapoff /swapfile || true";
    )?;

    let memory_size = mem_info()?.total;
    let swap_size = memory_size * 2;
    let swap_size_mb = swap_size / 1024;
    let swap_block_size_mb = 32;
    let swap_num_blocks = swap_size_mb / swap_block_size_mb;

    run_cmd!(
        info "Allocating swapfile (${swap_size}MB)";
        sudo dd if=/dev/zero of=/swapfile bs=${swap_block_size_mb}MB count=${swap_num_blocks};
        chmod 600 /swapfile;
        mkswap /swapfile;
        swapon /swapfile;
    )?;
    Ok(())
}

// TODO replace run_fun here
pub fn get_uuid() -> eyre::Result<String> {
    let uuid = run_fun!(findmnt -no UUID -T /swapfile)?;
    Ok(uuid)
}

// TODO replace run_fun here
pub fn get_offset() -> eyre::Result<usize> {
    let first_block = run_fun!(filefrag -v /swapfile | grep " 0:")?;
    let first_block = first_block.replace(" ", "");
    let first_block = first_block.replace("..", ":");
    let block_iter = first_block.split(":");
    let offset_result = block_iter.skip(3).next().expect("TODO");

    let offset: usize = offset_result.parse()?;
    Ok(offset)
}

pub fn set_grub_options(uuid: String, resume_offset: usize) -> eyre::Result<()> {
    grub::set_variable("resume".into(), format!("UUID={uuid}"))?;
    grub::set_variable("resume_offset".into(), resume_offset.to_string())?;

    // TODO use update-grub instead of grub-mkconfig
    run_cmd!(
        grub-mkconfig -o /boot/grub/grub.cfg;
        grub-mkconfig -o /boot/efi/EFI/ubuntu/grub.cfg;
    )?;
    Ok(())
}

pub fn set_initramfs_options(uuid: String, resume_offset: usize) -> eyre::Result<()> {
    let mut resume_file = File::create("/etc/initramfs-tools/conf.d/resume")?;
    write!(
        resume_file,
        "RESUME=UUID={uuid} resume_offset={resume_offset}"
    )?;
    resume_file.sync_all()?;
    run_cmd!(update-initramfs -c -k all)?;
    Ok(())
}
