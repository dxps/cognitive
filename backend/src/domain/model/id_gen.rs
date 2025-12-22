use nid::{Nanoid, alphabet::Base62Alphabet};
use shlib::domain::model::Id;

pub fn new_id() -> Id {
    let id: Nanoid<10, Base62Alphabet> = Nanoid::new();
    Id { 0: id.to_string() }
}
