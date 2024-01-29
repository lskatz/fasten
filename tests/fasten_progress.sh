#!/usr/bin/env bash
# Minimal test suite for fasten progress (telatin 2024)

THIS_SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
THIS_SCRIPT_NAME=$(basename "$0" | sed 's/\.sh//g')
source "${THIS_SCRIPT_DIR}/test_functions.sh"

GOT_PERL=$(which perl)

if [ -z "$GOT_PERL" ]; then
    echo "Perl not found, skipping test"
    exit 0
fi


## Here we test that STDOUT is passed using --print

# shellcheck disable=SC2016
TOT=$("$GOT_PERL" -e 'my $c=0;for (1..1000) { 
    $c++;
    print "\@fasten_test$c\nAAA\n+\nIII\n";
    sleep 0.1;
 }' | "$BIN" --id "test-suite" --print | grep -c 'fasten_test' | grep -w 1000) 

equal "$TOT" "1000" "Testing sort order of 1000 reads"


## Here we test the final message 
# shellcheck disable=SC2016
"$GOT_PERL" -e 'my $c=0;for (1..1000) { 
    $c++;
    print "\@fasten_test$c\nAAA\n+\nIII\n";
    sleep 0.1;
 }' | "$BIN" --id "test-suite" 2> "$TEST_TMP_FILE"

END=$(grep "Finished" "$TEST_TMP_FILE" | cut -f 3 -d ":")
equal "$END" " Finished progress on 4000 reads" "Testing progress output"

done_testing