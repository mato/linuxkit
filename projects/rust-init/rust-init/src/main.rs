use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

extern crate nix;
use nix::mount::{mount, MsFlags, MS_NOEXEC, MS_NOSUID, MS_NODEV};
use nix::unistd::execv;
use std::ffi::CString;

mod lib;

fn cat(path: &Path) -> io::Result<String> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn main() {
    const NONE: Option<&'static [u8]> = None;

    mount(Some(b"tmpfs".as_ref()),
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

    for line in cat(&Path::new("/proc/cmdline")).unwrap().lines() {
        for opt in line.split_whitespace() {
            if opt.starts_with("console=") {
                let tty = &opt[8..];
                let inittab = OpenOptions::new().append(true)
                    .open("/mnt/etc/inittab")
                    .unwrap_or_else(|e| panic!("open /mnt/etc/inittab failed: {}", e));
                writeln!(&inittab, "{}::once:cat /etc/issue", tty).unwrap();
                writeln!(&inittab,
                    "{}::respawn:/sbin/getty -n -l /bin/sh -L 115200 {} vt100",
                    tty, tty)
                    .unwrap();
            }
        }
    }

    execv(&CString::new("/bin/busybox").unwrap(),
        &[CString::new("switch_root").unwrap(),
          CString::new("/mnt").unwrap(),
          CString::new("/sbin/init").unwrap()]).unwrap();

    panic!();
}
