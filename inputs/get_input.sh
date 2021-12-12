#!/bin/bash
SESSION=`cat session_cookie.txt`
curl https://adventofcode.com/2021/day/$1/input --cookie "session=$SESSION" > ./$1