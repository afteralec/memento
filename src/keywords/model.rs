use std::{collections::HashMap, default::Default};

use crate::Id;

#[derive(Debug, Clone)]
pub struct Keywords {
    pub ids_by_keyword: HashMap<String, Vec<Id>>,
    pub keywords_by_id: HashMap<Id, Vec<String>>,
    pub sdesc_by_id: HashMap<Id, String>,
}

impl Keywords {
    pub fn new() -> Self {
        Keywords {
            ..Default::default()
        }
    }

    pub fn get_id_by_keyword(
        &self,
        keyword: &str,
        index: usize,
        ignore_id: &Option<Id>,
    ) -> Option<Id> {
        if let Some(id_vec) = self.ids_by_keyword.get(keyword) {
            if let Some(ignore_id) = ignore_id {
                let id_vec: Vec<_> = id_vec
                    .iter()
                    .filter(|id| id != &ignore_id)
                    .cloned()
                    .collect();

                id_vec.get(index).cloned()
            } else {
                id_vec.get(index).cloned()
            }
        } else {
            None
        }
    }

    pub fn add_keyword(&mut self, id: &Id, keyword: &str) {
        let keyword = keyword.to_lowercase();

        if let Some(keyword_vec) = self.keywords_by_id.get_mut(id) {
            keyword_vec.push(keyword.clone());
        } else {
            let keyword_vec = vec![keyword.clone()];
            self.keywords_by_id.insert(*id, keyword_vec);
        }

        if let Some(id_vec) = self.ids_by_keyword.get_mut(&keyword) {
            id_vec.push(*id);
        } else {
            let id_vec = vec![*id];
            self.ids_by_keyword.insert(keyword.clone(), id_vec);
        }
    }

    pub fn add_sdesc_to_id(&mut self, id: &Id, sdesc: &str) {
        self.sdesc_by_id.insert(*id, sdesc.to_string());
    }

    pub fn add_keywords(&mut self, id: &Id, keywords: Vec<String>) {
        for ref keyword in keywords {
            self.add_keyword(id, keyword);
        }
    }

    pub fn rem_keyword(&mut self, keyword: &str) -> Option<String> {
        let keyword = keyword.to_lowercase();
        let mut sdesc_result = None;

        if let Some(id) = self.get_id_by_keyword(&keyword, 0, &None) {
            self.rem_keyword_by_id(&id);
            if let Some(sdesc) = self.sdesc_by_id.remove(&id) {
                let _ = sdesc_result.insert(sdesc);
            };
        };

        sdesc_result
    }

    pub fn rem_keyword_by_id(&mut self, id: &Id) {
        if let Some(keyword_vec) = self.keywords_by_id.remove(id) {
            for keyword in keyword_vec {
                self.ids_by_keyword.remove(&keyword);
            }
        }
    }

    pub fn has_keyword(&mut self, keyword: &str) -> bool {
        self.ids_by_keyword.get(keyword).is_some()
    }
}

impl Default for Keywords {
    fn default() -> Self {
        Keywords {
            ids_by_keyword: HashMap::new(),
            keywords_by_id: HashMap::new(),
            sdesc_by_id: HashMap::new(),
        }
    }
}
