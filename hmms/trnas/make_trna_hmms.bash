#!/usr/bin/env bash

mkdir hmms

for aln in ./fastas/*.fna; do
    nm=$(echo $aln | cut -d/ -f3 | cut -d. -f1)
    ~/bin/hmmbuild "./hmms/${nm}.hmm" $aln
done