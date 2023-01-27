use datadriven::walk;

use postgres_datetime::datetime::{decode, parse};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        walk("tests/testdata/parse", |f| {
            f.run(|test_case| -> String { format!("{:?}\n", parse(&test_case.input)) })
        });

        walk("tests/testdata/decode", |f| {
            f.run(|test_case| -> String {
                println!("test_case: {}", &test_case.input);
                format!("{:?}\n", parse(&test_case.input).and_then(decode))
            })
        });
    }
}
