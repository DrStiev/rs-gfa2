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
            'A' | 'a' => 1065.to_string().parse::<usize>().ok(),
            'B' | 'b' => 1066.to_string().parse::<usize>().ok(),
            'C' | 'c' => 1067.to_string().parse::<usize>().ok(),
            'D' | 'd' => 1068.to_string().parse::<usize>().ok(),
            'E' | 'e' => 1069.to_string().parse::<usize>().ok(),
            'F' | 'f' => 1072.to_string().parse::<usize>().ok(),
            'G' | 'g' => 1073.to_string().parse::<usize>().ok(),
            'H' | 'h' => 1074.to_string().parse::<usize>().ok(),
            'I' | 'i' => 1075.to_string().parse::<usize>().ok(),
            'J' | 'j' => 1076.to_string().parse::<usize>().ok(),
            'K' | 'k' => 1077.to_string().parse::<usize>().ok(),
            'L' | 'l' => 1078.to_string().parse::<usize>().ok(),
            'M' | 'm' => 1079.to_string().parse::<usize>().ok(),
            'N' | 'n' => 1082.to_string().parse::<usize>().ok(),
            'O' | 'o' => 1083.to_string().parse::<usize>().ok(),
            'P' | 'p' => 1084.to_string().parse::<usize>().ok(),
            'Q' | 'q' => 1085.to_string().parse::<usize>().ok(),
            'R' | 'r' => 1086.to_string().parse::<usize>().ok(),
            'S' | 's' => 1087.to_string().parse::<usize>().ok(),
            'T' | 't' => 1088.to_string().parse::<usize>().ok(),
            'U' | 'u' => 1089.to_string().parse::<usize>().ok(),
            'V' | 'v' => 1092.to_string().parse::<usize>().ok(),
            'W' | 'w' => 1093.to_string().parse::<usize>().ok(),
            'X' | 'x' => 1094.to_string().parse::<usize>().ok(),
            'Y' | 'y' => 1095.to_string().parse::<usize>().ok(),
            'Z' | 'z' => 1096.to_string().parse::<usize>().ok(),
            /* this part can be used to parse id that combine either letters
            and digits, but the function enters panick mode before even reach
            this level
            '0' => 0.to_string().parse::<usize>().ok(),
            '1' => 1.to_string().parse::<usize>().ok(),
            '2' => 2.to_string().parse::<usize>().ok(),
            '3' => 3.to_string().parse::<usize>().ok(),
            '4' => 4.to_string().parse::<usize>().ok(),
            '5' => 5.to_string().parse::<usize>().ok(),
            '6' => 6.to_string().parse::<usize>().ok(),
            '7' => 7.to_string().parse::<usize>().ok(),
            '8' => 8.to_string().parse::<usize>().ok(),
            '9' => 9.to_string().parse::<usize>().ok(),
            */
            _ => 4242.to_string().parse::<usize>().ok(), // used for special character
        };
        res = format!("{}{}", res, acc.unwrap());
        x = x + 1; 
    }
    Some(res.parse::<usize>().ok().unwrap())
}