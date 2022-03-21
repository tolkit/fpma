#!/usr/bin/env bash

# aa2nucaln needs to be in path
# see https://github.com/tolkit/aa2nucaln

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
    mkdir "${taxid}_nuc_alignments"
done

for taxid in {$ANGIOSPERM_TAXID,$GYMNOSPERM_TAXID,$FERN_TAXID,$LYCOPOD_TAXID,$HORNWORT_TAXID,$MOSSES_TAXID,$LIVERWORT_TAXID}; do
    for aln in ./${taxid}_fna_alignments/*.fasta; do
        nm=$(echo $aln | cut -d. -f2 | cut -d/ -f3)
        echo "Processing $nm in $aln"
        aa2nucaln --fasta $aln > ./${taxid}_nuc_alignments/${nm}.fna
    done
done