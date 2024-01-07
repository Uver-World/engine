use std::collections::HashSet;

use bevy::{ecs::system::Query, render::view::Visibility};
use client_profile::models::color::Color;

use crate::entities::ui_entity::DisplayEntity;

pub struct Filter {
    pub color_filters: HashSet<Color>,
}

impl Filter {
    
    pub fn new() -> Self {
        Self  {
            color_filters: HashSet::new(),
        }
    }

    fn make_all_entities_visible(entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
        for (_, mut visibility) in entities {
                *visibility = Visibility::Visible;
        }
    }

    pub fn toggle_color_filter(&mut self, color_filter: Color, entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
        self.toggle_color(color_filter);
        
        if self.color_filters.is_empty() {
            return Self::make_all_entities_visible(entities);
        }
        
        self.update_filter(entities);
    }
    
    fn toggle_color(&mut self, color_filter: Color) {
        if !self.color_filters.remove(&color_filter) {
            self.color_filters.insert(color_filter);
        }
    }
    
    fn update_filter(&self, entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
        for (display_entity, mut visibility) in entities {
            if self.color_filters.contains(&display_entity.settings.group.color) {
                *visibility = Visibility::Visible
            } else {
                *visibility = Visibility::Hidden
            }
        }
    }
}
