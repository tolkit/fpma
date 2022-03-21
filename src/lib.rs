//! `fpma` ia a small library to parse and plot nhmmer.table files
//! generated by the flag `--tblout` on nhmmer.
//!
//! See the `plot` module for plotting details.
//!
//! See the `cli` module for the command line interface details.
//!
//! For an example of what the plot looks like, please see
//! <b><a href="mitome.html">this example.</a></b>

use std::{
    fs,
    io::{self, BufRead},
    path::PathBuf,
    process::Command,
    slice::Iter,
    str::FromStr,
};

use tempdir::TempDir;

/// A module to parse command line args
pub mod cli;

/// A module to plot the output of nhmmer (HMMER3).
pub mod plot;

/// Entry point for running hmmer.
///
/// Takes a mitochondrial genome path, the path to executable
/// nhmmer, and path to the generated HMM's (in this repo it's
/// "./fastas/hmms/")
///
pub fn run_hmmer(
    mitochondrial_genome_path: PathBuf,
    nhmmer_exec_path: PathBuf,
    hmm_path: PathBuf,
    tmp_dir: &TempDir,
) -> Result<(), Box<dyn std::error::Error>> {
    // the dir where the HMMs live.
    let hmms = fs::read_dir(hmm_path)?;

    for hmm in hmms {
        // sort out the HMM path
        let hmm_path = hmm.expect("Could not open HMM path").path();
        eprintln!(
            "[+]\tRunning nhmmer with HMM: {}",
            hmm_path.clone().display()
        );

        let hmm_name = hmm_path.file_name().unwrap().to_str().unwrap();

        // get basename of mitochondrial_genome_path
        let bn_mgp = mitochondrial_genome_path
            .as_path()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();

        // the name of *this* table.
        let hmm_table = format!(
            "{}/{}-{}.table",
            tmp_dir.path().as_os_str().to_str().unwrap(),
            hmm_name,
            bn_mgp
        );

        let output = Command::new(&nhmmer_exec_path)
            .arg("--tblout")
            .arg(hmm_table)
            .arg(hmm_path)
            .arg(&mitochondrial_genome_path)
            .output()
            .expect("failed to execute process");

        // clean up all the temporary files in the ./hmm_tables dir.

        assert!(output.status.success());
    }

    Ok(())
}

/// An enumeration of all the genes checked for in this pipeline.
///
/// There are 42.
///
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
pub enum MitoGene {
    Atp1,
    Atp4,
    Atp6,
    Atp8,
    Atp9,
    CcmB,
    CcmC,
    CcmFc,
    CcmFn,
    Cob,
    Cox1,
    Cox2,
    Cox3,
    MatR,
    MttB,
    Nad1,
    Nad2,
    Nad3,
    Nad4,
    Nad4L,
    Nad5,
    Nad6,
    Nad7,
    Nad9,
    Rpl2,
    Rpl5,
    /// Not yet recorded in angiosperms.
    Rpl6,
    Rpl10,
    Rpl16,
    Rps1,
    Rps2,
    Rps3,
    Rps4,
    Rps7,
    /// Not yet recorded in angiosperms.
    Rps8,
    Rps10,
    Rps11,
    Rps12,
    Rps13,
    Rps14,
    Rps19,
    Sdh3,
    Sdh4,
}

impl MitoGene {
    /// Returns an iterator over the MitoGenes.
    pub fn iterator() -> Iter<'static, MitoGene> {
        // so I don't have to type all the paths out!
        use self::MitoGene::*;
        static MITOGENES: [MitoGene; 43] = [
            Atp1, Atp4, Atp6, Atp8, Atp9, CcmB, CcmC, CcmFc, CcmFn, Cob, Cox1, Cox2, Cox3, MatR,
            MttB, Nad1, Nad2, Nad3, Nad4, Nad4L, Nad5, Nad6, Nad7, Nad9, Rpl2, Rpl5, Rpl6, Rpl10,
            Rpl16, Rps1, Rps2, Rps3, Rps4, Rps7, Rps8, Rps10, Rps11, Rps12, Rps13, Rps14, Rps19,
            Sdh3, Sdh4,
        ];
        MITOGENES.iter()
    }
}

