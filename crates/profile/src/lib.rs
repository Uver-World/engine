pub mod models;

use std::{fmt, fs::File};

use serde::{
    de::{self, Deserializer, MapAccess, SeqAccess, Visitor},
    Deserialize, Serialize,
};
use serde_json::Value;

use crate::models::{
    entity::{Entity, EntityGroup},
    surface::{Surface, SurfaceGroup},
};

#[derive(Serialize)]
pub struct Profile {
    project_name: String,
    entity_groups: Vec<EntityGroup>,
    entities: Vec<Entity>,
    surface_groups: Vec<SurfaceGroup>,
    surfaces: Vec<Surface>,
}

impl Profile {
    pub fn new(project_name: String) -> Self {
        Self {
            project_name: project_name,
            entity_groups: Vec::new(),
            entities: Vec::new(),
            surface_groups: Vec::new(),
            surfaces: Vec::new(),
        }
    }

    pub fn load(project_name: String) -> Result<Self, String> {
        let file_path = format!("{}.json", project_name);
        let file = File::open(&file_path);
        if file.is_err() {
            return Err(file.unwrap_err().to_string());
        }
        let file = file.unwrap();
        let settings = serde_json::from_reader(&file);
        if settings.is_err() {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                let _ = std::fs::write(format!("{}.old", &file_path), content);
            }
            return Err(format!("{:?}", settings.err().unwrap()));
        }
        Ok(settings.unwrap())
    }

    pub fn save(&self) {
        std::fs::write(
            format!("{}.json", self.project_name),
            serde_json::to_string_pretty(self).unwrap(),
        )
        .unwrap();
    }

    pub fn get_entities(&self) -> &Vec<Entity> {
        &self.entities
    }
}

impl<'de> de::Deserialize<'de> for Profile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            ProjectName,
            EntityGroups,
            Entities,
            SurfaceGroups,
            Surfaces,
        }

        struct ProfileVisitor;

        impl<'de> Visitor<'de> for ProfileVisitor {
            type Value = Profile;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Profile")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Profile, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let project_name = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let entity_groups: Vec<EntityGroup> = Vec::new();
                let entities: Vec<Entity> = Vec::new();
                let surface_groups: Vec<SurfaceGroup> = Vec::new();
                let surfaces: Vec<Surface> = Vec::new();

                Ok(Profile {
                    project_name,
                    entity_groups,
                    entities,
                    surface_groups,
                    surfaces,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Profile, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut project_name = None;
                let mut entity_groups = None;
                let mut entities: Option<Vec<Value>> = None;
                let mut surface_groups = None;
                let mut surfaces: Option<Vec<Value>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ProjectName => {
                            if project_name.is_some() {
                                return Err(de::Error::duplicate_field("project_name"));
                            }
                            project_name = Some(map.next_value()?);
                        }
                        Field::Entities => {
                            if entities.is_some() {
                                return Err(de::Error::duplicate_field("entities"));
                            }
                            entities = Some(map.next_value()?);
                        }
                        Field::EntityGroups => {
                            if entity_groups.is_some() {
                                return Err(de::Error::duplicate_field("entity_groups"));
                            }
                            entity_groups = Some(map.next_value()?);
                        }
                        Field::SurfaceGroups => {
                            if surface_groups.is_some() {
                                return Err(de::Error::duplicate_field("surface_groups"));
                            }
                            surface_groups = Some(map.next_value()?);
                        }
                        Field::Surfaces => {
                            if surfaces.is_some() {
                                return Err(de::Error::duplicate_field("surfaces"));
                            }
                            surfaces = Some(map.next_value()?);
                        }
                    }
                }

                let project_name: String =
                    project_name.ok_or_else(|| de::Error::missing_field("project_name"))?;
                let entity_groups: Vec<EntityGroup> =
                    entity_groups.ok_or_else(|| de::Error::missing_field("entity_groups"))?;
                let surface_groups: Vec<SurfaceGroup> =
                    surface_groups.ok_or_else(|| de::Error::missing_field("surface_groups"))?;

                let entities: Vec<Entity> = Entity::deserialize(entities, entity_groups.clone())
                    .ok_or_else(|| de::Error::missing_field("entities"))?;
                let surfaces: Vec<Surface> = Surface::deserialize(surfaces, surface_groups.clone())
                    .ok_or_else(|| de::Error::missing_field("surfaces"))?;

                Ok(Profile {
                    project_name,
                    entity_groups,
                    entities,
                    surface_groups,
                    surfaces,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            "project_name",
            "entity_groups",
            "entities",
            "surface_groups",
            "surfaces",
        ];
        deserializer.deserialize_struct("Profile", FIELDS, ProfileVisitor)
    }
}
