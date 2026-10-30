# Copyright (c) 2026 NTUR LAB S.r.l.
#
# This file is part of NTURDB.
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

Feature: Data modification

  Scenario: Set single property
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.id = 1 SET n.age = 31 RETURN n
      """
    Then analysis succeeds

  Scenario: Set boolean
    When the GQL query is analyzed:
      """
      MATCH (n:User) SET n.verified = true
      """
    Then analysis succeeds

  Scenario: Set multiple elements
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[r:FOLLOWS]->(m:User) SET n.active = true, r.watched_at = '2024-03-04', m.follower_count = 100
      """
    Then analysis succeeds

  Scenario: Set increment
    When the GQL query is analyzed:
      """
      MATCH (n:User) SET n.login_count = n.login_count + 1
      """
    Then analysis succeeds

  Scenario: Set concatenation
    When the GQL query is analyzed:
      """
      MATCH (n:User) SET n.full_name = n.first_name || ' ' || n.last_name
      """
    Then analysis succeeds

  Scenario: Set with case
    When the GQL query is analyzed:
      """
      MATCH (n:User) SET n.status = CASE WHEN n.age >= 18 THEN 'Adult' ELSE 'Minor' END
      """
    Then analysis succeeds

  Scenario: Set with function
    When the GQL query is analyzed:
      """
      MATCH (n:Article) SET n.title_length = CHAR_LENGTH(n.title)
      """
    Then analysis succeeds

  Scenario: Set null
    When the GQL query is analyzed:
      """
      MATCH (n:User) SET n.temporary_token = NULL
      """
    Then analysis succeeds

  Scenario: Set single label
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.premium = true SET n:PremiumUser
      """
    Then analysis succeeds

  Scenario: Remove property
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.id = 1 REMOVE n.temporary_field
      """
    Then analysis succeeds

  Scenario: Remove label
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.status = 'inactive' REMOVE n:Active
      """
    Then analysis succeeds

  Scenario: Remove relationship
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[r:TEMPORARY]->() REMOVE r.temporary_field
      """
    Then analysis succeeds

  Scenario: Remove deprecated label
    When the GQL query is analyzed:
      """
      MATCH (n) REMOVE n:Deprecated
      """
    Then analysis succeeds

  Scenario: Remove from banned
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.banned REMOVE n:Trusted
      """
    Then analysis succeeds

  Scenario: Remove a node label
    When the GQL query is analyzed:
      """
      MATCH (n:User) REMOVE n:Active
      """
    Then analysis succeeds

  Scenario: Remove edge property
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[r:FOLLOWS]->() REMOVE r.watched_at
      """
    Then analysis succeeds

  Scenario: Remove multiple properties
    When the GQL query is analyzed:
      """
      MATCH (n:User) REMOVE n.temp1, n.temp2
      """
    Then analysis succeeds

  Scenario: Remove mixed
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.banned REMOVE n.trusted_score, n:Trusted
      """
    Then analysis succeeds

  Scenario: Delete node
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.id = 1 DELETE n
      """
    Then analysis succeeds

  Scenario: Delete detach
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.status = 'deleted' DETACH DELETE n
      """
    Then analysis succeeds

  Scenario: Delete relationship
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[r:KNOWS]-(m) DELETE r
      """
    Then analysis succeeds

  Scenario: Delete spam
    When the GQL query is analyzed:
      """
      MATCH (n:Spam) DETACH DELETE n
      """
    Then analysis succeeds

  Scenario: Delete nodetach
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.banned NODETACH DELETE n
      """
    Then analysis succeeds

  Scenario: Delete multiple elements
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[r:CREATED]->(p:Post) WHERE p.flagged DELETE r, p
      """
    Then analysis succeeds

  Scenario: Delete detach multiple
    When the GQL query is analyzed:
      """
      MATCH (n:Spammer), (m:Bot) DETACH DELETE n, m
      """
    Then analysis succeeds

  Scenario: Insert single node
    When the GQL query is analyzed:
      """
      INSERT (n:User {name: 'John', age: 30})
      """
    Then analysis succeeds

  Scenario: Insert multiple nodes
    When the GQL query is analyzed:
      """
      INSERT (n:User {name: 'Alice', email: 'alice@example.com'}), (m:Company {name: 'TechCorp'})
      """
    Then analysis succeeds

  Scenario: Insert with properties
    When the GQL query is analyzed:
      """
      INSERT (n:User {id: 1, name: 'Bob', active: true})
      """
    Then analysis succeeds

  Scenario: Insert with relationship
    When the GQL query is analyzed:
      """
      INSERT (n:User {name: 'Charlie'})-[r:WORKS_AT]->(m:Company {name: 'StartupXYZ'})
      """
    Then analysis succeeds

  Scenario: Insert people knows
    When the GQL query is analyzed:
      """
      INSERT (a:Person {name: 'Diana'})-[r:KNOWS]->(b:Person {name: 'Eve'})
      """
    Then analysis succeeds

  Scenario: Insert minimal node
    When the GQL query is analyzed:
      """
      INSERT ()
      """
    Then analysis succeeds

  Scenario: Insert node only var
    When the GQL query is analyzed:
      """
      INSERT (n)
      """
    Then analysis succeeds

  Scenario: Insert node only label
    When the GQL query is analyzed:
      """
      INSERT (:User)
      """
    Then analysis succeeds

  Scenario: Insert node only props
    When the GQL query is analyzed:
      """
      INSERT ({name: 'OnlyProps'})
      """
    Then analysis succeeds

  Scenario: Insert multi label amp
    When the GQL query is analyzed:
      """
      INSERT (n:User&Admin {name: 'Root'})
      """
    Then analysis succeeds

  Scenario: Insert left edge
    When the GQL query is analyzed:
      """
      INSERT (a:Person)<-[r:MANAGES]-(b:Person)
      """
    Then analysis succeeds

  Scenario: Insert undirected edge
    When the GQL query is analyzed:
      """
      INSERT (a:Person)~[r:KNOWS]~(b:Person)
      """
    Then analysis succeeds

  Scenario: Insert empty edge filler right
    When the GQL query is analyzed:
      """
      INSERT (a)-[]->(b)
      """
    Then analysis succeeds

  Scenario: Insert empty edge filler left
    When the GQL query is analyzed:
      """
      INSERT (a)<-[]-(b)
      """
    Then analysis succeeds

  Scenario: Insert empty edge filler undirected
    When the GQL query is analyzed:
      """
      INSERT (a)~[]~(b)
      """
    Then analysis succeeds

  Scenario: Insert edge props
    When the GQL query is analyzed:
      """
      INSERT (a:User)-[r:LIKES {score: 10, active: true}]->(b:Post)
      """
    Then analysis succeeds

  Scenario: Insert long chain
    When the GQL query is analyzed:
      """
      INSERT (a)-[r1:REL1]->(b)-[r2:REL2]->(c)
      """
    Then analysis succeeds

  Scenario: Insert multiple paths mixed
    When the GQL query is analyzed:
      """
      INSERT (a:User {id: 1}), (b:User {id: 2})-[r:FOLLOWS]->(c:User {id: 3})
      """
    Then analysis succeeds

  Scenario: Set computed multiple
    When the GQL query is analyzed:
      """
      MATCH (n:User) SET n.full_name = n.first_name || ' ' || n.last_name, n.updated = CURRENT_TIMESTAMP
      """
    Then analysis succeeds

  Scenario: Set if null
    When the GQL query is analyzed:
      """
      MATCH (n:User) SET n.email = COALESCE(n.email, 'no-email@unknown.com')
      """
    Then analysis succeeds

  Scenario: Remove edge property multiple
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[r:FOLLOWS]->(m) REMOVE r.watched_at, r.score
      """
    Then analysis succeeds

  Scenario: Remove label conditional
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.suspended REMOVE n:Active, n:Verified
      """
    Then analysis succeeds

  Scenario: Insert complex chain
    When the GQL query is analyzed:
      """
      INSERT (a:Person {name: 'Alice'})-[r1:FOLLOWS]->(b:Person {name: 'Bob'})-[r2:FOLLOWS]->(c:Person {name: 'Charlie'})
      """
    Then analysis succeeds

  Scenario: Delete value
    When the GQL query is analyzed:
      """
      DELETE VALUE { MATCH (n:Temp) RETURN n }
      """
    Then analysis succeeds

  Scenario: Set property
    When the GQL query is analyzed:
      """
      MATCH (n) SET n.prop = 1
      """
    Then analysis succeeds

  Scenario: Use graph match set
    When the GQL query is analyzed:
      """
      USE my_graph MATCH (n:User) SET n.flag = true
      """
    Then analysis succeeds

  Scenario: Add label
    When the GQL query is analyzed:
      """
      MATCH (n:Person) SET n:Employee
      """
    Then analysis succeeds

  Scenario: Delete case
    When the GQL query is analyzed:
      """
      MATCH (n) DELETE CASE WHEN n.expired THEN n ELSE null END
      """
    Then analysis succeeds
