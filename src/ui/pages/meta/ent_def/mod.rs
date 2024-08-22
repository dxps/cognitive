mod ent_def_list_page;
use std::collections::HashMap;

pub use ent_def_list_page::*;

mod ent_def_new_page;
pub use ent_def_new_page::*;

mod ent_def_page;
pub use ent_def_page::*;

mod ent_def_form;
pub use ent_def_form::*;

use crate::{domain::model::Id, server::fns::list_attribute_defs};

// Commonly used logic.

pub(self) async fn fetch_all_attr_defs() -> HashMap<Id, String> {
    //
    let mut entries = HashMap::new();
    if let Ok(attr_defs) = list_attribute_defs().await {
        attr_defs.iter().for_each(|attr_def| {
            entries.insert(attr_def.id.clone(), attr_def.name.clone());
        });
    }
    entries
}
