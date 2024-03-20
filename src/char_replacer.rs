use duplicate::duplicate_item;


pub(crate) trait FindAndReplace {
    fn find_and_replace_occurrences(&self, text: String) -> String;
}

pub(crate) trait Replacer {
    fn to_replace(&self) -> &Vec<String>;
    fn replacement(&self) -> &str;
}

#[duplicate_item(SpecificReplacer; [EllipsesReplacer]; [OpeningQuotesReplacer]; [ClosingQuotesReplacer])]
impl Replacer for SpecificReplacer {
    fn to_replace(&self) -> &Vec<String> {
        &self.to_replace
    }

    fn replacement(&self) -> &str {
        &self.replacement
    }
}

impl<T> FindAndReplace for T where T: Replacer {
    fn find_and_replace_occurrences(&self, text: String) -> String {
        let mut result = text;
        for element_to_replace in self.to_replace() {
            result = result.replace(element_to_replace, self.replacement());
        }
        return result;
    }
}


/* ---------------------
   | ELLIPSES REPLACER |
   --------------------- */

pub(crate) struct EllipsesReplacer {
    to_replace: Vec<String>,
    replacement: String,
}

impl EllipsesReplacer {
    pub(crate) fn new(replacement_str: &str) -> EllipsesReplacer {
        let replacement = replacement_str.to_string();
        let mut to_replace = Vec::new();
        to_replace.push("....".to_string());
        to_replace.push("...".to_string());
        return EllipsesReplacer {
            to_replace,
            replacement,
        };
    }
}


/* -------------------
   | QUOTES REPLACER |
   ------------------- */

pub(crate) struct OpeningQuotesReplacer {
    to_replace: Vec<String>,
    replacement: String,
}

impl OpeningQuotesReplacer {
    pub(crate) fn new(replacement_str: &str) -> OpeningQuotesReplacer {
        let replacement = replacement_str.to_string();
        let mut to_replace = Vec::new();
        to_replace.push(" \"".to_string());
        to_replace.push(" ''".to_string());
        to_replace.push(" « ".to_string());
        to_replace.push(" &laquo; ".to_string());
        return OpeningQuotesReplacer {
            to_replace,
            replacement,
        };
    }
}

pub(crate) struct ClosingQuotesReplacer {
    to_replace: Vec<String>,
    replacement: String,
}

impl ClosingQuotesReplacer {
    pub(crate) fn new(replacement_str: &str) -> ClosingQuotesReplacer {
        let replacement = replacement_str.to_string();
        let mut to_replace = Vec::new();
        to_replace.push("\" ".to_string());
        to_replace.push("'' ".to_string());
        to_replace.push(" » ".to_string());
        to_replace.push(" &raquo; ".to_string());
        return ClosingQuotesReplacer {
            to_replace,
            replacement,
        };
    }
}
