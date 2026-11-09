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

Feature: Procedure calls

  Scenario: Call empty
    When the GQL query is analyzed:
      """
      CALL system_cleanup()
      """
    Then analysis succeeds

  Scenario: Call with args
    When the GQL query is analyzed:
      """
      CALL calculate_risk_score(100, 1.5, true)
      """
    Then analysis succeeds

  Scenario: Call with parameters
    When the GQL query is analyzed:
      """
      CALL my_schema.update_user($userId, $newStatus)
      """
    Then analysis succeeds

  Scenario: CALL with YIELD and NEXT
    When the GQL query is analyzed:
      """
      CALL get_system_status() YIELD status NEXT YIELD status RETURN status
      """
    Then analysis succeeds

  Scenario: Call args and yield
    When the GQL query is analyzed:
      """
      CALL find_shortest_path('NodeA', 'NodeB') YIELD result_path, total_cost
      """
    Then analysis succeeds

  Scenario: Call in pipeline
    When the GQL query is analyzed:
      """
      MATCH (n:User) CALL get_user_recommendations(n.id) YIELD recommended_id, score RETURN n.name, recommended_id, score
      """
    Then analysis succeeds

  Scenario: Call with namespace
    When the GQL query is analyzed:
      """
      CALL algo.pageRank('User', 'FOLLOWS') YIELD nodeId, rank
      """
    Then analysis succeeds

  Scenario: Call with namespace no yield
    When the GQL query is analyzed:
      """
      CALL algo.pageRank('User','FOLLOWS')
      """
    Then analysis succeeds

  Scenario: Call with variable args
    When the GQL query is analyzed:
      """
      CALL find_shortest_path($a, $b) YIELD result_path
      """
    Then analysis succeeds

  Scenario: Call with yield and order
    When the GQL query is analyzed:
      """
      MATCH (n) CALL enrich(n.id) YIELD score RETURN n, score ORDER BY score DESC LIMIT 10
      """
    Then analysis succeeds

  Scenario: Call in for loop
    When the GQL query is analyzed:
      """
      FOR id IN [1, 2, 3] CALL enrich_data(id) YIELD score RETURN id, score
      """
    Then analysis succeeds

  Scenario: Call with complex parameters
    When the GQL query is analyzed:
      """
      MATCH (n) CALL calculate_risk(n.age, n.salary, n.credit_score) YIELD risk_level, confidence RETURN risk_level, confidence
      """
    Then analysis succeeds

  Scenario: Call procedure with namespace yield
    When the GQL query is analyzed:
      """
      CALL analytics.compute_metrics('daily') YIELD metric_name, metric_value RETURN metric_name, metric_value ORDER BY metric_value DESC
      """
    Then analysis succeeds

  Scenario: Call subquery
    When the GQL query is analyzed:
      """
      MATCH (n) CALL { MATCH (m) RETURN m } RETURN n
      """
    Then analysis succeeds

  Scenario: Call procedure
    When the GQL query is analyzed:
      """
      MATCH (n) CALL (n) { MATCH (n)-[]->(m) RETURN m } RETURN n
      """
    Then analysis succeeds

  Scenario: CALL with basic YIELD
    When the GQL query is analyzed:
      """
      CALL name(1) YIELD columns
      """
    Then analysis succeeds

  Scenario: Call yield result
    When the GQL query is analyzed:
      """
      CALL my_proc() YIELD result RETURN result
      """
    Then analysis succeeds

  Scenario: Call procedure with table
    When the GQL query is analyzed:
      """
      CALL myProc(TABLE { MATCH (n) RETURN n })
      """
    Then analysis succeeds

  Scenario: Call subquery with schema
    When the GQL query is analyzed:
      """
      CALL { AT /myschemas/foo MATCH (n) RETURN n } RETURN 1
      """
    Then analysis succeeds

  Scenario: Call subquery with value
    When the GQL query is analyzed:
      """
      CALL { VALUE x = 1 MATCH (n) RETURN n } RETURN 1
      """
    Then analysis succeeds

  Scenario: Call subquery with schema creation
    When the GQL query is analyzed:
      """
      CALL { CREATE SCHEMA /s NEXT MATCH (n) SET n.prop = 1 } RETURN 1
      """
    Then analysis succeeds

  Scenario: Call with yield order limit
    When the GQL query is analyzed:
      """
      CALL algo.pageRank('User','FOLLOWS') YIELD nodeId, rank RETURN nodeId, rank ORDER BY rank DESC LIMIT 10
      """
    Then analysis succeeds
