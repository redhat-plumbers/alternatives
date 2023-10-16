use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
//use serde_yaml::Value;
use std::fs;
//use std::fs::File;
//use std::io::Write;

const BUILT_IN_DB_PATH: &str = "/tmp/alts.yaml";
const DROP_IN_DIR_PATH: &str = "/tmp/dropins";

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

impl Alternative {
    fn path (&self) -> &String {
        match self {
            Alternative::WithoutFollowers (alt) => return &alt.path,
            Alternative::WithFollowers (alt) => return &alt.path,
        }
    }
    fn link (&self) -> &String {
        match self {
            Alternative::WithoutFollowers (alt) => return &alt.link,
            Alternative::WithFollowers (alt) => return &alt.link,
        }
    }
    fn priority (&self) -> i32 {
        match self {
            Alternative::WithoutFollowers (alt) => return alt.priority,
            Alternative::WithFollowers (alt) => return alt.priority,
        }
    }

    fn new (link: String, path: String, priority: i32, followers: Vec<Follower>) -> Alternative {
        match  followers.len() {
            0 => return Alternative::WithoutFollowers(WithoutFollowers {link, path, priority}),
            _ => return Alternative::WithFollowers(WithFollowers {link, path, priority, followers}),
        }
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

impl AlternativeGroup {

    fn to_builtin (&self) -> AlternativeGroup {
        match self {
            AlternativeGroup::DropIn (group) => return AlternativeGroup::BuiltIn(BuiltIn {name : group.name.clone(), mode : "Auto".to_string(), manualPath : "".to_string(),  alternatives : group.alternatives.clone()}),
            AlternativeGroup::BuiltIn (group) => return AlternativeGroup::BuiltIn(BuiltIn {name : group.name.clone(), mode : group.mode.clone(), manualPath : group.manualPath.clone(), alternatives : group.alternatives.clone()}),
        }
    }

    fn to_dropin (&self) -> AlternativeGroup {
        match self {
            AlternativeGroup::BuiltIn (group) => return AlternativeGroup::DropIn(DropIn {name : group.name.clone(), alternatives : group.alternatives.clone()}),
            _ => return self.clone(),
        }
    }

    fn new_dropin (name: String, alternatives: Vec<Alternative>) -> AlternativeGroup {
        return AlternativeGroup::DropIn(DropIn {name, alternatives});
    }

    fn new_builtin (name: String, alternatives: Vec<Alternative>) -> AlternativeGroup {
        let aux = AlternativeGroup::new_dropin(name, alternatives);
        return aux.to_builtin();
    }

    fn append_alternative (&mut self, alt: Alternative) {
        match self {
            AlternativeGroup::DropIn (group) => group.alternatives.push(alt),
            AlternativeGroup::BuiltIn (group) => group.alternatives.push(alt),
        }
    }

    fn get_name (&self) -> String {
        let rv = match self {
            AlternativeGroup::DropIn (group) => &group.name,
            AlternativeGroup::BuiltIn (group) => &group.name,
        };
        return rv.clone();
    }

    fn get_alternatives (&self) -> Vec<Alternative> {
        let rv = match self {
            AlternativeGroup::DropIn (group) => &group.alternatives,
            AlternativeGroup::BuiltIn (group) => &group.alternatives,
        };
        return rv.clone();
    }

    fn alternative_group_cmp_name (&self, other: &AlternativeGroup) -> bool {
        return self.get_name() == other.get_name();
    }

}



// target/link_name -> same as ln command
fn create_symlinks (target: String, link_name: String) {
    //TODO - just debug function for now, no changes to the files on the disk
    println!("Making link: {:?} -> {:?}",link_name,target);

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

//Read the content of FS directory and return the read alternatives
fn read_dropins(path: String) -> Vec<AlternativeGroup>{
    let mut rv = Vec::new();
    for file in fs::read_dir(path).expect("Dir error") {
        let file_path = file.expect("TODO error").path();
        let content = fs::read_to_string(file_path).expect("TODO File error");
        let mut config: Vec<AlternativeGroup> = serde_yaml::from_str(content.as_str()).expect("Parse error");
        rv.append(&mut config);
    }

    return rv.iter().map(|x| x.to_dropin()).collect::<Vec<_>>();
}

/*
 * 1. filter the alternative groups by name
 * 2. if there are multiple alternative groups with a same name, merge them into one
 */
fn merge_dropins(name: String, groups: Vec<AlternativeGroup>) -> AlternativeGroup {
    let mut rv = AlternativeGroup::new_dropin(name.clone(),Vec::new());
    for a_g in groups {
        if a_g.get_name() == name {
            for alt in a_g.get_alternatives() {
                rv.append_alternative(alt);
            }
        }
    }
    return rv;
}

fn filter_buildins(name: String, groups: Vec<AlternativeGroup>) -> AlternativeGroup {
    let mut rv = AlternativeGroup::new_builtin(name.clone(),Vec::new());
    for a_g in groups {
        if a_g.get_name() == name {
            rv = a_g.clone();
        }
    }
    return rv;
}

/*
 * Returns the alternative with highest priority
 * If the list of alternatives is empty returns none
 * Undefined behavior when there are multiple alternatives with the smae prio
 */
fn highest_prio(alts: Vec<Alternative>) -> Option<Alternative> {
    if alts.len() == 0 {
        return None;
    }
    return None;

}

//reads a single DB file and returns a list of of structs for each NAME entry
fn read_config(path: String) -> Vec<AlternativeGroup> {
    let content = fs::read_to_string(path).expect("File error");
    let rv: Vec<AlternativeGroup> = serde_yaml::from_str(content.as_str()).expect("Parse error");
    return rv;
}

fn write_config(path: String, content: Vec<AlternativeGroup>) -> std::io::Result<()>{
    let yaml = serde_yaml::to_string(&content).expect("Parsing error");
    fs::write(path,yaml.as_bytes()).expect("Writing to file");
    Ok(())
}

struct ConfigFile {
    path: String,
    content: Vec<AlternativeGroup>,
    modified: bool
}


fn write_db (files: Vec<ConfigFile>) -> std::io::Result<()> {
    for file in files {
        if file.modified == true {
            write_config (file.path, file.content);
        }
    }
    Ok(())

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
    Auto { name: String },
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
            /*
             * 1. Parse the arguments
             * 2. Read the builtin DB file
             * 3. Append new alternative to the builtin DB file
             * 4. Write the modified DB file, if the the alternative group is set to manual then end else:
             * 5. Run the quivalent of the AUTO branch
             */
            println!("In the Install Branch!");
            println!("Name: {:?}, Path: {:?}", name, path);
            //Parse the arguments
            let followers = unwrap_followers(follower);
            let alt = Alternative::new(link.to_string(),path.to_string(),*priority,followers);
            //Read the bultin DB file
            let mut built_in_db = read_config(BUILT_IN_DB_PATH.to_string());
            //Find the correct alternative group

            let mut done = false;

            // check whether the alternative group with given name already exists and update it
            // Maybe TODO check for duplicit entries in the same alternative group
            for a_g in &mut built_in_db {
                if a_g.get_name() == name.to_string() {
                    println!("Alternative Group Found!: {:?}", a_g);
                    a_g.append_alternative (alt.clone());
                    done = true;
                    break;
                }
            }

            // this is a new alternative group = create it and append it
            if done == false {
                let new_alt_group = AlternativeGroup::new_builtin(name.to_string(), [alt.clone()].to_vec());
                built_in_db.push(new_alt_group);
                println!("Alternative Group Not Found!: {:?}", built_in_db);

            }
            write_config(BUILT_IN_DB_PATH.to_string(),built_in_db);
            // if the alternative group is se to auto - check the priorites and update the symlinks
        }
        Commands::Display {
            ref name,
        } => {
            let builtins = read_config(BUILT_IN_DB_PATH.to_string());
            println!("Built in:\n {:?}", filter_buildins(name.to_string(), builtins));
            let dropins = read_dropins(DROP_IN_DIR_PATH.to_string());
            println!("Drop ins:\n {:?}", merge_dropins(name.to_string(), dropins));
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