// TODO: for angiosperms, don't include rpl6 and rps8?
impl FromStr for MitoGene {
    type Err = &'static str;

    /// Implementation of `FromStr` for MitoGene.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "atp1" => Ok(MitoGene::Atp1),
            "atp4" => Ok(MitoGene::Atp4),
            "atp6" => Ok(MitoGene::Atp6),
            "atp8" => Ok(MitoGene::Atp8),
            "atp9" => Ok(MitoGene::Atp9),
            "ccmB" => Ok(MitoGene::CcmB),
            "ccmC" => Ok(MitoGene::CcmC),
            "ccmFc" => Ok(MitoGene::CcmFc),
            "ccmFn" => Ok(MitoGene::CcmFn),
            "cob" => Ok(MitoGene::Cob),
            "cox1" => Ok(MitoGene::Cox1),
            "cox2" => Ok(MitoGene::Cox2),
            "cox3" => Ok(MitoGene::Cox3),
            "matR" => Ok(MitoGene::MatR),
            "mttB" => Ok(MitoGene::MttB),
            "nad1" => Ok(MitoGene::Nad1),
            "nad2" => Ok(MitoGene::Nad2),
            "nad3" => Ok(MitoGene::Nad3),
            "nad4" => Ok(MitoGene::Nad4),
            "nad4L" => Ok(MitoGene::Nad4L),
            "nad5" => Ok(MitoGene::Nad5),
            "nad6" => Ok(MitoGene::Nad6),
            "nad7" => Ok(MitoGene::Nad7),
            "nad9" => Ok(MitoGene::Nad9),
            "rpl2" => Ok(MitoGene::Rpl2),
            "rpl5" => Ok(MitoGene::Rpl5),
            "rpl6" => Ok(MitoGene::Rpl6),
            "rpl10" => Ok(MitoGene::Rpl10),
            "rpl16" => Ok(MitoGene::Rpl16),
            "rps1" => Ok(MitoGene::Rps1),
            "rps2" => Ok(MitoGene::Rps2),
            "rps3" => Ok(MitoGene::Rps3),
            "rps4" => Ok(MitoGene::Rps4),
            "rps7" => Ok(MitoGene::Rps7),
            "rps8" => Ok(MitoGene::Rps8),
            "rps10" => Ok(MitoGene::Rps10),
            "rps11" => Ok(MitoGene::Rps11),
            "rps12" => Ok(MitoGene::Rps12),
            "rps13" => Ok(MitoGene::Rps13),
            "rps14" => Ok(MitoGene::Rps14),
            "rps19" => Ok(MitoGene::Rps19),
            "sdh3" => Ok(MitoGene::Sdh3),
            "sdh4" => Ok(MitoGene::Sdh4),
            _ => Err("Gene not present in the current set of genes."),
        }
    }
}

/// The strandedness of the HMM hit in the genome.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Strand {
    Positive,
    Negative,
}

/// Implementation of `FromStr` for Strand.
impl FromStr for Strand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Strand::Positive),
            "-" => Ok(Strand::Negative),
            _ => Err("The input was neither `-` nor `+`."),
        }
    }
}

/// A row representation of the tabular output of nhmmer
/// using the `--tblout` option.
#[derive(Debug)]
#[allow(dead_code)]
pub struct NhmmerRow {
    target_name: String,
    query_name: MitoGene,
    hmm_from: i32,
    hmm_to: i32,
    ali_from: i32,
    ali_to: i32,
    env_from: i32,
    env_to: i32,
    sq_len: i32,
    strand: Strand,
    e_value: f32,
    score: f32,
    bias: f32,
}

/// Database representation of the tabular output of
/// nhmmer.
#[derive(Debug)]
pub struct Nhmmer {
    pub rows: Vec<NhmmerRow>,
}

impl Nhmmer {
    /// A new `Nhmmer` instance.
    pub fn new() -> Self {
        Self { rows: vec![] }
    }

