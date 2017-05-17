use std::path::Path;
use std::path::PathBuf;
use std::fs;
extern crate walkdir;
use self::walkdir::WalkDir;

pub fn copy_tree(src: &Path, dest: &Path) -> Result<u64, String> {
    let src = match fs::canonicalize(src) {
        Err(err) => {
            let e = format!("{:?}: {}", src, err);
            return Err(e);
        }
        Ok(src) => src,
    };
    let src = src.as_path();
    if !src.is_dir() {
        let e = format!("Not a directory: {:?}", src);
        return Err(e);
    }
    let dest = match fs::canonicalize(dest) {
        Err(err) => {
            let e = format!("{:?}: {}", dest, err);
            return Err(e);
        }
        Ok(dest) => dest,
    };
    let dest = dest.as_path();
    if !dest.is_dir() {
        let e = format!("Not a directory: {:?}", dest);
        return Err(e);
    }
    if dest.starts_with(src) {
        let e = format!("Attempt to copy a directory {:?} into itself, {:?}",
            src, dest);
        return Err(e);
    }

    let mut total_bytes = 0;
    let wd = WalkDir::new(src);
    for dent in wd {
        let dent = match dent {
            Err(err) => return Err(err.to_string()),
            Ok(dent) => dent,
        };
        let dent = dent.path();
        if dent == src {
            continue;
        }
        let target = PathBuf::from(dest)
            .join(dent.strip_prefix(src).unwrap());
        if dent.is_dir() {
            match fs::create_dir(&target) {
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
    Ok(total_bytes)
}
