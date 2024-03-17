use crate::char_replacer::{EllipsesReplacer, Replacer};

pub mod char_replacer;
mod html_char_replacer;

fn main() {
    let replacer = EllipsesReplacer::new("&mldr;");
    println!("{} {}", replacer.to_replace()[0], replacer.replacement());


}
