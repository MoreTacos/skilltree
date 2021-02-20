pub enum Text<'a> {
    EmptyString,
    Settings,
    ChatWith(&'a str),
}

impl<'a> ToString for Text<'a> {
    fn to_string(&self) -> String {
        match self {
            Text::EmptyString => "".into(),
            Text::Settings => "Settings".into(),
            Text::ChatWith(name) => format!("Chat with {}", name),
        }
    }
}

impl Default for Text<'_> {
    fn default() -> Self {
        Text::EmptyString
    }
}
