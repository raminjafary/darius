pub fn parse_args(args: Vec<String>) -> () {
    match args.len() {
        1 => {
            println!(
                "\x1b[38;5;206m{}\x1b[0m",
                "No args has passed to the darius cli!"
            );
        }

        2 => {
            let args_one = &args[1];
            if args_one == "-f" {
                println!(
                    "\x1b[38;5;51;87;123m{} is used for specifying file name or directory!\x1b[0m",
                    args_one
                );
            }
        }
        _ => {
            eprintln!("\x1b[31;1;4m{}\x1b[0m", "invalid commands or arguments");
            println!("{:#?}", &args[1..args.len()]);
        }
    }
}
