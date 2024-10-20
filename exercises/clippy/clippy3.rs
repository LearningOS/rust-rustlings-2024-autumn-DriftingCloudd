// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // Instead of calling unwrap on None, we simply don't call unwrap.
    if my_option.is_none() {
        // Do nothing
    }

    // Correcting missing commas in the array
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // Using the clear method to empty the vector
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();  // Empties the vector
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Using std::mem::swap to swap values
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
