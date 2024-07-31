use crate::parser::{Pair, Parse, ParseError, Rule};

// A color with RGBA channels
#[derive(Clone, Debug, PartialEq)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl Parse for Color {
    fn parse(pair: Pair) -> Result<Self, ParseError> {
        assert_eq!(pair.as_rule(), Rule::color_literal);

        let mut pairs = pair.into_inner();
        let s = &pairs.next().unwrap().as_str()[1..];

        let hex4bit = |pos| u8::from_str_radix(&s[pos..pos + 1], 16).map(|v| v as f32 / 15.0);
        let hex8bit = |pos| u8::from_str_radix(&s[pos..pos + 2], 16).map(|v| v as f32 / 255.0);

        match s.len() {
            // #RGB or #RGBA single digit hex
            3 | 4 => Ok(Color::new(
                hex4bit(0)?,
                hex4bit(1)?,
                hex4bit(2)?,
                if s.len() == 4 { hex4bit(3)? } else { 1.0 },
            )),
            // #RRGGBB or #RRGGBBAA double digit hex
            6 | 8 => Ok(Color::new(
                hex8bit(0)?,
                hex8bit(2)?,
                hex8bit(4)?,
                if s.len() == 8 { hex8bit(6)? } else { 1.0 },
            )),
            _ => Err(ParseError::ParseColorError(s.to_string())),
        }
    }
}
