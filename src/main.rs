// Using env to get file arguments
use std::env;

// this function returns the computer 
fn computer() -> String {
    // I created a Vector and then a mutable string and finally I
    // Concatenated the mutable string wit each element from the vec

    let mut stri = String::new();
    let a = vec![
        "\t.--.\n",
        "\t|__| .-------.\n",
        "\t|=.| |.-----.|\n",
        "\t|--| || WSP ||\n",
        "\t|  | |'-----'|\n",
        "\t|__|~')_____('\n"  
    ];
    for i in &a {
        stri += i;
    }

    // returning it 
    stri
}

//  Main Function
fn main() {
    let arguments: Vec<String> = env::args().collect();
    let mut text: String = String::new();
    let text_array = &arguments[2..];
    let space: String = " ".to_string();
    for i in text_array {
        
        let pol = i.to_owned()+" ";
        text += &pol.to_string();

    }


    let computerr = computer();

    print! ("\x1B[2J\x1B[1;1H");

    println!("\n{}",&computerr);

    println!("\n\t↑↑↑ Has Said '{}'",text); 
}
