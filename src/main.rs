use std::fmt;
use std::collections::HashMap;
use std::rc::Rc;

mod idcounter;
mod muttcounter;
mod person;

use idcounter::IdCounter;
// use muttcounter::MuttCounter;
use person::Person;

pub struct Other {
    id: i32
}

impl fmt::Display for Other {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "den har id {}", self.id.to_string())
    }
}

pub struct Data {
    personer: HashMap<i32, Person>,
    other: Vec<Other>,
    komponenter: HashMap<i32, String>,
}

fn main() {
    let mut counter = IdCounter::new();
    let kajsa = Person::new("Kajsa", 10, counter.nextId());
    let pelle = Person::new("Pelle", 13, counter.nextId());

    let mut barnen = Vec::new();
    barnen.push(kajsa);
    barnen.push(pelle);

    for barn in barnen {
        barn.bajsa();
    }

    let mut gillarglass = HashMap::new();
    gillarglass.insert("Pelle", "Igloo");
    gillarglass.insert("Kajsa", "Dajmstrut");

    for (barn, glass) in gillarglass {
        println!("{} gillar {}", barn, glass);
    }
}
