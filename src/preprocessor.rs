use nom::{
    IResult,
    character::complete::{
        none_of,
        char,
        line_ending,
        not_line_ending,
    },
    bytes::complete::{
        tag,
        is_not,
        escaped,
    },
    branch::{
        alt,
        permutation,
    },
    combinator::{
        map,
        all_consuming,
    },
    multi::{
        many0,
    },
    sequence::delimited,
    error::{
        VerboseError,
        convert_error,
    },
    Err::Error,
};

pub fn preprocess(s: &str) -> Result<String, String> {
    let preproccessed = source_code_all_consuming(s);
    match preproccessed {
        Ok(source_code) => {
            Ok(source_code.1)
        }
        Err(error) => {
            match error {
                Error(e) => {
                    Err(convert_error(s, e))
                },
                _ => {
                    Err("Unknown error".to_owned())
                }
            }
        },
    }
}
pub fn source_code_all_consuming(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    all_consuming(source_code)(s)
}
pub fn source_code(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    map(
        many0(
            alt((
                comment,
                string,
                other,
            )),
        ),
        |strings| {
            let mut buffer = String::new();
            for s in strings {
                buffer.push_str(&s);
            }
            buffer
        }
    )(s)
}
pub fn comment(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    map(
        permutation((
            tag("#"),
            map(
                not_line_ending,
                |s: &str| {
                    s.to_owned()
                }
            ),
            line_ending,
        )),
        |(_, _, _)| {
            "\n".to_owned()
        },
    )(s)
}
pub fn string(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    map(
        delimited(
            char('"'),
            escaped(
                none_of("\"\\"),
                '\\',
                tag("\""),
            ),
            char('"'),
        ),
        |s: &str| {
            let mut buffer = String::new();
            buffer.push('\"');
            buffer.push_str(s);
            buffer.push('\"');
            buffer
        }
    )(s)
}
pub fn other(s: &str) -> IResult<&str, String, VerboseError<&str>> {
    map(
        is_not("#\""),
        |s: &str| {
            s.to_owned()
        },
    )(s)
}
