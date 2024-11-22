use crate::common::{AocError, InputType};
use core::fmt;
use log::debug;
use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag},
    IResult,
};
use std::{
    cell::RefCell,
    fmt::Display,
    io::{Error, ErrorKind},
    rc::Rc,
};

fn not_space(s: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(s)
}

fn integer(s: &str) -> IResult<&str, &str> {
    is_a("0123456789")(s)
}

#[derive(Debug)]
struct FileWithSize {
    //    name: String,
    size: usize,
}
impl FileWithSize {
    fn new(/* name: &str, */ size: usize) -> FileWithSize {
        FileWithSize {
            //name: name.to_string(),
            size,
        }
    }
}

type FileRef = Rc<RefCell<FileWithSize>>;

#[derive(Debug)]
struct Directory {
    name: String,
    sub_directories: Vec<DirRef>,
    files: Vec<FileRef>,
}
impl Directory {
    fn new(name: &str) -> Directory {
        Directory {
            name: name.to_string(),
            sub_directories: Vec::new(),
            files: Vec::new(),
        }
    }
    fn add_directory(&mut self, dir: DirRef) {
        self.sub_directories.push(dir);
    }

    fn add_file(&mut self, file: FileRef) {
        self.files.push(file);
    }

    fn files_count(&self) -> usize {
        self.files.len()
    }

    fn files_size(&self) -> usize {
        self.files.iter().fold(0, |acc, f| acc + f.borrow().size)
    }

    fn dirs_count(&self) -> usize {
        self.sub_directories.len()
    }

    fn dirs_size(&self) -> usize {
        self.sub_directories
            .iter()
            .fold(0, |acc, d| acc + d.borrow().total_size())
    }

    fn total_size(&self) -> usize {
        self.files_size() + self.dirs_size()
    }

    fn display_dir(&self) {
        debug!("{}", self);
    }

    fn display_dir_tree(&self, prefix: &str) {
        debug!("{}{}", prefix, self);
        self.sub_directories.iter().for_each(|d| {
            d.borrow()
                .display_dir_tree(format!("{}{}", "  ", prefix).as_str());
        })
    }
}

fn add_if<F>(dir: &DirRef, predicate: &F, collection: &mut Vec<DirRef>)
where
    F: Fn(&DirRef) -> bool,
{
    if predicate(&dir) {
        collection.push(dir.clone());
    }
    dir.borrow().sub_directories.iter().for_each(|sd| {
        add_if(&sd, predicate, collection);
    })
}

impl Display for Directory {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: #files={} files_size={} #dirs={} #dirs_size={} #total_size={}",
            self.name,
            self.files_count(),
            self.files_size(),
            self.dirs_count(),
            self.dirs_size(),
            self.total_size()
        )
    }
}

#[derive(Debug)]
enum LineType {
    ChangeDir(String),
    Ls,
    Subdirectory(String),
    FileSize(String, usize),
}

fn change_directory(input: &str) -> IResult<&str, LineType> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir_name) = not_space(input)?;
    Ok((input, LineType::ChangeDir(dir_name.to_string())))
}
fn ls(input: &str) -> IResult<&str, LineType> {
    let (input, _) = tag("$ ls")(input)?;
    Ok((input, LineType::Ls))
}
fn sub_directory(input: &str) -> IResult<&str, LineType> {
    let (input, _) = tag("dir ")(input)?;
    let (input, dir_name) = not_space(input)?;
    Ok((input, LineType::Subdirectory(dir_name.to_string())))
}
fn file_size(input: &str) -> IResult<&str, LineType> {
    let (input, size_str) = integer(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, file_name) = not_space(input)?;
    Ok((
        input,
        LineType::FileSize(file_name.to_string(), size_str.parse().unwrap()),
    ))
}

type DirRef = Rc<RefCell<Directory>>;
fn new_dir_ref(dir: Directory) -> DirRef {
    Rc::new(RefCell::new(dir))
}

pub fn part1() -> Result<String, AocError> {
    let root_directory = build_dir_tree()?;

    debug!("PART1: Dir tree");
    root_directory.borrow().display_dir_tree(" - ");

    let mut smaller_dirs = Vec::<DirRef>::new();
    add_if(
        &root_directory,
        &|d| d.borrow().total_size() <= 100000,
        &mut smaller_dirs,
    );

    debug!("\nPART1: Dirs <= 100000");
    for d in &smaller_dirs {
        d.borrow().display_dir()
    }

    let sum_smaller_dirs_total_sizes = smaller_dirs
        .as_slice()
        .iter()
        .fold(0, |a, d| a + d.borrow().total_size());

    Ok(format!("{}", sum_smaller_dirs_total_sizes))
}

pub fn part2() -> Result<String, AocError> {
    let root_directory = build_dir_tree()?;

    let mut all_dirs = Vec::<DirRef>::new();
    add_if(&root_directory, &|_| true, &mut all_dirs);
    all_dirs.sort_by(|a, b| a.borrow().total_size().cmp(&b.borrow().total_size()));

    let available = 70000000 - root_directory.borrow().total_size();
    let need_to_free = 30000000 - available;
    debug!(
        "\nPART2:  Avalailable={}, need to free={}",
        available, need_to_free
    );

    if need_to_free <= 0 {
        panic!("\nPART2: Don't need to free any space");
    } else {
        debug!("\nPART2:  Need to free up >= {}", need_to_free);
        debug!("\nPART2: All dirs sorted");
        for d in &all_dirs {
            d.borrow().display_dir()
        }

        let smallest = all_dirs
            .iter()
            .find(|d| d.borrow().total_size() >= need_to_free);
        match smallest {
            Some(d) => return Ok(format!("{}", &d.borrow().total_size())),
            None => panic!("There is no directory big enough"),
        }
    }
}

fn build_dir_tree() -> Result<Rc<RefCell<Directory>>, Error> {
    let lines = InputType::Challenge.get_input_lines(7)?;
    let root_directory = new_dir_ref(Directory::new("/"));
    let mut path: Vec<DirRef> = vec![root_directory.clone()];
    for line in lines {
        //debug!("L:{}", line);
        match parse_line(&line)? {
            LineType::ChangeDir(dir) => {
                if dir == "/" {
                    path.truncate(1);
                } else if dir == ".." {
                    path.remove(path.len() - 1);
                } else {
                    let new_dir = new_dir_ref(Directory::new(dir.as_str()));

                    let a = path.last();
                    match a {
                        Some(p) => {
                            p.borrow_mut().add_directory(new_dir.clone());
                        }
                        None => panic!("Can't proceed.  No path to add current path to {:?}", path),
                    }
                    path.push(new_dir);
                }
                //debug!("cd'd into {:?}", path);
            }
            LineType::Ls => {}
            LineType::Subdirectory(_) => {}
            LineType::FileSize(_, size) => {
                let new_dir = Rc::new(RefCell::new(FileWithSize::new(size)));

                let a = path.last();
                match a {
                    Some(p) => {
                        p.borrow_mut().add_file(new_dir.clone());
                    }
                    None => panic!("Can't proceed.  No path to add current path to {:?}", path),
                }
            }
        }
    }
    Ok(root_directory)
}

fn parse_line(line: &String) -> Result<LineType, Error> {
    let line_type = match alt((ls, change_directory, sub_directory, file_size))(&line) {
        Ok((_, lt)) => lt,
        Err(e) => {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Unrecognized line type {:?}", e),
            ))
        }
    };
    Ok(line_type)
}
