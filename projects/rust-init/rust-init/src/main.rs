extern crate nix;
use nix::mount::{mount, MsFlags};

fn main() {
    println!("Hello, World!");

    const NONE: Option<&'static [u8]> = None;
    mount(NONE,
          "/mnt",
          Some(b"tmpfs".as_ref()),
          MsFlags::empty(),
          NONE)
        .unwrap_or_else(|e| panic!("mount failed: {}", e));

    loop {
    }
}
