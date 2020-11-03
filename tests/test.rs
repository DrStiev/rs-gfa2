use gfa2::{
    gfa2::GFA2,
    parser_gfa2::GFA2Parser,
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
    let _gfa2: GFA2<BString, OptionalFields> =
        parser.parse_file(&"./tests/gfa2_files/graph.gfa").unwrap();

    // println!("{}", gfa2);
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
