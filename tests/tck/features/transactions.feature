# Copyright (c) 2026 NTUR LAB S.r.l. (https://ntur-lab.com)
#
# This file is part of gql-rs (https://ntur-lab.com/gql-rs)
#
# Licensed under the EUPL, Version 1.2 or - as soon they will be approved by
# the European Commission - subsequent versions of the EUPL (the "Licence");
# You may not use this work except in compliance with the Licence.
# You may obtain a copy of the Licence at:
#
# https://joinup.ec.europa.eu/software/page/eupl
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the Licence is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the Licence for the specific language governing permissions and
# limitations under the Licence.

Feature: Transactions

  Scenario: Commit transaction
    When the GQL query is analyzed:
      """
      COMMIT
      """
    Then analysis succeeds

  Scenario: Rollback transaction
    When the GQL query is analyzed:
      """
      ROLLBACK
      """
    Then analysis succeeds

  Scenario: Start transaction plain
    When the GQL query is analyzed:
      """
      START TRANSACTION
      """
    Then analysis succeeds

  Scenario: Start transaction read only
    When the GQL query is analyzed:
      """
      START TRANSACTION READ ONLY
      """
    Then analysis succeeds

  Scenario: Start transaction read write
    When the GQL query is analyzed:
      """
      START TRANSACTION READ WRITE
      """
    Then analysis succeeds

  Scenario: Start transaction multiple modes
    When the GQL query is analyzed:
      """
      START TRANSACTION READ ONLY, READ WRITE
      """
    Then analysis succeeds

  Scenario: Start transaction with body
    When the GQL query is analyzed:
      """
      START TRANSACTION RETURN 1
      """
    Then analysis succeeds

  Scenario: Start transaction with body and commit
    When the GQL query is analyzed:
      """
      START TRANSACTION RETURN 1 COMMIT
      """
    Then analysis succeeds

  Scenario: Start transaction with body and rollback
    When the GQL query is analyzed:
      """
      START TRANSACTION RETURN 1 ROLLBACK
      """
    Then analysis succeeds

  Scenario: Procedure then commit
    When the GQL query is analyzed:
      """
      RETURN 1 COMMIT
      """
    Then analysis succeeds

  Scenario: Procedure then rollback
    When the GQL query is analyzed:
      """
      RETURN 1 ROLLBACK
      """
    Then analysis succeeds
