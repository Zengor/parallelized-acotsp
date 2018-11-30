use crate::util::{IntegerMatrix};
use std::str::FromStr;

pub struct FileData {
    pub metadata: Metadata,
    pub data: InstanceData,
}

pub struct InstanceData {
    pub size: usize,
    pub distances: IntegerMatrix,
}

impl InstanceData {

}

#[allow(non_camel_case_types)]
pub enum DataDescriptionType {
    NODE_COORD_SECTION,
    // Other variants left out until implemented
}

#[allow(non_camel_case_types)]
pub enum EdgeWeightType {
    EUC_2D,
    // Other variants left out until implemented
}
impl FromStr for EdgeWeightType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}


#[derive(Default)]
pub struct Metadata {
    pub name: String,
    pub edge_weight_type: Option<EdgeWeightType>,
}
