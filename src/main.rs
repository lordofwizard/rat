use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Read;
use std::path::Path;
use colored::Colorize;
fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("{}","Dude rat expects some kinda file name or something".red());
    }
    for i in 2..(args.len()+1){
        let path = Path::new(&args[i-1]);
        let display = path.display();
        let mut file_string :String = String::new();
        let mut file = match File::open(&path) {
            Err(some) => panic!("{}{}","File Opening failed. Here's Why \n".red(),some),
            Ok(file) => file,
        };
        match file.read_to_string(&mut file_string){
            Err(some) => panic!("{} here is what happend\n{}","something went wrong while converting the file to string weird shit".red(),some),
            Ok(_) => print!("{}",file_string),
        }
    }

}
