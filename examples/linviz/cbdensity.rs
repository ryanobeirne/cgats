use super::*;
use std::fs::File;

// The density ramp for a given logical ink channel
pub type Density = Vec<f32>;

// The density linearization object
pub struct CbDensity {
    pub cyan: Density,
    pub magenta: Density,
    pub yellow: Density,
    pub black: Density,
    pub spot: Vec<(Density, Rgb)>,
}

impl<'a> CbDensity {
    // Convert ColorBurst CGATS to a density matrix that gnuplot can work with
    pub fn from_cgats(cgats: &Cgats) -> Result<CbDensity> {
        if ! cgats.is_colorburst() {
            return err!("CGATS is not ColorBurst format!")
        }

        // Determine how many channels are in this linearization
        let cols = cgats.fields.len();
        let rows = cgats.data_map.len();
        let channels = rows / CB_LEN;

        if channels < 4 {
            return err!("ColorBurst linearization files must contain a minimum of CMYK channels!");
        }

        let matrix = parse_cgats_to_matrix(&cgats, cols)?;

        // Pick out the density ramp for the given channel
        let cyan    = pick(&matrix, 0, 0);
        let magenta = pick(&matrix, 1, 21);
        let yellow  = pick(&matrix, 2, 42);
        let black   = pick(&matrix, 3, 63);
        let mut spot = Vec::new();

        // Pick out density ramps for spot colors and determine the ink color
        if channels > 4 {
            for channel in 4..channels {
                spot.push(density_rgb(&matrix, channel));
            }
        }

        Ok(CbDensity {
            cyan, magenta, yellow, black, spot
        })
    }

    // Determine the maxmimum density value in a CBDensity object for fitting axes into plots
    pub fn max_density(&self) -> &f32 {
        self.channels()
            .flat_map(|channel| channel.iter())
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    // Normalize the density on a scale from 0 to 1
    pub fn normalize(&mut self) {
        norm(&mut self.cyan);
        norm(&mut self.magenta);
        norm(&mut self.yellow);
        norm(&mut self.black);

        for (density, _rgb) in self.spot.iter_mut() {
            norm(density);
        }
    }

    pub fn channels(&'a self) -> Channels<'a> {
        self.into_iter()
    }
}

#[test]
fn channel_test() {
    let mut cbd = CbDensity::from_cgats(&Cgats::from_file("test_files/colorburst1.lin").unwrap()).unwrap();
    cbd.normalize();
    for (i, channel) in cbd.channels().enumerate() {
        eprintln!("{}: {:?}", i, channel);
    }
}

// Normalize a slice of f32 on a scale from 0 to 1
fn norm<T>(v: &mut [T])
where T: num::Float {
    let max = v.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();
    let min = v.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();

    for i in v.iter_mut() {
        *i = (*i - min) / (max - min);
    }
}

#[test]
fn normal() {
    let normal = &mut [1.5_f32, 2.8, 6.3, 7.2, 10.8];
    norm(normal);
    let expected = &[0.0, 0.13978493, 0.516129, 0.6129032, 1.0];

    assert_eq!(normal, expected);
}

impl<'a> IntoIterator for &'a CbDensity {
    type Item = &'a Density;
    type IntoIter = Channels<'a>;
    fn into_iter(self) -> Self::IntoIter {
        Channels {
            inner: self,
            index: 0,
        }
    }
}

/// Iterator over logical channels in a ColorBurst linearization
pub struct Channels<'a> {
    inner: &'a CbDensity,
    index: usize,
}

impl<'a> Iterator for Channels<'a> {
    type Item = &'a Density;
    fn next(&mut self) -> Option<Self::Item> {
        let channel = match self.index {
            0 => &self.inner.cyan,
            1 => &self.inner.magenta,
            2 => &self.inner.yellow,
            3 => &self.inner.black,
            i => {
                if let Some(density) = self.inner.spot.get(i - 4) {
                    &density.0
                } else {
                    return None;
                }
            },
        };

        self.index += 1;

        Some(channel)
    }
}

