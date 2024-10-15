///!
///! Commonly used logic.
///!
use crate::domain::model::Id;
use crate::server::fns::list_attribute_defs;
use std::collections::HashMap;

pub async fn fetch_all_attr_defs() -> HashMap<Id, String> {
    //
    let mut entries = HashMap::new();
    if let Ok(attr_defs) = list_attribute_defs().await {
        attr_defs.iter().for_each(|attr_def| {
            entries.insert(attr_def.id.clone(), attr_def.name.clone());
        });
    }
    entries
}
