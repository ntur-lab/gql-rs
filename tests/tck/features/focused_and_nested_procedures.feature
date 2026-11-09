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

@procedures
Feature: Focused and nested procedures

  Scenario: Focused primitive result
    When the GQL query is analyzed:
      """
      USE social RETURN 1
      """
    Then analysis succeeds

  Scenario: Focused nested query
    When the GQL query is analyzed:
      """
      USE social { RETURN 1 }
      """
    Then analysis succeeds

  Scenario: Focused query part followed by a result
    When the GQL query is analyzed:
      """
      USE social MATCH (n) RETURN n
      """
    Then analysis succeeds

  Scenario: Multiple focused query parts
    When the GQL query is analyzed:
      """
      USE social MATCH (n) USE archive MATCH (m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Focused nested data-modifying procedure
    When the GQL query is analyzed:
      """
      USE social { MATCH (n) SET n.active = TRUE FINISH }
      """
    Then analysis succeeds

  Scenario: Replace all properties
    When the GQL query is analyzed:
      """
      MATCH (n) SET n = {name: 'Neo'} RETURN n
      """
    Then analysis succeeds

  Scenario: Undefined binding table is rejected semantically
    When the GQL query is analyzed:
      """
      FOR x IN TABLE t RETURN x
      """
    Then analysis fails
