use std::env;
use std::io;
use std::fs::{self, DirEntry, Metadata};
use std::path::PathBuf;
use std::ffi::OsStr;
use std::os::unix::fs::MetadataExt; // to get file mode (executable or not)
use std::process::exit;

use ansi_term::Colour::{Blue, Yellow, Purple, Red, Green, RGB};
use ansi_term::{Style, ANSIGenericString};

use std::cmp::Ordering;
use core::borrow::Borrow;
use std::cmp::Ordering::{Greater, Less, Equal};
use std::time::SystemTime;

type Integer = i32;

const IMG_EXTENSIONS : [&str; 4] = ["svg", "png", "jpg", "jpeg"];
const VID_EXTENSIONS : [&str; 3] = ["mp4", "mkv", "flv"];
const DATA_EXTENSIONS : [&str; 2] = ["json", "xml"];
const ARCHIVE_EXTENSIONS : [&str; 5] = ["gz", "zip", "rar", "tar", "7z"];
const NO_LIMIT : Integer = -1;

const TILDE : &str = "~";
const DOT : &str = ".";
const ASTERISK : &str = "*";

const NO_COMPARATOR : &str = "none";
const NAME_COMPARATOR : &str = "name";
const LAST_MODIFIED_COMPARATOR: &str = "modified";

const COMPARATORS : [&str; 3] = [NO_COMPARATOR, NAME_COMPARATOR, LAST_MODIFIED_COMPARATOR];

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
    if "" > "" {

    }
    print!("{}", coloured_name);
}

/**
Comparator stuff
**/
fn get_names(d1 : &DirEntry, d2 : &DirEntry) -> (String, String) {
    let n1 : String = String::from(d1.file_name().to_str().unwrap());
    let n2 : String = String::from(d2.file_name().to_str().unwrap());
    return (n1, n2);
}

fn get_metadata(d1 : &DirEntry, d2 : &DirEntry) -> (Metadata, Metadata) {
    let m1 : Metadata = d1.metadata().unwrap();
    let m2 : Metadata = d2.metadata().unwrap();
    return (m1, m2);
}

fn name_comparator(d1 : &DirEntry, d2 : &DirEntry) -> Ordering {
    let names : (String, String)  = get_names(d1, d2);
    return names.0.cmp(names.1.borrow());
}

fn get_comparator(comparator_name : &str, reversed: bool, directory_first : bool) -> Box<Fn(&DirEntry,&DirEntry) -> Ordering> {
    if comparator_name == NO_COMPARATOR { //in this case, directory_first should always be true
        return Box::new(directory_comparator);
    }
    let func = match comparator_name {
        NAME_COMPARATOR=> name_comparator,
        LAST_MODIFIED_COMPARATOR => last_modified_comparator,
        _ => { println!("Unknown comparator {}, exiting.", comparator_name); exit(1); }
    };
    return Box::new(move |d1: &DirEntry, d2: &DirEntry| {
        if directory_first {
            let order : Ordering = directory_comparator(d1, d2);
            if order != Equal {
                return order;
            }
        }
        let mut order : Ordering = func(d1, d2);
        if reversed {
            order = order.reverse();
        }
        return order;
    });
}

fn directory_comparator(d1 : &DirEntry, d2 : &DirEntry) -> Ordering {
    let metadatas : (Metadata, Metadata) = get_metadata(d1, d2);
    if metadatas.0.is_dir() && metadatas.1.is_file() {
        return Less;
    } else if metadatas.1.is_dir() && metadatas.0.is_file() {
        return Greater;
    }
    return Equal;
}

fn last_modified_comparator(d1 : &DirEntry, d2 : &DirEntry) -> Ordering {
    let metadatas : (Metadata, Metadata) = get_metadata(d1, d2);
    let last_modified: (SystemTime, SystemTime) = (metadatas.0.modified().unwrap(), metadatas.1.modified().unwrap());
    return last_modified.1.cmp(last_modified.0.borrow());
}

struct DirectoryVisitor {
    all : bool,
    only_dirs: bool,
    max_level: Integer,
    comparator: String,
    reversed_sorting: bool,
    directory_first: bool,
    prefix_filter: String,
    suffix_filter: String,
    file_filter: String,
    branch_indexes : Box<Vec<Integer>>
}

impl DirectoryVisitor {

    fn path_filter(&self, dir : &DirEntry) -> bool {
        let filename : String = String::from(dir.file_name().to_str().unwrap());
        if !dir.path().is_dir() {
            if self.only_dirs {
                return false;
            } else if !self.file_filter.is_empty() {
                return filename.eq(&self.file_filter);
            }
            let mut keep : bool = true;
            if !self.prefix_filter.is_empty() {
                keep = filename.starts_with(&self.prefix_filter);
            }
            if !self.suffix_filter.is_empty() {
                keep = keep && filename.ends_with(&self.suffix_filter);
            }
            return keep;
        }
        return self.all || !filename.starts_with(DOT);
    }

