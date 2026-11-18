# Built-in Functions

This page lists every built-in function form admitted by the repository grammar, as
recorded in the built-in function registry (`src/ast/validator/builtins.rs`). The
registry is the single source of truth for built-in signature metadata: validation and
return-type inference of built-in calls are dispatched exclusively through it.

## How calls are classified

The AST builder produces every built-in function form as an `Expression::FunctionCall`
whose `name` is the canonical upper-case name listed below. This includes ordinary
functions, aggregates, function-like predicates (`n IS LABELED Person` becomes
`IS_LABELED`) and CASE abbreviations (`COALESCE`, `NULLIF`).

When the validator meets a function call it looks the name up in the registry:

- **Registered name** (unqualified, case-insensitive): the call is a built-in. Its argument
  count, set quantifier, function-specific restrictions, argument types (where known) and
  nested aggregates are checked, and its return type follows the registry rule.
- **Any other name**, including every qualified name such as `schema.floor`: the call is a
  user-defined function. Its arguments are validated as ordinary expressions, no signature is
  enforced, and its return type is unknown.

Type constraints are only enforced when the argument type is known. A call is never
rejected solely because an argument type is unknown (for example a property access or an
untyped variable), and `NULL` is accepted wherever a value is expected.

## Registry

Arity is the number of arguments in the AST node. Keyword modifiers that the AST carries
as string-literal arguments are listed among the arguments (`NORMALIZE`'s normal form,
`DURATION_BETWEEN`'s qualifier, `TRIM`'s trim specification).

