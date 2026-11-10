# GQL Validator - Enforced Rules

This document provides a complete list of the semantic rules enforced by the `gql-rs` AST validator.

## Scope and Dispatch

The validator is executed via `validate_query_ast` and dispatches to:

- Query
- Data
- Next
- Let
- Session
- Transaction
- Catalog

All errors are emitted as `GqlError::Validation` with span information when available.

## Variable Scope

- Declared variables are tracked in a set (`HashSet`).
- `MATCH`/`LET`/`FOR`/`YIELD`/`CALL` update scope.
- Any variable used in an expression must be declared, except special cases with empty scope (for example, standalone
  CALL).

## Statement Rules: Session/Transaction/Catalog

### Session

- `SET` requires at least one set_clause and no reset_clause.
- `RESET` requires at least one reset_clause.
- `CLOSE` cannot have set/reset payload.
- `ACTIVITY` requires at least one clause.

Session `SET`:

- `SCHEMA` must be non-empty.
- `GRAPH` must have a valid graph expression.
- `TIME ZONE` literal must be non-empty.
- `PARAMETER` must have a non-empty parameter specification; initializer, when present, must be non-empty.

Session `RESET`:

- Parameter specification must be non-empty when target is Parameter.

### Transaction

- Only `START` can carry mode.

### Catalog/DDL

- DDL block must be non-empty.
- `CREATE GRAPH`: `OR REPLACE` and `IF NOT EXISTS` cannot coexist.
- `CREATE GRAPH TYPE`: `OR REPLACE` and `IF NOT EXISTS` cannot coexist.
- Schema/object names are validated: required fields must be non-empty.
- Raw DDL text must be non-empty.
- `OF GRAPH TYPE` / `COPY OF` references must be non-empty.
- Nested graph type specification must be non-empty.
- Node/Edge type specification text must be non-empty.
- Property type: name and value_type must be non-empty.

### Graph Expression Clause

- Text must be non-empty.
- CurrentGraph kind must be consistent with `CURRENT_GRAPH`/`CURRENT PROPERTY GRAPH`.
- Other kinds must not include `CURRENT_GRAPH` semantics.
- GraphReference and ObjectNameOrBindingVariable cannot contain structural delimiters (), {}, [].
- GraphReference cannot start with the `VARIABLE` keyword.

## Query Pipeline Rules

### MATCH / Path Pattern

- Path pattern must be non-empty.
- Path variable, when present, must be non-empty.
- Pattern cannot start with an edge.
- Pattern must end with a node.
- Path mode `WALK` is incompatible with the shortest path search.
- Path mode `SIMPLE`/`ACYCLIC` forbids immediate self-loop on the same node id.

Node/Edge/PropertyMap:

- Node/Edge variable, when present, must be non-empty.
- Property key must be non-empty.
- Labels/properties are validated as expressions.

### LET

- `LET` must contain at least one variable definition.
- Variable name must be non-empty.
- Duplicate names in the same `LET` clause are forbidden.

### FILTER/HAVING

- Condition is semantically validated as an expression.
- Condition must be boolean (or Any/BindingTable in supported cases).
- Aggregate functions are forbidden in `WHERE`/`FILTER`.

### FOR

- Iteration variable must be non-empty.
- Ordinality/offset variable, when present, must be non-empty.

### RETURN

- `RETURN` must contain at least one item.
- `RETURN` alias, when present, must be non-empty.
- Duplicate `RETURN` aliases are forbidden.

### GROUP BY

- `GROUP BY ()` cannot have keys.
- Standard `GROUP BY` must have at least one key.
- Key must be non-empty.
- Key must reference a known variable (scope or `RETURN` alias).
- Non-aggregated variables in `RETURN` must appear in `GROUP BY` (or be represented by an already grouped corresponding
  alias).

### ORDER BY

- At least one item is required.
- Expressions are semantically validated.
- Required sortable type: numeric, string-like, temporal, boolean, or Any.

### LIMIT/OFFSET

- Clause cannot be empty (count or offset is required).
- Page value parameters must be non-empty and prefixed with $.

### Subquery in Pipeline

- FromSubquery validates nested query in isolated scope.
- Projected columns from the subquery are exported to outer scope (alias, variable, derivable property name).

## Set Operation Rules

- Both branches must end with `RETURN` (not `FINISH`).
- Arity must be compatible when inferable.
- Alias signature must be compatible when inferable.
- Per-column type compatibility is enforced when inferable.
- Any is considered compatible.
- Compatible families: numeric, string-like, temporal, homogeneous boolean, path-path, record-record.
- Lists are recursively compatible on element type.
- Union is compatible if all members are compatible with the opposite type.

