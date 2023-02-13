#!/bin/bash

for f in $(find . -name "*.rs" -not -path "./target/*"); do
    echo "Formatting ${f}..."
    rustfmt --edition=2021 $f
done