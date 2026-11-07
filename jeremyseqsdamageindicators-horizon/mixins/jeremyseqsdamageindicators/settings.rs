// The mod's settings page: one ON/OFF button per option of DamageIndicatorsConfig, then a - and a + button per
// number option the mod's own screen has a slider for (same range and step), kept with Horizon's settings
// (one file per option in the mod's data folder). Other number and text options keep their values.
fn jeremyseqsdamageindicators_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::port_jeremyseqsdamageindicators as m;
    use crate::game::port_jeremyseqsdamageindicators::java::damage_indicators_config as cfg;
    if mod_id != m::ID {
        return;
    }
    cfg::ensure_loaded(|k| game.native_setting(m::ID, k));
    let c = cfg::INSTANCE.get();
    for (key, label) in cfg::DamageIndicatorsConfig::TOGGLES {
        buttons.push(format!("{label}: {}", if c.flag(key) { "ON" } else { "OFF" }));
    }
    for (key, label, ..) in cfg::DamageIndicatorsConfig::NUMBERS {
        buttons.push(format!("{label}: {}  [-]", c.number_text(key)));
        buttons.push(format!("{label}  [+]"));
    }
}
