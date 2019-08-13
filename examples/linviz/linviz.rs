
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

// ColorBurst linearization sample increments
const X_AXES: [u8; CB_LEN] = [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100];

// String to name Averaged ColorBurst CGATS linearizations
const AVERAGE: &str = "::AVERAGE::";

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
    let mut cbdm = CbDensityMap::from(cgv);
    if matches.is_present("normalize") {
        cbdm.normalize();
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
    let sleep_time = sleep_time(cbdm.len());
    let dmax = cbdm.dmax();
    let len = cbdm.len();
    let single = len == 1;

    for (file, cbd) in cbdm.iter() {
        if clear { fig.clear_axes(); }
        
        let is_avg = file == AVERAGE;
        let single_or_avg = is_avg || single;

        let title = if is_avg && ! clear {
            std::borrow::Cow::Borrowed(" ")
        } else {
            Path::new(&file).file_name().unwrap().to_string_lossy()
        };

        let (lw, ps): (PlotOption<&str>, PlotOption<&str>) = if single_or_avg {
            (LineWidth(4.0), PointSize(1.0))
        } else {
            (LineWidth(1.0), PointSize(0.5))
        };

        let (c, m, y, k) = if single_or_avg {
            (CYAN.into(), MAGENTA.into(), YELLOW.into(), BLACK.into())
        } else {
            (trans(CYAN), trans(MAGENTA), trans(YELLOW), trans(BLACK))
        };

        let axes = fig.axes2d()
            .set_title(&title, &[])
            .set_x_label("Input %", &[])
            .set_x_ticks(Some((AutoOption::Fix(5.0), 0)), &[], &[])
            .set_y_ticks(Some((AutoOption::Fix(0.1), 0)), &[], &[])
            .set_y_label("Output Density", &[])
            .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(X_AXES[20] as f64))
            .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(dmax.into()))
            .set_x_grid(true)
            .set_y_grid(true);

        axes.lines_points(
                &X_AXES, &cbd.cyan,
                &[PointSymbol('O'), Color(&c), ps, lw]
            )
            .lines_points(
                &X_AXES, &cbd.magenta,
                &[PointSymbol('O'), Color(&m), ps, lw]
            )
            .lines_points(
                &X_AXES, &cbd.yellow,
                &[PointSymbol('O'), Color(&y), ps, lw]
            )
            .lines_points(
                &X_AXES, &cbd.black,
                &[PointSymbol('O'), Color(&k), ps, lw]
            );

        for (density, color) in cbd.spot.iter() {
            let x = if single_or_avg {
                color.to_hex()
            } else {
                trans(&color.to_hex())
            };

            axes.lines_points(
                    &X_AXES, density,
                    &[PointSymbol('O'), Color(&x), ps, lw]
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