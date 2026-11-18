// Copyright (c) 2026 NTUR LAB S.r.l. (https://ntur-lab.com)
//
// This file is part of gql-rs (https://ntur-lab.com/gql-rs)
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
//
// Authors:
//   Ntur Lab S.r.l. <dev@ntur-lab.com>,
//   Nicola Miotto <nicola.miotto@ntur-lab.com>,
//   Matteo Boscolo <matteo.boscolo@ntur-lab.com>

//! Tests of the built-in function registry (issue #19).

use super::super::builtins::*;
use super::super::expression::*;
use crate::ast::model::*;
use crate::ast::validator::tests::common_tests::*;
use crate::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::Path;

/// Variables available to hand-built calls, with the types the registry can check.
fn scope() -> (HashSet<String>, HashMap<String, TypeSpec>) {
    let declared = HashSet::from(
        ["n", "m", "e", "untyped", "bytes", "path", "list", "when"].map(str::to_string),
    );
    let inferred = HashMap::from([
        ("bytes".to_string(), TypeSpec::Bytes { max_length: None }),
        ("path".to_string(), TypeSpec::Path),
        ("list".to_string(), list_type(TypeSpec::Integer)),
        ("when".to_string(), TypeSpec::Date),
    ]);
    (declared, inferred)
}

fn call_with(name: &str, arguments: Vec<Expression>) -> FunctionCall {
    function_call(name, arguments)
}

fn validate(call: &FunctionCall) -> Result<(), crate::GqlError> {
    let (declared, inferred) = scope();
    validate_function_call_signature(call, &declared, &inferred)
}

/// A valid argument for the constraint.
fn valid_argument(kind: ArgKind) -> Expression {
    match kind {
        ArgKind::Any | ArgKind::Numeric | ArgKind::NonNegativeInteger => integer(1),
        ArgKind::Fraction => float(0.5),
        ArgKind::NumericOrDuration => float(1.5),
        ArgKind::CharacterString | ArgKind::CharacterOrByteString | ArgKind::TemporalSource => {
            string("text")
        }
        ArgKind::ByteString => variable("bytes"),
        ArgKind::Collection => variable("list"),
        ArgKind::Path => variable("path"),
        ArgKind::Datetime => variable("when"),
        ArgKind::ElementVariable => variable("n"),
        ArgKind::PropertyName | ArgKind::LabelExpression => string("name"),
        ArgKind::Keyword(values) => string(values[0]),
    }
}

/// An argument of the wrong shape or type for the constraint, if one exists.
fn invalid_argument(kind: ArgKind) -> Option<Expression> {
    match kind {
        ArgKind::Any | ArgKind::LabelExpression => None,
        ArgKind::ElementVariable | ArgKind::PropertyName | ArgKind::Keyword(_) => Some(integer(1)),
        _ => Some(boolean(true)),
    }
}

fn kinds_for(entry: &BuiltinFunction, count: usize) -> Vec<ArgKind> {
    match entry.parameters {
        Parameters::Positional(kinds) => (0..count)
            .map(|i| kinds.get(i).copied().unwrap_or(ArgKind::Any))
            .collect(),
        Parameters::Repeated(kind) => vec![kind; count],
        Parameters::Trim => match count {
            1 => vec![ArgKind::CharacterOrByteString],
            2 => vec![ArgKind::CharacterOrByteString; 2],
            _ => vec![
                ArgKind::Keyword(&["LEADING", "TRAILING", "BOTH"]),
                ArgKind::CharacterOrByteString,
                ArgKind::CharacterOrByteString,
            ],
        },
    }
}

fn valid_counts(arity: Arity) -> Vec<usize> {
    match arity {
        Arity::Exact(n) => vec![n],
        Arity::Range(min, max) => (min..=max).collect(),
        Arity::AtLeast(min) => vec![min, min + 1],
    }
}

fn invalid_counts(arity: Arity) -> Vec<usize> {
    let mut counts = match arity {
        Arity::Exact(n) => vec![n + 1],
        Arity::Range(_, max) => vec![max + 1],
        Arity::AtLeast(_) => vec![],
    };
    let min = match arity {
        Arity::Exact(n) => n,
        Arity::Range(min, _) | Arity::AtLeast(min) => min,
    };
    if min > 0 {
        counts.push(min - 1);
    }
    counts
}

