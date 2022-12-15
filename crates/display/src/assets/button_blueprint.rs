use bevy::prelude::*;
use bevy::{ecs::system::Query, text::Text, ui::Interaction};

use crate::ClientDisplay;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Children, &mut Style),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut client_display: ResMut<ClientDisplay>,
) {
    for (interaction, children, mut style) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = Color::rgb_u8(0, 0, 0).into();
                client_display.is_toggled = true;
                let mut i = 1.0;
                let mut temp = Val::Percent(i);
                while temp != style.position.left {
                    i += 11.;
                    temp = Val::Percent(i);
                }
                i += 11.;
                temp = Val::Percent(i);
                style.position.left = temp;
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::rgb_u8(100, 100, 100).into()
            }
            Interaction::None => text.sections[0].style.color = Color::rgb_u8(176, 176, 176).into(),
        }
    }
}
