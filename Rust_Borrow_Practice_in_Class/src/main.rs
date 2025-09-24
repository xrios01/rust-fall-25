// Problem 1
fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    let mut out = String::with_capacity(s1.len() + s2.len());
    out.push_str(s1);
    out.push_str(s2);
    out
}

// Problem2: Clone and Modify
fn clone_and_modify(s: &String) -> String {
    // Your code here
    let mut cloned = s.clone();
    cloned.push_str("World!");
    cloned
}

// Problem 3: Mutable Reference Sum
#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    *total = 0;                // reset total
    for x in low..=high {      // loop from low to high
        *total += x;           // add each number total
    }
}



fn main() {
    // Problem 1
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"

    // Problem 2 
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"

    // Problem 3
    // create necessary variables and test your function for low 0 high 100
    // total should be 5050
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Sum from 0 to 100 is {}", total); // total should be 5050
}
