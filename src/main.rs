use std::env;
use std::io;
use std::fs::{self, DirEntry};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::os::unix::fs::MetadataExt; // to get file mode (executable or not)
use std::process::exit;

use ansi_term::Colour::{Blue, Yellow, Purple, Red, Green, RGB};
use ansi_term::{Style, ANSIGenericString};

type Integer = i32;

const IMG_EXTENSIONS : [&str; 4] = ["svg", "png", "jpg", "jpeg"];
const VID_EXTENSIONS : [&str; 3] = ["mp4", "mkv", "flv"];
const DATA_EXTENSIONS : [&str; 2] = ["json", "xml"];
const ARCHIVE_EXTENSIONS : [&str; 5] = ["gz", "zip", "rar", "tar", "7z"];
const NO_LIMIT : Integer = -1;

const TILDE : &str = "~";
const DOT : &str = ".";

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

fn ends_with(s : &str, arr: &[&str]) -> bool {
    return arr.iter().any(|ext| s.ends_with(ext));
}

fn is_executable(path : &&PathBuf) -> bool {
    let mode = fs::metadata(path).unwrap().mode();
    return mode & 0o111 != 0;
}

fn print_path(path : &PathBuf) {
    let opt_str : Option<&str> = from_opt_ostr(path.file_name());
    if opt_str.is_none() {
        return;
    }
    let name : &str = opt_str.unwrap();
    let mut coloured_name : ANSIGenericString<str>;
    if path.is_dir() {
        coloured_name = Blue.bold().paint(name);
    } else if ends_with(name, &IMG_EXTENSIONS) {
        coloured_name = RGB(255,105,180).bold().paint(name)
    } else if ends_with(name, &VID_EXTENSIONS) {
        coloured_name = Purple.bold().paint(name);
    } else if ends_with(name, &DATA_EXTENSIONS) {
        coloured_name = Yellow.bold().paint(name);
    } else if ends_with(name, &ARCHIVE_EXTENSIONS) {
        coloured_name = Red.bold().paint(name);
    } else if is_executable(&path) {
        coloured_name = Green.bold().paint(name);
    } else {
        coloured_name = Style::default().paint(name);
    }
    print!("{}", coloured_name);
}

struct DirectoryVisitor {
    all : bool,
    only_dirs: bool,
    max_level: Integer
}

impl DirectoryVisitor {

    fn file_filter(&self, dir : &DirEntry) -> bool {
        if self.only_dirs && !dir.path().is_dir() {
            return false;
        }
        return self.all || !dir.file_name().to_str().unwrap().starts_with(".");
    }

    //TODO bug in '|' printing in lines below
    fn visit(&self, l : Integer, path: &PathBuf, branch_indexes : &mut Box<Vec<Integer>>) -> io::Result<()> {

        for _i in 0..l {
            print!("__");
        }

        print_path(path);
        print!("\n");
        if self.max_level != NO_LIMIT && l >= self.max_level {
            return Ok(());
        }
        let nb_spaces : Integer = 2 * (l + l * 2 + (l * (l - 1)) / 2); //magic formula to print well
        branch_indexes.push(nb_spaces);
        let s_index : usize = branch_indexes.len() - 1;
        if path.is_dir() {
            let paths : Vec<DirEntry>= fs::read_dir(path)?
                .map(|r : Result<DirEntry, std::io::Error>| r.unwrap())
                .filter(|dir| self.file_filter(dir))
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
                self.visit(l + 1, &entry.path(), branch_indexes)?;
            }
        }
        Ok(())
    }
}


fn to_int(s : &str) -> Integer {
    let mut r : Integer = 0;
    for c in s.chars() {
        if c < '0' || c > '9' {
            println!("{} is not a number", s);
            exit(1);
        }
        r = 10 * r + (c.to_digit(10).unwrap() - '0'.to_digit(10).unwrap()) as Integer;
    }
    return r;
}

fn to_path_buf(s : &str) -> PathBuf {
    let result : String;
    if s.starts_with(TILDE) {
        result = s.replace(TILDE, std::env::var("HOME").unwrap().as_str());
    } else if s.starts_with(DOT) {
        result = s.replace(DOT, env::current_dir().unwrap().to_str().unwrap());
    } else {
        result = String::from(s);
    }
    return PathBuf::from(result);
}
extern crate clap;
use clap::{Arg, App, ArgMatches, Values};

fn main() -> std::io::Result<()> {
    let matches : ArgMatches = App::new("tree")
        .version("1.0")
        .author("Tambue Nelson F. <tambapps@gmail.com>")
        .about("Prints the file architecture of a directory")
        .arg(Arg::with_name("paths")
            .help("path to list files from")
            .multiple(true)
            .required(false))
        .arg(Arg::with_name("all")
            .short("-a")
            .long("-all")
            .help("include hidden directories")
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("directory")
            .short("-d")
            .long("--directory")
            .help("display only directories")
            .required(false)
            .takes_value(false))
        .arg(Arg::with_name("maxLevel")
            .short("-l")
            .long("--maxLevel")
            .help("Max level of depth")
            .required(false)
            .takes_value(true))
        .get_matches();

    let dir_visitor : DirectoryVisitor = DirectoryVisitor{
        all: matches.is_present("all"),
        only_dirs: matches.is_present("directory"),
        max_level: matches.value_of("maxLevel").map(to_int).unwrap_or(NO_LIMIT)
    };

    let paths : Vec<PathBuf>;
    let opt_paths : Option<Values> = matches.values_of("paths");
    if opt_paths.is_some() {
        paths = opt_paths.unwrap().map(to_path_buf).collect();
    } else {
        paths = vec!(env::current_dir()?);
    }

    for path in paths {
        let mut b :  Box<Vec<Integer>> =  Box::new(Vec::new());
        dir_visitor.visit(0, &path, &mut b)?;
    };
    Ok(())
}