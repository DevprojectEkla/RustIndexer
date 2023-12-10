use search_engine::{
    types::{IndexDoc, PseudoHash, TermFreq},
    utils::generate_uid,
};
use std::{collections::HashMap, path::PathBuf};
/// This struct is the representation of a file path and a dictionnary of term and frequencies for
/// that file
pub struct IndexModel<'a> {
    id: String,
    file_path: String,
    data: &'a PseudoHash,
}

impl<'a> IndexModel<'a> {
    pub fn new(id: String, file_path: String, data: &'a PseudoHash) -> Self {
        Self {
            id,
            file_path,
            data,
        }
    }
}
#[derive(Clone)]
pub struct StoredIndexModel {
    id: String,
    data: IndexDoc,
}

impl StoredIndexModel {
    pub fn new() -> Self {
        Self {
            id: generate_uid(),
            data: HashMap::new(),
        }
    }

    pub fn find_index(&self, key: &str) -> IndexModel {
        //TODO:Implement the logic
        let path = PathBuf::from(key);
        // let path = String::from(key)
        let data = self.data.get(&path);
        let path_string: String = path.to_string_lossy().into_owned();
        IndexModel::new(
            generate_uid(),
            path_string,
            data.expect("the value should be found for this key"),
        )
    }
}
