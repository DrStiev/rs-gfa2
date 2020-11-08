use gfa2::{
    gfa2::GFA2,
    gfa1::GFA,
    parser_gfa2::GFA2Parser,
    parser_gfa1::{GFAParser, GFAParserLineIter},
    tag::OptionalFields,
};
use bstr::BString;
 
#[test] 
fn can_parse_gfa2_file_with_tag() {
    let parser: GFA2Parser<BString, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<BString, OptionalFields> =
        parser.parse_file(&"./tests/gfa2_files/sample2.gfa").unwrap();
    
    let head = gfa2.headers.len();
    let seg = gfa2.segments.len();
    let frag = gfa2.fragments.len();
    let edge = gfa2.edges.len();
    let gap = gfa2.gaps.len();
    let ogroup = gfa2.groups_o.len();
    let ugroup = gfa2.groups_u.len();

    assert_eq!(head, 4);
    assert_eq!(seg, 9);
    assert_eq!(frag, 2);
    assert_eq!(edge, 6);
    assert_eq!(gap, 2);
    assert_eq!(ogroup, 2);
    assert_eq!(ugroup, 2);

    println!("{}", gfa2);
}

#[test]
fn can_parse_gfa2_file_with_no_tag() {
    let parser: GFA2Parser<BString, ()> = GFA2Parser::new();
    let gfa2: GFA2<BString, ()> =
        parser.parse_file(&"./tests/gfa2_files/data.gfa").unwrap();

    let head = gfa2.headers.len();
    let seg = gfa2.segments.len();
    let frag = gfa2.fragments.len();
    let edge = gfa2.edges.len();
    let gap = gfa2.gaps.len();
    let ogroup = gfa2.groups_o.len();
    let ugroup = gfa2.groups_u.len();

    assert_eq!(head, 1);
    assert_eq!(seg, 9);
    assert_eq!(frag, 14);
    assert_eq!(edge, 12);
    assert_eq!(gap, 0);
    assert_eq!(ogroup, 2);
    assert_eq!(ugroup, 0);


    println!("{}", gfa2);
}

#[test]
fn can_parse_gfa2_file_usize() {
    let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
    let gfa2: GFA2<usize, ()> =
        parser.parse_file(&"./tests/gfa2_files/sample2.gfa").unwrap();

    println!("{}", gfa2);
}

#[test]
fn can_parse_gfa2_file_asterix_usize() {
    let parser: GFA2Parser<usize, ()> = GFA2Parser::new();
    let gfa2: GFA2<usize, ()> =
        parser.parse_file(&"./tests/gfa2_files/data.gfa").unwrap();

    println!("{}", gfa2);
}

#[test]
fn can_parse_gfa2_graph() {
    let parser: GFA2Parser<BString, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<BString, OptionalFields> =
        parser.parse_file(&"./tests/gfa2_files/graph.gfa").unwrap();

    let head = gfa2.headers.len();
    let seg = gfa2.segments.len(); // 61
    let frag = gfa2.fragments.len(); //11
    let edge = gfa2.edges.len(); // 84
    let gap = gfa2.gaps.len(); //2
    let ogroup = gfa2.groups_o.len(); // 2
    let ugroup = gfa2.groups_u.len(); // 2

    println!(
    "Header lines: {}\n
    Segment lines: {}\n 
    Fragment lines: {}\n
    Edge lines: {}\n
    Gap lines: {}\n
    GroupO lines: {}\n
    GroupU lines: {}\n",
        head, seg, frag, edge, gap, ogroup, ugroup);
}

#[test]
fn can_parse_gfa2_with_multiple_tag() {
    let parser: GFA2Parser<BString, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<BString, OptionalFields> =
        parser.parse_file(&"./tests/gfa2_files/sample.gfa").unwrap();

    let head = gfa2.headers.len();
    let seg = gfa2.segments.len();
    let frag = gfa2.fragments.len();
    let edge = gfa2.edges.len();
    let gap = gfa2.gaps.len();
    let ogroup = gfa2.groups_o.len();
    let ugroup = gfa2.groups_u.len();

    assert_eq!(head, 2);
    assert_eq!(seg, 1);
    assert_eq!(frag, 0);
    assert_eq!(edge, 1);
    assert_eq!(gap, 0);
    assert_eq!(ogroup, 0);
    assert_eq!(ugroup, 0);

    println!("{}", gfa2);
}

