// A click on the mod's settings page: change the state and keep it.
fn fullbright_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::fullbright as fb;
    if mod_id != fb::ID {
        return;
    }
    let game = &*game;
    let mut state = fb::state(|key| game.native_setting(fb::ID, key));
    state.click(button);
    fb::set_state(state);
    for (key, value) in state.stored() {
        game.native_set_setting(fb::ID, key, &value);
    }
}
