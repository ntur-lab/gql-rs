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

//! Registry of the built-in functions admitted by the GQL grammar.
//!
//! Every function form the AST builder can produce as a [`FunctionCall`] (ordinary built-in
//! functions, aggregates, function-like predicates and CASE abbreviations) has exactly one
//! entry here. An entry records the identity of the function, the grammar keywords that
//! produce it, accepted argument counts, applicable modifiers, the argument constraints that
//! the current type model can express, and the return type rule. Validation and return-type
//! inference of built-in calls are dispatched exclusively through this registry; names that
//! are not registered are user-defined functions and keep the generic handling (arguments are
//! validated, no signature is enforced, the return type is unknown).

use super::*;

/// Function family of a built-in function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionFamily {
    /// Set functions (COUNT, SUM, PERCENTILE_CONT, ...).
    Aggregate,
    /// Numeric functions (ABS, MOD, FLOOR, trigonometric and logarithmic functions, ...).
    Numeric,
    /// Character string functions.
    CharacterString,
    /// Byte string functions.
    ByteString,
    /// Functions accepting character or byte strings.
    CharacterOrByteString,
    /// List functions.
    List,
    /// Path functions.
    Path,
    /// Date, time and datetime functions.
    Temporal,
    /// Duration functions.
    Duration,
    /// Graph element functions and function-like predicates.
    GraphElement,
    /// CASE abbreviations (COALESCE, NULLIF).
    Conditional,
    /// Session value functions.
    Session,
}

/// Accepted argument counts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Arity {
    /// Exactly the given number of arguments.
    Exact(usize),
    /// Between the given bounds, inclusive.
    Range(usize, usize),
    /// At least the given number of arguments.
    AtLeast(usize),
}

impl Arity {
    pub(crate) fn accepts(self, count: usize) -> bool {
        match self {
            Arity::Exact(n) => count == n,
            Arity::Range(min, max) => (min..=max).contains(&count),
            Arity::AtLeast(min) => count >= min,
        }
    }

    fn describe(self) -> String {
        fn word(n: usize) -> String {
            match n {
                0 => "zero".to_string(),
                1 => "one".to_string(),
                2 => "two".to_string(),
                3 => "three".to_string(),
                other => other.to_string(),
            }
        }
        fn noun(n: usize) -> &'static str {
            if n == 1 { "argument" } else { "arguments" }
        }

        match self {
            Arity::Exact(0) => "does not accept arguments".to_string(),
            Arity::Exact(n) => format!("requires exactly {} {}", word(n), noun(n)),
            Arity::Range(0, max) => format!("accepts at most {} {}", word(max), noun(max)),
            Arity::Range(1, 3) => "requires one, two, or three arguments".to_string(),
            Arity::Range(min, max) => format!("requires {} or {} arguments", word(min), word(max)),
            Arity::AtLeast(min) => format!("requires at least {} {}", word(min), noun(min)),
        }
    }
}

/// Constraint on one argument, expressed with the current type model. Type constraints are
/// only enforced when the argument type is known; unknown types are never rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ArgKind {
    /// Any value.
    Any,
    /// Any numeric value.
    Numeric,
    /// An exact integral value; integer literals must not be negative.
    NonNegativeInteger,
    /// A numeric value; numeric literals must lie in `[0, 1]`.
    Fraction,
    /// A numeric or duration value.
    NumericOrDuration,
    /// A character string.
    CharacterString,
    /// A byte string.
    ByteString,
    /// A character or byte string.
    CharacterOrByteString,
    /// A list, character or byte string, path, or binding table (SIZE, CARDINALITY).
    Collection,
    /// A path.
    Path,
    /// A date, time or datetime value (not a duration).
    Datetime,
    /// A character string or a record constructor (temporal constructor arguments).
    TemporalSource,
    /// A reference to a graph element binding variable.
    ElementVariable,
    /// A non-empty property name carried as a string literal.
    PropertyName,
    /// A label expression built from label-name literals and boolean operators.
    LabelExpression,
    /// A keyword modifier carried as a string literal, one of the listed spellings.
    Keyword(&'static [&'static str]),
}

/// Argument shape of a built-in function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Parameters {
    /// One constraint per position; positions beyond the list are unconstrained.
    Positional(&'static [ArgKind]),
    /// The same constraint for every argument.
    Repeated(ArgKind),
    /// `TRIM`, which covers both the character/byte string form
    /// `TRIM([spec] [char] FROM source)` and the list form `TRIM(list, count)`.
    Trim,
}

/// Return type rule of a built-in function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ReturnRule {
    Boolean,
    Integer,
    Float,
    CharacterString,
    Date,
    ZonedTime,
    LocalTime,
    ZonedDateTime,
    LocalDateTime,
    Duration,
    /// `DURATION_BETWEEN`: a duration carrying the optional qualifier argument.
    DurationWithQualifier,
    /// A list of unknown element type.
    AnyList,
    /// A list whose element type is the type of the given argument.
    ListOfArgument(usize),
    /// The type of the given argument.
    SameAsArgument(usize),
    /// A character string, or a byte string when the given argument is a byte string.
    StringLikeArgument(usize),
    /// AVG: a duration for duration arguments, a float otherwise.
    Average,
    /// The common type of all arguments when they agree.
    CommonArgumentType,
    /// The type of the list or string being trimmed.
    Trim,
}

