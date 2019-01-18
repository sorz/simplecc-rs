use criterion::{Criterion, criterion_group, criterion_main};
use simplecc::Dict;


fn bench_load_opencc_s2t(c: &mut Criterion) {
    let chars = include_str!("../OpenCC/data/dictionary/STCharacters.txt");
    let phras = include_str!("../OpenCC/data/dictionary/STPhrases.txt");
    c.bench_function("load_opencc_s2t", move |b| {
        b.iter_with_setup(
            || chars.lines().chain(phras.lines()).collect(),
            |lines: Vec<&str>| Dict::load_lines(lines.iter())
        );
    });
}


criterion_group!(
    name = load_dict;
    config = Criterion::default().sample_size(10);
    targets = bench_load_opencc_s2t
);
criterion_main!(load_dict);
