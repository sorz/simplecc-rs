//! A simple Chinese Convert library (partially) compatible with
//! [OpenCC](https://github.com/BYVoid/OpenCC/)'s 
//! [dictionaries](https://github.com/BYVoid/OpenCC/tree/master/data/dictionary).
//! 
//! * Simple
//! 
//!   No complex configurations, all need is a text dictionary and input text.
//! 
//!   Built-in dictionaries included if `builtin_dicts` feature is on.
//! 
//! * Fast
//! 
//!   Use hashmap with tree structure for dictionary, faster than original
//!   OpenCC.
use std::{
    collections::HashMap,
    collections::hash_map::Entry,
    io::{Read, BufRead, BufReader},
    mem,
};

#[cfg(feature = "builtin_dicts")]
pub mod dicts;

#[cfg(test)]
mod tests;

/// Dictionary to convert text.
/// 
/// This library doesn't contain dictionary file. You may download them from
/// [OpenCC's repo](https://github.com/BYVoid/OpenCC/tree/master/data/dictionary).
/// 
/// For chaining multiple dicts, just concat all to one file (in any order).
/// 
/// # Built-in dictionaries
/// The library includes optional dictionaries when `builtin_dicts`
/// feature is on. Disabled by default.
/// 
///
/// # File Format
/// The format is the same as text format (not the compiled binary one)
/// in OpenCC project.
/// 
/// Specifically, one rule per line, two columns per rule splitted by a TAB
/// (`\t`). First column is original word/phrase, the other is converted one,
/// which may contains multiples word/phrase splitted by a space (` `), but
/// only the first one is used, others will be ignored. 
/// 
#[derive(Debug, Clone)]
pub struct Dict {
    roots: Vec<DictNode>,
}

#[derive(Debug, Clone)]
struct Leaf {
    key: Box<str>,
    value: Box<str>,
}

#[derive(Debug, Clone, Default)]
struct Node {
    value: Option<Box<str>>,
    tails: HashMap<char, DictNode>,
}

#[derive(Debug, Clone)]
enum DictNode {
    Leaf (Leaf),
    Node (Node),
}

impl DictNode {
    fn node() -> Self {
        DictNode::Node (
            Node::default()
        )
    }

    fn unwrap_node_mut(&mut self) -> &mut Node {
        match self {
            DictNode::Node (node) => node,
            DictNode::Leaf (_) => panic!("expect Node, found Leaf"),
        }

    }

    fn into_leaf(self) -> Leaf {
        match self {
            DictNode::Leaf (leaf) => leaf,
            DictNode::Node (_) => panic!("expect Leaf, found Node"),
        }
    }

    fn leaf(key: &str, value: Box<str>) -> Self {
        DictNode::Leaf (
            Leaf {
                key: key.into(),
                value,
            }
        )
    }

    fn add(&mut self, key: &str, value: Box<str>) {
        let self_node = match self {
            DictNode::Node (node) => node,
            DictNode::Leaf (_) => {
                let node = Node::default();
                let leaf = mem::replace(self, DictNode::Node(node));
                let Leaf { key, value } = leaf.into_leaf();
                let mut node = self.unwrap_node_mut();
                let mut key_chars = key.chars();
                node.value = if let Some(hash_key) = key_chars.next() {
                    let suffix = key_chars.as_str().into();
                    node.tails.insert(hash_key, DictNode::leaf(suffix, value));
                    None
                } else {
                    Some(value)
                };
                node
            }
        };

        let mut key_chars = key.chars();
        if let Some(hash_key) = key_chars.next() {
            let suffix = key_chars.as_str().into();
            match self_node.tails.entry(hash_key) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().add(suffix, value);
                }
                Entry::Vacant(entry) => {
                    entry.insert(DictNode::leaf(suffix, value));
                }
            };
        } else {
            self_node.value = Some(value);
        }
    }

    fn prefix_match<'a, 'b>(&'a self, query: &'b str)
            -> Option<(&'b str, &'a str)> {
        match self {
            &DictNode::Leaf ( Leaf { ref key, ref value } ) => {
                if query.starts_with(&**key) {
                    Some((&query[..key.len()], &value))
                } else {
                    None
                }
            },
            &DictNode::Node ( Node { ref value, ref tails } ) => {
                let mut query_chars = query.chars();
                let hash_key = query_chars.next();
                let suffix = query_chars.as_str();

                hash_key.and_then(|hash_key| {
                    tails.get(&hash_key)
                        .and_then(|node| node.prefix_match(suffix))
                        .map(|(prefix, value)| {
                            let n = query.len() - suffix.len() + prefix.len();
                            (&query[..n], value)
                        })
                }).or_else(||
                    value.as_ref().map(|v| ("", &**v))
                )
            }
        }
    }
}

impl Dict {
    /// Load dict from string
    pub fn load_str<T>(raw: T) -> Self
    where T: AsRef<str> {
        Dict::load_lines(raw.as_ref().lines())
    }

    /// Load dict from lines of string.
    pub fn load_lines<T, S>(lines: T) -> Self
    where T: Iterator<Item=S>,
          S: AsRef<str> {
        let mut root = DictNode::node();
        lines.filter_map(|line| {
            let mut cols = line.as_ref().splitn(2, '\t');
            let key = cols.next()?;
            let value = cols.next()?.splitn(2, ' ').next()?;
            Some((key.into(), value.into()))
        }).for_each(|(key, value): (String, Box<str>)| {
            root.add(&key, value)
        });
        Dict { roots: vec![root] }
    }

    /// Load dict file.
    /// Unrecognizable data will be silently ignored.
    pub fn load<T>(reader: T) -> Self
    where T: Read {
        let lines = BufReader::new(reader).lines().filter_map(|l| l.ok());
        Dict::load_lines(lines)
    }

    /// Return the new dict that chained together.
    pub fn chain(self, other: Dict) -> Self {
        let Dict { mut roots } = self;
        roots.extend(other.roots);
        Dict { roots }
    }

    fn replace(dict: &DictNode, mut text: &str) -> String {
        let mut output = String::with_capacity(text.len());
        while !text.is_empty() {
            match dict.prefix_match(text) {
                Some((prefix, value)) => {
                    output.push_str(value);
                    text = &text[prefix.len()..];
                },
                None => {
                    let mut chars = text.chars();
                    output.push(chars.next().unwrap());
                    text = chars.as_str();
                }
            }
        }
        output
    }

    /// Use this dict to convert string.
    /// Return converted text.
    pub fn replace_all(&self, text: &str) -> String {
        let mut buffer = Dict::replace(&self.roots[0], text);
        for dict in &self.roots[1..] {
            buffer = Dict::replace(dict, &buffer);
        }
        buffer
    }
}
