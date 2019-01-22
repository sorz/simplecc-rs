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
use std::collections::HashMap;
use std::io::{Read, BufRead, BufReader};
use std::mem;

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

#[derive(Debug, Clone)]
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
            Node {
                value: None,
                tails: HashMap::new(),
            }
        )
    }

    fn leaf(key: &str, value: Box<str>) -> Self {
        DictNode::Leaf (
            Leaf {
                key: key.into(),
                value,
            }
        )
    }

    fn destruct(self) -> (Option<Box<str>>, HashMap<char, DictNode>) {
        match self {
            DictNode::Node ( Node { value, tails } ) => (value, tails),
            DictNode::Leaf ( Leaf { key, value } ) => {
                let mut tails = HashMap::new();
                let mut key_chars = key.chars();
                let value = if let Some(hash_key) = key_chars.next() {
                    let suffix = key_chars.as_str().into();
                    tails.insert(hash_key, DictNode::leaf(suffix, value));
                    None
                } else {
                    Some(value)
                };
                (value, tails)
            }
        }
    }

    fn add(self, key: &str, value: &str) -> Self {
        let (self_value, mut tails) = self.destruct();
        let mut key_chars = key.chars();
        if let Some(hash_key) = key_chars.next() {
            let suffix = key_chars.as_str().into();
            let node = if let Some(subnode) = tails.remove(&hash_key) {
                subnode.add(suffix, value)
            } else {
                DictNode::leaf(suffix, value.into())
            };
            tails.insert(hash_key, node);
            DictNode::Node ( Node { value: self_value, tails } )
        } else {
            DictNode::Node ( Node { value: Some(value.into()), tails } )
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
        let root = lines.filter_map(|line| {
                let mut cols = line.as_ref().splitn(2, '\t');
                let key = cols.next()?;
                let value = cols.next()?.splitn(2, ' ').next()?;
                Some((key.into(), value.into()))
            }).fold(DictNode::node(), |dict, (key, value): (String, String)| {
                dict.add(&key, &value)
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