## Data Rules (DML)

### INSERT

- At least one path pattern is required.
- Path pattern must be non-empty.
- Pattern must start and end with a node.

### SET

For `SET` property/label/variable:

- Target variable must be declared (or a `$-prefixed parameter`, when allowed by flow).
- Value/labels are validated as expressions.

Additional type constraints:

- `SET` property cannot assign BindingTable or Graph.
- `SET` variable cannot assign BindingTable.
- `SET` label must receive string-like, list-like, or Any.

### REMOVE

- Target variable must be declared (or a `$ parameter` in allowed contexts).
- `REMOVE` label validates labels as expressions.

### DELETE

- All expressions in delete clause are semantically validated.

## Expression Rules

### Variables and Property Access

- Variable name must be non-empty.
- Unknown variable is not allowed.
- Property access: object must be non-empty and declared; property must be non-empty.

### Unary

- `NOT` requires `BOOLEAN` (with supported compatibility for Any/string-like/BindingTable in specific language cases).
- Unary + and - require numeric (or Any).

### Binary Operators

- `AND`/`OR`/`XOR`: require `BOOLEAN` when types are known; string-like label expressions are supported.
- +: string+string concatenation is forbidden (use ||).
-
    `+ `and `-`: require numeric-numeric or duration-duration, except Any.
- `*`, `/`, `%`, `^`: require numeric-numeric, except Any.
- `||`: requires string-like/path, except Any.
- `<`, `<=`, `>`, `>=`: require comparable operands, except Any.
- `=~/Regex`: requires string-like, except Any.
- `=` and `!=`: allowed (with boolean result inference).

### Functions

Specific rules:

- `ELEMENT_ID`: exactly 1 argument; argument must be a declared variable.
- `LEFT`/`RIGHT`: exactly 2 arguments; second argument must be a non-negative integer.
- `LOWER`/`UPPER`: exactly 1 string-like or Any argument.
- `TRIM`: 1, 2, or 3 arguments.
- `SUM`/`AVG`: exactly 1 numeric or Any argument.
- `SIZE`/`CARDINALITY`: exactly 1 list/string/path/binding table/Any argument.
- `COUNT`: exactly 1 argument.
- `COALESCE`: at least 1 argument; arguments must be mutually compatible.
- `DATE`: at most 1 argument.
- `CURRENT_DATE`/`CURRENT_TIME`/`CURRENT_TIMESTAMP`: no arguments.
- `LOCAL_TIME`/`LOCAL_DATETIME`/`ZONED_TIME`/`ZONED_DATETIME`: at most 1 argument.
- `DURATION_BETWEEN`: exactly 2 temporally compatible arguments (or Any).

### CASE

- `CASE` simple: test expression is validated; each `WHEN` value is validated and compatible with test type when
  inferable.
- `CASE` searched: each condition is validated and boolean.
- `THEN`/`ELSE` are semantically validated.

### CAST

- `CAST` validates source expression.
- Typed LIST cast with dynamic parameters cannot be statically validated: error.
- Source-to-target conversion must be compatible according to is_cast_compatible matrix.

### Array

- Each element is validated.
- Element types must be mutually compatible (numeric promotion/list recursion/Any).

### Map

- Property key must be non-empty.
- Each value is semantically validated.

### PathConstructor

- Must be non-empty.
- Number of elements must be odd (node, edge, node, ...).
- Only variable references are allowed.
- Referenced variables must be declared (when scope is non-empty).

### WHEN Operand (Advanced CASE Operand)

- Comparison: right expression is validated.
- ValueTypePredicate: target_type is mandatory.
- NormalizedPredicate: normal form, when present, must be non-empty and one of {NFC, NFD, NFKC, NFKD}.
- LabeledPredicate: label_expression is mandatory/non-empty.
- Source/DestinationPredicate: edge reference is mandatory/non-empty and declared (when scope is non-empty).

### CALL Clause

- Procedure name must be non-empty.
- Inline `CALL`:
    - cannot define `YIELD`.
    - textual body must be non-empty.
    - structured statement is mandatory.
    - inline argument scope allows only binding variables.
    - duplicate variables in inline scope are forbidden.
- Named `CALL` cannot contain inline payload.
- `CALL` arguments are semantically validated when declared scope exists.
- `YIELD`:
    - at least one item,
    - item name must be non-empty,
    - alias must be non-empty,
    - duplicate output names are forbidden,
    - duplicate aliases are forbidden.

## Inferred Type and Conservative Principle

- When a type cannot be reliably inferred, the validator uses Any and applies conservative rules.
- Deterministic incompatibilities are blocked; ambiguous Any cases are tolerated to avoid false positives.
