#![cfg(test)]
mod tests {

    use trim_margin::MarginTrimmable;

    use crate::test::{run_test_expected, TestArgExpected};

    #[test]
    fn is_not_undefined() {
        run_test_expected({
            TestArgExpected {
                pattern: r#"
                    |engine marzano(0.1)
                    |language js
                    |or {
                    |    `1`,
                    |    `2` where $is_two = true
                    |} where {
                    |    if ($is_two <: not undefined) {
                    |        $n = `two`
                    |    } else {
                    |        $n = `one`
                    |    }
                    |} => $n"#
                    .trim_margin()
                    .unwrap(),
                source: r#"
                console.log(1);
                console.log(2)"#
                    .to_owned(),
                expected: r#"
                console.log(one);
                console.log(two)"#
                    .to_owned(),
            }
        })
        .unwrap();
    }
}
