use crate::char_replacer::{ApostropheReplacer, ClosingQuotesReplacer, EllipsesReplacer, FindAndReplace, OpeningQuotesReplacer};

pub(crate) fn utf8_ellipses_replace(source: String) -> String {
    let replacer = EllipsesReplacer::new("…");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}

pub(crate) fn utf8_opening_quotes_replace(source: String) -> String {
    let replacer = OpeningQuotesReplacer::new(" « ");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}

pub(crate) fn utf8_closing_quotes_replace(source: String) -> String {
    let replacer = ClosingQuotesReplacer::new(" » ");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}

pub(crate) fn utf8_apostrophe_replace(source: String) -> String {
    let replacer = ApostropheReplacer::new("’");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}
