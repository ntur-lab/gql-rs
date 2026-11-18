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

// Built-in GQL function names used by builder and validator.
pub mod function_names {
    pub const ABS: &str = "ABS";
    pub const ACOS: &str = "ACOS";
    pub const ALL_DIFFERENT: &str = "ALL_DIFFERENT";
    pub const ASIN: &str = "ASIN";
    pub const ATAN: &str = "ATAN";
    pub const AVG: &str = "AVG";
    pub const BTRIM: &str = "BTRIM";
    pub const BYTE_LENGTH: &str = "BYTE_LENGTH";
    pub const CARDINALITY: &str = "CARDINALITY";
    pub const CEIL: &str = "CEIL";
    pub const CHAR_LENGTH: &str = "CHAR_LENGTH";
    pub const CHARACTER_LENGTH: &str = "CHARACTER_LENGTH";
    pub const COALESCE: &str = "COALESCE";
    pub const COLLECT_LIST: &str = "COLLECT_LIST";
    pub const COS: &str = "COS";
    pub const COSH: &str = "COSH";
    pub const COT: &str = "COT";
    pub const COUNT: &str = "COUNT";
    pub const CURRENT_DATE: &str = "CURRENT_DATE";
    pub const CURRENT_TIME: &str = "CURRENT_TIME";
    pub const CURRENT_TIMESTAMP: &str = "CURRENT_TIMESTAMP";
    pub const DATE: &str = "DATE";
    pub const DEGREES: &str = "DEGREES";
    pub const DURATION: &str = "DURATION";
    pub const DURATION_BETWEEN: &str = "DURATION_BETWEEN";
    pub const ELEMENT_ID: &str = "ELEMENT_ID";
    pub const ELEMENTS: &str = "ELEMENTS";
    pub const EXP: &str = "EXP";
    pub const FLOOR: &str = "FLOOR";
    pub const IS_DESTINATION_OF: &str = "IS_DESTINATION_OF";
    pub const IS_DIRECTED: &str = "IS_DIRECTED";
    pub const IS_LABELED: &str = "IS_LABELED";
    pub const IS_NORMALIZED: &str = "IS_NORMALIZED";
    pub const IS_SOURCE_OF: &str = "IS_SOURCE_OF";
    pub const LEFT: &str = "LEFT";
    pub const LN: &str = "LN";
    pub const LOCAL_DATETIME: &str = "LOCAL_DATETIME";
    pub const LOCAL_TIME: &str = "LOCAL_TIME";
    pub const LOCAL_TIMESTAMP: &str = "LOCAL_TIMESTAMP";
    pub const LOG: &str = "LOG";
    pub const LOG10: &str = "LOG10";
    pub const LOWER: &str = "LOWER";
    pub const LTRIM: &str = "LTRIM";
    pub const MAX: &str = "MAX";
    pub const MIN: &str = "MIN";
    pub const MOD: &str = "MOD";
    pub const NORMALIZE: &str = "NORMALIZE";
    pub const NULLIF: &str = "NULLIF";
    pub const OCTET_LENGTH: &str = "OCTET_LENGTH";
    pub const PATH_LENGTH: &str = "PATH_LENGTH";
    pub const PERCENTILE_CONT: &str = "PERCENTILE_CONT";
    pub const PERCENTILE_DISC: &str = "PERCENTILE_DISC";
    pub const POWER: &str = "POWER";
    pub const PROPERTY_EXISTS: &str = "PROPERTY_EXISTS";
    pub const RADIANS: &str = "RADIANS";
    pub const RIGHT: &str = "RIGHT";
    pub const RTRIM: &str = "RTRIM";
    pub const SAME: &str = "SAME";
    pub const SESSION_USER: &str = "SESSION_USER";
    pub const SIN: &str = "SIN";
    pub const SINH: &str = "SINH";
    pub const SIZE: &str = "SIZE";
    pub const SQRT: &str = "SQRT";
    pub const STDDEV_POP: &str = "STDDEV_POP";
    pub const STDDEV_SAMP: &str = "STDDEV_SAMP";
    pub const SUM: &str = "SUM";
    pub const TAN: &str = "TAN";
    pub const TANH: &str = "TANH";
    pub const TRIM: &str = "TRIM";
    pub const UPPER: &str = "UPPER";
    pub const ZONED_DATETIME: &str = "ZONED_DATETIME";
    pub const ZONED_TIME: &str = "ZONED_TIME";
}