/// How completely the registry validates a function.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ValidationLevel {
    /// Arity, modifiers and argument types are fully checked within the current type model.
    Complete,
    /// Some constraints cannot be checked yet; the reason names the missing model.
    Partial(&'static str),
}

/// Signature metadata of one built-in function.
#[derive(Debug)]
// Grammar tokens, validation level and examples are read by the registry coverage tests and
// documentation checks.
#[cfg_attr(not(test), allow(dead_code))]
pub(crate) struct BuiltinFunction {
    /// Canonical name produced by the AST builder.
    pub(crate) name: &'static str,
    /// Function family.
    pub(crate) family: FunctionFamily,
    /// Grammar keyword tokens that produce this function.
    pub(crate) grammar_tokens: &'static [&'static str],
    /// Accepted argument counts.
    pub(crate) arity: Arity,
    /// Argument constraints.
    pub(crate) parameters: Parameters,
    /// Whether a DISTINCT/ALL set quantifier is admitted.
    pub(crate) set_quantifier: bool,
    /// Whether the function is an aggregate.
    pub(crate) aggregate: bool,
    /// Return type rule.
    pub(crate) returns: ReturnRule,
    /// Validation completeness.
    pub(crate) validation: ValidationLevel,
    /// A query exercising the grammar form of this function.
    pub(crate) example: &'static str,
}

const NORMAL_FORMS: &[&str] = &["NFC", "NFD", "NFKC", "NFKD"];
const TRIM_SPECIFICATIONS: &[&str] = &["LEADING", "TRAILING", "BOTH"];
const DURATION_QUALIFIERS: &[&str] = &["YEAR TO MONTH", "DAY TO SECOND"];

const PATH_TYPING: &str = "path variables are not typed until the semantic context of issue #12";
const COMPARABLE_TYPING: &str =
    "comparable-type constraints need the typed semantic context of issue #12";
const TEMPORAL_STRING: &str =
    "the contents of temporal strings and the fields of temporal records are not checked";

macro_rules! builtin {
    (
        $name:expr, $family:ident, [$($token:expr),*], $arity:expr, $parameters:expr,
        $returns:expr, $validation:expr, $example:expr
        $(, set_quantifier: $sq:expr)? $(, aggregate: $agg:expr)?
    ) => {
        BuiltinFunction {
            name: $name,
            family: FunctionFamily::$family,
            grammar_tokens: &[$($token),*],
            arity: $arity,
            parameters: $parameters,
            set_quantifier: false $(|| $sq)?,
            aggregate: false $(|| $agg)?,
            returns: $returns,
            validation: $validation,
            example: $example,
        }
    };
}

use ArgKind as A;
use Arity::{AtLeast, Exact, Range};
use Parameters::{Positional, Repeated};
use ValidationLevel::{Complete, Partial};

