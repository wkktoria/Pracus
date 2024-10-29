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

    #[arg(long, default_value_t = String::from("java"))]
    tech: String,
}

fn main() {
    let args = Args::parse();

    let tech = args.tech;
    let available_techs = ["java".to_string(), "python".to_string()];

    if !available_techs.contains(&tech) {
        panic!("Unavailable tech");
    }

    let mut job_offers = job_offers_scraper::scrap_justjoinit_job_offers(&tech);
    job_offers.append(&mut job_offers_scraper::scrap_nofluffjobs_job_offers(&tech));
    job_offers.append(&mut job_offers_scraper::scrap_pracujpl_job_offers(&tech));

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
