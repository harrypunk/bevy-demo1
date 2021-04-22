use bevy::prelude::*;

pub struct StylePlugin;

impl Plugin for StylePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(set_display.system());
    }
}

//select classes
pub struct Display;
pub struct Normal;

//interaction
#[derive(Clone, PartialEq)]
pub enum Status {
    Focused,
    Normal,
    Disabled,
}

pub type ColorHandle = Handle<ColorMaterial>;

//style resources
pub struct DisplayStyle {
    pub normal: ColorHandle,
    pub focused: ColorHandle,
    pub disabled: ColorHandle,
}
/* pub struct NormalStyle {
    normal: ColorHandle,
    focused: ColorHandle,
} */

fn set_display(
    mut query: Query<(&mut ColorHandle, &Status), (With<Display>,)>,
    style: Res<DisplayStyle>,
) {
    query.iter_mut().for_each(|(mut material, st)| {
        *material = {
            match st {
                Status::Focused => style.focused.clone(),
                Status::Disabled => style.disabled.clone(),
                _ => style.normal.clone(),
            }
        }
    });
}
