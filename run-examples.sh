#!/bin/bash

while getopts "v" opt; do
  case $opt in
    v)
      set -x
      ;;
    *)
      echo "Valid options:"
      echo "  -v    verbose"
      exit 1
      ;;
  esac
done

for e in `ls examples/`; do
  example_name=`basename -s .rs $e`
  cargo run --example $example_name && make -f MakefileDot $example_name
done

exit
