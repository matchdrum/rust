fn new_stack(maxsize: usize) -> Vec<char> {
    let vec: Vec<char> = Vec::with_capacity(maxsize);
    vec
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    let popped: Option<char> = stack.pop();
    popped
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if (stack.len() == maxsize) {
        println!("Max length already!");
    } else {
        stack.push(item);
        println!("Stack {:?}", stack);
    }
}

fn size(stack: &mut Vec<char>, item: char, maxsize: usize) {
    if (stack.len() == maxsize) {
        println!("Max length already!");
    } else {
        stack.push(item);
        println!("Stack {:?}", stack);
    }
}

fn main() {
    let stack = new_stack(5);
    println!("Hello, stack2! {:?}", stack);
}
