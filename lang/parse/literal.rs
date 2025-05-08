// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, syntax::*};

impl Parse for Color {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::color_literal);
        let values = &pair;
        let values = &values.as_str()[1..];

        let hex4bit = |pos| u8::from_str_radix(&values[pos..pos + 1], 16).map(|v| v as f32 / 15.0);
        let hex8bit = |pos| u8::from_str_radix(&values[pos..pos + 2], 16).map(|v| v as f32 / 255.0);

        match values.len() {
            // #RGB or #RGBA single digit hex
            3 | 4 => Ok(Color::new(
                hex4bit(0)?,
                hex4bit(1)?,
                hex4bit(2)?,
                if values.len() == 4 { hex4bit(3)? } else { 1.0 },
            )),
            // #RRGGBB or #RRGGBBAA double digit hex
            6 | 8 => Ok(Color::new(
                hex8bit(0)?,
                hex8bit(2)?,
                hex8bit(4)?,
                if values.len() == 8 { hex8bit(6)? } else { 1.0 },
            )),
            _ => Err(ParseError::ParseColorError(values.to_string())),
        }
    }
}

impl Parse for Literal {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::literal);

        let inner = pair.inner().next().expect(INTERNAL_PARSE_ERROR);

        let s = match inner.as_rule() {
            Rule::number_literal => Literal::Number(NumberLiteral::parse(inner)?),
            Rule::integer_literal => {
                Literal::Integer(Refer::new(inner.as_str().parse::<i64>()?, pair.into()))
            }
            Rule::bool_literal => match inner.as_str() {
                "true" => Literal::Bool(Refer::new(true, pair.into())),
                "false" => Literal::Bool(Refer::new(false, pair.into())),
                _ => unreachable!(),
            },
            Rule::color_literal => Literal::Color(Refer::new(Color::parse(inner)?, pair.into())),
            _ => unreachable!(),
        };

        Ok(s)
    }
}

impl std::str::FromStr for Literal {
    type Err = ParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Parser::parse_rule::<Self>(Rule::literal, s, 0)
    }
}

impl Parse for NumberLiteral {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::number_literal);

        let mut inner = pair.inner();
        let number_token = inner.next().expect("Expected number token");

        assert!(
            number_token.as_rule() == Rule::number
                || number_token.as_rule() == Rule::integer_literal
        );

        let value = number_token.as_str().parse::<f64>()?;

        let mut unit = Unit::None;

        if let Some(unit_token) = inner.next() {
            unit = Unit::parse(unit_token)?;
        }
        Ok(NumberLiteral(value, unit, pair.clone().into()))
    }
}

impl Parse for Unit {
    fn parse(pair: Pair) -> ParseResult<Self> {
        use std::str::FromStr;
        match Unit::from_str(pair.as_str()) {
            Ok(unit) => Ok(unit),
            Err(_) => Err(ParseError::UnknownUnit(pair.as_str().to_string())),
        }
    }
}

impl std::str::FromStr for Unit {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Scalars
            "" => Ok(Self::None),
            "%" => Ok(Self::Percent),

            // Lengths
            "m" => Ok(Self::Meter),
            "cm" => Ok(Self::Centimeter),
            "mm" => Ok(Self::Millimeter),
            "µm" => Ok(Self::Micrometer),
            "in" => Ok(Self::Inch),
            "\"" => Ok(Self::Inch),
            "ft" => Ok(Self::Foot),
            "\'" => Ok(Self::Foot),
            "yd" => Ok(Self::Foot),

            // Angles
            "deg" => Ok(Self::Deg),
            "°" => Ok(Self::DegS),
            "grad" => Ok(Self::Grad),
            "turn" => Ok(Self::Turn),
            "rad" => Ok(Self::Rad),

            // Weights
            "g" => Ok(Self::Gram),
            "kg" => Ok(Self::Kilogram),
            "lb" => Ok(Self::Pound),
            "oz" => Ok(Self::Ounce),

            // Areas
            "m²" | "m2" => Ok(Self::Meter2),
            "cm²" | "cm2" => Ok(Self::Centimeter2),
            "mm²" | "mm2" => Ok(Self::Millimeter2),
            "µm²" | "µm2" => Ok(Self::Micrometer2),
            "in²" | "in2" => Ok(Self::Inch2),
            "ft²" | "ft2" => Ok(Self::Foot2),
            "yd²" | "yd2" => Ok(Self::Yard2),

            // Volumes
            "m³" | "m3" => Ok(Self::Meter3),
            "cm³" | "cm3" => Ok(Self::Centimeter3),
            "mm³" | "mm3" => Ok(Self::Millimeter3),
            "µm³" | "µm3" => Ok(Self::Micrometer3),
            "in³" | "in3" => Ok(Self::Inch3),
            "ft³" | "ft3" => Ok(Self::Foot3),
            "yd³" | "yd3" => Ok(Self::Yard3),
            "ml" => Ok(Self::Milliliter),
            "cl" => Ok(Self::Centiliter),
            "l" => Ok(Self::Liter),
            "µl" => Ok(Self::Microliter),

            // Unknown
            _ => Err(ParseError::UnknownUnit(s.to_string())),
        }
    }
}
