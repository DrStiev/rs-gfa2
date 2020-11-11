# GFA2 in Rust
This crate provides Rust types and parsers for the GFA (Graphical
Fragment Assembly) format, version 1 and 2.

## Compatibility
This library is compatible with the [version 2 specification](https://github.com/GFA-spec/GFA-spec/blob/master/GFA2.md) and [version 1 specification](https://github.com/GFA-spec/GFA-spec/blob/master/GFA1.md) of GFA.\
This library it's a variation of the library developed by **Christian Fischer** [link here](https://github.com/chfi/rs-gfa).

## Usage
This library performs 1 main operation on a file: 
- Checking if a file is conform to a specified format and then create a GFA2 (or GFA) object.\
Given a file for example: file.gfa2 
```
H	VN:Z:2.0
H	ul:Z:https://github.com/pmelsted/GFA-spec/issues/7#issuecomment-219685552
S	11	5	ACCTT	pg:J:{"Human":[{"target":"chr1","pos":1500,"strand":true}],"Mouse":[{"target":"chr2","pos":2000,"strand":false}],"ecoli":[{"target":"chr1","pos":2000,"strand":false},{"target":"chr1","pos":3000,"strand":true}]}
S	12	6	TCAAGG
S	13	7	CTTGATT
E	*	11+	12-	1	5$	2	6$	4M
E	*	12-	13+	0	5	0	5	5M
E	*	11+	13+	2	5$	0	3	3M
O	14	11+ 12- 13+
```
This can be parsed to create 4 different GFA2 Objects:
1. GFA2<BString, OptionalFields>, this create an Object containing all the fields parsed as [BString](https://docs.rs/bstr/0.2.14/bstr/struct.BString.html) with all the optional tags.\
```rust
let parser: GFA2Parser<BString, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<BString, OptionalFields> = parser
        .parse_file(&"./tests/gfa2_files/spec_q7.gfa")
        .unwrap();
    println!("{:#?}", gfa2);
```
The Object obtained will look like this:
```
GFA2 {
    headers: [
        Header {
            version: Some("VN:Z:2.0"),
            tag: [],
        },
        Header {
            version: Some("ul:Z:https://github.com/pmelsted/GFA-spec/issues7#issuecomment-219685552"),
            tag: [],
        },
    ],
    segments: [
        Segment {
            id: "11",
            len: "5",
            sequence: "ACCTT",
            tag: [
                OptField {
                    tag: [112,103],
                    value: J("pg:J:{\"Human\":[{\"target\":\"chr1\",\"pos\":1500,\"strand\":true}],\"Mouse\":[{\"target\":\"chr2\",\"pos\":2000,\"strand\":false}],\"ecoli\":[{\"target\":\"chr1\",\"pos\":2000,\"strand\":false},{\"target\":\"chr1\",\"pos\":3000,\"strand\":true}]}"),
                },
            ],
        },
        Segment {
            id: "12",
            len: "6",
            sequence: "TCAAGG",
            tag: [],
        },
        Segment {
            id: "13",
            len: "7",
            sequence: "CTTGATT",
            tag: [],
        },
    ],
    fragments: [],
    edges: [
        Edge {
            id: "*",
            sid1: "11+",
            sid2: "12-",
            beg1: "1",
            end1: "5$",
            beg2: "2",
            end2: "6$",
            alignment: "4M",
            tag: [],
        },
        Edge {
            id: "*",
            sid1: "12-",
            sid2: "13+",
            beg1: "0",
            end1: "5",
            beg2: "0",
            end2: "5",
            alignment: "5M",
            tag: [],
        },
        Edge {
            id: "*",
            sid1: "11+",
            sid2: "13+",
            beg1: "2",
            end1: "5$",
            beg2: "0",
            end2: "3",
            alignment: "3M",
            tag: [],
        },
    ],
    gaps: [],
    groups_o: [
        GroupO {
            id: "14",
            var_field: "11+ 12- 13+",
            tag: [],
            _segment_names: PhantomData,
        },
    ],
    groups_u: [],
}
```
2. GFA2<usize, OptionalFields>, this create an Object containing all the fields parsed as [usize](https://doc.rust-lang.org/std/primitive.usize.html) with all the optional tags.\
This kind of conversion use the [ASCII](https://www.ascii-code.com/) codes to convert letters and printable symbols into numbers.\
Moreover, this types needs to perform an additional conversion for the Orientation fields [+-] associated with the ref tag of Fragment, Edge, Gap and O-Group, converting the [+] symbol as [0] and [-] as [1].\
```rust
let parser: GFA2Parser<usize, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<usize, OptionalFields> = parser
        .parse_file(&"./tests/gfa2_files/spec_q7.gfa")
        .unwrap();
    println!("{:#?}", gfa2);
```
The Object obtained will look like this:
```
GFA2 {
    headers: [
        Header {
            version: Some("VN:Z:2.0"),
            tag: [],
        },
        Header {
            version: Some("ul:Z:https://github.com/pmelsted/GFA-spec/issues/7#issuecomment-219685552"),
            tag: [],
        },
    ],
    segments: [
        Segment {
            id: 11,
            len: "5",
            sequence: "ACCTT",
            tag: [
                OptField {
                    tag: [112,103],
                    value: J("pg:J:{\"Human\":[{\"target\":\"chr1\",\"pos\":1500,\"strand\":true}],\"Mouse\":[{\"target\":\"chr2\",\"pos\":2000,\"strand\":false}],\"ecoli\":[{\"target\":\"chr1\",\"pos\":2000,\"strand\":false},{\"target\":\"chr1\",\"pos\":3000,\"strand\":true}]}"),
                },
            ],
        },
        Segment {
            id: 12,
            len: "6",
            sequence: "TCAAGG",
            tag: [],
        },
        Segment {
            id: 13,
            len: "7",
            sequence: "CTTGATT",
            tag: [],
        },
    ],
    fragments: [],
    edges: [
        Edge {
            id: 42,
            sid1: 110,
            sid2: 121,
            beg1: "1",
            end1: "5$",
            beg2: "2",
            end2: "6$",
            alignment: "4M",
            tag: [],
        },
        Edge {
            id: 42,
            sid1: 121,
            sid2: 130,
            beg1: "0",
            end1: "5",
            beg2: "0",
            end2: "5",
            alignment: "5M",
            tag: [],
        },
        Edge {
            id: 42,
            sid1: 110,
            sid2: 130,
            beg1: "2",
            end1: "5$",
            beg2: "0",
            end2: "3",
            alignment: "3M",
            tag: [],
        },
    ],
    gaps: [],
    groups_o: [
        GroupO {
            id: "14",
            var_field: "11+ 12- 13+",
            tag: [],
            _segment_names: PhantomData,
        },
    ],
    groups_u: [],
}
```
In this example the character "*" it's converted as the number "42", that is the DECIMAL code associated with to the character in the ASCII TABLE.

3. GFA2<BString, ()>, this create an Object containing all the fields parsed as BString without the optional tags.
4. GFA2<usize, ()>, this create an Object containing all the fields parsed as usize without the optional tags. 

- A GFA Object can be pretty printed thanks to the implementation of the ``` Display traits ```:
```rust
let parser: GFA2Parser<BString, OptionalFields> = GFA2Parser::new();
    let gfa2: GFA2<BString, OptionalFields> = parser
        .parse_file(&"./tests/gfa2_files/spec_q7.gfa")
        .unwrap();
    println!("{}", gfa2);
```
Obtaining: 
```
H	VN:Z:2.0
H	ul:Z:https://github.com/pmelsted/GFA-spec/issues/7#issuecomment-219685552
S	11	5	ACCTT	pg:J:{"Human":[{"target":"chr1","pos":1500,"strand":true}],"Mouse":[{"target":"chr2","pos":2000,"strand":false}],"ecoli":[{"target":"chr1","pos":2000,"strand":false},{"target":"chr1","pos":3000,"strand":true}]}
S	12	6	TCAAGG
S	13	7	CTTGATT
E	*	11+	12-	1	5$	2	6$	4M
E	*	12-	13+	0	5	0	5	5M
E	*	11+	13+	2	5$	0	3	3M
O	14	11+ 12- 13+
```