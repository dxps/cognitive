use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub const APP_LOCALSTORAGE_KEY: &str = "cognitive_state";

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UiStorage<T>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub data: Option<T>,
    localstorage: Option<web_sys::Storage>,
    ls_key: String,
}

impl<T> UiStorage<T>
where
    T: Clone + Debug + Serialize + for<'de> Deserialize<'de>,
{
    pub fn new(ls_key: &str) -> Result<Self, String> {
        let mut data = None;
        let w = web_sys::window().expect("No global `window` exists!");
        if let Ok(Some(storage)) = w.local_storage() {
            if let Ok(Some(value)) = storage.get(ls_key) {
                if let Ok(d) = serde_json::from_str(&value) {
                    data = Some(d);
                }
            }
            let state = Self {
                data,
                localstorage: Some(storage),
                ls_key: ls_key.to_string(),
            };
            Ok(state)
        } else {
            Err("No localstorage found".into())
        }
    }

    pub fn save_to_localstorage(&mut self) {
        //
        if self.data.is_some() {
            let data = self.data.as_ref().unwrap();
            self.localstorage
                .as_ref()
                .unwrap()
                .set_item(&self.ls_key, &serde_json::to_string(&data).unwrap())
                .unwrap();
            debug!(">>> [UiStorage] Saved to localstorage: {:#?}", data);
        } else {
            self.localstorage.as_ref().unwrap().remove_item(&self.ls_key).unwrap();
            debug!(">>> [UiStorage] Removed from localstorage.");
        }
    }
}
