use std::env;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use rbx_types::Ref;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn test(file_name : String) -> Result<(), Box<dyn Error>> {

    // Using buffered I/O is recommended with rbx_binary
    let input = BufReader::new(File::open(file_name)?);

    // rbx_binary always returns a DOM with a DataModel at the top level.
    // To get to the instances from our file, we need to go one level deeper.
    let mut dom = rbx_binary::from_reader(input)?;

    let mut vec : Vec<Ref> = Vec::new();

    fn find_scripts(dom1 : &rbx_dom_weak::WeakDom, parent : &rbx_dom_weak::Instance, vec1 : &mut Vec<Ref>) {
        for &referent in parent.children() {
            let instance = dom1.get_by_ref(referent).unwrap();

            if instance.class == "Script" || instance.class == "LocalScript" {

                vec1.push(referent);

                for (key, value) in &instance.properties {
                    let source;
                    if key == "Source" {
                        source = value;
                    }

                    println!("  {}: {:?}", key, value);
                }
            }

            find_scripts(dom1, instance, vec1);
        }
    }

    print_type_of(&dom);

    let root = dom.root();

    find_scripts(&dom, root, &mut vec);

    for referent in vec.iter_mut() {
        let instance = dom.get_by_ref_mut(*referent).unwrap();
        println!("{}", instance.name);


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