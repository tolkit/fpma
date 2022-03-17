use crate::{MitoGene, Strand};
use std::{collections::BTreeMap, fs, io::Write};

/// Size of the margins in the plot.
static MARGIN: usize = 15;

/// `PlotData` row entry.
pub struct PlotDataRow {
    pub query_name: MitoGene,
    pub env_from: i32,
    pub env_to: i32,
    pub strand: Strand,
    pub e_value: f32,
}

/// `PlotData` database. Composed of a `BTreeMap`, where
/// the keys are the contigs/fasta ID's and the values
/// are a vector of `PlotDataRow`.
pub struct PlotData {
    pub data: BTreeMap<String, Vec<PlotDataRow>>,
}

impl PlotData {
    /// Create a new instance of `PlotData`.
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }
    /// Plot... work in progress.
    pub fn plot(&self) -> Result<(), Box<dyn std::error::Error>> {
        // make the writable svg file
        let output = "test";
        let out_filename = format!("{}.svg", output);
        let mut svg_file = fs::File::create(out_filename)?;

        let no_of_entries = self.data.len();

        let width = 800;
        let subplot_height = 300;
        let height = subplot_height * no_of_entries;

        // construct the svg
        let svg = format!(
            "<?xml version='1.0' encoding='UTF-8'  standalone='no' ?> <!DOCTYPE svg \
             PUBLIC '-//W3C//DTD SVG 1.0//EN' \
             'http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd'> <svg version='1.0' \
             width='{}' height='{}' xmlns='http://www.w3.org/2000/svg' \
             xmlns:xlink='http://www.w3.org/1999/xlink'> \
            \
             <style type='text/css'> \
             .chromosome_line:hover {{ stroke-opacity: 1.0; stroke: crimson; stroke-width: 2; }} \
             </style> \
            \
              <defs>
                <!-- This is a right arrow pointer --> 
                <marker id='right_point' viewBox='0 0 10 10'
                    refX='1' refY='5'
                    markerUnits='strokeWidth'
                    markerWidth='10' markerHeight='10'
                    orient='auto'>
                <path d='M 0 0 L 10 5 L 0 10 z' fill='#f00'/>
                </marker>

                <!-- This is a left arrow pointer --> 
                <marker id='left_point' viewBox='0 0 10 10'
                    refX='1' refY='5'
                    markerUnits='strokeWidth'
                    markerWidth='10' markerHeight='10'
                    orient='auto'>
                <path d='M 0 5 L 10 10 L 10 0 z' fill='#f00'/>
                </marker>

            </defs>
                <rect x='' y='' width='' height='20' />
                <line x1='{}' y1='{}' x2='{}' y2='{}' stroke='black' style = 'stroke-width: 8;' marker-end='url(#right_point)' marker-start='url(#left_point)' />
                \
             </svg>",
            width,
            height,
            MARGIN,
            height - MARGIN,
            width - MARGIN,
            height - MARGIN,
        );

        svg_file.write_all(svg.as_bytes()).expect("unable to write");

        Ok(())
    }
}
