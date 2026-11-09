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

Feature: Query composition

  Scenario: Standalone LET
    When the GQL query is analyzed:
      """
      LET two = 2
      """
    Then analysis succeeds

  Scenario: Let simple
    When the GQL query is analyzed:
      """
      MATCH (n:User) LET double_age = n.age * 2 RETURN n.name, double_age
      """
    Then analysis succeeds

  Scenario: Let multiple
    When the GQL query is analyzed:
      """
      MATCH (n:User) LET a = n.score + 10, b = n.score - 10 RETURN a, b
      """
    Then analysis succeeds

  Scenario: FILTER after MATCH
    When the GQL query is analyzed:
      """
      MATCH (n:User) FILTER (n:User) FILTER n.active = true
      """
    Then analysis succeeds

  Scenario: Let and filter
    When the GQL query is analyzed:
      """
      MATCH (n:User) LET bonus = n.salary * 0.2 FILTER bonus > 1000 FILTER bonus < 10 RETURN n.name, bonus
      """
    Then analysis succeeds

  Scenario: For simple
    When the GQL query is analyzed:
      """
      FOR x IN [1, 2, 3] RETURN x
      """
    Then analysis succeeds

  Scenario: For with match
    When the GQL query is analyzed:
      """
      MATCH (n:User) FOR tag IN n.tags RETURN n.name, tag
      """
    Then analysis succeeds

  Scenario: Full iso pipeline
    When the GQL query is analyzed:
      """
      MATCH (n:User) FOR email IN n.emails LET domain = RIGHT(email, 4) FILTER domain = '.com' RETURN n.name, email
      """
    Then analysis succeeds

  Scenario: Pipeline with intersect
    When the GQL query is analyzed:
      """
      RETURN 1 UNION RETURN 2 INTERSECT RETURN 2
      """
    Then analysis succeeds

  Scenario: Pipeline with otherwise
    When the GQL query is analyzed:
      """
      RETURN 1 EXCEPT RETURN 1 OTHERWISE RETURN 3
      """
    Then analysis succeeds

  Scenario: Pipeline with all set ops
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n.id UNION ALL MATCH (m) RETURN m.id EXCEPT ALL MATCH (x) RETURN x.id
      """
    Then analysis succeeds

  Scenario: Pipeline for filter let
    When the GQL query is analyzed:
      """
      FOR x IN [1,2,3,4,5] LET y = x * x FILTER y > 5 LET z = y + 10 RETURN x, y, z
      """
    Then analysis succeeds

  Scenario: Let filter let chain
    When the GQL query is analyzed:
      """
      MATCH (n:User) LET base = n.salary LET bonus = base * 0.2 FILTER bonus > 1000 LET total = base + bonus RETURN n.name, total
      """
    Then analysis succeeds

  Scenario: Pipeline match for filter
    When the GQL query is analyzed:
      """
      MATCH (n) FOR t IN n.tags FILTER t <> '' RETURN n.id, t
      """
    Then analysis succeeds

  Scenario: Standalone FILTER WHERE
    When the GQL query is analyzed:
      """
      FILTER WHERE TRUE RETURN 1
      """
    Then analysis succeeds

  Scenario: Filter where
    When the GQL query is analyzed:
      """
      MATCH (n) FILTER WHERE n.age > 21 RETURN n
      """
    Then analysis succeeds

  Scenario: LET in a query pipeline
    When the GQL query is analyzed:
      """
      MATCH (n) LET x = n.age RETURN x
      """
    Then analysis succeeds

  Scenario: For with ordinality
    When the GQL query is analyzed:
      """
      FOR x IN [1, 2, 3] WITH ORDINALITY i RETURN x, i
      """
    Then analysis succeeds

  Scenario: Subquery
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN VALUE { MATCH (m) RETURN m.x }
      """
    Then analysis succeeds

  Scenario: Yield
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) YIELD n, m RETURN n
      """
    Then analysis succeeds

  Scenario: Next
    When the GQL query is analyzed:
      """
      MATCH (n) NEXT RETURN n
      """
    Then analysis succeeds

  Scenario: Optional branch
    When the GQL query is analyzed:
      """
      OPTIONAL { MATCH (n) MATCH (m) } RETURN n, m
      """
    Then analysis succeeds

  Scenario: Exists
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE EXISTS { MATCH (n) MATCH (m) } RETURN n
      """
    Then analysis succeeds

  Scenario: For with offset
    When the GQL query is analyzed:
      """
      FOR x IN [1, 2, 3] WITH OFFSET i RETURN x, i
      """
    Then analysis succeeds
