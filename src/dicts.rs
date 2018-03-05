//! A set of built-in OpenCC dictionaries.
//! 
//! Turn on `builtin_dicts` feature to enable them.
use Dict;

macro_rules! builtin_dicts {
    ( $x:expr, $( $y:expr ),* ) => {
        {
            let lines = include_str!(
                concat!("../OpenCC/data/dictionary/", $x, ".txt")).lines();
            let mut lines = Box::new(lines) as Box<Iterator<Item=&'static str>>;
            $(
                let text = include_str!(
                    concat!("../OpenCC/data/dictionary/", $y, ".txt"));
                lines = Box::new(lines.chain(text.lines()));
            )*
            Dict::load_lines(lines)
        }
    };
}

lazy_static! {
    /// Simplified Chinese to Traditional Chinese
    pub static ref S2T: Dict = {
        builtin_dicts!("STCharacters", "STPhrases")
    };

    /// Traditional Chinese to Simplified Chinese
    pub static ref T2S: Dict = {
        builtin_dicts!("TSCharacters", "TSPhrases")
    };

}