use structopt::StructOpt;
use std::fs::File;
use std::io;

mod lib;

#[derive(StructOpt, Debug)]
#[structopt(name = "lorsum", about = "lorum ipsum generator")]
struct Opt {
    #[structopt(short, long, default_value = "1")]
    paragraphs: u32,

    #[structopt(short, long)]
    file: String,
}

fn main() -> io::Result<()> {
    // Get flags
    let opt: Opt = Opt::from_args();

    // Open file
    let file = File::open(opt.file.clone())?;
    let lang_def = lib::LangDef::from_reader(file);
    println!("{:#?}", lang_def);
    

    println!("{:#?}", opt);

    Ok(())
}
