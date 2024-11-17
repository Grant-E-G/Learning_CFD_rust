use plotters::prelude::*;
use clap::Parser;

use std::process::{Command, Stdio};
use std::io::{ BufRead, BufReader};

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

}

fn plot_frame(data: &Vec<f64>, frame_number: usize) -> Result<(), Box<dyn std::error::Error>> {
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let u_lenght = opts.grid_point_number_x as usize; 
    let x_delta = opts.total_x_delta / opts.grid_point_number_x as f64;
    let t_delta = opts.total_t_delta / opts.grid_point_number_t as f64;
    let mut u_wave_state = vec![1.0; u_lenght];
    // We are using inital conditions u is 2.0 for 0.5 <= x <= 1.0, and 1.0 otherwise
    let lower_bound = (0.5 / x_delta) as usize;
    let upper_bound = (1.0 / x_delta) as usize;
    u_wave_state[lower_bound..upper_bound].fill(2.0);
    print!("{:?} \n", u_wave_state );


    // update the wave state
    let mut wave_state_total_2d = vec![vec![0.0; u_lenght]; opts.grid_point_number_t as usize];
    // set the initial state
    wave_state_total_2d[0] = u_wave_state.clone();
    for t in 0..opts.grid_point_number_t -1{
        // Reflective boundary conditions
        wave_state_total_2d[t as usize + 1][0] = wave_state_total_2d[t as usize][u_lenght - 1];
        wave_state_total_2d[t as usize + 1][u_lenght - 1] = wave_state_total_2d[t as usize][u_lenght - 2];
        for x in 1..u_lenght - 1 {
            wave_state_total_2d[t as usize + 1][x] = wave_state_total_2d[t as usize][x] 
            *(1.0 - t_delta/x_delta * (wave_state_total_2d[t as usize][x] 
            - wave_state_total_2d[t as usize][x - 1]));
        }

    }
    for (i, data) in wave_state_total_2d.iter().enumerate() {
        plot_frame(data, i)?;
    }
    // bash command: ffmpeg -framerate 24 -i frame_%04d.png -c:v libx264 -pix_fmt yuv420p output_video.mp4
    let mut child = Command::new("bash")
    .arg("-c")
    .arg("ffmpeg -framerate 24 -i frame_%04d.png -c:v libx264 -pix_fmt yuv420p output_video_lesson2.mp4") // Example of a long-running command
    .stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn command");

    if let Some(stdout) = child.stdout.take() {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            println!("{}", line?);
        }
    }

    let status = child.wait()?;
    println!("Command exited with status: {}", status);
    println!("total wave state {:?}", wave_state_total_2d);

    Ok(())


    
}