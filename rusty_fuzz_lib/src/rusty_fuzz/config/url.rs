use reqwest::Url;

pub struct UrlIterator {
    urls: Vec<Url>,
    base_url: Url,
    current_url_idx: Option<usize>,
}

impl UrlIterator {
    pub fn new() -> Self {
        Self {
            inner: Vec::new(),
            current_idx: None,
        }
    }

    pub fn add_url(mut self, url: Url) -> Self {
        // we push only already parsed urls in here
        self.inner.push(url)
    }

    pub fn add_urls(mut self, urls: Vec<Url>) -> Self {
        // in case we have multiple urls that we want to push
        // to our iterator
        for url in urls {
            self = self.add_url(url);
        }
        self
    }

    pub fn reset(&mut self) {
        self.current_url_idx = None;
    }
}

impl Iterator for UrlIterator {
    type Item = Url;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &self.current_url_idx {
                Some(idx) if *idx == self.inner.len() => break None,
                _ => {}
            }
        }
    }
}
