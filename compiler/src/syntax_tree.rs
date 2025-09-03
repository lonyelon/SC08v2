fn is_space(c: char) -> bool {
    c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

pub fn clean(code: &str) -> String {
    let mut start = 0;
    while is_space(code.chars().nth(start).unwrap()) {
        start += 1;
    }

    let mut end = code.len() - 1;
    while is_space(code.chars().nth(end).unwrap()) {
        end -= 1;
    }

    String::from(code[start..end + 1].replace("\t", " ").replace("\n", " "))
}

pub struct TreeNode {
    pub value: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        TreeNode {
            value: String::new(),
            children: Vec::new(),
        }
    }

    pub fn from(code: &str) -> TreeNode {
        let mut element = TreeNode::new();

        let mut children = Vec::new();
        if code.chars().nth(0).unwrap() == '(' && code.chars().nth(code.len() - 1).unwrap() == ')' {
            let mut i = 1;

            while code.chars().nth(i).unwrap() != ' ' && code.chars().nth(i).unwrap() != ')' && i < code.len() {
                element.value.push(code.chars().nth(i).unwrap());
                i += 1;
            }

            while code.chars().nth(i).unwrap() == ' ' && i < code.len() {
                i += 1;
            }

            if code.chars().nth(i).unwrap() != ')' {
                while i < code.len() - 1 {
                    if code.chars().nth(i).unwrap() == '(' {
                        let start = i;
                        let mut parenthesis_count = 0;
                        while i < code.len() - 1 {
                            match code.chars().nth(i).unwrap() {
                                '(' => parenthesis_count += 1,
                                ')' => parenthesis_count -= 1,
                                _ => {},
                            };
                            if parenthesis_count == 0 {
                                break;
                            }
                            i += 1;
                        }
                        i += 1;
                        children.push(&code[start..i]);
                    } else {
                        let start = i;
                        while code.chars().nth(i).unwrap() != ' ' && i < code.len() - 1 {
                            i += 1;
                        }
                        children.push(&code[start..i]);
                    }
                    while code.chars().nth(i).unwrap() == ' ' && i < code.len() {
                        i += 1;
                    }
                }
            }
        } else {
            element.value = String::from(code);
        }
        
        for arg in children {
            let child_node = TreeNode::from(arg);
            element.children.push(child_node);
        }

        return element;
    }

    pub fn print(&self, depth: u64) {
        for _ in 0..depth {
            print!(" ");
        }
        println!("- {}", self.value);
        for child in &self.children {
            child.print(depth + 2);
        }
    }
}
