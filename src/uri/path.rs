use nom::{
    bytes::complete::tag,
    combinator::opt,
    error::{context, ErrorKind},
    multi::many0,
    sequence::{terminated, tuple},
    AsChar, InputTakeAtPosition,
};

use crate::uri::CustomResult;

// What does this even do?
pub fn url_code_points<T>(input: T) -> CustomResult<T, T>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let char_item = item.as_char();
            !(char_item == '-') && !char_item.is_alphanum() && !(char_item == '.')
            // ... actual ascii code points and url encoding...:
            // https://infra.spec.whatwg.org/#ascii-code-point
        },
        ErrorKind::AlphaNumeric,
    )
}

// Converts "/path/to/my/blog/index.php"
// to vec!["path", "to", "my", "blog", "index.php"]
pub fn path_parser(input: &str) -> CustomResult<&str, Vec<&str>> {
    context(
        "path",
        tuple((
            // detect the start of the path
            tag("/"),
            // gather all path elements into a vector
            many0(
                // a path element is anything matching url_code_points followed by a /
                terminated(url_code_points, tag("/")),
            ),
            // Detect anything that follows the last slash, for example, "index.php"
            opt(url_code_points),
        )),
    )(input)
    .map(|(next_input, res)| {
        // res looks like this: ("/", Vec<&str>, Option<str>)
        let mut path: Vec<&str> = res.1.iter().map(|p| p.to_owned()).collect();
        if let Some(last) = res.2 {
            path.push(last);
        }
        (next_input, path)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_parser() {
        assert_eq!(path_parser("/a/b/c?d"), Ok(("?d", vec!["a", "b", "c"])));

        assert_eq!(
            path_parser("/a/1234/c.txt?d"),
            Ok(("?d", vec!["a", "1234", "c.txt"]))
        );

        assert_eq!(
            path_parser("/a/b-c-d/c/?d"),
            Ok(("?d", vec!["a", "b-c-d", "c"]))
        );

        assert_eq!(
            path_parser("/a/1234/c/?d"),
            Ok(("?d", vec!["a", "1234", "c"]))
        );

        assert_eq!(
            path_parser("/a/1234/c.txt?d"),
            Ok(("?d", vec!["a", "1234", "c.txt"]))
        );
    }
}