    /// In place mutation of `Nhmmer` where tables are read from the
    /// `TABLE_PATH` and pushed into the database.
    ///
    /// Critically, the output is then sorted by target name, query name
    /// and then E-value.
    ///
    /// TODO: perhaps make unique temporary direcory within `TABLE_PATH`
    /// so that multiple runs can be made at the same time.
    pub fn read_tables_and_parse(
        &mut self,
        tmp_dir: &TempDir,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // this gets all the files in the table path directory
        let tables = fs::read_dir(tmp_dir)?;

        for table in tables {
            let table_path = table.expect("Could not open table path").path();

            let table_file = fs::File::open(table_path)?;
            let table_file_lr = io::BufReader::new(table_file).lines();

            for line in table_file_lr {
                let l = line?;
                if l.starts_with("#") {
                    continue;
                }
                let l_vec = l.split_whitespace().collect::<Vec<&str>>();

                let row = NhmmerRow {
                    target_name: l_vec[0].parse::<String>()?,
                    query_name: l_vec[2].parse::<MitoGene>()?,
                    hmm_from: l_vec[4].parse::<i32>()?,
                    hmm_to: l_vec[5].parse::<i32>()?,
                    ali_from: l_vec[6].parse::<i32>()?,
                    ali_to: l_vec[7].parse::<i32>()?,
                    env_from: l_vec[8].parse::<i32>()?,
                    env_to: l_vec[9].parse::<i32>()?,
                    sq_len: l_vec[10].parse::<i32>()?,
                    strand: l_vec[11].parse::<Strand>()?,
                    e_value: l_vec[12].parse::<f32>()?,
                    score: l_vec[13].parse::<f32>()?,
                    bias: l_vec[14].parse::<f32>()?,
                };

                self.rows.push(row);
            }
        }

        // now order by target name, and query name and e value
        self.rows.sort_by(|a, b| {
            a.target_name.cmp(&b.target_name).then(
                a.query_name
                    .cmp(&b.query_name)
                    .then(a.e_value.partial_cmp(&b.e_value).unwrap()),
            )
        });

        Ok(())
    }

    /// Returns a `PlotData` object to be plotted if specified.
    ///
    /// The `Nhmmer` database is filtered on E-value threshold,
    /// and then only the top E-value hit for each gene is kept.
    ///
    /// A TSV is printed to STDOUT indicating presence/absence
    /// of genes.
    pub fn filter_table_and_print(
        &mut self,
        e_value: f32,
    ) -> Result<plot::PlotData, Box<dyn std::error::Error>> {
        // filter the rows on some E - value threshold
        self.rows.retain(|row| row.e_value < e_value);

        // collect the hits from each fasta into a separate vec
        // filter out secondary hits at the same time.
        // this data will eventually be plotted
        let mut plot_data = plot::PlotData::new();
        let mut index = 0;
        let mut current_gene = MitoGene::Atp1;
        self.rows.iter().for_each(|e| {
            let group = plot_data
                .data
                .entry(e.target_name.clone())
                .or_insert(vec![]);
            if index == 0 {
                group.push(plot::PlotDataRow {
                    query_name: e.query_name,
                    env_from: e.env_from,
                    env_to: e.env_to,
                    strand: e.strand,
                    e_value: e.e_value,
                    seq_len: e.sq_len,
                });
            } else if e.query_name == current_gene {
                () // can't continue in a closure?
            } else if e.query_name != current_gene {
                group.push(plot::PlotDataRow {
                    query_name: e.query_name,
                    env_from: e.env_from,
                    env_to: e.env_to,
                    strand: e.strand,
                    e_value: e.e_value,
                    seq_len: e.sq_len,
                });
                current_gene = e.query_name;
            }

            index += 1;
        });

        let headers: Vec<String> = plot_data.data.iter().map(|(k, _)| k.clone()).collect();

        println!("Mitogene\t{}", headers.join("\t"));

        for mitogene in MitoGene::iterator() {
            // we want to know if the gene is present for each contig
            // BTreeMap so keys should stay in order.
            let is_present: Vec<bool> = plot_data
                .data
                .iter()
                .map(|(_, v)| {
                    let collect_mitogenes: Vec<MitoGene> = v.iter().map(|e| e.query_name).collect();
                    collect_mitogenes.contains(&mitogene)
                })
                .collect();

            print!("{:?}\t", mitogene);
            for e in is_present {
                match e {
                    true => print!("true\t"),
                    false => print!("false\t"),
                }
            }
            println!("")
        }

        Ok(plot_data)
    }
}
