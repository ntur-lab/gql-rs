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

@lexical @case-insensitive
Feature: Case-insensitive lexical elements
  GQL reserved words and boolean literals are recognized independently of case.

  Scenario: Lowercase keywords
    When the GQL query is analyzed:
      """
      match (n:User) where n.age > 18 return n.name
      """
    Then analysis succeeds

  Scenario: Mixed-case keywords and boolean literals
    When the GQL query is analyzed:
      """
      mAtCh (n:User) WhErE tRuE aNd NoT fAlSe ReTuRn n.name
      """
    Then analysis succeeds

  Scenario: Lowercase boolean and unknown literals
    When the GQL query is analyzed:
      """
      return true, false, unknown
      """
    Then analysis succeeds
