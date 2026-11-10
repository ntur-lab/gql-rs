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
#
# Authors:
#   Ntur Lab S.r.l. <dev@ntur-lab.com>
#   Nicola Miotto <nicola.miotto@ntur-lab.com>,
#   Matteo Boscolo <matteo.boscolo@ntur-lab.com>

Feature: Catalog and session statements

  Scenario: Create schema not built
    When the GQL query is analyzed:
      """
      CREATE SCHEMA /myschemas/foo
      """
    Then analysis succeeds

  Scenario: Drop schema not built
    When the GQL query is analyzed:
      """
      DROP SCHEMA /myschemas/foo
      """
    Then analysis succeeds

  Scenario: Create graph not built
    When the GQL query is analyzed:
      """
      CREATE GRAPH social_graph ANY GRAPH
      """
    Then analysis succeeds

  Scenario: Drop property graph not built
    When the GQL query is analyzed:
      """
      DROP PROPERTY GRAPH social_graph
      """
    Then analysis succeeds

  Scenario: Create graph type not built
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE my_type LIKE CURRENT_GRAPH
      """
    Then analysis succeeds

  Scenario: Drop graph type not built
    When the GQL query is analyzed:
      """
      DROP GRAPH TYPE IF EXISTS my_type
      """
    Then analysis succeeds

  Scenario: Create schema if not exists
    When the GQL query is analyzed:
      """
      CREATE SCHEMA IF NOT EXISTS /myschemas/bar
      """
    Then analysis succeeds

  Scenario: Create property graph any
    When the GQL query is analyzed:
      """
      CREATE PROPERTY GRAPH my_graph ANY PROPERTY GRAPH
      """
    Then analysis succeeds

  Scenario: Create property graph copy
    When the GQL query is analyzed:
      """
      CREATE PROPERTY GRAPH my_graph LIKE CURRENT_GRAPH AS COPY OF CURRENT_GRAPH
      """
    Then analysis succeeds

  Scenario: Create graph type node type
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE my_type { NODE TYPE Person }
      """
    Then analysis succeeds

  Scenario: Drop property graph if exists
    When the GQL query is analyzed:
      """
      DROP PROPERTY GRAPH IF EXISTS my_graph
      """
    Then analysis succeeds

  Scenario: Session set graph
    When the GQL query is analyzed:
      """
      SESSION SET GRAPH $g = my_graph
      """
    Then analysis succeeds

  Scenario: Session set value
    When the GQL query is analyzed:
      """
      SESSION SET VALUE $v = 42
      """
    Then analysis succeeds

  Scenario: Session set table
    When the GQL query is analyzed:
      """
      SESSION SET TABLE $t = my_table
      """
    Then analysis succeeds

  Scenario: Session set table literal
    When the GQL query is analyzed:
      """
      SESSION SET TABLE $t = { MATCH (n) RETURN n }
      """
    Then analysis succeeds

  Scenario: Session reset characteristics
    When the GQL query is analyzed:
      """
      SESSION RESET CHARACTERISTICS
      """
    Then analysis succeeds

  Scenario: Drop schema if exists
    When the GQL query is analyzed:
      """
      DROP SCHEMA IF EXISTS /myschemas/foo
      """
    Then analysis succeeds

  Scenario: Create graph like current graph
    When the GQL query is analyzed:
      """
      CREATE GRAPH social_graph LIKE CURRENT_GRAPH
      """
    Then analysis succeeds

  Scenario: Create graph typed reference
    When the GQL query is analyzed:
      """
      CREATE GRAPH social_graph TYPED my_graph_type
      """
    Then analysis succeeds

  Scenario: Create graph or replace
    When the GQL query is analyzed:
      """
      CREATE OR REPLACE GRAPH social_graph ANY GRAPH
      """
    Then analysis succeeds

  Scenario: Create graph if not exists
    When the GQL query is analyzed:
      """
      CREATE GRAPH IF NOT EXISTS social_graph ANY GRAPH
      """
    Then analysis succeeds

  Scenario: Create property graph if not exists
    When the GQL query is analyzed:
      """
      CREATE PROPERTY GRAPH IF NOT EXISTS social_graph ANY PROPERTY GRAPH
      """
    Then analysis succeeds

  Scenario: Create graph with source
    When the GQL query is analyzed:
      """
      CREATE GRAPH social_graph ANY GRAPH AS COPY OF CURRENT_GRAPH
      """
    Then analysis succeeds

  Scenario: Drop graph if exists
    When the GQL query is analyzed:
      """
      DROP GRAPH IF EXISTS social_graph
      """
    Then analysis succeeds

  Scenario: Create graph type copy of
    When the GQL query is analyzed:
      """
      CREATE GRAPH TYPE my_graph_type COPY OF base_graph_type
      """
    Then analysis succeeds

  Scenario: Create property graph type or replace
    When the GQL query is analyzed:
      """
      CREATE OR REPLACE PROPERTY GRAPH TYPE my_graph_type LIKE CURRENT_GRAPH
      """
    Then analysis succeeds

  Scenario: Drop property graph type
    When the GQL query is analyzed:
      """
      DROP PROPERTY GRAPH TYPE my_graph_type
      """
    Then analysis succeeds

  Scenario: Session set schema current schema
    When the GQL query is analyzed:
      """
      SESSION SET SCHEMA CURRENT_SCHEMA
      """
    Then analysis succeeds

  Scenario: Session set time zone
    When the GQL query is analyzed:
      """
      SESSION SET TIME ZONE 'UTC'
      """
    Then analysis succeeds

  Scenario: Session reset graph
    When the GQL query is analyzed:
      """
      SESSION RESET GRAPH
      """
    Then analysis succeeds

  Scenario: Session reset time zone
    When the GQL query is analyzed:
      """
      SESSION RESET TIME ZONE
      """
    Then analysis succeeds

  Scenario: Session reset all parameters
    When the GQL query is analyzed:
      """
      SESSION RESET ALL PARAMETERS
      """
    Then analysis succeeds

  Scenario: Session set then reset
    When the GQL query is analyzed:
      """
      SESSION SET GRAPH CURRENT_GRAPH SESSION RESET GRAPH
      """
    Then analysis succeeds

  Scenario: Session close
    When the GQL query is analyzed:
      """
      SESSION CLOSE
      """
    Then analysis succeeds

  Scenario: Value binding
    When the GQL query is analyzed:
      """
      VALUE x = 1 MATCH (n) RETURN n
      """
    Then analysis succeeds

  Scenario: Table binding
    When the GQL query is analyzed:
      """
      TABLE t = my_table MATCH (n) RETURN n
      """
    Then analysis succeeds

  Scenario: Graph binding
    When the GQL query is analyzed:
      """
      GRAPH g = my_graph MATCH (n) RETURN n
      """
    Then analysis succeeds
