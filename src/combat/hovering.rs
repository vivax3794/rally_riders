use crate::prelude::*;

pub struct HoveringPlugin;

#[derive(Component)]
pub struct Hoverable {
    pub size: Vec2,
}

#[derive(Component)]
pub struct Hovered;

impl Plugin for HoveringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_hover_state);
    }
}

fn update_hover_state(
    mut commands: Commands,
    targets: Query<(Entity, &Hoverable, &Transform)>,
    window: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };
    let Ok((camera, camera_trans)) = camera.get_single() else {
        return;
    };

    let Some(cursor) = window.cursor_position() else {
        return;
    };
    let Some(cursor) = camera.viewport_to_world_2d(camera_trans, cursor) else {
        return;
    };

    for (target, info, trans) in &targets {
        let bounds = info.size * trans.scale.truncate() / 2.0;
        let rel_mouse = cursor - trans.translation.truncate();

        let hovered = rel_mouse.x.abs() <= bounds.x && rel_mouse.y.abs() <= bounds.y;
        if hovered {
            commands.entity(target).insert(Hovered);
        } else {
            commands.entity(target).remove::<Hovered>();
        }
    }
}
