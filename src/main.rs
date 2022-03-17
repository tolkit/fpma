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
  --hmms-path     Path to the directory containing all the HMM files.\
                  The default is \"./fastas/hmms/\", as generated in this repo.

EXAMPLE:
  fpma --plant-mito ./mito.fasta --nhmmer-path ./nhmmer
";

#[derive(Debug)]
struct AppArgs {
    mitochondrial_genome: std::path::PathBuf,
    path_to_nhmmer: std::path::PathBuf,
    path_to_hmms: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse the arguments
    let args = match parse_args() {
        Ok(v) => v,
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

    run_hmmer(mitochondrial_genome_path, nhmmer_path, path_to_hmms)?;

    let mut table_parser = Nhmmer::new();

    table_parser.read_tables_and_parse()?;

    table_parser.filter_table_and_print()?;

    for row in table_parser.rows {
        println!("{:?}", row);
    }

    Ok(())
}

fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = AppArgs {
        mitochondrial_genome: pargs.value_from_os_str("--plant-mito", parse_path)?,
        path_to_nhmmer: pargs.value_from_os_str("--nhmmer-path", parse_path)?,
        path_to_hmms: pargs.opt_value_from_os_str("--hmms-path", parse_path)?,
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
