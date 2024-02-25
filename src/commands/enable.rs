use std::fs::File;
use std::io::Write;
use std::{fs::copy, path::PathBuf};

use cmd_lib::{run_cmd, run_fun};
use sys_info::mem_info;

use crate::grub;
use crate::systemd;

pub struct SwapOptions {
    pub size: Option<u64>,
    pub path: Option<String>,
}

pub struct Options {
    pub swap: SwapOptions,
}

/// Enable hibernation on the system
pub fn run(options: Options) -> eyre::Result<()> {
    sanity_check()?;

    run_cmd!(info "Enabling hibernation")?;
    backup_files()?;

    let swapfile_path = create_swapfile(&options.swap)?;

    let uuid = get_uuid(&swapfile_path)?;
    run_cmd!(info ${swapfile_path} uuid=$uuid)?;

    let offset = get_offset(&swapfile_path)?;
    run_cmd!(info ${swapfile_path} offset=$offset)?;

    set_grub_options(uuid.clone(), offset)?;

    // DONT NEED THESE TWO FOR THE STEAM DECK
    // TODO add options to skip them
    // set_initramfs_options(uuid, offset)?;
    // systemd::install()?;

    run_cmd!(
        info Done;
        info Please restart your system;
        info After restarting it, you can test your setup with the following command:;
        info sudo systemctl hibernate;
    )?;

    Ok(())
}

/// Test for OS dependencies and users' permissions
pub fn sanity_check() -> eyre::Result<()> {
    let uid = unsafe { libc::getuid() };
    eyre::ensure!(uid == 0, "hibernation-control must be ran as root");

    run_cmd!(info Checking dependencies)?;
    for i in [
        "swapoff",
        "dd",
        "chmod",
        "mkswap",
        "swapon",
        "findmnt",
        "filefrag",
        "update-grub",
        // "grub-mkconfig",
        // "update-initramfs",
    ] {
        let test_command = run_fun!(which ${i};);
        if test_command.is_err() {
            return Err(eyre::eyre!("Command '{}' not found.", i));
        }
    }
    Ok(())
}

pub fn backup_files() -> eyre::Result<()> {
    let suffix = "hibernation.bk";
    let file_list = vec!["/etc/default/grub"];

    for i in file_list {
        let mut file_path = PathBuf::from(i);
        file_path.set_extension(suffix);
        copy(i, file_path).ok();
    }
    Ok(())
}

pub fn create_swapfile(options: &SwapOptions) -> eyre::Result<String> {
    let swapfile = options.path.clone().unwrap_or("/swapfile".to_string());
    run_cmd!(
        info "Creating swapfile (${swapfile})";
        bash -c "swapoff ${swapfile} || true";
    )?;

    let forced_swap_size = options.size.clone();
    let memory_size = mem_info()?.total;
    let swap_size = memory_size * 2;
    let swap_size = match forced_swap_size {
        Some(size) => size,
        None => swap_size,
    };
    let swap_size_mb = swap_size / 1024;
    let swap_block_size_mb = 32;
    let swap_num_blocks = swap_size_mb / swap_block_size_mb;

    run_cmd!(
        info "Allocating swapfile (${swap_size}MB)";
        sudo dd if=/dev/zero of=${swapfile} bs=${swap_block_size_mb}MB count=${swap_num_blocks};
        chmod 600 ${swapfile};
        mkswap ${swapfile};
        swapon ${swapfile};
    )?;
    Ok(swapfile)
}

// TODO replace run_fun here
pub fn get_uuid(filepath: &str) -> eyre::Result<String> {
    let uuid = run_fun!(findmnt -no UUID -T ${filepath})?;
    Ok(uuid)
}

// TODO replace run_fun here
pub fn get_offset(filepath: &str) -> eyre::Result<usize> {
    let first_block = run_fun!(filefrag -v ${filepath} | grep " 0:")?;
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

    run_cmd!(
        update-grub;
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
