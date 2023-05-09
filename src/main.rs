use clap::{Parser, Subcommand};

//type Follower = String;

#[derive(Debug, Clone)]
struct Follower {
    link: String,
    name: String,
    path: String,
}

// Converts the vector of string returned byt the parser to vector of followers.
// NOTE: Does not check, whether the length of the input is a multiple of 3, the parser should provide an input of the proper length, but some further check here might be appropriate
fn unwrap_followers(arg: &Vec<String>) -> Vec<Follower> {
    let mut rv = Vec::new();
    for chunk in arg.chunks(3) {
        let path = chunk[0].clone();
        let name = chunk[1].clone();
        let link = chunk[2].clone();
        rv.push(Follower { link, name, path });
    }
    rv
}

//Based on https://linux.die.net/man/8/alternatives
// TODO add aliases for some now obsolate commands
#[derive(Subcommand, Debug)]
enum Commands {
    /// TODO: Help text goes here
    #[command(long_flag = "install")]
    Install {
        /// TODO: Help text goes here
        link: String,
        /// TODO: Help text goes here
        name: String,
        /// TODO: Help text goes here
        path: String,
        /// TODO: Help text goes here
        priority: i32,
        /// TODO: Help text goes here
        #[arg(long, num_args=3, value_names=["LINK", "NAME", "PATH"])]
        follower: Vec<String>,
        /// TODO: Help text goes here
        initscript: Option<String>,
    },
    /// TODO: Help text goes here
    #[command(long_flag = "remove")]
    Remove { name: String, path: String },
    /// TODO: Help text goes here
    #[command(long_flag = "set")]
    Set { name: String, path: String },
    /// TODO: Help text goes here
    #[command(long_flag = "auto")]
    Auto { name: String, path: String },
    /// TODO: Help text goes here
    #[command(long_flag = "display")]
    Display { name: String },
    /// TODO: Help text goes here
    #[command(long_flag = "config")]
    Config { name: String },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /*
     * Common Options
     */
    /// Generate more comments about what alternatives is doing.
    #[arg[long]]
    verbose: bool,

    /// Don't generate any comments unless errors occur. This option is not yet implemented.
    #[arg[long]]
    quiet: bool,

    /// Don't actually do anything, just say what would be done. This option is not yet implemented.
    #[arg[long]]
    test: bool,

    /// Specifies the alternatives directory, when this is to be different from the default.
    #[arg(long, value_name = "alt_dir")]
    altdir: Option<String>,

    /// Specifies the administrative directory, when this is to be different from the default.
    #[arg(long, value_name = "admin_dir")]
    admindir: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Arguments::parse();

    match cli.command {
        Commands::Install {
            ref link,
            ref name,
            ref path,
            ref priority,
            ref follower,
            ref initscript,
        } => {
            println!("In the Install Brannch!");
            let followers = unwrap_followers(follower);
            println!("{:?}", followers);
            println!("Verbose: {:?}", cli.verbose);
        }
        Commands::Remove { ref name, ref path } => {
            println!("In the remove Branch!");
            println!("Name: {:?}, Path: {:?}", name, path)
        }
        _ => println!("We've got a problem"),
    };
}

// TODO: more tests!
#[cfg(test)]
pub mod cli {
    use super::*;
    use clap::CommandFactory;

    #[test]
    #[ignore = "FIXME"]
    fn arg_debug_assert() {
        Arguments::command().debug_assert();
    }

    #[test]
    #[ignore = "FIXME"]
    fn arg_test() {
        let testing_vec: Vec<_> = "--test".split(" ").collect();
        let cli = Arguments::command()
            .no_binary_name(true)
            .get_matches_from(testing_vec);
        assert_eq!(cli.get_flag("test"), true);
    }

    #[test]
    #[ignore = "FIXME"]
    fn arg_install() {
        let testing_vec: Vec<_> = "--install link name path prio".split(" ").collect();
        let cli = Arguments::command()
            .no_binary_name(true)
            .get_matches_from(testing_vec);
        assert!(cli.contains_id("install"));
    }
}
