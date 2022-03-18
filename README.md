# Fast Plant Mitochondria Annotation

Using a set of known genes (~40), we can predict how many of these occur in a mitochondrial genome assembly. In this repository there is a set of HMM files (`fastas/hmms`) which describe these genes.

The executable here runs `nhmmer` on these genes across the mitochondrial genome of interest.

## fpma

The help page is as follows.

```bash
<Max Brown>
Fast plant mito annotation (fmpa).

USAGE:
  fpma --plant-mito <PATH> --nhmmer-path <PATH>
FLAGS:
  -h, --help            Prints help information
ARGS:
  --plant-mito    Path to the plant mitochondrial genome
  --nhmmer-path   Path to the nhmmer executable (HMMER3)
OPTIONAL ARGS:
  --hmms-path     Path to the directory containing all the
                  HMM files. The default is \"./fastas/hmms/\",
                  as generated in this repo.
  --plot          Generate an SVG plot of where the annotated
                  genes occur. Requires a name, no default.
  --e-value       The E-value cut-off determining presence of
                  mito gene. <default 0.001>
                  
EXAMPLE:
  fpma --plant-mito ./mito.fasta --nhmmer-path ./nhmmer --plot output > output.tsv
```

And an example output SVG from the program.

<img src="./mitome.svg">