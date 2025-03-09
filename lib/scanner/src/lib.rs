mod stream_creator;

use stream_creator::SourceStreamGenerator;

pub struct Scanner {
    source_code: Vec<char>,
}

impl Scanner {
    pub fn new(source_code: String) -> Self {
        Scanner {
            source_code: source_code.chars().collect(),
        }
    }

    pub fn scan(&self) {
        let stream_creator =
            SourceStreamGenerator::new(self.source_code.clone(), vec![], None, None);
        let stream_creator: SourceStreamGenerator = stream_creator.generate();
        println!("{:?}", stream_creator.get_processed());
    }
}
