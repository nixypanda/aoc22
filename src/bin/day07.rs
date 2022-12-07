use aoc22::parsers::decimal;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::alphanumeric1,
    multi::separated_list0,
    IResult,
};

use indextree::Arena;

const SMALL_SIZE_THRESHOLD: usize = 100000;
const TOTAL_DISK_SIZE: usize = 70000000;
const FREE_SPACE_FOR_UPDATE: usize = 30000000;

#[derive(Debug, PartialEq, Eq, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum DirectoryContent {
    Directory(String),
    File(File),
}

impl DirectoryContent {
    fn is_root(&self) -> bool {
        if let DirectoryContent::Directory(root_str) = self {
            root_str == &"/".to_string()
        } else {
            false
        }
    }
    fn is_parent(&self) -> bool {
        if let DirectoryContent::Directory(parent_str) = self {
            parent_str == &"..".to_string()
        } else {
            false
        }
    }
}

#[derive(Debug)]
enum Command {
    ChangeDirectory(DirectoryContent),
    List(Vec<DirectoryContent>),
}

// parsing

fn filename(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace())(input)
}

fn dirname(input: &str) -> IResult<&str, &str> {
    alt((alphanumeric1, tag("/"), tag("..")))(input)
}

fn entry_dir(input: &str) -> IResult<&str, DirectoryContent> {
    let (input, _) = tag("dir")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, dirname) = dirname(input)?;

    Ok((input, DirectoryContent::Directory(dirname.to_string())))
}

// Define a parser to parse an entry.
fn entry_file(input: &str) -> IResult<&str, DirectoryContent> {
    let (input, size) = decimal(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, filename) = filename(input)?;

    let file = File {
        name: filename.to_string(),
        size,
    };

    Ok((input, DirectoryContent::File(file)))
}

fn cd(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("cd")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, dir) = dirname(input)?;

    Ok((
        input,
        Command::ChangeDirectory(DirectoryContent::Directory(dir.to_string())),
    ))
}

fn ls(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("ls")(input)?;
    let (input, _) = tag("\n")(input)?;
    let (input, entries) = separated_list0(tag("\n"), alt((entry_dir, entry_file)))(input)?;

    Ok((input, Command::List(entries)))
}

fn command(input: &str) -> IResult<&str, Command> {
    let (input, _) = tag("$")(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, command) = alt((cd, ls))(input)?;

    Ok((input, command))
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list0(tag("\n"), command)(input)
}

fn parse(commands_str: &str) -> Vec<Command> {
    match commands(commands_str) {
        Ok((_remaining, commands)) => commands,
        Err(e) => panic!("{:?}", e),
    }
}

fn commands_to_tree(commands: Vec<Command>) -> (Arena<DirectoryContent>, indextree::NodeId) {
    let root_dir = DirectoryContent::Directory("/".to_string());
    let mut tree = Arena::new();
    let mut current_node_id = tree.new_node(root_dir);
    let root_node_id = current_node_id;

    for command in commands {
        match command {
            Command::ChangeDirectory(dir) => {
                if dir.is_root() {
                    current_node_id = root_node_id;
                } else if dir.is_parent() {
                    if let Some(parent_node_id) = tree[current_node_id].parent() {
                        current_node_id = parent_node_id;
                    } else {
                        panic!("Accessing non-existant parent")
                    }
                } else {
                    let maybe_dir = current_node_id
                        .children(&tree)
                        .into_iter()
                        .find(|child| tree[*child].get() == &dir);

                    if let Some(dir) = maybe_dir {
                        current_node_id = dir;
                    } else {
                        panic!("This directory does not exist");
                    }
                }
            }
            Command::List(contents) => {
                for content in contents {
                    let child = tree.new_node(content);
                    current_node_id.append(child, &mut tree);
                }
            }
        }
    }

    (tree, root_node_id)
}

fn populate_size_list(
    directory_tree: &Arena<DirectoryContent>,
    current_id: indextree::NodeId,
    size_arena: &mut Vec<(String, usize)>,
) -> usize {
    match directory_tree[current_id].get() {
        DirectoryContent::File(file) => file.size,
        DirectoryContent::Directory(name) => {
            let mut total_size = 0;
            for child in current_id.children(directory_tree) {
                let size = populate_size_list(directory_tree, child, size_arena);
                total_size += size;
            }
            size_arena.push((name.clone(), total_size));
            total_size
        }
    }
}

fn part1(size_list: &[(String, usize)]) -> usize {
    size_list
        .iter()
        .map(|(_, size)| size)
        .filter(|size| **size <= SMALL_SIZE_THRESHOLD)
        .sum::<usize>()
}

fn part2(size_list: &[(String, usize)]) -> usize {
    let used_size = size_list
        .iter()
        .find(|(name, _size)| name == "/")
        .unwrap()
        .1;
    let currently_free = TOTAL_DISK_SIZE - used_size;
    let need_to_free = FREE_SPACE_FOR_UPDATE - currently_free;

    *size_list
        .iter()
        .map(|(_, size)| size)
        .sorted()
        .find(|size| **size > need_to_free)
        .unwrap()
}

fn main() {
    let input = include_str!("../../data/day07.txt");
    let result = parse(input);
    let (tree, root) = commands_to_tree(result);

    let mut sizes = Vec::new();
    populate_size_list(&tree, root, &mut sizes);

    println!("Day 07 - Part 1: {}", part1(&sizes));
    println!("Day 07 - Part 2: {}", part2(&sizes));
}
