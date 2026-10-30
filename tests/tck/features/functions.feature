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

Feature: Functions and expressions

  Scenario: Count expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COUNT(n.age)
      """
    Then analysis succeeds

  Scenario: Lowercase count expression
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN count(n.name)
      """
    Then analysis succeeds

  Scenario: Mixed case count expression
    When the GQL query is analyzed:
      """
      mAtCh (n) ReTuRn CoUnT(n)
      """
    Then analysis succeeds

  Scenario: Lowercase sum expression
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN sum(n.age)
      """
    Then analysis succeeds

  Scenario: Lowercase avg distinct expression
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN avg(DISTINCT n.age)
      """
    Then analysis succeeds

  Scenario: Reserved function names can be delimited as properties
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN n.`count`, n.`sum`
      """
    Then analysis succeeds

  Scenario: Count distinct
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COUNT(DISTINCT n.city)
      """
    Then analysis succeeds

  Scenario: Cardinality
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN CARDINALITY(n.followers)
      """
    Then analysis succeeds

  Scenario: Sum expression alone
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SUM(n.age)
      """
    Then analysis succeeds

  Scenario: Avg expression alone
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN AVG(n.age)
      """
    Then analysis succeeds

  Scenario: Min max expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN MIN(n.age), MAX(n.age)
      """
    Then analysis succeeds

  Scenario: Upper function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN UPPER(n.name)
      """
    Then analysis succeeds

  Scenario: Lower function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN LOWER(n.name)
      """
    Then analysis succeeds

  Scenario: Char length function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CHAR_LENGTH(n.name)
      """
    Then analysis succeeds

  Scenario: Coalesce expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COALESCE(n.email, 'unknown')
      """
    Then analysis succeeds

  Scenario: Nullif expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN NULLIF(n.city, 'Rome')
      """
    Then analysis succeeds

  Scenario: Trim function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN TRIM(n.description)
      """
    Then analysis succeeds

  Scenario: Left function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN LEFT(n.name, 3)
      """
    Then analysis succeeds

  Scenario: Right function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN RIGHT(n.email, 10)
      """
    Then analysis succeeds

  Scenario: Count all
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COUNT(n)
      """
    Then analysis succeeds

  Scenario: Count asterisk
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COUNT(*)
      """
    Then analysis succeeds

  Scenario: Sum
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SUM(n.balance)
      """
    Then analysis succeeds

  Scenario: Sum distinct
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SUM(DISTINCT n.balance)
      """
    Then analysis succeeds

  Scenario: Sum expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SUM(n.base_salary + n.bonus)
      """
    Then analysis succeeds

  Scenario: Avg distinct
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN AVG(DISTINCT n.age)
      """
    Then analysis succeeds

  Scenario: Avg expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN AVG(n.age * 2)
      """
    Then analysis succeeds

  Scenario: Min max
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN MIN(n.salary), MAX(n.salary)
      """
    Then analysis succeeds

  Scenario: Collect list
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COLLECT_LIST(n.name)
      """
    Then analysis succeeds

  Scenario: Stddev samp
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN STDDEV_SAMP(n.age)
      """
    Then analysis succeeds

  Scenario: Stddev pop
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN STDDEV_POP(n.score)
      """
    Then analysis succeeds

  Scenario: Aggregate with relationship
    When the GQL query is analyzed:
      """
      MATCH (n:User)-[r:RATES]->(p) RETURN p, AVG(r.rating) AS avg_rating
      """
    Then analysis succeeds

  Scenario: Multiple grouping keys
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.department, n.role, COUNT(*)
      """
    Then analysis succeeds

  Scenario: Grouping and aliases
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN n.department AS dept, AVG(n.salary) AS avg_salary
      """
    Then analysis succeeds

  Scenario: Signed value expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN -n.age, +n.score
      """
    Then analysis succeeds

  Scenario: Normalized predicate
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.name IS NORMALIZED RETURN n
      """
    Then analysis succeeds

  Scenario: Not normalized predicate
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE n.name IS NOT NFC NORMALIZED RETURN n
      """
    Then analysis succeeds

  Scenario: Normalize function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN NORMALIZE(n.name), NORMALIZE(n.name, NFKD)
      """
    Then analysis succeeds

  Scenario: Trigonometric and log functions
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SIN(n.score), LOG(10, n.score), LOG10(n.score), LN(n.score), EXP(n.score)
      """
    Then analysis succeeds

  Scenario: Duration between function
    When the GQL query is analyzed:
      """
      RETURN DURATION_BETWEEN(ZONED_DATETIME('2026-01-01T00:00:00Z'), ZONED_DATETIME('2026-01-02T00:00:00Z'))
      """
    Then analysis succeeds

  Scenario: Duration function
    When the GQL query is analyzed:
      """
      RETURN DURATION('P1D')
      """
    Then analysis succeeds

  Scenario: Duration abs function
    When the GQL query is analyzed:
      """
      RETURN ABS(DURATION('P1D'))
      """
    Then analysis succeeds

  Scenario: Byte length functions
    When the GQL query is analyzed:
      """
      RETURN BYTE_LENGTH(X'001122'), OCTET_LENGTH(X'001122')
      """
    Then analysis succeeds

  Scenario: Path length function
    When the GQL query is analyzed:
      """
      MATCH p = (a)-[e]->(b) RETURN PATH_LENGTH(p)
      """
    Then analysis succeeds

  Scenario: Cast to string
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.age AS STRING)
      """
    Then analysis succeeds

  Scenario: Cast to integer
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.id AS INTEGER)
      """
    Then analysis succeeds

  Scenario: Cast to float
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN CAST(n.score AS FLOAT)
      """
    Then analysis succeeds

  Scenario: Cast date
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.created_at AS DATE) AS creation_date
      """
    Then analysis succeeds

  Scenario: Cast timestamp
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.log_time AS TIMESTAMP) AS exact_time
      """
    Then analysis succeeds

  Scenario: Cast boolean
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.is_active_string AS BOOLEAN) AS is_active
      """
    Then analysis succeeds

  Scenario: Cast complex expression
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST((n.salary * 1.2) + 500 AS INTEGER) AS new_salary
      """
    Then analysis succeeds

  Scenario: Cast in where
    When the GQL query is analyzed:
      """
      MATCH (n:User) WHERE CAST(n.external_id AS INTEGER) > 1000 RETURN n
      """
    Then analysis succeeds

  Scenario: Cast in function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CHAR_LENGTH(CAST(n.age AS STRING)) AS age_digits
      """
    Then analysis succeeds

  Scenario: Cast string length
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.name AS STRING(128)) AS s
      """
    Then analysis succeeds

  Scenario: Cast char
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.code AS CHAR(12)) AS c
      """
    Then analysis succeeds

  Scenario: Cast varchar
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.email AS VARCHAR(255)) AS v
      """
    Then analysis succeeds

  Scenario: Cast bytes
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.payload AS BYTES(1024)) AS b
      """
    Then analysis succeeds

  Scenario: Cast binary
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.hash AS BINARY(32)) AS h
      """
    Then analysis succeeds

  Scenario: Cast varbinary
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.blob AS VARBINARY(256)) AS vb
      """
    Then analysis succeeds

  Scenario: Cast decimal
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.salary AS DECIMAL(18,4)) AS d
      """
    Then analysis succeeds

  Scenario: Cast float
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.score AS FLOAT(24)) AS f
      """
    Then analysis succeeds

  Scenario: Cast float32
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.score AS FLOAT32) AS f32
      """
    Then analysis succeeds

  Scenario: Cast double precision
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.score AS DOUBLE PRECISION) AS fd
      """
    Then analysis succeeds

  Scenario: Cast time with timezone
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.t AS TIME WITH TIME ZONE) AS ttz
      """
    Then analysis succeeds

  Scenario: Cast time without timezone
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.t AS TIME WITHOUT TIME ZONE) AS tltz
      """
    Then analysis succeeds

  Scenario: Cast timestamp with timezone
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.ts AS TIMESTAMP WITH TIME ZONE) AS tsz
      """
    Then analysis succeeds

  Scenario: Cast timestamp without timezone
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.ts AS TIMESTAMP WITHOUT TIME ZONE) AS tsl
      """
    Then analysis succeeds

  Scenario: Cast duration year to month
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.dur AS DURATION(YEAR TO MONTH)) AS dym
      """
    Then analysis succeeds

  Scenario: Cast duration day to second
    When the GQL query is analyzed:
      """
      MATCH (n:Event) RETURN CAST(n.dur AS DURATION(DAY TO SECOND)) AS dds
      """
    Then analysis succeeds

  Scenario: Cast list string
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.tags AS LIST<STRING(20)>[10]) AS ls
      """
    Then analysis succeeds

  Scenario: Cast list integer
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.tags AS INTEGER LIST[5]) AS li
      """
    Then analysis succeeds

  Scenario: Cast list any
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.tags AS LIST[5]) AS la
      """
    Then analysis succeeds

  Scenario: Cast any
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.legacy_data AS ANY) AS anyv
      """
    Then analysis succeeds

  Scenario: Cast any value
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.legacy_data AS ANY VALUE) AS anyval
      """
    Then analysis succeeds

  Scenario: Cast any property value
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.legacy_data AS ANY PROPERTY VALUE) AS anyprop
      """
    Then analysis succeeds

  Scenario: Cast any value closed union
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CAST(n.legacy AS ANY VALUE<STRING(32)|INTEGER>) AS v
      """
    Then analysis succeeds

  Scenario: Abs function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN ABS(n.balance)
      """
    Then analysis succeeds

  Scenario: Floor function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN FLOOR(n.rating)
      """
    Then analysis succeeds

  Scenario: Ceil function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CEIL(n.score)
      """
    Then analysis succeeds

  Scenario: Sqrt function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN SQRT(n.performance)
      """
    Then analysis succeeds

  Scenario: Power function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN POWER(n.multiplier, 2)
      """
    Then analysis succeeds

  Scenario: Mod function
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN MOD(n.id, 10)
      """
    Then analysis succeeds

  Scenario: Current date
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CURRENT_DATE
      """
    Then analysis succeeds

  Scenario: Current time
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CURRENT_TIME
      """
    Then analysis succeeds

  Scenario: Current timestamp
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN CURRENT_TIMESTAMP
      """
    Then analysis succeeds

  Scenario: Local time
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN LOCAL_TIME
      """
    Then analysis succeeds

  Scenario: Local timestamp
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN LOCAL_TIMESTAMP
      """
    Then analysis succeeds

  Scenario: Local datetime with param
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN LOCAL_DATETIME('2024-01-01T10:15:30')
      """
    Then analysis succeeds

  Scenario: Local time with param
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN LOCAL_TIME('10:15:30')
      """
    Then analysis succeeds

  Scenario: Zoned datetime with param
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN ZONED_DATETIME('2024-01-01T10:15:30+01:00')
      """
    Then analysis succeeds

  Scenario: Normalized nfc
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.name IS NFC NORMALIZED RETURN n
      """
    Then analysis succeeds

  Scenario: Normalized nfkd
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE n.description IS NFKD NORMALIZED RETURN n
      """
    Then analysis succeeds

  Scenario: Not normalized double negative
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE NOT (n.email IS NOT NFD NORMALIZED) RETURN n
      """
    Then analysis succeeds

  Scenario: Normalize with form
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN NORMALIZE(n.name, NFD) AS normalized_name
      """
    Then analysis succeeds

  Scenario: Aggregate distinct sum
    When the GQL query is analyzed:
      """
      MATCH (n:Transaction) RETURN SUM(DISTINCT n.amount) AS unique_total
      """
    Then analysis succeeds

  Scenario: Multiple distinct aggregates
    When the GQL query is analyzed:
      """
      MATCH (n:User) RETURN COUNT(DISTINCT n.country), SUM(DISTINCT n.visits), AVG(DISTINCT n.time_spent)
      """
    Then analysis succeeds

  Scenario: Duration difference
    When the GQL query is analyzed:
      """
      RETURN DURATION_BETWEEN(ZONED_DATETIME('2026-01-01T00:00:00Z'), ZONED_DATETIME('2026-12-31T23:59:59Z'))
      """
    Then analysis succeeds

  Scenario: Date operations
    When the GQL query is analyzed:
      """
      MATCH (e:Event) RETURN e.eventDate, DATE('2026-03-31'), e.eventDate = DATE('2026-03-31')
      """
    Then analysis succeeds

  Scenario: Cast chained
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN CAST(CAST(n.input_value AS INTEGER) AS STRING)
      """
    Then analysis succeeds

  Scenario: Cast
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN CAST(n.prop AS STRING)
      """
    Then analysis succeeds

  Scenario: Not normalized
    When the GQL query is analyzed:
      """
      MATCH (n) WHERE NOT (n.name IS NORMALIZED) RETURN n
      """
    Then analysis succeeds

  Scenario: Return normalized
    When the GQL query is analyzed:
      """
      RETURN NORMALIZE('Ã ', NFD)
      """
    Then analysis succeeds

  Scenario: Return graph dollar
    When the GQL query is analyzed:
      """
      RETURN GRAPH $$my_graph
      """
    Then analysis succeeds

  Scenario: Return count
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN [COUNT(n)]
      """
    Then analysis succeeds

  Scenario: Return elements
    When the GQL query is analyzed:
      """
      MATCH p = (a)-[]->(b) RETURN ELEMENTS(p)
      """
    Then analysis succeeds

  Scenario: Return ltrim
    When the GQL query is analyzed:
      """
      RETURN LTRIM("hello")
      """
    Then analysis succeeds

  Scenario: Return trim leading
    When the GQL query is analyzed:
      """
      RETURN TRIM(LEADING FROM "hello")
      """
    Then analysis succeeds

  Scenario: Return trim x00
    When the GQL query is analyzed:
      """
      RETURN TRIM(X'00' FROM X'001122')
      """
    Then analysis succeeds

  Scenario: Return collect list
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN COLLECT_LIST(n.name)
      """
    Then analysis succeeds

  Scenario: Return percentile cont
    When the GQL query is analyzed:
      """
      MATCH (n) RETURN PERCENTILE_CONT(n.score, 0.5)
      """
    Then analysis succeeds

  Scenario: Return cardinality
    When the GQL query is analyzed:
      """
      RETURN CARDINALITY([1, 2, 3])
      """
    Then analysis succeeds

  Scenario: Return size
    When the GQL query is analyzed:
      """
      RETURN SIZE([1, 2, 3])
      """
    Then analysis succeeds

  Scenario: Return timestamp
    When the GQL query is analyzed:
      """
      RETURN CURRENT_TIMESTAMP
      """
    Then analysis succeeds

  Scenario: Return duration
    When the GQL query is analyzed:
      """
      RETURN DURATION 'P1Y2M'
      """
    Then analysis succeeds

  Scenario: Elements with length
    When the GQL query is analyzed:
      """
      MATCH p = (a)-[]->(b) RETURN ELEMENTS(p), SIZE(ELEMENTS(p))
      """
    Then analysis succeeds

  Scenario: Path with length
    When the GQL query is analyzed:
      """
      MATCH p = (a)-[]->(b) RETURN PATH_LENGTH(p) AS len
      """
    Then analysis succeeds
