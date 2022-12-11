use std::fs;
use std::str::Lines;

#[derive(Debug, PartialEq)]
pub struct File {
    size: u32,
}

#[derive(Debug, PartialEq)]
pub struct Directory {
    size: u32,
    files: Vec<File>,
    directories: Vec<Directory>,
}

fn parse_directory_change(line: &str, lines: &mut Lines<'_>, directory: &mut Directory) -> bool {
    let name = line.split(" ").last().unwrap();
    if name == ".." {
        return true;
    }

    let mut newdir = Directory { files: vec![], directories: vec![], size: 0, };
    parse(lines, &mut newdir);
    directory.size += newdir.size;
    directory.directories.push(newdir);
    false
}

fn parse_command(line: &str, lines: &mut Lines<'_>, directory: &mut Directory) -> bool {
    if line.starts_with("$ ls") {
        return false;
    }
    
    return parse_directory_change(line, lines, directory);
}

fn parse_file_listing(line: &str, directory: &mut Directory) {
    let mut file = line.split(" ");
    let size = u32::from_str_radix(file.next().unwrap(), 10).unwrap();
    directory.files.push(File { size: size, });
    directory.size += size;
}

fn parse_listing(line: &str, directory: &mut Directory) {
    if line.starts_with("dir") {
        return;
    }

    parse_file_listing(line, directory);
}

fn parse(lines: &mut Lines<'_>, directory: &mut Directory) {
    match lines.next() {
        Some(line) => {
            if line.starts_with('$') {
                let pop = parse_command(line, lines, directory);
                if pop { return };
            } else {
                parse_listing(line, directory);
            }
            parse(lines, directory);
        },
        _ => {}
    }
}

pub fn parse_root(input: String) -> Directory {
    let mut lines = input.lines();
    lines.next();
    let mut root = Directory {
        files: vec![],
        directories: vec![],
        size: 0,
    }; 
    parse(&mut lines, &mut root);
    root
}

pub fn filtered_sum(dir: &Directory, sum: &mut u32, max: u32) {
    if dir.size <= max {
        *sum += dir.size;
    }
    for d in &dir.directories{
        filtered_sum(&d, sum, max);
    }
}

pub fn min_greater_than(dir: &Directory, cumul: &mut u32, free: u32) {
    if dir.size >= free {
        if *cumul > dir.size {
            *cumul = dir.size;
        }
    }
    for d in &dir.directories {
        min_greater_than(&d, cumul, free);
    }
}

pub fn solve() {
    let input = fs::read_to_string("resources/day7.txt").unwrap();
    let root = parse_root(input);
    let mut sum: u32 = 0;
    filtered_sum(&root, &mut sum, 100000);
    let mut cumul: u32 = root.size;
    min_greater_than(&root, &mut cumul, 30000000 - (70000000 - root.size));
    println!("sum: {sum}");
    println!("directory size: {cumul}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_parsing() {
        let input = String::from(r#"$ cd /
$ ls
dir a
dir b
90000 c.txt
10000 d.txt
$ cd a
$ ls
10000 e.txt
$ cd ..
$ cd b
$ ls
50000 f.txt
"#);
        let actual = parse_root(input);
        let expected = Directory {
            files: vec![
                File { size: 90000, },
                File { size: 10000, },
            ],
            directories: vec![
                Directory {
                    files: vec![
                        File { size: 10000, },
                    ],
                    directories: vec![],
                    size: 10000,
                },
                Directory {
                    files: vec![
                        File { size: 50000, },
                    ],
                    directories: vec![],
                    size: 50000,
                },
            ],
            size: 160000,
        };
        assert_eq!(expected, actual);
    }
}
