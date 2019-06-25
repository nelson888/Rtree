use std::env;
use std::io;
use std::fs::{self};
use std::path::PathBuf;
use std::ffi::OsStr;

type Integer = i32;

fn print_path(path : &PathBuf) {
    let opt_ostr : Option<&OsStr> = path.file_name();
    if opt_ostr.is_none() {
        return;
    }
    let opt_str : Option<&str> = opt_ostr.unwrap().to_str();
    if opt_str.is_none() {
        return;
    }
    print!("{}", opt_str.unwrap());
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
        for entry in fs::read_dir(path)? {
            for _i in 0..nb_spaces {
                print!(" ");
            }
            print!("|");
            let entry = entry?;
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