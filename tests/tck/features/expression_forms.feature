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

@expressions
Feature: Additional expression forms

  Scenario: Date literal
    When the GQL query is analyzed:
      """
      RETURN DATE '2024-01-31'
      """
    Then analysis succeeds

  Scenario: Time literal
    When the GQL query is analyzed:
      """
      RETURN TIME '12:30:45'
      """
    Then analysis succeeds

  Scenario: Datetime literal
    When the GQL query is analyzed:
      """
      RETURN DATETIME '2024-01-31T12:30:45'
      """
    Then analysis succeeds

  Scenario: Path value constructor with an edge step
    When the GQL query is analyzed:
      """
      MATCH (a)-[e]->(b) RETURN PATH[a, e, b]
      """
    Then analysis succeeds

  Scenario: List trim function
    When the GQL query is analyzed:
      """
      RETURN TRIM([1, 2, 3], 1)
      """
    Then analysis succeeds

  Scenario: Explicit list value constructor
    When the GQL query is analyzed:
      """
      RETURN LIST[1, 2, 3]
      """
    Then analysis succeeds

  Scenario: User-defined function with namespace
    When the GQL query is analyzed:
      """
      RETURN example.math.custom_fn(21)
      """
    Then analysis succeeds
