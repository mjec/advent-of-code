macro_rules! make_dispatch {
    ( $this:expr,  $( $name:expr => $general:path => $help:path => $description:expr ),+ ) => {
        pub fn dispatch(args: &mut Vec<String>) -> Result<(), i32> {
            if args.len() == 0 {
                println!("Further argument required to {}.", $this);
                print_list_of_methods();
                return Err(1); // == return_codes::INVALID_ARGUMENT
            }
            match args[0].to_ascii_lowercase().as_str() {
                $(
                    $name => $general(&mut args.split_off(1)), // $description (help counterpart: $help)
                )*
                a => {
                    println!("Unknown argument for {}: {}.", $this, a);
                    print_list_of_methods();
                    Err(1) // == return_codes::INVALID_ARGUMENT
                }
            }
        }

        pub fn dispatch_help(args: &mut Vec<String>) -> Result<(), i32> {
            if args.len() == 0 {
                print_list_of_methods();
                return Ok(());
            }

            match args[0].to_ascii_lowercase().as_str() {
                $(
                    $name => $help(&mut args.split_off(1)), // $description (general counterpart: $general)
                )*
                a => {
                    println!("Unknown argument for {}: {}.", $this, a);
                    print_list_of_methods();
                    Err(1) // == return_codes::INVALID_ARGUMENT
                }
            }
        }

        fn print_list_of_methods() -> () {
            println!("The following positional arguments of {} are supported:", $this);
            $(
                println!("{}\t{}", $name, $description); // $general / $help
            )*
        }
    };
}

macro_rules! make_dispatch_and_help {
    ( $this:expr, $( $name:expr => $general:path => $help:ident => $arguments:expr => $description:expr ),+ ) => {
        $(
            // help for $general
            fn $help (_: &mut Vec<String>) -> Result<(), i32> {
                println!("Usage: {} {} {}\n{}", $this, $name, $arguments, $description);
                Ok(())
            }
        )*

        make_dispatch!($this, $( $name => $general => $help => $description ),* );
    };
}
