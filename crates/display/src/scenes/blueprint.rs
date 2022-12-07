pub struct Blueprint;

impl Blueprint {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(DisplayState::Blueprint).with_system(construct))
            .add_system_set(SystemSet::on_exit(DisplayState::Blueprint).with_system(destroy))
            .add_system_set(SystemSet::on_update(DisplayState::Blueprint).with_system(update_status),);
    }

    pub fn construct() {
        println!("Constructing blueprint scene");
    }

    pub fn destroy() {
        println!("Destroying blueprint scene");
    }

    pub fn update_status() {
        println!("Updating blueprint scene");
    }
}