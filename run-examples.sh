#!/bin/bash

for e in `ls examples/`; do
  cargo run --example `basename -s .rs $e`;
done

exit
