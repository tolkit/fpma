# Fast Plant Mitochondria Annotation

<p align="center">
    <img width="300" height="132" src="https://www.darwintreeoflife.org/wp-content/themes/dtol/dist/assets/gfx/dtol-logo-round.png">
</p>

Using a set of known genes (43 core + 31 tRNA), we can predict how many of these occur in an angiosperm mitochondrial genome assembly. In this repository there is a set of HMM files (`fastas/hmms`) which describe these genes.

The executable here runs `nhmmer` on these genes across the mitochondrial genome of interest.

## Requirements

You will need to direct the program to the executable path of `nhmmer`, and to a directory of HMMs (provided in this repository under `./hmms/`).

## fpma

The help page is as follows.

```bash
<Max Brown; Wellcome Sanger 2022>
Fast plant mito annotation (fmpa).
Version: 0.1.2

USAGE:
  fpma --plant-mito <PATH> --nhmmer-path <PATH> --hmms-path <PATH>
FLAGS:
  -h, --help            Prints help information
  -v, --version         Prints version information
ARGS:
  --plant-mito          Path to the plant mitochondrial genome
  --nhmmer-path         Path to the nhmmer executable (HMMER3)
  --hmms-path           Path to the directory containing a set of
                        HMM files. Download from:
                        https://github.com/tolkit/fpma/hmms/
OPTIONAL ARGS:
  --plot                Generate an HTML SVG of where the annotated
                        genes occur. Requires a name.
  --e-value             The E-value cut-off determining presence of
                        mito gene. <default 0.001>
  --gff                 Output a GFF3 file of gene locations.
                  
EXAMPLE:
  fpma --plant-mito ./mito.fasta --nhmmer-path ./nhmmer --hmms-path ./hmms/angiosperm_hmms/
```

Optionally an HTML plot is created. Please see the <b><a href="https://tolkit.github.io/fpma/">the docs</a></b> for more detail on behind the scenes, and for a <b><a href="https://tolkit.github.io/fpma/fpma/mitome.html">HTML plot preview.</a></b>

## Examples

Clone this repo to get the relevant code (`fpma`) & data (a bunch of HMMs).

```bash
# clone the repo
git clone https://github.com/tolkit/fpma
# install the binary to your path (REQUIRES RUST)
cd fpma && cargo install --path .
```

All ready to quickly annotate your plant mitochondrial genome. Say it's a flowering plant - the relevant HMM files are in the `./hmms/angiosperm_hmms` directory.

```bash
# executed in the fpma cloned directory

# executable
fpma \
# path to your plant mito
--plant-mito path/to/your/mito.fasta \
# path to executable HMMER3
--nhmmer-path path/to/nhmmer \
# path to the angiosperm HMM directory
--hmms-path ./hmms/angiosperm_hmms/ \
# make an HTML plot with the name `mitome.html`
--plot mitome.html \
# make a GFF3 with name `mitome.gff`
--gff mitome.gff \
# add a very stringent E-value cut-off
--e-value 0.0000000000001 > out.txt
# out.txt contains a simple boolean matrix TSV
# of whether each gene was present in each fasta
# record in the input fasta
```

## Disclaimer

This annotator is not supposed to be extremely complete nor accurate. The aim is just to determine the presence/absence of genes which should be present on a plant mitochondrion *speedily* and *without hassle*. 
