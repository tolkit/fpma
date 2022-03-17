#!/usr/bin/env bash

# I am using MAFFT version v7.471 (2020/Jul/3).

mkdir fna_alignments

for fasta in ./fastas/*.fasta; do
    # this is a bit janky
    nm=$(echo $fasta | cut -d. -f2 | cut -d/ -f3)
    /Users/mb39/homebrew/Cellar/mafft/7.471/bin/mafft $fasta > ./fna_alignments/${nm}.aln.fasta
done
