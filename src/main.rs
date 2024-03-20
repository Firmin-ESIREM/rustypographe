use crate::html_char_replacer::{html_apostrophe_replace, html_closing_quotes_replace, html_ellipses_replace, html_opening_quotes_replace};
use crate::utf8_char_replacer::{utf8_apostrophe_replace, utf8_closing_quotes_replace, utf8_ellipses_replace, utf8_opening_quotes_replace};

mod char_replacer;
mod html_char_replacer;
mod utf8_char_replacer;

fn main() {
    const BASE_TEXT: &str = "La nuit était calme, éclairée seulement par la lueur de la lune, lorsque soudain, un bruissement dans les buissons attira mon attention... Intrigué, je m'approchai lentement, le cœur battant, me demandant ce qui pouvait bien se cacher dans les \"ténèbres\" nocturnes.";
    println!("Base text: {}", BASE_TEXT);
    let mut res = utf8_ellipses_replace(BASE_TEXT.to_string());
    res = utf8_opening_quotes_replace(res);
    res = utf8_closing_quotes_replace(res);
    res = utf8_apostrophe_replace(res);
    println!("Corrected, UTF-8: {}", res);
    let mut res = html_ellipses_replace(BASE_TEXT.to_string());
    res = html_opening_quotes_replace(res);
    res = html_closing_quotes_replace(res);
    res = html_apostrophe_replace(res);
    println!("Corrected, HTML: {}", res);
}
