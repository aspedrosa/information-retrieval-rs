use std::collections::HashSet;
use std::fs;
use std::fs::{DirEntry, File, ReadDir};
use std::rc::Rc;

use crate::parsers::files::{FileParser, TrecAsciiMedline2004FileParser};

pub trait FileParserResolver {
    fn resolve_file_parser(&self, file: DirEntry) -> Option<Box<dyn FileParser>>;
}

pub struct CorpusReader<FPR: FileParserResolver> {
    recursive: bool,
    file_parser_resolver: FPR,
    file_stack: Vec<ReadDir>,
}

impl<FPR: FileParserResolver> CorpusReader<FPR> {
    pub fn new(path: &str, recursive: bool, file_parser_resolver: FPR) -> Self {
        let file_stack = vec![
            fs::read_dir(path).unwrap(), // FIXME no unwrap
        ];

        CorpusReader {
            recursive,
            file_parser_resolver,
            file_stack,
        }
    }
}

impl<FPR: FileParserResolver> Iterator for CorpusReader<FPR> {
    type Item = Box<dyn FileParser>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.file_stack.is_empty() {
            let file = self.file_stack.first_mut().unwrap().next();
            if file.is_none() {
                self.file_stack.pop();
                continue;
            }

            let file = file.unwrap().unwrap(); // FIXME no unwrap
            if file.file_type().unwrap().is_dir() && self.recursive {
                let list_dir = fs::read_dir(file.path()).unwrap(); // FIXME no unwrap
                self.file_stack.push(list_dir);
                continue;
            }

            let file_parser = self.file_parser_resolver.resolve_file_parser(file);
            if file_parser.is_none() {
                continue;
            }

            return Some(file_parser.unwrap());
        }

        None
    }
}

pub struct TrecAsciiMedline2004FileParserResolver {
    fields_to_save: Rc<HashSet<String>>,
}

impl TrecAsciiMedline2004FileParserResolver {
    pub fn new(mut fields_to_save: HashSet<String>) -> Self {
        fields_to_save.insert("PMID".to_string());
        TrecAsciiMedline2004FileParserResolver {
            fields_to_save: Rc::new(fields_to_save),
        }
    }
}

impl FileParserResolver for TrecAsciiMedline2004FileParserResolver {
    fn resolve_file_parser(&self, file: DirEntry) -> Option<Box<dyn FileParser>> {
        if file.path().to_str().unwrap().ends_with(".gz") {
            return Some(Box::new(TrecAsciiMedline2004FileParser::new(
                File::open(file.path()).unwrap(), // FIXME might not be able to open
                Rc::clone(&self.fields_to_save),
            )));
        }

        None
    }
}
