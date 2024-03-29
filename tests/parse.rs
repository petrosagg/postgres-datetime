use datadriven::walk;

use postgres_datetime::datetime::decode;
use postgres_datetime::datetime_raw::parse_datetime;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        walk("tests/testdata/parse", |f| {
            f.run(|test_case| -> String { format!("{:?}\n", parse_datetime(&test_case.input)) })
        });

        walk("tests/testdata/decode", |f| {
            f.run(|test_case| -> String {
                format!("{:?}\n", parse_datetime(&test_case.input).and_then(decode))
            })
        });
    }
}
