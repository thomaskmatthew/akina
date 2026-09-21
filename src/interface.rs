use std::io::Write;

pub fn interface() {



    loop {
        println!("choose options");
        println!("1: view current party");
        println!("2: view Box");
        println!("3: view pokedex");
        println!("4: Exit");

        let choice = prompt("> ");

        match choice.as_str() {
            "1" => {
                println!("> view current party");
            }

            "2" => {
                println!("> view current party");
            }

            "3" => {
                println!("> view current party");
            }

            "4" => {
                break;
            }

            _ => println!("Invalid choice"),
        }
    }


}

fn prompt(text: &str) -> String {
    print!("{}", text);

    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();

    response.trim().to_string()
}