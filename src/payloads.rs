use std::fs::OpenOptions;
use std::io::{self, prelude::*};
use std::{error, fs::File, path::PathBuf};

use zip::{result::ZipError, write::FileOptions, ZipWriter};

use blake2::{Blake2s256, Digest};

use std::ffi::CString;

use winapi::shared::minwindef::BOOL;
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::fileapi::GetFileAttributesA;
use winapi::um::fileapi::SetFileAttributesA;
use winapi::um::winnt::FILE_ATTRIBUTE_HIDDEN;

use crate::gui::State;
use crate::windows;

pub fn archive(path: &PathBuf) -> Result<File, ZipError> {
    let zip_path = get_zip_path(path);
    let zip_file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(zip_file); // archive name (should be filename.zip)
    zip.start_file(path.to_str().to_owned().unwrap(), FileOptions::default())?;

    zip.finish()
}

fn get_zip_path(path: &PathBuf) -> PathBuf {
    // let mut new_path = PathBuf::from(path);
    // new_path.pop();
    // new_path.push(path.file_stem().unwrap());
    // new_path.set_extension("zip");

    let mut new_path = PathBuf::from(path.parent().unwrap());
    new_path.push(path.file_stem().unwrap());
    new_path.set_extension("zip");

    new_path
}

pub fn hash(path: &PathBuf) -> io::Result<()> {
    //Result<(), Box<dyn error::Error>>
    let mut file = File::open(&path)?;
    let mut hasher = Blake2s256::new();
    let n = io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    #[cfg(debug_assertions)]
    {
        println!("Path: {}", path.display());
        println!("Bytes processed: {}", n);
        println!("Hash value: {:x}", hash);
    }

    let mut hash_file_path = PathBuf::from(path.parent().unwrap());
    hash_file_path.push(path.file_stem().unwrap());
    hash_file_path.set_extension("hash");

    let mut hash_file = File::create(hash_file_path)?;
    hash_file.write_all(&hash)?;
    // hash_file.sync_all()?;

    Ok(())
}

#[cfg(windows)]
pub fn make_hidden(path: &PathBuf) -> Result<(), Box<dyn error::Error>> {
    let path_str = path.to_str().unwrap();
    let file_name = CString::new(path_str)?;

    unsafe {
        let attr: DWORD = GetFileAttributesA(file_name.as_ptr());
        if (attr & FILE_ATTRIBUTE_HIDDEN) == 0 {
            let res: BOOL = SetFileAttributesA(file_name.as_ptr(), attr | FILE_ATTRIBUTE_HIDDEN);
            if res == 0 {
                let error = GetLastError();
                // return Err(io::Error::new(io::ErrorKind::Other, error.into()).into());
                return Err(error.to_string().into());
            }
        }
    }

    Ok(())
}

pub fn do_things(state: &State) -> Result<(), Box<dyn error::Error>> {
    let path = &PathBuf::from(&state.file_path);
    let mut options = OpenOptions::new();
    options.read(true);
    if state.should_hide {
        options.write(true);
    }
    match options.open(path) {
        Ok(_) => {
            if state.should_archive {
                archive(path)?;
            }
            if state.should_hash {
                hash(path)?;
            }
            if state.should_hide {
                make_hidden(path)?;
            }
        }
        Err(_) => {
            if windows::is_elevated() {
                return Err("Can't run even as administrator!".into());
            } else {
                let args: Vec<String> = std::env::args().collect();
                let new_args = format!(
                    "{} {} {} {}",
                    state.file_path, state.should_archive, state.should_hash, state.should_hide
                );
                windows::run_as_administrator(&args[0], &new_args);
            }
        }
    }

    Ok(())
}