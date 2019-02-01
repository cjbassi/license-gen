use chrono::prelude::*;
use chrono::Local;
use std::process::{self, Command};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    license: String,
    name: Option<String>,
}

fn main() {
    let args = Args::from_args();
    let year = Local::now().year();
    let name = args.name.unwrap_or_else(|| {
        String::from_utf8(
            Command::new("git")
                .args(&["config", "user.name"])
                .output()
                .expect("failed to query git config for name")
                .stdout,
        )
        .expect("failed to parse git config output")
        .trim()
        .to_owned()
    });
    if name == "" {
        eprintln!("no username configured with git config");
        process::exit(1);
    }
    let license_text = match args.license.as_ref() {
        "bsd" | "BSD" => include_str!("../templates/bsd-3.0.txt"),
        "mit" | "MIT" => include_str!("../templates/mit.txt"),
        "gpl" | "GPL" | "gpl-3" | "GPL-3" => include_str!("../templates/gpl-3.0.txt"),
        "agpl" | "AGPL" | "agpl-3" | "AGPL-3" => include_str!("../templates/agpl-3.0.txt"),
        "apache" | "Apache" => include_str!("../templates/apache-2.0.txt"),
        "CC-BY" | "CCBY" | "ccby" => include_str!("../templates/cc-by-4.0.txt"),
        "CC-BY-NC" | "CCBYNC" | "ccbync" => include_str!("../templates/cc-by-nc-4.0.txt"),
        "CC-BY-SA" | "CCBYSA" | "ccbysa" => include_str!("../templates/cc-by-sa-4.0.txt"),
        "CC-BY-NC-SA" | "CCBYNCSA" | "ccbyncsa" => include_str!("../templates/cc-by-nc-sa-4.0.txt"),
        "cc0" | "CC0" => include_str!("../templates/cc-zero-1.0.txt"),
        "lgpl" | "LGPL" | "lgpl-3" | "LGPL-3" => include_str!("../templates/lgpl-3.0.txt"),
        "mpl" | "MPL" => include_str!("../templates/mpl-2.0.txt"),
        "unlicense" | "Unlicense" | "UNLICENSE" => include_str!("../templates/unlicense.txt"),
        _ => {
            eprintln!("unknown license: {}", args.license);
            process::exit(1);
        }
    };
    print!("{}", license_text.replace("{YEAR}", &year.to_string()).replace("{AUTHOR}", &name));
}
