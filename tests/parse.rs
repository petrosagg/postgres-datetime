use datadriven::walk;

use postgres_datetime::datetime::parse;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        walk("tests/testdata", |f| {
            f.run(|test_case| -> String {
                format!("{:?}", parse(&test_case.input))
            })
        });
    }
}
