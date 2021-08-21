use crate::token::{Attribute, Tag, Token, DOCTYPE};

#[derive(Debug)]
enum State {
    Data,
    RCDATA,
    RAWTEXT,
    ScriptData,
    PLAINTEXT,
    TagOpen,
    EndTagOpen,
    TagName,
    RCDATALessThanSign,
    RCDATAEndTagOpen,
    RCDATAEndTagName,
    RAWTEXTLessThanSign,
    RAWTEXTEndTagOpen,
    RAWTEXTEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    BeforeAttributeValue,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    DOCTYPE,
    BeforeDOCTYPEName,
    DOCTYPEName,
    AfterDOCTYPEName,
    AfterDOCTYPEPublicKeyword,
    BeforeDOCTYPEPublicIdentifier,
    DOCTYPEPublicIdentifierDoubleQuoted,
    DOCTYPEPublicIdentifierSingleQuoted,
    AfterDOCTYPEPublicIdentifier,
    BetweenDOCTYPEPublicAndSystemIdentifiers,
    AfterDOCTYPESystemKeyword,
    BeforeDOCTYPESystemIdentifier,
    DOCTYPESystemIdentifierDoubleQuoted,
    DOCTYPESystemIdentifierSingleQuoted,
    AfterDOCTYPESystemIdentifier,
    BogusDOCTYPE,
    CDATASection,
    CDATASectionBracket,
    CDATASectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}

pub struct Tokenizer {
    input: String,
    pos: usize,
    current_state: State,
    return_state: State,
}

impl Tokenizer {
    pub fn new(input: String) -> Self {
        let input = input.trim().to_string().replace('\n', "");
        Self {
            input,
            pos: 0,
            current_state: State::Data,
            return_state: State::Data,
        }
    }

    fn current_char(&self) -> char {
        self.input.chars().nth(self.pos).unwrap()
    }

    #[allow(non_snake_case)]
    fn switch_to(&mut self, State: State) {
        self.current_state = State;
        self.pos += 1;
    }

    #[allow(non_snake_case)]
    fn switch_to_without_consuming(&mut self, State: State) {
        self.current_state = State;
    }

    fn next_chars(&self, string: &str) -> bool {
        if self.input[self.pos + 1..].starts_with(string) {
            true
        } else {
            false
        }
    }

    fn advance_by(&mut self, n: usize) {
        self.pos += n
    }

    fn consume_if(&mut self, condition: bool) {
        if condition {
            self.pos += 1
        }
    }

    fn emit_token(&self, token: &Token) {
        match token {
            Token::Character(c) => {
                if c == &' ' {
                    return;
                }
            }
            _ => {}
        }
        println!("{:?}", token);
    }

    fn emit_current_char_as_token(&mut self) {
        self.emit_token(&Token::Character(self.current_char()));
        self.pos += 1;
    }

