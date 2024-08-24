fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
    println!("I'm a Rustacean");

    let x = 51 + /* 90 + */ 15;
    println!("Is `x` 10 or 100? x = {}", x);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

             
}