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
    
    let y_axes = [0_f32; 125].iter().enumerate()
        .map(|(i,_)| i as f32)
        .collect::<Vec<f32>>();

    let mut fig = gnuplot::Figure::new();
    fig.set_terminal("qt noenhanced size 1024,720", "");

    for (file, cgats) in cgv.iter() {
        fig.clear_axes();
        
        let title = Path::new(file).file_name().unwrap().to_string_lossy();
        let matrix = parse_cgats_to_matrix(cgats, cgats.fields.len())?;

        fig.axes2d()
            .set_x_label("Input %", &[])
            .set_x_ticks(Some((AutoOption::Auto, 0)), &[], &[])
            .set_y_ticks(Some((AutoOption::Fix(0.1), 0)), &[], &[])
            .set_y_label("Output %", &[])
            .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(125.0))
            .set_y_range(AutoOption::Fix(0.0), AutoOption::Fix(1.5))
            .lines_points(
                y_axes.clone(), matrix.get_columns(3).get_data(),
                &[Caption(&title), PointSymbol('O'), Color("black"), PointSize(0.5)]
            );

        fig.set_pre_commands("set termoption noenhanced");

        std::thread::sleep_ms(100);

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