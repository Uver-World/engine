use serde::{
    ser::{self, SerializeStruct, Serializer},
    Deserialize, Serialize,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct SurfaceGroup {
    pub group: String,
}

pub struct Surface {
    group: String,
}

impl Surface {
    pub fn new(group: SurfaceGroup) -> Self {
        Self { group: group.group }
    }

    pub fn deserialize(map: Option<Vec<String>>, groups: Vec<SurfaceGroup>) -> Option<Vec<Self>> {
        if map.is_none() {
            return None;
        }
        let map = map.unwrap();
        Some(Vec::new())
    }
}

impl ser::Serialize for Surface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Surface", 2)?;
        state.serialize_field("group", &self.group)?;
        state.end()
    }
}
