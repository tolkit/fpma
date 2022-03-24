#!/usr/bin/env bash

for fasta in ./fastas/*.fasta; do
    nm=$(echo $fasta | cut -d/ -f3 | cut -d. -f1)
    mmft len -le 110 $fasta > "./fastas/${nm}.filtered.fasta"
done