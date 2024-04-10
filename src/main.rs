use std::env;
use std::fs::File;
use std::io::{BufReader,BufWriter};
use std::error::Error;
use rbx_types::{Ref, Variant};
use rbx_dom_weak::{InstanceBuilder, WeakDom, Instance};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn test(file_name : String) -> Result<(), Box<dyn Error>> {
    let bool_val = false;

    // Using buffered I/O is recommended with rbx_binary
    let input = BufReader::new(File::open(file_name)?);

    // rbx_binary always returns a DOM with a DataModel at the top level. To get to the instances from our file, we need to go one level deeper.
    let mut dom = rbx_binary::from_reader(input)?;
    let root = dom.root();
    let mut vec : Vec<Ref> = Vec::new();

    fn find_scripts(dom1 : &WeakDom, parent : &Instance, vec1 : &mut Vec<Ref>) {
        for &referent in parent.children() {
            let instance = dom1.get_by_ref(referent).unwrap();

            if instance.class == "Script" || instance.class == "LocalScript" {
                vec1.push(referent);

                println!("{}" , instance.name);
            }

            find_scripts(dom1, instance, vec1);
        }
    }

    print_type_of(&dom);
    find_scripts(&dom, root, &mut vec);

    if bool_val {
        return Ok(());
    }

    // iterate over script referents and create new instance
    for referent in vec.iter() {
        let instance = dom.get_by_ref(*referent).unwrap();
        
        let mut builder = InstanceBuilder::new(instance.class.clone()).with_properties(instance.properties.clone().into_iter());
        builder.add_property("Source", String::from("source"));

        let new_ref = dom.insert(*referent, builder);
        //let new_instance = dom.get_by_ref(new_ref).unwrap();

/*         for (key, value) in &new_instance.properties {
            //print_type_of(&value);

            let source = if key == "Source" {
                match value {
                    Variant => String::from("Variant1"),
                }

            } else {
                String::from("")
            };

            println!(" {}: {:?}", key, value);
        } */
    }

    //write new model or place file
    let output = BufWriter::new(File::create("output.rbxl")?);
    rbx_binary::to_writer(output, &dom, &[dom.root_ref()])?;

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