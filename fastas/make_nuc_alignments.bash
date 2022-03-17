#!/usr/bin/env bash

# aa2nucaln needs to be in path
# see https://github.com/tolkit/aa2nucaln

mkdir nuc_alignments

for aln in ./fna_alignments/*.fasta; do
    nm=$(echo $aln | cut -d. -f2 | cut -d/ -f3)
    echo "Processing $nm"
    aa2nucaln --fasta $aln > ./nuc_alignments/${nm}.fna
done