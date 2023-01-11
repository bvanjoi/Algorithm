use std::fmt::Debug;

#[derive(Debug)]
pub struct Trie<T>
where
    T: Clone + Ord + Eq + Debug,
{
    value: Option<T>,
    children: Vec<Trie<T>>,
    terminal: bool,
}

impl<T: Clone + Ord + Eq + Debug> Trie<T> {
    pub fn new() -> Self {
        Self {
            children: vec![],
            value: None,
            terminal: false,
        }
    }

    fn find_in_children(children: &Vec<Trie<T>>, target: &T) -> Result<usize, usize> {
        children.binary_search_by(|node| {
            node.value
                .as_ref()
                .expect("Every node expect root should had value")
                .cmp(target)
        })
    }

    pub fn insert<Q: IntoIterator<Item = T>>(&mut self, value: Q) {
        let mut now = self;
        let mut insert_stage = false;
        let mut insert_index = None;
        for item in value {
            if !insert_stage {
                let mut next_index = None;
                let result = Self::find_in_children(&mut now.children, &item);
                if let Ok(index) = result {
                    next_index = Some(index);
                } else if let Err(index) = result {
                    insert_index = Some(index);
                    insert_stage = true;
                }

                if let Some(index) = next_index {
                    now = &mut now.children[index];
                }
            }

            if insert_stage {
                let node = Trie {
                    value: Some(item),
                    children: vec![],
                    terminal: false,
                };
                if let Some(index) = insert_index {
                    now.children.insert(index, node);
                    now = &mut now.children[index];
                    insert_index = None
                } else {
                    now.children.push(node);
                    let last = now.children.len() - 1;
                    now = &mut now.children[last];
                }
            }
        }
        now.terminal = true;
    }

    fn get_most_common_node<Q: AsRef<[T]>>(&self, value: Q) -> Option<&Trie<T>> {
        let mut now = self;
        for item in value.as_ref().iter() {
            if let Ok(index) = Self::find_in_children(&now.children, &item) {
                now = &now.children[index];
            } else {
                return None;
            }
        }
        Some(now)
    }

    pub fn search<Q: AsRef<[T]>>(&self, value: Q) -> bool {
        dbg!(self);
        self.get_most_common_node(value)
            .map(|node| node.terminal)
            .unwrap_or_default()
    }

    pub fn starts_with<Q: AsRef<[T]>>(&self, value: Q) -> bool {
        self.get_most_common_node(value).is_some()
    }
}

#[test]
fn case0() {
    let mut trie = Trie::new();
    trie.insert("apple".bytes());
    assert!(trie.search("apple".to_string()));
    assert!(!trie.search("app".to_string()));
    assert!(trie.starts_with("app".to_string()));
    trie.insert("app".bytes());
    assert!(trie.search("app".to_string()));
}

#[test]
fn case1() {
    let mut trie = Trie::new();
    trie.insert("app".bytes());
    trie.insert("apple".bytes());
    trie.insert("beer".bytes());
    trie.insert("add".bytes());
    trie.insert("jam".bytes());
    trie.insert("rental".bytes());
    assert!(!trie.search("apps".to_string()));
    assert!(trie.search("app".to_string()));
    assert!(!trie.search("ad".to_string()));
    assert!(!trie.search("applepie".to_string()));
    assert!(!trie.search("rest".to_string()));
    assert!(!trie.search("jan".to_string()));
    assert!(!trie.search("rent".to_string()));
    assert!(trie.search("beer".to_string()));
    assert!(trie.search("jam".to_string()));
    assert!(!trie.starts_with("apps".to_string()));
    assert!(trie.starts_with("app".to_string()));
    assert!(trie.starts_with("ad".to_string()));
    assert!(!trie.starts_with("applepipe".to_string()));
    assert!(!trie.starts_with("rest".to_string()));
    assert!(!trie.starts_with("jan".to_string()));
    assert!(trie.starts_with("rent".to_string()));
    assert!(trie.starts_with("beer".to_string()));
    assert!(trie.starts_with("jam".to_string()));
}

#[test]
fn case2() {
    let mut trie = Trie::new();
    trie.insert("nemathelminth".bytes());
    trie.insert("entracte".bytes());
    assert!(trie.search("nemathelminth".to_string()));
    assert!(trie.search("entracte".to_string()));
    assert!(!trie.search("spittlestaff".to_string()));
    trie.insert("spittlestaff".bytes());
    assert!(!trie.search("hematocrit".to_string()));
    trie.insert("hematocrit".bytes());
    trie.insert("inachid".bytes());
    trie.insert("phthalan".bytes());
    trie.insert("mev".bytes());
    assert!(trie.search("inachid".to_string()));
    assert!(trie.search("phthalan".to_string()));
    assert!(trie.search("mev".to_string()));
    assert!(!trie.search("hematoid".to_string()));
    assert!(!trie.search("kingmaking".to_string()));
    assert!(!trie.search("brent".to_string()));
    trie.insert("hematoid".bytes());
    assert!(!trie.search("epollicate".to_string()));
}
