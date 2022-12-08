use std::collections::HashMap;

use inpt::{inpt, Inpt};
use itertools::Itertools;

type Input = Dir;

#[derive(Debug, Inpt)]
enum ShellHistory {
    #[inpt(regex = r"\$ cd (.+)")]
    CD(Location),
    #[inpt(regex = r"\$ ls")]
    LS,
    #[inpt(regex = r"dir (.+)")]
    Directory(String),
    #[inpt(regex = r"(\d+) (.+)")]
    File { size: usize, name: String },
}

#[derive(Debug, Inpt)]
enum Location {
    #[inpt(regex = "/")]
    Root,
    #[inpt(regex = "..")]
    Parent,
    #[inpt(regex = "(.*)")]
    Directory(String),
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct QualifiedFile {
    path: Vec<String>,
    name: String,
    size: usize,
}

pub fn parse(input: String) -> Result<Input, get_inputs::Error> {
    let history = input
        .lines()
        .map(|x| -> ShellHistory { inpt::<ShellHistory>(x).unwrap() });

    let mut path: Vec<String> = Vec::new();

    let mut files: Vec<QualifiedFile> = history
        .filter_map(|cmd| match cmd {
            ShellHistory::CD(loc) => {
                match loc {
                    Location::Root => path.clear(),
                    Location::Parent => drop(path.pop()),
                    Location::Directory(name) => path.push(name),
                };
                None
            }
            ShellHistory::LS => None,
            ShellHistory::Directory(_) => None,
            ShellHistory::File { size, name } => Some(QualifiedFile {
                path: path.clone(),
                name,
                size,
            }),
        })
        .collect();

    files.sort();

    Ok(treeify(files))
}

#[derive(Debug, Default)]
pub struct Dir {
    cached_size: usize,
    subdirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

fn treeify(qualified_files: Vec<QualifiedFile>) -> Dir {
    let mut out = Dir::default();

    for QualifiedFile { name, size, .. } in qualified_files.iter().filter(|x| x.path.is_empty()) {
        out.files.insert(name.to_owned(), size.to_owned());
        out.cached_size += size;
    }

    let subdirs = qualified_files
        .iter()
        .filter(|x| !x.path.is_empty())
        .group_by(|x| x.path.first().unwrap());

    for (k, files) in &subdirs {
        let subdir = treeify(
            files
                .map(|x| QualifiedFile {
                    path: x.path.split_at(1).1.to_vec(),
                    name: x.name.to_owned(),
                    size: x.size,
                })
                .collect(),
        );
        out.cached_size += subdir.cached_size;
        out.subdirs.insert(k.to_owned(), subdir);
    }

    out
}

pub fn run(input: Input) -> () {
    // println!("{:?}", input);

    let root_size = input.cached_size;
    let sizes = deletable_metrics(input);
    // println!("{:?}", sizes);

    let total = sizes.iter().filter(|&&x| x <= 100000).sum::<usize>();
    println!("{}", total);

    let required = root_size - 40000000;

    let minimum = sizes
        .iter()
        .filter(|&&x| x > required)
        .min()
        .unwrap()
        .to_owned();

    println!("{}", minimum);
}

fn deletable_metrics(dir: Dir) -> Vec<usize> {
    let mut out: Vec<usize> = Vec::new();

    out.push(dir.cached_size);

    for (_, subdir) in dir.subdirs {
        out.extend(deletable_metrics(subdir));
    }
    out
}

// let unused = ;
// let required = root_size - 40000000;
