use report_parser::errors::ParsingError;
use report_parser::parsers::bin::BinReportParser;
use report_parser::record::BankRecordParser;
use std::fs::File;
use std::io::BufReader;

pub const RECORD_HEADER: &[u8; 4] = b"YPBN";

fn main() -> Result<(), ParsingError> {
  let f = File::open("./report_files/records_example.bin")?;
  let mut reader = BufReader::new(f);

  let records = BinReportParser::from_read(&mut reader)?;

  println!("{:#?}", records);
  println!("{}", records.len());

  Ok(())
}
