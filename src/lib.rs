use std::collections::HashMap;
use std::io::{Read, BufRead, BufReader};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Dict {
    root: DictNode,
}

#[derive(Debug)]
enum DictNode {
    Leaf {
        key: Box<str>,
        value: Box<str>,
    },
    Node {
        value: Option<Box<str>>,
        tails: HashMap<char, DictNode>,
    }
}

impl DictNode {
    fn new() -> Self {
        DictNode::Node {
            value: None,
            tails: HashMap::new(),
        }
    }

    fn leaf(key: &str, value: Box<str>) -> Self {
        DictNode::Leaf {
            key: key.into(),
            value,
        }
    }

    fn destruct(self) -> (Option<Box<str>>, HashMap<char, DictNode>) {
        match self {
            DictNode::Node { value, tails } => (value, tails),
            DictNode::Leaf { key, value } => {
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
            DictNode::Node { value: self_value, tails }
        } else {
            DictNode::leaf("", value.into())
        }
    }

    fn prefix_match<'a, 'b>(&'a self, query: &'b str)
            -> Option<(&'b str, &'a str)> {
        match self {
            &DictNode::Leaf { ref key, ref value } => {
                if query.starts_with(&**key) {
                    Some((&query[..key.len()], &value))
                } else {
                    None
                }
            },
            &DictNode::Node { ref value, ref tails } => {
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

    /// Load dict from string lines.
    pub fn load_lines<T, S>(lines: T) -> Self
    where T: Iterator<Item=S>,
          S: AsRef<str> {
        let root = lines.filter_map(|line| {
                let mut cols = line.as_ref().splitn(2, '\t');
                let key = cols.next()?;
                let value = cols.next()?.splitn(2, ' ').next()?;
                Some((key.into(), value.into()))
            }).fold(DictNode::new(), |dict, (key, value): (String, String)| {
                dict.add(&key, &value)
            });
        Dict { root }
    }

    /// Load dict file.
    /// The format is the same as OpenCC's text dictionary file.
    /// Unrecognizable data will be silently ignored.
    pub fn load<T>(reader: T) -> Self
    where T: Read {
        let lines = BufReader::new(reader).lines().filter_map(|l| l.ok());
        Dict::load_lines(lines)
    }

    /// Use this dict to convert string.
    /// Return converted text.
    pub fn replace_all(&self, mut text: &str) -> String {
        let mut output = String::with_capacity(text.len());
        while !text.is_empty() {
            match self.root.prefix_match(text) {
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
}
