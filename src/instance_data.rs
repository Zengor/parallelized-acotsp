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

pub enum DataDescriptionType {
    NODE_COORD_SECTION,
    DEPOT_SECTION,
    DEMAND_SECTION,
    EDGE_DATA_SECTION,
    FIXED_EDGES_SECTION,
    DISPLAY_DATA_SECTION,
    TOUR_SECTION,
    EDGE_WEIGHT_SECTION,

}

pub enum EdgeWeightType {
    EXPLICIT,
    EUC_2D,
    EUC_3D,
    MAX_2D,
    MAX_3D,
    MAN_2D,
    MAN_3D,
    CEIL_2D,
    GEO,
    ATT,
    XRAY1,
    XRAY2,
    SPECIAL
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
