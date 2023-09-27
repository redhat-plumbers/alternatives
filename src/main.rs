use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use serde_yaml::Value;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Follower {
    link: String,
    name: String,
    path: String,
}

// Converts the vector of string returned byt the parser to vector of followers.
// NOTE: Does not check, whether the length of the input is a multiple of 3, the parser should provide an input of the proper length, but some further check here might be appropriate
fn unwrap_followers(arg: &[String]) -> Vec<Follower> {
    let mut rv = Vec::new();

    for chunk in arg.chunks(3) {
        rv.push(Follower {
            link: chunk[0].clone(),
            name: chunk[1].clone(),
            path: chunk[2].clone(),
        });
    }
    rv
}

// The following two structs form the content of the the DB files.
// FIXME Rename these two structs to something more descriptive
// This structure stores a singel alternative
/*#[derive(Serialize,Deserialize,Debug,Clone)]
struct Alternative {
    link:  String,
    path:  String,
    priority:  i32,
    followers: Option<Vec<Follower>>
}*/


#[derive(Serialize,Deserialize,Debug,Clone)]
struct WithFollowers {link: String, path: String, priority: i32, followers: Vec<Follower>}
#[derive(Serialize,Deserialize,Debug,Clone)]
struct WithoutFollowers {link: String, path: String, priority: i32}

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(untagged)]
enum Alternative {
    WithFollowers(WithFollowers),
    WithoutFollowers(WithoutFollowers),
}

fn create_alternative (link: String, path: String, priority: i32, followers: Vec<Follower>) -> Alternative{
    match  followers.len() {
        0 => return Alternative::WithoutFollowers(WithoutFollowers {link, path, priority}),
        _ => return Alternative::WithFollowers(WithFollowers {link, path, priority, followers}),
    }
}


#[derive(Serialize,Deserialize,Debug,Clone)]
struct BuiltIn {name: String, mode: String, manualPath: String, alternatives: Vec<Alternative>}
#[derive(Serialize,Deserialize,Debug,Clone)]
struct DropIn {name: String, alternatives: Vec<Alternative>}

//This structure contains the header + the vector of alternatives
#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(untagged)]
enum AlternativeGroup {
    BuiltIn (BuiltIn),
    DropIn  (DropIn),
}


fn create_dropin (name: String, alternatives: Vec<Alternative>) -> AlternativeGroup {
    return AlternativeGroup::DropIn(DropIn {name, alternatives});
}

fn group_to_builtin (drop_in: AlternativeGroup) -> AlternativeGroup {
    match drop_in {
        AlternativeGroup::DropIn (group) => return AlternativeGroup::BuiltIn(BuiltIn {name : group.name, mode : "Auto".to_string(), manualPath : "".to_string(),  alternatives : group.alternatives}),
        AlternativeGroup::BuiltIn (group) => return AlternativeGroup::BuiltIn(BuiltIn {name : group.name, mode : group.mode, manualPath : group.manualPath, alternatives : group.alternatives}),
    }
}

/*
 *
# naming based on: https://linux.die.net/man/8/alternatives# generic name for the group/maste symlink
- name: /usr/bin/editor
  mode: manual #manual or auto
# override info for the manual mode
  manual_path: /usr/bin/emacs
  group:
   - path: /usr/bin/vim
     link: sl_to_vim
     priority: 10
     follower:
       - link: foo1
         name: foo1
         path: foo1
       - link: foo2
         name: foo2
         path: foo2
   - path: /usr/bin/emacs
     link: sl_to_emacs
     priority: 9- name: version_control
  mode: auto
  manual_path:
  group:
  - path: /usr/bin/path
    link: sl_to_git
    priority: 100
  - path: /usr/bin/svn
    link: sl_to_subversion
    priority: 10
 *
 *
 */

//reads a single DB file and returns a list of of structs for each NAME entry
fn read_config(path: String) {
    let content = Value::String(fs::read_to_string(path).expect("File error"));
    let rv: AlternativeGroup = serde_yaml::from_value(content).unwrap();
}

fn write_config(path: String, content: &[&AlternativeGroup]) -> std::io::Result<()>{
    let yaml = serde_yaml::to_string(content).expect("Parsing error");
    fs::write(path,yaml.as_bytes()).expect("Writing to file");
    Ok(())
}

// add an alternative to the ones read from the db
fn add_alternative(){
}

//Based on https://linux.die.net/man/8/alternatives
// TODO add aliases for some now obsolete commands
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
        #[arg(long, num_args=3, alias="slave", value_names=["LINK", "NAME", "PATH"])]
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
            println!("In the Install Branch!");
            println!("Name: {:?}, Path: {:?}", name, path);
            let followers = unwrap_followers(follower);
            let alt = create_alternative(link.to_string(),path.to_string(),*priority,followers);
            let alts = group_to_builtin (create_dropin (name.to_string(), [alt.clone()].to_vec()));

            let res = serde_yaml::to_string(&[&alts]);
            write_config("/tmp/foo".to_string(), &[&alts]);
            println!("{:?}", alt);
            println!("{:?}", res);
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
