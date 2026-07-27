use shared::shooting::WeaponKind;

pub const WEAPON_AK_47_MODEL_PATH: &str = "models/weapons/ak47/ak47.glb";
pub const WEAPON_GLOCK_MODEL_PATH: &str = "models/weapons/glock/glock.glb";
pub const WEAPON_P90_MODEL_PATH: &str = "models/weapons/p90/p90.glb";
pub const WEAPON_SNIPER_RIFLE_1_01_PATH: &str =
    "models/weapons/SniperRifle/SniperRifle1_01.glb";

pub fn get_path_to_model_for_weapon_kind(weapon_type: &WeaponKind) -> String {
    match weapon_type {
        WeaponKind::Glock => WEAPON_GLOCK_MODEL_PATH.to_string(),
        WeaponKind::AK47 => WEAPON_AK_47_MODEL_PATH.to_string(),
        WeaponKind::P90 => WEAPON_P90_MODEL_PATH.to_string(),
        WeaponKind::SniperRifle => WEAPON_SNIPER_RIFLE_1_01_PATH.to_string(),
    }
}
