use std::{
    env,
    process::{self, Command, Stdio},
};

// First step, we want to be able to run something like:
// cargo run -- execute /bin/sh
fn main() {
    // KISS
    // arg[0] -> all the command
    // arg[1] -> execute
    // arg[2] -> /bin/zsh
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        eprintln!("Two argv are required: execute <CMD>");
        process::exit(1);
    }

    //println!("{:?}", argv);

    match argv[1].as_str() {
        "execute" => {
            let mut child = Command::new(&argv[2])
                .args(&argv[3..])
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .unwrap();

            match child.wait() {
                Ok(_) => process::exit(0),
                Err(_) => process::exit(1),
            };
        }
        _ => {
            eprintln!("Only execute is supported for now");
            process::exit(1);
        }
    };
}
