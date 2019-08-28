
use la::Matrix;
use cgats::*;
use gnuplot::{AxesCommon, AutoOption, Figure, PlotOption, PlotOption::*};
use lab::Lab;

use std::collections::BTreeMap;
use std::path::Path;

#[macro_use]
extern crate clap;
mod cli;

mod colors;
use colors::*;

mod cbdensity;
use cbdensity::*;

#[macro_export]
macro_rules! err {
    ($($tt:tt)*) => { Err(Error::from(format!($($tt)*))) }
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

// The number of density samples in a ColorBurst linearization channel
const CB_LEN: usize = 21;
pub type CbDens = [f32; CB_LEN];

// ColorBurst linearization sample increments
const X_AXES: CbDens = [0.0, 0.05, 0.10, 0.15, 0.20, 0.25, 0.30, 0.35, 0.40, 0.45, 0.50, 0.55, 0.60, 0.65, 0.70, 0.75, 0.80, 0.85, 0.90, 0.95, 1.0];

// String to name Averaged ColorBurst CGATS linearizations
const AVERAGE: &str = "{::AVERAGE::}";

fn main() -> Result<()> {
    let matches = cli::app().get_matches();


    // Collect file arguments as CGATS
    let mut cgv = files_to_cgats(matches.values_of("files").unwrap());

    // Exit if we have no CGATS to work with
    if cgv.is_empty() {
        eprintln!("No valid ColorBurst linearization files found!");
        std::process::exit(1);
    }

    // If we can average them all, add the average to the plot
    if cgv.len() > 1 {
        if let Ok(avg) = Cgats::average(cgv.values().cloned().collect::<Vec<_>>()) {
            cgv.insert(AVERAGE.into(), avg);
            eprintln!("Average of {} Inserted", cgv.len() - 1);
        }
    }

    // Convert the ColorBurst CGATS density to a format that gnuplot can plot
    let mut cbdm: CbDensityMap = cgv.iter().collect();
    if matches.is_present("normalize") {
        eprintln!("Normalizing...");
        cbdm.normalize();
        // Insert the default linear density curve
        cbdm.insert_first_spot(Density::default());
    }

    // Make the gnuplot Figure
    let mut fig = Figure::new();
    fig.set_terminal("qt size 1024,720", "").set_enhanced_text(false);
    fig.show();

    // Plot the density to the Figure
    plot_cbd(&mut fig, cbdm, matches.is_present("clear"));

    // // Generate the info that's going into gnuplot
    // #[cfg(debug_assertions)] fig.echo_to_file("gnuplot.plg");

    // Show the figure with plotted axes and quit plotting;
    fig.show();
    fig.close();

    Ok(())
}

// Plot the density to the figure
fn plot_cbd(fig: &mut Figure, cbdm: CbDensityMap, clear: bool) {
    let dmax = cbdm.dmax();
    let len = cbdm.len();
    let sleep_time = sleep_time(len);
    let single = len == 1;

    // Loop through the CbDensities
    for (file, cbd) in cbdm.iter() {
        if clear { fig.clear_axes(); }
        
        let is_avg = file == AVERAGE;
        let single_or_avg = is_avg || single;

        // Set an empty title for non-files
        let title = if is_avg && ! clear {
            std::borrow::Cow::Borrowed(" ")
        } else {
            Path::new(&file).file_name().unwrap().to_string_lossy()
        };

        // Determine the Line Width and Point Size
        let (lw, ps): (PlotOption<&str>, PlotOption<&str>) = if single_or_avg {
            (LineWidth(4.0), PointSize(1.0))
        } else {
            (LineWidth(1.0), PointSize(0.5))
        };

        // Setup the axes
        let axes = fig.axes2d()
            .set_title(&title, &[])
            .set_x_label("Input %", &[])
            .set_x_ticks(Some((AutoOption::Fix(0.05), 0)), &[], &[])
            .set_y_ticks(Some((AutoOption::Fix(0.05), 0)), &[], &[])
            .set_y_label("Output Density", &[])
            .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(X_AXES[CB_LEN - 1] as f64))
            .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(dmax.into()))
            .set_x_grid(true)
            .set_y_grid(true);

        // Loop through the channels
        for channel in cbd.channels() {
            // Determine channel plot line color
            let color = if single_or_avg {
                channel.rgb.to_hex_solid()
            } else {
                channel.rgb.to_hex_trans()
            };

            // Plot the channel
            axes.lines_points(
                &X_AXES, &channel.inner,
                &[PointSymbol('O'), Color(&color), ps, lw]
            );
        }

        // fig.save_to_png(&format!("output/{}.png", title), 1024, 720);
        if clear {
            fig.show();
            sleep(sleep_time);
        }
    }

}

// Determine how long to sleep between axes plots
fn sleep_time(len: usize) -> u64 {
    match len {
        0 | 1 => 0,
        2..=5 => 1000,
        _ => 5000 / len as u64
    }
}

// Sleep for milliseconds
fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}