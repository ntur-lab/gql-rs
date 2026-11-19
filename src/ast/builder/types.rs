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

//! Structural construction of declared value types (`valueType` and its alternatives).
//!
//! Every alternative of the grammar is mapped by inspecting its CST accessors, never by
//! normalizing the source text. Type parameters are checked here because the GQL syntax rules
//! reject them (for example a scale greater than the precision): such declarations fail with
//! [`GqlError::INVALID_TYPE_PARAMETER`].

use super::*;

impl GQLAstBuilder {
    /// Builds a declared [`ValueType`] (base type and nullability) from a `valueType` context.
    pub(super) fn build_value_type(value_type: &ValueTypeContextAll) -> GqlResult<ValueType> {
        match value_type {
            ValueTypeContextAll::PredefinedTypeLabelContext(ctx) => {
                let predefined = ctx
                    .predefinedType()
                    .ok_or_else(|| Self::parse_err("CST not valid: predefinedType missing".into()))?;
                Self::build_predefined_value_type(&predefined)
            }
            ValueTypeContextAll::PathValueTypeLabelContext(ctx) => {
                let path = ctx
                    .pathValueType()
                    .ok_or_else(|| Self::parse_err("CST not valid: pathValueType missing".into()))?;
                Ok(ValueType::new(TypeSpec::Path, path.notNull().is_some()))
            }
            ValueTypeContextAll::RecordTypeLabelContext(ctx) => {
                let record = ctx
                    .recordType()
                    .ok_or_else(|| Self::parse_err("CST not valid: recordType missing".into()))?;
                let fields = record
                    .fieldTypesSpecification()
                    .map(|spec| Self::build_field_types(&spec))
                    .transpose()?;
                Ok(ValueType::new(TypeSpec::Record { fields }, record.notNull().is_some()))
            }
            ValueTypeContextAll::ListValueTypeAlt1Context(ctx) => {
                let element = ctx
                    .valueType()
                    .ok_or_else(|| Self::parse_err("CST not valid: LIST<T> without element type".into()))?;
                Self::build_list_value_type(
                    Self::build_value_type(&element)?,
                    ctx.maxLength(),
                    ctx.notNull().is_some(),
                )
            }
            ValueTypeContextAll::ListValueTypeAlt2Context(ctx) => {
                let element = ctx
                    .valueType()
                    .ok_or_else(|| Self::parse_err("CST not valid: T LIST without element type".into()))?;
                Self::build_list_value_type(
                    Self::build_value_type(&element)?,
                    ctx.maxLength(),
                    ctx.notNull().is_some(),
                )
            }
            ValueTypeContextAll::ListValueTypeAlt3Context(ctx) => Self::build_list_value_type(
                ValueType::nullable(TypeSpec::Any),
                ctx.maxLength(),
                ctx.notNull().is_some(),
            ),
            ValueTypeContextAll::OpenDynamicUnionTypeLabelContext(ctx) => {
                Ok(ValueType::new(TypeSpec::Any, ctx.notNull().is_some()))
            }
            ValueTypeContextAll::DynamicPropertyValueTypeLabelContext(ctx) => {
                Ok(ValueType::new(TypeSpec::PropertyValue, ctx.notNull().is_some()))
            }
            ValueTypeContextAll::ClosedDynamicUnionTypeAtl1Context(ctx) => {
                Self::build_union_value_type(ctx.valueType_all())
            }
            ValueTypeContextAll::ClosedDynamicUnionTypeAtl2Context(ctx) => {
                Self::build_union_value_type(ctx.valueType_all())
            }
            ValueTypeContextAll::Error(_) => Self::parse_error("CST not valid: invalid valueType context"),
        }
    }

    /// Builds a union type. Nested unions are flattened. A union contains the null value as soon
    /// as one of its members does, so it is `NOT NULL` only when every member is.
    fn build_union_value_type(members: Vec<Rc<ValueTypeContextAll<'_>>>) -> GqlResult<ValueType> {
        let mut flat = Vec::new();
        let mut not_null = true;
        for member in members {
            let member = Self::build_value_type(&member)?;
            not_null &= member.not_null;
            match member.base {
                TypeSpec::Union { members } => flat.extend(members),
                other => flat.push(other),
            }
        }
        Ok(ValueType::new(TypeSpec::Union { members: flat }, not_null))
    }

