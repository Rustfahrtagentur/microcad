#/bin/bash

find -name "*.log" | grep "doc/" | while read -r file; do
    echo
    echo "========= $file ========="
    echo
    cat "$file"
done