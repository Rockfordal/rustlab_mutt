// mod muttcounter;

use std::rc::Rc;
use std::ops::Deref;
use std::cell::{RefCell, RefMut};
// pub use muttcounter::MuttCounter;

pub struct Person {
    id: i32,
    name: String,
    age: i32,
}

impl Person { 
    // pub fn new(name: &'static str, age: i32, mut c: super::MuttCounter) -> Person {
    pub fn new(name: &'static str, age: i32, id: i32) -> Person {
        Person {
            id:  id,
            name: name.to_string(),
            age: age,
        }
    }

    pub fn bajsa(&self) {
        println!("Nu ska {}. {} {} år Bajsa!", &self.id, &self.name, &self.age);
    }
}
