// The toggle key went down or up: the player's choice of key, kept as
// `key.fullbright.toggle` in `horizon-mods.txt`.
fn fullbright_key(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    use crate::game::fullbright as fb;
    // Turned off in Options > Mods: nothing until the next build leaves it out.
    if !game.native_on(fb::ID) {
        return;
    }
    // Only the press counts, and only while playing: keys reach this hook on
    // every screen, and typing the key in chat must not toggle.
    if !pressed || !game.native_playing() {
        return;
    }
    let bound = game.native_mod_key(fb::ID, fb::KEY, fb::DEFAULT_KEY);
    if key != bound {
        return;
    }
    let game = &*game;
    let mut state = fb::state(|key| game.native_setting(fb::ID, key));
    state.on = !state.on;
    fb::set_state(state);
    for (key, value) in state.stored() {
        game.native_set_setting(fb::ID, key, &value);
    }
}
