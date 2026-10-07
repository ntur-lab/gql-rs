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

@paths
Feature: Path pattern forms

  Scenario: Full edge pointing left
    When the GQL query is analyzed:
      """
      MATCH (a)<-[e]-(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Full edge left or undirected
    When the GQL query is analyzed:
      """
      MATCH (a)<~[e]~(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Full edge undirected or right
    When the GQL query is analyzed:
      """
      MATCH (a)~[e]~>(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Full edge left or right
    When the GQL query is analyzed:
      """
      MATCH (a)<-[e]->(b) RETURN a
      """
    Then analysis succeeds

  Scenario: Fixed graph pattern quantifier
    When the GQL query is analyzed:
      """
      MATCH ((a)-[e]->(b)){3} RETURN a
      """
    Then analysis succeeds

  Scenario: Referenced path variable
    When the GQL query is analyzed:
      """
      MATCH p = (a)-[e]->(b) YIELD p RETURN p
      """
    Then analysis succeeds

  Scenario: Simplified path defaulting left
    When the GQL query is analyzed:
      """
      MATCH <-/ Knows /- RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified path defaulting left or undirected
    When the GQL query is analyzed:
      """
      MATCH <~/ Knows /~ RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified path defaulting undirected or right
    When the GQL query is analyzed:
      """
      MATCH ~/ Knows /~> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified path defaulting left or right
    When the GQL query is analyzed:
      """
      MATCH <-/ Knows /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified fixed quantifier
    When the GQL query is analyzed:
      """
      MATCH -/ Knows{3} /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified questioned term
    When the GQL query is analyzed:
      """
      MATCH -/ Knows? /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified undirected override
    When the GQL query is analyzed:
      """
      MATCH -/ ~Knows /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified right override
    When the GQL query is analyzed:
      """
      MATCH -/ Knows> /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified undirected or right override
    When the GQL query is analyzed:
      """
      MATCH -/ ~Knows> /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified left or right override
    When the GQL query is analyzed:
      """
      MATCH -/ <Knows> /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified any-direction override
    When the GQL query is analyzed:
      """
      MATCH -/ -Knows /-> RETURN 1
      """
    Then analysis succeeds

  Scenario: Simplified negated label
    When the GQL query is analyzed:
      """
      MATCH -/ !Knows /-> RETURN 1
      """
    Then analysis succeeds
