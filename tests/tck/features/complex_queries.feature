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

Feature: Complex queries

  Scenario: Complex nested query with pagination
    When the GQL query is analyzed:
      """
      CALL {
                  FOR company IN [{name: "ducati*"}]
                  CALL db.index.queryNodes('bvdNameFullText', company.name)
                      YIELD node AS n, score
                  FILTER
                      EXISTS {
                          MATCH (n)-[:HAS_CONTACT]->(website:Website)
                          WHERE VALUE {
                              CALL string_contains(LOWER(website.website_address), LOWER('duc'))
                              YIELD result
                              RETURN result
                          }
                      }
                          AND VALUE {
                          CALL string_contains(LOWER(n.name), LOWER('ati'))
                          YIELD result
                          RETURN result
                      }
                  RETURN DISTINCT n, score
              }
              LET website = VALUE {
                  MATCH (n)-[:HAS_CONTACT]->(w:Website)
                  WHERE w.website_address IS NOT NULL
                  RETURN w.website_address
                  LIMIT 1
              }
              ORDER BY website ASC, score ASC, ELEMENT_ID(n)
              OFFSET 200
              LIMIT 100
              RETURN
                  n,
                  VALUE {
                      MATCH (n)-[r:IS_SHAREHOLDER_OF|HAS_POSITION|HAS_BRANCH]-()
                      RETURN COUNT(*)
                  } AS cnt,
                  VALUE { MATCH (n)-[:HAS_CONTACT]->(w:Website) RETURN COLLECT_LIST(w) } AS website,
                  VALUE { MATCH (n)-[:HAS_CONTACT]->(s:Social) RETURN COLLECT_LIST(s) } AS social,
                  VALUE { MATCH (n)-[:HAS_CONTACT]->(e:Email) RETURN COLLECT_LIST(e) } AS email,
                  VALUE { MATCH (n)-[:HAS_CONTACT]->(p:Phone) RETURN COLLECT_LIST(p) } AS phone,
                  VALUE { MATCH (n)-[:HAS_ADDRESS]->(a:Address) RETURN COLLECT_LIST(a) } AS address,
                  VALUE { MATCH (n)-[:HAS_ADDRESS]->(a:Address)-[:HAS_CITY]->(c:City) RETURN COLLECT_LIST(c) } AS city,
                  VALUE { MATCH (n)-[:HAS_CONTACT]->(c:Country) RETURN COLLECT_LIST(c) } AS country
          
      """
    Then analysis succeeds
