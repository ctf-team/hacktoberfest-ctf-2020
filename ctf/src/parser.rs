extern crate nom;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::multi::separated_list;
use nom::sequence::*;
use nom::*;

extern crate indexmap;
use indexmap::IndexMap;

use std::str;

use crate::commands::Token;

fn match_word(i: &str) -> IResult<&str, &str> {
    take_till(|c| c == ' ' || c == '\n')(i)
}

fn get_quoted(i: &str) -> IResult<&str, &str> {
    delimited(
        alt((tag("'"), tag("\""))),
        take_till(|c| c == '\'' || c == '"'),
        alt((tag("'"), tag("\""))),
    )(i)
}

fn match_quote(i: &str) -> IResult<&str, &str> {
    if i.starts_with('\'') || i.starts_with('"') {
        get_quoted(i)
    } else {
        is_not(" ")(i)
    }
}

fn parse_words(i: &str) -> IResult<&str, Vec<&str>> {
    separated_list(tag(" "), alt((match_quote, match_word)))(i)
}

#[derive(Debug)]
struct KeyVal {
    pub key: String,
    pub value: String,
}

fn parse_keyval(i: &str) -> Result<KeyVal, &str> {
    let output: IResult<&str, Vec<&str>> =
        separated_list(tag("="), alt((get_quoted, is_not("="))))(i);

    let (_, result) = output.unwrap();

    if result.len() != 2 {
        return Err("Please specify your arguments with '--key=val' syntax.");
    }

    Ok(KeyVal {
        key: result[0].to_string(),
        value: result[1].to_string(),
    })
}

fn parse_key_values(input: &Vec<&str>) -> IndexMap<String, String> {
    // parse args by --key val or just --key --key
    let mut args: IndexMap<String, String> = IndexMap::new();
    let mut key: String = String::default();
    let mut in_key: bool = false;
    for i in 0..input.len() {
        if input[i].starts_with("-") {
            if in_key && i == input.len() - 1 {
                // insert
                args.insert(key.to_string(), String::default());
                // insert final key
                args.insert(input[i].to_string(), String::default());
            }
            // parse with special parser.
            let output = parse_keyval(input[i]);
            match output {
                Ok(v) => {
                    args.insert(v.key, v.value);
                    in_key = false;
                }
                Err(_) => {
                    key = input[i].to_string();
                    in_key = true;
                }
            };
            continue;
        }

        if in_key {
            args.insert(key, input[i].to_string());
            in_key = !in_key;
            key = String::default();
        } else {
            args.insert(input[i].to_string(), String::default());
        }
    }

    args
}

pub fn parse_input(i: &String) -> Result<Token, &str> {
    if i.len() <= 0 {
        return Err("");
    }

    let (_, parsed_words) = parse_words(i.as_str()).unwrap();
    let parsed_keyval = parse_key_values(&parsed_words[1..].to_vec());

    Ok(Token {
        name: parsed_words[0].to_string(),
        parameters: parsed_words[1..].to_vec(),
        args: parsed_keyval,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_word() {
        let (_processed, output) = match_word("ayy lmao").unwrap();
        assert_eq!(output, "ayy");
    }

    #[test]
    fn can_get_quoted() {
        let (_processed, output) = get_quoted("'ayy lmao' lol").unwrap();
        assert_eq!(output, "ayy lmao");
        let (_processed, output) = get_quoted("\"ayy lmao\" lol").unwrap();
        assert_eq!(output, "ayy lmao");
    }

    #[test]
    fn can_parse_quotes_and_words() {
        let (_processed, output) = parse_words("ayy lmao").unwrap();
        assert_eq!(output, vec!["ayy", "lmao"]);
        let (_processed, output) = parse_words("ayy 'lmao dude sup' lol").unwrap();
        assert_eq!(output, vec!["ayy", "lmao dude sup", "lol"]);
        let (_processed, output) =
            parse_words("ayy lmao this is a really long string boiii").unwrap();
        assert_eq!(
            output,
            vec!["ayy", "lmao", "this", "is", "a", "really", "long", "string", "boiii"]
        );
        let (_processed, output) = parse_words("ayy 'lmao o' \"this is\" my 'string g\"").unwrap();
        assert_eq!(output, vec!["ayy", "lmao o", "this is", "my", "string g"]);
    }

    #[test]
    fn can_parse_keyval() {
        let output = parse_keyval("--key=val").unwrap();
        assert_eq!(output.key, "--key");
        assert_eq!(output.value, "val");
        let output = parse_keyval("-key=val").unwrap();
        assert_eq!(output.key, "-key");
        assert_eq!(output.value, "val");
        let output = parse_keyval("key=").unwrap_err();
        assert_eq!(
            output,
            "Please specify your arguments with \'--key=val\' syntax."
        );
    }

    #[test]
    fn can_parse_key_values() {
        let output = parse_key_values(&vec!["--key", "val", "--test=testa"]);
        assert_eq!(
            output,
            [
                ("--key".to_string(), "val".to_string()),
                ("--test".to_string(), "testa".to_string())
            ]
            .iter()
            .cloned()
            .collect::<IndexMap<String, String>>()
        );

        let output = parse_key_values(&vec!["test", "--key", "--key1"]);
        assert_eq!(
            output,
            [
                ("test".to_string(), String::default()),
                ("--key".to_string(), String::default()),
                ("--key1".to_string(), String::default())
            ]
            .iter()
            .cloned()
            .collect::<IndexMap<String, String>>()
        );

        let output = parse_key_values(&vec!["--key1=\"value1\"", "--key2='value2'"]);
        assert_eq!(
            output,
            [
                ("--key1".to_string(), "value1".to_string()),
                ("--key2".to_string(), "value2".to_string())
            ]
            .iter()
            .cloned()
            .collect::<IndexMap<String, String>>()
        );
    }

    #[test]
    fn can_parse_entire_command_input() {
        let x = "test stuff --arg1=val1 --arg2 --arg3".to_string();
        let output = parse_input(&x).unwrap();
        assert_eq!(output.name, "test");
        assert_eq!(
            output.parameters,
            vec!["stuff", "--arg1=val1", "--arg2", "--arg3"]
        );
        assert_eq!(
            output.args,
            [
                ("stuff".to_string(), String::default()),
                ("--arg1".to_string(), "val1".to_string()),
                ("--arg2".to_string(), String::default()),
                ("--arg3".to_string(), String::default())
            ]
            .iter()
            .cloned()
            .collect::<IndexMap<String, String>>()
        );
    }
}
