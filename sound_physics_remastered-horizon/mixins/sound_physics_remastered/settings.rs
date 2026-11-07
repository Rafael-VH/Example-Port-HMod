// The mod's settings page: one button per option, labels pushed in order.
// Stands in for the Java config screen.
fn sound_physics_remastered_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::port_sound_physics_remastered as m;
    if mod_id != m::ID {
        return;
    }
    // TODO: push a label per option, e.g. format!("Enabled: {}", if on { "ON" } else { "OFF" }),
    // reading each with game.native_setting(m::ID, key).
    let _ = (game, buttons);
}
