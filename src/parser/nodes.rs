use std::collections::HashMap;

use nom::{
    bytes::complete::{tag,take_until},
    IResult, multi::{separated_list0}, 
};

use crate::{parser::{header, body::body}};
use crate::{YarnNode};

pub fn yarn_nodes(input: &str) -> IResult<&str, HashMap<String, YarnNode>> {

    let (input, nodes_raw ) =  separated_list0(tag("==="), take_until("==="))(input)?;
    let mut yarn_nodes:HashMap<String, YarnNode> = HashMap::new();

    for node_raw in nodes_raw.iter(){
        let mut node = YarnNode{..Default::default()};
        if let Ok( (body_raw, (title, tags)) ) = header(node_raw) {
            node.title = title.to_string();
            node.tags = tags;

            if let Ok((_, root_node)) = body(body_raw.trim_start()) {
                node.branch = root_node;
            }
            yarn_nodes.insert(title.to_string(), node);

        } else {
            println!("ERROR")
            // ERROR !!
        }
    }
    Ok((input, yarn_nodes))
}
