const COLUMN_NAMES: [&str; 5] = ["Title", "Company", "Location", "Link", "Source"];

pub mod csv {
    use crate::{job_offers_scraper::JobOffer, COLUMN_NAMES};

    pub fn save_to_csv(job_offers: Vec<JobOffer>) {
        let path = std::path::Path::new("data/job_offers.csv");
        let mut writer = csv::Writer::from_path(path).unwrap();

        writer.write_record(COLUMN_NAMES).unwrap();

        for job_offer in job_offers {
            let title = job_offer.title.unwrap().trim().to_string();
            let company = job_offer.company.unwrap().trim().to_string();
            let location = job_offer.location.unwrap().trim().to_string();
            let link = job_offer.link.unwrap().trim().to_string();
            let source = job_offer.source.unwrap().value().to_string();

            writer
                .write_record(&[title, company, location, link, source])
                .unwrap();
        }

        writer.flush().unwrap();
    }
}

pub mod gui {
    use eframe::egui;
    use egui_extras::{Column, TableBuilder};

    use crate::{job_offers_scraper::JobOffer, COLUMN_NAMES};

    pub fn run(job_offers: Vec<JobOffer>) {
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
                        for column_name in COLUMN_NAMES {
                            header.col(|ui| {
                                ui.heading(column_name);
                            });
                        }
                    })
                    .body(|mut body| {
                        for job_offer in &job_offers {
                            body.row(30.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(job_offer.clone().title.unwrap().trim());
                                });
                                row.col(|ui| {
                                    ui.label(job_offer.clone().company.unwrap().trim());
                                });
                                row.col(|ui| {
                                    ui.label(job_offer.clone().location.unwrap().trim());
                                });
                                row.col(|ui| {
                                    ui.hyperlink_to(
                                        "visit website",
                                        job_offer.clone().link.unwrap(),
                                    );
                                });
                                row.col(|ui| {
                                    ui.label(job_offer.clone().source.unwrap().value());
                                });
                            });
                        }
                    });
            });
        });
    }
}

pub mod job_offers_scraper {
    mod constants {
        pub const JUST_JOIN_IT: &str = "justjoin.it";
        pub const NO_FLUFF_JOBS: &str = "nofluffjobs.com";
        pub const PRACUJ_PL: &str = "pracuj.pl";

        pub const JUST_JOIN_IT_JOB_OFFER_SELECTOR: &str = "div.MuiBox-root.css-8xzgzu";
        pub const NO_FLUFF_JOBS_JOB_OFFER_SELECTOR: &str = "a.posting-list-item";
        pub const PRACUJ_PL_JOB_OFFER_SELECTOR: &str = "div.tiles_cobg3mp";

        pub const JUST_JOIN_IT_TITLE_SELECTOR: &str = "h3.css-3hs82j";
        pub const NO_FLUFF_JOBS_TITLE_SELECTOR: &str = "h3.posting-title__position";
        pub const PRACUJ_PL_TITLE_SELECTOR: &str = "h2.tiles_h1p4o5k6";

        pub const JUST_JOIN_IT_COMPANY_SELECTOR: &str = "span";
        pub const NO_FLUFF_JOBS_COMPANY_SELECTOR: &str = "h4.company-name";
        pub const PRACUJ_PL_COMPANY_SELECTOR: &str = "h3.tiles_chl8gsf.size-caption.core_t1rst47b";

        pub const JUST_JOIN_IT_LOCATION_SELECTOR: &str = "span.css-1o4wo1x";
        pub const NO_FLUFF_JOBS_LOCATION_SELECTOR: &str = "span.tw-text-right";
        pub const PRACUJ_PL_LOCATION_SELECTOR: &str = "h4.size-caption.core_t1rst47b";
    }

    use std::fmt;

    #[non_exhaustive]
    #[derive(Clone)]
    pub enum Source {
        NotProvided,
        JustJoinIt,
        NoFluffJobs,
        PracujPl,
    }

    impl Source {
        pub fn value(&self) -> &str {
            match *self {
                Source::JustJoinIt => constants::JUST_JOIN_IT,
                Source::NoFluffJobs => constants::NO_FLUFF_JOBS,
                Source::PracujPl => constants::PRACUJ_PL,
                Source::NotProvided => "-",
            }
        }
    }

    #[derive(Clone)]
    pub struct JobOffer {
        pub title: Option<String>,
        pub company: Option<String>,
        pub location: Option<String>,
        pub link: Option<String>,
        pub source: Option<Source>,
    }

