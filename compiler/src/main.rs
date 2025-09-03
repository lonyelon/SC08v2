use std::env;
use std::fs;

mod compiler;
mod syntax_tree;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Argument #1 missing: source file path");
    }

    let file_contents = fs::read_to_string(&args[1]).unwrap();
    let code = syntax_tree::clean(&file_contents);
    let tree = syntax_tree::TreeNode::from(&code);
    let asm = compiler::compile(&tree);
    for e in asm {
        println!("{}\t{}\t{}", e.0, e.1, e.2);
    }
}