| Function | Family | Grammar keyword(s) | Arity | Arguments | Modifiers | Return type | Validation |
|---|---|---|---|---|---|---|---|
| `COUNT` | Aggregate | COUNT | 1 | any | DISTINCT / ALL | INTEGER | Complete |
| `SUM` | Aggregate | SUM | 1 | numeric or duration | DISTINCT / ALL | type of argument 1 | Complete |
| `AVG` | Aggregate | AVG | 1 | numeric or duration | DISTINCT / ALL | FLOAT, or DURATION for durations | Complete |
| `MIN` | Aggregate | MIN | 1 | any | DISTINCT / ALL | type of argument 1 | Partial — comparable-type constraints need the typed semantic context of issue #12 |
| `MAX` | Aggregate | MAX | 1 | any | DISTINCT / ALL | type of argument 1 | Partial — comparable-type constraints need the typed semantic context of issue #12 |
| `COLLECT_LIST` | Aggregate | COLLECT_LIST | 1 | any | DISTINCT / ALL | LIST<type of argument 1> | Complete |
| `STDDEV_SAMP` | Aggregate | STDDEV_SAMP | 1 | numeric | DISTINCT / ALL | FLOAT | Complete |
| `STDDEV_POP` | Aggregate | STDDEV_POP | 1 | numeric | DISTINCT / ALL | FLOAT | Complete |
| `PERCENTILE_CONT` | Aggregate | PERCENTILE_CONT | 2 | numeric, numeric in [0, 1] | DISTINCT / ALL | FLOAT | Complete |
| `PERCENTILE_DISC` | Aggregate | PERCENTILE_DISC | 2 | numeric, numeric in [0, 1] | DISTINCT / ALL | type of argument 1 | Complete |
| `ABS` | Numeric | ABS | 1 | numeric or duration | — | type of argument 1 | Complete |
| `MOD` | Numeric | MOD | 2 | numeric, numeric | — | common argument type | Complete |
| `FLOOR` | Numeric | FLOOR | 1 | numeric | — | type of argument 1 | Complete |
| `CEIL` | Numeric | CEIL / CEILING | 1 | numeric | — | type of argument 1 | Complete |
| `SQRT` | Numeric | SQRT | 1 | numeric | — | FLOAT | Complete |
| `POWER` | Numeric | POWER | 2 | numeric, numeric | — | FLOAT | Complete |
| `EXP` | Numeric | EXP | 1 | numeric | — | FLOAT | Complete |
| `LN` | Numeric | LN | 1 | numeric | — | FLOAT | Complete |
| `LOG10` | Numeric | LOG10 | 1 | numeric | — | FLOAT | Complete |
| `LOG` | Numeric | LOG_KW | 2 | numeric, numeric | — | FLOAT | Complete |
| `SIN` | Numeric | SIN | 1 | numeric | — | FLOAT | Complete |
| `COS` | Numeric | COS | 1 | numeric | — | FLOAT | Complete |
| `TAN` | Numeric | TAN | 1 | numeric | — | FLOAT | Complete |
| `COT` | Numeric | COT | 1 | numeric | — | FLOAT | Complete |
| `SINH` | Numeric | SINH | 1 | numeric | — | FLOAT | Complete |
| `COSH` | Numeric | COSH | 1 | numeric | — | FLOAT | Complete |
| `TANH` | Numeric | TANH | 1 | numeric | — | FLOAT | Complete |
| `ASIN` | Numeric | ASIN | 1 | numeric | — | FLOAT | Complete |
| `ACOS` | Numeric | ACOS | 1 | numeric | — | FLOAT | Complete |
| `ATAN` | Numeric | ATAN | 1 | numeric | — | FLOAT | Complete |
| `DEGREES` | Numeric | DEGREES | 1 | numeric | — | FLOAT | Complete |
| `RADIANS` | Numeric | RADIANS | 1 | numeric | — | FLOAT | Complete |
| `CHAR_LENGTH` | Character string | CHAR_LENGTH | 1 | string | — | INTEGER | Complete |
| `CHARACTER_LENGTH` | Character string | CHARACTER_LENGTH | 1 | string | — | INTEGER | Complete |
| `UPPER` | Character string | UPPER | 1 | string | — | STRING | Complete |
| `LOWER` | Character string | LOWER | 1 | string | — | STRING | Complete |
| `LTRIM` | Character string | LTRIM | 1–2 | each: string | — | STRING | Complete |
| `RTRIM` | Character string | RTRIM | 1–2 | each: string | — | STRING | Complete |
| `BTRIM` | Character string | BTRIM | 1–2 | each: string | — | STRING | Complete |
| `NORMALIZE` | Character string | NORMALIZE | 1–2 | string, NFC/NFD/NFKC/NFKD | — | STRING | Complete |
| `BYTE_LENGTH` | Byte string | BYTE_LENGTH | 1 | byte string | — | INTEGER | Partial — byte string values have no literal representation until issue #10 |
| `OCTET_LENGTH` | Byte string | OCTET_LENGTH | 1 | byte string | — | INTEGER | Partial — byte string values have no literal representation until issue #10 |
| `LEFT` | Character or byte string | LEFT | 2 | string or byte string, non-negative integer | — | STRING or BYTES (as argument 1) | Complete |
| `RIGHT` | Character or byte string | RIGHT | 2 | string or byte string, non-negative integer | — | STRING or BYTES (as argument 1) | Complete |
| `TRIM` | Character or byte string | TRIM | 1–3 | string form: [spec] [char] source; list form: list, non-negative integer | — | STRING/BYTES, or the list type | Partial — TRIM(spec FROM source) and TRIM('x' FROM source) share one argument shape until the expression redesign of 0.1.0 |
| `SIZE` | List | SIZE | 1 | list, string, path or binding table | — | INTEGER | Complete |
| `CARDINALITY` | List | CARDINALITY | 1 | list, string, path or binding table | — | INTEGER | Complete |
| `ELEMENTS` | Path | ELEMENTS | 1 | path | — | LIST<ANY> | Partial — path variables are not typed until the semantic context of issue #12 |
| `PATH_LENGTH` | Path | PATH_LENGTH | 1 | path | — | INTEGER | Partial — path variables are not typed until the semantic context of issue #12 |
| `CURRENT_DATE` | Temporal | CURRENT_DATE | 0 | — | — | DATE | Complete |
| `DATE` | Temporal | DATE | 0–1 | string or record | — | DATE | Partial — temporal string contents and record fields need the literal representations of issue #10 |
| `CURRENT_TIME` | Temporal | CURRENT_TIME | 0 | — | — | ZONED TIME | Complete |
| `ZONED_TIME` | Temporal | ZONED_TIME | 0–1 | string or record | — | ZONED TIME | Partial — temporal string contents and record fields need the literal representations of issue #10 |
| `LOCAL_TIME` | Temporal | LOCAL_TIME | 0–1 | string or record | — | LOCAL TIME | Partial — temporal string contents and record fields need the literal representations of issue #10 |
| `CURRENT_TIMESTAMP` | Temporal | CURRENT_TIMESTAMP | 0 | — | — | ZONED DATETIME | Complete |
| `ZONED_DATETIME` | Temporal | ZONED_DATETIME | 0–1 | string or record | — | ZONED DATETIME | Partial — temporal string contents and record fields need the literal representations of issue #10 |
| `LOCAL_TIMESTAMP` | Temporal | LOCAL_TIMESTAMP | 0 | — | — | LOCAL DATETIME | Complete |
| `LOCAL_DATETIME` | Temporal | LOCAL_DATETIME | 0–1 | string or record | — | LOCAL DATETIME | Partial — temporal string contents and record fields need the literal representations of issue #10 |
| `DURATION` | Duration | DURATION | 1 | string or record | — | DURATION | Partial — temporal string contents and record fields need the literal representations of issue #10 |
| `DURATION_BETWEEN` | Duration | DURATION_BETWEEN | 2–3 | date/time/datetime, date/time/datetime, YEAR TO MONTH / DAY TO SECOND | — | DURATION (with qualifier) | Complete |
| `ELEMENT_ID` | Graph element / predicate | ELEMENT_ID | 1 | element variable | — | STRING | Complete |
| `PROPERTY_EXISTS` | Graph element / predicate | PROPERTY_EXISTS | 2 | element variable, property name | — | BOOLEAN | Complete |
| `IS_LABELED` | Graph element / predicate | LABELED / COLON | 2 | element variable, label expression | — | BOOLEAN | Complete |
| `IS_DIRECTED` | Graph element / predicate | DIRECTED | 1 | element variable | — | BOOLEAN | Complete |
| `IS_SOURCE_OF` | Graph element / predicate | SOURCE | 2 | element variable, element variable | — | BOOLEAN | Complete |
| `IS_DESTINATION_OF` | Graph element / predicate | DESTINATION | 2 | element variable, element variable | — | BOOLEAN | Complete |
| `ALL_DIFFERENT` | Graph element / predicate | ALL_DIFFERENT | ≥ 2 | each: element variable | — | BOOLEAN | Complete |
| `SAME` | Graph element / predicate | SAME | ≥ 2 | each: element variable | — | BOOLEAN | Complete |
| `IS_NORMALIZED` | Character string | NORMALIZED | 1–2 | string, NFC/NFD/NFKC/NFKD | — | BOOLEAN | Complete |
| `COALESCE` | CASE abbreviation | COALESCE | ≥ 2 | each: any | — | common argument type | Partial — comparable-type constraints need the typed semantic context of issue #12 |
| `NULLIF` | CASE abbreviation | NULLIF | 2 | each: any | — | type of argument 1 | Partial — comparable-type constraints need the typed semantic context of issue #12 |
| `SESSION_USER` | Session | SESSION_USER | 0 | — | — | STRING | Complete |

