use std::path::Path;

mod permcop;

use permcop::rules::Ruleset;

fn main()
{
    println!("Permcop: Starting!");

    let _ = permcop::process_directory(Path::new("/web/wordpress"), Ruleset::new());
}

