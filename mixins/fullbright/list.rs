// The mod's row in Options > Mods, with its key.
fn fullbright_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::fullbright as fb;
    mods.push(crate::game::native_hooks::NativeMod {
        id: fb::ID.into(),
        title: format!("{} {}", fb::NAME, fb::VERSION),
        description: fb::DESCRIPTION.into(),
        keys: vec![[
            fb::KEY.into(),
            "Toggle fullbright".into(),
            fb::DEFAULT_KEY.into(),
        ]],
    });
}
