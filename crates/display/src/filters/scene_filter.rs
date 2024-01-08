use bevy::ecs::system::{ResMut, Query};
use bevy::render::view::Visibility;
use bevy_egui::egui::Ui;
use bevy_egui::{egui, EguiContexts};

use crate::ClientDisplay;
use crate::entities::ui_entity::DisplayEntity;

pub fn filter_system(
    mut egui_context: EguiContexts,
    mut client_display: ResMut<ClientDisplay>,
    mut entities: Query<(&DisplayEntity, &mut Visibility)>  
) {
    egui::Window::new("Enum Selector").show(egui_context.ctx_mut(), |ui| {
        ui.heading("Choose Options");

        color_filter(&mut client_display, ui, &mut entities);
        group_filter(&mut client_display, ui, &mut entities);
        shape_filter(&mut client_display, ui, &mut entities);
        direction_filter(&mut client_display, ui, &mut entities);
    });
}

fn color_filter(client_display: &mut ResMut<ClientDisplay>, ui: &mut Ui, entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
    let color_filters = client_display.filter.color_filters.clone();
        
    for color in color_filters {
        let mut is_selected = client_display.filter.toggled_color_filters.contains(&color);

        if ui.checkbox(&mut is_selected, format!("{:?}", &color)).clicked() {
            client_display.filter.toggle_color_filter(color.clone(), entities);
        }
    }
}

fn group_filter(client_display: &mut ResMut<ClientDisplay>, ui: &mut Ui, entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
    let group_filters = client_display.filter.group_filters.clone();
        
    for group in group_filters {
        let mut is_selected = client_display.filter.toggled_group_filters.contains(&group);

        if ui.checkbox(&mut is_selected, format!("{:?}", &group)).clicked() {
            client_display.filter.toggle_group_filter(group.clone(), entities);
        }
    }
}

fn shape_filter(client_display: &mut ResMut<ClientDisplay>, ui: &mut Ui, entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
    let shape_filters = client_display.filter.shape_filters.clone();
        
    for shape in shape_filters {
        let mut is_selected = client_display.filter.toggled_shape_filters.contains(&shape);

        if ui.checkbox(&mut is_selected, format!("{:?}", &shape)).clicked() {
            client_display.filter.toggle_shape_filter(shape.clone(), entities);
        }
    }
}

fn direction_filter(client_display: &mut ResMut<ClientDisplay>, ui: &mut Ui, entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
    let direction_filters = client_display.filter.direction_filters.clone();
        
    for direction in direction_filters {
        let mut is_selected = client_display.filter.toggled_direction_filters.contains(&direction);

        if ui.checkbox(&mut is_selected, format!("{:?}", &direction)).clicked() {
            client_display.filter.toggle_direction_filter(direction.clone(), entities);
        }
    }
}
