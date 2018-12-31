#!/bin/sh

echo have $1
x=`cargo run --quiet --release --bin solve -- "$1"`
echo play $x
echo ' get' `cargo run --quiet --release --bin play -- "$1" "$x"`
