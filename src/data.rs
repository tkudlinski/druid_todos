use std::{
    fs::File,
    io::{BufReader, Error},
};

use druid::{im::Vector, Data, Env, EventCtx, Lens};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::delegate::DELETE;

const DB_FILE: &str = "todos.json";

#[derive(Clone, Data, Lens)]
pub struct Todos {
    new_todo: String,
    todos: Vector<TodoItem>,
}

impl Todos {
    pub fn delete_todo(&mut self, id: &Uuid) {
        self.todos.retain(|item| &item.id != id);

        self.save_to_json().unwrap();
    }

    fn add_todo(&mut self) {
        self.todos.push_front(TodoItem::new(&self.new_todo));
        self.new_todo = "".into();
        self.save_to_json().unwrap();
    }

    pub fn click_add(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.add_todo();
    }

    pub fn save_to_json(&self) -> Result<(), Error> {
        let todo_vec: Vec<TodoItem> = self.todos.iter().map(|item| item.to_owned()).collect();
        let serialized = serde_json::to_string_pretty(&todo_vec)?;
        std::fs::write(DB_FILE, serialized)?;
        Ok(())
    }

    pub fn load_from_json() -> Self {
        let file = File::open(DB_FILE);

        match file {
            Ok(file) => {
                let reader = BufReader::new(file);
                let todos: Vec<TodoItem> = serde_json::from_reader(reader).unwrap_or(vec![]);
                Self {
                    todos: Vector::from(todos),
                    new_todo: String::new(),
                }
            }
            Err(_) => Self {
                todos: Vector::new(),
                new_todo: String::new(),
            },
        }
    }

    pub fn clear_completed(_ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        data.todos.retain(|item| !item.done);

        data.save_to_json().unwrap();
    }
}

#[derive(Clone, Data, Lens, Serialize, Deserialize)]
pub struct TodoItem {
    #[data(same_fn = "PartialEq::eq")]
    pub id: Uuid,
    pub done: bool,
    pub text: String,
}

impl TodoItem {
    pub fn new(text: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            done: false,
            text: text.into(),
        }
    }
    pub fn click_delete(ctx: &mut EventCtx, data: &mut Self, _env: &Env) {
        ctx.submit_command(DELETE.with(data.id));
    }
}

impl Todos {
    /*fn new() -> Result<Todos, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(DB_FILE)?;
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todos { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                todos: Vector::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(DB_FILE)?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }*/
}
