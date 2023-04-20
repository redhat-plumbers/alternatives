use std::fmt::Debug;
use std::fmt::Formatter;

struct LinkSet<'a> {
    title: &'a str,    /* print */
    facility: &'a str, /* /usr/bin/lpr */
    target: &'a str,   /* /usr/bin/lpr.LPRng */
}

struct Alternative<'b> {
    priority: i32,
    leader: &'b LinkSet<'b>,
    followers: Vec<LinkSet<'b>>,
    initscript: &'b str,
    num_followers: i32,
    family: &'b str,
}

struct AlternativeSet<'c> {
    mode: &'c str,
    alts: Vec<Alternative<'c>>,
    num_alts: i32,
    best: i32,
    current: i32,
    current_link: &'c str,
}

fn main() {
    let link = LinkSet {
        title: "vim",
        facility: "/usr/bin/vim",
        target: "/usr/bin/vim.tiny",
    };

    let alt = Alternative {
        priority: 10,
        leader: &link,
        followers: vec![],
        initscript: "init.d/vim",
        num_followers: 1,
        family: "vim",
    };

    let altset = AlternativeSet {
        mode: "auto",
        alts: vec![alt],
        num_alts: 1,
        best: 10,
        current: 10,
        current_link: "/usr/bin/vim.tiny",
    };

    impl Debug for LinkSet<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "title: {}, facility: {}, target: {}",
                self.title, self.facility, self.target
            )
        }
    }

    impl Debug for Alternative<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "priority: {}, leader: {:?}, followers: {:?}, initscript: {}, num_followers: {}, family: {}", self.priority, self.leader, self.followers, self.initscript, self.num_followers, self.family)
        }
    }

    impl Debug for AlternativeSet<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "mode: {}, alts: {:?}, num_alts: {}, best: {}, current: {}, current_link: {}",
                self.mode, self.alts, self.num_alts, self.best, self.current, self.current_link
            )
        }
    }

    println!("AlternativeSet: {:#?}", altset);

    println!("Hello, world!");
}
