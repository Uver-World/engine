use bevy::prelude::*;
use bevy::{
    ecs::system::Query,
    text::Text,
    ui::{BackgroundColor, Interaction},
};

pub fn button_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => text.sections[0].style.color = Color::rgb_u8(0, 0, 0).into(),
            Interaction::Hovered => {
                text.sections[0].style.color = Color::rgb_u8(100, 100, 100).into()
            }
            Interaction::None => text.sections[0].style.color = Color::rgb_u8(176, 176, 176).into(),
        }
    }
}
