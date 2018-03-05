# simplecc
[![Travis Build Status](https://travis-ci.org/sorz/simplecc-rs.svg?branch=master)](https://travis-ci.org/sorz/simplecc-rs)
[![crates.io](https://img.shields.io/crates/v/simplecc.svg)](https://crates.io/crates/simplecc)
[![API docs](https://docs.rs/simplecc/badge.svg)](https://docs.rs/simplecc)

A simple Chinese Convert library (partially) compatible with
[OpenCC](https://github.com/BYVoid/OpenCC/)'s 
[dictionaries](https://github.com/BYVoid/OpenCC/tree/master/data/dictionary).

* Simple
  
  No complex configurations, all need is a text dictionary and input text.

  Built-in dictionaries included if `builtin_dicts` feature is on.

* Fast

  Using hashmap with tree structure, faster than original OpenCC.

This project is used on
[asstosrt-wasm](https://github.com/sorz/asstosrt-wasm).
