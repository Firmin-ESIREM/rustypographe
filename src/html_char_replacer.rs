use crate::char_replacer::{ApostropheReplacer, ClosingQuotesReplacer, EllipsesReplacer, FindAndReplace, NbspPunctuationMarkReplacer, OpeningQuotesReplacer};

pub(crate) fn html_ellipses_replace(source: String) -> String {
    let replacer = EllipsesReplacer::new("&mldr;");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}

pub(crate) fn html_opening_quotes_replace(source: String) -> String {
    let replacer = OpeningQuotesReplacer::new(" &laquo;&nbsp;");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}

pub(crate) fn html_closing_quotes_replace(source: String) -> String {
    let replacer = ClosingQuotesReplacer::new("&nbsp;&raquo; ");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}

pub(crate) fn html_apostrophe_replace(source: String) -> String {
    let replacer = ApostropheReplacer::new("&CloseCurlyQuote;");
    let result = replacer.find_and_replace_occurrences(source);
    return result;
}
