use pyo3::prelude::*;
use umya_spreadsheet::{reader, Spreadsheet};

use crate::sample_ext_lib::sheet::Sheet;

#[pyclass]
pub struct Book {
    #[pyo3(get, set)]
    pub path: String,
    value: Spreadsheet
}

#[pymethods]
impl Book {
    #[new]
    pub fn new(path: String) -> Self {
        let _path = std::path::Path::new(&path);
        let book = reader::xlsx::read(_path).unwrap();
        Book { path, value: book }
    }

    pub fn __repr__(&self) -> String {
        format!("<Book path='{}'>", self.path)
    }

    #[getter(sheetnames)]
    pub fn sheetnames(&self) -> Vec<String> {
        self.value.get_sheet_collection()
            .iter()
            .map(|sheet| sheet.get_name().to_string())
            .collect()
    }

    pub fn __iter__(&self) -> Vec<Sheet> {
        self.value.get_sheet_collection()
            .iter()
            .map(|sheet| Sheet::new(sheet.get_name().to_string(), sheet.clone()))
            .collect()
    }

    pub fn get_sheet_by_name(&self, name: String) -> Sheet {
        self.get_sheet_by_name_ref(&name)
            .unwrap_or_else(|| panic!("Sheet '{}' not found", name))
    }

    fn get_sheet_by_index(&self, index: usize) -> Sheet {
        self.get_sheet_by_index_ref(&index)
            .unwrap_or_else(|| panic!("Sheet at index '{}' not found", index))
    }
}

impl Book {
    pub fn get_sheet_by_name_ref(&self, name: &String) -> Option<Sheet> {
        self.value.get_sheet_by_name(&name)
            .map(|sheet| Sheet::new(sheet.get_name().to_string(), sheet.clone()))
    }

    pub fn get_sheet_by_index_ref(&self, index: &usize) -> Option<Sheet> {
        self.value.get_sheet(index)
            .map(|sheet| Sheet::new(sheet.get_name().to_string(), sheet.clone()))
    }
}


