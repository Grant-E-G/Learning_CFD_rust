use plotters::prelude::*;
use clap::Parser;

use std::process::{Command, Stdio};
use std::io::{self, BufRead, BufReader};

#[derive(Parser)]
struct Opts {
    #[clap(long, default_value_t = 100)]
    grid_point_number_x: u64,
    #[clap(long, default_value_t = 2.0)]
    total_x_delta: f64,
    #[clap(long, default_value_t = 100)]
    grid_point_number_t: u64,
    #[clap(long, default_value_t = 1.0)]
    total_t_delta: f64,
    #[clap(long, default_value_t = 1.0)]
    wave_speed: f64,


}

fn plot_wave_frame(data: &Vec<f64>, frame_number: usize) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("frame_{:04}.png", frame_number);
    let root = BitMapBackend::new(&file_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Wave Equation Dynamics", ("sans-serif", 50))
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..data.len(), -3.0..3.0)?;

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        data.iter().enumerate().map(|(x, &y)| (x, y)),
        &RED,
    ))?;
    Ok(())
}
