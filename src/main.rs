use std::env;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
  
fn test(file_name : String) -> Result<(), Box<dyn Error>> {

    // Using buffered I/O is recommended with rbx_binary
    let input = BufReader::new(File::open(file_name)?);

    let dom = rbx_binary::from_reader(input)?;

    // rbx_binary always returns a DOM with a DataModel at the top level.
    // To get to the instances from our file, we need to go one level deeper.

    println!("Root instances in file:");
    for &referent in dom.root().children() {
        let instance = dom.get_by_ref(referent).unwrap();
        println!("- {}", instance.name);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_name>", args[0]);
        std::process::exit(1);
    }

    // Extract the file name from the command-line arguments
    let file_name = &args[1];

    test(file_name.to_string()).expect("REASON");
}