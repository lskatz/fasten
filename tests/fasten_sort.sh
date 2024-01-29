#!/usr/bin/env bash
# Minimal test suite for fasten sort (telatin 2024)

THIS_SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
THIS_SCRIPT_NAME=$(basename "$0" | sed 's/\.sh//g')
source "${THIS_SCRIPT_DIR}/test_functions.sh"

# Test fasten_sort with two reads
FIRST=$(echo -e "@ciao\nAAA\n+\nIII\n@andrea\nCCC\n+\nEEE" | $BIN | head -n 1)
equal "$FIRST" "@andrea" "Testing sort order of two reads"

done_testing