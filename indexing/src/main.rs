use lib::parsers::corpus::{CorpusReader, TrecAsciiMedline2004FileParserResolver};
use std::collections::HashSet;

fn main() {
    let mut fields_to_save = HashSet::new();
    fields_to_save.insert("TI".parse().unwrap());
    let file_parser_resolver = TrecAsciiMedline2004FileParserResolver::new(fields_to_save);

    let cr = CorpusReader::new("corpus", false, file_parser_resolver);

    for file_parser in cr {
        for doc in file_parser {
            println!("{}", doc);
        }
    }
}
