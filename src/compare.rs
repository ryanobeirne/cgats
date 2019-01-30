use super::*;

extern crate deltae;
use deltae::*;

use std::str::FromStr;
use std::path::Path;

impl Cgats {
    fn new_with_fields(fields: DataFormat) -> Cgats {
        Cgats {
            vendor: Some(Vendor::Cgats),
            meta: DataVec::new(),
            fields,
            data_map: DataMap::new(),
        }
    }

    fn derive(&self) -> Cgats {
        Cgats {
            vendor: self.vendor,
            meta: self.meta.clone(),
            fields: self.fields.clone(),
            data_map: DataMap::new(),
        }
    }

    fn reindex_sample_id(&mut self) {
        let sid_index = self.fields.iter()
            .position(|f| *f == Field::SAMPLE_ID);

        match sid_index {
            Some(index) => {
                for (key, value) in self.data_map.iter_mut(){
                    value.values[index] = CgatsValue::from_str(&key.to_string())
                        .expect("Cannot parse value from key <usize>!");
                }
            },
            None => ()
        }

    }

    fn insert_sample_id(&mut self) {
        let sid_index = self.fields.iter()
            .position(|f| *f == Field::SAMPLE_ID);

        match sid_index {
            Some(_) => {
                self.reindex_sample_id();
            },
            None => {
                self.fields.insert(0, Field::SAMPLE_ID);
                for (key, value) in self.data_map.iter_mut(){
                    value.values.insert(0,
                        CgatsValue::from_str(&key.to_string())
                            .expect("Cannot parse value from key <usize>!")
                    );
                }
            },
        }
    }

    fn has_lab(&self) -> bool {
        self.fields.contains(&Field::LAB_L) &&
        self.fields.contains(&Field::LAB_A) &&
        self.fields.contains(&Field::LAB_B)
    }
}

