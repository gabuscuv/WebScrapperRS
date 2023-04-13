pub(crate) mod Event;
use std::error::Error;
use libxml::parser::{self, Parser};
use libxml::xpath::{self, Context};

// use crate::Event::Event;

const XPATH: &str = "//*[@id=\"deventos\"]/table/tr/*[not(self::th) and not(@id=\"tdinfo\" or @id=\"tdgoogle\") and text()]";

const URLPATH : &str = "https://www.devuego.es/deventos/";

const FIELDS : i16 = 4;

const PARSEOPTIONS : parser::ParserOptions = parser::ParserOptions {
    recover: true,
    no_def_dtd: false,
    no_error: true,
    no_warning: true,
    pedantic: false,
    no_blanks: true,
    no_net: false,
    no_implied: false,
    compact: true,
    ignore_enc: false,
    encoding: None,
  };
/// use libxml::bindings::_xmlParserCtxt;
/// 

fn main() -> Result<(), Box<dyn Error>> {
    let mut counter: i16 = 0;
    if !xpath::is_well_formed_xpath(XPATH)
    {
        println!("Is Not well formed");
        return Ok(());
    }
    let resp = reqwest::blocking::get(URLPATH)?.text()?;
    // let mut res: *mut _xmlParserCtxt;
    let parser = Parser::default_html();
    let document = parser.parse_string_with_options(resp,PARSEOPTIONS).unwrap();
    let xpath = Context::new(&document).unwrap();
    let result = xpath.evaluate(XPATH).unwrap();
    for node in &result.get_nodes_as_vec() {
        println!("Found: {}", node.get_content());
        // let s = Event::Event(String("Hi"), String("Hi"), todo!(), todo!(), todo!());
        
        counter = counter + 1;
        if counter == FIELDS
        {
            // Do Stuff
            counter = 0;
        }
      }
      
    Ok(())
}