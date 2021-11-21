use flate2::read::GzDecoder;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::rc::Rc;

use crate::parsers::documents::Document;

pub trait FileParser: Iterator<Item = Document> {}

pub struct TrecAsciiMedline2004FileParser {
    reader: Lines<BufReader<GzDecoder<File>>>,
    files_to_save: Rc<HashSet<String>>,
}

impl TrecAsciiMedline2004FileParser {
    fn create_document(&self, document_content: Vec<String>) -> Document {
        let mut doc = Document::new();

        let mut doc_content_iter = document_content.iter();

        let mut line = doc_content_iter.next().unwrap();
        let mut label: String = String::from(line.chars().take(4).collect::<String>().trim());
        let mut content: String = line.chars().skip(6).collect();
        loop {
            if line.starts_with(' ') {
                content.push_str(line);
            } else if self.files_to_save.contains(&*label) {
                if label == "PMID" {
                    doc.set_id(content.parse::<u64>().unwrap())
                } else {
                    doc.add_string_to_tokenize(content);
                }
            }

            let new_line = doc_content_iter.next();
            if new_line.is_none() {
                break;
            } else {
                line = new_line.unwrap();
            }

            label = String::from(line.chars().take(4).collect::<String>().trim());
            content = line.chars().skip(6).collect();
        }

        doc
    }

    pub(crate) fn new(reader: File, files_to_save: Rc<HashSet<String>>) -> Self {
        TrecAsciiMedline2004FileParser {
            reader: BufReader::new(GzDecoder::new(reader)).lines(),
            files_to_save,
        }
    }
}

impl Iterator for TrecAsciiMedline2004FileParser {
    type Item = Document;

    fn next(&mut self) -> Option<Self::Item> {
        let mut document_content: Vec<String> = Vec::new();

        while let Some(line) = self.reader.next() {
            let line = line.unwrap();
            if line.is_empty() {
                return Some(self.create_document(document_content));
            }

            document_content.push(line);
        }

        None
    }
}

impl FileParser for TrecAsciiMedline2004FileParser {}
