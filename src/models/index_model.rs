use search_engine::{
    index::Index,
    types::{FromJson, IndexDoc, PseudoHash, TermFreq},
    utils::generate_uid,
};
use std::{collections::HashMap, path::PathBuf};
/// This struct is the representation of a file path and a dictionnary of term and frequencies for
/// that file
pub struct IndexModel<'a> {
    id: String,
    file_path: String,
    data: &'a TermFreq,
}

impl<'a> IndexModel<'a> {
    pub fn new(id: String, file_path: String, data: &'a TermFreq) -> Self {
        Self {
            id,
            file_path,
            data,
        }
    }
}
#[derive(Clone)]
pub struct StoredIndexModel {
    pub id: String,
    pub data: Index,
}

impl StoredIndexModel {
    pub fn new() -> Self {
        Self {
            id: generate_uid(),
            data: Index::from_json("data/_index-index.json").unwrap(),
        }
    }

    //pub fn find_index(&self, key: &str) -> IndexModel {
    //    //TODO:Implement the logic
    //    let path = PathBuf::from(key);
    //    // let path = String::from(key)
    //    let data = self.data;
    //    let path_string: String = path.to_string_lossy().into_owned();
    //    IndexModel::new(generate_uid(), path_string, data.index)
    //}
}
