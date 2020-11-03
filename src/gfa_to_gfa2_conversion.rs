use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use bstr::BString;

use crate::gfa2::{GFA2, Header, Segment, Edge, GroupO};
use crate::tag::*;

/// Very BASIC converter from GFA1 format to GFA2 format.\
/// For now it consider only S-, L- and P- lines.
/// ignoring all the others.
/// WIP
fn convert_gfa_file(path: String) -> GFA2<BString, OptionalFields> {
    let mut gfa2 = GFA2::new();
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for(_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut line_split = line.split_whitespace();
        let prefix = line_split.next().unwrap();
        
        match prefix {
            "H" => {
                // default header
                // ignore tag fields
                gfa2.headers = vec![
                    Header::new(Some("VN:Z:2.0".into()))
                ];
            },
            "S" => {
                let id = BString::from(line_split.next().unwrap());
                let sequence = BString::from(line_split.next().unwrap());
                let len = BString::from(sequence.len().to_string());

                let mut tag = line_split.next();
                let mut opt_fields: OptionalFields = vec![];
                while !tag.is_none() {
                    opt_fields.push(OptField::parse(tag.unwrap().as_bytes()).unwrap());
                    tag = line_split.next();
                }
                
                let segment = Segment {
                    id: id,
                    len: len, 
                    sequence: sequence,
                    tag: opt_fields,
                };
                gfa2.segments.push(segment);
            },
            "L" => {
                // placeholder value
                let id = BString::from("*");

                let from_node = BString::from(line_split.next().unwrap());
                let from_node_orient = BString::from(line_split.next().unwrap());
                let to_node = BString::from(line_split.next().unwrap());
                let to_node_orient = BString::from(line_split.next().unwrap());

                // placeholder values
                let beg1 = BString::from("0");
                let end1 = BString::from("0$");
                let beg2 = BString::from("0");
                let end2 = BString::from("0$");

                let alignment = BString::from(line_split.next().unwrap());

                let mut tag = line_split.next();
                let mut opt_fields: OptionalFields = vec![];
                while !tag.is_none() {
                    opt_fields.push(OptField::parse(tag.unwrap().as_bytes()).unwrap());
                    tag = line_split.next();
                }

                let edge = Edge {
                    id: id,
                    sid1: BString::from(format!("{}{}", from_node, from_node_orient)),
                    sid2: BString::from(format!("{}{}", to_node, to_node_orient)),
                    beg1: beg1,
                    end1: end1,
                    beg2: beg2,
                    end2: end2,
                    alignment: alignment,
                    tag: opt_fields,
                };
                gfa2.edges.push(edge);
            },
            // TODO: the C-line should be inserted with the same L-line in the 
            // corresponding E-line
            "C" => (),
            "P" => {
                let id = BString::from(line_split.next().unwrap());
                let seg_ids = line_split.next().unwrap();
                let res = BString::from(str::replace(seg_ids, ",", " "));

                let mut tag = line_split.next();
                let mut opt_fields: OptionalFields = vec![];
                while !tag.is_none() {
                    opt_fields.push(OptField::parse(tag.unwrap().as_bytes()).unwrap());
                    tag = line_split.next();
                }

                let ogroup = GroupO::new(
                    id,
                    res,
                    opt_fields,
                );
                gfa2.groups_o.push(ogroup);
            },
            // ignore all the other lines (typically comment-lines)
            _ => (),
        }
    }
    gfa2
}

fn write_on_file(gfa2: GFA2<BString, OptionalFields>, path: String) -> std::io::Result<()> {
    let path = format!("{}{}", path, "2");
    let mut file = File::create(path)?;
    file.write_all(format!("{}", gfa2).as_bytes())?;
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn can_parse_and_write_big_file() {
        // conversion and write on file, about 5.30 minutes
        let path: String = "./tests/big_files/ape-4-0.10b.gfa".to_string();
        //let path: String = "./tests/big_files/test.gfa".to_string();
        let gfa2: GFA2<BString, OptionalFields> = convert_gfa_file(path.clone());
        match write_on_file(gfa2, path) {
            Ok(_) => println!("File converted and saved correctly!"),
            Err(why) => println!("Error: {}", why),
        }
    }

    #[test]
    fn can_parse_and_wirte_file_with_tags() {
        let path: String = "./tests/big_files/diatom.gfa".to_string();
        let gfa2: GFA2<BString, OptionalFields> = convert_gfa_file(path.clone());
        match write_on_file(gfa2, path) {
            Ok(_) => println!("File converted and saved correctly!"),
            Err(why) => println!("Error: {}", why),
        }
    }
}