    fn build_list_value_type(
        element: ValueType,
        max_length: Option<Rc<MaxLengthContextAll<'_>>>,
        not_null: bool,
    ) -> GqlResult<ValueType> {
        let max_length = max_length
            .map(|ctx| Self::length_parameter(&ctx.get_text(), "list maximum length", Self::extract_span(&*ctx), 1))
            .transpose()?;
        Ok(ValueType::new(
            TypeSpec::List {
                element_type: Box::new(element),
                max_length,
            },
            not_null,
        ))
    }

    /// Builds the fields of a closed record or binding-table type, rejecting duplicate names.
    pub(super) fn build_field_types(spec: &FieldTypesSpecificationContextAll) -> GqlResult<Vec<FieldType>> {
        let mut fields: Vec<FieldType> = Vec::new();
        let Some(list) = spec.fieldTypeList() else {
            return Ok(fields);
        };
        for field in list.fieldType_all() {
            let name = field
                .fieldName()
                .map(|name| Self::identifier_text(&name.get_text()))
                .ok_or_else(|| Self::parse_err("CST not valid: fieldName missing in fieldType".into()))?;
            if fields.iter().any(|existing| existing.name == name) {
                return Err(GqlError::invalid_type_parameter(
                    format!("Duplicate field name '{name}' in record or binding table type"),
                    Self::extract_span(&*field),
                ));
            }
            let value_type = field
                .valueType()
                .ok_or_else(|| Self::parse_err("CST not valid: valueType missing in fieldType".into()))?;
            fields.push(FieldType {
                name,
                value_type: Self::build_value_type(&value_type)?,
            });
        }
        Ok(fields)
    }

    fn build_predefined_value_type(pre: &PredefinedTypeContextAll) -> GqlResult<ValueType> {
        if let Some(boolean) = pre.booleanType() {
            return Ok(ValueType::new(TypeSpec::Boolean, boolean.notNull().is_some()));
        }
        if let Some(string) = pre.characterStringType() {
            let (min_length, max_length) = Self::string_lengths(
                string.minLength().map(|c| (c.get_text(), Self::extract_span(&*c))),
                string.maxLength().map(|c| (c.get_text(), Self::extract_span(&*c))),
                string.fixedLength().map(|c| (c.get_text(), Self::extract_span(&*c))),
                Self::extract_span(&*string),
            )?;
            return Ok(ValueType::new(
                TypeSpec::String { min_length, max_length },
                string.notNull().is_some(),
            ));
        }
        if let Some(bytes) = pre.byteStringType() {
            let (min_length, max_length) = Self::string_lengths(
                bytes.minLength().map(|c| (c.get_text(), Self::extract_span(&*c))),
                bytes.maxLength().map(|c| (c.get_text(), Self::extract_span(&*c))),
                bytes.fixedLength().map(|c| (c.get_text(), Self::extract_span(&*c))),
                Self::extract_span(&*bytes),
            )?;
            return Ok(ValueType::new(
                TypeSpec::Bytes { min_length, max_length },
                bytes.notNull().is_some(),
            ));
        }
        if let Some(numeric) = pre.numericType() {
            return Self::build_numeric_value_type(&numeric);
        }
        if let Some(temporal) = pre.temporalType() {
            return Self::build_temporal_value_type(&temporal);
        }
        if let Some(reference) = pre.referenceValueType() {
            return Self::build_reference_value_type(&reference);
        }
        if let Some(immaterial) = pre.immaterialValueType() {
            if immaterial.nullType().is_some() {
                return Ok(ValueType::nullable(TypeSpec::Null));
            }
            // `NOTHING` and `NULL NOT NULL` denote the empty type, which excludes null.
            return Ok(ValueType::new(TypeSpec::Nothing, true));
        }
        Self::parse_error(format!("CST not valid: unsupported predefined type {}", pre.get_text()))
    }

    /// Resolves the `(min, max)` length pair of a character or byte string type.
    /// `CHAR(n)` / `BINARY(n)` are fixed-length and become `(n, n)`.
    fn string_lengths(
        min: Option<(String, Span)>,
        max: Option<(String, Span)>,
        fixed: Option<(String, Span)>,
        span: Span,
    ) -> GqlResult<(Option<usize>, Option<usize>)> {
        if let Some((text, span)) = fixed {
            let fixed = Self::length_parameter(&text, "fixed length", span, 1)?;
            return Ok((Some(fixed), Some(fixed)));
        }
        let min = min
            .map(|(text, span)| Self::length_parameter(&text, "minimum length", span, 0))
            .transpose()?;
        let max = max
            .map(|(text, span)| Self::length_parameter(&text, "maximum length", span, 1))
            .transpose()?;
        if let (Some(min), Some(max)) = (min, max)
            && min > max
        {
            return Err(GqlError::invalid_type_parameter(
                format!("Minimum length {min} is greater than maximum length {max}"),
                span,
            ));
        }
        Ok((min, max))
    }

