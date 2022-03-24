# tRNA's

Transfer RNA's also occur in the mitochondrial genome of plants. Note that these are so far only for angiosperm tRNA's.

## List of tRNA's

This is a non-exhaustive list based on this paper:
- <a href="https://www.sciencedirect.com/science/article/pii/S1567724919303447">Interchangeable parts: The evolutionarily dynamic tRNA population in plant mitochondria</a>

Most apparently occur in the mitochondria, but others are plastid, or even bacteria derived. As we are only interested in those present in plant mitochondria, this list is not the complete complement of plant tRNAs.

```
# 31 currently
trnA(UGC)
trnC(GCA)
trnD(GUC)
trnE(UUC)
trnF(GAA)
trnG(GCC)
trnG(UCC)
trnH(GUG)
trnI(CAU)
trnI(GAU)
trnK(UUU)
trnL(CAA)
trnL(GAG)
trnL(UAA)
trnL(UAG)
trnfM(CAU)
trnM(CAU)
trnN(GUU)
trnP(UGG)
trnQ(UUG)
trnR(ACG)
trnR(UCG)
trnR(UCU)
trnS(GCU)
trnS(GGA)
trnS(UGA)
trnT(UGU)
trnV(GAC)
trnV(UAC)
trnW(CCA)
trnY(GUA)
```

## Pipeline to make HMMs

More tricky to make these, as NCBI does not keep separate records for tRNA's like they do for genes.

```bash
# fetch the metadata
# creates a bunch of gene-anticodon text files
bash fetch_trna_metadata.bash 
# format the metadata to make it easier to parse.
 # loops over the .txt files and makes a bunch of formatted files.
bash format_metadata.bash
# the hard work
# use eutils to fetch the mitochondrial genome accession
# and filter using the start/stop coordinates of the tRNA
bash make_trna_fastas.bash
# there are some records which are SUPER long
# we need to filter these out.
bash filter_len.bash
# now make the MSAs
bash make_msas.bash
# and the HHMMs
bash make_trna_hmms.bash
# and clean the dirs.
bash clean.bash
```

Test case sort of done for angiosperms. Need to create the others (ferns, lycopods, etc...).