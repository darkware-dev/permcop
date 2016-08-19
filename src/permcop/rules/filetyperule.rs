pub struct FiletypeRule
{
    target: String
}

impl Clone for FiletypeRule
{
    fn clone(&self) -> Self {
        let rule = FiletypeRule::new(self.target.as_str());

        return rule;
    }
}

impl FiletypeRule {
    pub fn new(target:&str) -> FiletypeRule {
        let rule = FiletypeRule {
            target: String::from(target)
        };

        println!("Created TYPE Rule: {}", target);

        return rule;
    }
}
