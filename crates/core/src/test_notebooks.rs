#[cfg(test)]
mod tests {
    use insta::assert_snapshot;
    use marzano_language::target_language::TargetLanguage;

    use crate::{
        api::MatchResult,
        pattern_compiler::src_to_problem_libs,
        test_utils::{run_on_test_files, SyntheticFile},
    };

    use std::collections::BTreeMap;

    #[test]
    fn test_base_case() {
        let pattern_src = r#"
        language python

        `print($x)` => `flink($x)`
        "#;
        let libs = BTreeMap::new();

        let matching_src = include_str!("../../../crates/cli_bin/fixtures/notebooks/tiny_nb.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);
        println!("{:?}", results);
        assert!(!results.iter().any(|r| r.is_error()));

        let rewrite = results
            .iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)))
            .unwrap();

        if let MatchResult::Rewrite(rewrite) = rewrite {
            assert_snapshot!(rewrite.rewritten.content);
        } else {
            panic!("Expected a rewrite");
        }
    }

    #[test]
    fn test_old_notebooks() {
        let pattern_src = r#"
        language python

        `print($x)` => `flink($x)`
        "#;
        let libs = BTreeMap::new();

        let matching_src = include_str!("../../../crates/cli_bin/fixtures/notebooks/old_nb.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);
        // We *do* expect an error on old notebooks
        assert!(results.iter().any(|r| r.is_error()));
    }

    #[test]
    fn test_changing_size() {
        // The rewrite has a different length, so the source map needs to be used

        let pattern_src = r#"
        language python

        `print($x)` => `THIS_IS_MUCH_MUCH_MUCH_MUCH_MUCH_MUCH_LONGER($x)`
        "#;
        let libs = BTreeMap::new();

        let matching_src = include_str!("../../../crates/cli_bin/fixtures/notebooks/tiny_nb.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);

        println!("{:?}", results);
        assert!(!results.iter().any(|r| r.is_error()));

        let rewrite = results
            .iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)))
            .unwrap();

        if let MatchResult::Rewrite(rewrite) = rewrite {
            assert_snapshot!(rewrite.rewritten.content);
        } else {
            panic!("Expected a rewrite");
        }
    }

    #[test]
    fn test_multi_cell_small() {
        // The rewrite has a different length, so the source map needs to be used

        let pattern_src = r#"
        language python

        `print($x)` => `p($x)`
        "#;
        let libs = BTreeMap::new();

        let matching_src =
            include_str!("../../../crates/cli_bin/fixtures/notebooks/multi_cell.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);

        println!("{:?}", results);
        assert!(!results.iter().any(|r| r.is_error()));

        let rewrite = results
            .iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)))
            .unwrap();

        if let MatchResult::Rewrite(rewrite) = rewrite {
            assert_snapshot!(rewrite.rewritten.content);
        } else {
            panic!("Expected a rewrite");
        }
    }

    #[test]
    fn test_sequential() {
        // Make sure we handle sequential transforms too

        let pattern_src = r#"
        language python

    sequential {
            bubble file($body) where $body <: contains bubble `print($x)` => `p($x)`,
            bubble file($body) where $body <: contains bubble `p($y)` => `s($y)`,
            bubble file($body) where $body <: contains bubble `s($a)` => `flint($a)`,
            bubble file($body) where $body <: contains bubble `flint($a)` => `x($a, 10)`,
    }
    "#;
        let libs = BTreeMap::new();

        let matching_src =
            include_str!("../../../crates/cli_bin/fixtures/notebooks/multi_cell.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);

        println!("{:?}", results);
        assert!(!results.iter().any(|r| r.is_error()));

        let rewrite = results
            .iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)))
            .unwrap();

        if let MatchResult::Rewrite(rewrite) = rewrite {
            assert_snapshot!(rewrite.rewritten.content);
        } else {
            panic!("Expected a rewrite");
        }
    }

    #[test]
    fn test_insertion() {
        let pattern_src = r#"
        language python

        `from langchain.agents import AgentType, initialize_agent, load_tools` as $anchor where {
                $anchor += `\nfrom foo insert new_import`
        }
        "#;
        let libs = BTreeMap::new();

        let matching_src =
            include_str!("../../../crates/cli_bin/fixtures/notebooks/langchain_cp.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);
        assert!(!results.iter().any(|r| r.is_error()));

        let rewrite = results
            .iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)))
            .unwrap();

        if let MatchResult::Rewrite(rewrite) = rewrite {
            assert_snapshot!(rewrite.rewritten.content);
        } else {
            panic!("Expected a rewrite");
        }
    }

    #[test]
    fn test_weird_side_effects_orphans() {
        let pattern_src = r#"
        language python

        or {
            `from langchain.agents import $stuffs` as $anchor where {
                $stuffs <: contains `load_tools` => .,
                $anchor += `\nfrom my_thing import tools`
            }
        }
        "#;
        let libs = BTreeMap::new();

        let matching_src =
            include_str!("../../../crates/cli_bin/fixtures/notebooks/langchain_open.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);
        assert!(!results.iter().any(|r| r.is_error()));

        let rewrite = results
            .iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)))
            .unwrap();

        if let MatchResult::Rewrite(rewrite) = rewrite {
            assert!(!rewrite.rewritten.content.contains("\"gent_chain.run"));
            assert_snapshot!(rewrite.rewritten.content);
        } else {
            panic!("Expected a rewrite");
        }
    }

    #[test]
    fn test_python3_kernelspec() {
        let pattern_src = r#"
        language python

       `langchain` => `fangchain`
        "#;
        let libs = BTreeMap::new();

        let matching_src =
            include_str!("../../../crates/cli_bin/fixtures/notebooks/kind_of_python.ipynb");

        let pattern = src_to_problem_libs(
            pattern_src.to_string(),
            &libs,
            TargetLanguage::from_extension("ipynb").unwrap(),
            None,
            None,
            None,
            None,
        )
        .unwrap()
        .problem;

        // Basic match works
        let test_files = vec![SyntheticFile::new(
            "target.ipynb".to_owned(),
            matching_src.to_owned(),
            true,
        )];
        let results = run_on_test_files(&pattern, &test_files);
        for r in &results {
            if r.is_error() {
                panic!("{:?}", r);
            }
        }

        let rewrite = results
            .iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)))
            .unwrap();

        if let MatchResult::Rewrite(rewrite) = rewrite {
            assert_snapshot!(rewrite.rewritten.content);
        } else {
            panic!("Expected a rewrite");
        }
    }
}
