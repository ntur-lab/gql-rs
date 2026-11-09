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

@catalog @types
Feature: Type system and catalog references

  Scenario: Relative parent schema reference
    When the GQL query is analyzed:
      """
      AT ../schemas/example RETURN 1
      """
    Then analysis succeeds

  Scenario: Delimited graph name
    When the GQL query is analyzed:
      """
      USE "social graph" RETURN 1
      """
    Then analysis succeeds

  Scenario: Home graph reference
    When the GQL query is analyzed:
      """
      USE HOME_GRAPH RETURN 1
      """
    Then analysis succeeds

  Scenario: Delimited binding table name
    When the GQL query is analyzed:
      """
      SESSION SET TABLE $result = "result table"
      """
    Then analysis succeeds

  Scenario: Qualified binding table name
    When the GQL query is analyzed:
      """
      SESSION SET TABLE $result = /example/result_table
      """
    Then analysis succeeds

  Scenario: Node pattern with labels and property types
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE TYPE Person (p LABEL Person {name STRING, age INTEGER}) }
      """
    Then analysis succeeds

  Scenario: Node type with key label set
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE TYPE Person (p LABEL Person IMPLIES LABEL Employee {name STRING}) }
      """
    Then analysis succeeds

  Scenario: Directed edge pattern pointing right
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE TYPE Person (p), NODE TYPE Company (c), DIRECTED EDGE TYPE WorksAt (p)-[LABEL WorksAt]->(c) }
      """
    Then analysis succeeds

  Scenario: Directed edge pattern pointing left
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE TYPE Person (p), NODE TYPE Company (c), DIRECTED EDGE TYPE Employs (c)<-[LABEL Employs]-(p) }
      """
    Then analysis succeeds

  Scenario: Undirected edge pattern
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE TYPE Person (p), UNDIRECTED EDGE TYPE Knows (p)~[LABEL Knows]~(p) }
      """
    Then analysis succeeds

  Scenario: Edge phrase with directed endpoint pair
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE Person AS p, NODE Company AS c, DIRECTED EDGE WorksAt CONNECTING (p TO c) }
      """
    Then analysis succeeds

  Scenario: Edge phrase with key label and property types
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE Person AS p, NODE Company AS c, DIRECTED EDGE WorksAt LABEL WorksAt IMPLIES LABEL Employment {since STRING} CONNECTING (p TO c) }
      """
    Then analysis succeeds

  Scenario: Edge phrase with left-pointing endpoint pair
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE Person AS p, NODE Company AS c, DIRECTED EDGE Employs CONNECTING (c <- p) }
      """
    Then analysis succeeds

  Scenario: Edge phrase with undirected endpoint pair
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE social { NODE Person AS p, UNDIRECTED EDGE Knows CONNECTING (p ~ p) }
      """
    Then analysis succeeds

  Scenario: Closed graph reference value type
    When the GQL query is analyzed:
      """
      VALUE g TYPED PROPERTY GRAPH { NODE Person } = PROPERTY GRAPH CURRENT_GRAPH FINISH
      """
    Then analysis succeeds

  Scenario: Closed node reference value type
    When the GQL query is analyzed:
      """
      VALUE n TYPED NODE Person = NULL FINISH
      """
    Then analysis succeeds

  Scenario: Open edge reference value type
    When the GQL query is analyzed:
      """
      VALUE e TYPED ANY EDGE = NULL FINISH
      """
    Then analysis succeeds

  Scenario: Open node reference value type
    When the GQL query is analyzed:
      """
      VALUE n TYPED ANY NODE = NULL FINISH
      """
    Then analysis succeeds

  Scenario: Closed edge reference value type
    When the GQL query is analyzed:
      """
      VALUE e TYPED DIRECTED EDGE WorksAt CONNECTING (p TO c) = NULL FINISH
      """
    Then analysis succeeds

  Scenario: Object expression primary in graph initializer
    When the GQL query is analyzed:
      """
      GRAPH g = VARIABLE 1 FINISH
      """
    Then analysis succeeds
