/// file that is used to define all the common types that can be 
/// parsed and used as SegmentId
use crate::parser_gfa2::ParseFieldError;

use bstr::{BString, ByteSlice};
use lazy_static::lazy_static;
use regex::bytes::Regex;

/// Trait for the types that can be parsed and used as segment IDs;
/// will probably only be usize and BString.
pub trait SegmentId: std::fmt::Display + Sized + Default {
    const ERROR: ParseFieldError;

    // define the functions
    fn parse_opt_id(input: &[u8]) -> Option<Self>;
    fn parse_id(input: &[u8]) -> Option<Self>;
    fn parse_ref(input: &[u8]) -> Option<Self>;

    fn parse_next<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_id(next.as_ref()).ok_or(Self::ERROR)
    }

    fn parse_next_opt<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_opt_id(next.as_ref()).ok_or(Self::ERROR)
    }

    fn parse_next_ref<I>(mut input: I) -> Result<Self, ParseFieldError>
    where
        I: Iterator,
        I::Item: AsRef<[u8]>,
    {
        let next = input.next().ok_or(ParseFieldError::MissingFields)?;
        Self::parse_ref(next.as_ref()).ok_or(Self::ERROR)
    }
}

// usize cannot handle the non digit characters ([0-9])
// but the GFA2 format heavily rely on alphanumeric characters ([A-Za-z0-9])
impl SegmentId for usize {
    const ERROR: ParseFieldError = ParseFieldError::UintIdError;
    
    fn parse_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref REX: Regex =
                Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        if REX.is_match(input.as_ref()){
            lazy_static!{
            // controls if the id is a digit
            static ref RE: Regex = Regex::new(r"(?-u)[0-9]+").unwrap();
            }
            lazy_static!{
                // controls if the id is an alphanumeric character and replace it with a random
                // number 
                static ref RE2: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
            }

            if RE.is_match(input.as_ref()) {
                input.to_str().ok()?.parse::<usize>().ok()
            } else if RE2.is_match(input.as_ref()) {
                convert_alphanumeric(input)
            } else {
                panic!("Error! character {:?} cannot be parsed as usize", input)
            }
        } else {
            panic!("Error! the id tag it's not correct")
        }
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref REX: Regex =
                Regex::new(r"(?-u)[!-~]+|\*").unwrap();
        }
        if REX.is_match(input.as_ref()){
            lazy_static!{
                // controls if the id is a digit
                static ref RE: Regex = Regex::new(r"(?-u)[0-9]+").unwrap();
            }
            lazy_static!{
                static ref RE1: Regex = Regex::new(r"(?-u)\*").unwrap();
            }
            lazy_static!{
                // controls if the id is optional and then substitute it with a random
                // number 
                static ref RE2: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
            }

            if RE.is_match(input.as_ref()) {
                input.to_str().ok()?.parse::<usize>().ok()
            } else if RE1.is_match(input.as_ref()) {
                // maybe it's a bit verbose this type of conversion
                101.to_string().parse::<usize>().ok()
            } else if RE2.is_match(input.as_ref()) {
                convert_alphanumeric(input)
            } else {
                panic!("Error! character {:?} cannot be parsed as usize", input)
            }
        } else {
            panic!("Error! the optional id tag it's not correct")
        }        
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref REX: Regex =
                Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
        }
        if REX.is_match(input.as_ref()) {
            let last = input.len() - 1;

            let orient = match input[last] {
                b'+' => vec![b'0'],
                b'-' => vec![b'1'],
                _ => panic!("reference segment did not include orientation"),
            };
            let segment_id = &input[..last];
            lazy_static!{
                // controls if the id is a digit
                static ref RE: Regex = Regex::new(r"(?-u)[0-9]+").unwrap();
            }
            lazy_static!{
                // controls if the id is an alphanumeric character and replace it with a random
                // number 
                static ref RE2: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
            }

            if RE.is_match(segment_id.as_ref()) {
                format!("{}{}", segment_id.to_str().ok()?, orient.to_str().ok()?).parse::<usize>().ok()
            } else if RE2.is_match(segment_id.as_ref()) {
                let alphanumeric_id = convert_alphanumeric(segment_id);
                format!("{}{}", alphanumeric_id.unwrap(), orient.to_str().ok()?).parse::<usize>().ok()
            } else {
                panic!("Error! character {:?} cannot be parsed as usize", segment_id)
            }
        } else {
            panic!("Error! the reference tag it's not correct")
        }
    }
}

