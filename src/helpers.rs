use std::{cmp::max, collections::BTreeMap, path::PathBuf};

use crate::structs::ListItem;

pub fn insert_parents(list: &mut BTreeMap<PathBuf, ListItem>, item: &ListItem) {
    let mut path = item.key.clone();
    while let Some(parent) = path.parent().map(|x| x.to_owned()) {
        if parent == PathBuf::from("/") {
            break;
        }
        match list.entry(parent.clone()) {
            std::collections::btree_map::Entry::Vacant(v) => {
                v.insert(ListItem {
                    key: parent.join(""),
                    last_modified: item.last_modified,
                    size: 0,
                    etag: String::new(),
                });
            }
            std::collections::btree_map::Entry::Occupied(mut o) => {
                o.get_mut().last_modified = max(o.get().last_modified, item.last_modified);
            }
        }
        path = parent;
    }
}
