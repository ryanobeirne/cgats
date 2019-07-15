use la::Matrix;
use cgats::*;
use gnuplot::{AxesCommon, AutoOption, PlotOption::*};

use std::env::args;
use std::collections::BTreeMap;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let cgv = args()
        .skip(1)
        .map(|arg| (arg.clone(), Cgats::from_file(arg)))
        .filter(|(_file, cgats)| cgats.is_ok())
        .map(|(file, cgats)| (file, cgats.unwrap()))
        .collect::<BTreeMap<_,_>>();
    
    let y_axes = [0_f32; 21].iter().enumerate()
        .map(|(i,_)| i as f32)
        .collect::<Vec<f32>>();

    let mut fig = gnuplot::Figure::new();
    fig.set_terminal("qt noenhanced size 1024,720", "");

    for (file, cgats) in cgv.iter() {
        fig.clear_axes();
        
        let title = Path::new(file).file_name().unwrap().to_string_lossy();
        let cbd = CbDensity::from(cgats);

        fig.axes2d()
            .set_x_label("Input %", &[])
            .set_x_ticks(Some((AutoOption::Auto, 0)), &[], &[])
            .set_y_ticks(Some((AutoOption::Fix(0.1), 0)), &[], &[])
            .set_y_label("Output %", &[])
            .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(y_axes.len() as f64))
            .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(1.5))
            .lines_points(
                y_axes.clone(), cbd.cyan,
                &[Caption(&title), PointSymbol('O'), Color("cyan"), PointSize(0.5)]
            )
            .lines_points(
                y_axes.clone(), cbd.magenta,
                &[Caption(""), PointSymbol('O'), Color("magenta"), PointSize(0.5)]
            )
            .lines_points(
                y_axes.clone(), cbd.yellow,
                &[Caption(""), PointSymbol('O'), Color("yellow"), PointSize(0.5)]
            )
            .lines_points(
                y_axes.clone(), cbd.black,
                &[Caption(""), PointSymbol('O'), Color("black"), PointSize(0.5)]
            );

        std::thread::sleep_ms(250);

        fig.show();
    }

    fig.close();

    Ok(())
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
    spot: Vec<Density>,
}

impl CbDensity {

}

impl From<&Cgats>  for CbDensity {
    fn from(cgats: &Cgats) -> CbDensity {
        let cols = cgats.fields.len();
        let rows = cgats.data_map.len();
        let channels = rows / 21;
        debug_assert_eq!(cols, 7);
        debug_assert_eq!(rows % 21, 0);
        debug_assert!(channels >= 4);

        let matrix = parse_cgats_to_matrix(&cgats, cols).expect("PARSE CGATS TO MATRIX");

        let cyan = matrix.get_columns(0).get_data().iter().take(21).cloned().collect();
        let magenta = matrix.get_columns(1).get_data().iter().skip(21).take(21).cloned().collect();
        let yellow = matrix.get_columns(2).get_data().iter().skip(42).take(21).cloned().collect();
        let black = matrix.get_columns(3).get_data().iter().skip(63).take(21).cloned().collect();

        CbDensity {
            cyan, magenta, yellow, black,
            spot: vec![]
        }
    }
}