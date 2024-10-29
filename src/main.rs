use clap::Parser;
use eframe::egui;
use egui_extras::{Column, TableBuilder};
use pracus::{
    scrap_justjoinit_job_offers, scrap_nofluffjobs_job_offers, scrap_pracujpl_job_offers, Source,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    csv: bool,

    #[arg(short, long, default_value_t = true)]
    gui: bool,
}

fn main() {
    let args = Args::parse();

    let column_names: [&str; 5] = ["Title", "Company", "Location", "Link", "Source"];

    let mut job_offers = scrap_justjoinit_job_offers();
    job_offers.append(&mut scrap_nofluffjobs_job_offers());
    job_offers.append(&mut scrap_pracujpl_job_offers());

    if args.csv {
        println!("Creating CSV file...")
    }

    if args.gui {
        println!("Creating GUI...");
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 480.0]),
            ..Default::default()
        };

        let _ = eframe::run_simple_native("Pracuś | Job Offers", options, move |ctx, _frame| {
            egui::CentralPanel::default().show(ctx, |ui| {
                TableBuilder::new(ui)
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .column(Column::auto().resizable(true))
                    .header(20.0, |mut header| {
                        for column_name in column_names {
                            header.col(|ui| {
                                ui.heading(column_name);
                            });
                        }
                    })
                    .body(|mut body| {
                        for job_offer in &job_offers {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(
                                        &<Option<std::string::String> as Clone>::clone(
                                            &job_offer.title,
                                        )
                                        .unwrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.label(
                                        &<Option<std::string::String> as Clone>::clone(
                                            &job_offer.company,
                                        )
                                        .unwrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.label(
                                        &<Option<std::string::String> as Clone>::clone(
                                            &job_offer.location,
                                        )
                                        .unwrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.hyperlink_to(
                                        "visit website",
                                        &<Option<std::string::String> as Clone>::clone(
                                            &job_offer.link,
                                        )
                                        .unwrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.label(
                                        &*<Option<Source> as Clone>::clone(&job_offer.source)
                                            .unwrap()
                                            .value(),
                                    );
                                });
                            });
                        }
                    });
            });
        });
    }
}
