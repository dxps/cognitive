use shlib::domain::model::Id;

pub fn new_id() -> Id {
    Id {
        0: nid::Nanoid::<12>::new().to_string(),
    }
}