fn valid_call(entry: &BuiltinFunction, count: usize) -> FunctionCall {
    let arguments = kinds_for(entry, count)
        .into_iter()
        .map(valid_argument)
        .collect();
    call_with(entry.name, arguments)
}

/// Collects the names of every function call in a serialized AST.
fn collect_function_names(value: &serde_json::Value, names: &mut BTreeSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Object(call)) = map.get("FunctionCall")
                && let Some(serde_json::Value::String(name)) = call.get("name")
            {
                names.insert(name.clone());
            }
            for child in map.values() {
                collect_function_names(child, names);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_function_names(item, names);
            }
        }
        _ => {}
    }
}

#[test]
fn registry_names_and_grammar_tokens_are_unique() {
    let mut names = BTreeSet::new();
    let mut tokens = BTreeSet::new();
    for entry in BUILTIN_FUNCTIONS {
        assert!(names.insert(entry.name), "duplicate entry {}", entry.name);
        assert_eq!(
            entry.name,
            entry.name.to_ascii_uppercase(),
            "{}",
            entry.name
        );
        assert!(
            !entry.grammar_tokens.is_empty(),
            "{} has no grammar tokens",
            entry.name
        );
        for token in entry.grammar_tokens {
            assert!(tokens.insert(*token), "token {token} claimed twice");
        }
        assert_eq!(
            entry.aggregate,
            entry.family == FunctionFamily::Aggregate,
            "{}",
            entry.name
        );
        assert_eq!(entry.set_quantifier, entry.aggregate, "{}", entry.name);
    }
}

#[test]
fn every_registry_entry_is_produced_by_the_builder() {
    for entry in BUILTIN_FUNCTIONS {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let ast = analyzer
            .exec(entry.example)
            .unwrap_or_else(|error| panic!("{}: {} -> {error}", entry.name, entry.example));
        let json = serde_json::to_value(&ast).expect("AST serializes");
        let mut names = BTreeSet::new();
        collect_function_names(&json, &mut names);
        assert!(
            names.contains(entry.name),
            "{}: example {} produced {names:?}",
            entry.name,
            entry.example
        );
        for name in &names {
            assert!(
                lookup_builtin(name).is_some(),
                "{}: example produced unregistered function {name}",
                entry.name
            );
        }
    }
}

#[test]
fn every_builtin_accepts_its_valid_argument_counts() {
    for entry in BUILTIN_FUNCTIONS {
        for count in valid_counts(entry.arity) {
            let call = valid_call(entry, count);
            assert!(
                validate(&call).is_ok(),
                "{} with {count} arguments: {:?}",
                entry.name,
                validate(&call)
            );
        }
    }
}

#[test]
fn every_builtin_rejects_invalid_argument_counts() {
    for entry in BUILTIN_FUNCTIONS {
        let counts = invalid_counts(entry.arity);
        assert!(!counts.is_empty(), "{} has no invalid arity", entry.name);
        for count in counts {
            let call = call_with(entry.name, vec![integer(1); count]);
            let error = validate(&call)
                .expect_err(&format!("{} must reject {count} arguments", entry.name));
            let message = error.to_string();
            assert!(
                message.contains("requires")
                    || message.contains("accepts at most")
                    || message.contains("does not accept"),
                "{}: {message}",
                entry.name
            );
        }
    }
}

#[test]
fn every_constrained_argument_rejects_wrong_types_and_accepts_unknown_types() {
    for entry in BUILTIN_FUNCTIONS {
        if matches!(entry.parameters, Parameters::Trim) {
            continue;
        }
        let count = valid_counts(entry.arity).into_iter().max().unwrap_or(0);
        let kinds = kinds_for(entry, count);
        for (position, kind) in kinds.iter().enumerate() {
            let Some(wrong) = invalid_argument(*kind) else {
                continue;
            };
            let mut call = valid_call(entry, count);
            call.arguments[position] = wrong;
            assert!(
                validate(&call).is_err(),
                "{} argument {} ({kind:?}) must be rejected",
                entry.name,
                position + 1
            );

            if !matches!(kind, ArgKind::PropertyName | ArgKind::Keyword(_)) {
                let mut call = valid_call(entry, count);
                call.arguments[position] = variable("untyped");
                assert!(
                    validate(&call).is_ok(),
                    "{} argument {} must accept an unknown type: {:?}",
                    entry.name,
                    position + 1,
                    validate(&call)
                );
            }
        }
    }
}

