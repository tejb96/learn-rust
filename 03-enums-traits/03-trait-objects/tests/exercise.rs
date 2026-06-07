// DO NOT EDIT — implement the solution in src/lib.rs

use trait_objects::{pipeline, Formatter, Prefix, Uppercase};

#[test]
fn uppercase_formats() {
    let f = Uppercase;
    assert_eq!(f.format("hi"), "HI");
}

#[test]
fn prefix_formats() {
    let f = Prefix { tag: "[log]".into() };
    assert_eq!(f.format("start"), "[log] start");
}

#[test]
fn pipeline_chains() {
    let formatters: Vec<Box<dyn Formatter>> = vec![
        Box::new(Prefix { tag: ">".into() }),
        Box::new(Uppercase),
    ];
    assert_eq!(pipeline("go", &formatters), "> GO");
}
