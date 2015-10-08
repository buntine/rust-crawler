mod crawl {
    pub struct Crawler {
        pub domain: String,
        pub base: String,
        visited: Vec<String>,
    }

    impl Crawler {
        pub fn new(domain: String, base: String) -> Crawler {
            Crawler{domain: domain, base: base, visited: vec![]}
        }

        pub fn crawl(&mut self) {
            let base = self.base.clone();
            self.parse_webpage(0, &base)
        }

        fn parse_webpage(&mut self, depth: i32, path: &String) {
            println!("Parsing {}", path);
        }
    }
}

fn main() {
    let mut crawler = crawl::Crawler::new("cloudopenweek.deakin.edu.au".to_string(), "/".to_string());

    crawler.crawl();
}
