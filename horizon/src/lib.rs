//! Better Mount HUD for Horizon: keeps the player's food bar visible while riding a mount
//! and shifts mount health hearts and air bubbles above it without overlapping.

use std::borrow::Cow;

use mod_api::client::hud_layout::{Blend, HeartKind};
use mod_api::{self, ClientHost, GuiDraw, HealthHudEvent, HealthInputs, Hooks};

const WHITE: u32 = 0xFFFF_FFFF;

fn sprite(name: &'static str, x: i32, y: i32) -> GuiDraw {
    GuiDraw::Sprite {
        sprite: Cow::Borrowed(name),
        x,
        y,
        w: 9,
        h: 9,
        region: None,
        color: WHITE,
        blend: Blend::Textured,
    }
}

fn heart_sprites(kind: HeartKind, hardcore: bool, absorbing: bool) -> (&'static str, &'static str) {
    if absorbing {
        return if hardcore {
            ("hud/heart/absorbing_hardcore_full", "hud/heart/absorbing_hardcore_half")
        } else {
            ("hud/heart/absorbing_full", "hud/heart/absorbing_half")
        };
    }
    match (kind, hardcore) {
        (HeartKind::Poisoned, true) => ("hud/heart/poisoned_hardcore_full", "hud/heart/poisoned_hardcore_half"),
        (HeartKind::Poisoned, false) => ("hud/heart/poisoned_full", "hud/heart/poisoned_half"),
        (HeartKind::Withered, true) => ("hud/heart/withered_hardcore_full", "hud/heart/withered_hardcore_half"),
        (HeartKind::Withered, false) => ("hud/heart/withered_full", "hud/heart/withered_half"),
        (HeartKind::Frozen, true) => ("hud/heart/frozen_hardcore_full", "hud/heart/frozen_hardcore_half"),
        (HeartKind::Frozen, false) => ("hud/heart/frozen_full", "hud/heart/frozen_half"),
        (HeartKind::Normal, true) => ("hud/heart/hardcore_full", "hud/heart/hardcore_half"),
        (HeartKind::Normal, false) => ("hud/heart/full", "hud/heart/half"),
    }
}

