#!/bin/bash

# Find all _check.py files in subdirectories
find . -type f -name '*_check.py' | while read file; do
    # Navigate to the file's directory
    cd "$(dirname "$file")"
    echo $file
    # Execute the Python file
    python3 "$(basename "$file")"
    
    # Navigate back to the original directory
    cd -
done
