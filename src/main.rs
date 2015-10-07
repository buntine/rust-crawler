struct Crawler {
    base: &'static str,
    visited: Vec<String>,
}

impl Crawler {
    fn new(base: &'static str) -> Crawler {
        Crawler{base: base, visited: vec![]}
    }

    fn crawl(&mut self) {
        println!("Crawling {};", self.base);
    }
}

fn main() {
    let mut crawler = Crawler::new("http://cloudopenweek.deakin.edu.au/");

    crawler.crawl();
}
