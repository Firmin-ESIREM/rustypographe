trait FindAndReplace {
    fn find_occurrences(&self);
    fn replace_occurences(&self);
}

pub trait Replacer {
    fn to_replace(&self) -> Vec<String>;
    fn replacement(&self) -> &str;
    fn occurences(&self) -> &Vec<Vec<u8>>;
}

pub struct EllipsesReplacer {
    occurences: Vec<Vec<u8>>,
    replacement: String
}

impl EllipsesReplacer {
    pub fn new(replacement_str: &str) -> EllipsesReplacer {
        let mut occurences = Vec::new();
        let replacement = replacement_str.to_string();
        return EllipsesReplacer {
            occurences,
            replacement
        };
    }
}

impl Replacer for EllipsesReplacer {
    fn to_replace(&self) -> Vec<String> {
        let mut to_replace = Vec::new();
        to_replace.push("...".to_string());
        to_replace.push("....".to_string());
        return to_replace;
    }

    fn replacement(&self) -> &str {
        &self.replacement
    }

    fn occurences(&self) -> &Vec<Vec<u8>> {
        &self.occurences
    }
}

impl<T> FindAndReplace for T where T: Replacer {
    fn find_occurrences(&self) {
        todo!()
    }

    fn replace_occurences(&self) {
        todo!()
    }
}
