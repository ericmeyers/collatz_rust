use collatz_rust::collatz_length_brian;
use plotters::prelude::*;

fn main()  {

  let high = 1000000000;

  let xs: Vec<i32> = (1..=high).collect();
  let points = xs.into_iter().map(|n| {let y = collatz_length_brian(n as i64); (n,y)});
  let root_area = BitMapBackend::new("myplot.png", (1200, 800))
    .into_drawing_area();

  root_area.fill(&WHITE).unwrap();


  let mut ctx = ChartBuilder::on(&root_area)
    .set_label_area_size(LabelAreaPosition::Left, 40)
    .set_label_area_size(LabelAreaPosition::Bottom, 40)
    .caption("Plotters Rust", ("sans-serif", 40))
    .build_cartesian_2d(1..high, 0..700)
    .unwrap();

  ctx.configure_mesh().draw().unwrap();

  let series = points.map(|point| Circle::new(point, 1, &BLACK));

  ctx.draw_series(series).unwrap();



}
