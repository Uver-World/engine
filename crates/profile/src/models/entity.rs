use serde::{
    ser::{self, SerializeStruct, Serializer},
    Deserialize, Serialize,
};
use serde_json::{from_value, Value};

use crate::models::{color::Color, direction::Direction, location::Location, shape::Shape};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct EntityGroup {
    pub group: String,
    pub color: Color,
    pub speed: f32,
    pub directions: Vec<Direction>,
    pub shape: Shape,
}

#[derive(Clone, PartialEq)]
pub struct Entity {
    pub group: EntityGroup,
    pub location: Location,
}

impl Entity {
    pub fn new(group: EntityGroup, location: Location) -> Self {
        Self { group, location }
    }

    pub fn deserialize(map: Option<Vec<Value>>, groups: Vec<EntityGroup>) -> Option<Vec<Self>> {
        if map.is_none() {
            return None;
        }
        let map = map.unwrap();
        let mut entities = Vec::new();

        for entity in map {
            match deserialize_entity(entity, &groups) {
                Ok(entity) => entities.push(entity),
                Err(_) => return None,
            }
        }

        Some(entities)
    }
}

impl ser::Serialize for Entity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Entity", 2)?;
        state.serialize_field("group", &self.group.group)?;
        state.serialize_field("location", &self.location)?;
        state.end()
    }
}

fn retrieve_group(target: &str, groups: &Vec<EntityGroup>) -> Option<EntityGroup> {
    for group in groups {
        if target == group.group {
            return Some(group.to_owned());
        }
    }
    None
}

fn deserialize_entity(entity: Value, groups: &Vec<EntityGroup>) -> Result<Entity, String> {
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

    let location = match entity
        .get("location")
        .and_then(|value| Some(from_value::<Location>(value.to_owned())))
        .map_or(None, |location| location.ok())
    {
        Some(location) => location,
        None => return Err("location incorrect".to_string()),
    };

    Ok(Entity::new(group, location))
}
