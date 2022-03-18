use crate::{MitoGene, Strand};
use std::{collections::BTreeMap, fs, io::Write};

/// Size of the margins in the plot.
static MARGIN: usize = 30;
/// The width of the plot.
static WIDTH: usize = 1200;
/// The height of each subplot.
static SUBPLOT_HEIGHT: usize = 200;

/// `PlotData` row entry.
pub struct PlotDataRow {
    pub query_name: MitoGene,
    pub env_from: i32,
    pub env_to: i32,
    pub strand: Strand,
    pub e_value: f32,
    pub seq_len: i32,
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
    pub fn plot(&self, output: &str) -> Result<(), Box<dyn std::error::Error>> {
        // make the writable svg file
        let out_filename = format!("{}.svg", output);
        let mut svg_file = fs::File::create(out_filename)?;

        let no_of_entries = self.data.len();

        let height = SUBPLOT_HEIGHT * no_of_entries;

        // function to add all the annotations
        let base_chroms = generate_plot_annotations(self);

        // construct the svg
        let svg = format!(
            "<?xml version='1.0' encoding='UTF-8'  standalone='no' ?> <!DOCTYPE svg \
             PUBLIC '-//W3C//DTD SVG 1.0//EN' \
             'http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd'> <svg version='1.0' \
             width='{}' height='{}' xmlns='http://www.w3.org/2000/svg' \
             xmlns:xlink='http://www.w3.org/1999/xlink'> \
            \
             <style type='text/css'> \
                <!-- no styling yet -->
             </style> \
            \
              <defs>
                <!-- This is an arrow pointer --> 
                <marker id='right_point' viewBox='0 0 10 10'
                    refX='1' refY='5'
                    markerUnits='strokeWidth'
                    markerWidth='3' markerHeight='3'
                    orient='auto'>
                    <path d='M 0 0 L 10 5 L 0 10 z' fill='#f00'/>
                </marker>
            </defs>
                {}
                \
             </svg>",
            WIDTH, height, base_chroms
        );

        svg_file.write_all(svg.as_bytes()).expect("unable to write");

        Ok(())
    }
}

#[allow(unused_variables)]
fn generate_plot_annotations(data: &PlotData) -> String {
    // a big string to add all the SVG elements of interest
    let mut base_chroms = String::new();

    // iterate over the chromosomes
    // reverse because SVG coordinate system.
    for (mut el, (contig_id, mitogenes)) in data.data.iter().enumerate().rev() {
        // el == 0 does nothing, so add 1!
        el += 1;
        let x1 = MARGIN;
        let y1 = (SUBPLOT_HEIGHT * el) - MARGIN;
        let x2 = WIDTH - MARGIN;
        let y2 = (SUBPLOT_HEIGHT * el) - MARGIN;

        let line = format!("<line x1='{x1}' y1='{y1}' x2='{x2}' y2='{y2}' stroke='black' style = 'stroke-width: 3;' />\n");

        // labels at the top of each subplot.
        let y_label_offset = 25;
        let y_text_label = (y1 - SUBPLOT_HEIGHT) + MARGIN + y_label_offset;
        let contig_text_label =
            format!("<text x='{x1}' y='{y_text_label}' class='small'>{contig_id}</text>");

        base_chroms += &line;
        base_chroms += &contig_text_label;

        // now add each of the mitogenes in turn
        // data point scales
        let x_data_min = 0.0;
        let x_data_max = mitogenes[0].seq_len as f32;
        // visualisation scales
        let x_viz_min = MARGIN as f32;
        let x_viz_max = (WIDTH - MARGIN) as f32;

        for PlotDataRow {
            query_name,
            env_from,
            env_to,
            strand,
            e_value,
            seq_len,
        } in mitogenes
        {
            let x1_scaled = scale_x(
                *env_from as f32,
                x_data_min,
                x_data_max,
                x_viz_min,
                x_viz_max,
            );
            let x2_scaled = scale_x(*env_to as f32, x_data_min, x_data_max, x_viz_min, x_viz_max);

            // now adjust the height and end marker
            // based on strandedness
            let (y_gene, marker) = match strand {
                Strand::Positive => ((y1 as f32) - 10.0, "marker-end='url(#right_point)'"),
                Strand::Negative => ((y1 as f32) - 50.0, "marker-end='url(#right_point)'"),
            };

            let gene_line = format!("
            <line x1='{x1_scaled}' y1='{y_gene}' x2='{x2_scaled}' y2='{y_gene}' stroke='black' style = 'stroke-width: 3;' {marker}>
                <title>{:?}</title>
            </line>
            ", query_name);

            base_chroms += &gene_line;
        }

        // lastly add scale labels
        for label in 0..=5 {
            let axis_label_len = ((x_data_max / 5.0) * label as f32).round();

            // account for the margin.
            let axis_label_len_scaled = if label != 0 {
                scale_x(axis_label_len, x_data_min, x_data_max, x_viz_min, x_viz_max)
            } else {
                MARGIN as f32
            };

            let axis_label_text_y = y1 + 15;

            let axis_label_text = format_axis_label_len(axis_label_len);

            let axis_label = format!(
                "<text x='{axis_label_len_scaled}' y='{axis_label_text_y}' class='small'>{axis_label_text}</text>\n"
            );

            base_chroms += &axis_label;
        }
    }

    base_chroms
}

fn scale_x(x: f32, x_data_min: f32, x_data_max: f32, x_viz_min: f32, x_viz_max: f32) -> f32 {
    // scale into range [x_viz_min, x_viz_max]
    (x_viz_max - x_viz_min) * ((x - x_data_min) / (x_data_max - x_data_min))
}

fn format_axis_label_len(x: f32) -> String {
    format!("{} bp", x)
}
