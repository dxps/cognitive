use shlib::domain::model::Id;

pub fn new_id() -> Id {
    Id {
        0: nid::Nanoid::<10>::new().to_string(),
    }
}
