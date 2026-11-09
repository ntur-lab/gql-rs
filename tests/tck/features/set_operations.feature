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

Feature: Set operations

  Scenario: Union
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.name UNION MATCH (m:Admin) RETURN m.name
      """
    Then analysis succeeds

  Scenario: Union all
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.city AS city UNION ALL MATCH (c:Company) RETURN c.city AS city
      """
    Then analysis succeeds

  Scenario: Union distinct
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.city AS city UNION DISTINCT MATCH (c:Company) RETURN c.city AS city
      """
    Then analysis succeeds

  Scenario: INTERSECT projected aliases
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[:LIKES]->(p:Post) RETURN p.id AS post_id INTERSECT MATCH (m:Admin)-[:LIKES]->(p:Post) RETURN p.id AS post_id
      """
    Then analysis succeeds

  Scenario: INTERSECT ALL projected aliases
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u.id AS id INTERSECT ALL MATCH (a:Admin) RETURN a.id AS id
      """
    Then analysis succeeds

  Scenario: Intersect distinct
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u.id AS id INTERSECT DISTINCT MATCH (a:Admin) RETURN a.id AS id
      """
    Then analysis succeeds

  Scenario: Except distinct
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.name AS name EXCEPT DISTINCT MATCH (a:Admin) RETURN a.name AS name
      """
    Then analysis succeeds

  Scenario: EXCEPT projected aliases
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.name AS name EXCEPT MATCH (a:Admin) RETURN a.name AS name
      """
    Then analysis succeeds

  Scenario: EXCEPT ALL projected aliases
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u.name AS name EXCEPT ALL MATCH (a:Admin) RETURN a.name AS name
      """
    Then analysis succeeds

  Scenario: Otherwise
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.name OTHERWISE MATCH (m:Admin) RETURN m.name
      """
    Then analysis succeeds

  Scenario: Chain union otherwise
    When the GQL query is analyzed:
      """
      MATCH (a:Alpha) RETURN a.id AS id UNION MATCH (b:Beta) RETURN b.id AS id OTHERWISE MATCH (c:Gamma) RETURN c.id AS id
      """
    Then analysis succeeds

  Scenario: Chain otherwise except
    When the GQL query is analyzed:
      """
      MATCH (a:Alpha) RETURN a.id AS id OTHERWISE MATCH (b:Beta) RETURN b.id AS id EXCEPT MATCH (c:Gamma) RETURN c.id AS id
      """
    Then analysis succeeds

  Scenario: Return union
    When the GQL query is analyzed:
      """
      RETURN 1 UNION RETURN 2
      """
    Then analysis succeeds

  Scenario: Return otherwise
    When the GQL query is analyzed:
      """
      RETURN 1 OTHERWISE RETURN 2
      """
    Then analysis succeeds

  Scenario: Chain long mixed
    When the GQL query is analyzed:
      """
      MATCH (a:A) RETURN a.id AS id UNION ALL MATCH (b:B) RETURN b.id AS id EXCEPT DISTINCT MATCH (c:C) RETURN c.id AS id INTERSECT MATCH (d:D) RETURN d.id AS id
      """
    Then analysis succeeds

  Scenario: Next
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u NEXT MATCH (p:Post) RETURN p
      """
    Then analysis succeeds

  Scenario: Next with yield
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u.id AS id NEXT YIELD id MATCH (p:Post) WHERE p.author_id = id RETURN p
      """
    Then analysis succeeds

  Scenario: Next chain
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u NEXT MATCH (p:Post) RETURN p NEXT MATCH (c:Comment) RETURN c
      """
    Then analysis succeeds

  Scenario: Next with union
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u.name UNION MATCH (a:Admin) RETURN a.name NEXT MATCH (x:Audit) RETURN x
      """
    Then analysis succeeds

  Scenario: Next with set
    When the GQL query is analyzed:
      """
      MATCH (u:User) RETURN u NEXT MATCH (u:User) SET u.active = true
      """
    Then analysis succeeds

  Scenario: Union then except
    When the GQL query is analyzed:
      """
      RETURN 1 UNION RETURN 2 EXCEPT RETURN 1
      """
    Then analysis succeeds

  Scenario: Intersect then union
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n.id INTERSECT MATCH (m) RETURN m.id UNION MATCH (x) RETURN x.id
      """
    Then analysis succeeds

  Scenario: Mixed precedence all
    When the GQL query is analyzed:
      """
      RETURN 1 UNION ALL RETURN 1 EXCEPT DISTINCT RETURN 1 INTERSECT RETURN 1
      """
    Then analysis succeeds

  Scenario: Match with otherwise
    When the GQL query is analyzed:
      """
      MATCH (n:Dog) RETURN n OTHERWISE MATCH (m:Cat) RETURN m
      """
    Then analysis succeeds

  Scenario: Union match
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n UNION MATCH (m) RETURN m
      """
    Then analysis succeeds

  Scenario: EXCEPT node results
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n EXCEPT MATCH (m) RETURN m
      """
    Then analysis succeeds

  Scenario: EXCEPT ALL node results
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n EXCEPT ALL MATCH (m) RETURN m
      """
    Then analysis succeeds

  Scenario: INTERSECT node results
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n INTERSECT MATCH (m) RETURN m
      """
    Then analysis succeeds

  Scenario: INTERSECT ALL node results
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n INTERSECT ALL MATCH (m) RETURN m
      """
    Then analysis succeeds
