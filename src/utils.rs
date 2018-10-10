use std::iter::FromIterator;

pub fn replace_template(content: &mut String, from: &str, to: &str) {
    loop {
        match content.find(from) {
            Some(idx) => content.replace_range(idx..idx + from.len(), to),
            _ => break,
        }
    }
}

pub fn transform_to_camel_case(s: &str) -> String {
    let words = s.split("_");
    let mut res = String::with_capacity(s.len());
    for word in words {
        let word_chars = word.char_indices().map(|(idx, ch)| {
            if idx == 0 {
                ch.to_uppercase().collect::<Vec<_>>()[0]
            } else {
                ch
            }
        });
        res = res + &String::from_iter(word_chars);
    }
    res
}

pub fn replace_underline_to_dash(prefix: &str, s: &str) -> String {
    let res = String::with_capacity(s.len() + prefix.len());
    res + prefix + &s.replace("_", "-")
}
