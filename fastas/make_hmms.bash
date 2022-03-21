#!/usr/bin/env bash

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
    mkdir "${taxid}_hmms"
done

for taxid in {$ANGIOSPERM_TAXID,$GYMNOSPERM_TAXID,$FERN_TAXID,$LYCOPOD_TAXID,$HORNWORT_TAXID,$MOSSES_TAXID,$LIVERWORT_TAXID}; do
    for fna in ./${taxid}_nuc_alignments/*.fna; do
        nm=$(echo $fna | cut -d. -f2 | cut -d/ -f3)
        echo $nm
        ~/bin/hmmbuild ${taxid}_hmms/${nm}.hmm $fna
    done
done

# rename HMM dirs
mv 3398_hmms angiosperm_hmms
mv 1437180_hmms gymnosperm_hmms
mv 241806_hmms fern_hmms
mv 1521260_hmms lycopod_hmms
mv 13809_hmms hornwort_hmms
mv 3208_hmms moss_hmms
mv 3195_hmms liverwort_hmms