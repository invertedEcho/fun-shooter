use bevy::prelude::*;

pub fn build_normal_text_block(
    text: &str,
    font_handle: Handle<Font>,
    font_size: FontSize,
) -> impl Bundle {
    (
        Text::new(text),
        TextFont {
            font: FontSource::Handle(font_handle),
            font_size,
            ..default()
        },
    )
}
