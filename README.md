# Fast Plant Mitochondria Annotation

<p align="center">
    <img width="300" height="132" src="https://www.darwintreeoflife.org/wp-content/themes/dtol/dist/assets/gfx/dtol-logo-round.png">
</p>

Using a set of known genes (~43), we can predict how many of these occur in an angiosperm mitochondrial genome assembly. In this repository there is a set of HMM files (`fastas/hmms`) which describe these genes.

The executable here runs `nhmmer` on these genes across the mitochondrial genome of interest.

## Requirements

You will need to direct the program to the executable path of `nhmmer`, and to a directory of HMMs (provided in this repository under `./fastas/hmms`).

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

## Testing

In the case of the crab apple mitochondrial genome, we see absence of the following genes:

- rpl2 (present at a higher E-value than 0.001)
- rpl6 (absent in angiosperms)
- rps7 (known to be absent in apples)
- rps8 (absent in angiosperms)
- rps10/11 (known to be absent in mitochondrial genome in apples, but present in nuclear DNA.)
