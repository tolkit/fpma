use crate::Strand;
use std::fs::File;
use std::io::Write;

/// GFF3 File Format header.
static HEADER: &str = "##gff-version 3\n";

/// Fields of the GFF3.
pub struct GFF3Field {
    /// The sequence ID
    pub seqid: String,
    /// The source (here fpma)
    pub source: String,
    ///
    pub r#type: String,
    pub start: usize,
    pub end: usize,
    pub score: f32,
    pub strand: Strand,
    pub phase: u8,
    pub attributes: String,
}

/// Vector representation of a GFF3.
pub struct GFF3 {
    pub inner: Vec<GFF3Field>,
}

impl GFF3 {
    /// New instance of a GFF3.
    pub fn new() -> Self {
        GFF3 { inner: vec![] }
    }
    /// Add a record to a GFF3 file.
    pub fn add_record(&mut self, gff_field: GFF3Field) {
        self.inner.push(gff_field);
    }
}

/// Write a GFF3 to file.
pub fn write_gff3(gff: GFF3, buffer: &mut File) -> Result<(), std::io::Error> {
    // write the header
    buffer.write(HEADER.as_bytes())?;
    // now write all the fields
    for GFF3Field {
        seqid,
        source,
        r#type,
        start,
        end,
        score,
        strand,
        phase,
        attributes,
    } in gff.inner
    {
        writeln!(
            buffer,
            "{seqid}\t{source}\t{type}\t{start}\t{end}\t{score}\t{strand}\t{phase}\t{attributes}"
        )?;
    }

    Ok(())
}
