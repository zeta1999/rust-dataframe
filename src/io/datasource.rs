//! Data source evaluators and readers

use crate::expression::{DataSourceType, Dataset, Reader};
use arrow::csv::{Reader as CsvReader, ReaderBuilder as CsvBuilder};
use arrow::error::ArrowError;
use arrow::ipc::reader::FileReader as ArrowFileReader;
use std::fs::File;

pub trait DataSourceEval {
    fn get_dataset(&self) -> Result<Dataset, ArrowError>;
}

impl DataSourceEval for Reader {
    fn get_dataset(&self) -> Result<Dataset, ArrowError> {
        use DataSourceType::*;
        match &self.source {
            Csv(path, options) => {
                let mut builder = CsvBuilder::new()
                    .has_headers(options.has_headers)
                    .infer_schema(options.max_records)
                    .with_batch_size(options.batch_size)
                    .with_delimiter(options.delimiter.unwrap_or(b','));
                match options.projection.clone() {
                    Some(projection) => builder = builder.with_projection(projection),
                    None => {}
                };
                // TODO set schema if user has set one
                let file = File::open(&path)?;
                let csv_reader = builder.build(file)?;
                let schema = csv_reader.schema();
                Ok(Dataset {
                    name: "csv_source".to_owned(),
                    columns: schema.fields().iter().map(|f| f.clone().into()).collect(),
                })
            }
            Json(path) => panic!(),
            Parquet(path) => panic!(),
            Arrow(path) => {
                let file = File::open(&path)?;
                let reader = ArrowFileReader::try_new(file)?;
                Ok(Dataset {
                    name: "ipc_file_source".to_owned(),
                    columns: reader
                        .schema()
                        .fields()
                        .iter()
                        .map(|f| f.clone().into())
                        .collect(),
                })
            }
            Sql(table, options) => panic!(),
        }
    }
}
