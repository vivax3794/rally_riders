use crate::prelude::*;

pub struct PositionPlugin;

impl Plugin for PositionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, position_based_on_relative);
    }
}

#[derive(Clone, Copy)]
pub enum AxisAnchor {
    Neg,
    Center,
    Pos,
}

#[derive(Clone, Copy)]
pub struct RelativeAxis {
    pub anchor: AxisAnchor,
    pub amount: f32,
}

impl RelativeAxis {
    fn get_real_value(self, half_space: f32) -> f32 {
        match self.anchor {
            AxisAnchor::Center => self.amount,
            AxisAnchor::Neg => -half_space + self.amount,
            AxisAnchor::Pos => half_space - self.amount,
        }
    }
}

#[derive(Clone, Copy, Component)]
pub struct Relative {
    pub x: Option<RelativeAxis>,
    pub y: Option<RelativeAxis>,
}

fn position_based_on_relative(
    mut query: Query<(&mut Transform, &Relative)>,
    window: Query<&Window>,
) {
    let Ok(window) = window.get_single() else {
        return;
    };

    let width = window.width();
    let height = window.height();

    for (mut trans, rel) in &mut query {
        if let Some(x) = rel.x {
            trans.translation.x = x.get_real_value(width / 2.0);
        }
        if let Some(y) = rel.y {
            trans.translation.y = y.get_real_value(height / 2.0);
        }
    }
}
