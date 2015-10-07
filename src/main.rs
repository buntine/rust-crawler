mod crawl {
    pub struct Crawler {
        pub base: String,
        visited: Vec<String>,
    }

    impl Crawler {
        pub fn new(base: String) -> Crawler {
            Crawler{base: base, visited: vec![]}
        }

        pub fn crawl(&mut self) {
            let base = self.base.clone();
            self.parseWebpage(0, &base)
        }

        fn parseWebpage(&mut self, depth: i32, url: &String) {
            println!("Parsing {}", url);
        }
    }
}

fn main() {
    let mut crawler = crawl::Crawler::new("http://cloudopenweek.deakin.edu.au/".to_string());

    crawler.crawl();
}
