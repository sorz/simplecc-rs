//! A set of built-in OpenCC dictionaries.
//! 
//! Turn on `builtin_dicts` feature to enable them.
use Dict;

lazy_static! {
    /// Simplified Chinese to Traditional Chinese
    pub static ref S2T: Dict = {
        let dict = concat!(
            include_str!("../OpenCC/data/dictionary/STCharacters.txt"),
            include_str!("../OpenCC/data/dictionary/STPhrases.txt"),
        );
        Dict::load_str(dict)
    };

    /// Traditional Chinese to Simplified Chinese
    pub static ref T2S: Dict = {
        let dict = concat!(
            include_str!("../OpenCC/data/dictionary/TSCharacters.txt"),
            include_str!("../OpenCC/data/dictionary/TSPhrases.txt"),
        );
        Dict::load_str(dict)
    };

}