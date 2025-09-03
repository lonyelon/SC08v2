use std::collections::HashMap;
use super::syntax_tree;

#[derive(PartialEq)]
enum ArgumentType {
    Number,
    Variable,
    Expression
}

fn get_argument_type(argument: &syntax_tree::TreeNode)-> ArgumentType {
    if argument.children.len() > 0 {
        return ArgumentType::Expression;
    }
    match argument.value.chars().nth(0).unwrap() {
        '0'..'9' => ArgumentType::Number,
        'A'..'z' => ArgumentType::Variable,
        _ => panic!("Invalid argument name")
    }
}


fn _compile(node: &syntax_tree::TreeNode, tag_count: &mut u64) -> Vec<(String, String, String)> {
    let mut result = Vec::new();
    match node.value.as_str() {
        ">" => {
            result.push((String::new(), String::from("LDD.ram"), node.children.get(0).unwrap().value.clone()));
            result.push((String::new(), String::from("GRE.ram"), node.children.get(1).unwrap().value.clone()));
        },
        "=" => {
            let y = node.children.get(1).unwrap();
            match get_argument_type(&y) {
                ArgumentType::Number => result.push((String::new(), String::from("LDD.rom"), y.value.clone())),
                ArgumentType::Variable => result.push((String::new(), String::from("LDD.ram"), y.value.clone())),
                ArgumentType::Expression => {
                    for e in _compile(y, tag_count) {
                        result.push(e);
                    }
                },
            }
            let x = node.children.get(0).unwrap();
            if get_argument_type(&x) == ArgumentType::Variable {
                result.push((String::new(), String::from("STD.ram"), x.value.clone()));
            } else {
                panic!("#1 has to be a variable in '='.");
            }
        },
        "==" => {
            result.push((String::new(), String::from("LDD.ram"), node.children.get(0).unwrap().value.clone()));
            result.push((String::new(), String::from("EQU.ram"), node.children.get(1).unwrap().value.clone()));
        },
        "+=" => {
            result.push((String::new(), String::from("LDD.ram"), node.children.get(0).unwrap().value.clone()));
            result.push((String::new(), String::from("ADD.num"), node.children.get(1).unwrap().value.clone()));
            result.push((String::new(), String::from("STD.ram"), node.children.get(0).unwrap().value.clone()));
        },
        "-=" => {
            if node.children.len() != 2 {
                panic!("'-=' requires exactly 2 children.");
            }

            let x = node.children.get(0).unwrap();
            if get_argument_type(&x) != ArgumentType::Variable {
                panic!("#1 has to be a variable in '='.");
            }
            result.push((String::new(), String::from("LDD.ram"), node.children.get(0).unwrap().value.clone()));

            let y = node.children.get(1).unwrap();
            match get_argument_type(&y) {
                ArgumentType::Number => result.push((String::new(), String::from("SUB.num"), y.value.clone())),
                ArgumentType::Variable => result.push((String::new(), String::from("SUB.ram"), y.value.clone())),
                ArgumentType::Expression => panic!("#2 of '-=' cannot be a expression")
            }

            result.push((String::new(), String::from("STD.ram"), node.children.get(0).unwrap().value.clone()));
        },
        "asm" => {
            match node.children.len() {
                3 => {
                    result.push((
                        node.children.get(0).unwrap().value.clone(),
                        node.children.get(1).unwrap().value.clone(),
                        node.children.get(2).unwrap().value.clone()
                    ));
                },
                2 => {
                    result.push((
                        String::new(),
                        node.children.get(0).unwrap().value.clone(),
                        node.children.get(1).unwrap().value.clone()
                    ));
                },
                1 => {
                    result.push((
                        String::new(),
                        node.children.get(0).unwrap().value.clone(),
                        String::new()
                    ));
                },
                _ => {
                    panic!("Wrong number of arguments for 'asm'.");
                }
            }

        }
        "halt" => {
            if node.children.len() != 0 {
                panic!("'halt' cannot take any arguments.");
            }

            result.push((format!(":halt-{}", tag_count), String::from("JUM.num"), format!(":halt-{}", *tag_count)));
            *tag_count += 1;
        },
        "if" => {
            let current_tag_count = *tag_count;
            *tag_count += 1;

            match node.children.len() {
                3 => {
                    for e in _compile(node.children.get(0).unwrap(), tag_count) {
                        result.push(e);
                    }
                    result.push((String::new(), String::from("JUM.num"), format!(":else-{}", current_tag_count)));
                    for e in _compile(node.children.get(1).unwrap(), tag_count) {
                        result.push(e);
                    }
                    result.push((String::new(), String::from("LDD.num"), String::from("0xFF")));
                    result.push((String::new(), String::from("JUM.num"), format!(":endif-{}", current_tag_count)));
                    result.push((String::from(":else"), String::from("NOI.noa"), String::new()));
                    for e in _compile(node.children.get(2).unwrap(), tag_count) {
                        result.push(e);
                    }
                    result.push((format!(":endif-{}", current_tag_count), String::from("NOI.noa"), String::new()));
                },
                2 => {
                    for e in _compile(node.children.get(0).unwrap(), tag_count) {
                        result.push(e);
                    }
                    result.push((String::new(), String::from("JUM.num"), format!(":endif-{}", current_tag_count)));
                    for e in _compile(node.children.get(1).unwrap(), tag_count) {
                        result.push(e);
                    }
                    result.push((format!(":endif-{}", current_tag_count), String::from("NOI.noa"), String::new()));
                },
                _ => {
                    panic!("Wrong number of arguments for 'if'.");
                }
            }
        },
        "input" => {
            if node.children.len() != 1 {
                panic!("'input' requires exactly 1 child.");
            }

            let x = node.children.get(0).unwrap();
            if get_argument_type(&x) != ArgumentType::Variable {
                panic!("#1 for 'input' must be a variable name.");
            }

            result.push((String::new(), String::from("LDD.inp"), String::new()));
            result.push((String::new(), String::from("STD.ram"), x.value.clone()));
        },
        "ins" => {
            for child in &node.children {
                for e in _compile(child, tag_count) {
                    result.push(e);
                }
            }
        },
        "output" => {
            if node.children.len() != 1 {
                panic!("'output' requires exactly 1 child.");
            }

            let x = node.children.get(0).unwrap();
            if get_argument_type(&x) != ArgumentType::Variable {
                panic!("#1 for 'output' must be a variable name.");
            }

            result.push((String::new(), String::from("LDD.ram"), x.value.clone()));
            result.push((String::new(), String::from("STD.out"), String::new()));
        },
        "var" => {
            if node.children.len() != 3 {
                panic!("'var' requires exactly 3 children.");
            }

            let x = node.children.get(0).unwrap();
            if get_argument_type(&x) != ArgumentType::Variable {
                panic!("#1 for 'var' has to be a variable name.");
            }

            let y = node.children.get(1).unwrap();
            if get_argument_type(&y) != ArgumentType::Number {
                panic!("#2 for 'var' has to be a number (memory address).");
            }

            let z = node.children.get(2).unwrap();
            if get_argument_type(&z) != ArgumentType::Expression {
                panic!("#3 for 'var' has to be an expression.");
            }

            result.push((String::from("#var"), x.value.clone(), y.value.clone()));
            for e in _compile(node.children.get(2).unwrap(), tag_count) {
                result.push(e);
            }
        },
        "while" => {
            let current_tag_count = *tag_count;
            *tag_count += 1;

            result.push((format!(":while-{}", current_tag_count), String::from("NOI.noa"), String::new()));
            for e in _compile(node.children.get(0).unwrap(), tag_count) {
                result.push(e);
            }
            result.push((String::new(), String::from("EQU.num"), String::from("0x00")));
            result.push((String::new(), String::from("JUM.num"), format!(":endwhile-{}", current_tag_count)));
            for e in _compile(node.children.get(1).unwrap(), tag_count) {
                result.push(e);
            }
            result.push((String::new(), String::from("LDD.num"), String::from("0xFF")));
            result.push((String::new(), String::from("JUM.num"), format!(":while-{}", current_tag_count)));
            result.push((format!(":endwhile-{}", current_tag_count), String::from("NOI.noa"), String::new()));
        },
        &_ => {},
    }
    result
}

