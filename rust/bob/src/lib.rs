pub fn reply(message: &str) -> &str {
    let trimmed: &str = message.trim();

    if trimmed.is_empty() {
        return "Fine. Be that way!";
    }

    let question = trimmed.ends_with("?");

    let yell = trimmed.chars().any(char::is_alphabetic)
        && trimmed.chars().all(|c| c.is_uppercase() || !c.is_alphabetic());

    match (question, yell) {
        (true, true) => "Calm down, I know what I'm doing!",
        (true, false) => "Sure.",
        (false, true) => "Whoa, chill out!",
        (false, false) => "Whatever."
    }
}
