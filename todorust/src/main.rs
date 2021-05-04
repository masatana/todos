use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::cmp::{Ord, PartialEq, PartialOrd};
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Eq)]
struct Mytodo {
    id: u32,
    create_at: String,
    deadline: String,
    todo: String,
    priority: String,
}

impl PartialEq for Mytodo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl PartialOrd for Mytodo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for Mytodo {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

fn get_from_stdio(usage: &str) -> String {
    println!("{}", usage);
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .ok()
        .expect("failed to read line");
    line.trim().to_string()
}

fn create(todos: &mut std::vec::Vec<Mytodo>) {
    create_with_id(todos, 0);
}

fn create_with_id(todos: &mut std::vec::Vec<Mytodo>, update_id: u32) {
    let todo = get_from_stdio("todo what?");
    let priority = get_from_stdio("priority?");
    let deadline = get_from_stdio("deadline?");
    let create_at = Local::now().format("%Y%m%d %H%M%S").to_string();
    let mut next_id = 1;
    if update_id == 0 {
        if let Some(max_id) = todos.into_iter().max() {
            next_id = max_id.id + 1;
        }
    } else {
        next_id = update_id;
    }
    todos.push(Mytodo {
        id: next_id,
        create_at: create_at,
        deadline: deadline,
        todo: todo,
        priority: priority,
    })
}

fn load() -> Vec<Mytodo> {
    let file = File::open("/tmp/testfile")
        .ok()
        .expect("failed to open file");
    let todovec: Vec<Mytodo> = serde_json::from_reader(&file)
        .ok()
        .expect("failed to deseialize");
    todovec
}

fn read(todovec: &std::vec::Vec<Mytodo>) {
    println!("{:?}", todovec);
}

fn delete_internal(todovec: &mut std::vec::Vec<Mytodo>, explain: &str) -> u32 {
    read(todovec);
    let _id = get_from_stdio(explain).parse::<u32>().unwrap();
    if let Some(index) = todovec.into_iter().position(|x| x.id == _id) {
        todovec.remove(index);
    }
    _id
}

fn update(todovec: &mut std::vec::Vec<Mytodo>) {
    let _id = delete_internal(todovec, "update_id?");
    create_with_id(todovec, _id);
}

fn delete(todovec: &mut std::vec::Vec<Mytodo>) {
    delete_internal(todovec, "delete_id?");
}

fn save(todomap: &std::vec::Vec<Mytodo>) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("/tmp/testfile")
        .ok()
        .expect("failed to open file");
    serde_json::to_writer(&file, todomap)
        .ok()
        .expect("failed to write data");
}

fn main() {
    let mut todovec: std::vec::Vec<Mytodo> = load();

    loop {
        let mut line = String::new();
        let now = Local::now().format("%Y%m%d %H%M%S").to_string();
        println!("{}: Choose ops: CREATE, READ, UPDATE, DELETE, END", now);
        std::io::stdin()
            .read_line(&mut line)
            .ok()
            .expect("failed to read line");
        match line.trim() {
            "CREATE" => create(&mut todovec),
            "READ" => read(&todovec),
            "UPDATE" => update(&mut todovec),
            "DELETE" => delete(&mut todovec),
            "END" => break,
            "SAVE" => save(&todovec),
            _ => continue,
        }
    }
}
