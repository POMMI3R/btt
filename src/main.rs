use std::{ process::Command, ops::RangeInclusive, fs, fs::File, io::prelude::* };
use clap::{ Arg, App };
use rand::{ seq::SliceRandom, thread_rng };
use itertools::Itertools;

fn get_params() -> (String, RangeInclusive< usize >, RangeInclusive< usize >) {
    let fid = "main.c";
    let osd = 2;
    let oed = 100;
    let ssd = 1;
    let sed = 1000;

    let command = App::new("B-Tree Tester")
        .version("0.1.0")
        .arg(Arg::with_name("file")
            .long("file")
            .takes_value(true)
            .help(&format!("Source file  to test (Default: {:6})", fid)))
        .arg(Arg::with_name("order-start")
            .long("order-start")
            .takes_value(true)
            .help(&format!("Start  value of Order(Default: {:6})", osd)))
        .arg(Arg::with_name("order-end")
            .long("order-end")
            .takes_value(true)
            .help(&format!("End    value of Order(Default: {:6})", oed)))
        .arg(Arg::with_name("size-start")
            .long("size-start")
            .takes_value(true)
            .help(&format!("Start  size  of Input(Default: {:6})", ssd)))
        .arg(Arg::with_name("size-end")
            .long("size-end")
            .takes_value(true)
            .help(&format!("End    size  of Input(Default: {:6})", sed)))
        .get_matches();

    let fi = command.value_of("file"       ).unwrap_or(fid);
    let os = command.value_of("order-start").and_then(|x| x.parse::< usize >().ok()).unwrap_or(osd);
    let oe = command.value_of("order-end"  ).and_then(|x| x.parse::< usize >().ok()).unwrap_or(oed);
    let ss = command.value_of("size-start" ).and_then(|x| x.parse::< usize >().ok()).unwrap_or(ssd);
    let se = command.value_of("size-end"   ).and_then(|x| x.parse::< usize >().ok()).unwrap_or(sed);

    (fi.to_string(), os..=oe, ss..=se)
}

fn main() -> Result< (), Box< dyn std::error::Error > > {
    let (filename, order_range, size_range) = get_params();
    Command::new("gcc").arg(filename).spawn()?.wait()?;
    fs::create_dir("data").unwrap_or(());

    for size in size_range {
        print!("TEST SIZE = {} ... ", size);

        let mut data = (0..size).collect::< Vec< _ > >();
        data.shuffle(&mut thread_rng());
        let input = data.iter().map(|x| format!("i {}\n", x)).join("");

        for order in order_range.clone() {
            let ifilename = format!("data/input{}-{}.txt" , order, size);
            let ofilename = format!("data/output{}-{}.txt", order, size);

            let mut ifile = File::create(&ifilename)?;
            ifile.write(format!("{}\n{}p", order, input).as_bytes())?;
            ifile.flush()?;

            Command::new("./a.out")
                .arg(&ifilename)
                .arg(&ofilename)
                .spawn().unwrap().wait()?;

            if let Some(_) = fs::read_to_string(&ofilename)?
                .trim()
                .split(" ")
                .map(|x| x.parse::< usize >())
                .enumerate()
                .find(|(i, x)| x.as_ref().map_or(true, |y| i != y)) {
                println!("FAILED with ORDER = {}\n", order);
                return Ok(());
            }

            fs::remove_file(ofilename)?;
            fs::remove_file(ifilename)?;
        }

        println!("PASSED");
    }

    fs::remove_dir_all("data")?;
    Ok(())
}