    fn build_numeric_value_type(numeric: &NumericTypeContextAll) -> GqlResult<ValueType> {
        if let Some(approximate) = numeric.approximateNumericType() {
            return Self::build_approximate_value_type(&approximate);
        }
        let exact = numeric
            .exactNumericType()
            .ok_or_else(|| Self::parse_err("CST not valid: numericType without alternative".into()))?;
        if let Some(decimal) = exact.decimalExactNumericType() {
            let precision = decimal
                .precision()
                .map(|p| Self::precision_parameter(&p.get_text(), "DECIMAL precision", Self::extract_span(&*p)))
                .transpose()?;
            let scale = decimal
                .scale()
                .map(|s| Self::scale_parameter(&s.get_text(), "DECIMAL scale", Self::extract_span(&*s)))
                .transpose()?;
            Self::check_scale(precision, scale, Self::extract_span(&*decimal))?;
            return Ok(ValueType::new(
                TypeSpec::Decimal { precision, scale },
                decimal.notNull().is_some(),
            ));
        }
        let binary = exact
            .binaryExactNumericType()
            .ok_or_else(|| Self::parse_err("CST not valid: exactNumericType without alternative".into()))?;
        if let Some(signed) = binary.signedBinaryExactNumericType() {
            if let Some(verbose) = signed.verboseBinaryExactNumericType() {
                return Self::build_verbose_integer_value_type(&verbose, true);
            }
            let size = if signed.INT8().is_some() {
                IntegerSize::Bits8
            } else if signed.INT16().is_some() {
                IntegerSize::Bits16
            } else if signed.INT32().is_some() {
                IntegerSize::Bits32
            } else if signed.INT64().is_some() {
                IntegerSize::Bits64
            } else if signed.INT128().is_some() {
                IntegerSize::Bits128
            } else if signed.INT256().is_some() {
                IntegerSize::Bits256
            } else if signed.SMALLINT().is_some() {
                IntegerSize::Small
            } else if signed.BIGINT().is_some() {
                IntegerSize::Big
            } else {
                IntegerSize::Regular {
                    precision: Self::integer_precision(signed.precision())?,
                }
            };
            return Ok(ValueType::new(
                TypeSpec::Integer { signed: true, size },
                signed.notNull().is_some(),
            ));
        }
        let unsigned = binary
            .unsignedBinaryExactNumericType()
            .ok_or_else(|| Self::parse_err("CST not valid: binaryExactNumericType without alternative".into()))?;
        if let Some(verbose) = unsigned.verboseBinaryExactNumericType() {
            return Self::build_verbose_integer_value_type(&verbose, false);
        }
        let size = if unsigned.UINT8().is_some() {
            IntegerSize::Bits8
        } else if unsigned.UINT16().is_some() {
            IntegerSize::Bits16
        } else if unsigned.UINT32().is_some() {
            IntegerSize::Bits32
        } else if unsigned.UINT64().is_some() {
            IntegerSize::Bits64
        } else if unsigned.UINT128().is_some() {
            IntegerSize::Bits128
        } else if unsigned.UINT256().is_some() {
            IntegerSize::Bits256
        } else if unsigned.USMALLINT().is_some() {
            IntegerSize::Small
        } else if unsigned.UBIGINT().is_some() {
            IntegerSize::Big
        } else {
            IntegerSize::Regular {
                precision: Self::integer_precision(unsigned.precision())?,
            }
        };
        Ok(ValueType::new(
            TypeSpec::Integer { signed: false, size },
            unsigned.notNull().is_some(),
        ))
    }

