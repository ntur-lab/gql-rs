// Copyright (c) 2026 NTUR LAB S.r.l.
//
// This file is part of NTURDB.
//
// Licensed under the EUPL, Version 1.2 or - as soon they will be approved by
// the European Commission - subsequent versions of the EUPL (the "Licence");
// You may not use this work except in compliance with the Licence.
// You may obtain a copy of the Licence at:
//
// https://joinup.ec.europa.eu/software/page/eupl
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licence is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licence for the specific language governing permissions and
// limitations under the Licence.

use gql_parser::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};
use libtest_mimic::{Arguments, Failed, Trial};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let args = Arguments::from_args();
    let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/integration/fixtures/ast");
    let trials = load_trials(&fixture_root).unwrap_or_else(|error| {
        eprintln!("Unable to load AST fixtures: {error}");
        std::process::exit(2);
    });

    libtest_mimic::run(&args, trials).exit();
}

fn load_trials(root: &Path) -> Result<Vec<Trial>, String> {
    let mut query_paths = Vec::new();
    collect_query_paths(root, &mut query_paths)?;
    query_paths.sort();

    if query_paths.is_empty() {
        return Err(format!("no .gql fixtures found below {}", root.display()));
    }

    Ok(query_paths
        .into_iter()
        .map(|query_path| {
            let expected_path = query_path.with_extension("ast.json");
            let test_name = query_path
                .strip_prefix(root)
                .unwrap_or(&query_path)
                .with_extension("")
                .display()
                .to_string();

            Trial::test(test_name, move || compare_ast(&query_path, &expected_path))
        })
        .collect())
}

fn collect_query_paths(directory: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(directory)
        .map_err(|error| format!("cannot read {}: {error}", directory.display()))?;

    for entry in entries {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_query_paths(&path, paths)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("gql") {
            paths.push(path);
        }
    }

    Ok(())
}

fn compare_ast(query_path: &Path, expected_path: &Path) -> Result<(), Failed> {
    let query = fs::read_to_string(query_path)
        .map_err(|error| format!("cannot read {}: {error}", query_path.display()))?;
    let mut analyzer = DefaultGQLQueryAnalyzer::new();
    let ast = analyzer
        .exec(&query)
        .map_err(|error| format!("cannot analyze {}: {error:?}", query_path.display()))?;
    let actual = serde_json::to_value(ast)
        .map_err(|error| format!("cannot serialize AST for {}: {error}", query_path.display()))?;

    if std::env::var_os("UPDATE_AST_FIXTURES").is_some() {
        let mut serialized = serde_json::to_string_pretty(&actual)
            .map_err(|error| format!("cannot format AST for {}: {error}", query_path.display()))?;
        serialized.push('\n');
        fs::write(expected_path, serialized)
            .map_err(|error| format!("cannot update {}: {error}", expected_path.display()))?;
        return Ok(());
    }

    let expected_source = fs::read_to_string(expected_path)
        .map_err(|error| format!("cannot read {}: {error}", expected_path.display()))?;
    let expected: Value = serde_json::from_str(&expected_source)
        .map_err(|error| format!("invalid JSON in {}: {error}", expected_path.display()))?;

    if actual != expected {
        let expected_pretty = serde_json::to_string_pretty(&expected).unwrap_or(expected_source);
        let actual_pretty = serde_json::to_string_pretty(&actual)
            .unwrap_or_else(|_| "<unprintable AST>".to_string());
        return Err(format!(
            "AST mismatch for {}\n\nexpected:\n{}\n\nactual:\n{}",
            query_path.display(),
            expected_pretty,
            actual_pretty
        )
        .into());
    }

    Ok(())
}
