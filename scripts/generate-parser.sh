#!/bin/sh

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

set -eu

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
repository_root=$(CDPATH= cd -- "$script_dir/.." && pwd)
grammar_dir="$repository_root/resources"
generator="$grammar_dir/antlr4-4.8-2-SNAPSHOT-complete.jar"
output_dir="$repository_root/src/antlr"
utility_dir="$output_dir/utility"

if ! command -v java >/dev/null 2>&1; then
    echo "error: Java is required to run the ANTLR generator" >&2
    exit 1
fi

if [ ! -f "$generator" ]; then
    echo "error: ANTLR generator not found at $generator" >&2
    exit 1
fi

temporary_dir=$(mktemp -d "${TMPDIR:-/tmp}/gql-rs-antlr.XXXXXX")
trap 'rm -rf "$temporary_dir"' EXIT HUP INT TERM

(
    cd "$grammar_dir"
    java -jar "$generator" \
        -Dlanguage=Rust \
        -listener \
        -visitor \
        -o "$temporary_dir" \
        GQL.g4
)

generated_sources="gqllexer.rs gqlparser.rs gqllistener.rs gqlvisitor.rs"
generated_utility="GQL.interp GQL.tokens GQLLexer.interp GQLLexer.tokens"

for file in $generated_sources $generated_utility; do
    if [ ! -f "$temporary_dir/$file" ]; then
        echo "error: generator did not produce $file" >&2
        exit 1
    fi
done

mkdir -p "$utility_dir"

install_generated_source() {
    generated_file="$1"
    licensed_file="$temporary_dir/licensed-$generated_file"

    {
        printf '%s\n' \
            '// Copyright (c) 2026 NTUR LAB S.r.l.' \
            '//' \
            '// This file is part of NTURDB.' \
            '//' \
            '// Licensed under the EUPL, Version 1.2 or - as soon they will be approved by' \
            '// the European Commission - subsequent versions of the EUPL (the "Licence");' \
            '// You may not use this work except in compliance with the Licence.' \
            '// You may obtain a copy of the Licence at:' \
            '//' \
            '// https://joinup.ec.europa.eu/software/page/eupl' \
            '//' \
            '// Unless required by applicable law or agreed to in writing, software' \
            '// distributed under the Licence is distributed on an "AS IS" BASIS,' \
            '// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.' \
            '// See the Licence for the specific language governing permissions and' \
            '// limitations under the Licence.' \
            ''
        cat "$temporary_dir/$generated_file"
    } > "$licensed_file"

    install -m 0644 "$licensed_file" "$output_dir/$generated_file"
}

for file in $generated_sources; do
    install_generated_source "$file"
done

for file in $generated_utility; do
    install -m 0644 "$temporary_dir/$file" "$utility_dir/$file"
done

echo "ANTLR Rust sources generated successfully."
