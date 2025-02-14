#!/bin/bash

# If no files are specified, use all .tt files in the examples directory
if [ "$#" -eq 0 ]; then
    files=($(ls examples/*.tt))
else
    files=("$@")
fi

# Loop through all specified files
for file in "${files[@]}"; do
    echo "Running compiler on $file"
    cargo run -- "$file"
    if [ $? -eq 0 ]; then
        echo "$file compiled successfully"
    else
        echo "Error compiling $file"
    fi
    echo "-----------------------------------"
done
