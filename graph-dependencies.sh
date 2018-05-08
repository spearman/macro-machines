#!/bin/bash
set -x

cargo graph > dependencies.dot && dot -Tpng dependencies.dot > dependencies.png

exit
