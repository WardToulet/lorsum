// # Lorsum file format
// A lorusm exists out of sections started with a `_` followed by the section name. All section
// names are lowercase and must respect the order described in this document.
//
// ## Types
// A list of all the word types in the file seperated by a newline.
//
// ## Templates
// A list of lines with `{ ident }` in place of the types. All idents must be previously
// specified in the types section.
//
// ## Lists
// A list of lists per type each type starts with `:type` followed by a list of words sperated by
// whitespace until the next `:type`. All types specified in the types section must be provided at
// least one item.

use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::io::{Read};

#[derive(Debug)]
pub struct LangDef {
    dictionary: HashMap<String, Vec<String>>,
    templates: Vec<String>,
}

enum ParsingMode {
    Initial,
    Types,
    Lists,
    Templates,
}

impl LangDef {
    pub fn from_reader<T: Read>(reader: T) -> Result<Self, String> {
        let buffered = BufReader::new(reader);
        let mut lines = buffered.lines();
        let mut mode = ParsingMode::Initial;
        let mut list_name: Option<String> = None;

        let mut lang_def = LangDef {
            dictionary: HashMap::new(),
            templates: Vec::new(),
        };

        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                continue;
            }

            match mode {
               ParsingMode::Initial => {
                    match line.as_str() {
                        "_types" => { 
                            mode = ParsingMode::Types;
                        },
                        _ => { 
                            return Err(format!("file must start with _types section"));
                        }
                    }    
               }, 
               ParsingMode::Types => {
                    match line.as_str() {
                        "_templates" => { 
                            mode = ParsingMode::Templates;
                        },
                        type_name => {
                            lang_def.dictionary.insert(String::from(type_name), Vec::new());
                        },
                    }    
               }, 
               ParsingMode::Templates => {
                    match line.as_str() {
                        "_lists" => { 
                            mode = ParsingMode::Lists;
                        },
                        template => {
                            lang_def.templates.push(String::from(template));
                        },
                    }    
               }, 
               ParsingMode::Lists => {
                    match line {
                        current_list if line.starts_with(':') => {
                            list_name = Some(String::from(&current_list[1..]));
                        },
                        word => {
                            if list_name.is_none() {
                                return Err(format!("No list started, a list must be started by :ident before items can be provided"));
                            }

                            if let Some(word_list) = lang_def.dictionary.get_mut(&list_name.clone().unwrap()) {
                                word_list.push(String::from(word));
                            } else {
                                return Err(format!("No wordlist named {}.", list_name.unwrap()));
                            }
                        },
                    }    
               }
            }
        }
        Ok(lang_def)
    }
}

#[cfg(test)]
pub mod tests {
    use std::fs::File;
    use super::*;

    #[test]
    fn parse_basic() {
        //let basic = "_types\na\n_templates\nhello { a }\n_lists\n:a\nward";
        // TODO: mock instead of opening file
        let file = File::open("./files/buzz.lorsum").unwrap();

        let lang_def = LangDef::from_reader(file);
        println!("{:#?}", lang_def);
    }
}
