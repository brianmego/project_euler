#!/bin/bash

PROBLEM=$1
YELLOW='\033[1;33m'
NC='\033[0m' # No Color
PYTHON_BIN=./python/.env/bin/python

function run_language {
    # $1 is Language name
    # $2 is binary path
    printf "$1:\t"
    if [[ -f $2 ]]; then
        $2
    else
        printf "${YELLOW}None${NC}\n"
    fi
}

run_language Python python/$1.py
run_language C c/$1
run_language Rust rust/target/release/$1
