use std::path::Path;
use std::path::PathBuf;
use std::fs;
use std::os::unix::fs::symlink;

extern crate walkdir;
use self::walkdir::WalkDir;
use self::walkdir::WalkDirIterator;

extern crate same_file;
use self::same_file::Handle;

pub fn copy_tree(src: &Path, dest: &Path) -> Result<u64, String> {
    if !src.is_dir() {
        let e = format!("Not a directory: {:?}", src);
        return Err(e);
    }
    if !dest.is_dir() {
        let e = format!("Not a directory: {:?}", dest);
        return Err(e);
    }
    let dest_handle = Handle::from_path(dest).unwrap();

    let mut total_bytes = 0;
    let mut it = WalkDir::new(src).min_depth(1).into_iter();

    while let Some(dent) = it.next() {
        let dent = match dent {
            Err(err) => return Err(err.to_string()),
            Ok(dent) => dent,
        };
        let dent = dent.path();
        let target = PathBuf::from(dest)
            .join(dent.strip_prefix(src).unwrap());
        if dent.is_dir() {
            let dent_handle = Handle::from_path(dent).unwrap();
            if dent_handle == dest_handle {
                it.skip_current_dir();
                continue;
            }
            match fs::create_dir(&target) {
                Err(err) => {
                    let e = format!("{:?}: {}", target, err);
                    return Err(e);
                }
                Ok(_) => continue,
            }
        }
        else {
            let attr = fs::symlink_metadata(dent).unwrap();
            if attr.file_type().is_symlink() {
                let link = fs::read_link(dent).unwrap();
                match symlink(&link, &target) {
                    Err(err) => {
                        let e = format!("{:?}: {}", target, err);
                        return Err(e);
                    }
                    Ok(_) => continue,
                }
            }
            else {
                match fs::copy(&dent, &target) {
                    Err(err) => {
                        let e = format!("{:?}: {}", dent, err);
                        return Err(e);
                    }
                    Ok(bytes) => total_bytes += bytes,
                }
            }
        }
    }
    Ok(total_bytes)
}
