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

Feature: Graph selection and SELECT statements

  Scenario: Use graph clause with match
    When the GQL query is analyzed:
      """
      USE my_graph MATCH (n:User) RETURN n
      """
    Then analysis succeeds

  Scenario: Use current graph clause
    When the GQL query is analyzed:
      """
      USE CURRENT_GRAPH MATCH (n) RETURN n
      """
    Then analysis succeeds

  Scenario: Select without from
    When the GQL query is analyzed:
      """
      SELECT 1 AS one
      """
    Then analysis succeeds

  Scenario: Select group by having
    When the GQL query is analyzed:
      """
      SELECT n.city AS city, COUNT(*) AS total FROM my_graph MATCH (n:User) GROUP BY city HAVING COUNT(*) > 0
      """
    Then analysis succeeds

  Scenario: Select with having avg
    When the GQL query is analyzed:
      """
      SELECT n.department AS dept, AVG(n.salary) AS avg_sal FROM my_graph MATCH (n:Employee) GROUP BY dept HAVING AVG(n.salary) > 60000
      """
    Then analysis succeeds

  Scenario: Select with subquery in from
    When the GQL query is analyzed:
      """
      SELECT name, a.age FROM { MATCH (n:Person), (a:Actor) RETURN n.name, a }
      """
    Then analysis succeeds
