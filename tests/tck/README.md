# GQL Technology Compatibility Kit

The feature files in `features/` describe public GQL behaviour independently from the
Rust AST representation. They are grouped by language capability rather than by their
source or implementation history.

The portable step vocabulary is intentionally small:

```gherkin
When the GQL query is analyzed:
  """
  RETURN 1
  """
Then analysis succeeds
```

Negative scenarios use `Then analysis fails`. Scenarios tagged `@pending` represent valid
syntax that the current analyzer does not yet support completely and are ignored by default.

Run the TCK from the repository root:

```text
cargo test --test gql_tck
```

The Rust adapter uses the Cucumber runtime and is split into:

- `main.rs`: runner configuration and `@pending` filtering;
- `world.rs`: isolated state for each scenario;
- `steps.rs`: typed `When` and `Then` step definitions.

AST-specific expectations do not belong in the TCK; they are stored as golden fixtures
under `tests/fixtures/ast`.
