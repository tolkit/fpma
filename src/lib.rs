//! `fpma` ia a small cli/library to parse and plot nhmmer.table files
//! generated by the flag `--tblout` on nhmmer.
//!
//! See the `plot` module for plotting details.
//!
//! See the `cli` module for the command line interface details.
//!
//! The `gff` module creates GFF3 files from the nhmmer.table files.
//!
//! For an example of what the plot looks like, please see
//! <b><a href="mitome.html">this example.</a></b>

use std::{
    fmt::{Display, Formatter},
    fs,
    io::{self, BufRead, Write},
    path::PathBuf,
    process::Command,
    slice::Iter,
    str::FromStr,
};

use strum_macros::Display;
use tempdir::TempDir;

/// A lightweight GFF3 writer.
pub mod gff;

/// A module to parse command line args.
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
            .expect("Failed to execute process.");

        assert!(output.status.success());
    }

    Ok(())
}

/// An enumeration of all the genes checked for in this pipeline.
///
/// There are 43 core genes currently, some of which are not present in all
/// of the plant clades.
///
/// In addition, there are 31 currently enumerated tRNA genes.
///
#[derive(Debug, Display, Hash, Eq, PartialEq, Clone, Copy, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
pub enum MitoGene {
    Atp1,
    Atp4,
    Atp6,
    Atp8,
    Atp9,
    /// Lost in hornwort, lycopytes
    CcmB,
    /// Lost in hornwort, lycopytes
    CcmC,
    /// Lost in hornwort, lycopytes
    CcmFc,
    /// Lost in hornwort, lycopytes
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
    /// Potentially absent from Seed plants
    TrnA_UGC,
    TrnC_GCA,
    TrnD_GUC,
    TrnE_UUC,
    TrnF_GAA,
    TrnG_GCC,
    /// Potentially absent from Seed plants
    TrnG_UCC,
    /// Potentially absent from Seed plants
    TrnH_GUG,
    TrnI_CAU,
    /// Potentially absent from Seed plants
    TrnI_GAU,
    TrnK_UUU,
    /// Potentially absent from Seed plants
    TrnL_CAA,
    /// Potentially absent from Seed plants
    TrnL_GAG,
    /// Potentially absent from Seed plants
    TrnL_UAA,
    /// Potentially absent from Seed plants
    TrnL_UAG,
    TrnfM_CAU,
    /// Potentially absent from Seed plants
    TrnM_CAU,
    /// Potentially absent from Seed plants
    TrnN_GUU,
    TrnP_UGG,
    TrnQ_UUG,
    /// Potentially absent from Seed plants
    TrnR_ACG,
    /// Potentially absent from Seed plants
    TrnR_UCG,
    /// Potentially absent from Seed plants
    TrnR_UCU,
    TrnS_GCU,
    /// Potentially absent from Seed plants
    TrnS_GGA,
    TrnS_UGA,
    /// Potentially absent from Seed plants
    TrnT_UGU,
    /// Potentially absent from Seed plants
    TrnV_GAC,
    TrnV_UAC,
    TrnW_CCA,
    TrnY_GUA,
}

impl MitoGene {
    /// Returns an iterator over the MitoGenes.
    pub fn iterator() -> Iter<'static, MitoGene> {
        // so I don't have to type all the paths out!
        use self::MitoGene::*;
        static MITOGENES: [MitoGene; 74] = [
            Atp1, Atp4, Atp6, Atp8, Atp9, CcmB, CcmC, CcmFc, CcmFn, Cob, Cox1, Cox2, Cox3, MatR,
            MttB, Nad1, Nad2, Nad3, Nad4, Nad4L, Nad5, Nad6, Nad7, Nad9, Rpl2, Rpl5, Rpl6, Rpl10,
            Rpl16, Rps1, Rps2, Rps3, Rps4, Rps7, Rps8, Rps10, Rps11, Rps12, Rps13, Rps14, Rps19,
            Sdh3, Sdh4, TrnA_UGC, TrnC_GCA, TrnD_GUC, TrnE_UUC, TrnF_GAA, TrnG_GCC, TrnG_UCC,
            TrnH_GUG, TrnI_CAU, TrnI_GAU, TrnK_UUU, TrnL_CAA, TrnL_GAG, TrnL_UAA, TrnL_UAG,
            TrnfM_CAU, TrnM_CAU, TrnN_GUU, TrnP_UGG, TrnQ_UUG, TrnR_ACG, TrnR_UCG, TrnR_UCU,
            TrnS_GCU, TrnS_GGA, TrnS_UGA, TrnT_UGU, TrnV_GAC, TrnV_UAC, TrnW_CCA, TrnY_GUA,
        ];
        MITOGENES.iter()
    }
}

// TODO: for angiosperms, don't include rpl6 and rps8?
// Note that these variant MUST equal the name in line 2
// of the HMM file (i.e. the 'NAME' variable).
impl FromStr for MitoGene {
    type Err = &'static str;

