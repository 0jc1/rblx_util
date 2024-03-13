use std::env;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn test(file_name : String) -> Result<(), Box<dyn Error>> {

    // Using buffered I/O is recommended with rbx_binary
    let input = BufReader::new(File::open(file_name)?);

    let dom = rbx_binary::from_reader(input)?;

    fn find_scripts(dom1 : &rbx_dom_weak::WeakDom, parent : &rbx_dom_weak::Instance) {
        for &referent in parent.children() {
            let instance = dom1.get_by_ref(referent).unwrap();
            println!("- {}", instance.name);

            if instance.name == "Script" {

                for (key, value) in &instance.properties {
                    println!("  {}: {:?}", key, value);
                }
            }

            println!();

            find_scripts(dom1, instance);
        }
    }

    // rbx_binary always returns a DOM with a DataModel at the top level.
    // To get to the instances from our file, we need to go one level deeper.

    print_type_of(&dom);

    // for &referent in dom.root().children() {
    //     let instance = dom.get_by_ref(referent).unwrap();
    //     println!("- {}", instance.name);
    //     print_type_of(&referent);
    //     //find_scripts(instance.children());
    // }

    let root = dom.root();

    find_scripts(&dom, root);

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