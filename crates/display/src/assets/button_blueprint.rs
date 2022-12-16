use bevy::prelude::*;
use bevy::{ecs::system::Query, text::Text, ui::Interaction};

use crate::assets::blueprint;
use crate::assets::blueprint::spawn_blueprint;
use crate::assets::modal_blueprint::draw_modal;
use crate::scenes::blueprint::Blueprint;
use crate::ClientDisplay;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Children, &mut Transform),
        (Changed<Interaction>, With<Button>),
    >,
    mut button_query: Query<(&Children, &mut Style), With<Button>>,
    mut text_query: Query<&mut Text>,
    mut client_display: ResMut<ClientDisplay>,
    assets: Res<blueprint::Assets>,
    mut commands: Commands,
    mut entity: Query<Entity, With<Blueprint>>,
    ass: Res<AssetServer>,
) {
    for (interaction, children, _transform) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].style.color = Color::rgb_u8(0, 0, 0).into();
                if text.sections[0].value == "+" {
                    client_display.is_toggled = true;
                }
                if text.sections[0].value == "Valider" {
                    for (child, mut sty) in &mut button_query {
                        let text_button = text_query.get_mut(child[0]).unwrap();
                        if text_button.sections[0].value == "+" {
                            let mut i = 1.0;
                            let mut temp = Val::Percent(i);
                            while temp != sty.position.left {
                                i += 11.;
                                temp = Val::Percent(i);
                            }
                            for entity in &mut entity {
                                commands.entity(entity).with_children(|parent| {
                                    spawn_blueprint(
                                        parent.spawn_empty(),
                                        &assets,
                                        Vec2 { x: i, y: 87. },
                                    )
                                });
                            }
                            i += 11.;
                            temp = Val::Percent(i);
                            sty.position.left = temp;
                        }
                    }
                }
            }
            Interaction::Hovered => {
                text.sections[0].style.color = Color::rgb_u8(100, 100, 100).into()
            }
            Interaction::None => text.sections[0].style.color = Color::rgb_u8(176, 176, 176).into(),
        }
    }
    if client_display.is_toggled {
        draw_modal(commands, entity, ass);
        client_display.is_toggled = false;
    }
}
