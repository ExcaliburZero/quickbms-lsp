#!/bin/bash

num_failed_tests=0

for f in ../examples/*.bms
do
    echo "=================="
    echo "$f"
    echo "------"
    if ! tree-sitter parse "$f"; then
        echo "Test failed"
        num_failed_tests=$((num_failed_tests + 1))
    fi
done

echo "=================="
if [ $num_failed_tests -ge 1 ]; then
    echo "$num_failed_tests tests failed."
    exit 1
else
    echo "Sucessfully parsed all example files."
fi