    pub fn run(&mut self) {
        let mut current_token = Token::EndOfFile;
        let mut attribute_id = 0;
        let mut temp_buffer = String::new();

        while self.pos <= self.input.len() - 1 {
            #[cfg(feature = "debug")]
            {
                println!("Current state: '{:?}'", self.current_state);
                println!(
                    "Pos: {}/{} | '{}'",
                    self.pos,
                    self.input.len(),
                    self.current_char()
                );
            }
            match self.current_state {
                State::Data => {
                    if self.current_char() == '&' {
                        self.return_state = State::Data;
                        self.switch_to_without_consuming(State::CharacterReference);
                    } else if self.current_char() == '<' {
                        self.switch_to(State::TagOpen);
                    } else {
                        self.emit_current_char_as_token();
                    }
                }
                State::RCDATA => {
                    if self.current_char() == '&' {
                        self.return_state = State::RCDATA;
                        self.switch_to_without_consuming(State::CharacterReference);
                    } else if self.current_char() == '<' {
                        self.switch_to(State::RCDATALessThanSign);
                    } else {
                        self.emit_current_char_as_token();
                    }
                }
                State::RAWTEXT => {
                    if self.current_char() == '<' {
                        self.switch_to(State::RAWTEXTLessThanSign);
                    } else {
                        self.emit_current_char_as_token();
                    }
                }
                State::ScriptData => {
                    if self.current_char() == '<' {
                        self.switch_to(State::ScriptDataLessThanSign);
                    } else {
                        self.emit_current_char_as_token();
                    }
                }
                State::PLAINTEXT => {
                    self.emit_current_char_as_token();
                }
                State::TagOpen => {
                    if self.current_char() == '!' {
                        self.switch_to_without_consuming(State::MarkupDeclarationOpen);
                    } else if self.current_char() == '/' {
                        self.switch_to(State::EndTagOpen);
                    } else if self.current_char().is_ascii_alphabetic() {
                        current_token = Token::StartTag(Tag {
                            ..Default::default()
                        });
                        self.switch_to_without_consuming(State::TagName);
                    } else if self.current_char() == '?' {
                        current_token = Token::Comment("".to_string());
                        self.switch_to_without_consuming(State::BogusComment);
                    } else {
                        self.emit_token(&Token::Character('>'));
                        self.switch_to_without_consuming(State::Data);
                    }
                }
                State::EndTagOpen => {
                    if self.current_char().is_ascii_alphabetic() {
                        current_token = Token::EndTag(Tag {
                            ..Default::default()
                        });
                        self.switch_to_without_consuming(State::TagName);
                    } else if self.current_char() == '>' {
                        self.switch_to(State::Data);
                    } else {
                        current_token = Token::Comment("".to_string());
                        self.switch_to_without_consuming(State::BogusComment);
                    }
                }
                State::TagName => {
                    if self.current_char().is_ascii_whitespace() {
                        self.switch_to(State::BeforeAttributeName);
                    } else if self.current_char() == '/' {
                        self.switch_to(State::SelfClosingStartTag);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else if self.current_char().is_ascii_uppercase() {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                t.tag_name.push(self.current_char().to_ascii_lowercase());
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                t.tag_name.push(self.current_char().to_ascii_lowercase());
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.pos += 1;
                    } else {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                t.tag_name.push(self.current_char().to_ascii_lowercase());
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                t.tag_name.push(self.current_char().to_ascii_lowercase());
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.pos += 1;
                    }
                }
                State::RCDATALessThanSign => {
                    if self.current_char() == '/' {
                        temp_buffer = String::new();
                        self.switch_to(State::RCDATAEndTagOpen);
                    } else {
                        self.emit_token(&Token::Character('<'));
                        self.switch_to_without_consuming(State::RCDATA);
                    }
                }
                State::RCDATAEndTagOpen => {
                    if self.current_char().is_ascii_alphabetic() {
                        current_token = Token::EndTag(Tag {
                            ..Default::default()
                        });
                    } else {
                        self.emit_token(&Token::Character('<'));
                        self.switch_to_without_consuming(State::RCDATA);
                    }
                }
                State::RCDATAEndTagName => {
                    if self.current_char().is_ascii_whitespace() {
                    } else if self.current_char() == '/' {
                    } else if self.current_char() == '>' {
                    } else if self.current_char().is_ascii_alphabetic() {
                    } else {
                    }
                }
                State::RAWTEXTLessThanSign => {
                    if self.current_char() == '/' {
                        temp_buffer = String::new();
                        self.switch_to(State::RAWTEXTEndTagOpen);
                    } else {
                        self.emit_token(&Token::Character('<'));
                        self.switch_to_without_consuming(State::RAWTEXT);
                    }
                }
                State::RAWTEXTEndTagOpen => {
                    if self.current_char().is_ascii_alphabetic() {
                        current_token = Token::EndTag(Tag {
                            ..Default::default()
                        });
                        self.switch_to_without_consuming(State::RAWTEXT);
                    } else {
                        self.emit_token(&Token::Character('<'));
                        self.emit_token(&Token::Character('/'));
                        self.switch_to_without_consuming(State::RAWTEXT);
                    }
                }
                State::RAWTEXTEndTagName => {
                    if self.current_char().is_ascii_whitespace() {
                    } else if self.current_char() == '/' {
                    } else if self.current_char() == '>' {
                    } else if self.current_char().is_ascii_alphabetic() {
                    } else {
                    }
                }
                State::ScriptDataLessThanSign => {
                    if self.current_char() == '/' {
                        temp_buffer = String::new();
                        self.switch_to(State::ScriptDataEndTagOpen);
                    } else if self.current_char() == '!' {
                        self.emit_token(&Token::Character('<'));
                        self.emit_token(&Token::Character('!'));
                        self.switch_to(State::ScriptData);
                    } else {
                        self.emit_token(&Token::Character('<'));
                        self.switch_to_without_consuming(State::ScriptData);
                    }
                }
                State::ScriptDataEndTagOpen => todo!(),
                State::ScriptDataEndTagName => todo!(),
                State::ScriptDataEscapeStart => todo!(),
                State::ScriptDataEscapeStartDash => todo!(),
                State::ScriptDataEscaped => todo!(),
                State::ScriptDataEscapedDash => todo!(),
                State::ScriptDataEscapedDashDash => todo!(),
                State::ScriptDataEscapedLessThanSign => todo!(),
                State::ScriptDataEscapedEndTagOpen => todo!(),
                State::ScriptDataEscapedEndTagName => todo!(),
                State::ScriptDataDoubleEscapeStart => todo!(),
                State::ScriptDataDoubleEscaped => todo!(),
                State::ScriptDataDoubleEscapedDash => todo!(),
                State::ScriptDataDoubleEscapedDashDash => todo!(),
                State::ScriptDataDoubleEscapedLessThanSign => todo!(),
                State::ScriptDataDoubleEscapeEnd => todo!(),
                State::BeforeAttributeName => {
                    self.consume_if(self.current_char().is_ascii_whitespace());
                    if self.current_char() == '/' && self.current_char() == '>' {
                        self.switch_to_without_consuming(State::AfterAttributeName);
                    } else if self.current_char() == '=' {
                        // Should rewrite this into a function...
                        // Start a new attribute in the current tag token.
                        // Set that attribute's name to the current input character,
                        // and its value to the empty string.
                        // Switch to the attribute name state.
                        // TODO: Keep track of index in attributes vec
                        // and reset it when a new token starts or sth like that
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                if t.attributes.is_empty() {
                                    attribute_id = 0;
                                } else {
                                    attribute_id += 1;
                                }
                                t.attributes.insert(
                                    0,
                                    Attribute {
                                        name: self.current_char().to_string(),
                                        ..Default::default()
                                    },
                                );
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                if t.attributes.is_empty() {
                                    attribute_id = 0;
                                } else {
                                    attribute_id += 1;
                                }
                                t.attributes.insert(
                                    0,
                                    Attribute {
                                        name: self.current_char().to_string(),
                                        ..Default::default()
                                    },
                                );
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.switch_to(State::AttributeName);
                    } else {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                if t.attributes.is_empty() {
                                    attribute_id = 0;
                                } else {
                                    attribute_id += 1;
                                }
                                t.attributes.insert(
                                    attribute_id,
                                    Attribute {
                                        ..Default::default()
                                    },
                                );
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                if t.attributes.is_empty() {
                                    attribute_id = 0;
                                } else {
                                    attribute_id += 1;
                                }
                                t.attributes.insert(
                                    attribute_id,
                                    Attribute {
                                        ..Default::default()
                                    },
                                );
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.switch_to_without_consuming(State::AttributeName);
                    }
                }
                State::AttributeName => {
                    if self.current_char().is_ascii_whitespace()
                        || self.current_char() == '/'
                        || self.current_char() == '>'
                    {
                        self.switch_to_without_consuming(State::AfterAttributeName);
                    } else if self.current_char() == '=' {
                        self.switch_to(State::BeforeAttributeValue);
                    } else {
                        // Combines ascii upper and else
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                t.attributes[attribute_id]
                                    .name
                                    .push(self.current_char().to_ascii_lowercase());
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                t.attributes[attribute_id]
                                    .name
                                    .push(self.current_char().to_ascii_lowercase());
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.pos += 1;
                    }
                }
                State::AfterAttributeName => {
                    self.consume_if(self.current_char().is_ascii_whitespace());
                    if self.current_char() == '/' {
                        self.switch_to(State::SelfClosingStartTag);
                    } else if self.current_char() == '=' {
                        self.switch_to(State::BeforeAttributeValue);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                if t.attributes.is_empty() {
                                    attribute_id = 0;
                                } else {
                                    attribute_id += 1;
                                }
                                t.attributes.insert(
                                    attribute_id,
                                    Attribute {
                                        ..Default::default()
                                    },
                                );
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                if t.attributes.is_empty() {
                                    attribute_id = 0;
                                } else {
                                    attribute_id += 1;
                                }
                                t.attributes.insert(
                                    attribute_id,
                                    Attribute {
                                        ..Default::default()
                                    },
                                );
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.switch_to_without_consuming(State::AttributeName);
                    }
                }
                State::BeforeAttributeValue => {
                    self.consume_if(self.current_char().is_ascii_whitespace());
                    if self.current_char() == '"' {
                        self.switch_to(State::AttributeValueDoubleQuoted);
                    } else if self.current_char() == '\'' {
                        self.switch_to(State::AttributeValueSingleQuoted);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else {
                        self.switch_to_without_consuming(State::AttributeValueUnquoted);
                    }
                }
                State::AttributeValueDoubleQuoted => {
                    if self.current_char() == '"' {
                        self.switch_to(State::AfterAttributeValueQuoted);
                    } else if self.current_char() == '&' {
                        self.return_state = State::AttributeValueDoubleQuoted;
                        self.switch_to(State::CharacterReference);
                    } else {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                t.attributes[attribute_id].value.push(self.current_char());
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                t.attributes[attribute_id].value.push(self.current_char());
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.pos += 1;
                    }
                }
                State::AttributeValueSingleQuoted => {
                    if self.current_char() == '\'' {
                        self.switch_to(State::AfterAttributeValueQuoted);
                    } else if self.current_char() == '&' {
                        self.return_state = State::AttributeValueDoubleQuoted;
                        self.switch_to(State::CharacterReference);
                    } else {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                t.attributes[attribute_id].value.push(self.current_char());
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                t.attributes[attribute_id].value.push(self.current_char());
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.pos += 1;
                    }
                }
                State::AttributeValueUnquoted => {
                    if self.current_char().is_ascii_whitespace() {
                        self.switch_to(State::BeforeAttributeName);
                    } else if self.current_char() == '&' {
                        self.return_state = State::AttributeValueUnquoted;
                        self.switch_to(State::CharacterReference);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                t.attributes[attribute_id].value.push(self.current_char());
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                t.attributes[attribute_id].value.push(self.current_char());
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.pos += 1;
                    }
                }
                State::AfterAttributeValueQuoted => {
                    if self.current_char().is_ascii_whitespace() {
                        self.switch_to(State::BeforeAttributeName);
                    } else if self.current_char() == '/' {
                        self.switch_to(State::SelfClosingStartTag);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else {
                        self.switch_to_without_consuming(State::BeforeAttributeName);
                    }
                }
                State::SelfClosingStartTag => {
                    if self.current_char() == '>' {
                        current_token = match current_token {
                            Token::StartTag(mut t) => {
                                t.self_closing = true;
                                Token::StartTag(t)
                            }
                            Token::EndTag(mut t) => {
                                t.self_closing = true;
                                Token::EndTag(t)
                            }
                            _ => panic!("Not a tag token!"),
                        };
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else {
                        self.switch_to_without_consuming(State::BeforeAttributeName);
                    }
                }
                State::BogusComment => {
                    if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c.push(self.current_char());
                                Token::Comment(c)
                            }
                            _ => panic!("Not a comment token!"),
                        };
                        self.pos += 1;
                    }
                }
                State::MarkupDeclarationOpen => {
                    if self.next_chars("--") {
                        self.advance_by(2);
                        current_token = Token::Comment("".to_string());
                        self.switch_to(State::CommentStart);
                    } else if self.next_chars("DOCTYPE") {
                        self.advance_by("DOCTYPE".len());
                        self.switch_to(State::DOCTYPE);
                    }
                }
                State::CommentStart => {
                    if self.current_char() == '-' {
                        self.switch_to(State::CommentStartDash)
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data)
                    } else {
                        self.switch_to_without_consuming(State::Comment)
                    }
                }
                State::CommentStartDash => {
                    if self.current_char() == '-' {
                        self.switch_to(State::CommentEnd);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    }
                }
                State::Comment => {
                    if self.current_char() == '<' {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c.push(self.current_char());
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                        self.switch_to(State::CommentLessThanSign);
                    } else if self.current_char() == '-' {
                        self.switch_to(State::CommentEndDash);
                    } else {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c.push(self.current_char());
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                        self.pos += 1;
                    }
                }
                State::CommentLessThanSign => {
                    if self.current_char() == '!' {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c.push(self.current_char());
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                        self.switch_to(State::CommentLessThanSignBang);
                    } else if self.current_char() == '<' {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c.push(self.current_char());
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                    } else {
                        self.switch_to_without_consuming(State::Comment);
                    }
                }
                State::CommentLessThanSignBang => {
                    if self.current_char() == '-' {
                        self.switch_to(State::CommentLessThanSignBangDash);
                    } else {
                        self.switch_to_without_consuming(State::Comment);
                    }
                }
                State::CommentLessThanSignBangDash => {
                    if self.current_char() == '-' {
                        self.switch_to(State::CommentLessThanSignBangDashDash);
                    } else {
                        self.switch_to_without_consuming(State::CommentEndDash);
                    }
                }
                State::CommentLessThanSignBangDashDash => {
                    self.switch_to_without_consuming(State::CommentEnd);
                }
                State::CommentEndDash => {
                    if self.current_char() == '-' {
                        self.switch_to(State::CommentEnd);
                    } else {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c.push('-');
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                        self.switch_to_without_consuming(State::Comment);
                    }
                }
                State::CommentEnd => {
                    if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else if self.current_char() == '!' {
                        self.switch_to(State::CommentEndBang);
                    } else if self.current_char() == '-' {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c.push('-');
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                    } else {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c += "--";
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                    }
                }
                State::CommentEndBang => {
                    if self.current_char() == '-' {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                c += "!--";
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                        self.switch_to(State::CommentEndDash);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else {
                        current_token = match current_token {
                            Token::Comment(mut c) => {
                                // Should this be "!--"?
                                c += "!--";
                                Token::Comment(c)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                        self.switch_to_without_consuming(State::Comment);
                    }
                }
                State::DOCTYPE => {
                    if self.current_char().is_ascii_whitespace() || self.current_char() == '>' {
                        self.switch_to(State::BeforeDOCTYPEName)
                    } else if self.current_char() == '>' {
                        self.switch_to(State::BeforeDOCTYPEName);
                    } else {
                        self.switch_to(State::BeforeDOCTYPEName);
                    }
                }
                State::BeforeDOCTYPEName => {
                    self.consume_if(self.current_char().is_whitespace());
                    if self.current_char().is_ascii_uppercase() {
                        current_token = Token::DOCTYPE(DOCTYPE {
                            name: self.current_char().to_ascii_lowercase().to_string(),
                            ..Default::default()
                        });
                        self.switch_to(State::DOCTYPEName);
                    } else {
                        current_token = Token::DOCTYPE(DOCTYPE {
                            name: self.current_char().to_string(),
                            ..Default::default()
                        });
                        self.switch_to(State::DOCTYPEName);
                    }
                }
                State::DOCTYPEName => {
                    if self.current_char().is_whitespace() {
                        self.switch_to(State::AfterDOCTYPEName);
                    } else if self.current_char() == '>' {
                        self.emit_token(&current_token);
                        self.switch_to(State::Data);
                    } else if self.current_char().is_ascii_uppercase() {
                    } else {
                        current_token = match current_token {
                            Token::DOCTYPE(mut d) => {
                                d.name.push(self.current_char());
                                Token::DOCTYPE(d)
                            }
                            _ => panic!("{:?}", current_token),
                        };
                        self.pos += 1;
                    }
                }
                State::AfterDOCTYPEName => {
                    self.consume_if(self.current_char().is_whitespace());
                    if self.current_char() == '>' {
                        self.switch_to(State::Data);
                    }
                }
                State::AfterDOCTYPEPublicKeyword => todo!(),
                State::BeforeDOCTYPEPublicIdentifier => todo!(),
                State::DOCTYPEPublicIdentifierDoubleQuoted => todo!(),
                State::DOCTYPEPublicIdentifierSingleQuoted => todo!(),
                State::AfterDOCTYPEPublicIdentifier => todo!(),
                State::BetweenDOCTYPEPublicAndSystemIdentifiers => todo!(),
                State::AfterDOCTYPESystemKeyword => todo!(),
                State::BeforeDOCTYPESystemIdentifier => todo!(),
                State::DOCTYPESystemIdentifierDoubleQuoted => todo!(),
                State::DOCTYPESystemIdentifierSingleQuoted => todo!(),
                State::AfterDOCTYPESystemIdentifier => todo!(),
                State::BogusDOCTYPE => todo!(),
                State::CDATASection => todo!(),
                State::CDATASectionBracket => todo!(),
                State::CDATASectionEnd => todo!(),
                State::CharacterReference => todo!(),
                State::NamedCharacterReference => todo!(),
                State::AmbiguousAmpersand => todo!(),
                State::NumericCharacterReference => todo!(),
                State::HexadecimalCharacterReferenceStart => todo!(),
                State::DecimalCharacterReferenceStart => todo!(),
                State::HexadecimalCharacterReference => todo!(),
                State::DecimalCharacterReference => todo!(),
                State::NumericCharacterReferenceEnd => todo!(),
            }
        }

        self.emit_token(&Token::EndOfFile);
    }
}