    /// Builds `[SIGNED | UNSIGNED] INTEGER8 | ... | SMALL INTEGER | INTEGER[(p)] | BIG INTEGER`.
    fn build_verbose_integer_value_type(
        verbose: &VerboseBinaryExactNumericTypeContextAll,
        signed: bool,
    ) -> GqlResult<ValueType> {
        let size = if verbose.INTEGER8().is_some() {
            IntegerSize::Bits8
        } else if verbose.INTEGER16().is_some() {
            IntegerSize::Bits16
        } else if verbose.INTEGER32().is_some() {
            IntegerSize::Bits32
        } else if verbose.INTEGER64().is_some() {
            IntegerSize::Bits64
        } else if verbose.INTEGER128().is_some() {
            IntegerSize::Bits128
        } else if verbose.INTEGER256().is_some() {
            IntegerSize::Bits256
        } else if verbose.SMALL().is_some() {
            IntegerSize::Small
        } else if verbose.BIG().is_some() {
            IntegerSize::Big
        } else {
            IntegerSize::Regular {
                precision: Self::integer_precision(verbose.precision())?,
            }
        };
        Ok(ValueType::new(
            TypeSpec::Integer { signed, size },
            verbose.notNull().is_some(),
        ))
    }

    fn integer_precision(precision: Option<Rc<PrecisionContextAll<'_>>>) -> GqlResult<Option<u32>> {
        precision
            .map(|p| Self::precision_parameter(&p.get_text(), "integer precision", Self::extract_span(&*p)))
            .transpose()
    }

    fn build_approximate_value_type(approximate: &ApproximateNumericTypeContextAll) -> GqlResult<ValueType> {
        let size = if approximate.FLOAT16().is_some() {
            FloatSize::Bits16
        } else if approximate.FLOAT32().is_some() {
            FloatSize::Bits32
        } else if approximate.FLOAT64().is_some() {
            FloatSize::Bits64
        } else if approximate.FLOAT128().is_some() {
            FloatSize::Bits128
        } else if approximate.FLOAT256().is_some() {
            FloatSize::Bits256
        } else if approximate.REAL().is_some() {
            FloatSize::Real
        } else if approximate.DOUBLE().is_some() {
            FloatSize::Double
        } else {
            let precision = approximate
                .precision()
                .map(|p| Self::precision_parameter(&p.get_text(), "FLOAT precision", Self::extract_span(&*p)))
                .transpose()?;
            let scale = approximate
                .scale()
                .map(|s| Self::scale_parameter(&s.get_text(), "FLOAT scale", Self::extract_span(&*s)))
                .transpose()?;
            Self::check_scale(precision, scale, Self::extract_span(approximate))?;
            FloatSize::Regular { precision, scale }
        };
        Ok(ValueType::new(
            TypeSpec::Float { size },
            approximate.notNull().is_some(),
        ))
    }

    fn build_temporal_value_type(temporal: &TemporalTypeContextAll) -> GqlResult<ValueType> {
        if let Some(duration) = temporal.temporalDurationType() {
            let qualifier = duration
                .temporalDurationQualifier()
                .ok_or_else(|| Self::parse_err("CST not valid: DURATION without qualifier".into()))?;
            let qualifier = if qualifier.YEAR().is_some() {
                DurationQualifier::YearToMonth
            } else {
                DurationQualifier::DayToSecond
            };
            return Ok(ValueType::new(
                TypeSpec::Duration {
                    qualifier: Some(qualifier),
                },
                duration.notNull().is_some(),
            ));
        }
        let instant = temporal
            .temporalInstantType()
            .ok_or_else(|| Self::parse_err("CST not valid: temporalType without alternative".into()))?;
        if let Some(ctx) = instant.datetimeType() {
            return Ok(ValueType::new(TypeSpec::ZonedDateTime, ctx.notNull().is_some()));
        }
        if let Some(ctx) = instant.localdatetimeType() {
            return Ok(ValueType::new(TypeSpec::LocalDateTime, ctx.notNull().is_some()));
        }
        if let Some(ctx) = instant.dateType() {
            return Ok(ValueType::new(TypeSpec::Date, ctx.notNull().is_some()));
        }
        if let Some(ctx) = instant.timeType() {
            return Ok(ValueType::new(TypeSpec::ZonedTime, ctx.notNull().is_some()));
        }
        if let Some(ctx) = instant.localtimeType() {
            return Ok(ValueType::new(TypeSpec::LocalTime, ctx.notNull().is_some()));
        }
        Self::parse_error(format!(
            "CST not valid: unsupported temporal type {}",
            temporal.get_text()
        ))
    }

