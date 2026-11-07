// A click on button `button` of the mod's settings page: flips that option, or moves that number a
// step, and keeps it.
fn jeremyseqsdamageindicators_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::port_jeremyseqsdamageindicators as m;
    use crate::game::port_jeremyseqsdamageindicators::java::damage_indicators_config as cfg;
    if mod_id != m::ID {
        return;
    }
    cfg::ensure_loaded(|k| game.native_setting(m::ID, k));
    let toggles = cfg::DamageIndicatorsConfig::TOGGLES;
    cfg::INSTANCE.with(|c| {
        if let Some(&(key, _)) = toggles.get(button) {
            c.toggle(key);
        } else if let Some(&(key, ..)) = cfg::DamageIndicatorsConfig::NUMBERS.get((button - toggles.len()) / 2) {
            c.nudge(key, if (button - toggles.len()) % 2 == 0 { -1 } else { 1 });
        } else {
            return;
        }
        c.save_settings(|k, v| game.native_set_setting(m::ID, k, v));
    });
}
