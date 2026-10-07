# Updating the GQL grammar and generated Rust parser

This guide describes how to change `resources/GQL.g4` and regenerate the Rust
lexer, parser, listener, and visitor used by `gql-rs`.

## Source and generated files

The grammar is the source of truth:

- `resources/GQL.g4`

ANTLR generates these files:

- `src/antlr/gqllexer.rs`
- `src/antlr/gqlparser.rs`
- `src/antlr/gqllistener.rs`
- `src/antlr/gqlvisitor.rs`
- `src/antlr/utility/GQL.interp`
- `src/antlr/utility/GQL.tokens`
- `src/antlr/utility/GQLLexer.interp`
- `src/antlr/utility/GQLLexer.tokens`

`src/antlr/mod.rs` is maintained by hand. Do not delete or overwrite it during
generation.

Generated files must not be edited manually. A later regeneration would discard
such changes, and the grammar would no longer explain the parser's behaviour.

## Compatible generator

The crate currently uses `antlr-rust = "0.3.0-beta"`. Its generated sources are
compatible with the Rust-target ANTLR 4.8 generator historically named:

```text
antlr4-4.8-2-SNAPSHOT-complete.jar
```

This is the Rust-target generator distributed by the `antlr4rust` project, not
an unmodified ANTLR Java-target release. Do not replace it with a newer standard
ANTLR JAR without also planning and testing a runtime/code-generator migration.

The exact generator used by this project is already stored at:

```text
resources/antlr4-4.8-2-SNAPSHOT-complete.jar
```

Java must also be available:

```sh
java -version
```

## Grammar organization

`GQL.g4` is a combined grammar:

- lower-camel-case rules are parser rules;
- upper-case rules are lexer rules;
- numbered comments correspond to sections of the GQL specification;
- semantic checks belong in `src/ast/validator`, not in lexer or parser rules;
- CST-to-AST mapping belongs in `src/ast/builder`.

Keywords are case-insensitive because their lexer rules are composed from the
`A` through `Z` fragments. The ANTLR 4.8 Rust generator does not support the
newer `caseInsensitive` grammar option.

## Update workflow

1. Change `resources/GQL.g4`.
2. Identify every builder branch that consumes the changed parser context.
3. Regenerate the ANTLR artifacts.
4. Review all generated differences before changing builder code.
5. Update the CST-to-AST builder and semantic validator where necessary.
6. Add or update focused tests for the syntax and resulting AST.
7. Run the complete test suite.

Keep grammar changes separate from unrelated generated-code or AST refactors
whenever possible. This makes generated differences easier to audit.

## Generate the Rust sources automatically

From the repository root, run:

```sh
./scripts/generate-parser.sh
```

The script:

1. checks that Java and the bundled JAR are available;
2. generates every artifact in a temporary directory;
3. verifies that all eight expected files were produced;
4. replaces the generated Rust and utility files only after successful
   generation;
5. leaves `src/antlr/mod.rs` untouched.

This prevents a failed or partial ANTLR run from leaving `src/antlr` in a mixed
state.

## Manual generator invocation for diagnosis

Run the generator from `resources`. Running it from this directory keeps ANTLR
from reproducing the input directory inside the output directory.

From the repository root, the following command reproduces the generator invocation in a
temporary output directory:

```sh
cd resources
temporary_dir=$(mktemp -d)
java -jar antlr4-4.8-2-SNAPSHOT-complete.jar \
  -Dlanguage=Rust \
  -listener \
  -visitor \
  -o "$temporary_dir" \
  GQL.g4
cd ..
```

The generator emits four Rust files and these utility files:

```text
GQL.interp
GQL.tokens
GQLLexer.interp
GQLLexer.tokens
```

Use this form to inspect raw generator output only. The automatic script also installs the
files in the correct directories and prepends the required EUPL/SPDX header to generated
Rust sources. Do not replace repository files manually or regenerate `src/antlr/mod.rs`.

## Review the generated changes

Generation can produce a large diff. Confirm that it contains only the expected
parser, lexer, listener, visitor, and utility changes:

```sh
git status --short
git diff --stat
git diff -- resources/GQL.g4 src/antlr
```

Pay particular attention to:

- changed rule-context names used by `src/ast/builder`;
- new or removed token constants;
- changes to visitor and listener methods;
- unexpected generator-version changes in generated file headers;
- generated files appearing in an unintended nested directory.

If unrelated generated output changes, stop and verify the JAR version and the
generation command before updating application code.

## Verify the parser

First check that the crate compiles, then run the tests:

```sh
cargo check
cargo test
```

When a grammar rule changes, successful compilation is not sufficient. The AST
builder may still accept the new CST while losing or misclassifying information.
Add assertions for the complete affected AST structure before considering the
grammar update complete.

## Troubleshooting

### `error(31): ANTLR cannot generate Rust code`

The JAR does not contain the Rust target. Use the approved Rust-target ANTLR JAR
rather than the standard ANTLR distribution.

### Output is generated under `src/antlr/resources`

Run the command from inside `resources` as shown above, with `GQL.g4` as the
input path rather than `resources/GQL.g4`.

### Rust compilation fails after regeneration

Verify that the generator matches `antlr-rust = "0.3.0-beta"`. Then inspect
changed context types and update the corresponding code in `src/ast/builder`.

### Parsing succeeds but the AST is wrong

The grammar controls the CST, not the final AST. Inspect the generated context
and update the relevant builder module. If the issue requires scope or type
knowledge, implement the check in `src/ast/validator`.
