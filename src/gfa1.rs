pub use crate::gfa2::{orientation::*, traits::*};
use crate::tag::*;

use bstr::{BStr, BString, ByteSlice};
use serde::{Deserialize, Serialize};
use std::fmt;

/// This module defines the various GFA line types, the GFA object,
/// and some utility functions and types.

/// Simple representation of a parsed GFA file, using a Vec<T> to
/// store each separate GFA line type.\
/// Returns a GFA object
///
/// # Examples
/// ```ignore
///
/// let gfa: GFA<BString, OptionalFields> = GFA {
///     headers: vec![
///         Header::new(Some("VN:Z:1.0".into())),
///     ],
///     segments: vec![
///         Segment::new(b"A", b"AAAAAAACGT"),
///     ],
///     links: vec![
///         Link::new(b"15", Orientation::Backward, b"10", Orientation::Forward, b"4M"),
///     ],
///     containments: vec![
///         Containmnet::new(b"1", Orientation::Backward, b"2", Orientation::Forward, b"110", b"100M"),
///     ],
///     paths: vec![
///         Path::new(b"14", b"11+,12-,13+", vec![b"4M", b"5M"]),
///     ],
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct GFA<N, T: OptFields> {
    pub headers: Vec<Header<T>>,
    pub segments: Vec<Segment<N, T>>,
    pub links: Vec<Link<N, T>>,
    pub containments: Vec<Containment<N, T>>,
    pub paths: Vec<Path<N, T>>,
}

impl<N: SegmentId, T: OptFields> fmt::Display for GFA<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.headers
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.segments
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.links
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.containments
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
            self.paths
                .iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\n"),
        )
    }
}

/// Enum containing the different kinds of GFA lines.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Line<N, T: OptFields> {
    Header(Header<T>),
    Segment(Segment<N, T>),
    Link(Link<N, T>),
    Containment(Containment<N, T>),
    Path(Path<N, T>),
}

