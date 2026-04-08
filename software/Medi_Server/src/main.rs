use std::io;

fn main() {
    // able to turn off the program.
    let mut keep_running = true;
    while (keep_running) {
        println!("# - Input Comand:");
        let mut comand = String::new();
        let input_result = io::stdin().read_line(&mut comand);

        match input_result {
            Ok(_t) => println!("# - loaded input: Sucsess"),
            Err(_error) => panic!("# - failed loading input"),
        }

        let comand = comand.trim();
        match comand {
            "Test" => println!("# - sucsess"),
            "exit" => {
                println!("# - Qutting Prograimg");
                keep_running = false
            }
            _ => println!("# - Invalid comand: {comand}\n\n"),
        }
    }
}
