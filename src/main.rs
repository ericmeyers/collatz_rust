
use itertools::iterate;
use itertools::Itertools;

use plotly::layout::{Layout, Axis};
use plotly::{Plot, Scatter};
use plotly::common::Mode;


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



fn main() {
    let xs:Vec<i64> = (1..=10000).collect();
    let ys:Vec<_> = xs.iter().map(collatz_length).collect();

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



}
