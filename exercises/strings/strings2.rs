// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let word = String::from("green"); // Try not changing this line :)

    // because String implements deref<target=str>, it can be dereferenced (*word) as a str
    // with deref coercion this allows for the syntax below (compiler auto derefs as needed)
    // it will do &(*word), where (*word) becomes type str
    // https://stackoverflow.com/questions/23975391/how-to-convert-a-string-into-a-static-str
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