    impl fmt::Display for JobOffer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Job offer with title: {}, from: {}",
                self.title.clone().unwrap(),
                self.source.clone().unwrap().value()
            )
        }
    }

    fn create_link_with_prefix(prefix: &str, href: String) -> String {
        let mut link = href;
        link.insert_str(0, &prefix);
        link
    }

    pub fn scrap_job_offers(source: Source, tech: &String) -> Vec<JobOffer> {
        let mut job_offers: Vec<JobOffer> = Vec::new();

        let url: String = match source {
            Source::NotProvided => panic!("Invalid source"),
            Source::JustJoinIt => {
                if tech.eq("java") || tech.eq("python") {
                    format!("https://justjoin.it/job-offers/all-locations/{tech}?experience-level=junior")
                } else {
                    "https://justjoin.it/job-offers/all-locations/java?experience-level=junior"
                        .to_string()
                }
            }
            Source::NoFluffJobs => {
                if tech.eq("java") || tech.eq("python") {
                    format!(
                        "https://nofluffjobs.com/pl/{tech}?criteria=seniority%3Dtrainee%2Cjunior"
                    )
                } else {
                    "https://nofluffjobs.com/pl/java?criteria=seniority%3Dtrainee%2Cjunior"
                        .to_string()
                }
            }
            Source::PracujPl => {
                if tech.eq("java") {
                    "https://it.pracuj.pl/praca?et=1%2C3%2C17&itth=38".to_string()
                } else if tech.eq("python") {
                    "https://it.pracuj.pl/praca?et=1%2C3%2C17&itth=37".to_string()
                } else {
                    "https://it.pracuj.pl/praca?et=1%2C3%2C17&itth=38".to_string()
                }
            }
        };
        let response = reqwest::blocking::get(url);

        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        let html_job_offers_selector = match source {
            Source::NotProvided => panic!("Invalid source"),
            Source::JustJoinIt => {
                scraper::Selector::parse(constants::JUST_JOIN_IT_JOB_OFFER_SELECTOR).unwrap()
            }
            Source::NoFluffJobs => {
                scraper::Selector::parse(constants::NO_FLUFF_JOBS_JOB_OFFER_SELECTOR).unwrap()
            }
            Source::PracujPl => {
                scraper::Selector::parse(constants::PRACUJ_PL_JOB_OFFER_SELECTOR).unwrap()
            }
        };
        let html_job_offers = document.select(&html_job_offers_selector);

        for html_job_offer in html_job_offers {
            let title_selector = match source {
                Source::NotProvided => panic!("Invalid source"),
                Source::JustJoinIt => constants::JUST_JOIN_IT_TITLE_SELECTOR,
                Source::NoFluffJobs => constants::NO_FLUFF_JOBS_TITLE_SELECTOR,
                Source::PracujPl => constants::PRACUJ_PL_TITLE_SELECTOR,
            };
            let title = html_job_offer
                .select(&scraper::Selector::parse(&title_selector).unwrap())
                .next()
                .map(|e| e.text().collect::<String>());

            let company_selector = match source {
                Source::NotProvided => panic!("Invalid source"),
                Source::JustJoinIt => constants::JUST_JOIN_IT_COMPANY_SELECTOR,
                Source::NoFluffJobs => constants::NO_FLUFF_JOBS_COMPANY_SELECTOR,
                Source::PracujPl => constants::PRACUJ_PL_COMPANY_SELECTOR,
            };
            let company = html_job_offer
                .select(&scraper::Selector::parse(&company_selector).unwrap())
                .next()
                .map(|e| e.text().collect::<String>());

            let location_selector = match source {
                Source::NotProvided => panic!("Invalid source"),
                Source::JustJoinIt => constants::JUST_JOIN_IT_LOCATION_SELECTOR,
                Source::NoFluffJobs => constants::NO_FLUFF_JOBS_LOCATION_SELECTOR,
                Source::PracujPl => constants::PRACUJ_PL_LOCATION_SELECTOR,
            };
            let location = html_job_offer
                .select(&scraper::Selector::parse(&location_selector).unwrap())
                .next()
                .map(|e| e.text().collect::<String>());

            let link = match source {
                Source::NotProvided => panic!("Invalid source"),
                Source::JustJoinIt => {
                    let href = html_job_offer
                        .select(&scraper::Selector::parse("a").unwrap())
                        .next()
                        .and_then(|a| a.value().attr("href"))
                        .map(str::to_owned);
                    Some(create_link_with_prefix(
                        "https://justjoin.it",
                        href.unwrap(),
                    ))
                }
                Source::NoFluffJobs => {
                    let href = html_job_offer.value().attr("href").map(str::to_owned);
                    Some(create_link_with_prefix(
                        "https://nofluffjobs.com",
                        href.unwrap(),
                    ))
                }
                Source::PracujPl => {
                    let href = html_job_offer
                        .select(&scraper::Selector::parse("a.core_n194fgoq").unwrap())
                        .next()
                        .and_then(|e| e.value().attr("href"))
                        .map(str::to_owned);
                    Some(href.unwrap())
                }
            };

            let job_offer = JobOffer {
                title,
                company,
                location,
                link,
                source: Some(source.clone()),
            };

            job_offers.push(job_offer);
        }

        return job_offers;
    }
}
