#!/usr/bin/env bash

mkdir fastas

for meta in ./*.meta.txt; do
    # format file name for saving file.
    nm=$(echo $meta | cut -d. -f2 | cut -d/ -f2)
    echo $nm

    while IFS="" read -r p || [ -n "$p" ]; do
        ACCESSION=$(echo $p | awk '{print $1}')
        START=$(echo $p | awk '{print $2}')
        STOP=$(echo $p | awk '{print $3}')
        STRAND=$(echo $p | awk '{print $4}')

        [ "$STRAND" != "complement" ] && STRAND_INT=1 || STRAND_INT=2

        curl -L "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=nuccore&id=${ACCESSION}&strand=${STRAND_INT}&seq_start=${START}&seq_stop=${STOP}&rettype=fasta&retmode=text&usehistory=y" >> "./fastas/${nm}.fasta"
        
    done < $meta

done
