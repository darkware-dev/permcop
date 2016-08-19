use std::collections::HashMap;
use std::path::Path;

use std::error::Error;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

use permcop::rules::FiletypeRule;

pub struct Ruleset
{
    type_rules: HashMap<String, FiletypeRule>
}


impl Clone for Ruleset {
    fn clone(&self) -> Self {
        let mut rules = Ruleset::new();

        for (filetype, rule) in &self.type_rules {
            rules.type_rules.insert(filetype.clone(), rule.clone());
        }

        return rules;
    }
}

impl Ruleset
{
    /// Construct a new empty [Ruleset]
    pub fn new() -> Ruleset {
        Ruleset {
            type_rules: HashMap::new()
        }
    }

    pub fn load(&self, rule_path: &Path) {

        // Open the path in read-only mode, returns `io::Result<File>`
        let file = match File::open(&rule_path) {
            Err(why) => panic!("couldn't open {}: {}", rule_path.display(), Error::description(&why)),
            Ok(file) => file,
        };
        let reader = BufReader::new(file);
        for line in reader.lines() {
            match line  {
                Ok(line) => self.load_line(line),
                Err(e) => println!("ERROR: {}", e),
            }
        }
    }

    fn load_line(&self, line: String) {
        let cleanline = String::from(line.trim());
        if cleanline.starts_with("#") {
            return;
        }

        let mut parts = line.splitn(2, ":");
        let ruledef = parts.next().unwrap();
        let optdef = parts.next().unwrap();

        let mut opts:HashMap<String,String> = HashMap::new();

        println!("RULE> {}", ruledef);
        for opt in optdef.split(",") {
            let mut optparts = opt.splitn(2, "=");
            let opt_name = optparts.next().unwrap();
            let opt_val = optparts.next().unwrap();
            println!("      -> [{}] {}", opt_name, opt_val);
            opts.insert(String::from(opt_name), String::from(opt_val));
        }

        match ruledef.find("[") {
            Some(start) => {
                //TODO: Ensure the last character is ]
                let ruletype = &ruledef[0 .. start];
                let target = &ruledef[start+1 .. &ruledef.len()-1];
                println!("+ Rule handler: {} @ {}", ruletype, target);

                self.add_rule(ruletype, target, opts);
            }
            None => {}
        }
    }

    fn add_rule(&self, rule_type: &str, rule_target: &str, opts: HashMap<String,String>) {
        println!("Adding rule: {} [{}]", rule_type, rule_target);
        println!("   Options:");
        for (opt_name, opt_value) in opts {
            println!("      {} = {}", opt_name, opt_value);
        }
    }
}
