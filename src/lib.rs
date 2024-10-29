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
                Source::JustJoinIt => "justjoin.it",
                Source::NoFluffJobs => "nofluffjobs.com",
                Source::PracujPl => "pracuj.pl",
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
            write!(f, "Job offer with title: {}", self.title.clone().unwrap())
        }
    }

    pub fn scrap_justjoinit_job_offers() -> Vec<JobOffer> {
        let mut job_offers: Vec<JobOffer> = Vec::new();

        let response = reqwest::blocking::get(
            "https://justjoin.it/job-offers/all-locations/java?experience-level=junior",
        );
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        let html_job_offers_selector =
            scraper::Selector::parse("div.MuiBox-root.css-8xzgzu").unwrap();
        let html_job_offers = document.select(&html_job_offers_selector);

        for html_job_offer in html_job_offers {
            let title = html_job_offer
                .select(&scraper::Selector::parse("h3.css-3hs82j").unwrap())
                .next()
                .map(|h3| h3.text().collect::<String>());
            let company = html_job_offer
                .select(&scraper::Selector::parse("span").unwrap())
                .next()
                .map(|span| span.text().collect::<String>());
            let location = html_job_offer
                .select(&scraper::Selector::parse("span.css-1o4wo1x").unwrap())
                .next()
                .map(|span| span.text().collect::<String>());
            let href = html_job_offer
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned);
            let mut full_href = href.clone().unwrap().to_owned();
            full_href.insert_str(0, "https://justjoin.it");
            let link = Some(full_href);

            let job_offer = JobOffer {
                title,
                company,
                location,
                link,
                source: Some(Source::JustJoinIt),
            };

            job_offers.push(job_offer);
        }

        return job_offers;
    }

    pub fn scrap_nofluffjobs_job_offers() -> Vec<JobOffer> {
        let mut job_offers: Vec<JobOffer> = Vec::new();

        let response = reqwest::blocking::get(
            "https://nofluffjobs.com/pl/Java?criteria=seniority%3Dtrainee%2Cjunior",
        );
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        let html_job_offers_selector = scraper::Selector::parse("a.posting-list-item").unwrap();
        let html_job_offers = document.select(&html_job_offers_selector);

        for html_job_offer in html_job_offers {
            let title = html_job_offer
                .select(&scraper::Selector::parse("h3.posting-title__position").unwrap())
                .next()
                .map(|h3| h3.text().collect::<String>());
            let company = html_job_offer
                .select(&scraper::Selector::parse("h4.company-name").unwrap())
                .next()
                .map(|h4| h4.text().collect::<String>());
            let location = html_job_offer
                .select(&scraper::Selector::parse("span.tw-text-right").unwrap())
                .next()
                .map(|span| span.text().collect::<String>());
            let href = html_job_offer.value().attr("href");
            let mut full_href = href.clone().unwrap().to_owned();
            full_href.insert_str(0, "https://nofluffjobs.com");
            let link = Some(full_href);

            let job_offer = JobOffer {
                title,
                company,
                location,
                link,
                source: Some(Source::NoFluffJobs),
            };

            job_offers.push(job_offer);
        }

        return job_offers;
    }

    pub fn scrap_pracujpl_job_offers() -> Vec<JobOffer> {
        let mut job_offers: Vec<JobOffer> = Vec::new();

        let response = reqwest::blocking::get("https://it.pracuj.pl/praca?et=1%2C3%2C17&itth=38");
        let html_content = response.unwrap().text().unwrap();
        let document = scraper::Html::parse_document(&html_content);

        let html_job_offers_selector = scraper::Selector::parse("div.tiles_cobg3mp").unwrap();
        let html_job_offers = document.select(&html_job_offers_selector);

        for html_job_offer in html_job_offers {
            let title = html_job_offer
                .select(&scraper::Selector::parse("h2.tiles_h1p4o5k6").unwrap())
                .next()
                .map(|h3| h3.text().collect::<String>());
            let company = html_job_offer
                .select(
                    &scraper::Selector::parse("h3.tiles_chl8gsf.size-caption.core_t1rst47b")
                        .unwrap(),
                )
                .next()
                .map(|h3| h3.text().collect::<String>());
            let location = html_job_offer
                .select(&scraper::Selector::parse("h4.size-caption.core_t1rst47b").unwrap())
                .next()
                .map(|h4| h4.text().collect::<String>());
            let full_href = html_job_offer
                .select(&scraper::Selector::parse("a.core_n194fgoq").unwrap())
                .next()
                .and_then(|a| a.value().attr("href"))
                .map(str::to_owned);
            let link = Some(full_href.unwrap());

            let job_offer = JobOffer {
                title,
                company,
                location,
                link,
                source: Some(Source::PracujPl),
            };

            job_offers.push(job_offer);
        }

        return job_offers;
    }
}
