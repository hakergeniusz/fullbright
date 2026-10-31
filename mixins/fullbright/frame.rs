// The lightmap this frame: full brightness while the mod is on, the player's
// own brightness (the video slider and Horizon Extras) otherwise. The slider,
// options.txt and the Extras' own full light are never changed, so turning the
// mod off (or off in Options > Mods) gives the player's own brightness back.
fn fullbright_frame(game: &mut crate::game::Game<'_>) {
    use crate::game::fullbright as fb;
    // Turned off in Options > Mods: the state is nothing to apply, and what
    // the last frame left on the renderer is given back below.
    let state = if game.native_on(fb::ID) {
        let game = &*game;
        Some(fb::state(|key| game.native_setting(fb::ID, key)))
    } else {
        None
    };
    let Some(renderer) = game.renderer.as_mut() else {
        return;
    };
    match state.filter(|state| state.on) {
        Some(state) => match state.mode {
            fb::Mode::Full => renderer.fullbright = true,
            fb::Mode::Gamma => renderer.gamma = state.gamma(),
        },
        None => {
            renderer.fullbright = game.settings.extras.fullbright;
            renderer.gamma = game.settings.gamma;
        }
    }
}