    /// Implementation of `FromStr` for MitoGene.
    ///
    /// I don't use `strum_macros::EnumString` because
    /// I need lowercase letters at the beginning.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // core genes
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
            // tRNA genes
            "trnA-UGC" => Ok(MitoGene::TrnA_UGC),
            "trnC-GCA" => Ok(MitoGene::TrnC_GCA),
            "trnD-GUC" => Ok(MitoGene::TrnD_GUC),
            "trnE-UUC" => Ok(MitoGene::TrnE_UUC),
            "trnF-GAA" => Ok(MitoGene::TrnF_GAA),
            "trnG-GCC" => Ok(MitoGene::TrnG_GCC),
            "trnH-GUG" => Ok(MitoGene::TrnH_GUG),
            "trnI-CAU" => Ok(MitoGene::TrnI_CAU),
            "trnI-GAU" => Ok(MitoGene::TrnI_GAU),
            "trnK-UUU" => Ok(MitoGene::TrnK_UUU),
            "trnL-CAA" => Ok(MitoGene::TrnL_CAA),
            "trnL-GAG" => Ok(MitoGene::TrnL_GAG),
            "trnL-UAA" => Ok(MitoGene::TrnL_UAA),
            "trnL-UAG" => Ok(MitoGene::TrnL_UAG),
            "trnM-CAU" => Ok(MitoGene::TrnM_CAU),
            "trnN-GUU" => Ok(MitoGene::TrnN_GUU),
            "trnP-UGG" => Ok(MitoGene::TrnP_UGG),
            "trnQ-UUG" => Ok(MitoGene::TrnQ_UUG),
            "trnR-ACG" => Ok(MitoGene::TrnR_ACG),
            "trnR-UCG" => Ok(MitoGene::TrnR_UCG),
            "trnR-UCU" => Ok(MitoGene::TrnR_UCU),
            "trnS-GCU" => Ok(MitoGene::TrnS_GCU),
            "trnS-GGA" => Ok(MitoGene::TrnS_GGA),
            "trnS-UGA" => Ok(MitoGene::TrnS_UGA),
            "trnT-UGU" => Ok(MitoGene::TrnT_UGU),
            "trnV-GAC" => Ok(MitoGene::TrnV_GAC),
            "trnW-CCA" => Ok(MitoGene::TrnW_CCA),
            "trnY-GUA" => Ok(MitoGene::TrnY_GUA),
            "trnfM-CAU" => Ok(MitoGene::TrnfM_CAU),
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

/// Implementation of `FromStr` for `Strand`.
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

/// An implementation of `Display` for `Strand`.
impl Display for Strand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match &self {
            Strand::Positive => write!(f, "{}", '+'),
            Strand::Negative => write!(f, "{}", '-'),
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

    /// In place mutation of `Nhmmer` where tables are read from a
    /// `tempdir::TmpDir` and pushed into the database.
    ///
    /// Critically, the output is then sorted by target name, query name
    /// and then E-value.
    ///
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

    /// Produce a GFF from the NHMMER tables.
    ///
    /// This function is pretty copy-heavy and I *might*
    /// re-factor at some point.
    #[allow(unused_variables)]
    pub fn make_gff3(
        &self,
        path: PathBuf,
        e_value_cli: f32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut gff3 = gff::GFF3::new();
        // iterate over the rows
        for NhmmerRow {
            target_name,
            query_name,
            hmm_from,
            hmm_to,
            ali_from,
            ali_to,
            env_from,
            env_to,
            sq_len,
            strand,
            e_value,
            score,
            bias,
        } in &self.rows
        {
            if *e_value > e_value_cli {
                continue;
            }
            let query_name = format!("{}", query_name);
            let type_: String = match query_name.starts_with("Trn") {
                true => "tRNA".into(),
                // I think I'm annotating either CDS or exons...
                false => "CDS".into(),
            };

            gff3.add_record(gff::GFF3Field {
                seqid: target_name.clone(),
                source: String::from("fpma"),
                r#type: type_,
                start: *env_from as usize,
                end: *env_to as usize,
                score: *score,
                strand: *strand,
                phase: 0,
                attributes: format!("Name={}", query_name),
            })
        }
        // create the file
        let mut gff_file = fs::File::create(path)?;

        gff::write_gff3(gff3, &mut gff_file)?;

        gff_file.flush()?;

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
        // EDIT: no longer filter out secondary hits at the same time.
        // as these show us the exons.
        // this data will eventually be plotted
        let mut plot_data = plot::PlotData::new();
        self.rows.iter().for_each(|e| {
            let group = plot_data
                .data
                .entry(e.target_name.clone())
                .or_insert(vec![]);
            group.push(plot::PlotDataRow {
                query_name: e.query_name,
                env_from: e.env_from,
                env_to: e.env_to,
                strand: e.strand,
                e_value: e.e_value,
                seq_len: e.sq_len,
            });
        });

        // print completeness data
        // potentially to be made optional but hey-ho.
        plot_data.completeness_angiosperms();

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

            print!("{}\t", mitogene);
            // we collect as we don't want to print the last tab
            let mut presence_string = String::new();

            for e in is_present {
                match e {
                    true => presence_string += "true\t",
                    false => presence_string += "false\t",
                }
            }
            presence_string.pop();
            println!("{}", presence_string);
        }

        Ok(plot_data)
    }
}