macro_rules! some_line_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<N, T: OptFields> Line<N, T> {
            pub fn $name(self) -> Option<$tgt> {
                if let $variant(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

some_line_fn!(some_header, Header<T>, Line::Header);
some_line_fn!(some_segment, Segment<N, T>, Line::Segment);
some_line_fn!(some_link, Link<N, T>, Line::Link);
some_line_fn!(some_containment, Containment<N, T>, Line::Containment);
some_line_fn!(some_path, Path<N, T>, Line::Path);

macro_rules! some_line_ref_fn {
    ($name:ident, $tgt:ty, $variant:path) => {
        impl<'a, N, T: OptFields> LineRef<'a, N, T> {
            pub fn $name(self) -> Option<&'a $tgt> {
                if let $variant(x) = self {
                    Some(x)
                } else {
                    None
                }
            }
        }
    };
}

some_line_ref_fn!(some_header, Header<T>, LineRef::Header);
some_line_ref_fn!(some_segment, Segment<N, T>, LineRef::Segment);
some_line_ref_fn!(some_link, Link<N, T>, LineRef::Link);
some_line_ref_fn!(some_containment, Containment<N, T>, LineRef::Containment);
some_line_ref_fn!(some_path, Path<N, T>, LineRef::Path);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum LineRef<'a, N, T: OptFields> {
    Header(&'a Header<T>),
    Segment(&'a Segment<N, T>),
    Link(&'a Link<N, T>),
    Containment(&'a Containment<N, T>),
    Path(&'a Path<N, T>),
}

impl<N, T: OptFields> GFA<N, T> {
    /// Insert a GFA line (wrapped in the Line enum) into an existing
    /// GFA. Simply pushes it into the corresponding Vec in the GFA,
    /// or replaces the header, so there's no deduplication or sorting
    /// taking place.
    pub fn insert_line(&mut self, line: Line<N, T>) {
        use Line::*;
        match line {
            Header(h) => self.headers.push(h),
            Segment(s) => self.segments.push(s),
            Link(s) => self.links.push(s),
            Containment(s) => self.containments.push(s),
            Path(s) => self.paths.push(s),
        }
    }

    /// Consume a GFA object to produce an iterator over all the lines
    /// contained within. The iterator first produces all segments, then
    /// links, then containments, and finally paths.
    pub fn lines_into_iter(self) -> impl Iterator<Item = Line<N, T>> {
        use Line::*;
        let heads = self.headers.into_iter().map(Header);
        let segs = self.segments.into_iter().map(Segment);
        let links = self.links.into_iter().map(Link);
        let conts = self.containments.into_iter().map(Containment);
        let paths = self.paths.into_iter().map(Path);

        heads.chain(segs).chain(links).chain(conts).chain(paths)
    }

    /// Return an iterator over references to the lines in the GFA
    pub fn lines_iter(&'_ self) -> impl Iterator<Item = LineRef<'_, N, T>> {
        use LineRef::*;
        let heads = self.headers.iter().map(Header);
        let segs = self.segments.iter().map(Segment);
        let links = self.links.iter().map(Link);
        let conts = self.containments.iter().map(Containment);
        let paths = self.paths.iter().map(Path);

        heads.chain(segs).chain(links).chain(conts).chain(paths)
    }
}

impl<N: SegmentId, T: OptFields> GFA<N, T> {
    pub fn new() -> Self {
        Default::default()
    }
}

/// The header line of a GFA graph
/// /// Returns an Header line
///
/// # Examples
/// ```ignore
/// // inizialize a simple header
/// let header = "VN:Z:1.0";
/// let header_ = Header {
///     version: Some("VN:Z:1.0".into()),
///     optional: (),
/// };
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Header<T: OptFields> {
    pub version: Option<BString>,
    pub optional: T,
}

impl<T: OptFields> Default for Header<T> {
    fn default() -> Self {
        Header {
            version: Some("1.0".into()),
            optional: Default::default(),
        }
    }
}

impl<T: OptFields> Header<T> {
    pub fn new(version: Option<BString>) -> Self {
        Header {
            version,
            optional: Default::default(),
        }
    }
}

impl<T: OptFields> fmt::Display for Header<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opt = vec![];
        for tag in self.optional.fields() {
            opt.push(tag);
        }
        if let Some(v) = &self.version {
            write!(
                f,
                "H\t{}\t{}",
                v,
                opt.iter()
                    .fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
            )
        } else {
            write!(f, "H")
        }
    }
}

/// A segment in a GFA graph. Generic over the name type, but
/// currently the parser is only defined for N = BString
/// Returns a Segment line
///
/// # Examples
/// ```ignore
/// // inizialize a simple segment
/// let segment = "1\tAAAAAAACGT";
/// let segment_: Segment<BString, _> = Segment {
///     name: "1".into(),
///     sequence: "AAAAAAACGT".into(),
///     optional:(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Segment<N, T: OptFields> {
    pub name: N,
    pub sequence: BString,
    pub optional: T,
}

impl<T: OptFields> Segment<BString, T> {
    pub fn new(name: &[u8], sequence: &[u8]) -> Self {
        Segment {
            name: BString::from(name),
            sequence: BString::from(sequence),
            optional: Default::default(),
        }
    }
}

impl<N: SegmentId, T: OptFields> fmt::Display for Segment<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opt = vec![];
        for tag in self.optional.fields() {
            opt.push(tag);
        }
        write!(
            f,
            "S\t{}\t{}\t{}",
            self.name,
            self.sequence.as_bstr(),
            opt.iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Link line
///
/// # Examples
/// ```ignore
/// // inizialize a simple link
/// let link = "15\t-\t10\t+\t20M";
/// let link_: Link<BString, _> = Link {
///     from_segment: "15".into(),
///     from_orient: Orientation::Backward,
///     to_segment: "10".into(),
///     to_orient: Orientation::Forward,
///     overlap: 20M
///     optional:(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Link<N, T: OptFields> {
    pub from_segment: N,
    pub from_orient: Orientation,
    pub to_segment: N,
    pub to_orient: Orientation,
    pub overlap: BString,
    pub optional: T,
}

impl<T: OptFields> Link<BString, T> {
    pub fn new(
        from_segment: &[u8],
        from_orient: Orientation,
        to_segment: &[u8],
        to_orient: Orientation,
        overlap: &[u8],
    ) -> Link<BString, T> {
        Link {
            from_segment: from_segment.into(),
            from_orient,
            to_segment: to_segment.into(),
            to_orient,
            overlap: overlap.into(),
            optional: Default::default(),
        }
    }
}

impl<N: SegmentId, T: OptFields> fmt::Display for Link<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opt = vec![];
        for tag in self.optional.fields() {
            opt.push(tag);
        }
        write!(
            f,
            "L\t{}\t{}\t{}\t{}\t{}\t{}",
            self.from_segment,
            self.from_orient,
            self.to_segment,
            self.to_orient,
            self.overlap,
            opt.iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// Returns a Containment line
///
/// # Examples
/// ```ignore
/// // inizialize a simple link
/// let containment = "15\t-\t10\t+\t4\t20M";
/// let containment_: Containment<BString, _> = Containment {
///     container_name: "15".into(),
///     container_orient: Orientation::Backward,
///     contained_name: "10".into(),
///     contained_orient: Orientation::Forward,
///     pos: 4
///     overlap: 20M
///     optional:(),
/// };
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Containment<N, T: OptFields> {
    pub container_name: N,
    pub container_orient: Orientation,
    pub contained_name: N,
    pub contained_orient: Orientation,
    pub pos: usize,
    pub overlap: BString,
    pub optional: T,
}

impl<N: SegmentId, T: OptFields> fmt::Display for Containment<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opt = vec![];
        for tag in self.optional.fields() {
            opt.push(tag);
        }
        write!(
            f,
            "C\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.container_name,
            self.container_orient,
            self.contained_name,
            self.contained_orient,
            self.pos,
            self.overlap,
            opt.iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

/// The step list that the path actually consists of is an unparsed
/// BString to keep memory down; use path.iter() to get an iterator
/// over the parsed path segments and orientations.\
/// Returns a Path line
/// # Examples
/// ```ignore
/// // inizialize a simple o-group
/// let path = "14\t11+,12-,13+\t4M,5M";
/// let path_: Path<BString, _> = Path::new(
///     "14".into(),
///     "11+,12-,13+".into(),
///     "4M,5M".into(),
///     (),
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Hash)]
pub struct Path<N, T: OptFields> {
    pub path_name: BString,
    pub segment_names: BString,
    pub overlaps: BString,
    pub optional: T,
    _segment_names: std::marker::PhantomData<N>,
}

impl<N: SegmentId, T: OptFields> Path<N, T> {
    pub fn new(path_name: BString, segment_names: BString, overlaps: BString, optional: T) -> Self {
        Path {
            path_name,
            segment_names,
            overlaps,
            optional,
            _segment_names: std::marker::PhantomData,
        }
    }
}

impl<N: SegmentId, T: OptFields> Path<N, T> {
    /// Parses (and copies!) a segment ID in the path segment list
    fn parse_segment_id(input: &[u8]) -> Option<(N, Orientation)> {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Path segment did not include orientation"),
        };
        let seg = &input[..last];
        let id = N::parse_id(seg)?;
        Some((id, orient))
    }
}

impl<T: OptFields> Path<BString, T> {
    /// Produces an iterator over the segments of the given path,
    /// parsing the orientation and producing a slice to each segment
    /// name
    pub fn iter(&self) -> impl Iterator<Item = (&'_ BStr, Orientation)> {
        self.segment_names.split_str(b",").map(Self::segment_id_ref)
    }

    fn segment_id_ref(input: &[u8]) -> (&'_ BStr, Orientation) {
        use Orientation::*;
        let last = input.len() - 1;
        let orient = match input[last] {
            b'+' => Forward,
            b'-' => Backward,
            _ => panic!("Path segment did not include orientation"),
        };
        let seg = &input[..last];
        (seg.as_ref(), orient)
    }
}

impl<T: OptFields> Path<usize, T> {
    /// Produces an iterator over the usize segments of the given
    /// path.
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (usize, Orientation)> + 'a {
        self.segment_names
            .split_str(b",")
            .filter_map(Self::parse_segment_id)
    }
}

impl<N: SegmentId, T: OptFields> fmt::Display for Path<N, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut opt = vec![];
        for tag in self.optional.fields() {
            opt.push(tag);
        }
        write!(
            f,
            "P\t{}\t{}\t{}\t{}",
            self.path_name,
            self.segment_names.as_bstr().to_string(),
            self.overlaps.as_bstr().to_string(),
            opt.iter()
                .fold(String::new(), |acc, str| acc + &str.to_string() + "\t"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_iter() {
        use Orientation::*;

        let path: Path<BString, _> =
            Path::new("14".into(), "11+,12-,13+".into(), "4M,5M".into(), ());

        let mut path_iter = path.iter();
        assert_eq!(Some(("11".into(), Forward)), path_iter.next());
        assert_eq!(Some(("12".into(), Backward)), path_iter.next());
        assert_eq!(Some(("13".into(), Forward)), path_iter.next());
        assert_eq!(None, path_iter.next());
    }

    #[test]
    fn gfa_line_ref_iter() {
        let parser: crate::parser_gfa1::GFAParser<usize, ()> = crate::parser_gfa1::GFAParser::new();
        let gfa = parser.parse_file(&"./tests/gfa1_files/lil.gfa").unwrap();
        let gfa_lineref = gfa.lines_iter();

        for line in gfa_lineref {
            let seg = line.some_segment();
            println!("{:?}", seg);
        }
    }
}