/// Builds the full status-bar draw list when mounted, keeping food visible and shifting
/// mount hearts and air bubbles upward so nothing overlaps.
fn build_mounted_hud(inputs: &HealthInputs, wi: i32, hi: i32) -> Vec<GuiDraw> {
    let mut draws = Vec::new();
    let left_x = wi / 2 - 91;
    let right_x = wi / 2 + 91;
    let base_y = hi - 39;

    // 1. Player health + absorption
    let health = (inputs.health.ceil() as i32).max(0);
    let max_health = (inputs.max_health.ceil() as i32).max(0);
    let absorption = (inputs.absorption.ceil() as i32).max(0);
    let total_hearts = (max_health + absorption + 1) / 2;
    let max_health_hearts = (max_health + 1) / 2;
    let health_rows = ((total_hearts + 9) / 10).max(1);
    let row_height = (10 - (health_rows - 2)).max(3);
    let container = if inputs.hardcore {
        "hud/heart/container_hardcore"
    } else {
        "hud/heart/container"
    };

    for idx in (0..total_hearts).rev() {
        let row = idx / 10;
        let col = idx % 10;
        let x = left_x + col * 8;
        let y = base_y - row * row_height;
        draws.push(sprite(container, x, y));

        if idx >= max_health_hearts {
            let abs_half = idx * 2 - max_health_hearts * 2;
            let (full, half) = heart_sprites(inputs.kind, inputs.hardcore, true);
            if abs_half + 1 < absorption {
                draws.push(sprite(full, x, y));
            } else if abs_half + 1 == absorption {
                draws.push(sprite(half, x, y));
            }
        } else {
            let (full, half) = heart_sprites(inputs.kind, inputs.hardcore, false);
            if idx * 2 + 1 < health {
                draws.push(sprite(full, x, y));
            } else if idx * 2 + 1 == health {
                draws.push(sprite(half, x, y));
            }
        }
    }

    // 2. Player armor
    if inputs.armor > 0 {
        let armor_y = base_y - (health_rows - 1) * row_height - 10;
        for i in 0..10 {
            let x = left_x + i * 8;
            let tex = if i * 2 + 1 < inputs.armor {
                "hud/armor_full"
            } else if i * 2 + 1 == inputs.armor {
                "hud/armor_half"
            } else {
                "hud/armor_empty"
            };
            draws.push(sprite(tex, x, armor_y));
        }
    }

    // 3. Player food bar (kept visible while riding a mount)
    let (food_empty, food_full, food_half) = if inputs.hunger {
        ("hud/food_empty_hunger", "hud/food_full_hunger", "hud/food_half_hunger")
    } else {
        ("hud/food_empty", "hud/food_full", "hud/food_half")
    };
    for i in 0..10 {
        let x = right_x - i * 8 - 9;
        draws.push(sprite(food_empty, x, base_y));
        if i * 2 + 1 < inputs.food {
            draws.push(sprite(food_full, x, base_y));
        } else if i * 2 + 1 == inputs.food {
            draws.push(sprite(food_half, x, base_y));
        }
    }

    // 4. Mount health hearts shifted above the food bar
    let mount_base_y = base_y - 10;
    let total_mount_hearts = inputs.vehicle_hearts.max(0);
    let mount_rows = (total_mount_hearts + 9) / 10;
    for idx in 0..total_mount_hearts {
        let row = idx / 10;
        let col = idx % 10;
        let x = right_x - col * 8 - 9;
        let y = mount_base_y - row * 10;
        draws.push(sprite("hud/heart/vehicle_container", x, y));
        draws.push(sprite("hud/heart/vehicle_full", x, y));
    }

    // 5. Air bubbles shifted above mount health
    if inputs.under_water || inputs.air < inputs.max_air {
        let air_y = mount_base_y - mount_rows * 10;
        let max_air = inputs.max_air.max(1);
        let full_bubbles = ((inputs.air.max(0) * 10) + max_air - 1) / max_air;
        let partial = (inputs.air.max(0) * 10) / max_air;
        for i in 0..10 {
            let x = right_x - i * 8 - 9;
            if i < partial {
                draws.push(sprite("hud/air", x, air_y));
            } else if i < full_bubbles {
                draws.push(sprite("hud/air_bursting", x, air_y));
            } else {
                draws.push(sprite("hud/air_empty", x, air_y));
            }
        }
    }

    draws
}

#[derive(Default)]
struct BetterMountHudMod;

#[mod_api::horizon_mod]
impl Hooks for BetterMountHudMod {
    fn health_hud(&self, _host: &mut dyn ClientHost, event: &mut HealthHudEvent<'_>) -> Option<Vec<GuiDraw>> {
        if event.inputs.vehicle_hearts <= 0 {
            return None;
        }
        Some(build_mounted_hud(event.inputs, event.wi, event.hi))
    }
}

#[cfg(test)]
mod tests {
    use super::build_mounted_hud;
    use mod_api::client::hud_layout::HeartKind;
    use mod_api::{GuiDraw, HealthInputs};

    #[test]
    fn renders_food_and_shifts_mount_hearts_above_food() {
        let inputs = HealthInputs {
            tick: 0,
            millis: 0,
            health: 20.0,
            max_health: 20.0,
            absorption: 0.0,
            armor: 0,
            food: 20,
            saturation: 5.0,
            kind: HeartKind::Normal,
            regeneration: false,
            hunger: false,
            hardcore: false,
            damage_cooldown: false,
            air: 300,
            max_air: 300,
            under_water: false,
            vehicle_hearts: 10,
        };
        let draws = build_mounted_hud(&inputs, 427, 240);
        let has_food = draws.iter().any(|d| matches!(d, GuiDraw::Sprite { sprite, y, .. } if sprite == "hud/food_full" && *y == 201));
        let has_mount_shifted = draws.iter().any(|d| matches!(d, GuiDraw::Sprite { sprite, y, .. } if sprite == "hud/heart/vehicle_full" && *y == 191));
        assert!(has_food);
        assert!(has_mount_shifted);
    }
}
