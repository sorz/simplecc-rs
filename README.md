# simplecc
[![crates.io](https://img.shields.io/crates/v/simplecc.svg)](https://crates.io/crates/simplecc)
[![API docs](https://docs.rs/simplecc/badge.svg)](http://docs.rs/simplecc)

A simple Chinese Convert library (partially) compatible with
[OpenCC](https://github.com/BYVoid/OpenCC/)'s 
[dictionaries](https://github.com/BYVoid/OpenCC/tree/master/data/dictionary).

* Simple
  
  No complex configurations, all need is a text dictionary and input text.

* Fast

  Use hashmap with tree structure for dictionary, faster than original OpenCC.

This project is used on
[asstosrt-wasm](https://github.com/sorz/asstosrt-wasm).
