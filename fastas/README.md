# Generating the HMM files

All sequences are publicly available through NCBI. Requires the following softwares:

- <a href="https://mafft.cbrc.jp/alignment/software/">MAFFT</a>
- <a href="http://hmmer.org/">HMMER3</a>
- <a href="https://github.com/tolkit/aa2nucaln">aa2nuclan</a>


The pipeline is as follows:

```bash
# first, download the fastas using the eutils API
# see https://www.ncbi.nlm.nih.gov/books/NBK25500/
bash get_fastas.bash
# make the alignments using MAFFT (possibly needs tweaking)
bash make_alignments.bash
# convert these protein alignments to nucleotide alignments
bash make_nuc_alignments.bash
# finally create the hmms
bash make_hmms.bash
```