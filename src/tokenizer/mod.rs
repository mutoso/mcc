/*
    Copyright © 2019 Alastair Feille

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/.
*/

use regex::Regex;

mod patterns;

#[derive(Debug)]
pub struct Token
{
    pub name:   String,
    pub lexeme: String
}

pub fn run(input: &str) -> Vec<Token>
{
    let mut tokens: Vec<Token> = Vec::new();
    let pattern_vec = patterns::load_patterns();
    let mut in_comment = false;
    let mut multiline_comment_text = String::new();
    for (i, line) in input.lines().enumerate()
    {
        let line_no = i + 1;
        let mut line: String = String::from(line);
        if in_comment
        {
            if line.contains("*/")
            {
                let comment_regex = Regex::new(r"^.*\*/").unwrap();

                let comment = comment_regex.find(&line).unwrap().as_str();
                multiline_comment_text.push_str(&comment);

                let new_line = comment_regex.replace(&line, "").to_string();
                line = String::from(new_line);

                tokens.push(Token { name:   String::from("comment"),
                                    lexeme: String::from(multiline_comment_text.clone()) });

                multiline_comment_text = String::new();

                in_comment = false;
            }
            // we're in the middle of a multiline comment that
            // has neither /* nor */
            else
            {
                multiline_comment_text.push_str(&line);
                multiline_comment_text.push('\n');
                continue;
            }
        }

        if line.trim().starts_with("//")
        {
            tokens.push(Token { name:   String::from("comment"),
                                lexeme: String::from(line.trim().clone()) });
            continue;
        }
        else if line.trim().starts_with("/*") && !in_comment
        {
            in_comment = true;
            // multiline comment ends on same line
            if line.contains("*/")
            {
                let comment_regex = Regex::new(r"/\*.*\*/").unwrap();

                let comment = comment_regex.find(&line).unwrap().as_str();
                multiline_comment_text.push_str(&comment);

                let new_line = comment_regex.replace(&line, "").to_string();
                line = String::from(new_line);

                tokens.push(Token { name:   String::from("comment"),
                                    lexeme: String::from(multiline_comment_text.clone()) });

                multiline_comment_text = String::new();

                in_comment = false;
            }
            // otherwise, it ends on some line further down
            else
            {
                let comment_regex = Regex::new(r"/\*.*").unwrap();

                let comment = comment_regex.find(&line).unwrap();
                multiline_comment_text.push_str(&comment.as_str());
                multiline_comment_text.push('\n');

                continue;
            }
        }
        if !in_comment
        {
            // split string literal properly
            //let words = line.split_whitespace();
            let words = split_whitespace_and_strings(&line);
            for word in words
            {
                let mut current_word = String::from(word);
                while !current_word.is_empty()
                {
                    let token = get_token(&current_word, &pattern_vec);
                    if token.is_none()
                    {
                        println!("{}", line);
                    }
                    else
                    {
                        let token = token.unwrap();
                        let new_word = current_word.replacen(&token.lexeme, "", 1);
                        debug!("Line {}: Word changed from \"{}\" to \"{}\"",
                               line_no, current_word, new_word);
                        current_word = new_word;
                        tokens.push(token);
                    }
                }
            }
        }
    }

    return tokens;
}

fn get_token(word: &str, pattern_vec: &Vec<patterns::Pattern>) -> Option<Token>
{
    for pattern in pattern_vec
    {
        if pattern.regex.is_match(word)
        {
            let mat = pattern.regex.find(word).unwrap();
            if mat.start() == 0
            {
                let lexeme = mat.as_str();
                return Some(Token { name:   pattern.name.clone(),
                                    lexeme: String::from(lexeme) });
            }
        }
    }
    return None;
}

fn split_whitespace_and_strings(input: &str) -> Vec<String>
{
    let mut v = Vec::new();
    for (index, character) in input.chars().enumerate()
    {
        if character == '"'
        {
            v.push(index);
        }
        if index != 0
        {
            // ignore it if it's an escaped quote (i.e. \")
            if input.chars().nth(index - 1) == Some('\\')
            {
                v.pop();
            }
            // or if it's a character literal (i.e. '"')
            if input.chars().nth(index - 1) == Some('\'')
               && input.chars().nth(index + 1) == Some('\'')
            {
                v.pop();
            }
        }
    }

    // if no quotes
    if v.is_empty()
    {
        return input.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
    }

    if v.len() % 2 != 0
    {
        panic!("Odd number of quotations! String: {}", input);
    }

    let pairs: Vec<_> = v.chunks(2).collect();
    let chars: Vec<_> = input.chars().collect();
    let mut results = Vec::new();
    for (index, pair) in pairs.iter().enumerate()
    {
        let start = pair[0];
        let end = pair[1];
        // if beginning
        if index == 0
        {
            results.push((chars[0..start].to_vec().iter().collect::<String>(), false));
            // the string itself
            results.push((chars[start..=end].to_vec().iter().collect::<String>(), true));
        }
        // if end
        else if index == pairs.len() - 1
        {
            let end_of_prev = pairs[index - 1][1];
            results.push((chars[end_of_prev + 1..start].to_vec()
                                                       .iter()
                                                       .collect::<String>(),
                          false));
            // the quoted string itself
            results.push((chars[start..=end].to_vec().iter().collect::<String>(), true));
            results.push((chars[end + 1..chars.len() - 1].to_vec()
                                                         .iter()
                                                         .collect::<String>(),
                          false));
        }
        // if middle
        else
        {
            let end_of_prev = pairs[index - 1][1];
            results.push((chars[end_of_prev + 1..start].to_vec()
                                                       .iter()
                                                       .collect::<String>(),
                          false));
            // the string itself
            results.push((chars[start..=end].to_vec().iter().collect::<String>(), true));
        }
    }
    let mut final_results = Vec::new();
    for (string, is_string_literal) in results
    {
        if is_string_literal
        {
            final_results.push(string);
        }
        else
        {
            // tokenize it further
            for item in string.split_whitespace()
            {
                final_results.push(String::from(item));
            }
        }
    }
    return final_results;
}
