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

Feature: Graph matching and patterns

  Scenario: Where equals
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.name = $param1 RETURN n LIMIT $myLimit
      """
    Then analysis succeeds

  Scenario: Where greater than
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age > 18 RETURN n
      """
    Then analysis succeeds

  Scenario: Where greater equals
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age >= 18 RETURN n
      """
    Then analysis succeeds

  Scenario: Where less than
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age < 18 RETURN n
      """
    Then analysis succeeds

  Scenario: Where less equals
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age <= 18 RETURN n
      """
    Then analysis succeeds

  Scenario: Where not equals
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age <> 18 RETURN n
      """
    Then analysis succeeds

  Scenario: Where not
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE NOT n.verified RETURN n
      """
    Then analysis succeeds

  Scenario: Where and
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age > 18 AND n.verified RETURN n
      """
    Then analysis succeeds

  Scenario: Where or
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age < 18 OR n.verified RETURN n
      """
    Then analysis succeeds

  Scenario: WHERE XOR with property predicates
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age < 18 XOR n.verified RETURN n
      """
    Then analysis succeeds

  Scenario: Where value is false
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.status IS FALSE RETURN n
      """
    Then analysis succeeds

  Scenario: Where is null
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.status IS NULL RETURN n
      """
    Then analysis succeeds

  Scenario: Where is unknown
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.optional_field IS UNKNOWN RETURN n
      """
    Then analysis succeeds

  Scenario: Where is labeled 1
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n IS LABELED User RETURN n
      """
    Then analysis succeeds

  Scenario: Where is labeled 2
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n IS LABELED (User&Admin) RETURN n
      """
    Then analysis succeeds

  Scenario: Where is labeled 3
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n IS LABELED (User|Admin) RETURN n
      """
    Then analysis succeeds

  Scenario: Where is labeled 4
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n IS LABELED (User&!Admin) RETURN n
      """
    Then analysis succeeds

  Scenario: Where is not true
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.status IS NOT TRUE RETURN n
      """
    Then analysis succeeds

  Scenario: Where is not false
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.status IS NOT FALSE RETURN n
      """
    Then analysis succeeds

  Scenario: Where is not null
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.status IS NOT NULL RETURN n
      """
    Then analysis succeeds

  Scenario: Where is not unknown
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.optional_field IS NOT UNKNOWN RETURN n
      """
    Then analysis succeeds

  Scenario: Where expression1
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age > 18 AND (n.verified OR n.trusted) RETURN n
      """
    Then analysis succeeds

  Scenario: Where expression2
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE (n.age > 18 AND n.verified) OR (n.age < 18 AND n.trusted) RETURN n
      """
    Then analysis succeeds

  Scenario: Where expression3
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.age > 18 AND (n.verified OR (n.trusted AND n.score > 100)) RETURN n
      """
    Then analysis succeeds

  Scenario: Where expression4
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE (n.age > 18 AND n.verified) OR (n.age < 18 AND (n.trusted OR n.score > 100)) AND (n.status IS NULL OR n.optional_field IS NOT UNKNOWN) RETURN n
      """
    Then analysis succeeds

  Scenario: Array constructor
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN [n.name, n.email, n.phone]
      """
    Then analysis succeeds

  Scenario: Numeric array
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN [1.0, 2.5, 3.7]
      """
    Then analysis succeeds

  Scenario: Numeric literal markers accept standard case variants
    When the GQL query is analyzed:
      """
      RETURN 1e2, 1E2, 1.5f, 1.5D, 0xAf
      """
    Then analysis succeeds

  Scenario: Size function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SIZE([n.name, n.email])
      """
    Then analysis succeeds

  Scenario: Return alone
    When the GQL query is analyzed:
      """
      RETURN 1
      """
    Then analysis succeeds

  Scenario: Match return
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.name, n.email, n.age
      """
    Then analysis succeeds

  Scenario: Distinct single
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN DISTINCT n.city
      """
    Then analysis succeeds

  Scenario: Distinct node
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN DISTINCT n:User
      """
    Then analysis succeeds

  Scenario: Simple alias
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n AS user
      """
    Then analysis succeeds

  Scenario: Multiple aliases
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.name AS username, n.email AS email
      """
    Then analysis succeeds

  Scenario: Order by implicit
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.name
      """
    Then analysis succeeds

  Scenario: Order by ascending word
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.name ASCENDING
      """
    Then analysis succeeds

  Scenario: Order by descending word
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.age DESCENDING
      """
    Then analysis succeeds

  Scenario: Order by expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.age * 2 DESC
      """
    Then analysis succeeds

  Scenario: Order by function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY ELEMENT_ID(n) ASC
      """
    Then analysis succeeds

  Scenario: Order by implicit nulls first
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.score NULLS FIRST
      """
    Then analysis succeeds

  Scenario: Simple limit
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n LIMIT 10
      """
    Then analysis succeeds

  Scenario: Limit zero
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n LIMIT 0
      """
    Then analysis succeeds

  Scenario: Skip zero
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n SKIP 0
      """
    Then analysis succeeds

  Scenario: Offset limit
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n OFFSET 5 LIMIT 10
      """
    Then analysis succeeds

  Scenario: Skip
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n SKIP 5
      """
    Then analysis succeeds

  Scenario: Skip limit
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n SKIP 5 LIMIT 10
      """
    Then analysis succeeds

  Scenario: Limit parameter
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n LIMIT $myLimit
      """
    Then analysis succeeds

  Scenario: Offset parameter
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n OFFSET $pageOffset
      """
    Then analysis succeeds

  Scenario: Skip parameter limit
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n SKIP $skipVal LIMIT $limitVal
      """
    Then analysis succeeds

  Scenario: Order limit
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.name LIMIT 20
      """
    Then analysis succeeds

  Scenario: Order offset limit full
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.score DESC OFFSET 10 LIMIT 5
      """
    Then analysis succeeds

  Scenario: Order skip limit full
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n ORDER BY n.score ASC SKIP 100 LIMIT 50
      """
    Then analysis succeeds

  Scenario: Group by simple
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.city AS city, COUNT(n) GROUP BY city
      """
    Then analysis succeeds

  Scenario: Group by multiple keys
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.department AS dept, n.role AS role, COUNT(n) GROUP BY dept, role
      """
    Then analysis succeeds

  Scenario: Group by expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN (n.age / 10) AS decade, COUNT(n) GROUP BY decade
      """
    Then analysis succeeds

  Scenario: Group empty
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COUNT(n) AS total_users, AVG(n.age) AS average_age GROUP BY ()
      """
    Then analysis succeeds

  Scenario: Simple path
    When the GQL query is analyzed:
      """
      MATCH (n)-[r]-(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Path length 1
    When the GQL query is analyzed:
      """
      MATCH (n)-[r:KNOWS]->{1,1}(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Path length 2
    When the GQL query is analyzed:
      """
      MATCH (n)-[]->(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Path range
    When the GQL query is analyzed:
      """
      MATCH (n)-[]->{1,3}(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Path min 1
    When the GQL query is analyzed:
      """
      MATCH (n)-[]->{1,}(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Path 3 nodes
    When the GQL query is analyzed:
      """
      MATCH (a)-[r1]->(b)-[r2]->(c) RETURN a, b, c
      """
    Then analysis succeeds

  Scenario: Walk mode
    When the GQL query is analyzed:
      """
      MATCH WALK (n:User)-[]->{1,3}(m:Content) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Trail mode
    When the GQL query is analyzed:
      """
      MATCH TRAIL (n:User)-[]->{1,4}(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Simple mode
    When the GQL query is analyzed:
      """
      MATCH ACYCLIC (n:User)-[]->{1,5}(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Case no else
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CASE WHEN n.age >= 18 THEN 'Adult' END AS adult_status
      """
    Then analysis succeeds

  Scenario: Case returning expressions
    When the GQL query is analyzed:
      """
      MATCH (n:Employee) RETURN CASE WHEN n.bonus_eligible THEN n.salary * 1.2 ELSE n.salary END AS final_salary
      """
    Then analysis succeeds

  Scenario: Case complex logic
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CASE WHEN n.age >= 18 AND n.country = 'IT' THEN 'Adult Italian' ELSE 'Other' END
      """
    Then analysis succeeds

  Scenario: Case in aggregate
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SUM(CASE n.status WHEN 'active' THEN 1 ELSE 0 END) AS total_active_users
      """
    Then analysis succeeds

  Scenario: Case in where
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE CASE n.role WHEN 'admin' THEN true ELSE false END RETURN n
      """
    Then analysis succeeds

  Scenario: Case nested case
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CASE WHEN n.active THEN CASE WHEN n.premium THEN 'Active Premium' ELSE 'Active Free' END ELSE 'Inactive' END AS full_status
      """
    Then analysis succeeds

  Scenario: Double match case
    When the GQL query is analyzed:
      """
      MATCH (p:Person) OPTIONAL MATCH (p)-[:WORKS_FOR]->(c:Company) RETURN p.name, CASE WHEN c IS NULL THEN 'Freelance' WHEN c.industry = 'Tech' THEN 'Tech Worker' ELSE 'Other Industry' END AS employment_status
      """
    Then analysis succeeds

  Scenario: Exists basic
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE EXISTS(MATCH (n)-[:FOLLOWS]->()) RETURN n
      """
    Then analysis succeeds

  Scenario: Not exists
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE NOT EXISTS(MATCH (n)-[r:BLOCKED_BY]->()) RETURN n
      """
    Then analysis succeeds

  Scenario: Exists purchased
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE EXISTS(MATCH (n)-[r:PURCHASED]->(p:Premium)) RETURN n
      """
    Then analysis succeeds

  Scenario: Where exists and
    When the GQL query is analyzed:
      """
      MATCH (p:Person) WHERE EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Python'})} AND EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Java'})} RETURN p.name
      """
    Then analysis succeeds

  Scenario: Where not exists
    When the GQL query is analyzed:
      """
      MATCH (p:Person) WHERE NOT EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Cobol'})} RETURN p.name
      """
    Then analysis succeeds

  Scenario: Where exists or
    When the GQL query is analyzed:
      """
      MATCH (p:Person) WHERE EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Python'})} OR EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Java'})} RETURN p.name
      """
    Then analysis succeeds

  Scenario: Where exists xor
    When the GQL query is analyzed:
      """
      MATCH (p:Person) WHERE EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Python'})} XOR EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Java'})} RETURN p.name
      """
    Then analysis succeeds

  Scenario: Where exists and not exists
    When the GQL query is analyzed:
      """
      MATCH (p:Person) WHERE EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Python'})} AND NOT EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Java'})} RETURN p.name
      """
    Then analysis succeeds

  Scenario: Where exists or not exists
    When the GQL query is analyzed:
      """
      MATCH (p:Person) WHERE EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Python'})} OR NOT EXISTS {MATCH (p)-[:HAS_SKILL]->(:Skill {name: 'Java'})} RETURN p.name
      """
    Then analysis succeeds

  Scenario: Where not parenthesized
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE NOT (n.age > 18 AND n.verified) RETURN n
      """
    Then analysis succeeds

  Scenario: Where multiple not
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE NOT n.verified AND NOT n.suspended RETURN n
      """
    Then analysis succeeds

  Scenario: Where xor with and
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE (n.age < 18 XOR n.verified) AND (n.status IS NOT NULL) RETURN n
      """
    Then analysis succeeds

  Scenario: Diffrent edges
    When the GQL query is analyzed:
      """
      MATCH DIFFERENT EDGES (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Repeatable elements
    When the GQL query is analyzed:
      """
      MATCH REPEATABLE ELEMENTS (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Path matching
    When the GQL query is analyzed:
      """
      MATCH p = (n)-[e]->(m) RETURN p
      """
    Then analysis succeeds

  Scenario: All paths
    When the GQL query is analyzed:
      """
      MATCH ALL PATHS (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Keep walk
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) KEEP WALK RETURN n
      """
    Then analysis succeeds

  Scenario: Keep all paths
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) KEEP ALL PATHS RETURN n
      """
    Then analysis succeeds

  Scenario: All walk paths
    When the GQL query is analyzed:
      """
      MATCH ALL WALK PATHS (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: All trail paths
    When the GQL query is analyzed:
      """
      MATCH ALL TRAIL PATHS (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: All simple paths
    When the GQL query is analyzed:
      """
      MATCH ALL SIMPLE PATHS (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: All acyclic paths
    When the GQL query is analyzed:
      """
      MATCH ALL ACYCLIC PATHS (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Any
    When the GQL query is analyzed:
      """
      MATCH ANY (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: All shortest
    When the GQL query is analyzed:
      """
      MATCH ALL SHORTEST (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Any shortest
    When the GQL query is analyzed:
      """
      MATCH ANY SHORTEST (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Shortest 5
    When the GQL query is analyzed:
      """
      MATCH SHORTEST 5 (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Shortest 3 group
    When the GQL query is analyzed:
      """
      MATCH SHORTEST 3 GROUP (n)-[e]->(m) RETURN n
      """
    Then analysis succeeds

  Scenario: Union
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b) |+| (c)-[e2]->(d) RETURN a
      """
    Then analysis succeeds

  Scenario: Union star
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b)* |+| (c)-[e2]->(d) RETURN a
      """
    Then analysis succeeds

  Scenario: Intersection
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b) | (c)-[e2]->(d) RETURN a
      """
    Then analysis succeeds

  Scenario: Intersection star
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b)* | (c)-[e2]->(d) RETURN a
      """
    Then analysis succeeds

  Scenario: Optional
    When the GQL query is analyzed:
      """
      MATCH ((a)-[e]->(b))* RETURN a
      """
    Then analysis succeeds

  Scenario: Range
    When the GQL query is analyzed:
      """
      MATCH (a)-[e]->{1,5}(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Zero or one
    When the GQL query is analyzed:
      """
      MATCH (a)-[e]->(b)? RETURN a
      """
    Then analysis succeeds

  Scenario: Parenthesized
    When the GQL query is analyzed:
      """
      MATCH ((a)-[e]->(b)) RETURN a
      """
    Then analysis succeeds

  Scenario: Regex
    When the GQL query is analyzed:
      """
      MATCH ~/ Person /~ RETURN 1
      """
    Then analysis succeeds

  Scenario: Where
    When the GQL query is analyzed:
      """
      MATCH (a)-[e WHERE a.age > 5]->(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Traversal
    When the GQL query is analyzed:
      """
      MATCH (a)~[r]~(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Directed
    When the GQL query is analyzed:
      """
      MATCH (a)->(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Undirected
    When the GQL query is analyzed:
      """
      MATCH (a)~(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Bound
    When the GQL query is analyzed:
      """
      MATCH (a)(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Edge
    When the GQL query is analyzed:
      """
      MATCH -[r]-> RETURN r
      """
    Then analysis succeeds

  Scenario: Path
    When the GQL query is analyzed:
      """
      MATCH (p = (a)-[e]->(b)) RETURN a
      """
    Then analysis succeeds

  Scenario: Walk
    When the GQL query is analyzed:
      """
      MATCH (WALK (a)-[e]->(b)) RETURN a
      """
    Then analysis succeeds

  Scenario: Filtered path
    When the GQL query is analyzed:
      """
      MATCH ((a)-[e]->(b) WHERE e.weight > 5) RETURN a
      """
    Then analysis succeeds

  Scenario: Nested paths
    When the GQL query is analyzed:
      """
      MATCH (a)-[e]->((b)-[f]->(c) WHERE a.age > 5) RETURN a
      """
    Then analysis succeeds

  Scenario: Labeled path
    When the GQL query is analyzed:
      """
      MATCH (n:Person){1,5} RETURN n
      """
    Then analysis succeeds

  Scenario: Range labeled
    When the GQL query is analyzed:
      """
      MATCH (n:Person){3,} RETURN n
      """
    Then analysis succeeds

  Scenario: Unlabeled path
    When the GQL query is analyzed:
      """
      MATCH (n:%) RETURN n
      """
    Then analysis succeeds

  Scenario: Regex edge
    When the GQL query is analyzed:
      """
      MATCH -/ Person /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Regex edge union
    When the GQL query is analyzed:
      """
      MATCH -/ Person | Employee /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Regex edge multiset alternation
    When the GQL query is analyzed:
      """
      MATCH -/ Person |+| Employee /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Regex edge negation
    When the GQL query is analyzed:
      """
      MATCH -/ <~ Person /- RETURN 1
      """
    Then analysis succeeds

  Scenario: Regex edge partial
    When the GQL query is analyzed:
      """
      MATCH -/ < Person /- RETURN 1
      """
    Then analysis succeeds

  Scenario: Element id
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN ELEMENT_ID(n)
      """
    Then analysis succeeds

  Scenario: Directed edge
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) WHERE e IS DIRECTED RETURN e
      """
    Then analysis succeeds

  Scenario: Directed edge not
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]-(m) WHERE e IS NOT DIRECTED RETURN e
      """
    Then analysis succeeds

  Scenario: Labeled node
    When the GQL query is analyzed:
      """
      MATCH (n:Person) WHERE n:Person RETURN n
      """
    Then analysis succeeds

  Scenario: Source node
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) WHERE n IS SOURCE OF e RETURN n
      """
    Then analysis succeeds

  Scenario: Not source node
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) WHERE n IS NOT SOURCE OF e RETURN n
      """
    Then analysis succeeds

  Scenario: Destination node
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) WHERE m IS DESTINATION OF e RETURN m
      """
    Then analysis succeeds

  Scenario: Not destination node
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) WHERE m IS NOT DESTINATION OF e RETURN m
      """
    Then analysis succeeds

  Scenario: All different
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b), (c)-[e2]->(d) WHERE ALL_DIFFERENT(a,c) RETURN a
      """
    Then analysis succeeds

  Scenario: All different three
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b), (c)-[e2]->(d), (x)-[e3]->(y) WHERE ALL_DIFFERENT(a,c,x) RETURN a
      """
    Then analysis succeeds

  Scenario: Same
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b), (c)-[e2]->(d) WHERE SAME(a,c) RETURN a
      """
    Then analysis succeeds

  Scenario: Same three
    When the GQL query is analyzed:
      """
      MATCH (a)-[e1]->(b), (c)-[e2]->(d), (x)-[e3]->(y) WHERE SAME(a,c,x) RETURN a
      """
    Then analysis succeeds

  Scenario: Property exists
    When the GQL query is analyzed:
      """
      MATCH (n:Person) WHERE PROPERTY_EXISTS(n, name) RETURN n
      """
    Then analysis succeeds

  Scenario: Order by
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n ORDER BY n.name NULLS FIRST
      """
    Then analysis succeeds

  Scenario: Where literal
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE 'hello' = 1 RETURN n
      """
    Then analysis succeeds

  Scenario: Is typed
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INT32 RETURN n
      """
    Then analysis succeeds

  Scenario: Typed integer
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INTEGER RETURN n
      """
    Then analysis succeeds

  Scenario: Order by node
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) RETURN n ORDER BY m
      """
    Then analysis succeeds

  Scenario: Path equality
    When the GQL query is analyzed:
      """
      MATCH p = (a)-[e]->(b), q = (c)-[f]->(d) WHERE p = q RETURN p
      """
    Then analysis succeeds

  Scenario: Long property name
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
      """
    Then analysis succeeds

  Scenario: Return table
    When the GQL query is analyzed:
      """
      RETURN TABLE { MATCH (n) RETURN n }
      """
    Then analysis succeeds

  Scenario: Let alone
    When the GQL query is analyzed:
      """
      LET x = 1 RETURN x
      """
    Then analysis succeeds

  Scenario: Return let
    When the GQL query is analyzed:
      """
      RETURN LET x = 1 IN x + 1 END
      """
    Then analysis succeeds

  Scenario: Match path with name
    When the GQL query is analyzed:
      """
      MATCH p = (a)-[]->(b) RETURN PATH [a] || PATH [b]
      """
    Then analysis succeeds

  Scenario: WHERE XOR with boolean literals
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE TRUE XOR FALSE RETURN n
      """
    Then analysis succeeds

  Scenario: Return group by
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n.name AS name, COUNT(*) AS cnt GROUP BY name ORDER BY COUNT(*)
      """
    Then analysis succeeds

  Scenario: Order by offset
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n ORDER BY n.name OFFSET 10
      """
    Then analysis succeeds

  Scenario: Order by limit
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n ORDER BY n.name LIMIT 10
      """
    Then analysis succeeds

  Scenario: Order by concatenation
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n.name AS name ORDER BY n.name || n.surname
      """
    Then analysis succeeds

  Scenario: Group by
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n GROUP BY n
      """
    Then analysis succeeds

  Scenario: Order by alias
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n.name AS alias ORDER BY alias
      """
    Then analysis succeeds

  Scenario: Join group by
    When the GQL query is analyzed:
      """
      MATCH (n)-[e]->(m) RETURN n.name, COUNT(*) GROUP BY n
      """
    Then analysis succeeds

  Scenario: Match uint8
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UINT8 RETURN n
      """
    Then analysis succeeds

  Scenario: Match int8
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INT8 RETURN n
      """
    Then analysis succeeds

  Scenario: Match uint16
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UINT16 RETURN n
      """
    Then analysis succeeds

  Scenario: Match int16
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INT16 RETURN n
      """
    Then analysis succeeds

  Scenario: Match usmallint
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED USMALLINT RETURN n
      """
    Then analysis succeeds

  Scenario: Match uint32
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UINT32 RETURN n
      """
    Then analysis succeeds

  Scenario: Match uint
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UINT RETURN n
      """
    Then analysis succeeds

  Scenario: Match int
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INT(10) RETURN n
      """
    Then analysis succeeds

  Scenario: Match ubigint
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UBIGINT RETURN n
      """
    Then analysis succeeds

  Scenario: Match uint64
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UINT64 RETURN n
      """
    Then analysis succeeds

  Scenario: Match int64
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INT64 RETURN n
      """
    Then analysis succeeds

  Scenario: Match uint128
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UINT128 RETURN n
      """
    Then analysis succeeds

  Scenario: Match int128
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INT128 RETURN n
      """
    Then analysis succeeds

  Scenario: Match uint256
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED UINT256 RETURN n
      """
    Then analysis succeeds

  Scenario: Match int256
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED INT256 RETURN n
      """
    Then analysis succeeds

  Scenario: Match decimal
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED DECIMAL(10) RETURN n
      """
    Then analysis succeeds

  Scenario: Match smallint
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED SMALLINT RETURN n
      """
    Then analysis succeeds

  Scenario: Match bigint
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED BIGINT RETURN n
      """
    Then analysis succeeds

  Scenario: Match float16
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.age IS TYPED FLOAT16 RETURN n
      """
    Then analysis succeeds

  Scenario: Match float32
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED FLOAT32 RETURN n
      """
    Then analysis succeeds

  Scenario: Match float
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED FLOAT(10) RETURN n
      """
    Then analysis succeeds

  Scenario: Match real
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED REAL RETURN n
      """
    Then analysis succeeds

  Scenario: Match float64
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED FLOAT64 RETURN n
      """
    Then analysis succeeds

  Scenario: Match float128
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED FLOAT128 RETURN n
      """
    Then analysis succeeds

  Scenario: Match float256
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED FLOAT256 RETURN n
      """
    Then analysis succeeds

  Scenario: Match string
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED STRING(1, 100) RETURN n
      """
    Then analysis succeeds

  Scenario: Match string long
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED STRING(100) RETURN n
      """
    Then analysis succeeds

  Scenario: Match char
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED CHAR(10) RETURN n
      """
    Then analysis succeeds

  Scenario: Match bytes
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED BYTES RETURN n
      """
    Then analysis succeeds

  Scenario: Match bytes range
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED BYTES(1, 100) RETURN n
      """
    Then analysis succeeds

  Scenario: Match bytes long
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED BYTES(100) RETURN n
      """
    Then analysis succeeds

  Scenario: Match binary
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED BINARY(10) RETURN n
      """
    Then analysis succeeds

  Scenario: Match struct
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED {name STRING} RETURN n
      """
    Then analysis succeeds

  Scenario: Match any record
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED ANY RECORD RETURN n
      """
    Then analysis succeeds

  Scenario: Return nested record
    When the GQL query is analyzed:
      """
      RETURN { inner: { name: 1 } }
      """
    Then analysis succeeds

  Scenario: Match path
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED PATH RETURN n
      """
    Then analysis succeeds

  Scenario: Match any graph
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED ANY GRAPH RETURN n
      """
    Then analysis succeeds

  Scenario: Match table
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED TABLE {name STRING} RETURN n
      """
    Then analysis succeeds

  Scenario: Match union
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED ANY VALUE<INT32 | STRING> RETURN n
      """
    Then analysis succeeds

  Scenario: Match any
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED ANY RETURN n
      """
    Then analysis succeeds

  Scenario: Match property value
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED PROPERTY VALUE RETURN n
      """
    Then analysis succeeds

  Scenario: Match null
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED NULL RETURN n
      """
    Then analysis succeeds

  Scenario: Match nothing
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED NOTHING RETURN n
      """
    Then analysis succeeds

  Scenario: Match int32 not null
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.x IS TYPED INT32 NOT NULL RETURN n
      """
    Then analysis succeeds

  Scenario: Shortest with specific count
    When the GQL query is analyzed:
      """
      MATCH SHORTEST 2(n)-[e]->(m) RETURN n, m
      """
    Then analysis succeeds

  Scenario: Shortest group with return
    When the GQL query is analyzed:
      """
      MATCH SHORTEST 5 GROUP (n)-[]->(m) RETURN COUNT(*) AS paths_count
      """
    Then analysis succeeds
