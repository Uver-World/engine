use serde::{
    ser::{self, SerializeStruct, Serializer},
    Deserialize, Serialize,
};

use crate::models::location::Location;

#[derive(Serialize, Deserialize, Clone)]
pub struct EntityGroup {
    pub group: String,
}

pub struct Entity {
    group: String,
    location: Location,
}

impl Entity {
    pub fn new(group: EntityGroup, location: Location) -> Self {
        Self {
            group: group.group,
            location,
        }
    }

    pub fn deserialize(map: Option<Vec<String>>, groups: Vec<EntityGroup>) -> Option<Vec<Self>> {
        if map.is_none() {
            return None;
        }
        let map = map.unwrap();
        Some(Vec::new())
    }
}

impl ser::Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Entity", 2)?;
        state.serialize_field("group", &self.group)?;
        state.serialize_field("location", &self.location)?;
        state.end()
    }
}
