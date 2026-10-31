// The mod's settings page: the toggle, the mode and the gamma strength.
fn fullbright_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::fullbright as fb;
    if mod_id != fb::ID {
        return;
    }
    let game = &*game;
    let state = fb::state(|key| game.native_setting(fb::ID, key));
    buttons.extend(state.labels());
}
