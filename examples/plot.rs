// cargo run --features plotters --example plot

#[cfg(feature = "plotters")]
use plotters::prelude::*;

#[cfg(feature = "plotters")]
const OUT_FILE_NAME: &str = "out.png";
#[cfg(feature = "plotters")]
const N: isize = 50;

#[cfg(feature = "plotters")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 640)).into_drawing_area();

    root.fill(&WHITE)?;

    let chart = ChartBuilder::on(&root)
        .caption("Gaussian primes", ("sans-serif", 24).into_font())
        .margin(10)
        .build_cartesian_2d(0..N, 0..N)?;

    // chart.configure_mesh().draw()?;

    let plotting_area = chart.plotting_area();

    let primes = gaussiant::get_g_primes(N);
    for p in primes {
        println!("{p}");
        let x = p.0.re;
        let y = p.0.im;
        plotting_area.draw_pixel((x, y), &BLACK)?;
    }

    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}

#[cfg(not(feature = "plotters"))]
fn main() {}
