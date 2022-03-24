#!/usr/bin/env bash

for fasta in ./fastas/*.filtered.fasta; do
    nm=$(echo $fasta | cut -d/ -f3 | cut -d. -f1)
    /Users/mb39/homebrew/Cellar/mafft/7.471/bin/mafft $fasta > "./fastas/${nm}.fna"
done