#[test]
#[ignore]
fn can_parse_big_file_gfa2() {
    // parsing file and counting items, about 7 minutes (WITHOUT PROGRESSBAR)
    // parsing file and counting items, about 14 minutes (WITH PROGRESSBAR)
    let parser: GFA2Parser<BString, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<BString, OptionalFields> =
        parser.parse_file(&"./tests/big_files/ape-4-0.10b.gfa2").unwrap();

    let head = gfa2.headers.len();
    let seg = gfa2.segments.len(); // 715018
    let frag = gfa2.fragments.len();
    let edge = gfa2.edges.len(); // 985462
    let gap = gfa2.gaps.len();
    let ogroup = gfa2.groups_o.len();
    let ugroup = gfa2.groups_u.len();

    println!(
        "Header lines: {}\n
        Segment lines: {}\n 
        Fragment lines: {}\n
        Edge lines: {}\n
        Gap lines: {}\n
        GroupO lines: {}\n
        GroupU lines: {}\n",
        head, seg, frag, edge, gap, ogroup, ugroup);
}

#[test]
#[ignore]
fn can_parse_big_file_gfa1() {
    // parsing file and counting items, about 7 minutes (WITHOUT PROGRESSBAR)
    // parsing file and counting items, about 14 minutes (WITH PROGRESSBAR)
    let parser: GFAParser<BString, OptionalFields> = GFAParser::new();
    let gfa: GFA<BString, OptionalFields> =
        parser.parse_file(&"./tests/big_files/ape-4-0.10b.gfa").unwrap();

    let head = gfa.headers.len();
    let seg = gfa.segments.len(); // 715018
    let link = gfa.links.len();// 985462
    let cont = gfa.containments.len(); 
    let path = gfa.paths.len();

    println!(
        "Header lines: {}\n
        Segment lines: {}\n 
        Link lines: {}\n
        Containments lines: {}\n
        Path lines: {}\n",
        head, seg, link, cont, path);
}

#[test]
fn can_parse_gfa_lines() {
    let parser = GFAParser::new();
    let gfa: GFA<BString, ()> =
        parser.parse_file(&"./tests/gfa1_files/lil.gfa").unwrap();

    let num_segs = gfa.segments.len();
    let num_links = gfa.links.len();
    let num_paths = gfa.paths.len();
    let num_conts = gfa.containments.len();

    assert_eq!(num_segs, 15);
    assert_eq!(num_links, 20);
    assert_eq!(num_conts, 0);
    assert_eq!(num_paths, 3);

    println!("{}", gfa);
}

#[test]
fn gfa_usize_parser() {
    let usize_parser: GFAParser<usize, OptionalFields> = GFAParser::new();
    let usize_gfa = usize_parser.parse_file(&"./tests/gfa1_files/diatom.gfa");
    
    assert!(!usize_gfa.is_err())
}

#[test]
fn can_parse_medium_file_gfa1() {
    let parser: GFAParser<BString, OptionalFields> = GFAParser::new();
    let gfa: GFA<BString, OptionalFields> =
        parser.parse_file(&"./tests/big_files/test.gfa").unwrap();

    let head = gfa.headers.len();
    let seg = gfa.segments.len(); // 4058
    let link = gfa.links.len();// 10639
    let cont = gfa.containments.len(); 
    let path = gfa.paths.len(); // 7

    println!(
        "Header lines: {}\n
        Segment lines: {}\n 
        Link lines: {}\n
        Containments lines: {}\n
        Path lines: {}\n",
        head, seg, link, cont, path);
}

#[test]
fn can_parse_medium_file_gfa2() {
    let parser: GFA2Parser<BString, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<BString, OptionalFields> =
        parser.parse_file(&"./tests/big_files/test.gfa2").unwrap();

    let head = gfa2.headers.len();
    let seg = gfa2.segments.len(); // 4058
    let frag = gfa2.fragments.len();
    let edge = gfa2.edges.len(); // 10639
    let gap = gfa2.gaps.len();
    let ogroup = gfa2.groups_o.len(); // 7
    let ugroup = gfa2.groups_u.len();

    println!(
        "Header lines: {}\n
        Segment lines: {}\n 
        Fragment lines: {}\n
        Edge lines: {}\n
        Gap lines: {}\n
        GroupO lines: {}\n
        GroupU lines: {}\n",
        head, seg, frag, edge, gap, ogroup, ugroup);
}


#[test]
fn gfa_parser_line_iter() {
    use {
        bstr::io::BufReadExt,
        std::{fs::File, io::BufReader},
    };

    let parser: GFAParser<usize, ()> = GFAParser::new();
    let file = File::open(&"./tests/gfa1_files/lil.gfa").unwrap();
    let lines = BufReader::new(file).byte_lines().map(|x| x.unwrap());
    let parser_iter = GFAParserLineIter::from_parser(parser, lines);

    let segment_names = parser_iter
        .filter_map(|line| {
            let line = line.ok()?;
            let seg = line.some_segment()?;

            Some(seg.name)
        })
        .collect::<Vec<_>>();

    assert_eq!(segment_names, (1..=15).into_iter().collect::<Vec<_>>());
}