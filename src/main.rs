use std::collections::HashMap;

#[derive(Debug, Clone)]
enum LispType {
    Char(char),
    Number(i32),
    String(String),
}

#[derive(Debug)]
struct Stack {
    stack_vec: Vec<LispType>,
}
#[allow(dead_code)]
impl Stack {
    fn new() -> Self {
        Self {
            stack_vec: Vec::new(),
        }
    }

    fn push(&mut self, a: LispType) {
        self.stack_vec.push(a);
    }

    fn pop(&mut self) -> Option<LispType> {
        self.stack_vec.pop()
    }

    fn top(&self) -> Option<&LispType> {
        self.stack_vec.last()
    }
}

fn run_lisp(query: &str) -> HashMap<String, LispType> {
    let mut operand_stack = Stack::new();
    let mut operator_stack = Stack::new();
    let mut scope: HashMap<String, LispType> = HashMap::new();
    let mut function_registry: HashMap<String, fn(&mut Stack, &mut HashMap<String, LispType>)> =
        HashMap::new();

    function_registry.insert("put".to_string(), |stack, scope| {
        if let Some(LispType::String(var)) = stack.pop() {
            if let Some(value) = scope.get(&var) {
                println!("{} = {:?}", var, value);
            } else {
                panic!("'{}' not found in scope", var);
            }
        } else {
            panic!("No variable name for 'put'");
        }
    });

    function_registry.insert("setq".to_string(), |stack, scope| {
        if let Some(value) = stack.pop() {
            if let Some(LispType::String(var)) = stack.pop() {
                scope.insert(var, value);
            } else {
                panic!("No variable name for 'setq'");
            }
        } else {
            panic!("No value for 'setq'");
        }
    });

    let chars: Vec<char> = query.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        println!("{}", chars[i]);
        match chars[i] {
            '+' => operator_stack.push(LispType::Char('+')),
            '-' => operator_stack.push(LispType::Char('-')),
            '*' => operator_stack.push(LispType::Char('*')),
            '/' => operator_stack.push(LispType::Char('/')),
            'a'..='z' => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_lowercase() {
                    i += 1;
                }
                let identifier: String = chars[start..i].iter().collect();

                if function_registry.contains_key(&identifier) {
                    operator_stack.push(LispType::String(identifier));
                } else if scope.contains_key(&identifier) {
                    if let Some(var) = scope.get(&identifier) {
                        operand_stack.push(var.clone());
                    }
                } else {
                    operand_stack.push(LispType::String(identifier));
                }
                i -= 1;
            }
            '0'..='9' => {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                let number_str: String = chars[start..i].iter().collect();
                if let Ok(number) = number_str.parse::<i32>() {
                    operand_stack.push(LispType::Number(number));
                } else {
                    panic!("Error parsing: {}", number_str);
                }
                i -= 1;
            }
            ')' => {
                println!("{:?}", operand_stack);
                println!("{:?}", operator_stack);
                if let Some(operator) = operator_stack.pop() {
                    match operator {
                        LispType::Char(op) => {
                            let right = match operand_stack.pop() {
                                Some(LispType::Number(num)) => num,
                                _ => panic!("Invalid operand for operator '{}'", op),
                            };
                            let left = match operand_stack.pop() {
                                Some(LispType::Number(num)) => num,
                                _ => panic!("Invalid operand for operator '{}'", op),
                            };
                            let result = match op {
                                '+' => left + right,
                                '-' => left - right,
                                '*' => left * right,
                                '/' => left / right,
                                _ => panic!("Unsupported operator: {}", op),
                            };
                            operand_stack.push(LispType::Number(result));
                        }
                        LispType::String(func) => {
                            if let Some(function) = function_registry.get(&func) {
                                function(&mut operand_stack, &mut scope)
                            }
                        }
                        _ => panic!("Unexpected operator: {:?}", operator),
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }

    println!("Final scope: {:?}", scope);
    scope
}

fn main() {
    run_lisp(
        "
        (setq x 3)
        (put x)
        ",
    );
}
