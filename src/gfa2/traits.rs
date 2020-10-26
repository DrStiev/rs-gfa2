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
        lazy_static!{
            // controls if the id is a digit
            static ref RE: Regex = Regex::new(r"(?-u)[0-9]+").unwrap();
        }
        lazy_static!{
            // controls if the id is an alphanumeric character and replace it with a random
            // number (it should be unique)
            // example: S\tA\t10\tAAAAAAACGT -> S\t404\t10\tAAAAAAACGT
            //          S\tX\t10\tACGTCCACGT -> S\t404\t10\tACGTCCACGT
            // the 2 id should be unique and not the same
            static ref RE2: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        if RE.is_match(input.as_ref()) {
            input.to_str().ok()?.parse::<usize>().ok()
        } else if RE2.is_match(input.as_ref()) {
            // maybe it's a bit verbose this type of conversion
            404.to_string().parse::<usize>().ok()
        } else {
            panic!("Error! character {:?} cannot be parsed as usize", input)
        }
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        lazy_static!{
            // controls if the id is a digit
            static ref RE: Regex = Regex::new(r"(?-u)[0-9]+").unwrap();
        }
        lazy_static!{
            // controls if the id is optional and then substitute it with a random
            // number (it should be unique and associated with the relative segment id)
            static ref RE1: Regex = Regex::new(r"(?-u)\*").unwrap();
        }
        lazy_static!{
            static ref RE2: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        if RE.is_match(input.as_ref()) {
            input.to_str().ok()?.parse::<usize>().ok()
        } else if RE1.is_match(input.as_ref()) {
            // maybe it's a bit verbose this type of conversion
            101.to_string().parse::<usize>().ok()
        } else if RE2.is_match(input.as_ref()) {
            // maybe it's a bit verbose this type of conversion
            404.to_string().parse::<usize>().ok()
        } else {
            panic!("Error! character {:?} cannot be parsed as usize", input)
        }
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        lazy_static!{
            // controls if the id is a digit
            static ref RE: Regex = Regex::new(r"(?-u)[0-9]+").unwrap();
        }
        lazy_static!{
            // controls if the id is an alphanumeric character and replace it with a random
            // number (it should be unique and associated with the relative segment id)
            static ref RE2: Regex = Regex::new(r"(?-u)[!-~]+").unwrap();
        }
        if RE.is_match(input.as_ref()) {
            input.to_str().ok()?.parse::<usize>().ok()
        } else if RE2.is_match(input.as_ref()) {
            // maybe it's a bit verbose this type of conversion
            404.to_string().parse::<usize>().ok()
        }else {
            panic!("Error! character {:?} cannot be parsed as usize", input)
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