fn _clean(inp: Vec<(String, String, String)>) -> Vec<(String, String, String)> {
    let mut result = Vec::new();
    let mut last_tag = String::new();
    let mut tag_renames = HashMap::<String, String>::new();
    for ins in &inp {
        if ins.1.as_str() == "NOI.noa" {
            if ins.0.len() > 0 {
                if last_tag.len() == 0 {
                    last_tag = ins.0.clone();
                } else {
                    tag_renames.insert(ins.0.clone(), last_tag.clone());
                }
            }
        } else {
            if last_tag.len() == 0 {
                result.push(ins.clone());
            } else {
                if ins.0.len() > 0 {
                    tag_renames.insert(ins.0.clone(), last_tag.clone());
                }

                result.push((last_tag.clone(), ins.1.clone(), ins.2.clone()));
                last_tag = String::new();
            }
        }
    }

    for e in &mut result {
        if tag_renames.contains_key(&e.0) {
            e.0 = tag_renames.get(&e.0).unwrap().clone();
        }
        if tag_renames.contains_key(&e.2) {
            e.2 = tag_renames.get(&e.2).unwrap().clone();
        }
    }
    result
}

pub fn compile(node: &syntax_tree::TreeNode) -> Vec<(String, String, String)> {
    _clean(_compile(node, &mut 0))
}
