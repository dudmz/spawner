#!/bin/sh

CXX=g++

OUTPUT_DIR=./bin
TARGET=spawner

clean() {
    rm -rf $OUTPUT_DIR
}

build() {
    clean && mkdir -p $OUTPUT_DIR && $CXX -o $OUTPUT_DIR/$TARGET src/main.cpp
}

build
