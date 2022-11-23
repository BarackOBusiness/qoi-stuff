fn main() {
    println!("Hello, world!");
        
    println!("{}", whats(9+10));
}

fn whats(expression: i64) -> i64 {
    let return_value = match expression {
        19 => 21,
        21 => -1, // Ion no
        _  => expression
    };
    
    return_value
}
