# Contributing to gql-rs

Thank you for your interest in contributing to `gql-rs`.

This repository contains the GQL grammar, the generated ANTLR lexer and parser, the typed AST, semantic validation, and the portable GQL Technology Compatibility Kit.

## Development requirements

The project requires:

- a Rust toolchain compatible with the edition declared in `Cargo.toml`;
- Java, when regenerating the ANTLR lexer and parser;
- the ANTLR JAR included under `resources/`.

Optional quality tools may include:

- `cargo-nextest`;
- `cargo-llvm-cov`;
- SonarScanner.

Use the dependency versions declared by the repository. Do not update dependencies as part of an unrelated change.

## Repository structure

The main directories are:

```text
resources/                 GQL grammar and ANTLR generator
scripts/                   parser generation scripts
src/antlr/                 generated lexer and parser
src/ast/model/             typed AST structures
src/ast/builder/           CST-to-AST construction
src/ast/validator/         semantic validation
tests/unit/                isolated unit tests
tests/integration/         parser and AST integration tests
tests/integration/tck/     portable GQL compatibility scenarios
docs/                      project documentation
```

## Making changes

Before starting:

1. Create a branch from the current development branch.
2. Keep the change focused on a single purpose.
3. Avoid unrelated refactoring and bulk formatting.
4. Add or update tests for every behavioural change.
5. Update documentation when syntax, AST structures, validation rules, or development procedures change.

Do not run automatic formatting across the entire repository.

## Testing

Run unit tests with:

```bash
cargo test --test unit
```

Run integration tests with:

```bash
cargo test --test integration
```

Run the portable GQL TCK with:

```bash
cargo test --test tck
```

Run the complete Rust test suite with:

```bash
cargo test
```

All relevant suites must pass before opening a merge request.

## Test organisation

Use the appropriate test level:

- **Unit tests** verify individual AST structures, errors, validators, and isolated behaviour.
- **Integration tests** verify interactions between the parser, AST builder, and validator.
- **TCK scenarios** describe portable GQL behaviour independently from the Rust implementation.

AST tests must verify the complete relevant structure. Do not use assertions that only confirm that parsing returned a value.

TCK scenarios should use meaningful names and should represent a distinct language behaviour. Avoid duplicate scenarios that produce the same relevant syntax and AST contract without adding coverage.

Add both positive and negative cases when a grammar or validation change can succeed and fail in meaningfully different ways.

## Changing the GQL grammar

The canonical grammar is:

```text
resources/GQL.g4
```

After changing it, regenerate the Rust lexer and parser with:

```bash
./scripts/generate-parser.sh
```

Do not manually edit generated files under `src/antlr/`.

A grammar change must include:

- the updated `resources/GQL.g4`;
- all regenerated Rust sources;
- relevant integration or TCK tests;
- AST builder updates when the parse tree changes;
- validator updates when semantic rules change;
- documentation updates when supported syntax changes.

Review generated changes carefully to ensure they were produced only by the intended grammar modification.

## TCK contributions

Portable language scenarios belong under:

```text
tests/integration/tck/features/
```

A TCK scenario should:

- describe observable GQL behaviour;
- avoid implementation-specific Rust details;
- use a concise and descriptive scenario name;
- contain the smallest query that demonstrates the behaviour;
- specify whether analysis should succeed or fail;
- avoid depending on a running NturDB instance.

Implementation-specific AST assertions belong in integration fixtures rather than Gherkin scenarios.

## Documentation

When changing public behaviour, review:

- `README.md`;
- `CHANGELOG.md`;
- files under `docs/`;
- grammar generation instructions;
- test execution instructions;
- examples affected by the change.

Documentation must describe the current implementation and use executable examples where possible.

## Commit messages

Use concise Conventional Commit messages:

```text
feat: add support for ...
fix: correct ...
test: cover ...
docs: document ...
refactor: simplify ...
build: update ...
chore: maintain ...
```

Examples:

```text
feat: add path pattern AST support
fix: handle mixed-case reserved words
test: cover graph selection clauses
docs: document parser regeneration
```

Keep commits focused and do not combine unrelated changes.

## Merge requests

A merge request should include:

- a clear description of the change;
- the reason for the change;
- the affected grammar or AST areas;
- the tests that were executed;
- compatibility considerations;
- related issues, when available.

Clearly identify changes to generated ANTLR files.

Do not include build artifacts, IDE files, credentials, local environment files, or unrelated formatting changes.

## Licence headers

New Rust, grammar, test, feature, script, and configuration files must include the repository licence header using the appropriate comment syntax.

Use `//` for Rust and ANTLR grammar files. Use `#` for shell scripts, TOML, YAML, properties, Git ignore files, and Gherkin feature files.

The required header is:

```text
Copyright (c) 2026 NTUR LAB S.r.l.

This file is part of NTURDB.

Licensed under the EUPL, Version 1.2 or - as soon they will be approved by
the European Commission - subsequent versions of the EUPL (the "Licence");
You may not use this work except in compliance with the Licence.
You may obtain a copy of the Licence at:

https://joinup.ec.europa.eu/software/page/eupl

Unless required by applicable law or agreed to in writing, software
distributed under the Licence is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the Licence for the specific language governing permissions and
limitations under the Licence.
```

## Contribution licence

By submitting a contribution, you agree that it may be distributed under the licence declared by this repository.

See `LICENSE` for the complete licensing terms.