#!/usr/bin/env bash
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

passed=0
failed=0
failed_dirs=()

while IFS= read -r manifest; do
    dir="$(dirname "$manifest")"
    if [[ "$dir" == "$root" ]]; then
        continue
    fi
    if [[ "$dir" == *"/calc-core" ]]; then
        continue
    fi
    if (cd "$dir" && cargo test -q 2>/dev/null); then
        echo "PASS  $dir"
        passed=$((passed + 1))
    else
        echo "FAIL  $dir"
        failed=$((failed + 1))
        failed_dirs+=("$dir")
    fi
done < <(find . -path './.git' -prune -o -name Cargo.toml -print | sort)

echo
echo "Summary: $passed passed, $failed failed"
if ((${#failed_dirs[@]} > 0)); then
    echo "Failed lessons:"
    for d in "${failed_dirs[@]}"; do
        echo "  - $d"
    done
    exit 1
fi
