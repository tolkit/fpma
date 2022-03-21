use fpma::{cli::parse_args, run_hmmer, Nhmmer};
use tempdir::TempDir;

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
    let path_to_hmms = args.path_to_hmms;
    // default value 0.001
    let e_value = match args.e_value {
        Some(e) => e,
        None => 0.001,
    };

    // a temporary place to store these tables.
    let tmp_dir = TempDir::new("temp_tables")?;

    // execute nhmmer
    run_hmmer(
        mitochondrial_genome_path.clone(),
        nhmmer_path,
        path_to_hmms,
        &tmp_dir,
    )?;

    let mut table_parser = Nhmmer::new();
    table_parser.read_tables_and_parse(&tmp_dir)?;

    let plot_data = table_parser.filter_table_and_print(e_value)?;

    if args.plot.is_some() {
        plot_data.plot(&args.plot.unwrap())?
    }

    // make sure we close this dir.
    tmp_dir.close()?;

    Ok(())
}
