use bevy::{
    ecs::system::Query,
    prelude::{Button, Changed, With},
    ui::Interaction,
};

pub fn button_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
) {
    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println!("NTM");
            }
            Interaction::Hovered => {
                println!("FDP");
            }
            Interaction::None => {
                println!("Weeeee");
            }
        }
    }
}
