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
    }
}

fn size(stack: &Vec<char>) -> usize {
    stack.len()
}

fn main() {
    let input_string: String = String::from("I am going to reverse this string");
    let stack_size: usize = input_string.len();
    let mut stack: Vec<char> = new_stack(stack_size);
    let mut reversed: String = String::new();
   
    println!("Hello, string! {:?}", input_string);

    for i in input_string.chars() {
        push(&mut stack, i, stack_size);
    }

     println!("Hello, stack3! {:?}", stack);

    for i in 0..size(&stack) {
        reversed.push(pop(&mut stack).unwrap());
    }

    println!("reversi {}", reversed);

}
