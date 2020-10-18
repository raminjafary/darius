pub fn parse_args(args: Vec<String>) -> () {
    match args.len() {
        1 => {
            println!("No args has passed to the darius cli!");
        }

        2 => {
            let args_one = &args[1];
            if args_one == "-f" {
                println!(
                    "{} is used for specifying file name or directory!",
                    args_one
                );
            }
        }
        _ => {
            eprintln!("invalid commands or arguments");
            println!("{:#?}", &args[1..args.len()]);
        }
    }
}