#[test]
fn reindex() -> CgatsResult<()> {
    let mut cgo = Cgats::from_file("test_files/colorburst0.txt")?;
    cgo.insert_sample_id();
    println!("{}", cgo.write());

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
pub struct CgatsVec {
    collection: Vec<Cgats>,
}

impl CgatsVec {
    pub fn from_files<P: AsRef<Path>>(files: &Vec<P>) -> CgatsVec {
        CgatsVec {
            collection: files.iter()
                .filter_map(|f| Cgats::from_file(f).ok())
                .collect()
        }
    }

    fn can_compare(&self) -> CgatsResult<()> {
        if self.collection.is_empty() {
            return Err(CgatsError::NoData);
        }
        
        let prime = &self.collection[0];

        if ! self.collection.iter().all(|c|
            c.sample_count() == prime.sample_count() ||
            c.fields == prime.fields
        ) {
            return Err(CgatsError::CannotCompare);
        }

        Ok(())
    }

    fn can_delta(&self) -> bool {
        self.collection.len() == 2 &&
        self.collection[0].sample_count() == self.collection[1].sample_count() &&
        self.collection.iter().all(|cgo| cgo.has_lab())
    }

    fn same_fields(&self) -> bool {
        self.collection.iter()
            .map(|cgo| &cgo.fields)
            .all(|fields| fields == &self.collection[0].fields)
    }

    pub fn average(&self) -> CgatsResult<Cgats> {
        self.can_compare()?;

        let len = self.collection.len();
        if len == 1 {
            return Ok(self.collection[0].clone())
        }
        
        let prime = &self.collection[0];
        let mut cgats = prime.derive();

        for cgo in &self.collection {
            for (key, sample) in cgo.data_map.iter() {
                let div_sample = sample.divide_values(len);
                let prime_sample = prime.data_map.get(key).expect("Map does not contain key!");
                let entry = cgats.data_map.entry(*key)
                    .or_insert(prime_sample.zero());

                *entry = entry.add_values(&div_sample);
            }
        }

        cgats.meta.lines[0].raw_samples.push(format!("Average of {}", len));

        Ok(cgats)
    }

    pub fn concatenate(&self) -> CgatsResult<Cgats> {
        if !self.same_fields() {
            return Err(CgatsError::CannotCompare);
        }

        match self.collection.first() {
            Some(cgo) => {
                let mut new = cgo.clone();
                for other in self.collection.iter().skip(1) {
                    for sample in other.data_map.values() {
                        new.data_map.insert(new.data_map.len(), sample.clone());
                    }
                }
                new.reindex_sample_id();
                new.meta.meta_renumber_sets(new.data_map.len());
                Ok(new)
            },
            None => Err(CgatsError::NoData),
        }
    }

    pub fn deltae(&self, method: DEMethod) -> CgatsResult<Cgats> {
        if !self.can_delta() {
            return Err(CgatsError::CannotCompare);
        }

        let mut cgats = Cgats::new_with_fields(vec![
            Field::SAMPLE_ID, Field::from_de_method(method)
        ]);

        cgats.vendor = Some(Vendor::Cgats);
        cgats.meta = DataVec {
            lines: vec![DataLine {
                raw_samples: vec!["CGATS.17".to_string()]
            }]
        };

        let (sample0, sample1) = (&self.collection[0], &self.collection[1]);

        // We can unwrap these because `self.can_delta()` already vetted that both samples contain LAB
        let lab0_indexes = Field::lab_indexes(&sample0.fields).expect("Cannot find LAB in fields!");
        let lab1_indexes = Field::lab_indexes(&sample1.fields).expect("Cannot find LAB in fields!");

        for (index, sample) in sample0.data_map.iter() {
            let lab0 = sample.to_lab(&lab0_indexes)
                .expect("Cannot find LAB in fields!");
            let lab1 = sample1.data_map
                .get(index).expect("Key doesn't exist in map!")
                .to_lab(&lab1_indexes).expect("Cannot find LAB in fields!");
            let de = DeltaE::new(&lab0, &lab1, method);
            cgats.data_map.insert(*index,
                Sample {
                    values: vec![
                        CgatsValue::from_str(&index.to_string())?,
                        CgatsValue::from_float(de.value as Float),
                    ]
                });
        }

        Ok(cgats)
    }
}

#[test]
fn average_cgats() -> CgatsResult<()> {
    let cgv = CgatsVec::from_files(&vec![
        "test_files/cgats1.tsv", "test_files/cgats2.tsv"
    ]);
    let avg = cgv.average()?;

    let expected = Cgats::from_file("test_files/cgats5.tsv")?;

    println!("{}", avg.write());

    assert_eq!(avg.data_map, expected.data_map);
    Ok(())
}

#[test]
fn average_cb() -> CgatsResult<()> {
    let cgv = CgatsVec::from_files(&vec![
        "test_files/colorburst1.lin", "test_files/colorburst2.lin"
    ]);
    let avg = cgv.average()?;

    let expected = Cgats::from_file("test_files/colorburst3.lin")?;

    println!("{}", avg.write());

    assert_eq!(avg.data_map, expected.data_map);
    Ok(())
}

#[test]
fn cat_cgats() -> CgatsResult<()> {
    let cgv = CgatsVec::from_files(&vec![
        "test_files/cgats1.tsv", "test_files/cgats2.tsv"
    ]);
    let cat = cgv.concatenate()?;

    println!("{}", cat.write());

    assert_eq!(cat.data_map.keys().last(), Some(&21));
    assert_eq!(cat.data_map.len(), 22);
    Ok(())
}

#[test]
fn deltae() -> CgatsResult<()> {
    let cgv = CgatsVec::from_files(&vec![
        "test_files/colorburst2.lin", "test_files/colorburst3.lin"
    ]);
    let de_cgo = cgv.deltae(deltae::DEMethod::DE2000)?;
    let temp = test::mktemp()?;
    de_cgo.write_to_file(&temp)?;

    let reconstructed = Cgats::from_file(&temp)?;
    let expected = Cgats::from_file("test_files/deltae0.txt")?;

    assert_eq!(reconstructed.data_map, expected.data_map);
    std::fs::remove_file(temp)?;
    Ok(())
}