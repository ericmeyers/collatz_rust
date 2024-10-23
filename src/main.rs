
use plotpy::{Curve, Plot, StrError};

use collatz_rust::collatz_length_brian;

fn main() -> Result<(), StrError> {
    let xs: Vec<i64> = (1..=50000).collect();
    //let ys:Vec<_> = xs.iter().map(collatz_length).collect();
    let ys: Vec<_> = xs.iter().map(|&n| collatz_length_brian(n)).collect();

    let mut curve = Curve::new();
    curve
        .set_label("MatPlotLib Test")
        //       .set_line_alpha(0.8)
        //       .set_line_color("#5f9cd8")
        .set_line_style("None")
        //       .set_marker_color("#eeea83")
        .set_marker_every(1)
        //       .set_marker_line_color("#da98d1")
        .set_marker_line_width(2.5)
        .set_marker_size(2.0)
        .set_marker_style("o");

    curve.draw(&xs, &ys);
    let mut plot = Plot::new();
    plot.add(&curve);
    let _ = plot.show("myplot");

    Ok(())
}