#[test]
fn trim_validates_string_and_list_forms() {
    for call in [
        call_with("TRIM", vec![string(" a ")]),
        call_with("TRIM", vec![string("x"), string("xax")]),
        call_with("TRIM", vec![string("LEADING"), string("x"), string("xa")]),
        call_with("TRIM", vec![variable("bytes"), variable("bytes")]),
        call_with("TRIM", vec![variable("list"), integer(1)]),
        call_with("TRIM", vec![variable("untyped"), variable("untyped")]),
    ] {
        assert!(validate(&call).is_ok(), "{call:?}: {:?}", validate(&call));
    }

    for call in [
        call_with("TRIM", vec![integer(1)]),
        call_with("TRIM", vec![variable("list"), integer(-1)]),
        call_with("TRIM", vec![variable("list"), string("x")]),
        call_with("TRIM", vec![string("SIDEWAYS"), string("x"), string("xa")]),
        call_with("TRIM", vec![string("BOTH"), integer(1), string("xa")]),
    ] {
        assert!(validate(&call).is_err(), "{call:?} must be rejected");
    }
}

#[test]
fn function_specific_restrictions_are_enforced() {
    for call in [
        call_with("LEFT", vec![string("abc"), integer(-1)]),
        call_with("RIGHT", vec![string("abc"), float(1.5)]),
        call_with("PERCENTILE_CONT", vec![integer(1), float(1.5)]),
        call_with("PERCENTILE_DISC", vec![integer(1), integer(-1)]),
        call_with("NORMALIZE", vec![string("a"), string("NFX")]),
        call_with("IS_NORMALIZED", vec![string("a"), string("")]),
        call_with("ELEMENT_ID", vec![integer(1)]),
        call_with("PROPERTY_EXISTS", vec![variable("n"), string(" ")]),
        call_with("IS_SOURCE_OF", vec![variable("n"), integer(1)]),
        call_with("ALL_DIFFERENT", vec![variable("n"), string("m")]),
        call_with(
            "DURATION_BETWEEN",
            vec![variable("when"), variable("when"), string("WEEK")],
        ),
        call_with(
            "DURATION_BETWEEN",
            vec![
                variable("when"),
                Expression::Literal(Literal::Duration("P1D".to_string())),
            ],
        ),
        call_with("BYTE_LENGTH", vec![string("text")]),
        call_with("CHAR_LENGTH", vec![variable("bytes")]),
    ] {
        assert!(validate(&call).is_err(), "{call:?} must be rejected");
    }

    for call in [
        call_with("LEFT", vec![string("abc"), variable("untyped")]),
        call_with("LEFT", vec![variable("bytes"), integer(0)]),
        call_with("PERCENTILE_CONT", vec![integer(1), integer(1)]),
        call_with("NORMALIZE", vec![string("a"), string("nfkd")]),
        call_with(
            "DURATION_BETWEEN",
            vec![variable("when"), variable("when"), string("DAY TO SECOND")],
        ),
        call_with(
            "ABS",
            vec![Expression::Literal(Literal::Duration("P1D".to_string()))],
        ),
        call_with(
            "SUM",
            vec![Expression::Literal(Literal::Duration("P1D".to_string()))],
        ),
        call_with("FLOOR", vec![Expression::Literal(Literal::Null)]),
    ] {
        assert!(validate(&call).is_ok(), "{call:?}: {:?}", validate(&call));
    }
}

