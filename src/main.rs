use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::io::{self,BufReader};

mod Import_File;

fn main() -> io::Result<()> {
    
    //let path = env::current_dir()?;
    //et path = String::from("12");
    //let num:u8 = path.parse().unwrap();
    //println!("{}", num);
    let f = File::open("2460219.TXT")?;
    let f = BufReader::new(f);
    for line in f.lines(){
        //println!("{}", line.unwrap());
        Import_File::insert_data(line.unwrap());
    }

    Ok(())
}
