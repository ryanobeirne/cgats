use la::Matrix;
use cgats::*;
use gnuplot::{AxesCommon, AutoOption, Figure, PlotOption, PlotOption::*};
use lab::Lab;

use std::env::args;
use std::collections::BTreeMap;
use std::path::Path;

mod colors;
use colors::*;

macro_rules! err {
    ($($tt:tt)*) => { Err(Error::from(format!($($tt)*))) }
}

type Error = Box<std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

const USAGE: &str = "Usage:
    linviz cblinfile1.lin [cblinfile2.lin ...]";

const X_AXES: [u8; 21] = [0, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75, 80, 85, 90, 95, 100];

const AVERAGE: &str = "::AVERAGE::";

fn main() -> Result<()> {
    let mut cgv = files_to_cgats(args().skip(1));

    if cgv.is_empty() {
        eprintln!("No valid ColorBurst linearization files found!\n{}", USAGE);
        std::process::exit(1);
    }

    if let Ok(avg) = Cgats::average(cgv.values().cloned().collect::<Vec<_>>()) {
        cgv.insert(AVERAGE.into(), avg);
        eprintln!("Average Inserted");
    }

    let cbd_bt = cgats_to_cbdensity(cgv);

    // let sleep_time = sleep_time(cbd_bt.len());

    let mut fig = Figure::new();
    fig.set_terminal("qt size 1024,720", "").set_enhanced_text(false);
    // fig.set_terminal("png background rgb '#c0c0c0' size 1024,720", "derp.png").set_enhanced_text(false);
    fig.show();

    plot_cbd(&mut fig, cbd_bt);

    #[cfg(debug_assertions)] fig.echo_to_file("gnuplot.plg");
    fig.show();
    fig.close();

    Ok(())
}

fn files_to_cgats<I>(files: I) -> BTreeMap<String, Cgats> 
where I: Iterator, I::Item: Into<String> + AsRef<Path> {
    files.map(|file| { let cg = Cgats::from_file(&file); (file.into(), cg)})
    .filter(|(_file, cg)| cg.is_ok())
    .map(|(file, cg)| (file, cg.unwrap()))
    .filter(|(_, cg)| cg.is_colorburst())
    .collect::<BTreeMap<String, Cgats>>()
}

fn cgats_to_cbdensity(cgv: BTreeMap<String, Cgats>) -> BTreeMap<String, CbDensity> {
    cgv.into_iter()
        .map(|(file, cgv)| (file, CbDensity::from_cgats(&cgv)))
        .filter(|(_, cbd)| cbd.is_ok())
        .map(|(file, cbd)| (file, cbd.unwrap()))
        .collect::<BTreeMap<String, CbDensity>>()
}

// Determine largest density value
fn dmax(cbd_bt: &BTreeMap<String, CbDensity>) -> f32 {
    *cbd_bt.values()
        .map(|cbd| cbd.max_density())
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
}

fn plot_cbd(fig: &mut Figure, cbd_bt: BTreeMap<String, CbDensity>) {
    let dmax = dmax(&cbd_bt);

    for (file, cbd) in cbd_bt.iter() {
        // fig.clear_axes();
        
        let title = Path::new(&file).file_name().unwrap().to_string_lossy();

        let is_avg = file == AVERAGE;

        let (lw, ps): (PlotOption<&str>, PlotOption<&str>) = if is_avg {
            (LineWidth(4.0), PointSize(1.0))
        } else {
            (LineWidth(1.0), PointSize(0.5))
        };

        let (c, m, y, k) = if is_avg {
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
            .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(dmax.into()));

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
            let x = if is_avg {
                color.to_hex()
            } else {
                trans(&color.to_hex())
            };

            axes.lines_points(
                    &X_AXES, density,
                    &[PointSymbol('O'), Color(&x), ps, lw]
                );
        }

        // fig.show();
        // fig.save_to_png(&format!("output/{}.png", title), 1024, 720);
        // sleep(sleep_time);
    }

}

fn sleep_time(len: usize) -> u64 {
    match len {
        0 | 1 => 0,
        2..=5 => 1000,
        _ => 5000 / len as u64
    }
}

fn sleep(ms: u64) {
    std::thread::sleep(std::time::Duration::from_millis(ms));
}

fn parse_cgats_to_matrix(cgats: &Cgats, cols: usize) -> Result<Matrix<f32>> {
    let data = cgats.data_map.values()
        .flat_map(|sample| sample.values.iter())
        .filter_map(|val| val.float)
        .collect::<Vec<f32>>();

    let rows = data.len() / cols;

    if rows * cols == data.len() {
        Ok(Matrix::new(rows, cols, data))
    } else {
        Err(Box::new(std::io::Error::from(std::io::ErrorKind::InvalidInput)))
    }
}

type Density = Vec<f32>;

#[derive(Debug)]
struct CbDensity {
    cyan: Density,
    magenta: Density,
    yellow: Density,
    black: Density,
    spot: Vec<(Density, Rgb)>,
}

impl CbDensity {
    fn from_cgats(cgats: &Cgats) -> Result<CbDensity> {
        if ! cgats.is_colorburst() {
            return err!("CGATS is not ColorBurst format!")
        }

        let cols = cgats.fields.len();
        let rows = cgats.data_map.len();
        let channels = rows / 21;

        let matrix = parse_cgats_to_matrix(&cgats, cols).expect("PARSE CGATS TO MATRIX");

        let cyan    = pick(&matrix, 0, 0);
        let magenta = pick(&matrix, 1, 21);
        let yellow  = pick(&matrix, 2, 42);
        let black   = pick(&matrix, 3, 63);
        let mut spot = Vec::new();

        if channels > 4 {
            for channel in 4..channels {
                spot.push(density_rgb(&matrix, channel));
            }
        }

        Ok(CbDensity {
            cyan, magenta, yellow, black, spot
        })
    }

    fn max_density(&self) -> &f32 {
        self.cyan.iter()
            .chain(self.magenta.iter())
            .chain(self.yellow.iter())
            .chain(self.black.iter())
            .chain(self.spot.iter().flat_map(|s| s.0.iter()))
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }
}

fn density_rgb(matrix: &Matrix<f32>, channel: usize) -> (Density, Rgb) {
    let skip = channel * 21;
    let max = skip + 20;
    let maxrow = matrix.get_rows(max).get_data().to_owned();

    let (col, _dmax) = maxrow.iter().enumerate().take(4)
        .max_by(|(_a,a), (_b,b)| (a).partial_cmp(b).unwrap()).unwrap();

    let pick = pick(matrix, col, skip);

    let maxlab = maxrow.into_iter().skip(4).collect::<Vec<_>>();
    debug_assert_eq!(maxlab.len(), 3);
    let lab = Lab { l: maxlab[0], a: maxlab[1], b: maxlab[2] };

    let rgb = Rgb::from(&lab.to_rgb());

    (pick, rgb)
}

fn pick<T>(matrix: &Matrix<T>, col: usize, skip: usize) -> Vec<T> 
where T: Copy {
    matrix.get_columns(col).get_data().iter()
        .skip(skip)
        .take(21)
        .cloned()
        .collect()
}