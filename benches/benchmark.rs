use criterion::{Criterion, criterion_group, criterion_main};
use simplecc::Dict;


fn bench_load_opencc_s2t(c: &mut Criterion) {
    let chars = include_str!("../OpenCC/data/dictionary/STCharacters.txt");
    let phras = include_str!("../OpenCC/data/dictionary/STPhrases.txt");
    c.bench_function("load_opencc_s2t", move |b| {
        b.iter(|| {
            let lines = chars.lines().chain(phras.lines());
            Dict::load_lines(lines)
        });
    });
}


criterion_group!(load_dict, bench_load_opencc_s2t);
criterion_main!(load_dict);
