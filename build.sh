#!/bin/sh

CXX="g++"
TARGET_DIR="./bin"
TARGET="spawner"
FLAGS="-Wall -Werror -Wpedantic"

$CXX -Wall -Werror -Wpedantic -o $TARGET_DIR/$TARGET ./src/main.cpp