    //TODO bug in '|' printing in lines below
    fn visit(&mut self, l : Integer, path: &PathBuf) -> io::Result<()> {
        for _i in 0..l {
            print!("__");
        }
        print_path(path);
        print!("\n");
        if self.max_level != NO_LIMIT && l >= self.max_level {
            return Ok(());
        }
        if path.is_dir() {
            let nb_spaces : Integer = 2 * (l + l * 2 + (l * (l - 1)) / 2); //magic formula to print well
            let s_index : usize = self.branch_indexes.len();
            self.branch_indexes.push(nb_spaces);
            let mut p_result: Vec<DirEntry> = fs::read_dir(path)?
                .map(|r : Result<DirEntry, std::io::Error>| r.unwrap())
                .filter(|dir| self.path_filter(dir))
                .collect();
            let comparator_name : &str = self.comparator.as_str();

            if comparator_name != NO_COMPARATOR || self.directory_first {
                p_result.sort_by(get_comparator(comparator_name, self.reversed_sorting, self.directory_first));
            }
            let paths : Vec<DirEntry> = p_result;

            let files_count : usize = paths.len();
            for i in 0..files_count {
                let entry = &paths[i];
                for j in 0..nb_spaces {
                    if self.branch_indexes.contains(&j)  {
                        print!("|");
                    } else {
                        print!(" ");
                    }
                }
                print!("|");
                self.visit(l + 1, &entry.path())?;
            }
            self.branch_indexes.remove(s_index);
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
            .long("--max-level")
            .help("Max level of depth")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("sorting")
            .short("-s")
            .long("--sort")
            .help("Sorting by a criteria: file name, last modified date, or directory first")
            .required(false)
            .takes_value(true)
            .default_value(NO_COMPARATOR)
            .possible_values(COMPARATORS.borrow()))
        .arg(Arg::with_name("directory_first")
            .long("--dir-first")
            .help("Displaying all directories before files. Can be combined with the sorting option")
            .required(false)
            .takes_value(false)
            .possible_values(COMPARATORS.borrow()))
        .arg(Arg::with_name("reversed")
            .short("-r")
            .short("--reverse")
            .help("Sort in reverse order")
            .required(false)
            .takes_value(false)
            .possible_values(COMPARATORS.borrow()))
        .arg(Arg::with_name("file_filter")
            .short("-f")
            .short("--file-filter")
            .help("Specify a file name filter. Filters should either be a complete file name, a prefix (e.g \"prefix*\")\
            a suffix (e.g \"*suffix\"), or a prefix and suffix (e.g \"prefix*suffix\")")
            .required(false)
            .takes_value(true))
        .get_matches();

    let comparator : &str = matches.value_of("sorting").unwrap();

    let mut file_filter : String = String::new();
    let mut prefix_filter : String = String::new();
    let mut suffix_filter : String = String::new();

    let opt_filter = matches.value_of("file_filter");
    if opt_filter.is_some() {
        let filter : &str = opt_filter.unwrap();
        match filter.matches(ASTERISK).count() {
            0 => file_filter = String::from(filter),
            1 => {
                let index : usize = filter.find(ASTERISK).unwrap();
                if index == 0 {
                    let mut suffix : String = String::from(filter);
                    suffix.remove(0);
                    suffix_filter = suffix;
                } else if index == filter.len() - 1 {
                    let mut prefix : String = String::from(filter);
                    prefix.remove(filter.len() - 1);
                    prefix_filter = prefix;
                } else {
                    let filters : Vec<&str> = filter.split(ASTERISK).collect();
                    prefix_filter = String::from(filters[0]);
                    suffix_filter = String::from(filters[1]);
                }
            }
            _ =>  {
                println!("The filter is malformed. Run with the '--help' option for mor information");
                exit(1);
            }
        }
    }
    let mut dir_visitor: DirectoryVisitor = DirectoryVisitor{
        all: matches.is_present("all"),
        only_dirs: matches.is_present("directory"),
        max_level: matches.value_of("maxLevel").map(to_int).unwrap_or(NO_LIMIT),
        comparator: String::from(comparator),
        reversed_sorting: matches.is_present("reversed"),
        directory_first: matches.is_present("directory_first"),
        file_filter,
        prefix_filter,
        suffix_filter,
        branch_indexes: Box::new(Vec::new())
    };

    let paths : Vec<PathBuf>;
    let opt_paths : Option<Values> = matches.values_of("paths");
    if opt_paths.is_some() {
        paths = opt_paths.unwrap().map(to_path_buf).collect();
    } else {
        paths = vec!(env::current_dir()?);
    }

    for i in 0..paths.len() {
        let path : &PathBuf = &paths[i];
        dir_visitor.visit(0, &path)?;
        if i < paths.len() - 1 {
            println!();
        }
    }
    Ok(())
}