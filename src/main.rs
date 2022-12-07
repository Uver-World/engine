use client_display::*;
use client_profile::*;

fn main() {
    let display = ClientDisplay {
        profile: Profile::load(),
    };

    display.run_display();
}
