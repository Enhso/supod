#[derive(Debug, Clone, PartialEq)]
pub struct Podcast {
    pub type_: String,
    pub name: String,
    pub priority: bool,
    pub released: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PodcastList {
    pub podcasts: Vec<Podcast>,
}

impl PodcastList {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn long_podcasts(&self) -> Vec<&Podcast> {
        self.podcasts.iter().filter(|p| p.type_ == "long").collect()
    }

    pub fn short_podcasts(&self) -> Vec<&Podcast> {
        self.podcasts
            .iter()
            .filter(|p| p.type_ == "short")
            .collect()
    }

    pub fn get_at_index(&self, index: usize) -> Option<&Podcast> {
        self.podcasts.get(index)
    }

    pub fn move_to_bottom(&mut self, index: usize) -> bool {
        if index >= self.podcasts.len() {
            return false;
        }
        let podcast = self.podcasts.remove(index);
        self.podcasts.push(podcast);
        true
    }
}
