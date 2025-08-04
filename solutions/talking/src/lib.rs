pub fn talking(text: &str) -> &'static str {
    let t = text.trim();

    if t.is_empty() {
        return "Just say something!";
    }

    let is_question = t.ends_with('?');

    let letters = t.chars().filter(|c| c.is_alphabetic());
    let is_yelling = {
        let mut iter = letters.peekable();
        iter.peek().is_some() && iter.all(|c| c.is_uppercase())
    };

    match (is_yelling, is_question) {
        (true,  true)  => "Quiet, I am thinking!",
        (true,  false) => "There is no need to yell, calm down!",
        (false, true)  => "Sure.",
        _              => "Interesting",
    }
}