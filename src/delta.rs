use super::*;
use deltae::{color::LabValue, DEMethod, DeltaE};

impl CgatsVec {
    pub fn can_delta(&self) -> bool {
        self.len() == 2 &&
        self.0[0].len() == self.0[1].len() &&
        self.0[0].has_lab() &&
        self.0[1].has_lab()
    }

    pub fn deltae(&self, method: DEMethod) -> CgatsResult<CgatsObject> {
        if !self.can_delta() {
            return Err(CgatsError::CannotCompare)
        }

        let dft_method = DataFormatType::from_de_method(method);

        let lab_vec0 = self.0[0].data_map.to_lab_vec()?;
        let lab_vec1 = self.0[1].data_map.to_lab_vec()?;

        let mut cgo = CgatsObject::new_with_type(CgatsType::Cgats);
        cgo.raw_vec = RawVec(vec![
            vec!["CGATS.17".to_string()],
        ]);

        cgo.data_map.0 = lab_vec0.iter()
            .enumerate()            
            .map(|(index, lab)| {
                let de = DeltaE::new(lab, &lab_vec1[index], method);
                let value = CgatsValue::from_float(de.value as CgatsFloat);
                ((index, dft_method), value)
            })
            .collect();

        let mut sample_ids = cgo.data_map.sample_id_map();
        cgo.data_map.append_column(&mut sample_ids)?;

        cgo.data_format = vec![
            DataFormatType::SAMPLE_ID,
            dft_method
        ];


        cgo.raw_vec.push(
            vec!["NUMBER_OF_SETS".to_string(), cgo.data_map.sample_count().to_string()]
        );

        Ok(cgo)
    }
}

impl CgatsMap {
    pub fn to_lab_vec(&self) -> CgatsResult<Vec<LabValue>> {
        if !self.has_lab() { return Err(CgatsError::NoData)}

        let mut lab_arr: Vec<[CgatsFloat; 3]> = Vec::new();
        lab_arr.resize(self.sample_count(), [0.0, 0.0, 0.0]);

        for (i, f, v) in self.0.iter()
        .map(|((index, format), value)| (index, format, value.float) ) {
            if *f == DataFormatType::LAB_L {
                lab_arr[*i][0] = v;
            } else if *f == DataFormatType::LAB_A {
                lab_arr[*i][1] = v;
            } else if *f == DataFormatType::LAB_B {
                lab_arr[*i][2] = v;
            }
        }

        Ok(
            lab_arr.into_iter()
                .map(|[l, a, b]|
                    LabValue {
                        l: l.into(),
                        a: a.into(),
                        b: b.into()
                    })
                .collect()
        )
    }

    pub fn has_lab(&self) -> bool {
        self.0.contains_key(&(0, DataFormatType::LAB_L)) &&
        self.0.contains_key(&(0, DataFormatType::LAB_A)) &&
        self.0.contains_key(&(0, DataFormatType::LAB_B))
    }
}

impl DataFormatType {
    pub fn from_de_method(method: DEMethod) -> DataFormatType {
        match method {
            DEMethod::DE1976  => DataFormatType::DE_1976,
            DEMethod::DE1994  => DataFormatType::DE_1994,
            DEMethod::DE1994T => DataFormatType::DE_1994T,
            DEMethod::DE2000  => DataFormatType::DE_2000,
            DEMethod::DECMC1  => DataFormatType::DE_CMC,
            DEMethod::DECMC2  => DataFormatType::DE_CMC2,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn deltae() -> CgatsResult<()> {
        let cgv = CgatsVec::from_files(&vec![
            "test_files/colorburst1.lin",
            "test_files/colorburst2.lin"
        ]);

        let delta_cgo = cgv.deltae(DEMethod::DE2000)?;

        assert_eq!(
            delta_cgo.data_map.0.get(&(125, DataFormatType::DE_2000)).unwrap().value,
            "13.8491"
        );

        println!("{}", delta_cgo.print()?);

        Ok(())
    }
}