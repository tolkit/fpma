use crate::{MitoGene, Strand};
use std::{collections::BTreeMap, fs, io::Write};

/// Size of the margins in the plot.
static MARGIN: usize = 35;
/// The width of the plot.
static WIDTH: usize = 1200;
/// The height of each subplot.
static SUBPLOT_HEIGHT: usize = 200;
/// Function to make the HTML string.
fn make_html(svg: String) -> String {
    format!(
        "<!DOCTYPE html>
<html>
<head>
    <meta charset='UTF-8'>
    <title>Annotated Mito</title>
    <style type='text/css'>
        #tooltip {{
                background: cornsilk;
                border: 1px solid black;
                border-radius: 5px;
                padding: 5px;
            }}
    </style>
</head>

<body>
<!-- Tooltip div -->

    <div id='tooltip' display='none' style='position: absolute; display: none;'></div>

<!-- SVG here -->
    {}
</body>

<script>
    function showTooltip(evt, text) {{
        let tooltip = document.getElementById('tooltip');
        tooltip.innerHTML = text;
        tooltip.style.fontFamily = 'monospace';
        tooltip.style.display = 'block';
        tooltip.style.left = evt.pageX + 10 + 'px';
        tooltip.style.top = evt.pageY + 10 + 'px';
    }}

    function hideTooltip() {{
        var tooltip = document.getElementById('tooltip');
        tooltip.style.display = 'none';
    }}
</script>
</html>
    ",
        svg
    )
}

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
        let out_filename = format!("{}.html", output);
        let mut html_file = fs::File::create(out_filename)?;

        let no_of_entries = self.data.len();

        let height = SUBPLOT_HEIGHT * no_of_entries;

        // function to add all the annotations
        let base_chroms = generate_plot_annotations(self);

        // construct the svg
        let svg = format!(
            "<svg width='{}' height='{}'>
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

        let html = make_html(svg);

        html_file
            .write_all(html.as_bytes())
            .expect("unable to write");

        Ok(())
    }
}

/// A function which returns the arrows representing genes
/// along the axis of the mitochondrial contig.
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

        let line = format!("
            <line x1='{x1}' y1='{y1}' x2='{x2}' y2='{y2}' stroke='black' style = 'stroke-width: 3;' />\n"
        );

        // what's the actual best value... 75 looks okay
        let y1_mid = y1 - 75;
        let y2_mid = y2 - 75;
        // add mid line
        let mid_line = format!("
            <line x1='{x1}' y1='{y1_mid}' x2='{x2}' y2='{y2_mid}' stroke='black' stroke-dasharray='4' style = 'stroke-width: 1;' />\n"
        );

        // labels at the top of each subplot.
        let y_label_offset = 15;
        let y_text_label = (y1 - SUBPLOT_HEIGHT) + MARGIN + y_label_offset;
        let contig_text_label = format!(
            "
                <text x='{x1}' y='{y_text_label}' class='small' font-family='monospace'>{contig_id}</text>"
        );

        base_chroms += &line;
        base_chroms += &mid_line;
        base_chroms += &contig_text_label;

        // now add each of the mitogenes in turn
        // data point scales
        let x_data_min = 0.0;
        let x_data_max = mitogenes[0].seq_len as f32;
        // visualisation scales
        let x_viz_min = x1 as f32;
        let x_viz_max = x2 as f32;

        for PlotDataRow {
            query_name,
            env_from,
            env_to,
            strand,
            e_value,
            seq_len,
        } in mitogenes
        {
            // find the start and end of the genes
            let x1_scaled = scale_x(
                *env_from as f32,
                x_data_min,
                x_data_max,
                x_viz_min,
                x_viz_max,
            );
            let x2_scaled = scale_x(*env_to as f32, x_data_min, x_data_max, x_viz_min, x_viz_max);

            // add arrow
            let marker = "marker-end='url(#right_point)'";

            use rand::Rng;
            let mut rng = rand::thread_rng();
            // now adjust the height based on strandedness
            let y_gene = match strand {
                Strand::Positive => rng.gen_range((y1 as f32) - 70.0..=(y1 as f32) - 10.0),
                Strand::Negative => rng.gen_range(
                    (y1 as f32) - (SUBPLOT_HEIGHT - MARGIN - 25) as f32..=(y1 as f32) - 80.0,
                ),
            };

            // gene range in bp in a newline.
            let mitogene_plus_range = format!(
                "\"<b>{:?}</b>\" + \"<br/>\" + \"{} &rarr; {} bp\" + \"<br/>\" + \"<b>E-value</b>: {}\"",
                query_name,
                format_bp_pretty(*env_from),
                format_bp_pretty(*env_to),
                format!("{:.7}", e_value)
            );

            let gene_line = format!("
                <line x1='{x1_scaled}' y1='{y_gene}' x2='{x2_scaled}' y2='{y_gene}' stroke='black' style = 'stroke-width: 3;' {marker} onmousemove='showTooltip(evt, {mitogene_plus_range});' onmouseout='hideTooltip();'/>"
            );

            // because SVG markers don't trigger events for some reason...
            let circle_hover = format!(
                "<circle r='5' fill='transparent' cx='{x2_scaled}' cy='{y_gene}' onmousemove='showTooltip(evt, {mitogene_plus_range});' onmouseout='hideTooltip();''></circle>"
            );

            base_chroms += &gene_line;
            base_chroms += &circle_hover;
        }

        base_chroms += "\n";

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
                "<text x='{axis_label_len_scaled}' y='{axis_label_text_y}' class='small' text-anchor='middle' font-family='monospace'>{axis_label_text}</text>\n"
            );

            base_chroms += &axis_label;
        }
    }

    base_chroms
}

/// Scale an x value from the data scale to the visualisation scale.
fn scale_x(x: f32, x_data_min: f32, x_data_max: f32, x_viz_min: f32, x_viz_max: f32) -> f32 {
    // scale into range [x_viz_min, x_viz_max]
    ((x_viz_max - x_viz_min) * ((x - x_data_min) / (x_data_max - x_data_min))) + x_viz_min
}

/// Add `bp` to the end of a string.
fn format_axis_label_len(x: f32) -> String {
    format!("{} bp", x)
}

/// Pretty print `i32` numbers.
fn format_bp_pretty(n: i32) -> String {
    let mut s = String::new();
    let n_str = n.to_string();
    let a = n_str.chars().rev().enumerate();
    for (idx, val) in a {
        if idx != 0 && idx % 3 == 0 {
            s.insert(0, ',');
        }
        s.insert(0, val);
    }
    s
}
