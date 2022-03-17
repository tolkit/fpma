#!/usr/bin/env bash

mkdir hmms

for fna in ./nuc_alignments/*.fna; do
    nm=$(echo $fna | cut -d. -f2 | cut -d/ -f3)
    echo $nm
    ~/bin/hmmbuild hmms/${nm}.hmm $fna
done
