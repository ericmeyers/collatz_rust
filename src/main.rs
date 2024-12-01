use collatz_rust::collatz_length_brian;
use plotters::prelude::*;
use std::time::Instant;

fn main()  {

  let high = 1000000000;
  //let high = 100000;

  let xs  = 1..=high;
  let points = xs.map(|n| (n,collatz_length_brian(n)) );

  let start = Instant::now();

  let root_area = BitMapBackend::new("myplot.png", (1000, 600))
    .into_drawing_area();

  root_area.fill(&WHITE).unwrap();

  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Plotters Rust", ("sans-serif", 40))
    .build_cartesian_2d(1..high, 0..900)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

  let series = points.map(|point| Circle::new(point, 1, &BLACK));

  ctx.draw_series(series).unwrap();

  let duration = start.elapsed();
  println!("Time to compute points AND plot is: {:?}", duration);

}
