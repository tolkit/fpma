static HELP: &str = "\
<Max Brown; Wellcome Sanger 2022>
Fast plant mito annotation (fmpa).
Version: 0.1.1

USAGE:
  fpma --plant-mito <PATH> --nhmmer-path <PATH>
FLAGS:
  -h, --help            Prints help information
  -v, --version         Prints version information
ARGS:
  --plant-mito    Path to the plant mitochondrial genome
  --nhmmer-path   Path to the nhmmer executable (HMMER3)
  --hmms-path     Path to the directory containing all the
                  HMM files. Download from:
                  https://github.com/tolkit/fpma
OPTIONAL ARGS:
  --plot          Generate an HTML SVG of where the annotated
                  genes occur. Requires a name, no default.
  --e-value       The E-value cut-off determining presence of
                  mito gene. <default 0.001>
                  
EXAMPLE:
  fpma --plant-mito ./mito.fasta --nhmmer-path ./nhmmer --hmms-path ./angiosperm_hmms/
";

/// A `pico-args` struct to parse the command line args.
#[derive(Debug)]
pub struct AppArgs {
    pub mitochondrial_genome: std::path::PathBuf,
    pub path_to_nhmmer: std::path::PathBuf,
    pub path_to_hmms: std::path::PathBuf,
    pub e_value: Option<f32>,
    pub plot: Option<String>,
}

/// Parse the command line arguments.
pub fn parse_args() -> Result<AppArgs, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help and version have a higher priority.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    if pargs.contains(["-v", "--version"]) {
        print!("{}", "fpma version 0.1.1");
        std::process::exit(0);
    }

    let args = AppArgs {
        mitochondrial_genome: pargs.value_from_os_str("--plant-mito", parse_path)?,
        path_to_nhmmer: pargs.value_from_os_str("--nhmmer-path", parse_path)?,
        path_to_hmms: pargs.value_from_os_str("--hmms-path", parse_path)?,
        e_value: pargs.opt_value_from_fn("--e-value", parse_f32)?,
        plot: pargs.opt_value_from_str("--plot")?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

/// Parse `OsStr` to `PathBuf`.
fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

/// Parse `&str` to `f32`.
fn parse_f32(s: &str) -> Result<f32, &'static str> {
    s.parse().map_err(|_| "Cannot parse string to f32.")
}
