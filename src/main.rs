use clap::Parser;
pub mod runner;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // make a to-do list
    #[arg(short, long)]
    makelist: String,


}

fn main() {
    let args = Args::parse();

    // read command line argument
    println!("Creating a to-do list named: {}", args.makelist);

    // start runner
    let mut list = runner::ListRunner::new(Some(args.makelist));
    // start
    // list.run()?;
    // Ok(())

}