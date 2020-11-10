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
    fn add_type(&mut self, type_name: &str) {
        self.dictionary.insert(String::from(type_name), Vec::new());
    }

    fn add_word_to_type(&mut self, type_name: &str, word: &str) -> Result<(), String> {
        if let Some(word_list) = self.dictionary.get_mut(type_name) {
            word_list.push(String::from(word));
            Ok(())
        } else {
            Err(format!("No wordlist named {}.", type_name))
        }
    }

    // TODO: check if template contains unknown tyeps
    fn add_template(&mut self, template: &str) -> Result<(), String> {
        self.templates.push(String::from(template));
        Ok(())
    }
        
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
                            lang_def.add_type(type_name);
                        },
                    }    
               }, 
               ParsingMode::Templates => {
                    match line.as_str() {
                        "_lists" => { 
                            mode = ParsingMode::Lists;
                        },
                        template => {
                            lang_def.add_template(template)?;
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

                            lang_def.add_word_to_type(&list_name.clone().unwrap(), &word)?;
                        },
                    }    
                }
            }
        }
        Ok(lang_def)
    }
}