/// Every built-in function form admitted by the repository grammar.
pub(crate) static BUILTIN_FUNCTIONS: &[BuiltinFunction] = &[
    // Aggregates.
    builtin!(fn_names::COUNT, Aggregate, ["COUNT"], Exact(1), Positional(&[A::Any]),
        ReturnRule::Integer, Complete, "MATCH (n) RETURN COUNT(*), COUNT(DISTINCT n)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::SUM, Aggregate, ["SUM"], Exact(1), Positional(&[A::NumericOrDuration]),
        ReturnRule::SameAsArgument(0), Complete, "MATCH (n) RETURN SUM(n.x)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::AVG, Aggregate, ["AVG"], Exact(1), Positional(&[A::NumericOrDuration]),
        ReturnRule::Average, Complete, "MATCH (n) RETURN AVG(ALL n.x)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::MIN, Aggregate, ["MIN"], Exact(1), Positional(&[A::Any]),
        ReturnRule::SameAsArgument(0), Partial(COMPARABLE_TYPING), "MATCH (n) RETURN MIN(n.x)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::MAX, Aggregate, ["MAX"], Exact(1), Positional(&[A::Any]),
        ReturnRule::SameAsArgument(0), Partial(COMPARABLE_TYPING), "MATCH (n) RETURN MAX(n.x)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::COLLECT_LIST, Aggregate, ["COLLECT_LIST"], Exact(1), Positional(&[A::Any]),
        ReturnRule::ListOfArgument(0), Complete, "MATCH (n) RETURN COLLECT_LIST(n.x)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::STDDEV_SAMP, Aggregate, ["STDDEV_SAMP"], Exact(1), Positional(&[A::Numeric]),
        ReturnRule::Float, Complete, "MATCH (n) RETURN STDDEV_SAMP(n.x)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::STDDEV_POP, Aggregate, ["STDDEV_POP"], Exact(1), Positional(&[A::Numeric]),
        ReturnRule::Float, Complete, "MATCH (n) RETURN STDDEV_POP(n.x)",
        set_quantifier: true, aggregate: true),
    builtin!(fn_names::PERCENTILE_CONT, Aggregate, ["PERCENTILE_CONT"], Exact(2),
        Positional(&[A::Numeric, A::Fraction]), ReturnRule::Float, Complete,
        "MATCH (n) RETURN PERCENTILE_CONT(n.x, 0.5)", set_quantifier: true, aggregate: true),
    builtin!(fn_names::PERCENTILE_DISC, Aggregate, ["PERCENTILE_DISC"], Exact(2),
        Positional(&[A::Numeric, A::Fraction]), ReturnRule::SameAsArgument(0), Complete,
        "MATCH (n) RETURN PERCENTILE_DISC(DISTINCT n.x, 0.5)", set_quantifier: true, aggregate: true),
    // Numeric functions.
    builtin!(
        fn_names::ABS,
        Numeric,
        ["ABS"],
        Exact(1),
        Positional(&[A::NumericOrDuration]),
        ReturnRule::SameAsArgument(0),
        Complete,
        "RETURN ABS(-1), ABS(DURATION('P1D'))"
    ),
    builtin!(
        fn_names::MOD,
        Numeric,
        ["MOD"],
        Exact(2),
        Positional(&[A::Numeric, A::Numeric]),
        ReturnRule::CommonArgumentType,
        Complete,
        "RETURN MOD(7, 2)"
    ),
    builtin!(
        fn_names::FLOOR,
        Numeric,
        ["FLOOR"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::SameAsArgument(0),
        Complete,
        "RETURN FLOOR(1.5)"
    ),
    builtin!(
        fn_names::CEIL,
        Numeric,
        ["CEIL", "CEILING"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::SameAsArgument(0),
        Complete,
        "RETURN CEIL(1.5), CEILING(1.5)"
    ),
    builtin!(
        fn_names::SQRT,
        Numeric,
        ["SQRT"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN SQRT(4)"
    ),
    builtin!(
        fn_names::POWER,
        Numeric,
        ["POWER"],
        Exact(2),
        Positional(&[A::Numeric, A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN POWER(2, 3)"
    ),
    builtin!(
        fn_names::EXP,
        Numeric,
        ["EXP"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN EXP(1)"
    ),
    builtin!(
        fn_names::LN,
        Numeric,
        ["LN"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN LN(1)"
    ),
    builtin!(
        fn_names::LOG10,
        Numeric,
        ["LOG10"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN LOG10(100)"
    ),
    builtin!(
        fn_names::LOG,
        Numeric,
        ["LOG_KW"],
        Exact(2),
        Positional(&[A::Numeric, A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN LOG(2, 8)"
    ),
    builtin!(
        fn_names::SIN,
        Numeric,
        ["SIN"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN SIN(0)"
    ),
    builtin!(
        fn_names::COS,
        Numeric,
        ["COS"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN COS(0)"
    ),
    builtin!(
        fn_names::TAN,
        Numeric,
        ["TAN"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN TAN(0)"
    ),
    builtin!(
        fn_names::COT,
        Numeric,
        ["COT"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN COT(1)"
    ),
    builtin!(
        fn_names::SINH,
        Numeric,
        ["SINH"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN SINH(0)"
    ),
    builtin!(
        fn_names::COSH,
        Numeric,
        ["COSH"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN COSH(0)"
    ),
    builtin!(
        fn_names::TANH,
        Numeric,
        ["TANH"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN TANH(0)"
    ),
    builtin!(
        fn_names::ASIN,
        Numeric,
        ["ASIN"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN ASIN(0)"
    ),
    builtin!(
        fn_names::ACOS,
        Numeric,
        ["ACOS"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN ACOS(1)"
    ),
    builtin!(
        fn_names::ATAN,
        Numeric,
        ["ATAN"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN ATAN(0)"
    ),
    builtin!(
        fn_names::DEGREES,
        Numeric,
        ["DEGREES"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN DEGREES(1)"
    ),
    builtin!(
        fn_names::RADIANS,
        Numeric,
        ["RADIANS"],
        Exact(1),
        Positional(&[A::Numeric]),
        ReturnRule::Float,
        Complete,
        "RETURN RADIANS(180)"
    ),
    // Character string functions.
    builtin!(
        fn_names::CHAR_LENGTH,
        CharacterString,
        ["CHAR_LENGTH"],
        Exact(1),
        Positional(&[A::CharacterString]),
        ReturnRule::Integer,
        Complete,
        "RETURN CHAR_LENGTH('abc')"
    ),
    builtin!(
        fn_names::CHARACTER_LENGTH,
        CharacterString,
        ["CHARACTER_LENGTH"],
        Exact(1),
        Positional(&[A::CharacterString]),
        ReturnRule::Integer,
        Complete,
        "RETURN CHARACTER_LENGTH('abc')"
    ),
    builtin!(
        fn_names::UPPER,
        CharacterString,
        ["UPPER"],
        Exact(1),
        Positional(&[A::CharacterString]),
        ReturnRule::CharacterString,
        Complete,
        "RETURN UPPER('abc')"
    ),
    builtin!(
        fn_names::LOWER,
        CharacterString,
        ["LOWER"],
        Exact(1),
        Positional(&[A::CharacterString]),
        ReturnRule::CharacterString,
        Complete,
        "RETURN LOWER('ABC')"
    ),
    builtin!(
        fn_names::LTRIM,
        CharacterString,
        ["LTRIM"],
        Range(1, 2),
        Repeated(A::CharacterString),
        ReturnRule::CharacterString,
        Complete,
        "RETURN LTRIM('  a'), LTRIM('xxa', 'x')"
    ),
    builtin!(
        fn_names::RTRIM,
        CharacterString,
        ["RTRIM"],
        Range(1, 2),
        Repeated(A::CharacterString),
        ReturnRule::CharacterString,
        Complete,
        "RETURN RTRIM('a  '), RTRIM('axx', 'x')"
    ),
    builtin!(
        fn_names::BTRIM,
        CharacterString,
        ["BTRIM"],
        Range(1, 2),
        Repeated(A::CharacterString),
        ReturnRule::CharacterString,
        Complete,
        "RETURN BTRIM(' a '), BTRIM('xax', 'x')"
    ),
    builtin!(
        fn_names::NORMALIZE,
        CharacterString,
        ["NORMALIZE"],
        Range(1, 2),
        Positional(&[A::CharacterString, A::Keyword(NORMAL_FORMS)]),
        ReturnRule::CharacterString,
        Complete,
        "RETURN NORMALIZE('a'), NORMALIZE('a', NFKC)"
    ),
    // Byte string functions.
    builtin!(
        fn_names::BYTE_LENGTH,
        ByteString,
        ["BYTE_LENGTH"],
        Exact(1),
        Positional(&[A::ByteString]),
        ReturnRule::Integer,
        Complete,
        "MATCH (n) RETURN BYTE_LENGTH(X'00FF'), BYTE_LENGTH(n.payload)"
    ),
    builtin!(
        fn_names::OCTET_LENGTH,
        ByteString,
        ["OCTET_LENGTH"],
        Exact(1),
        Positional(&[A::ByteString]),
        ReturnRule::Integer,
        Complete,
        "MATCH (n) RETURN OCTET_LENGTH(X'00FF'), OCTET_LENGTH(n.payload)"
    ),
    // Character or byte string functions.
    builtin!(
        fn_names::LEFT,
        CharacterOrByteString,
        ["LEFT"],
        Exact(2),
        Positional(&[A::CharacterOrByteString, A::NonNegativeInteger]),
        ReturnRule::StringLikeArgument(0),
        Complete,
        "RETURN LEFT('abc', 2)"
    ),
    builtin!(
        fn_names::RIGHT,
        CharacterOrByteString,
        ["RIGHT"],
        Exact(2),
        Positional(&[A::CharacterOrByteString, A::NonNegativeInteger]),
        ReturnRule::StringLikeArgument(0),
        Complete,
        "RETURN RIGHT('abc', 2)"
    ),
    builtin!(
        fn_names::TRIM,
        CharacterOrByteString,
        ["TRIM"],
        Range(1, 3),
        Parameters::Trim,
        ReturnRule::Trim,
        Partial(
            "TRIM(spec FROM source) and TRIM('x' FROM source) share one argument shape until the expression redesign of 0.1.0"
        ),
        "RETURN TRIM(' a '), TRIM(LEADING 'x' FROM 'xa'), TRIM([1, 2, 3], 1)"
    ),
    // List functions.
    builtin!(
        fn_names::SIZE,
        List,
        ["SIZE"],
        Exact(1),
        Positional(&[A::Collection]),
        ReturnRule::Integer,
        Complete,
        "RETURN SIZE([1, 2])"
    ),
    builtin!(
        fn_names::CARDINALITY,
        List,
        ["CARDINALITY"],
        Exact(1),
        Positional(&[A::Collection]),
        ReturnRule::Integer,
        Complete,
        "RETURN CARDINALITY([1, 2])"
    ),
    builtin!(
        fn_names::ELEMENTS,
        Path,
        ["ELEMENTS"],
        Exact(1),
        Positional(&[A::Path]),
        ReturnRule::AnyList,
        Partial(PATH_TYPING),
        "MATCH p = (a)-[e]->(b) RETURN ELEMENTS(p)"
    ),
    // Path functions.
    builtin!(
        fn_names::PATH_LENGTH,
        Path,
        ["PATH_LENGTH"],
        Exact(1),
        Positional(&[A::Path]),
        ReturnRule::Integer,
        Partial(PATH_TYPING),
        "MATCH p = (a)-[e]->(b) RETURN PATH_LENGTH(p)"
    ),
    // Temporal functions.
    builtin!(
        fn_names::CURRENT_DATE,
        Temporal,
        ["CURRENT_DATE"],
        Exact(0),
        Positional(&[]),
        ReturnRule::Date,
        Complete,
        "RETURN CURRENT_DATE"
    ),
    builtin!(
        fn_names::DATE,
        Temporal,
        ["DATE"],
        Range(0, 1),
        Positional(&[A::TemporalSource]),
        ReturnRule::Date,
        Partial(TEMPORAL_STRING),
        "RETURN DATE(), DATE('2024-01-31')"
    ),
    builtin!(
        fn_names::CURRENT_TIME,
        Temporal,
        ["CURRENT_TIME"],
        Exact(0),
        Positional(&[]),
        ReturnRule::ZonedTime,
        Complete,
        "RETURN CURRENT_TIME"
    ),
    builtin!(
        fn_names::ZONED_TIME,
        Temporal,
        ["ZONED_TIME"],
        Range(0, 1),
        Positional(&[A::TemporalSource]),
        ReturnRule::ZonedTime,
        Partial(TEMPORAL_STRING),
        "RETURN ZONED_TIME('12:30:00+01:00')"
    ),
    builtin!(
        fn_names::LOCAL_TIME,
        Temporal,
        ["LOCAL_TIME"],
        Range(0, 1),
        Positional(&[A::TemporalSource]),
        ReturnRule::LocalTime,
        Partial(TEMPORAL_STRING),
        "RETURN LOCAL_TIME, LOCAL_TIME('12:30:00')"
    ),
    builtin!(
        fn_names::CURRENT_TIMESTAMP,
        Temporal,
        ["CURRENT_TIMESTAMP"],
        Exact(0),
        Positional(&[]),
        ReturnRule::ZonedDateTime,
        Complete,
        "RETURN CURRENT_TIMESTAMP"
    ),
    builtin!(
        fn_names::ZONED_DATETIME,
        Temporal,
        ["ZONED_DATETIME"],
        Range(0, 1),
        Positional(&[A::TemporalSource]),
        ReturnRule::ZonedDateTime,
        Partial(TEMPORAL_STRING),
        "RETURN ZONED_DATETIME('2024-01-31T12:30:00+01:00')"
    ),
    builtin!(
        fn_names::LOCAL_TIMESTAMP,
        Temporal,
        ["LOCAL_TIMESTAMP"],
        Exact(0),
        Positional(&[]),
        ReturnRule::LocalDateTime,
        Complete,
        "RETURN LOCAL_TIMESTAMP"
    ),
    builtin!(
        fn_names::LOCAL_DATETIME,
        Temporal,
        ["LOCAL_DATETIME"],
        Range(0, 1),
        Positional(&[A::TemporalSource]),
        ReturnRule::LocalDateTime,
        Partial(TEMPORAL_STRING),
        "RETURN LOCAL_DATETIME('2024-01-31T12:30:00')"
    ),
    // Duration functions.
    builtin!(
        fn_names::DURATION,
        Duration,
        ["DURATION"],
        Exact(1),
        Positional(&[A::TemporalSource]),
        ReturnRule::Duration,
        Partial(TEMPORAL_STRING),
        "RETURN DURATION('P1D')"
    ),
    builtin!(
        fn_names::DURATION_BETWEEN,
        Duration,
        ["DURATION_BETWEEN"],
        Range(2, 3),
        Positional(&[A::Datetime, A::Datetime, A::Keyword(DURATION_QUALIFIERS)]),
        ReturnRule::DurationWithQualifier,
        Complete,
        "RETURN DURATION_BETWEEN(CURRENT_DATE, CURRENT_DATE) YEAR TO MONTH"
    ),
    // Graph element functions and function-like predicates.
    builtin!(
        fn_names::ELEMENT_ID,
        GraphElement,
        ["ELEMENT_ID"],
        Exact(1),
        Positional(&[A::ElementVariable]),
        ReturnRule::CharacterString,
        Complete,
        "MATCH (n) RETURN ELEMENT_ID(n)"
    ),
    builtin!(
        fn_names::PROPERTY_EXISTS,
        GraphElement,
        ["PROPERTY_EXISTS"],
        Exact(2),
        Positional(&[A::ElementVariable, A::PropertyName]),
        ReturnRule::Boolean,
        Complete,
        "MATCH (n) RETURN PROPERTY_EXISTS(n, name)"
    ),
    builtin!(
        fn_names::IS_LABELED,
        GraphElement,
        ["LABELED", "COLON"],
        Exact(2),
        Positional(&[A::ElementVariable, A::LabelExpression]),
        ReturnRule::Boolean,
        Complete,
        "MATCH (n) RETURN n IS LABELED Person, n IS NOT LABELED Person & !City"
    ),
    builtin!(
        fn_names::IS_DIRECTED,
        GraphElement,
        ["DIRECTED"],
        Exact(1),
        Positional(&[A::ElementVariable]),
        ReturnRule::Boolean,
        Complete,
        "MATCH ()-[e]->() RETURN e IS DIRECTED"
    ),
    builtin!(
        fn_names::IS_SOURCE_OF,
        GraphElement,
        ["SOURCE"],
        Exact(2),
        Positional(&[A::ElementVariable, A::ElementVariable]),
        ReturnRule::Boolean,
        Complete,
        "MATCH (a)-[e]->(b) RETURN a IS SOURCE OF e"
    ),
    builtin!(
        fn_names::IS_DESTINATION_OF,
        GraphElement,
        ["DESTINATION"],
        Exact(2),
        Positional(&[A::ElementVariable, A::ElementVariable]),
        ReturnRule::Boolean,
        Complete,
        "MATCH (a)-[e]->(b) RETURN b IS DESTINATION OF e"
    ),
    builtin!(
        fn_names::ALL_DIFFERENT,
        GraphElement,
        ["ALL_DIFFERENT"],
        AtLeast(2),
        Repeated(A::ElementVariable),
        ReturnRule::Boolean,
        Complete,
        "MATCH (a)-[e]->(b) RETURN ALL_DIFFERENT(a, b)"
    ),
    builtin!(
        fn_names::SAME,
        GraphElement,
        ["SAME"],
        AtLeast(2),
        Repeated(A::ElementVariable),
        ReturnRule::Boolean,
        Complete,
        "MATCH (a)-[e]->(b) RETURN SAME(a, b)"
    ),
    builtin!(
        fn_names::IS_NORMALIZED,
        CharacterString,
        ["NORMALIZED"],
        Range(1, 2),
        Positional(&[A::CharacterString, A::Keyword(NORMAL_FORMS)]),
        ReturnRule::Boolean,
        Complete,
        "RETURN 'a' IS NORMALIZED, 'a' IS NOT NFD NORMALIZED"
    ),
    // CASE abbreviations.
    builtin!(
        fn_names::COALESCE,
        Conditional,
        ["COALESCE"],
        AtLeast(2),
        Repeated(A::Any),
        ReturnRule::CommonArgumentType,
        Partial(COMPARABLE_TYPING),
        "RETURN COALESCE(NULL, 1, 2)"
    ),
    builtin!(
        fn_names::NULLIF,
        Conditional,
        ["NULLIF"],
        Exact(2),
        Repeated(A::Any),
        ReturnRule::SameAsArgument(0),
        Partial(COMPARABLE_TYPING),
        "RETURN NULLIF(1, 2)"
    ),
    // Session value functions.
    builtin!(
        fn_names::SESSION_USER,
        Session,
        ["SESSION_USER"],
        Exact(0),
        Positional(&[]),
        ReturnRule::CharacterString,
        Complete,
        "RETURN SESSION_USER"
    ),
];

/// Looks up the built-in function produced by the builder under `name`. Qualified names
/// (`schema.function`) are always user-defined.
pub(crate) fn lookup_builtin(name: &str) -> Option<&'static BuiltinFunction> {
    let trimmed = name.trim();
    if trimmed.contains('.') {
        return None;
    }
    BUILTIN_FUNCTIONS
        .iter()
        .find(|entry| entry.name.eq_ignore_ascii_case(trimmed))
}

/// Returns true when `name` designates a registered aggregate function.
pub(crate) fn is_builtin_aggregate(name: &str) -> bool {
    lookup_builtin(name).is_some_and(|entry| entry.aggregate)
}

/// Returns the argument type when it is known, i.e. inferred and not ANY.
fn known_type(
    argument: &Expression,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Option<TypeSpec> {
    infer_expression_type(argument, inferred_types, Some(declared_variables))
        .filter(|t| !matches!(t, TypeSpec::Any))
}

fn is_integral_type(t: &TypeSpec) -> bool {
    matches!(t, TypeSpec::Integer { .. })
}

fn is_datetime_type(t: &TypeSpec) -> bool {
    is_temporal_like_type(t) && !is_duration_type(t)
}

/// Returns true when `t` satisfies the type part of `kind`. Unions are accepted when any of
/// their members is accepted.
fn type_satisfies(kind: ArgKind, t: &TypeSpec) -> bool {
    if let TypeSpec::Union { members } = t {
        return members.iter().any(|member| type_satisfies(kind, member));
    }

    match kind {
        A::Any | A::LabelExpression | A::ElementVariable | A::PropertyName | A::Keyword(_) => true,
        A::Numeric | A::Fraction => is_numeric_type(t),
        A::NonNegativeInteger => is_integral_type(t),
        A::NumericOrDuration => is_numeric_type(t) || is_duration_type(t),
        A::CharacterString => matches!(t, TypeSpec::String { .. }),
        A::ByteString => matches!(t, TypeSpec::Bytes { .. }),
        A::CharacterOrByteString => is_string_like_type(t),
        A::Collection => {
            is_string_like_type(t)
                || matches!(
                    t,
                    TypeSpec::List { .. } | TypeSpec::Path | TypeSpec::BindingTable { .. }
                )
        }
        A::Path => matches!(t, TypeSpec::Path),
        A::Datetime => is_datetime_type(t),
        A::TemporalSource => matches!(t, TypeSpec::String { .. } | TypeSpec::Record { .. }),
    }
}

fn kind_description(kind: ArgKind) -> String {
    match kind {
        A::Any => "a value".to_string(),
        A::Numeric => "a numeric argument".to_string(),
        A::NonNegativeInteger => "a non-negative integer".to_string(),
        A::Fraction => "a numeric argument between 0 and 1".to_string(),
        A::NumericOrDuration => "a numeric or duration argument".to_string(),
        A::CharacterString => "a string-like argument".to_string(),
        A::ByteString => "a byte string argument".to_string(),
        A::CharacterOrByteString => "a character or byte string argument".to_string(),
        A::Collection => "list/string/path argument".to_string(),
        A::Path => "a path argument".to_string(),
        A::Datetime => "temporal-compatible arguments".to_string(),
        A::TemporalSource => "a string or record argument".to_string(),
        A::ElementVariable => "a graph element variable reference".to_string(),
        A::PropertyName => "a property name".to_string(),
        A::LabelExpression => "a label expression".to_string(),
        A::Keyword(values) => format!("one of {}", values.join(", ")),
    }
}

/// Returns the value of a numeric literal argument, looking through unary signs: the builder
/// represents `-1` as a unary minus applied to the literal `1`.
fn numeric_literal_value(argument: &Expression) -> Option<f64> {
    match argument {
        Expression::Literal(Literal::Integer(value)) => Some(*value as f64),
        Expression::Literal(Literal::BigInteger(value)) => Some(*value as f64),
        Expression::Literal(Literal::Float(value)) => Some(*value),
        Expression::Literal(Literal::Decimal(text)) => text.parse().ok(),
        Expression::Unary(unary) => {
            let value = numeric_literal_value(&unary.expression)?;
            match unary.operator {
                Operator::Minus => Some(-value),
                Operator::Plus => Some(value),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Checks one argument against its constraint.
fn validate_argument(
    entry: &BuiltinFunction,
    call: &FunctionCall,
    position: usize,
    kind: ArgKind,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    let argument = &call.arguments[position];
    let reject = |requirement: String, found: Option<&TypeSpec>| {
        let detail = found
            .map(|t| format!(", got {}", type_name(t)))
            .unwrap_or_default();
        let message = if call.arguments.len() == 1 || matches!(kind, A::Datetime) {
            format!("{} requires {requirement}{detail}", entry.name)
        } else {
            format!(
                "{} argument {} requires {requirement}{detail}",
                entry.name,
                position + 1
            )
        };
        Err(validation_err(message, Some(&call.span.clone())))
    };

    // Structural restrictions that do not depend on type inference.
    match kind {
        A::ElementVariable if !matches!(argument, Expression::Variable { .. }) => {
            return reject(kind_description(kind), None);
        }
        A::PropertyName => {
            let valid = matches!(argument, Expression::Literal(Literal::String(name)) if !name.trim().is_empty());
            if !valid {
                return reject(kind_description(kind), None);
            }
        }
        A::Keyword(values) => {
            let valid = matches!(
                argument,
                Expression::Literal(Literal::String(value))
                    if values.iter().any(|candidate| candidate.eq_ignore_ascii_case(value.trim()))
            );
            if !valid {
                return reject(kind_description(kind), None);
            }
        }
        A::NonNegativeInteger
        if numeric_literal_value(argument).is_some_and(|value| value < 0.0) =>
            {
                return reject(kind_description(kind), None);
            }
        A::Fraction
        if numeric_literal_value(argument)
            .is_some_and(|value| !(0.0..=1.0).contains(&value)) =>
            {
                return reject(kind_description(kind), None);
            }
        _ => {}
    }

    // NULL is accepted wherever a value is expected.
    if matches!(argument, Expression::Literal(Literal::Null)) {
        return Ok(());
    }

    if let Some(t) = known_type(argument, declared_variables, inferred_types)
        && !type_satisfies(kind, &t)
    {
        return reject(kind_description(kind), Some(&t));
    }

    Ok(())
}

/// Validates the TRIM argument shapes.
fn validate_trim_arguments(
    entry: &BuiltinFunction,
    call: &FunctionCall,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    let first_type = call
        .arguments
        .first()
        .and_then(|argument| known_type(argument, declared_variables, inferred_types));
    let list_form = call.arguments.len() == 2
        && first_type
        .as_ref()
        .is_some_and(|t| matches!(t, TypeSpec::List { .. }));

    let kinds: &[ArgKind] = match call.arguments.len() {
        1 => &[A::CharacterOrByteString],
        2 if list_form => &[A::Any, A::NonNegativeInteger],
        2 => &[A::CharacterOrByteString, A::CharacterOrByteString],
        _ => &[
            A::Keyword(TRIM_SPECIFICATIONS),
            A::CharacterOrByteString,
            A::CharacterOrByteString,
        ],
    };

    for (position, kind) in kinds.iter().enumerate() {
        validate_argument(
            entry,
            call,
            position,
            *kind,
            declared_variables,
            inferred_types,
        )?;
    }
    Ok(())
}

/// Validates a call to a registered built-in function: argument count, set quantifier,
/// function-specific structural restrictions, argument types where known, and the nested
/// aggregate restriction.
pub(crate) fn validate_builtin_call(
    entry: &BuiltinFunction,
    call: &FunctionCall,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    if !entry.arity.accepts(call.arguments.len()) {
        return Err(validation_err(
            format!("{} {}", entry.name, entry.arity.describe()),
            Some(&call.span.clone()),
        ));
    }

    if !entry.set_quantifier && !matches!(call.distinct, DistinctQualifier::None) {
        return Err(validation_err(
            format!(
                "{} does not accept a DISTINCT or ALL set quantifier",
                entry.name
            ),
            Some(&call.span.clone()),
        ));
    }

    if entry.aggregate && call.arguments.iter().any(contains_aggregate_function) {
        return Err(validation_err(
            format!(
                "Aggregate function '{}' cannot contain nested aggregate expressions",
                call.name
            ),
            Some(&call.span.clone()),
        ));
    }

    match entry.parameters {
        Parameters::Positional(kinds) => {
            for (position, kind) in kinds.iter().enumerate().take(call.arguments.len()) {
                validate_argument(
                    entry,
                    call,
                    position,
                    *kind,
                    declared_variables,
                    inferred_types,
                )?;
            }
        }
        Parameters::Repeated(kind) => {
            for position in 0..call.arguments.len() {
                validate_argument(
                    entry,
                    call,
                    position,
                    kind,
                    declared_variables,
                    inferred_types,
                )?;
            }
        }
        Parameters::Trim => {
            validate_trim_arguments(entry, call, declared_variables, inferred_types)?
        }
    }

    if entry.returns == ReturnRule::CommonArgumentType
        && entry.family == FunctionFamily::Conditional
    {
        let mut accumulated: Option<TypeSpec> = None;
        for argument in &call.arguments {
            let Some(t) = known_type(argument, declared_variables, inferred_types) else {
                continue;
            };
            if let Some(previous) = &accumulated
                && !are_comparison_types_compatible(previous, &t)
            {
                return Err(validation_err(
                    format!("{} arguments must be mutually compatible", entry.name),
                    Some(&call.span.clone()),
                ));
            }
            accumulated = Some(t);
        }
    }

    Ok(())
}

/// Infers the return type of a call to a registered built-in function. Returns `None` when
/// the type depends on arguments whose type is unknown.
pub(crate) fn infer_builtin_return_type(
    entry: &BuiltinFunction,
    call: &FunctionCall,
    inferred_types: &HashMap<String, TypeSpec>,
    declared_variables: Option<&HashSet<String>>,
) -> Option<TypeSpec> {
    let argument_type = |position: usize| {
        call.arguments.get(position).and_then(|argument| {
            infer_expression_type(argument, inferred_types, declared_variables)
        })
    };

    match entry.returns {
        ReturnRule::Boolean => Some(TypeSpec::Boolean),
        ReturnRule::Integer => Some(TypeSpec::integer()),
        ReturnRule::Float => Some(TypeSpec::float()),
        ReturnRule::CharacterString => Some(TypeSpec::string()),
        ReturnRule::Date => Some(TypeSpec::Date),
        ReturnRule::ZonedTime => Some(TypeSpec::ZonedTime),
        ReturnRule::LocalTime => Some(TypeSpec::LocalTime),
        ReturnRule::ZonedDateTime => Some(TypeSpec::ZonedDateTime),
        ReturnRule::LocalDateTime => Some(TypeSpec::LocalDateTime),
        ReturnRule::Duration => Some(TypeSpec::duration()),
        ReturnRule::DurationWithQualifier => Some(TypeSpec::Duration {
            qualifier: match call.arguments.get(2) {
                Some(Expression::Literal(Literal::String(qualifier))) => {
                    match qualifier.to_ascii_uppercase().as_str() {
                        "YEAR TO MONTH" => Some(DurationQualifier::YearToMonth),
                        "DAY TO SECOND" => Some(DurationQualifier::DayToSecond),
                        _ => None,
                    }
                }
                _ => None,
            },
        }),
        ReturnRule::AnyList => Some(TypeSpec::list_of(TypeSpec::Any)),
        ReturnRule::ListOfArgument(position) => {
            Some(TypeSpec::list_of(argument_type(position).unwrap_or(TypeSpec::Any)))
        }
        ReturnRule::SameAsArgument(position) => argument_type(position),
        ReturnRule::StringLikeArgument(position) => match argument_type(position) {
            Some(bytes @ TypeSpec::Bytes { .. }) => Some(bytes),
            _ => Some(TypeSpec::string()),
        },
        ReturnRule::Average => match argument_type(0) {
            Some(duration @ TypeSpec::Duration { .. }) => Some(duration),
            _ => Some(TypeSpec::float()),
        },
        ReturnRule::CommonArgumentType => {
            let types = (0..call.arguments.len())
                .map(argument_type)
                .collect::<Option<Vec<_>>>()?;
            let known = types
                .iter()
                .filter(|t| !matches!(t, TypeSpec::Any))
                .collect::<Vec<_>>();
            match known.as_slice() {
                [] => Some(TypeSpec::Any),
                [first, rest @ ..] if rest.iter().all(|t| t == first) => Some((*first).clone()),
                _ if known.iter().all(|t| is_numeric_type(t)) => Some(
                    known
                        .iter()
                        .skip(1)
                        .fold((*known[0]).clone(), |acc, t| promote_numeric_type(&acc, t)),
                ),
                _ => Some(TypeSpec::Any),
            }
        }
        ReturnRule::Trim => {
            let first = argument_type(0);
            if call.arguments.len() == 2
                && let Some(list @ TypeSpec::List { .. }) = first
            {
                return Some(list);
            }
            match call.arguments.last().and_then(|argument| {
                infer_expression_type(argument, inferred_types, declared_variables)
            }) {
                Some(bytes @ TypeSpec::Bytes { .. }) => Some(bytes),
                _ => Some(TypeSpec::string()),
            }
        }
    }
}
