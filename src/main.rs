
use itertools::iterate;
use itertools::Itertools;

//use plotly::layout::{Layout, Axis};
//use plotly::{Plot, Scatter};
//use plotly::common::Mode;


use plotpy::{linspace, Curve, Plot, StrError};


fn collatz_next(n: &i64) -> i64 {

   if n % 2 == 0 {
      n / 2
   }
   else {
      3 * n + 1
   }

}

fn collatz_length(n: &i64) -> i64 {

    let v: Vec<_> = iterate(*n, collatz_next).take_while_inclusive(|&n| n > 1).collect();
    v.len() as i64
}



fn main() -> Result<(), StrError>  {
    let xs:Vec<i64> = (1..=5000000).collect();
    let ys:Vec<_> = xs.iter().map(collatz_length).collect();

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
    let _ = plot.show("myscript");
    Ok(())

/*
    let mut plot = Plot::new();
    plot.set_layout(Layout::new()
                    .title("Collatz Conjecture")
                    .height(800)
                    .width(1000)
                    .x_axis(Axis::new().title("Sequence Start"))
                    .y_axis(Axis::new().title("Sequence Length")));

    let trace = Scatter::new(xs,ys).mode(Mode::Markers);
    plot.add_trace(trace);
    plot.show();
*/
}