    fn build_reference_value_type(reference: &ReferenceValueTypeContextAll) -> GqlResult<ValueType> {
        if let Some(graph) = reference.graphReferenceValueType() {
            if let Some(open) = graph.openGraphReferenceValueType() {
                return Ok(ValueType::new(
                    TypeSpec::Graph { graph_type_spec: None },
                    open.notNull().is_some(),
                ));
            }
            let closed = graph
                .closedGraphReferenceValueType()
                .ok_or_else(|| Self::parse_err("CST not valid: graph reference without alternative".into()))?;
            let spec = Self::build_graph_type_spec_from_closed_graph_reference(&closed)?;
            return Ok(ValueType::new(
                TypeSpec::Graph {
                    graph_type_spec: Some(Box::new(spec)),
                },
                closed.notNull().is_some(),
            ));
        }
        if let Some(table) = reference.bindingTableReferenceValueType() {
            let fields = table
                .bindingTableType()
                .and_then(|t| t.fieldTypesSpecification())
                .ok_or_else(|| Self::parse_err("CST not valid: binding table type without fields".into()))?;
            return Ok(ValueType::new(
                TypeSpec::BindingTable {
                    fields: Some(Self::build_field_types(&fields)?),
                },
                table.notNull().is_some(),
            ));
        }
        if let Some(node) = reference.nodeReferenceValueType() {
            if let Some(open) = node.openNodeReferenceValueType() {
                return Ok(ValueType::new(
                    TypeSpec::Node { node_type: None },
                    open.notNull().is_some(),
                ));
            }
            let closed = node
                .closedNodeReferenceValueType()
                .ok_or_else(|| Self::parse_err("CST not valid: node reference without alternative".into()))?;
            let spec = closed
                .nodeTypeSpecification()
                .ok_or_else(|| Self::parse_err("CST not valid: closed node reference without spec".into()))?;
            return Ok(ValueType::new(
                TypeSpec::Node {
                    node_type: Some(Box::new(Self::build_vertex_type_spec_from_context(&spec)?)),
                },
                closed.notNull().is_some(),
            ));
        }
        if let Some(edge) = reference.edgeReferenceValueType() {
            if let Some(open) = edge.openEdgeReferenceValueType() {
                return Ok(ValueType::new(
                    TypeSpec::Edge { edge_type: None },
                    open.notNull().is_some(),
                ));
            }
            let closed = edge
                .closedEdgeReferenceValueType()
                .ok_or_else(|| Self::parse_err("CST not valid: edge reference without alternative".into()))?;
            let spec = closed
                .edgeTypeSpecification()
                .ok_or_else(|| Self::parse_err("CST not valid: closed edge reference without spec".into()))?;
            return Ok(ValueType::new(
                TypeSpec::Edge {
                    edge_type: Some(Box::new(Self::build_edge_type_spec_from_context(&spec)?)),
                },
                closed.notNull().is_some(),
            ));
        }
        Self::parse_error(format!(
            "CST not valid: unsupported reference type {}",
            reference.get_text()
        ))
    }

    /// Parses an unsigned integer type parameter (decimal, `0x`, `0o` or `0b`, with optional
    /// underscores) and checks its lower bound.
    fn length_parameter(text: &str, what: &str, span: Span, minimum: usize) -> GqlResult<usize> {
        let value = Self::decode_unsigned_integer_magnitude(text)
            .and_then(|value| usize::try_from(value).ok())
            .ok_or_else(|| GqlError::invalid_type_parameter(format!("Invalid {what} '{text}'"), span.clone()))?;
        if value < minimum {
            return Err(GqlError::invalid_type_parameter(
                format!("The {what} must be at least {minimum}, found {value}"),
                span,
            ));
        }
        Ok(value)
    }

    /// Parses a precision, which must be at least 1.
    fn precision_parameter(text: &str, what: &str, span: Span) -> GqlResult<u32> {
        let value = Self::length_parameter(text, what, span.clone(), 1)?;
        u32::try_from(value)
            .map_err(|_| GqlError::invalid_type_parameter(format!("The {what} {value} is too large"), span))
    }

    /// Parses a scale, which may be 0.
    fn scale_parameter(text: &str, what: &str, span: Span) -> GqlResult<u32> {
        let value = Self::length_parameter(text, what, span.clone(), 0)?;
        u32::try_from(value)
            .map_err(|_| GqlError::invalid_type_parameter(format!("The {what} {value} is too large"), span))
    }

    fn check_scale(precision: Option<u32>, scale: Option<u32>, span: Span) -> GqlResult<()> {
        if let (Some(precision), Some(scale)) = (precision, scale)
            && scale > precision
        {
            return Err(GqlError::invalid_type_parameter(
                format!("Scale {scale} is greater than precision {precision}"),
                span,
            ));
        }
        Ok(())
    }
}
