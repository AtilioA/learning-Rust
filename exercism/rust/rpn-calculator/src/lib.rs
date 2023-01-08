#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack: Vec<i32> = vec![];

    for input in inputs {
        match input {
            // Add value to the stack
            CalculatorInput::Value(val) => stack.push(*val),
            CalculatorInput::Add => {
                if stack.len() < 2 {
                    return None;
                }
                match stack.pop() {
                    Some(a) => match stack.pop() {
                        Some(b) => stack.push(a + b),
                        None => return None,
                    },
                    None => return None,
                }
            }
            CalculatorInput::Subtract => {
                if stack.len() < 2 {
                    return None;
                }
                match stack.pop() {
                    Some(a) => match stack.pop() {
                        Some(b) => stack.push(b - a),
                        None => return None,
                    },
                    None => return None,
                }
            }
            CalculatorInput::Multiply => {
                if stack.len() < 2 {
                    return None;
                }
                match stack.pop() {
                    Some(a) => match stack.pop() {
                        Some(b) => stack.push(a * b),
                        None => return None,
                    },
                    None => return None,
                }
            }
            CalculatorInput::Divide => {
                if stack.len() < 2 {
                    return None;
                }
                match stack.pop() {
                    Some(a) => match stack.pop() {
                        Some(b) => stack.push(b / a),
                        None => return None,
                    },
                    None => return None,
                }
            }
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        return None;
    }
}
