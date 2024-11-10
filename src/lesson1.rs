use plotters::prelude::*;
use clap::Parser;

#[derive(Parser)]
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


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let u_lenght = opts.grid_point_number_x as usize; 
    let x_delta = opts.total_x_delta / opts.grid_point_number_x as f32;
    let mut u_wave_state = vec![1.0; u_lenght];
    // We are using inital conditions u is 2.0 for 0.5 <= x <= 1.0, and 1.0 otherwise
    let lower_bound = (0.5 / x_delta) as usize;
    let upper_bound = (1.0 / x_delta) as usize;
    u_wave_state[lower_bound..upper_bound].fill(2.0);
    // doesnt seem to fill with 2.0
    print!("{:?} \n", u_wave_state );

    // show the initial state plot
    let root = BitMapBackend::new("wave_initial_state.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Wave Equation", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f32..opts.total_x_delta, 0f32..2.0).unwrap();
    chart.configure_mesh().draw().unwrap();
    chart.draw_series(LineSeries::new(
        (0..u_lenght).map(|i| (i as f32 * x_delta, u_wave_state[i])),
        &RED,
    ))?
    .label("Initial State")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw().unwrap();

    root.present().unwrap();

    Ok(())


    
}