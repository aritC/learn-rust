/*
    The main function is "always" the first code that runs in every executable Rust program.

    println! calls a "Rust macro". The ! at the end makes it a Rust Macro call. If we wanted to call a function we would not put the ! at the end and only do something like println(...).

    In console we type rustc main.rs to compile the code.
    Just like C/C++ compile and run are seperate steps. Like gcc rustc outputs a binary executable based on your OS. So our dir has 3 files:
                - main.exe -> binary executable
                - main.pdb -> Windows specific file which contains debugging information
                - main.rs -> source code

    Then in console we type .\main.exe to execute the code
*/

fn main() { 
    println!("Hello, World!");
}