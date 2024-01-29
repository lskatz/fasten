#!/usr/bin/env bash

# This should be sourced by other test scripts: die if not
if [ -z "$THIS_SCRIPT_DIR" ]; then
    echo "ERROR: test_functions.sh should be sourced by other test scripts"
    exit 1
fi
TEST_TMP_FILE=$(mktemp)
NUM=0
FAIL=0
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color
BIN=$(readlink -f "${THIS_SCRIPT_DIR}/../target/release/${THIS_SCRIPT_NAME}")
DEB_BIN=$(readlink -f "${THIS_SCRIPT_DIR}/../target/debug/${THIS_SCRIPT_NAME}")


echo -e " *** ${GREEN}Testing $THIS_SCRIPT_NAME${NC} (test: $TEST_TMP_FILE)*** "



function test {
    NUM=$((NUM+1))
    local msg="$1"
    local condition=$2
    if [ "$condition" ]; then
        echo -e "${GREEN}OK${NC}\t$NUM: $msg"
    else
        FAIL=$((FAIL+1))
        echo -e "${RED}FAIL${NC}\t$NUM: $msg"
    fi
}

function equal {
    local got="$1"
    local expected="$2"
    local msg="$3"
    NUM=$((NUM+1))
    if [ "$got" == "$expected" ]; then
        echo -e "${GREEN}OK${NC}\t$NUM: $msg [$got]"
    else
        FAIL=$((FAIL+1))
        echo -e "${RED}FAIL${NC}\t$NUM: $msg"
        echo -e "\tGot: $got"
        echo -e "\tExpected: $expected"
    fi
}

function different {
    local got="$1"
    local expected="$2"
    local msg="$3"
    NUM=$((NUM+1))
    if [ "$got" != "$expected" ]; then
        echo -e "${GREEN}OK${NC}\t$NUM: $msg [$got != $expected]"
    else
        FAIL=$((FAIL+1))
        echo -e "${RED}FAIL${NC}\t$NUM: $msg"
        echo -e "\tGot: $got"
        echo -e "\tequals to: $expected"
    fi
}

function getmd5 {
    # use md5sum on Linux, md5 on OSX
    if [ "$(uname)" == "Darwin" ]; then
        md5 -q "$1"
    else
        md5sum "$1" | cut -f 1 -d " "
    fi
}
function done_testing {
    if [ -e "$TEST_TMP_FILE" ]; then
        rm "$TEST_TMP_FILE"
    fi
    if [ "$FAIL" -eq 0 ]; then
        echo -e "${GREEN}OK!${NC}\tAll $NUM tests passed${NC}"
        exit 0
    else
        echo -e "${RED}$FAIL/$NUM errors${NC}\ttests failed${NC}"
        exit 1
    fi
}

test "Release binary $BIN" "-e $BIN"
test "Release debug $DEB_BIN" "-e $DEB_BIN"
test "Release binary --help" "$DEB_BIN --help"
test "Debug binary --help" "$DEB_BIN --help"

test "Release binary --version" "$DEB_BIN --version"
test "Debug binary --version" "$DEB_BIN --version"
