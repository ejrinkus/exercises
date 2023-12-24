#!/bin/bash

cd aoc${1}
cargo build --bin day${2}
