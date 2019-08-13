use super::*;

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

impl CbDensity {
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
        self.cyan.iter()
            .chain(self.magenta.iter())
            .chain(self.yellow.iter())
            .chain(self.black.iter())
            .chain(self.spot.iter().flat_map(|s| s.0.iter()))
            .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    // Normalize the density on a scale from 0 to 1
    pub fn normalize(mut self) -> Self {
        norm(&mut self.cyan);
        norm(&mut self.magenta);
        norm(&mut self.yellow);
        norm(&mut self.black);

        for (density, _rgb) in self.spot.iter_mut() {
            norm(density);
        }

        self
    }
}

// Normalize a slice of f32 on a scale from 0 to 1
fn norm(v: &mut [f32]) {
    let max = v.iter().max_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();
    let min = v.iter().min_by(|a,b| a.partial_cmp(b).unwrap()).unwrap().clone();

    for i in v.iter_mut() {
        *i = (*i - min) / (max - min);
    }
}

#[test]
fn normal() {
    let normal = &mut [1.5, 2.8, 6.3, 7.2, 10.8];
    norm(normal);
    let expected = &[0.0, 0.13978493, 0.516129, 0.6129032, 1.0];

    assert_eq!(normal, expected);
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
where I: Iterator, I::Item: Into<String> + AsRef<Path> {
    files.map(|file| { let cg = Cgats::from_file(&file); (file.into(), cg)})
    .filter(|(_file, cg)| cg.is_ok())
    .map(|(file, cg)| (file, cg.unwrap()))
    .filter(|(_, cg)| cg.is_colorburst())
    .collect::<BTreeMap<String, Cgats>>()
}

// Map CGATS objects to Density Matrices
pub fn cgats_to_cbdensity(cgv: BTreeMap<String, Cgats>, norm: bool) -> BTreeMap<String, CbDensity> {
    cgv.into_iter()
        .map(|(file, cgv)| (file, CbDensity::from_cgats(&cgv)))
        .filter(|(_, cbd)| cbd.is_ok())
        .map(|(file, cbd)| (file, {
            if norm {
                cbd.unwrap().normalize()
            } else {
                cbd.unwrap()
            }}))
        .collect::<BTreeMap<String, CbDensity>>()
}

// Determine largest density value for sizing the plot axes to fit
pub fn dmax(cbd_bt: &BTreeMap<String, CbDensity>) -> f32 {
    *cbd_bt.values()
        .map(|cbd| cbd.max_density())
        .max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
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