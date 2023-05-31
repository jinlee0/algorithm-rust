#!/bin/bash

buf=""
while IFS= read -r line; do
    if [[ $line == [a-z,0-9]* ]]; then
        if [[ $buf == "" ]]; then
            buf="$line"
        else
            buf="$buf\n$line"
        fi
    else
        echo -e "$buf" | cargo run < /dev/stdin
        echo ""
        buf=""
    fi
done < input.txt

echo -e "$buf" | cargo run < /dev/stdin