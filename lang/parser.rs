// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD Code Parser

#![allow(missing_docs)]

/// include grammar from file
#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
pub struct Parser;

use crate::{
    errors::*,
    src_ref::{SrcRef, SrcReferrer},
};

#[derive(Debug, Clone)]
pub struct Pair<'i>(pest::iterators::Pair<'i, Rule>, u64);

impl<'i> Pair<'i> {
    pub fn new(pest_pair: pest::iterators::Pair<'i, Rule>, source_hash: u64) -> Self {
        Self(pest_pair, source_hash)
    }

    pub fn source_hash(&self) -> u64 {
        self.1
    }

    pub fn set_source_hash(&mut self, hash: u64) {
        self.1 = hash
    }

    pub fn pest_pair(&self) -> &pest::iterators::Pair<'i, Rule> {
        &self.0
    }

    pub fn inner(&'i self) -> impl Iterator<Item = Self> {
        self.0.clone().into_inner().map(|p| Self(p, self.1))
    }
}

impl<'i> SrcReferrer for Pair<'i> {
    fn src_ref(&self) -> SrcRef {
        let pair = &self.0;
        let (line, col) = pair.line_col();
        SrcRef::new(
            pair.as_span().start()..pair.as_span().end(),
            line as u32,
            col as u32,
            self.1,
        )
    }
}

impl<'i> std::ops::Deref for Pair<'i> {
    type Target = pest::iterators::Pair<'i, Rule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Pairs<'i> = pest::iterators::Pairs<'i, Rule>;

pub trait Parse: Sized {
    fn parse(pair: Pair) -> ParseResult<Self>;
}

impl Parser {
    /// Helper function to parse a vector of pairs into a vector of T
    ///
    /// # Arguments
    ///
    /// - `pairs`: The pairs to parse
    /// - `f`: The function to parse the pair into `T`
    ///
    /// Returns a vector of `T`
    pub fn vec<'a, T>(pair: Pair<'a>, f: impl Fn(Pair<'a>) -> ParseResult<T>) -> ParseResult<Vec<T>>
    where
        T: Clone,
    {
        pair.0.into_inner().map(|p| f(Pair(p, pair.1))).collect()
    }

    /// Parse a rule for type `T`
    pub fn parse_rule<T>(rule: Rule, input: &str, src_hash: u64) -> anyhow::Result<T>
    where
        T: Parse + Clone,
    {
        use pest::Parser as _;

        if let Some(pair) = Parser::parse(rule, input.trim())?.next() {
            Ok(T::parse(Pair(pair, src_hash))?)
        } else {
            Err(anyhow::Error::msg("could not parse"))
        }
    }

    pub fn ensure_rule(pair: &Pair, expected: Rule) {
        let rule = pair.as_rule();
        assert_eq!(rule, expected, "Unexpected rule: {rule:?}");
    }

    /// Find an inner pair by rule
    pub fn find<T: Parse>(pair: &Pair, rule: Rule) -> Option<T> {
        match pair
            .inner()
            .find(|pair| pair.as_rule() == rule)
            .map(T::parse)
        {
            Some(Err(_)) | None => None,
            Some(Ok(x)) => Some(x),
        }
    }
}
