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

#[cfg(test)]
mod value_type_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError, ast::model::*};

    fn analyze(query: &str) -> Result<QueryAST, GqlError> {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer.exec(query)
    }

    fn return_expressions(query: &str) -> Vec<Expression> {
        let ast = analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        let Statement::Query(Query::Basic(basic)) = &ast.statement else {
            panic!("expected a basic query for: {query}");
        };
        let PrimitiveResultStatement::Return { return_clause, .. } = &basic.result_statement else {
            panic!("expected RETURN for: {query}");
        };
        return_clause
            .items()
            .unwrap_or_else(|| panic!("expected explicit RETURN items for: {query}"))
            .iter()
            .map(|item| item.expression.clone())
            .collect()
    }

    /// Returns the declared target type of `CAST(n.v AS <type_text>)`.
    fn cast_type(type_text: &str) -> ValueType {
        let query = format!("MATCH (n) RETURN CAST(n.v AS {type_text})");
        match return_expressions(&query).remove(0) {
            Expression::Cast(cast) => cast.target_type,
            other => panic!("expected CAST for {type_text}, got {other:?}"),
        }
    }

    fn base(type_text: &str) -> TypeSpec {
        let value_type = cast_type(type_text);
        assert!(!value_type.not_null, "{type_text} must be nullable");
        value_type.base
    }

    fn literal(query: &str) -> Literal {
        match return_expressions(query).remove(0) {
            Expression::Literal(literal) => literal,
            other => panic!("expected literal for {query}, got {other:?}"),
        }
    }

    fn assert_invalid_type_parameter(query: &str, message: &str) {
        let error = analyze(query).expect_err(&format!("query must fail: {query}"));
        assert!(
            matches!(error, GqlError::Parse { .. }),
            "query: {query}, error: {error}"
        );
        assert_eq!(
            error.context().code.as_deref(),
            Some(GqlError::INVALID_TYPE_PARAMETER),
            "query: {query}, error: {error}"
        );
        assert!(error.message().contains(message), "query: {query}, error: {error}");
        assert!(error.context().span.is_some(), "query: {query}");
    }

    fn integer(signed: bool, size: IntegerSize) -> TypeSpec {
        TypeSpec::Integer { signed, size }
    }

    #[test]
    fn not_null_is_preserved_independently_from_the_base_type() {
        assert_eq!(cast_type("INT"), ValueType::nullable(TypeSpec::integer()));
        assert_eq!(cast_type("INT NOT NULL"), ValueType::new(TypeSpec::integer(), true));
        for type_text in [
            "BOOLEAN NOT NULL",
            "STRING(5) NOT NULL",
            "BYTES NOT NULL",
            "DECIMAL(5, 2) NOT NULL",
            "FLOAT32 NOT NULL",
            "UNSIGNED INTEGER NOT NULL",
            "DATE NOT NULL",
            "DURATION(DAY TO SECOND) NOT NULL",
            "PATH NOT NULL",
            "RECORD NOT NULL",
            "ANY NODE NOT NULL",
            "ANY VALUE NOT NULL",
            "PROPERTY VALUE NOT NULL",
            "LIST<INT> NOT NULL",
        ] {
            assert!(cast_type(type_text).not_null, "{type_text}");
        }
    }

    #[test]
    fn list_elements_carry_their_own_nullability() {
        let value_type = cast_type("LIST<INT NOT NULL>");
        assert!(!value_type.not_null);
        let TypeSpec::List {
            element_type,
            max_length,
        } = value_type.base
        else {
            panic!("expected LIST");
        };
        assert_eq!(*element_type, ValueType::new(TypeSpec::integer(), true));
        assert_eq!(max_length, None);

        let value_type = cast_type("STRING LIST[3] NOT NULL");
        assert!(value_type.not_null);
        assert_eq!(
            value_type.base,
            TypeSpec::List {
                element_type: Box::new(ValueType::nullable(TypeSpec::string())),
                max_length: Some(3),
            }
        );
        assert_eq!(base("ARRAY<BOOL>"), TypeSpec::list_of(TypeSpec::Boolean));
        assert_eq!(base("LIST"), TypeSpec::list_of(TypeSpec::Any));
    }

    #[test]
    fn every_signed_integer_form_is_represented() {
        use IntegerSize::*;
        for (type_text, size) in [
            ("INT8", Bits8),
            ("INT16", Bits16),
            ("INT32", Bits32),
            ("INT64", Bits64),
            ("INT128", Bits128),
            ("INT256", Bits256),
            ("SMALLINT", Small),
            ("INT", Regular { precision: None }),
            ("INT(16)", Regular { precision: Some(16) }),
            ("BIGINT", Big),
            ("INTEGER8", Bits8),
            ("INTEGER16", Bits16),
            ("INTEGER32", Bits32),
            ("INTEGER64", Bits64),
            ("INTEGER128", Bits128),
            ("INTEGER256", Bits256),
            ("SMALL INTEGER", Small),
            ("INTEGER", Regular { precision: None }),
            ("INTEGER(32)", Regular { precision: Some(32) }),
            ("BIG INTEGER", Big),
            ("SIGNED INTEGER8", Bits8),
            ("SIGNED BIG INTEGER", Big),
            ("SIGNED INTEGER(8)", Regular { precision: Some(8) }),
        ] {
            assert_eq!(base(type_text), integer(true, size), "{type_text}");
        }
    }

    #[test]
    fn every_unsigned_integer_form_is_represented() {
        use IntegerSize::*;
        for (type_text, size) in [
            ("UINT8", Bits8),
            ("UINT16", Bits16),
            ("UINT32", Bits32),
            ("UINT64", Bits64),
            ("UINT128", Bits128),
            ("UINT256", Bits256),
            ("USMALLINT", Small),
            ("UINT", Regular { precision: None }),
            ("UINT(8)", Regular { precision: Some(8) }),
            ("UBIGINT", Big),
            ("UNSIGNED INTEGER8", Bits8),
            ("UNSIGNED INTEGER256", Bits256),
            ("UNSIGNED SMALL INTEGER", Small),
            ("UNSIGNED INTEGER", Regular { precision: None }),
            ("UNSIGNED BIG INTEGER", Big),
        ] {
            assert_eq!(base(type_text), integer(false, size), "{type_text}");
        }
    }

    #[test]
    fn approximate_and_decimal_numeric_types_keep_size_precision_and_scale() {
        for (type_text, size) in [
            ("FLOAT16", FloatSize::Bits16),
            ("FLOAT32", FloatSize::Bits32),
            ("FLOAT64", FloatSize::Bits64),
            ("FLOAT128", FloatSize::Bits128),
            ("FLOAT256", FloatSize::Bits256),
            (
                "FLOAT",
                FloatSize::Regular {
                    precision: None,
                    scale: None,
                },
            ),
            (
                "FLOAT(24)",
                FloatSize::Regular {
                    precision: Some(24),
                    scale: None,
                },
            ),
            (
                "FLOAT(10, 2)",
                FloatSize::Regular {
                    precision: Some(10),
                    scale: Some(2),
                },
            ),
            ("REAL", FloatSize::Real),
            ("DOUBLE", FloatSize::Double),
            ("DOUBLE PRECISION", FloatSize::Double),
        ] {
            assert_eq!(base(type_text), TypeSpec::Float { size }, "{type_text}");
        }
        for (type_text, precision, scale) in [
            ("DECIMAL", None, None),
            ("DECIMAL(10)", Some(10), None),
            ("DEC(10, 0)", Some(10), Some(0)),
            ("DECIMAL(38, 38)", Some(38), Some(38)),
        ] {
            assert_eq!(base(type_text), TypeSpec::Decimal { precision, scale }, "{type_text}");
        }
    }

    #[test]
    fn character_and_byte_string_lengths_are_preserved() {
        let string = |min_length, max_length| TypeSpec::String { min_length, max_length };
        let bytes = |min_length, max_length| TypeSpec::Bytes { min_length, max_length };
        for (type_text, expected) in [
            ("STRING", string(None, None)),
            ("STRING(10)", string(None, Some(10))),
            ("STRING(2, 10)", string(Some(2), Some(10))),
            ("STRING(0, 1)", string(Some(0), Some(1))),
            ("STRING(0x10)", string(None, Some(16))),
            ("CHAR", string(None, None)),
            ("CHAR(5)", string(Some(5), Some(5))),
            ("VARCHAR(5)", string(None, Some(5))),
            ("BYTES", bytes(None, None)),
            ("BYTES(4)", bytes(None, Some(4))),
            ("BYTES(1, 4)", bytes(Some(1), Some(4))),
            ("BINARY(4)", bytes(Some(4), Some(4))),
            ("VARBINARY(8)", bytes(None, Some(8))),
        ] {
            assert_eq!(base(type_text), expected, "{type_text}");
        }
    }

    #[test]
    fn temporal_and_duration_types_are_represented() {
        for (type_text, expected) in [
            ("DATE", TypeSpec::Date),
            ("ZONED DATETIME", TypeSpec::ZonedDateTime),
            ("TIMESTAMP WITH TIME ZONE", TypeSpec::ZonedDateTime),
            ("LOCAL DATETIME", TypeSpec::LocalDateTime),
            ("TIMESTAMP", TypeSpec::LocalDateTime),
            ("TIMESTAMP WITHOUT TIME ZONE", TypeSpec::LocalDateTime),
            ("ZONED TIME", TypeSpec::ZonedTime),
            ("TIME WITH TIME ZONE", TypeSpec::ZonedTime),
            ("LOCAL TIME", TypeSpec::LocalTime),
            ("TIME WITHOUT TIME ZONE", TypeSpec::LocalTime),
            (
                "DURATION(YEAR TO MONTH)",
                TypeSpec::Duration {
                    qualifier: Some(DurationQualifier::YearToMonth),
                },
            ),
            (
                "DURATION(DAY TO SECOND)",
                TypeSpec::Duration {
                    qualifier: Some(DurationQualifier::DayToSecond),
                },
            ),
        ] {
            assert_eq!(base(type_text), expected, "{type_text}");
        }
    }

    #[test]
    fn open_reference_types_are_distinguished_from_closed_ones() {
        for type_text in ["NODE", "ANY NODE", "VERTEX", "ANY VERTEX"] {
            assert_eq!(base(type_text), TypeSpec::Node { node_type: None }, "{type_text}");
        }
        for type_text in ["EDGE", "ANY EDGE", "RELATIONSHIP"] {
            assert_eq!(base(type_text), TypeSpec::Edge { edge_type: None }, "{type_text}");
        }
        for type_text in ["ANY GRAPH", "ANY PROPERTY GRAPH"] {
            assert_eq!(
                base(type_text),
                TypeSpec::Graph { graph_type_spec: None },
                "{type_text}"
            );
        }

        let TypeSpec::Node {
            node_type: Some(node_type),
        } = base("NODE TYPE Person")
        else {
            panic!("expected closed node reference type");
        };
        assert_eq!(node_type.identifier.as_deref(), Some("Person"));

        let TypeSpec::Edge {
            edge_type: Some(edge_type),
        } = base("DIRECTED EDGE TYPE Knows CONNECTING (Person TO Person)")
        else {
            panic!("expected closed edge reference type");
        };
        assert_eq!(edge_type.identifier.as_deref(), Some("Knows"));

        let TypeSpec::Graph {
            graph_type_spec: Some(_),
        } = base("PROPERTY GRAPH { NODE TYPE Person }")
        else {
            panic!("expected closed graph reference type");
        };
    }

    #[test]
    fn record_and_binding_table_fields_are_structural() {
        assert_eq!(base("RECORD"), TypeSpec::Record { fields: None });
        assert_eq!(base("ANY RECORD"), TypeSpec::Record { fields: None });

        let expected_fields = vec![
            FieldType {
                name: "a".to_string(),
                value_type: ValueType::nullable(TypeSpec::integer()),
            },
            FieldType {
                name: "b".to_string(),
                value_type: ValueType::new(
                    TypeSpec::Record {
                        fields: Some(vec![FieldType {
                            name: "c".to_string(),
                            value_type: ValueType::nullable(TypeSpec::Date),
                        }]),
                    },
                    true,
                ),
            },
        ];
        for type_text in [
            "RECORD { a INT, b { c DATE } NOT NULL }",
            "{ a :: INT, b TYPED RECORD { c DATE } NOT NULL }",
        ] {
            assert_eq!(
                base(type_text),
                TypeSpec::Record {
                    fields: Some(expected_fields.clone())
                },
                "{type_text}"
            );
        }
        assert_eq!(base("RECORD {}"), TypeSpec::Record { fields: Some(vec![]) });

        let value_type = cast_type("BINDING TABLE { a INT, b STRING NOT NULL } NOT NULL");
        assert!(value_type.not_null);
        assert_eq!(
            value_type.base,
            TypeSpec::BindingTable {
                fields: Some(vec![
                    FieldType {
                        name: "a".to_string(),
                        value_type: ValueType::nullable(TypeSpec::integer()),
                    },
                    FieldType {
                        name: "b".to_string(),
                        value_type: ValueType::new(TypeSpec::string(), true),
                    },
                ]),
            }
        );
        assert_eq!(base("TABLE {}"), TypeSpec::BindingTable { fields: Some(vec![]) });
    }

    #[test]
    fn immaterial_dynamic_and_union_types_are_distinguished() {
        assert_eq!(cast_type("NULL"), ValueType::nullable(TypeSpec::Null));
        assert_eq!(cast_type("NOTHING"), ValueType::new(TypeSpec::Nothing, true));
        assert_eq!(cast_type("NULL NOT NULL"), ValueType::new(TypeSpec::Nothing, true));
        assert_eq!(base("ANY"), TypeSpec::Any);
        assert_eq!(base("ANY VALUE"), TypeSpec::Any);
        assert_eq!(base("PROPERTY VALUE"), TypeSpec::PropertyValue);
        assert_eq!(base("ANY PROPERTY VALUE"), TypeSpec::PropertyValue);

        let union = TypeSpec::Union {
            members: vec![TypeSpec::integer(), TypeSpec::string(), TypeSpec::Date],
        };
        for type_text in [
            "INT | STRING | DATE",
            "ANY <INT | STRING | DATE>",
            "ANY VALUE <INT | STRING | DATE>",
        ] {
            assert_eq!(base(type_text), union, "{type_text}");
        }
        // A union contains null as soon as one of its members does.
        assert!(!cast_type("INT NOT NULL | STRING").not_null);
        assert!(cast_type("INT NOT NULL | STRING NOT NULL").not_null);
    }

    #[test]
    fn declared_types_are_built_for_is_typed_and_catalog_properties() {
        let expressions = return_expressions("MATCH (n) RETURN n.v IS TYPED UINT8 NOT NULL");
        let Expression::IsPredicate(predicate) = &expressions[0] else {
            panic!("expected IS TYPED predicate");
        };
        assert!(matches!(
            &predicate.predicate,
            IsPredicateKind::Typed(ValueType {
                base: TypeSpec::Integer {
                    signed: false,
                    size: IntegerSize::Bits8
                },
                not_null: true
            })
        ));

        let ast = analyze("CREATE GRAPH TYPE g AS { (p :Person { name STRING(1, 64) NOT NULL }) }")
            .expect("query should parse");
        let json = serde_json::to_string(&ast).expect("AST serializes");
        assert!(
            json.contains(
                r#""parsed_value_type":{"base":{"String":{"min_length":1,"max_length":64}},"not_null":true}"#
            ),
            "{json}"
        );
    }

    #[test]
    fn invalid_type_parameters_are_rejected() {
        for (type_text, message) in [
            ("STRING(10, 2)", "Minimum length 10 is greater than maximum length 2"),
            ("BYTES(3, 1)", "Minimum length 3 is greater than maximum length 1"),
            ("STRING(0)", "maximum length must be at least 1"),
            ("CHAR(0)", "fixed length must be at least 1"),
            ("BINARY(0)", "fixed length must be at least 1"),
            ("DECIMAL(5, 6)", "Scale 6 is greater than precision 5"),
            ("FLOAT(3, 4)", "Scale 4 is greater than precision 3"),
            ("DECIMAL(0)", "DECIMAL precision must be at least 1"),
            ("INT(0)", "integer precision must be at least 1"),
            ("UINT(0)", "integer precision must be at least 1"),
            ("LIST<INT>[0]", "list maximum length must be at least 1"),
            ("STRING(99999999999999999999999999)", "Invalid maximum length"),
            ("DECIMAL(99999999999)", "is too large"),
            ("RECORD { a INT, a STRING }", "Duplicate field name 'a'"),
            ("BINDING TABLE { a INT, a INT }", "Duplicate field name 'a'"),
        ] {
            assert_invalid_type_parameter(&format!("MATCH (n) RETURN CAST(n.v AS {type_text})"), message);
        }
        assert_invalid_type_parameter("MATCH (n) RETURN n.v IS TYPED STRING(3, 1)", "Minimum length 3");
        // Catalog statements that fail to build are kept as raw statements carrying the error.
        let ast =
            analyze("CREATE GRAPH TYPE g AS { (p :Person { name STRING(5, 2) }) }").expect("raw catalog statement");
        let debug = format!("{ast:?}");
        assert!(
            debug.contains("Minimum length 5 is greater than maximum length 2")
                && debug.contains(GqlError::INVALID_TYPE_PARAMETER),
            "{debug}"
        );
    }

    #[test]
    fn typed_temporal_byte_and_decimal_literals_use_dedicated_variants() {
        assert!(matches!(literal("RETURN DATE '2024-01-31'"), Literal::Date(v) if v == "2024-01-31"));
        assert!(matches!(literal("RETURN TIME '12:30:00+01:00'"), Literal::Time(v) if v == "12:30:00+01:00"));
        assert!(matches!(
            literal("RETURN TIMESTAMP '2024-01-31T12:30:00'"),
            Literal::DateTime(_)
        ));
        assert!(matches!(literal("RETURN DURATION 'P1D'"), Literal::Duration(v) if v == "P1D"));
        assert!(matches!(literal("RETURN X'00 ff 1A'"), Literal::Bytes(v) if v == vec![0x00, 0xFF, 0x1A]));
        assert!(matches!(literal("RETURN X''"), Literal::Bytes(v) if v.is_empty()));
        assert!(matches!(literal("RETURN 1_000.25M"), Literal::Decimal(v) if v == "1000.25"));
        assert!(matches!(literal("RETURN 1.5e3M"), Literal::Decimal(v) if v == "1.5e3"));
        assert!(matches!(literal("RETURN 7M"), Literal::Decimal(v) if v == "7"));
        // Unsuffixed decimals keep the approximate representation.
        assert!(matches!(literal("RETURN 1.5"), Literal::Float(v) if v == 1.5));
    }

    #[test]
    fn literal_and_declared_types_feed_validation() {
        for query in [
            "RETURN BYTE_LENGTH(X'0011'), OCTET_LENGTH(X'')",
            "RETURN CAST(X'00' AS BYTES(1))",
            "RETURN CAST(1.5M AS INT)",
            "RETURN 1.5M + 1",
            "RETURN DATE '2024-01-31' < DATE '2024-02-01'",
            "RETURN CAST(NULL AS INT)",
            "RETURN NULL IS TYPED INT NOT NULL",
            "MATCH (n) RETURN CAST(n.v AS INT8 NOT NULL)",
        ] {
            analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        }
        for (query, message) in [
            ("RETURN BYTE_LENGTH('abc')", "byte string"),
            ("RETURN CAST(X'00' AS INT)", "cannot cast BYTES to INTEGER"),
            (
                "RETURN CAST(NULL AS INT NOT NULL)",
                "cannot cast NULL to INTEGER NOT NULL",
            ),
            ("RETURN CAST(1 AS NOTHING)", "cannot cast INTEGER to NOTHING"),
            ("RETURN CAST(1 AS NULL)", "cannot cast INTEGER to NULL"),
            ("RETURN CAST(DATE '2024-01-31' AS INT)", "cannot cast DATE to INTEGER"),
        ] {
            let error = analyze(query).expect_err(&format!("query must fail: {query}"));
            assert!(
                matches!(error, GqlError::Validation { .. }),
                "query: {query}, error: {error}"
            );
            assert!(error.message().contains(message), "query: {query}, error: {error}");
        }
    }

    fn let_definitions(query: &str) -> Vec<VariableDefinition> {
        let ast = analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        let Statement::Query(Query::Basic(basic)) = &ast.statement else {
            panic!("expected a basic query for: {query}");
        };
        basic
            .pipeline
            .iter()
            .find_map(|step| match step {
                QueryStep::Let(let_statement) => Some(let_statement.variable_definitions.clone()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("expected LET for: {query}"))
    }

    #[test]
    fn typed_let_variables_keep_their_declared_type() {
        let definitions = let_definitions("LET VALUE x :: UINT8 NOT NULL = 5, VALUE y TYPED STRING(3) = 'abc', z = 1 RETURN x, y, z");
        assert_eq!(
            definitions[0].declared_type,
            Some(ValueType::new(integer(false, IntegerSize::Bits8), true))
        );
        assert_eq!(
            definitions[1].declared_type,
            Some(ValueType::nullable(TypeSpec::String {
                min_length: None,
                max_length: Some(3)
            }))
        );
        assert_eq!(definitions[2].declared_type, None);

        // An untyped VALUE definition has no declared type either.
        assert_eq!(let_definitions("LET VALUE w = 1 RETURN w")[0].declared_type, None);
    }

    #[test]
    fn typed_let_initializers_are_validated() {
        for query in [
            "LET VALUE x :: INT8 = 5 RETURN x",
            "LET VALUE x :: FLOAT32 = 5 RETURN x",
            "LET VALUE x :: DECIMAL(5, 2) = 1.5M RETURN x",
            "LET VALUE x :: ZONED DATETIME = TIMESTAMP '2024-01-31T12:00:00' RETURN x",
            "LET VALUE x :: LIST<INT> = [1, 2] RETURN x",
            "LET VALUE x :: INT = NULL RETURN x",
            "MATCH (n) LET VALUE x :: INT8 NOT NULL = n.age RETURN x",
            "RETURN LET VALUE x :: INT16 = 1 IN x + 1 END",
        ] {
            analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        }
        for (query, message) in [
            ("LET VALUE x :: INT8 = 'abc' RETURN x", "declared as INT8 cannot be initialized with a value of type STRING"),
            ("LET VALUE x :: INT NOT NULL = NULL RETURN x", "declared as INTEGER NOT NULL cannot be initialized with NULL"),
            ("LET VALUE x :: DATE = 1 RETURN x", "declared as DATE cannot be initialized with a value of type INTEGER"),
            ("LET VALUE x :: LIST<INT> = ['a'] RETURN x", "declared as LIST cannot be initialized with a value of type LIST"),
            ("RETURN LET VALUE x :: BOOLEAN = 1 IN x END", "declared as BOOLEAN cannot be initialized"),
        ] {
            let error = analyze(query).expect_err(&format!("query must fail: {query}"));
            assert!(matches!(error, GqlError::Validation { .. }), "query: {query}, error: {error}");
            assert!(error.message().contains(message), "query: {query}, error: {error}");
        }
        assert!(analyze("LET VALUE x :: STRING(5, 2) = 'a' RETURN x").is_err());
    }
}
