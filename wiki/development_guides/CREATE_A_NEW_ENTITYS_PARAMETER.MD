[return to list of guides](../README.MD)
# Create a new entity's parameter
> If you want to add a new parameter to the entity structure, you can do it by following these steps:

**1. Add the field to the entity's structure in: `crates/profile/src/models/entity.rs`**

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct Entity {
    pub group: EntityGroup,
    pub location: Location,
    pub name_of_new_field: u32,
}
```

**2. Update the deserialize_entity method to include the new field**
```rust
fn deserialize_entity(entity: Value, groups: &Vec<EntityGroup>) -> Result<Entity, String> {
    // let group = ...
    // let location = ...

    let name_of_new_field = match entity
        .get("name_of_new_field")
        .and_then(|value| Some(from_value::<u32>(value.to_owned())))
        .map_or(None, |name_of_new_field| name_of_new_field.ok())
    {
        Some(name_of_new_field) => name_of_new_field,
        None => return Err("name_of_new_field incorrect".to_string()),
    };

    Ok(Entity::new(group, location, name_of_new_field))
}
```


```rs
You're done, you can now use your new entity's field
```

[return to list of guides](../README.MD)