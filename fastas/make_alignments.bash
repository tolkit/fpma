#!/usr/bin/env bash

# I am using MAFFT version v7.471 (2020/Jul/3).

mkdir angiosperm_fna_alignments

# the taxids of all major land plants
ANGIOSPERM_TAXID=3398
GYMNOSPERM_TAXID=1437180
FERN_TAXID=241806
LYCOPOD_TAXID=1521260
HORNWORT_TAXID=13809
MOSSES_TAXID=3208
LIVERWORT_TAXID=3195

# make all the directories
for taxid in {$ANGIOSPERM_TAXID,$GYMNOSPERM_TAXID,$FERN_TAXID,$LYCOPOD_TAXID,$HORNWORT_TAXID,$MOSSES_TAXID,$LIVERWORT_TAXID}; do
    mkdir "${taxid}_fna_alignments"
done

for taxid in {$ANGIOSPERM_TAXID,$GYMNOSPERM_TAXID,$FERN_TAXID,$LYCOPOD_TAXID,$HORNWORT_TAXID,$MOSSES_TAXID,$LIVERWORT_TAXID}; do
    for fasta in ./${taxid}/*.fasta; do
        # this is a bit janky
        nm=$(echo $fasta | cut -d. -f2 | cut -d/ -f3)
        echo $taxid $nm
        /Users/mb39/homebrew/Cellar/mafft/7.471/bin/mafft $fasta > ./${taxid}_fna_alignments/${nm}.aln.fasta
    done
done