use std::collections::btree_map as bt;
type Iter<'a> = bt::Iter<'a, String, CbDensity>;
// type IterMut<'a> = bt::IterMut<'a, String, CbDensity>;
type Values<'a> = bt::Values<'a, String, CbDensity>;
type ValuesMut<'a> = bt::ValuesMut<'a, String, CbDensity>;
// type Keys<'a> = bt::Keys<'a, String, CbDensity>;

pub struct CbDensityMap {
    inner: BTreeMap<String, CbDensity>,
}

impl CbDensityMap {
    pub fn iter<'a>(&'a self) -> Iter {
        self.inner.iter()
    }

    // pub fn iter_mut<'a>(&'a mut self) -> IterMut {
    //     self.inner.iter_mut()
    // }

    // pub fn keys<'a>(&'a self) -> Keys {
    //     self.inner.keys()
    // }

    pub fn values<'a>(&'a self) -> Values {
        self.inner.values()
    }

    pub fn values_mut<'a>(&'a mut self) -> ValuesMut<'a> {
        self.inner.values_mut()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    // Determine the largest density value for setting plot bounds
    pub fn dmax(&self) -> f32 {
        *self.values()
            .map(|cbd| cbd.max_density())
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    pub fn normalize<'a>(&'a mut self) {
        self.values_mut().for_each(|density| density.normalize());
    }
}

impl From<BTreeMap<String, Cgats>> for CbDensityMap {
    fn from(cgv: BTreeMap<String, Cgats>) -> Self {
        cgv.into_iter()
            .map(|(file, cgv)| (file, CbDensity::from_cgats(&cgv)))
            .filter(|(_, cbd)| cbd.is_ok())
            .map(|(file, cbd)| (file, cbd.unwrap()))
            .collect()
    }
}

impl std::iter::FromIterator<(String, CbDensity)> for CbDensityMap {
    fn from_iter<T: IntoIterator<Item = (String, CbDensity)>>(iter: T) -> Self {
        CbDensityMap {
            inner: iter.into_iter().collect()
        }
    }
}

// Parse a CGATS object into a density Matrix
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

// Map file strings to CGATS objects
pub fn files_to_cgats<I>(files: I) -> BTreeMap<String, Cgats>
where I: Iterator, I::Item: ToString + AsRef<Path> + AsRef<std::ffi::OsStr> {
    files.filter(|file| Path::new(file).is_file())
    .filter(|f| File::open(f).is_ok())
    .map(|file| { let cg = Cgats::from_file(&file); (file.to_string(), cg)})
    .filter(|(_file, cg)| cg.is_ok())
    .map(|(file, cg)| (file, cg.unwrap()))
    .filter(|(_, cg)| cg.is_colorburst())
    .collect::<BTreeMap<String, Cgats>>()
}

// Get the Density Ramp and RGB color from a given density ramp in a matrix
pub fn density_rgb(matrix: &Matrix<f32>, channel: usize) -> (Density, Rgb) {
    // Skip to the relevant row
    let row = channel * CB_LEN;
    let max = row + CB_LEN - 1;
    let maxrow = matrix.get_rows(max).get_data().to_owned();

    // Loop through the density values in the row and find the maximum (for spot colors)
    let (col, _dmax) = maxrow.iter().enumerate().take(4)
        .max_by(|(_a,a), (_b,b)| (a).partial_cmp(b).unwrap()).unwrap();

    // Pick out the column with the highest max density
    let pick = pick(matrix, col, row);

    // Get the Lab values of the max density patch
    let maxlab = maxrow.into_iter().skip(4).collect::<Vec<_>>();
    debug_assert_eq!(maxlab.len(), 3);
    let lab = Lab { l: maxlab[0], a: maxlab[1], b: maxlab[2] };

    // Convert Lab to RGB
    let rgb = Rgb::from(&lab.to_rgb());

    (pick, rgb)
}

// Cherry pick a density ramp from a Matrix at column x row
pub fn pick<T>(matrix: &Matrix<T>, col: usize, row: usize) -> Vec<T>
where T: Copy {
    matrix.get_columns(col).get_data().iter()
        .skip(row)
        .take(CB_LEN)
        .cloned()
        .collect()
}