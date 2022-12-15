
use bevy::prelude::*;
use bevy::{ecs::system::Query, text::Text, ui::Interaction};

use crate::assets::blueprint;
use crate::assets::blueprint::spawn_blueprint;
use crate::scenes::blueprint::Blueprint;
use crate::ClientDisplay;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Children, &mut Style, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut client_display: ResMut<ClientDisplay>,
    assets: Res<blueprint::Assets>,
    mut commands: Commands,
    mut entity: Query<Entity, With<Blueprint>>,
    wnd: Res<Windows>,
) {
    for (interaction, children, mut style, _transform) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                // node.insert(NodeBundle {
                //     style: Style {
                //         display: Display::Flex,
                //         flex_direction: FlexDirection::Column,
                //         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                //         align_items: AlignItems::Center,
                //         align_content: AlignContent::Center,

                //         ..default()
                //     },
                //     background_color: Color::rgba(0., 0., 0., 0.).into(),
                //     ..default()
                // });
                // let ent = entity.iter().last();
                for entity in &mut entity {
                    commands.entity(entity).with_children(|parent| {
                        spawn_blueprint(
                            parent.spawn_empty(),
                            &assets,
                            Vec2 {
                                x: wnd.get_primary().unwrap().width() * 0.1,
                                y: wnd.get_primary().unwrap().height() * 0.8,
                            },
                        )
                    });
                }
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