impl SegmentId for BString {
    const ERROR: ParseFieldError = ParseFieldError::Utf8Error;

    fn parse_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?-u)[!-~]+|\*").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(?-u)[!-~]+[+-]").unwrap();
        }
        RE.find(input).map(|s| BString::from(s.as_bytes()))
    }
}

fn convert_alphanumeric(input: &[u8]) -> Option<usize> {
    // could use the usize::from_str_radix(_, 36) but 
    // I need the last 0 and 1 digit to be "special" and so not 
    // involved in any kind of conversion
    let len = input.len();
    let my_vec: Vec<char> = input.to_str().unwrap().chars().collect();
    let mut res: String = "".to_string();
    let mut x = 0;
    while x < len {
        let acc = match my_vec[x] {
            'A' | 'a' => "065".parse::<usize>().ok(),
            'B' | 'b' => "066".parse::<usize>().ok(),
            'C' | 'c' => "067".parse::<usize>().ok(),
            'D' | 'd' => "068".parse::<usize>().ok(),
            'E' | 'e' => "069".parse::<usize>().ok(),
            'F' | 'f' => "072".parse::<usize>().ok(),
            'G' | 'g' => "073".parse::<usize>().ok(),
            'H' | 'h' => "074".parse::<usize>().ok(),
            'I' | 'i' => "075".parse::<usize>().ok(),
            'J' | 'j' => "076".parse::<usize>().ok(),
            'K' | 'k' => "077".parse::<usize>().ok(),
            'L' | 'l' => "078".parse::<usize>().ok(),
            'M' | 'm' => "079".parse::<usize>().ok(),
            'N' | 'n' => "082".parse::<usize>().ok(),
            'O' | 'o' => "083".parse::<usize>().ok(),
            'P' | 'p' => "084".parse::<usize>().ok(),
            'Q' | 'q' => "085".parse::<usize>().ok(),
            'R' | 'r' => "086".parse::<usize>().ok(),
            'S' | 's' => "087".parse::<usize>().ok(),
            'T' | 't' => "088".parse::<usize>().ok(),
            'U' | 'u' => "089".parse::<usize>().ok(),
            'V' | 'v' => "092".parse::<usize>().ok(),
            'W' | 'w' => "093".parse::<usize>().ok(),
            'X' | 'x' => "094".parse::<usize>().ok(),
            'Y' | 'y' => "095".parse::<usize>().ok(),
            'Z' | 'z' => "096".parse::<usize>().ok(),
            /* this part can be used to parse id that combine either letters
            and digits, but the function enters panick mode before even reach
            this level
            '0' => "000".parse::<usize>().ok(), // not so sure about this
            '1' => "001".parse::<usize>().ok(), // not so sure about this
            '2' => "002".parse::<usize>().ok(),
            '3' => "003".parse::<usize>().ok(),
            '4' => "004".parse::<usize>().ok(),
            '5' => "005".parse::<usize>().ok(),
            '6' => "006".parse::<usize>().ok(),
            '7' => "007".parse::<usize>().ok(),
            '8' => "008".parse::<usize>().ok(),
            '9' => "009".parse::<usize>().ok(),
            */
            _ => "042".parse::<usize>().ok(), // used for "special" character
        };
        res = format!("{}{}", res, acc.unwrap());
        x = x + 1; 
    }
    Some(res.parse::<usize>().ok().unwrap())
}