#!/usr/bin/env bash
# Minimal test suite for fasten  (telatin 2024)

THIS_SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
THIS_SCRIPT_NAME=$(basename "$0" | sed 's/\.sh//g')
source "${THIS_SCRIPT_DIR}/test_functions.sh"

IN_FILE="${THIS_SCRIPT_DIR}/../testdata/four_reads.fastq"

"$BIN" --out-format FASTA < "$IN_FILE" > "$TEST_TMP_FILE"
FASTA_COUNT=$(grep -c ">" "$TEST_TMP_FILE")
FASTQ_COUNT=$(grep -c "^@" "$TEST_TMP_FILE")
equal "$FASTA_COUNT" "4" "Testing that the output is in FASTA format"
equal "$FASTQ_COUNT" "0" "Testing that the output is in not FASTQ format"

"$BIN" --out-format FASTQ --in-format FASTA < "$TEST_TMP_FILE" > "$TEST_TMP_FILE.2"

FASTA_COUNT=$(grep -c "^>" "$TEST_TMP_FILE.2")
FASTQ_COUNT=$(grep -c "^@r" "$TEST_TMP_FILE.2")

equal "$FASTQ_COUNT" "4" "Testing that the output is in not FASTA format"
equal "$FASTQ_COUNT" "4" "Testing that the output is in FASTQ format"
rm "$TEST_TMP_FILE.2"