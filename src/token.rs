#[derive(Debug, Clone)]
pub enum Token {
    DOCTYPE(DOCTYPE),
    StartTag(Tag),
    EndTag(Tag),
    Comment(String),
    Character(char),
    EndOfFile,
}

#[derive(Debug, Clone, Default)]
pub struct DOCTYPE {
    pub name: String,
    pub public_identifier: String,
    pub system_public_identifier: String,
    pub force_quirks: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Tag {
    pub tag_name: String,
    pub self_closing: bool,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Default)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