#[test]
fn set_quantifiers_are_only_accepted_by_aggregates() {
    for entry in BUILTIN_FUNCTIONS {
        let count = valid_counts(entry.arity)[0];
        for distinct in [DistinctQualifier::Distinct, DistinctQualifier::All] {
            let mut call = valid_call(entry, count);
            call.distinct = distinct.clone();
            assert_eq!(
                validate(&call).is_ok(),
                entry.set_quantifier,
                "{} with {distinct:?}: {:?}",
                entry.name,
                validate(&call)
            );
        }
    }
}

#[test]
fn every_aggregate_rejects_nested_aggregates() {
    for entry in BUILTIN_FUNCTIONS.iter().filter(|entry| entry.aggregate) {
        let mut call = valid_call(entry, valid_counts(entry.arity)[0]);
        call.arguments[0] = function("COUNT", vec![integer(1)]);
        let error = validate(&call).expect_err(entry.name);
        assert!(
            error
                .to_string()
                .contains("cannot contain nested aggregate expressions"),
            "{}: {error}",
            entry.name
        );
    }
}

#[test]
fn user_defined_functions_keep_generic_handling() {
    for name in ["myFunction", "schema.FLOOR", "ns.sub.count", "COLLECT_SET"] {
        assert!(lookup_builtin(name).is_none(), "{name}");
        let call = call_with(name, vec![boolean(true), string("x"), integer(1)]);
        assert!(validate(&call).is_ok(), "{name}");
        let (declared, inferred) = scope();
        assert_eq!(
            infer_function_call_type(&call, &inferred, Some(&declared)),
            None,
            "{name}"
        );
    }
}

#[test]
fn builtin_lookup_is_case_insensitive_for_unqualified_names() {
    assert_eq!(lookup_builtin("floor").map(|e| e.name), Some("FLOOR"));
    assert_eq!(lookup_builtin(" Upper ").map(|e| e.name), Some("UPPER"));
    assert!(lookup_builtin("pkg.upper").is_none());
}

