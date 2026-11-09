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
//   Ntur Lab S.r.l. <dev@ntur-lab.com>
//   Nicola Miotto <nicola.miotto@ntur-lab.com>,
//   Matteo Boscolo <matteo.boscolo@ntur-lab.com>

use serde::{Deserialize, Serialize};

/// GQL statement access intent inferred from AST semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionIntent {
    /// Statement can safely run in read-only mode.
    ReadOnly,
    /// Statement modifies graph data.
    DataWrite,
    /// Statement modifies catalog/schema metadata.
    CatalogWrite,
    /// Statement modifies both graph data and catalog/schema metadata.
    DataAndCatalogWrite,
}

impl TransactionIntent {
    /// Combines two intents, preserving the strongest requirement.
    pub fn merge(self, other: Self) -> Self {
        match (self, other) {
            (Self::DataAndCatalogWrite, _) | (_, Self::DataAndCatalogWrite) => {
                Self::DataAndCatalogWrite
            }
            (Self::DataWrite, Self::CatalogWrite) | (Self::CatalogWrite, Self::DataWrite) => {
                Self::DataAndCatalogWrite
            }
            (Self::DataWrite, _) | (_, Self::DataWrite) => Self::DataWrite,
            (Self::CatalogWrite, _) | (_, Self::CatalogWrite) => Self::CatalogWrite,
            (Self::ReadOnly, Self::ReadOnly) => Self::ReadOnly,
        }
    }

    /// Returns true when the statement needs a write-capable storage transaction.
    pub fn requires_write_tx(self) -> bool {
        !matches!(self, Self::ReadOnly)
    }
}
