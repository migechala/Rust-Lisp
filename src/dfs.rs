fn run_lisp(query: &str) -> HashMap<String, LispType> {
    let mut operand_stack = Stack::new();
    let mut operator_stack = Stack::new();
    let scope: HashMap<String, LispType> = HashMap::new();

    let mut i = 0;
    while i < query.len() {
        match query.as_bytes()[i] {
            b'+' => operator_stack.push(LispType::Char('+')),
            b'-' => operator_stack.push(LispType::Char('-')),
            b'a'..b'z' => {
                let b = i;
                while i < query.len()
                    && ('a'..='z')
                        .contains(unsafe { &char::from_u32_unchecked(query.as_bytes()[i].into()) })
                {
                    i += 1;
                    println!("{i}")
                }
                if let Some(value) = query.get(b..i) {
                    operator_stack.push(LispType::String(value.to_string()));
                } else {
                    println!("Something went wrong...")
                }
            }
            b'0'..b'9' => {
                let b = i;
                while i < query.len()
                    && ('0'..='9')
                        .contains(unsafe { &char::from_u32_unchecked(query.as_bytes()[i].into()) })
                {
                    i += 1;
                    println!("{i}")
                }
                if let Some(value) = query.get(b..i) {
                    if let Ok(number) = value.parse::<i32>() {
                        operand_stack.push(LispType::Number(number));
                    } else {
                        println!("Something went wrong...")
                    }
                } else {
                    println!("Something went wrong...")
                }
            }

            _ => {}
        }
        i += 1;
    }
    let tokens: Vec<&str> = query.split_whitespace().collect();
    for i in tokens {
        match i {
            "+" => operator_stack.push(LispType::Char('+')),
            "-" => operator_stack.push(LispType::Char('-')),
            "*" => operator_stack.push(LispType::Char('*')),
            "/" => operator_stack.push(LispType::Char('/')),
            ")" => {
                //Calculate
                if let Some(operator) = operator_stack.pop() {
                    match operator {
                        LispType::Char(j) => {
                            let r = match operand_stack.pop() {
                                Some(LispType::Number(i)) => i,
                                Some(LispType::String(i)) => {
                                    if let Some(value) = scope.get(&i) {
                                        match value {
                                            LispType::Number(cur_num) => *cur_num,
                                            _ => panic!("Expected a number in scope for key '{i}'"),
                                        }
                                    } else {
                                        panic!("Key '{i}' not found in scope");
                                    }
                                }
                                Some(other) => {
                                    panic!("Unexpected LispType: {:?}", other);
                                }
                                None => {
                                    panic!("Operand stack is empty");
                                }
                            };
                            let l = match operand_stack.pop() {
                                Some(LispType::Number(i)) => i,
                                Some(LispType::String(i)) => {
                                    if let Some(value) = scope.get(&i) {
                                        match value {
                                            LispType::Number(cur_num) => *cur_num,
                                            _ => panic!("Expected a number in scope for key '{i}'"),
                                        }
                                    } else {
                                        panic!("Key '{i}' not found in scope");
                                    }
                                }
                                Some(other) => {
                                    panic!("Unexpected LispType: {:?}", other);
                                }
                                None => {
                                    panic!("Operand stack is empty");
                                }
                            };
                            match j {
                                '+' => operand_stack.push(LispType::Number(r + l)),
                                '-' => operand_stack.push(LispType::Number(l - r)),
                                '*' => operand_stack.push(LispType::Number(r * l)),
                                '/' => operand_stack.push(LispType::Number(l / r)),
                                _ => {
                                    panic!("Err, invalid token {j}");
                                }
                            }
                        }
                        LispType::Command(command) => match command {
                            commands::LispCommands::WriteToConsole => match operand_stack.top() {
                                Some(to_be_written) => match to_be_written {
                                    LispType::String(w) => {
                                        if scope.contains_key(w) {
                                            match scope.get(w) {
                                                Some(p_var) => match p_var {
                                                    LispType::Char(p) => println!("{}", p),
                                                    LispType::Number(p) => println!("{}", p),
                                                    LispType::String(p) => println!("{}", p),
                                                    _ => panic!("Hashmap error"),
                                                },
                                                None => unreachable!(""),
                                            }
                                        }
                                    }
                                    LispType::Number(w) => println!("{}", w),
                                    _ => panic!("Not a valid type for console output"),
                                },
                                None => unreachable!("Operand stack should not be empty"),
                            },
                            commands::LispCommands::SetVar => {
                                let r = match operand_stack.pop() {
                                    Some(LispType::Number(i)) => i,
                                    _ => {
                                        panic!("Err, bad token {i}")
                                    }
                                };
                                let l = match operand_stack.pop() {
                                    Some(LispType::String(i)) => i,
                                    Some(LispType::Char(i)) => String::from(i),
                                    _ => {
                                        panic!("Err, bad token {i}")
                                    }
                                };
                                scope.insert(l, LispType::Number(r));
                            }
                        },
                        _ => {
                            operator_stack.push(LispType::String(i.to_string()));
                        }
                    };
                }
            }
            "(" => {}

            _ => {
                if let Ok(number) = i.parse::<i32>() {
                    operand_stack.push(LispType::Number(number));
                } else if let Ok(string) = i.parse::<String>() {
                    operand_stack.push(LispType::String(string));
                } else {
                    panic!("Err, invalid token {i}")
                }
            }
        }
    }
    if operand_stack.len() > 1 {
        panic!("Err, too many tokens")
    }
    operand_stack.pop();
    scope
}
