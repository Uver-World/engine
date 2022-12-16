use bevy::prelude::*;

use crate::scenes::blueprint::Blueprint;

pub fn draw_modal(
    mut commands: Commands,
    mut entity: Query<Entity, With<Blueprint>>,
    ass: Res<AssetServer>,
) {
    for entity in &mut entity {
        commands.entity(entity).with_children(|parent| {
            parent
                // Spawn du modal
                .spawn(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size::new(Val::Percent(50.), Val::Percent(50.)),
                        position: UiRect::new(
                            Val::Percent(25.),
                            Val::Px(0.),
                            Val::Percent(25.),
                            Val::Px(0.),
                        ),
                        ..default()
                    },
                    background_color: Color::rgb_u8(93, 109, 126).into(),
                    z_index: ZIndex::Local(1),
                    ..default()
                })
                .with_children(|parent| {
                    // Spawn du coté gauche
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_wrap: FlexWrap::Wrap,
                                position: UiRect::new(
                                    Val::Px(0.),
                                    Val::Px(0.),
                                    Val::Px(0.),
                                    Val::Px(0.),
                                ),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Percent(50.0), Val::Percent(90.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(5.),
                                            Val::Px(0.),
                                            Val::Percent(1.),
                                            Val::Percent(1.),
                                        ),
                                        size: Size::new(Val::Percent(90.0), Val::Percent(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        " Nom du groupe",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(5.),
                                            Val::Px(0.),
                                            Val::Percent(1.),
                                            Val::Percent(1.),
                                        ),
                                        size: Size::new(Val::Percent(90.0), Val::Percent(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        " Couleur",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(5.),
                                            Val::Px(0.),
                                            Val::Percent(1.),
                                            Val::Percent(1.),
                                        ),
                                        size: Size::new(Val::Percent(90.0), Val::Percent(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        " Vitesse",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(5.),
                                            Val::Px(0.),
                                            Val::Percent(1.),
                                            Val::Percent(1.),
                                        ),
                                        size: Size::new(Val::Percent(90.0), Val::Percent(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        " Forme",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        });
                })
                .with_children(|parent| {
                    // Spawn du coté droit
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_wrap: FlexWrap::Wrap,
                                position: UiRect::new(
                                    Val::Percent(50.),
                                    Val::Px(0.),
                                    Val::Px(0.),
                                    Val::Px(0.),
                                ),
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Percent(50.0), Val::Percent(90.0)),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(5.),
                                            Val::Px(0.),
                                            Val::Percent(1.),
                                            Val::Percent(1.),
                                        ),
                                        size: Size::new(Val::Percent(90.0), Val::Percent(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        " Suit",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(5.),
                                            Val::Px(0.),
                                            Val::Percent(1.),
                                            Val::Percent(1.),
                                        ),
                                        size: Size::new(Val::Percent(90.0), Val::Percent(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        " Fuit",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(5.),
                                            Val::Px(0.),
                                            Val::Percent(1.),
                                            Val::Percent(1.),
                                        ),
                                        size: Size::new(Val::Percent(90.0), Val::Percent(10.0)),
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        " Se rend à la direction",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 20.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        });
                })
                .with_children(|parent| {
                    // Spawn du footer
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                position: UiRect::new(
                                    Val::Px(0.),
                                    Val::Px(0.),
                                    Val::Percent(90.),
                                    Val::Px(0.),
                                ),
                                justify_content: JustifyContent::FlexEnd,
                                align_items: AlignItems::FlexEnd,
                                position_type: PositionType::Absolute,
                                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                                ..default()
                            },
                            background_color: Color::rgb_u8(123, 139, 156).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn(ButtonBundle {
                                    style: Style {
                                        position: UiRect::new(
                                            Val::Percent(0.),
                                            Val::Px(50.),
                                            Val::Px(0.),
                                            Val::Px(10.),
                                        ),
                                        size: Size::new(Val::Percent(10.0), Val::Percent(80.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..default()
                                    },
                                    background_color: Color::rgba(0., 0., 0., 0.).into(),
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        "Valider",
                                        TextStyle {
                                            font: ass.load("FiraCode-Regular.ttf"),
                                            font_size: 30.0,
                                            color: Color::rgb(1.0, 1., 1.0),
                                        },
                                    ));
                                });
                        });
                });
        });
    }
}
