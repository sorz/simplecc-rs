use super::Dict;

#[test]
fn test_prefix_match() {
    let dict = Dict::load_str("
A\ta'
B\tb' x
C\tc' x xx
ABC\tabc'
ABCD\tabcd'
DDD\tddd'
BB\tbb'");
    assert_eq!(Some(("A", "a'")), dict.roots[0].prefix_match("A"));
    assert_eq!(Some(("B", "b'")), dict.roots[0].prefix_match("BXX"));
    assert_eq!(Some(("ABC", "abc'")), dict.roots[0].prefix_match("ABCX"));
    assert_eq!(Some(("ABCD", "abcd'")), dict.roots[0].prefix_match("ABCDEFG"));
    assert_eq!(None, dict.roots[0].prefix_match("X"));
    assert_eq!(None, dict.roots[0].prefix_match("DD"));
}

#[test]
fn test_dict_simple() {
    let dict = Dict::load_str("
A\ta
B\tb
ABC\txxx
");
    assert_eq!("a", dict.replace_all("A"));
    assert_eq!("ab", dict.replace_all("AB"));
    assert_eq!("xxx", dict.replace_all("ABC"));
    assert_eq!("abxxxa", dict.replace_all("ABABCA"));
    assert_eq!("aXbXab", dict.replace_all("AXBXAB"));
}
