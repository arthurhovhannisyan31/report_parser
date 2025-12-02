use clap::Parser;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::ops::DerefMut;

use parser::errors::{ParsingError, SerializeError};
use parser::parsers::{BinRecord, CsvRecord, TxtRecord, csv::CVS_HEADERS};
use parser::record::{BankRecord, BankRecordParser};

mod configs;
mod errors;
use crate::configs::{CliArgs, DataFormat};
use crate::errors::ConverterErrors;

fn main() -> Result<(), ConverterErrors> {
  let cli = CliArgs::parse();

  let CliArgs {
    input,
    input_format,
    output_format,
  } = cli;

  let mut file_reader = BufReader::new(File::open(input)?);

  if input_format == DataFormat::Csv {
    // Skip headers line
    file_reader.read_line(&mut String::new())?;
  }

  let mut parsed_records = vec![];
  while let Ok(record) =
    read_record_from_source(&mut file_reader, &input_format)
  {
    parsed_records.push(record);
  }

  let stdout = io::stdout().lock();
  let mut buf_writer = BufWriter::new(stdout);

  if input_format == DataFormat::Csv {
    // Write headers line
    writeln!(&mut buf_writer, "{}", CVS_HEADERS)?;
  }

  for record in parsed_records {
    write_record_to_source(&mut buf_writer, record, &output_format)?;
  }

  buf_writer.flush()?;

  Ok(())
}

fn read_record_from_source(
  mut buffer: &mut impl BufRead,
  input_format: &DataFormat,
) -> Result<BankRecord, ParsingError> {
  match input_format {
    DataFormat::Bin => BinRecord::from_read(buffer.deref_mut()),
    DataFormat::Csv => CsvRecord::from_read(buffer.deref_mut()),
    DataFormat::Txt => TxtRecord::from_read(buffer.deref_mut()),
  }
}

fn write_record_to_source(
  mut buffer: &mut impl Write,
  record: BankRecord,
  input_format: &DataFormat,
) -> Result<(), SerializeError> {
  match input_format {
    DataFormat::Bin => BinRecord(record).write_to(buffer.deref_mut()),
    DataFormat::Csv => CsvRecord(record).write_to(buffer.deref_mut()),
    DataFormat::Txt => TxtRecord(record).write_to(buffer.deref_mut()),
  }
}

#[cfg(test)]
mod converter_test {

  // Invalid input file format
  // Missing input file
  // From all to all formats with assertion

  //cargo run -p converter -- -i ./mocks/records_example.bin --input-format bin --output-format txt > ./temp/temp.txt
}
