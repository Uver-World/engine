use serde::ser::{self, SerializeStruct, Serializer};
use serde_json::Value;

use super::SurfaceGroup;

pub struct Surface {
    pub group: SurfaceGroup,
}

impl Surface {
    pub fn new(group: SurfaceGroup) -> Self {
        Self { group }
    }

    pub fn deserialize(map: Option<Vec<Value>>, groups: Vec<SurfaceGroup>) -> Option<Vec<Self>> {
        if map.is_none() {
            return None;
        }
        let map = map.unwrap();

        let mut surfaces = Vec::new();

        for surface in map {
            match deserialize_surface(surface, &groups) {
                Ok(surface) => surfaces.push(surface),
                Err(_) => return None,
            }
        }

        Some(surfaces)
    }
}

impl ser::Serialize for Surface {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Surface", 2)?;
        state.serialize_field("group", &self.group.group)?;
        state.end()
    }
}
fn retrieve_group(target: &str, groups: &Vec<SurfaceGroup>) -> Option<SurfaceGroup> {
    for group in groups {
        if target == group.group {
            return Some(group.to_owned());
        }
    }
    None
}

fn deserialize_surface(entity: Value, groups: &Vec<SurfaceGroup>) -> Result<Surface, String> {
    let group = match entity
        .get("group")
        .and_then(|group| group.as_str())
        .map_or(Err("group field not found".to_string()), |group_str| {
            Ok(group_str)
        }) {
        Ok(group) => match retrieve_group(group, groups) {
            Some(group) => group,
            None => return Err(format!("group {} not found", group)),
        },
        Err(err) => return Err(err),
    };

    Ok(Surface::new(group))
}
