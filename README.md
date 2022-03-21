# Fast Plant Mitochondria Annotation

Using a set of known genes (~43), we can predict how many of these occur in an angiosperm mitochondrial genome assembly. In this repository there is a set of HMM files (`fastas/hmms`) which describe these genes.

The executable here runs `nhmmer` on these genes across the mitochondrial genome of interest.

## Requirements

You will need to direct the program to the executable path of `nhmmer`, and to a directory of HMMs (provided in this repository under `./fastas/hmms`).

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

Optionally an HTML plot is created. There is an example in this repository.

## Testing

In the case of the apple mitochondrial genome, we see absence of the following genes:

- rpl2 (present at a higher E-value than 0.001)
- rpl6 (absent in angiosperms)
- rps7 (known to be absent in apples)
- rps8 (absent in angiosperms)
- rps10/11 (known to be absent in mitochondrial genome in apples, but present in nuclear DNA.)

## Plans

- Make more general for Liverworts, Hornworts, Mosses, Lycopods, Ferns, and Gymnosperms.