#!/bin/bash

GRUN=$1

for f in ../../examples/*.bms
do
    echo $GRUN quickbms script "$f"
    $GRUN quickbms script "$f"
    if $GRUN quickbms script "$f" 2>&1 | grep "^line "; then
        echo "Test failed"
        false
    fi
done