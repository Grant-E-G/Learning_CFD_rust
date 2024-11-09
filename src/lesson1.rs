use plotters::prelude::*;
use clap::Parser;

struct Opts {
    #[clap(long, default_value_t = 100)]
    grid_point_number_x: u32,
    #[clap(long, default_value_t = 2.0)]
    total_x_delta: f32,
    #[clap(long, default_value_t = 100)]
    grid_point_number_t: u32,
    #[clap(long, default_value_t = 1.0)]
    total_t_delta: f32,
    #[clap(long, default_value_t = 1.0)]
    wave_speed: f32,


}


fn main() {
    let opts: Opts = Opts::parse();
    let u_lenght = opts.grid_point_number_x as usize; 
    let x_delta = opts.total_x_delta / opts.grid_point_number_x as f32;
    let mut u_wave_state = vec![1.0; u_lenght];
    // We are using inital conditions u is 2.0 for 0.5 <= x <= 1.0, and 1.0 otherwise
    let lower_bound
    
}