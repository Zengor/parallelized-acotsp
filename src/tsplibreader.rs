use crate::instance_data::{FileData, InstanceData, Metadata};
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn read_instance_file(f_name: &str) -> FileData {
    use crate::instance_data::DataDescriptionType::*;
    
    let f = File::open(f_name).expect("Failed opening file");
    let mut lines = BufReader::new(&f).lines();
    let mut metadata = Metadata::default();
    let mut size: usize = 0;
    let mut data_layout = NODE_COORD_SECTION;
    
    loop {
        let line = lines.next().unwrap().unwrap();
        let split: Vec<_> = line.split(": ").collect();
        match split[0].trim() {
            "NAME" => metadata.name = split[1].to_owned(),
            "DIMENSION" => size = split[1].parse().unwrap(),
            "EDGE_WEIGHT_TYPE" => metadata.edge_weight_type = Some(split[1].parse().unwrap()),
            "NODE_COORD_SECTION" => { 
                data_layout = NODE_COORD_SECTION;
                break;
            },
            _ => (),
        }
    }

    let instance_data = match data_layout {
        NODE_COORD_SECTION => read_node_coord_section(lines, &metadata, size),
        _ => unimplemented!()
    };

    FileData {
        metadata,
        data: instance_data,
    }
}

fn read_node_coord_section(lines: std::io::Lines<BufReader<&File>>,
                           metadata: &Metadata,
                           size: usize) -> InstanceData {
    use crate::util::distance_funcs;
    use crate::instance_data::EdgeWeightType::*;
    
    let mut nodes: Vec<(i32, i32)> = Vec::with_capacity(size);
    for line in lines {
        let line = line.unwrap();
        if line.trim() == "EOF" {
            break;
        }
        let split: Vec<_> = line.split_whitespace().collect();
        nodes.push((split[1].parse().unwrap(), split[2].parse().unwrap()));
    }
    let mut distances: crate::util::IntegerMatrix = Vec::with_capacity(size);
    for &i in nodes.iter() {
        let mut distances_from_this_node: Vec<u32> = Vec::with_capacity(size);
        for &j in nodes.iter() {
            match metadata.edge_weight_type.as_ref().expect("No defined edge_weight_type") {
                EUC_2D => distances_from_this_node.push(distance_funcs::euc_2d(i, j)),
                _ => unimplemented!(),
            }
        }
        distances.push(distances_from_this_node);
    }
    InstanceData {
        size,
        distances
    }
}
