/*
    Copyright © 2019 Alastair Feille

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

extern crate regex;
use regex::Regex;

#[derive(Debug)]
pub struct Pattern
{
    pub name:  String,
    pub regex: Regex
}

pub fn load_patterns() -> Vec<Pattern>
{
    let pattern_definitions = [// keywords
                               ("auto", r"auto"),
                               ("bool", r"_Bool"),
                               ("break", r"break"),
                               ("case", r"case"),
                               ("char", r"char"),
                               ("complex", r"_Complex"),
                               ("const", r"const"),
                               ("continue", r"continue"),
                               ("default", r"default"),
                               ("do", r"do"),
                               ("double", r"double"),
                               ("else", r"else"),
                               ("enum", r"enum"),
                               ("extern", r"extern"),
                               ("float", r"float"),
                               ("for", r"for"),
                               ("goto", r"goto"),
                               ("if", r"if"),
                               ("imaginary", r"_Imaginary"),
                               ("inline", r"inline"),
                               ("int", r"int"),
                               ("long", r"long"),
                               ("register", r"register"),
                               ("restrict", r"restrict"),
                               ("return", r"return"),
                               ("short", r"short"),
                               ("signed", r"signed"),
                               ("sizeof", r"sizeof"),
                               ("static", r"static"),
                               ("struct", r"struct"),
                               ("switch", r"switch"),
                               ("typedef", r"typedef"),
                               ("union", r"union"),
                               ("unsigned", r"unsigned"),
                               ("void", r"void"),
                               ("volatile", r"volatile"),
                               ("while", r"while"),
                               // identifiers
                               ("ident", r"[A-Za-z][A-Za-z0-9]*"),
                               // constants
                               ("decimal", r"[0-9]+\.[0-9]*"),
                               ("integer", r"[0-9]+"),
                               // literals
                               ("string_literal", r#""[^"\\]*(\\.[^"\\]*)*""#),
                               ("character_literal", r"'([^'\\\n]|\\.)'"),
                               // operators
                               ("shift_right_assign", r">>="),
                               ("shift_left_assign", r"<<="),
                               ("add_assign", r"\+="),
                               ("sub_assign", r"-="),
                               ("mul_assign", r"\*="),
                               ("div_assign", r"/="),
                               ("mod_assign", r"%="),
                               ("and_assign", r"&="),
                               ("xor_assign", r"\^="),
                               ("or_assign", r"\|="),
                               ("shift_right", r">>"),
                               ("shift_left", r"<<"),
                               ("increment", r"\+\+"),
                               ("decrement", r"--"),
                               ("deference_member", r"->"),
                               ("and", r"&&"),
                               ("or", r"\|\|"),
                               ("le", r"<="),
                               ("ge", r">="),
                               ("eq", r"=="),
                               ("ne", r"!="),
                               ("semicolon", r";"),
                               ("open_brace", r"\{"),
                               ("close_brace", r"\}"),
                               ("comma", r","),
                               ("colon", r":"),
                               ("equals", r"="),
                               ("open_paren", r"\("),
                               ("close_paren", r"\)"),
                               ("open_bracket", r"\["),
                               ("close_bracket", r"\]"),
                               ("period", r"\."),
                               ("ampersand", r"&"),
                               ("bang", r"!"),
                               ("tilde", r"~"),
                               ("dash", r"-"),
                               ("plus", r"\+"),
                               ("asterisk", r"\*"),
                               ("forward_slash", r"/"),
                               ("percent", r"%"),
                               ("lt", r"<"),
                               ("gt", r">"),
                               ("caret", r"\^"),
                               ("bar", r"\|"),
                               ("question_mark", r"\?"),
                               // whitespace
                               ("whitespace", r"[ \t]+")];

    let mut patterns: Vec<Pattern> = Vec::new();
    for line in pattern_definitions.iter()
    {
        let p = Pattern { name:  String::from(line.0),
                          regex: Regex::new(line.1).unwrap() };
        patterns.push(p);
    }

    return patterns;
}
