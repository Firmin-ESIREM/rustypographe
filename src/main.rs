use crate::html_char_replacer::{html_closing_quotes_replace, html_ellipses_replace, html_opening_quotes_replace};
use crate::utf8_char_replacer::{utf8_closing_quotes_replace, utf8_ellipses_replace, utf8_opening_quotes_replace};

mod char_replacer;
mod html_char_replacer;
mod utf8_char_replacer;

fn main() {
    let res = html_ellipses_replace("Je ne sais pas quoi dire... Peut-être le sais-tu ?....".to_string());
    println!("{}", res);
    let mut res = html_opening_quotes_replace("C'était une sorte de \"pâte\" gluante un peu bizarre.".to_string());
    res = html_closing_quotes_replace(res);
    println!("{}", res);

    let res = utf8_ellipses_replace("Je ne sais pas quoi dire... Peut-être le sais-tu ?....".to_string());
    println!("{}", res);
    let mut res = utf8_opening_quotes_replace("C'était une sorte de \"pâte\" gluante un peu bizarre.".to_string());
    res = utf8_closing_quotes_replace(res);
    println!("{}", res);
}
