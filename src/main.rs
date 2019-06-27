use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::fmt::Error;

type Integer = i32;

fn from_ostr(ostr : &OsStr) -> Option<&str> {
    let opt_str : Option<&str> = ostr.to_str();
    if opt_str.is_none() {
        return None;
    }
    return Some(opt_str.unwrap());
}
fn from_opt_ostr(opt_ostr : Option<&OsStr>) -> Option<&str> {
    if opt_ostr.is_none() {
        return None;
    }
   return from_ostr(opt_ostr.unwrap());
}

fn print_path(path : &PathBuf) {
    let opt_str : Option<&str> = from_opt_ostr(path.file_name());
    if opt_str.is_some() {
        print!("{}", opt_str.unwrap());
    }
}

//TODO print '|' in lines below
fn visit(l : Integer, path: &PathBuf) -> io::Result<()> {
    for _i in 0..l {
        print!("__");
    }
    print_path(path);
    print!("\n");
    let nb_spaces: Integer = 2 * (l + l * 2 + (l * (l - 1)) / 2) - 2; //magic formula to print well
    if path.is_dir() {
        for r_entry in fs::read_dir(path)? {
            for _i in 0..nb_spaces {
                print!(" ");
            }
            print!("|");
            let entry = r_entry?;
            if path.is_dir() {
                visit(l + 1, &entry.path())?;
            } else {
                print_path(path);
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    visit(0, &path)?;
    Ok(())
}