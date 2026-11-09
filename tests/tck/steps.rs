// Copyright (c) 2026 NTUR LAB S.r.l. (https://ntur-lab.com)
//
// This file is part of gql-rs (https://ntur-lab.com/gql-rs)
//
// Licensed under the EUPL, Version 1.2 or - as soon they will be approved by
// the European Commission - subsequent versions of the EUPL (the "Licence");
// You may not use this work except in compliance with the Licence.
// You may obtain a copy of the Licence at:
//
// https://joinup.ec.europa.eu/software/page/eupl
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licence is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licence for the specific language governing permissions and
// limitations under the Licence.

use cucumber::{gherkin::Step, then, when};
use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

use super::world::GqlWorld;

#[when("the GQL query is analyzed:")]
fn analyze_query(world: &mut GqlWorld, step: &Step) -> Result<(), String> {
    let query = step
        .docstring
        .as_deref()
        .ok_or_else(|| "the analysis step requires a GQL docstring".to_string())?;

    let mut analyzer = DefaultGQLQueryAnalyzer::new();
    world.analysis_result = Some(
        analyzer
            .exec(query.trim())
            .map(|_| ())
            .map_err(|error| format!("{error:?}")),
    );

    Ok(())
}

#[then("analysis succeeds")]
fn analysis_succeeds(world: &mut GqlWorld) -> Result<(), String> {
    match world.analysis_result.as_ref() {
        Some(Ok(())) => Ok(()),
        Some(Err(error)) => Err(format!(
            "expected analysis to succeed, but it failed: {error}"
        )),
        None => Err("analysis was not executed before checking its result".to_string()),
    }
}

#[then("analysis fails")]
fn analysis_fails(world: &mut GqlWorld) -> Result<(), String> {
    match world.analysis_result.as_ref() {
        Some(Err(_)) => Ok(()),
        Some(Ok(())) => Err("expected analysis to fail, but it succeeded".to_string()),
        None => Err("analysis was not executed before checking its result".to_string()),
    }
}
