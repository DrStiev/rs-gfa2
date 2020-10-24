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

// usize cannot handle the non alphanumeric characters ([A-Za-z0-9])
// but the GFA2 format heavily rely on these special characters (specially '*')
/// using ```usize``` performs an implicit conversion from letters [A-Za-z]
/// to digits [0-9] using the function:
/// usize::from_str_radix(string_to_covnvert, base_36)
impl SegmentId for usize {
    const ERROR: ParseFieldError = ParseFieldError::UintIdError;
    // performs a conversion could leads to an errror because the ids are 
    // only converted in the fields id and not in all the occurencies of 
    // the string
    // there's only 2 options, convert all the occurencies or convert back the id

    fn parse_id(input: &[u8]) -> Option<Self> {
        usize::from_str_radix(input.to_str().ok()?, 36).ok()
        //input.to_str().ok()?.parse::<usize>().ok()
    }

    fn parse_opt_id(input: &[u8]) -> Option<Self> {
        usize::from_str_radix(input.to_str().ok()?, 36).ok()
        //input.to_str().ok()?.parse::<usize>().ok()
    }

    fn parse_ref(input: &[u8]) -> Option<Self> {
        usize::from_str_radix(input.to_str().ok()?, 36).ok()
        //input.to_str().ok()?.parse::<usize>().ok()
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