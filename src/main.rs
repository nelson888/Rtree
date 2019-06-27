use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::ffi::OsStr;

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

//TODO bug in '|' printing in lines below
fn visit(l : Integer, path: &PathBuf, branch_indexes : &mut Box<Vec<Integer>>) -> io::Result<()> {
    for _i in 0..l {
        print!("__");
    }
    print_path(path);
    print!("\n");
    let nb_spaces: Integer = 2 * (l + l * 2 + (l * (l - 1)) / 2); //magic formula to print well
    branch_indexes.push(nb_spaces);
    let s_index : usize = branch_indexes.len() - 1;
    if path.is_dir() {
        let paths : Vec<DirEntry>= fs::read_dir(path)?
            .map(|r : Result<DirEntry, std::io::Error>| r.unwrap())
            .collect();
        let files_count : usize = paths.len();
        for i in 0..files_count {
            let entry = &paths[i];
            for j in 0..nb_spaces {
                if branch_indexes.contains(&j)  {
                    print!("|");
                } else {
                    print!(" ");
                }
            }
            print!("|");
            if  i == files_count - 1 {
                branch_indexes.remove(s_index);
            }
            visit(l + 1, &entry.path(), branch_indexes)?;
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let mut b:  Box<Vec<Integer>> =  Box::new(Vec::new());
    visit(0, &path, &mut b)?;
    Ok(())
}