use la::Matrix;
use cgats::*;
use gnuplot::{AxesCommon, AutoOption, PlotOption::*};
use lab::Lab;

use std::env::args;
use std::collections::BTreeMap;
use std::path::Path;

macro_rules! err {
    ($($tt:tt)*) => { Err(Error::from(format!($($tt)*))) }
}

type Error = Box<std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let cgv = args()
        .skip(1)
        .map(|arg| (arg.clone(), Cgats::from_file(arg)))
        .filter(|(_file, cg)| cg.is_ok())
        .map(|(file, cg)| (file, cg.unwrap()))
        .filter(|(_, cg)| cg.is_colorburst())
        .collect::<BTreeMap<_,_>>();
    
    let y_axes = [0_f32; 21].iter().enumerate()
        .map(|(i,_)| i as f32)
        .collect::<Vec<f32>>();

    let mut fig = gnuplot::Figure::new();
    fig.set_terminal("qt noenhanced size 1024,720", "");
    fig.set_pre_commands("set object rectangle from screen 0,0 to screen 1,1 behind fillcolor rgb '#000000' fillstyle solid noborder");
    fig.show();

    for (file, cgats) in cgv.iter() {
        fig.clear_axes();
        
        let title = Path::new(file).file_name().unwrap().to_string_lossy();
        let cbd = CbDensity::from_cgats(cgats)?;

        let axes = fig.axes2d()
            .set_x_label("Input %", &[])
            .set_x_ticks(Some((AutoOption::Fix(1.0), 0)), &[], &[])
            .set_y_ticks(Some((AutoOption::Fix(0.1), 0)), &[], &[])
            .set_y_label("Output Density", &[])
            .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix((y_axes.len() - 1) as f64))
            .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(2.0));

        axes.lines_points(
                y_axes.clone(), cbd.cyan,
                &[Caption(&title), PointSymbol('O'), Color("cyan"), PointSize(0.5)]
            )
            .lines_points(
                y_axes.clone(), cbd.magenta,
                &[PointSymbol('O'), Color("magenta"), PointSize(0.5)]
            )
            .lines_points(
                y_axes.clone(), cbd.yellow,
                &[PointSymbol('O'), Color("yellow"), PointSize(0.5)]
            )
            .lines_points(
                y_axes.clone(), cbd.black,
                &[PointSymbol('O'), Color("black"), PointSize(0.5)]
            );

        for (density, color) in cbd.spot.iter() {
            axes.lines_points(
                    y_axes.clone(), density,
                    &[PointSymbol('O'), Color(&color.to_hex()), PointSize(0.5)]
                );
        }

        sleep(250);
        fig.show();
    }

    fig.close();

    Ok(())
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

#[derive(Debug)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

impl From<&[u8; 3]> for Rgb {
    fn from(a: &[u8; 3]) -> Rgb {
        Rgb {
            red: a[0], green: a[1], blue: a[2] 
        }
    }
}

#[test]
fn rgbhex() {
    let rgb = Rgb { red: 0, green: 128, blue: 255 };
    assert_eq!(rgb.to_hex(), "#0080FF");
}