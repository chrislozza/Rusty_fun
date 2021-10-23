use reqwest::Url;
use std::collections::HashSet;
use select::document::Document;
use select::predicate::Name;

use std::panic;

struct WebCrawler {
    url: String,
}

impl WebCrawler {
    pub fn new(new_url : String) -> Self {
        WebCrawler {
            url : new_url
        }
    }

    pub fn is_valid_url(self, relative_url : Option<&str>) -> bool {
        let new_url = if relative_url != None { relative_url.unwrap() } else { self.url.as_str() };

        match Url::parse(new_url) {
            Ok(_result) => {
                let client = reqwest::blocking::Client::new();
                let request_response = client.get(new_url).send();

                if request_response.is_err() {
                    println!("Requst failed: {:?}", request_response);
                    return false
                }
                let request = request_response.unwrap();
                println!("Status for {}: {}", new_url, request.status());
                return true
            }
            Err(_e) => {
                return false
            }
        }
    }

    pub fn the_crawler(self, url : &str) -> bool {
        let client = reqwest::blocking::Client::new();
        let request_response = client.get(url).send();

        if request_response.is_err() {
            println!("Requst failed: {:?}", request_response);
            return false
        }
        let mut body = String::new();
        request_response.read_to_string(&mut body).unwrap();
        println!("Chris body is {}", body);
        return true
    }

    pub fn crawl_for_links(body : String) -> HashSet<String> {
        Document::from(body.as_str())
            .find(Name("a"))
            .filter_map(|n| n.attr("href"))
            .collect::<HashSet<String>>();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_url() {
        let crawler = WebCrawler::new("https://enhance.com".to_string());
        crawler.the_crawler("https://enhance.com")
    }

//    #[test]
//    #[should_panic]
//    fn test_invalid_url_format() {
//        let crawler = WebCrawler::new("http://{....}".to_string());
//        assert!(!crawler.is_valid_url(None));
//    }

//    #[test]
//    fn test_invalid_url() {
//        let crawler = WebCrawler::new("http://enhance,com".to_string());
//        assert!(!crawler.is_valid_url(None));
//    }
//
//    #[test]
//    fn test_has_() {
//        let crawler = WebCrawler::new("https://enhance.com".to_string());
//        assert!(crawler.is_valid_url(None));
//    }
//
//    #[test]
//    fn test_valid_url() {
//        let crawler = WebCrawler::new("https://enhance.com".to_string());
//        assert!(crawler.is_valid_url(None));
//    }
}
