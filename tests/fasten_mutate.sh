#!/usr/bin/env bash
# Minimal test suite for fasten mutate (telatin 2024)

THIS_SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
THIS_SCRIPT_NAME=$(basename "$0" | sed 's/\.sh//g')
source "${THIS_SCRIPT_DIR}/test_functions.sh"

INFILE="${THIS_SCRIPT_DIR}/../testdata/four_reads.fastq"


"$BIN" --snps 1 < "$INFILE" > "$TEST_TMP_FILE"

MD5_IN_EXPECTED="8a08ae75226dfacd60f6fe2a1000f100"
MD5=$(getmd5 "$TEST_TMP_FILE" | cut -f 1 -d " ")
MD5_IN=$(getmd5 "$INFILE" | cut -f 1 -d " ")

equal "$MD5_IN" "$MD5_IN_EXPECTED" "Testing that the input file wasnt changed"
different "$MD5" "$MD5_IN" "Testing that the output is different from the input"