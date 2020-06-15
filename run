#!/bin/bash

PROBLEM=$1
YELLOW='\033[1;33m'
NC='\033[0m' # No Color
PYTHON_BIN=./python/.env/bin/python
export TIMEFORMAT='%R seconds'

function run_language {
    # $1 is Language name
    # $2 is binary path
    printf "$1:\t"
    if [[ -f $2 ]]; then
        time $2
        printf "\n"
    else
        printf "${YELLOW}None${NC}\n"
    fi
}

make > /dev/null
run_language Python python/$1.py
run_language C c/$1.out
run_language Rust rust/target/release/$1

printf "Erlang:\t"
if [[ -f erlang/prog$1.erl ]]; then
    time escript erlang/runner.erl erlang/prog$1
    printf "\n"
else
    printf "${YELLOW}None${NC}\n"
fi
