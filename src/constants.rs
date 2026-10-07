// Copyright (c) 2026 NTUR LAB S.r.l.
//
// This file is part of NTURDB.
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

// Reserved GQL keywords used by parser and semantic layers.
pub mod keywords {
    //pub const GRAPH: &str = "GRAPH";
    pub const VALUE: &str = "VALUE";
    pub const TABLE: &str = "TABLE";
}

// Built-in GQL function names used by builder and validator.
pub mod function_names {
    pub const ABS: &str = "ABS";
    pub const ALL_DIFFERENT: &str = "ALL_DIFFERENT";
    pub const AVG: &str = "AVG";
    pub const CEIL: &str = "CEIL";
    pub const COLLECT_LIST: &str = "COLLECT_LIST";
    pub const COLLECT_SET: &str = "COLLECT_SET";
    pub const COUNT: &str = "COUNT";
    pub const CURRENT_DATE: &str = "CURRENT_DATE";
    pub const CURRENT_TIME: &str = "CURRENT_TIME";
    pub const CURRENT_TIMESTAMP: &str = "CURRENT_TIMESTAMP";
    pub const DATE: &str = "DATE";
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
    pub const LN: &str = "LN";
    pub const LOCAL_TIME: &str = "LOCAL_TIME";
    pub const LOG: &str = "LOG";
    pub const LOG10: &str = "LOG10";
    pub const MAX: &str = "MAX";
    pub const MIN: &str = "MIN";
    pub const MOD: &str = "MOD";
    pub const NORMALIZE: &str = "NORMALIZE";
    pub const PATH_LENGTH: &str = "PATH_LENGTH";
    pub const POWER: &str = "POWER";
    pub const PROPERTY_EXISTS: &str = "PROPERTY_EXISTS";
    pub const SAME: &str = "SAME";
    pub const SQRT: &str = "SQRT";
    pub const SUM: &str = "SUM";
    pub const TRIM: &str = "TRIM";
    pub const ZONED_DATETIME: &str = "ZONED_DATETIME";
    pub const ZONED_TIME: &str = "ZONED_TIME";
}

// Aggregate function names for quick semantic checks.
pub const AGGREGATE_FUNCTIONS: [&str; 7] = [
    function_names::COUNT,
    function_names::SUM,
    function_names::AVG,
    function_names::MIN,
    function_names::MAX,
    function_names::COLLECT_LIST,
    function_names::COLLECT_SET,
];