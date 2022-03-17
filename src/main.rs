// input a plant mitochondrial genome
// output something... I guess a .gb file eventually?

use fpma::{run_hmmer, Nhmmer};

const HELP: &str = "\
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
                  genes occur.
  --e-value       The E-value cut-off determining presence of
                  mito gene. <default 0.001>
                  
EXAMPLE:
  fpma --plant-mito ./mito.fasta --nhmmer-path ./nhmmer
";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse the arguments
    let args = match parse_args() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };

    // parse the two command line args.
    let mitochondrial_genome_path = args.mitochondrial_genome;
    let nhmmer_path = args.path_to_nhmmer;
    // otherwise default to what's in this repo.
    let path_to_hmms = match args.path_to_hmms {
        Some(p) => p,
        None => std::path::PathBuf::from("./fastas/hmms/"),
    };
    // default value 0.001
    let e_value = match args.e_value {
        Some(e) => e,
        None => 0.001,
    };

    // execute nhmmer
    run_hmmer(mitochondrial_genome_path.clone(), nhmmer_path, path_to_hmms)?;

    let mut table_parser = Nhmmer::new();
    table_parser.read_tables_and_parse()?;

    let plot_data = table_parser.filter_table_and_print(e_value)?;

    if args.plot {
        plot_data.plot()?
    }

    Ok(())
}

#[derive(Debug)]
struct AppArgs {
    mitochondrial_genome: std::path::PathBuf,
    path_to_nhmmer: std::path::PathBuf,
    path_to_hmms: Option<std::path::PathBuf>,
    e_value: Option<f32>,
    plot: bool,
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let plot = pargs.contains("--plot");

    let args = AppArgs {
        mitochondrial_genome: pargs.value_from_os_str("--plant-mito", parse_path)?,
        path_to_nhmmer: pargs.value_from_os_str("--nhmmer-path", parse_path)?,
        path_to_hmms: pargs.opt_value_from_os_str("--hmms-path", parse_path)?,
        e_value: pargs.opt_value_from_fn("--e-value", parse_f32)?,
        plot,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

fn parse_f32(s: &str) -> Result<f32, &'static str> {
    s.parse().map_err(|_| "Cannot parse string to f32.")
}
