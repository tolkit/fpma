#!/usr/bin/env bash

ANGIOSPERM_TAXID=3398
GYMNOSPERM_TAXID=1437180
FERN_TAXID=241806
LYCOPOD_TAXID=1521260
HORNWORT_TAXID=13809
MOSSES_TAXID=3208
LIVERWORT_TAXID=3195

# make all the directories
for taxid in {$ANGIOSPERM_TAXID,$GYMNOSPERM_TAXID,$FERN_TAXID,$LYCOPOD_TAXID,$HORNWORT_TAXID,$MOSSES_TAXID,$LIVERWORT_TAXID}; do
    mkdir "${taxid}"
done


for taxid in {$ANGIOSPERM_TAXID,$GYMNOSPERM_TAXID,$FERN_TAXID,$LYCOPOD_TAXID,$HORNWORT_TAXID,$MOSSES_TAXID,$LIVERWORT_TAXID}; do
    for trna in {'trnA(UGC)','trnC(GCA)','trnD(GUC)','trnE(UUC)','trnF(GAA)','trnG(GCC)','trnG(UCC)','trnH(GUG)','trnI(CAU)','trnI(GAU)','trnK(UUU)','trnL(CAA)','trnL(GAG)','trnL(UAA)','trnL(UAG)','trnfM(CAU)','trnM(CAU)','trnN(GUU)','trnP(UGG)','trnQ(UUG)','trnR(ACG)','trnR(UCG)','trnR(UCU)','trnS(GCU)','trnS(GGA)','trnS(UGA)','trnT(UGU)','trnV(GAC)','trnV(UAC)','trnW(CCA)','trnY(GUA)'}; do
        # get the gene and anticodon out.
        gene=$(echo $trna | cut -d'(' -f1)
        anticodon=$(echo $trna | cut -d'(' -f2 | cut -d')' -f1)
        
        # initial metadata fetches
        WEB_ENV_KEY=$(curl -L "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=gene&term=txid${taxid}[Organism]+AND+${gene}[All%20Fields]+AND+${anticodon}[All%20Fields]+AND+%28%22source%20mitochondrion%22[property]&retmax=2000&usehistory=y" | grep "WebEnv" | sed -e 's/.*<WebEnv>\(.*\)<\/WebEnv>/\1/')
        curl -L "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=gene&term=txid${taxid}[Organism]+AND+${gene}[All%20Fields]+AND+${anticodon}[All%20Fields]+AND+%28%22source%20mitochondrion%22[property]&rettype=fasta&retmode=text&WebEnv=${WEB_ENV_KEY}&query_key=1" > "./${taxid}/${gene}-${anticodon}.txt"
        
    done
done