#[test]
fn return_types_follow_registry_rules() {
    let (declared, inferred) = scope();
    let duration = Expression::Literal(Literal::Duration("P1D".to_string()));
    let cases = vec![
        (
            call_with("FLOOR", vec![float(1.5)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            call_with("FLOOR", vec![integer(1)]),
            Some(TypeSpec::Integer),
        ),
        (
            call_with("ABS", vec![duration.clone()]),
            Some(TypeSpec::Duration {
                precision: None,
                qualifier: None,
            }),
        ),
        (
            call_with("MOD", vec![integer(7), integer(2)]),
            Some(TypeSpec::Integer),
        ),
        (
            call_with("MOD", vec![integer(7), float(2.0)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            call_with("SQRT", vec![integer(4)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            call_with("SIN", vec![integer(0)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            call_with("CHAR_LENGTH", vec![string("a")]),
            Some(TypeSpec::Integer),
        ),
        (
            call_with("BYTE_LENGTH", vec![variable("bytes")]),
            Some(TypeSpec::Integer),
        ),
        (
            call_with("LEFT", vec![variable("bytes"), integer(1)]),
            Some(TypeSpec::Bytes { max_length: None }),
        ),
        (
            call_with("LEFT", vec![string("abc"), integer(1)]),
            Some(TypeSpec::String { max_length: None }),
        ),
        (
            call_with("TRIM", vec![variable("list"), integer(1)]),
            Some(list_type(TypeSpec::Integer)),
        ),
        (
            call_with("TRIM", vec![string("x"), string("xa")]),
            Some(TypeSpec::String { max_length: None }),
        ),
        (
            call_with("SIZE", vec![variable("list")]),
            Some(TypeSpec::Integer),
        ),
        (
            call_with("ELEMENTS", vec![variable("path")]),
            Some(list_type(TypeSpec::Any)),
        ),
        (
            call_with("PATH_LENGTH", vec![variable("path")]),
            Some(TypeSpec::Integer),
        ),
        (
            call_with("COLLECT_LIST", vec![string("a")]),
            Some(list_type(TypeSpec::String { max_length: None })),
        ),
        (
            call_with("AVG", vec![integer(1)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            call_with("AVG", vec![duration.clone()]),
            Some(TypeSpec::Duration {
                precision: None,
                qualifier: None,
            }),
        ),
        (call_with("SUM", vec![integer(1)]), Some(TypeSpec::Integer)),
        (
            call_with("SUM", vec![variable("untyped")]),
            Some(TypeSpec::Any),
        ),
        (
            call_with("PERCENTILE_CONT", vec![integer(1), float(0.5)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            call_with("STDDEV_POP", vec![integer(1)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (call_with("DATE", vec![]), Some(TypeSpec::Date)),
        (
            call_with("ZONED_TIME", vec![]),
            Some(TypeSpec::ZonedTime { precision: None }),
        ),
        (
            call_with("LOCAL_DATETIME", vec![]),
            Some(TypeSpec::LocalDateTime { precision: None }),
        ),
        (
            call_with("ZONED_DATETIME", vec![]),
            Some(TypeSpec::ZonedDateTime { precision: None }),
        ),
        (
            call_with(
                "DURATION_BETWEEN",
                vec![variable("when"), variable("when"), string("YEAR TO MONTH")],
            ),
            Some(TypeSpec::Duration {
                precision: None,
                qualifier: Some("YEAR TO MONTH".to_string()),
            }),
        ),
        (
            call_with("ELEMENT_ID", vec![variable("n")]),
            Some(TypeSpec::String { max_length: None }),
        ),
        (
            call_with("IS_DIRECTED", vec![variable("e")]),
            Some(TypeSpec::Boolean),
        ),
        (
            call_with("COALESCE", vec![integer(1), integer(2)]),
            Some(TypeSpec::Integer),
        ),
        (
            call_with("COALESCE", vec![integer(1), float(2.0)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            call_with(
                "COALESCE",
                vec![Expression::Literal(Literal::Null), string("a")],
            ),
            Some(TypeSpec::String { max_length: None }),
        ),
        (
            call_with("NULLIF", vec![string("a"), string("b")]),
            Some(TypeSpec::String { max_length: None }),
        ),
        (
            call_with("SESSION_USER", vec![]),
            Some(TypeSpec::String { max_length: None }),
        ),
    ];

    for (call, expected) in cases {
        assert_eq!(
            infer_function_call_type(&call, &inferred, Some(&declared)),
            expected,
            "{call:?}"
        );
    }
}

/// Extracts the body of every parser rule in the grammar, without comments and labels.
fn grammar_rules(grammar: &str) -> HashMap<String, String> {
    let mut rules = HashMap::new();
    let mut current: Option<(String, String)> = None;
    for raw_line in grammar.lines() {
        let line = raw_line.split("//").next().unwrap_or_default();
        if let Some((name, body)) = current.as_mut() {
            if line.trim() == ";" {
                rules.insert(name.clone(), body.clone());
                current = None;
            } else {
                body.push(' ');
                body.push_str(line);
            }
            continue;
        }
        let trimmed = line.trim();
        if !trimmed.is_empty()
            && trimmed
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_lowercase())
            && trimmed
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_')
        {
            current = Some((trimmed.to_string(), String::new()));
        }
    }
    rules
}

/// Splits a rule body into token references and rule references. Alternative labels
/// (`#name`) and element labels (`name = ...`) are skipped.
fn rule_symbols(body: &str) -> (Vec<String>, Vec<String>) {
    let chars: Vec<char> = body.chars().collect();
    let mut tokens = Vec::new();
    let mut rule_refs = Vec::new();
    let mut index = 0;
    while index < chars.len() {
        if !(chars[index].is_ascii_alphanumeric() || chars[index] == '_') {
            index += 1;
            continue;
        }
        let start = index;
        while index < chars.len() && (chars[index].is_ascii_alphanumeric() || chars[index] == '_') {
            index += 1;
        }
        let word: String = chars[start..index].iter().collect();
        let is_alternative_label = start > 0 && chars[start - 1] == '#';
        let next = chars[index..].iter().find(|c| !c.is_whitespace());
        let is_element_label = next == Some(&'=');
        if is_alternative_label || is_element_label {
            continue;
        }
        match word.chars().next() {
            Some(c) if c.is_ascii_uppercase() => tokens.push(word),
            Some(c) if c.is_ascii_lowercase() => rule_refs.push(word),
            _ => {}
        }
    }
    (tokens, rule_refs)
}

#[test]
fn registry_covers_every_grammar_builtin() {
    let grammar_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/GQL.g4");
    let grammar = fs::read_to_string(&grammar_path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", grammar_path.display()));
    let rules = grammar_rules(&grammar);

    // Grammar rules whose alternatives are built as function calls.
    let roots = [
        "valueFunction",
        "aggregateFunction",
        "caseAbbreviation",
        "element_idFunction",
        "generalValueSpecification",
        "directedPredicatePart2",
        "labeledPredicatePart2",
        "sourcePredicatePart2",
        "destinationPredicatePart2",
        "all_differentPredicate",
        "samePredicate",
        "property_existsPredicate",
        "normalizedPredicatePart2",
    ];
    // Argument, modifier and reference rules: their content is not a function form.
    let argument_rules: HashSet<&str> = [
        "valueExpression",
        "numericValueExpression",
        "numericValueExpressionBase",
        "numericValueExpressionExponent",
        "numericValueExpressionDividend",
        "numericValueExpressionDivisor",
        "generalLogarithmBase",
        "generalLogarithmArgument",
        "listValueExpression",
        "pathValueExpression",
        "characterStringValueExpression",
        "byteStringValueExpression",
        "cardinalityExpressionArgument",
        "datetimeSubtractionParameters",
        "trimOperands",
        "stringLength",
        "dependentValueExpression",
        "independentValueExpression",
        "dateFunctionParameters",
        "timeFunctionParameters",
        "datetimeFunctionParameters",
        "durationFunctionParameters",
        "elementVariableReference",
        "edgeReference",
        "propertyName",
        "labelExpression",
        "normalForm",
        "setQuantifier",
        "temporalDurationQualifier",
        "dynamicParameterSpecification",
        "userDefinedFunction",
    ]
    .into_iter()
    .collect();
    // Punctuation and keywords that only shape a function form.
    let structural_tokens: HashSet<&str> = [
        "LEFT_PAREN",
        "RIGHT_PAREN",
        "COMMA",
        "ASTERISK",
        "IS",
        "NOT",
        "OF",
    ]
    .into_iter()
    .collect();

    let mut grammar_tokens = BTreeSet::new();
    let mut visited = HashSet::new();
    let mut stack: Vec<String> = roots.iter().map(|root| root.to_string()).collect();
    while let Some(rule) = stack.pop() {
        if !visited.insert(rule.clone()) || argument_rules.contains(rule.as_str()) {
            continue;
        }
        let body = rules
            .get(&rule)
            .unwrap_or_else(|| panic!("grammar rule {rule} not found"));
        let (tokens, rule_refs) = rule_symbols(body);
        grammar_tokens.extend(
            tokens
                .into_iter()
                .filter(|token| !structural_tokens.contains(token.as_str())),
        );
        stack.extend(rule_refs);
    }

    let registry_tokens: BTreeSet<String> = BUILTIN_FUNCTIONS
        .iter()
        .flat_map(|entry| entry.grammar_tokens.iter().map(|token| token.to_string()))
        .collect();

    let missing: Vec<_> = grammar_tokens.difference(&registry_tokens).collect();
    let stale: Vec<_> = registry_tokens.difference(&grammar_tokens).collect();
    assert!(
        missing.is_empty(),
        "grammar built-in tokens missing from the registry: {missing:?}"
    );
    assert!(
        stale.is_empty(),
        "registry tokens not produced by any grammar built-in form: {stale:?}"
    );
}

#[test]
fn builtin_function_documentation_lists_every_entry() {
    let docs_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/builtin_functions.md");
    let docs = fs::read_to_string(&docs_path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", docs_path.display()));
    for entry in BUILTIN_FUNCTIONS {
        assert!(
            docs.contains(&format!("| `{}` |", entry.name)),
            "{} is not documented in docs/builtin_functions.md",
            entry.name
        );
        if let ValidationLevel::Partial(reason) = entry.validation {
            assert!(
                docs.contains(reason),
                "{} limitation is not documented: {reason}",
                entry.name
            );
        }
    }
}
