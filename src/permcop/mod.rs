use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

pub mod rules;

use self::rules::Ruleset;

pub fn process_directory(dir: &Path, rules: Ruleset) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        let mut rule_file_builder = PathBuf::from(dir);
        rule_file_builder.push(".permcop");
        rule_file_builder.set_extension("rules");
        let rule_file = rule_file_builder.as_path();

        if rule_file.exists() {
            rules.load(rule_file);
        }

        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(examine_directory(&entry.path()));
                let _ = try!(process_directory(&entry.path(), rules.clone()));
            } else {
                try!(examine_file(&entry.path()));
            }
        }
    }

    Ok(())
}

fn examine_directory(entry: &Path) -> io::Result<()>
{
    //println!("Examining Directory: {:?}", entry);

    Ok(())
}

fn examine_file(entry: &Path) -> io::Result<()>
{
    //println!("Examining: {:?}", entry);

    Ok(())
}
