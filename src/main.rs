use clap::Parser;
use pracus::job_offers_scraper::{self};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Save offers to CSV file
    #[arg(long, default_value_t = false)]
    csv: bool,

    /// Run GUI
    #[arg(long, default_value_t = false)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    let mut job_offers = job_offers_scraper::scrap_justjoinit_job_offers();
    job_offers.append(&mut job_offers_scraper::scrap_nofluffjobs_job_offers());
    job_offers.append(&mut job_offers_scraper::scrap_pracujpl_job_offers());

    if !args.csv && !args.gui {
        panic!("Run with --csv or --gui option")
    }

    if args.csv {
        pracus::csv::save_to_csv(job_offers.clone())
    }

    if args.gui {
        pracus::gui::run(job_offers.clone())
    }
}
