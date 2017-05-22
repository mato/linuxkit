extern crate nix;
use nix::mount::{mount, MsFlags, MS_NOEXEC, MS_NOSUID, MS_NODEV};
use nix::unistd::execv;

mod lib;

use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

use std::ffi::CString;

fn cat(path: &Path) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn append(s: &str, path: &Path) -> io::Result<()> {
    let mut f = try!(OpenOptions::new().append(true).open(path));

    f.write_all(s.as_bytes())
}

fn main() {
    println!("Hello, World!");

    const NONE: Option<&'static [u8]> = None;
    mount(Some(b"rootfs".as_ref()),
          "/mnt",
          Some(b"tmpfs".as_ref()),
          MsFlags::empty(),
          NONE)
        .unwrap_or_else(|e| panic!("mount /mnt failed: {}", e));

    lib::copy_tree(&Path::new("/"), &Path::new("/mnt"))
        .unwrap_or_else(|e| panic!("copy to /mnt failed: {}", e));

    mount(Some(b"proc".as_ref()),
          "/proc",
          Some(b"proc".as_ref()),
          MS_NOEXEC | MS_NOSUID | MS_NODEV,
          NONE)
        .unwrap_or_else(|e| panic!("mount /proc failed: {}", e));

    for opt in cat(&Path::new("/proc/cmdline")).unwrap().lines() {
        if opt.starts_with("console=") {
            let console = &opt[9..];
            append(format!("{}::once:cat /etc/issue", console).as_str(), 
                &Path::new("/mnt/etc/inittab"))
                .unwrap();
            append(format!(
                "{}::respawn:/sbin/getty -n -l /bin/sh -L 115200 {} vt100",
                console, console).as_str(), 
                &Path::new("/mnt/etc/inittab"))
                .unwrap();
        }
    }

    execv(&CString::new("/bin/busybox").unwrap(),
        &[CString::new("switch_root").unwrap(),
          CString::new("/mnt").unwrap(),
          CString::new("/bin/sh").unwrap()]).unwrap();
    
    loop {
    }
}
