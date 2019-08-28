use super::*;
use std::fs::File;

// The density ramp for a given logical ink channel
#[derive(Debug, Clone, Copy)]
pub struct Density {
    pub inner: CbDens,
    pub rgb: Rgb,
}

impl Default for Density {
    fn default() -> Self {
        Density {
            inner: X_AXES,
            rgb: Rgb::default(),
        }
    }
}

impl Density {
    // Normalize a Density on a scale from 0 to 1
    fn normalize(&mut self) {
        let max = self.inner.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();
        let min = self.inner.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();

        for i in self.inner.iter_mut() {
            *i = (*i - min) / (max - min);
        }
    }

    /// Normalize and return a density
    ///! let normal = Density::default().normal();
    #[allow(unused)]
    fn normal(mut self) -> Self {
        self.normalize();
        self
    }

    // Cherry pick a density ramp from a Matrix at column x row
    pub fn pick_from_matrix(matrix: &Matrix<f32>, col: usize, row: usize) -> CbDens {
        let mut arr = [0_f32; CB_LEN];
        for (i, data) in matrix.get_columns(col).get_data().iter()
            .skip(row)
            .take(CB_LEN)
            .enumerate() {
                arr[i] = *data;
            }

        arr
    }

    // Get the Density Ramp and RGB color from a given density ramp in a matrix
    pub fn from_matrix(matrix: &Matrix<f32>, channel: usize) -> Self {
        // Skip to the relevant row
        let row = channel * CB_LEN;
        let max = row + CB_LEN - 1;
        let maxrow = matrix.get_rows(max).get_data().to_owned();

        // Loop through the density values in the row and find the maximum (for spot colors)
        let (col, _dmax) = maxrow.iter().enumerate().take(4)
            .max_by(|(_a,a), (_b,b)| (a).partial_cmp(b).unwrap()).unwrap();

        // Get the Lab values of the max density patch
        let maxlab = maxrow.into_iter().skip(4).collect::<Vec<_>>();
        debug_assert_eq!(maxlab.len(), 3);
        let lab = Lab { l: maxlab[0], a: maxlab[1], b: maxlab[2] };

        // Pick out the column with the highest max density
        Density {
            inner: Density::pick_from_matrix(matrix, col, row),
            rgb: Rgb::from(&lab.to_rgb()),
        }
    }
}

// The density linearization object
pub struct CbDensity {
    pub cyan: Density,
    pub magenta: Density,
    pub yellow: Density,
    pub black: Density,
    pub spot: Vec<Density>,
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
        let cyan    = Density::from_matrix(&matrix, 0);
        let magenta = Density::from_matrix(&matrix, 1);
        let yellow  = Density::from_matrix(&matrix, 2);
        let black   = Density::from_matrix(&matrix, 3);
        let mut spot = Vec::new();

        // Pick out density ramps for spot colors and determine the ink color
        if channels > 4 {
            for channel in 4..channels {
                spot.push(Density::from_matrix(&matrix, channel));
            }
        }

        Ok(CbDensity {
            cyan, magenta, yellow, black, spot
        })
    }

    // Determine the maxmimum density value in a CBDensity object for fitting axes into plots
    pub fn max_density(&self) -> &f32 {
        self.channels()
            .flat_map(|channel| channel.inner.iter())
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    // Normalize the density on a scale from 0 to 1
    pub fn normalize(&'a mut self) {
        for channel in self.channels_mut() {
            channel.normalize();
        }
    }

    // Iterate over the color channels
    pub fn channels(&'a self) -> Channels<'a> {
        self.into_iter()
    }

    // Mutable Iterator over the color channels
    pub fn channels_mut(&'a mut self) -> ChannelsMut<'a> {
        self.into_iter()
    }

    // Push a density into the spot colors
    pub fn push_spot(&'a mut self, density: Density) {
        self.spot.push(density)
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

#[test]
fn normal() {
    let default = Density::default();
    let normal = Density::default().normal();

    assert_eq!(default.inner, normal.inner);
}

impl Default for CbDensity {
    fn default() -> Self {
        CbDensity {
            cyan: Density::default(),
            magenta: Density::default(),
            yellow: Density::default(),
            black: Density::default(),
            spot: Vec::new(),
        }
    }
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


use std::iter::{Chain, Once, once};
use std::slice::IterMut;

pub type ChannelsMut<'a> = Chain<Chain<Chain<Chain<Once<&'a mut Density>, Once<&'a mut Density>>, Once<&'a mut Density>>, Once<&'a mut Density>>, IterMut<'a, Density>>;

impl<'a> IntoIterator for &'a mut CbDensity {
    type Item = &'a mut Density;
    type IntoIter = ChannelsMut<'a>;
    fn into_iter(self) -> Self::IntoIter {
        once(&mut self.cyan)
            .chain(once(&mut self.magenta))
            .chain(once(&mut self.yellow))
            .chain(once(&mut self.black))
            .chain(&mut self.spot)
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
                    density
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
    #[allow(unused)]
    pub fn insert(&mut self, key: String, val: CbDensity) -> Option<CbDensity> {
        self.inner.insert(key, val)
    }
    pub fn iter<'a>(&'a self) -> Iter {
        self.inner.iter()
    }

    // pub fn iter_mut<'a>(&'a mut self) -> IterMut {
    //     self.inner.iter_mut()
    // }

    // pub fn keys<'a>(&'a self) -> Keys {
    //     self.inner.keys()
    // }

    // Iterator over the CbDensities
    pub fn values<'a>(&'a self) -> Values {
        self.inner.values()
    }

    // Mutable iterator over the CbDensities
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

    // Normalize all the densities
    pub fn normalize<'a>(&'a mut self) {
        self.values_mut().for_each(|density| density.normalize());
    }

    // Insert a spot color density into the first CbDensity
    pub fn insert_first_spot<'a>(&'a mut self, density: Density) -> Option<()> {
        self.values_mut()
            .nth(0)?
            .push_spot(density);
        Some(())
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

impl<'a> std::iter::FromIterator<(&'a String, &'a Cgats)> for CbDensityMap {
    fn from_iter<T: IntoIterator<Item = (&'a String, &'a Cgats)>>(iter: T) -> Self {
        CbDensityMap {
            inner: iter.into_iter()
                .map(|(file, cgats)| (file, CbDensity::from_cgats(&cgats)))
                .filter(|(_file, cgats)| cgats.is_ok())
                .map(|(file, cgats)| (file.clone(), cgats.unwrap()))
                .collect()
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
    files.filter(|file| {
        let ok_file = File::open(file).is_ok();
        if !ok_file { eprintln!("'{}': Unable to open file!", file.to_string()) }
        ok_file
    })
    .filter(|file| {
        let is_dir = Path::new(file).is_dir();
        if is_dir { eprintln!("'{}': File is a directory!!", file.to_string()) }
        !is_dir
    })
    .map(|file| { let cg = Cgats::from_file(&file); (file.to_string(), cg) })
    .filter(|(file, cg)| {
        let ok_cg = cg.is_ok();
        if !ok_cg { eprintln!("'{}': Invalid CGATS format!", file) }
        ok_cg
    })
    .map(|(file, cg)| (file, cg.unwrap()))
    .filter(|(file, cg)| {
        let is_cb = cg.is_colorburst();
        if !is_cb { eprintln!("'{}': Invalid ColorBurst Linearization format!", file) }
        is_cb
    })
    .collect()
}