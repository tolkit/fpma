#!/usr/bin/env bash

# maximum of 2000 fastas returned.
# use the eutils API for repeatable downloads.

# these are the list of proteins which should occur in the plant mitochondrial genome

# rpl6 and rps8 don't appear to appear in angiosperms

mkdir fastas

for protein in {atp1,atp4,atp6,atp8,atp9,ccmB,ccmC,ccmFc,ccmFn,cob,cox1,cox2,cox3,matR,mttB,nad1,nad2,nad3,nad4,nad4L,nad5,nad6,nad7,nad9,rpl2,rpl5,rpl10,rpl16,rps1,rps2,rps3,rps4,rps7,rps10,rps11,rps12,rps13,rps14,rps19,sdh3,sdh4}; do
    # Maybe this is hacky? But only way I could get it to work
    # get the web env key on each iteration of the loop.
    WEB_ENV_KEY=$(curl -L "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/esearch.fcgi?db=protein&term=txid3398[Organism]+AND+${protein}[All%20Fields]+AND+mitochondrion[filter]&retmax=2000&usehistory=y" | grep "WebEnv" | sed -e 's/.*<WebEnv>\(.*\)<\/WebEnv>/\1/')
    echo $protein
    echo "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&term=txid3398[Organism]+AND+${protein}[All%20Fields]+AND+mitochondrion[filter]&rettype=fasta&retmode=text&WebEnv=${WEB_ENV_KEY}&query_key=1"
    # make the request.
    curl -L "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&term=txid3398[Organism]+AND+${protein}[All%20Fields]+AND+mitochondrion[filter]&rettype=fasta&retmode=text&WebEnv=${WEB_ENV_KEY}&query_key=1" > ./fastas/${protein}.fasta
done
