use std::error::Error;
use std::io;
use std::process;

use csv::Writer;

fn read_csv_file() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        let record = result?;
        println!("data csv {:?}", record);
      
    }
    Ok(())
}

fn write_to_csv() -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path("data/address_write.csv")?;
    wtr.write_record(&["a", "b", "c"])?;
    wtr.write_record(&["x", "y", "z"])?;
    wtr.write_record(&["y", "k", "l"])?;

    wtr.flush()?;
    Ok(())
}

fn main() {
    if let Err(err) = read_csv_file() {
        println!("error running example: {}", err);
        process::exit(1);
    }

    // if let Err(err) = write_to_csv(){
    //     println!("error running write_to_csv: {}", err);
    //     process::exit(1);
    // }
   
}