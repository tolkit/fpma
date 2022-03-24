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

    // parse the command line args
    // the mitochondrial genome
    let mitochondrial_genome_path = args.mitochondrial_genome;
    // the path to nhmmer
    let nhmmer_path = args.path_to_nhmmer;
    // path to the directory of HMMs
    let path_to_hmms = args.path_to_hmms;
    // path to the plot HTML file
    let plot_html_path = args.plot;
    // check if this has a .html ending
    if plot_html_path.is_some() {
        let html = plot_html_path
            .clone()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with(".html");
        if !html {
            eprintln!("[-]\t--plot argument does not end with \".html\"");
            std::process::exit(1);
        }
    }
    let gff_path = args.gff;
    // path the the output GFF3 file.
    if gff_path.is_some() {
        let gff = gff_path
            .clone()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with(".gff");
        if !gff {
            eprintln!("[-]\t--gff argument does not end with \".gff\"");
            std::process::exit(1);
        }
    }

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

    if gff_path.is_some() {
        table_parser.make_gff3(gff_path.unwrap(), e_value)?;
    }

    // now we either print a summary, or we additionally plot it.
    let plot_data = table_parser.filter_table_and_print(e_value)?;

    if plot_html_path.is_some() {
        plot_data.plot(plot_html_path.unwrap())?
    }

    // make sure we close this dir.
    tmp_dir.close()?;

    Ok(())
}
