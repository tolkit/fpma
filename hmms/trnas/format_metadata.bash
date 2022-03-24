#!/usr/bin/env bash

for meta_file in ./*.txt; do
    nm=$(echo $meta_file | cut -d. -f2 | cut -d/ -f2)
    awk '/Annotation: Chromosome MT/ {split($5, arr, "\\.*"); gsub("\\(", "", arr[1]); gsub("\\)|,", "", arr[2]); gsub("\\)", "", $6); print $4 "\t" arr[1] "\t" arr[2] "\t" $6}' $meta_file > "${nm}.meta.txt"
done