## Validation limitations by signature family

Registry coverage does not imply complete GQL function semantics. The following checks are
not possible with the 0.0.4 models and are deferred to 0.1.0:

- **Aggregates `MIN`, `MAX` and CASE abbreviations `COALESCE`, `NULLIF`**: comparable-type constraints need the typed semantic context of issue #12. Arguments of known, incompatible
  types are rejected for `COALESCE`; otherwise any value is accepted.
- **Byte string functions `BYTE_LENGTH`, `OCTET_LENGTH`**: byte string values have no literal representation until issue #10. The byte string constraint is only
  checked against values typed as `BYTES`, so in practice arguments are usually unknown.
- **`TRIM`**: TRIM(spec FROM source) and TRIM('x' FROM source) share one argument shape until the expression redesign of 0.1.0. The list form `TRIM(list, count)` is
  recognized only when the first argument is known to be a list.
- **Path functions `ELEMENTS`, `PATH_LENGTH`**: path variables are not typed until the semantic context of issue #12. Arguments of a known, non-path type are rejected.
- **Temporal constructors `DATE`, `ZONED_TIME`, `LOCAL_TIME`, `ZONED_DATETIME`,
  `LOCAL_DATETIME` and `DURATION`**: temporal string contents and record fields need the literal representations of issue #10. Only the argument shape (string or record) is
  checked.

Further limitations that apply to every family:

- Precise nullability, reference, record and collection-element typing is not modeled
  (issues #10 and #12).
- Declared or inferred variable types are not propagated across query steps (issue #12),
  so most variable and property arguments have an unknown type.
- `FunctionCall` does not record whether a call was written as a built-in form or as a
  user-defined function call. An unqualified user-defined function whose name equals a
  registered name (for example `"IS_DIRECTED"(e)`) is validated as the built-in. Recording
  the origin requires a new AST field and is deferred to 0.1.0.
- `COUNT(*)` keeps its established representation: a single `Literal::String("*")`
  argument.
- `COLLECT_SET` is not part of the grammar and is not registered; a call to it is a
  user-defined function and is no longer treated as an aggregate.

## Expressions that fail explicitly

Since issue #9 the builder builds every expression structurally from its CST. Syntactically
valid expressions that the current AST cannot represent faithfully fail with a
`GqlError::Parse` whose context code is `GqlError::UNSUPPORTED_CONSTRUCT`, instead of
being turned into strings or reconstructed from source text:

| Expression | Example | Tracked by |
|---|---|---|
| Byte string literals | `X'00FF'` | issue #10 |
| Property reference on a non-variable expression | `n.address.city`, `$p.name`, `(1 + 2).x` | 0.1.0 expression redesign |
| Catalog, delimited or home graph references | `GRAPH cat.g`, ``GRAPH `g` ``, `GRAPH HOME_GRAPH` | issue #15 |
| Current graph | `GRAPH CURRENT_GRAPH` | issue #15 |
| Catalog or delimited binding table references | `TABLE /t`, ``TABLE `t` `` | issue #15 |

Related representations chosen with the existing AST:

- `DATE '...'`, `TIME '...'`, `DATETIME '...'` and `TIMESTAMP '...'` literals become
  `Literal::DateTime` with the decoded string; `DURATION '...'` becomes
  `Literal::Duration`. The specific temporal kind is not preserved (issue #10).
- `UNKNOWN` becomes `Literal::Null`, the null value of the boolean type.
- `SESSION_USER` becomes a niladic `SESSION_USER` built-in call.
- `GRAPH $$g` and `TABLE $$t` become `Literal::Parameter`.
- `GRAPH VARIABLE g`, `GRAPH g`, `TABLE t` become the corresponding variable or expression.
