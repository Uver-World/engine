pub mod scene_filter;

use std::collections::HashSet;

use bevy::{ecs::system::Query, render::view::Visibility};
use client_profile::models::{Color, Direction, Shape};

use crate::entities::ui_entity::DisplayEntity;

pub struct Filter {
    pub color_filters: HashSet<Color>,
    pub group_filters: HashSet<String>,
    pub shape_filters: HashSet<Shape>,
    pub direction_filters: Vec<Direction>,
    pub toggled_color_filters: HashSet<Color>,
    pub toggled_group_filters: HashSet<String>,
    pub toggled_shape_filters: HashSet<Shape>,
    pub toggled_direction_filters: Vec<Direction>,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            color_filters: HashSet::new(),
            group_filters: HashSet::new(),
            shape_filters: HashSet::new(),
            direction_filters: Vec::new(),
            toggled_color_filters: HashSet::new(),
            toggled_group_filters: HashSet::new(),
            toggled_shape_filters: HashSet::new(),
            toggled_direction_filters: Vec::new(),
        }
    }

    fn make_all_entities_visible(entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
        for (_, mut visibility) in entities {
            *visibility = Visibility::Visible;
        }
    }

    pub fn add_direction_filter(&mut self, direction: Direction) {
        if !self.direction_filters.contains(&direction) {
            self.direction_filters.push(direction);
        }
    }

    pub fn toggle_color_filter(
        &mut self,
        color_filter: Color,
        entities: &mut Query<(&DisplayEntity, &mut Visibility)>,
    ) {
        self.toggle_color(color_filter);
        self.update_filter(entities);
    }

    pub fn toggle_group_filter(
        &mut self,
        group_filter: String,
        entities: &mut Query<(&DisplayEntity, &mut Visibility)>,
    ) {
        self.toggle_group(group_filter);
        self.update_filter(entities);
    }

    pub fn toggle_shape_filter(
        &mut self,
        shape_filter: Shape,
        entities: &mut Query<(&DisplayEntity, &mut Visibility)>,
    ) {
        self.toggle_shape(shape_filter);
        self.update_filter(entities);
    }

    pub fn toggle_direction_filter(
        &mut self,
        direction_filter: Direction,
        entities: &mut Query<(&DisplayEntity, &mut Visibility)>,
    ) {
        self.toggle_direction(direction_filter);
        self.update_filter(entities);
    }

    fn toggle_direction(&mut self, direction_filter: Direction) {
        if !self.toggled_direction_filters.contains(&direction_filter) {
            self.toggled_direction_filters.push(direction_filter);
        } else {
            self.toggled_direction_filters
                .retain(|direction| direction != &direction_filter)
        }
    }

    fn toggle_shape(&mut self, shape_filter: Shape) {
        if !self.toggled_shape_filters.remove(&shape_filter) {
            self.toggled_shape_filters.insert(shape_filter);
        }
    }

    fn toggle_group(&mut self, group_filter: String) {
        if !self.toggled_group_filters.remove(&group_filter) {
            self.toggled_group_filters.insert(group_filter);
        }
    }

    fn toggle_color(&mut self, color_filter: Color) {
        if !self.toggled_color_filters.remove(&color_filter) {
            self.toggled_color_filters.insert(color_filter);
        }
    }

    fn update_filter(&self, entities: &mut Query<(&DisplayEntity, &mut Visibility)>) {
        if self.toggled_color_filters.is_empty()
            && self.toggled_group_filters.is_empty()
            && self.toggled_shape_filters.is_empty()
            && self.toggled_direction_filters.is_empty()
        {
            return Self::make_all_entities_visible(entities);
        }

        for (display_entity, mut visibility) in entities {
            if self
                .toggled_color_filters
                .contains(&display_entity.settings.group.color)
                || self
                    .toggled_group_filters
                    .contains(&display_entity.settings.group.name)
                || self
                    .toggled_shape_filters
                    .contains(&display_entity.settings.group.shape)
                || self.has_direction(display_entity)
            {
                *visibility = Visibility::Visible
            } else {
                *visibility = Visibility::Hidden
            }
        }
    }

    fn has_direction(&self, display_entity: &DisplayEntity) -> bool {
        for direction in &display_entity.settings.group.directions {
            if self.toggled_direction_filters.contains(direction) {
                return true;
            }
        }

        false
    }
}
