use crate::util::{BattleInfoSideTrait, CalculatorCommandTrait, CalculatorManagerTrait};
use engage::{
    battle::BattleInfoSide,
    calculator::{CalculatorManager, GameCalculatorCommand},
    gamedata::{terrain::TerrainData, unit::Unit},
    map::{
        image::{MapImage, MapImageTerrain},
        overlap::MapOverlap,
    },
    util::get_instance,
};
use unity::{prelude::OptionalMethod, system::Il2CppString};

mod command {
    pub const TERRAIN_AVO: &str = "地形回避";
    pub const TERRAIN_DEF: &str = "地形防御";
    pub const TERRAIN_HEAL: &str = "TerrainHeal";
    pub const TERRAIN_MOV: &str = "TerrainMov";
    pub const TERRAIN_IMMUNE_BREAK: &str = "TerrainImmuneBreak";
}

pub fn register_terrain(manager: &mut CalculatorManager) {
    if let Some(terrain_avo) = manager.find_checked(command::TERRAIN_AVO) {
        terrain_avo.assign_virtual_method("GetImpl", get_terrain_avo_command_unit as _);
    }
    if let Some(terrain_def) = manager.find_checked(command::TERRAIN_DEF) {
        terrain_def.assign_virtual_method("GetImpl", get_terrain_def_command_unit as _);
    }
    if let Some(terrain_heal) = manager.clone_from_name(command::TERRAIN_AVO) {
        terrain_heal.assign_virtual_method("get_Name", get_terrain_heal_command_name as _);
        terrain_heal.assign_virtual_method("GetImpl", get_terrain_heal_command_unit as _);
        terrain_heal.assign_vtable(31, get_terrain_heal_command_battle_info as _);
        manager.add_command(terrain_heal);
        if let Some(foe_terrain_heal) = terrain_heal.clone() {
            manager.add_command(foe_terrain_heal.reverse());
        }
    }
    if let Some(terrain_mov) = manager.clone_from_name(command::TERRAIN_AVO) {
        terrain_mov.assign_virtual_method("get_Name", get_terrain_mov_command_name as _);
        terrain_mov.assign_virtual_method("GetImpl", get_terrain_mov_command_unit as _);
        terrain_mov.assign_vtable(31, get_terrain_mov_command_battle_info as _);
        manager.add_command(terrain_mov);
        if let Some(foe_terrain_mov) = terrain_mov.clone() {
            manager.add_command(foe_terrain_mov.reverse());
        }
    }
    if let Some(terrain_immune_break) = manager.clone_from_name(command::TERRAIN_AVO) {
        terrain_immune_break
            .assign_virtual_method("get_Name", get_terrain_immune_break_command_name as _);
        terrain_immune_break
            .assign_virtual_method("GetImpl", get_terrain_immune_break_command_unit as _);
        terrain_immune_break.assign_vtable(31, get_terrain_immune_break_command_battle_info as _);
        manager.add_command(terrain_immune_break);
        if let Some(foe_terrain_immune_break) = terrain_immune_break.clone() {
            manager.add_command(foe_terrain_immune_break.reverse());
        }
    }
}

fn get_terrain_heal_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::TERRAIN_HEAL.into()
}
fn get_terrain_mov_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::TERRAIN_MOV.into()
}

fn get_terrain_immune_break_command_name(
    _this: &GameCalculatorCommand,
    _method: OptionalMethod,
) -> &'static Il2CppString {
    command::TERRAIN_IMMUNE_BREAK.into()
}

fn get_terrain_avo_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let avo = unit.get_terrain_avo();
    avo as f32
}

fn get_terrain_def_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let def = unit.get_terrain_def();
    def as f32
}

fn get_terrain_heal_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let heal = unit.get_terrain_heal();
    heal as f32
}

fn get_terrain_mov_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let mov = unit.get_terrain_mov();
    mov as f32
}

fn get_terrain_immune_break_command_unit(
    _this: &GameCalculatorCommand,
    unit: &Unit,
    _method: OptionalMethod,
) -> f32 {
    let immune_break = unit.is_terrain_immune_to_break();
    if immune_break {
        1f32
    } else {
        0f32
    }
}

fn get_terrain_heal_command_battle_info(
    _this: &GameCalculatorCommand,
    side: &BattleInfoSide,
    _method: OptionalMethod,
) -> f32 {
    let unit = side.get_unit();
    if let Some(unit) = unit {
        get_terrain_heal_command_unit(_this, unit, None)
    } else {
        0f32
    }
}

fn get_terrain_mov_command_battle_info(
    _this: &GameCalculatorCommand,
    side: &BattleInfoSide,
    _method: OptionalMethod,
) -> f32 {
    let unit = side.get_unit();
    if let Some(unit) = unit {
        get_terrain_mov_command_unit(_this, unit, None)
    } else {
        0f32
    }
}

fn get_terrain_immune_break_command_battle_info(
    _this: &GameCalculatorCommand,
    side: &BattleInfoSide,
    _method: OptionalMethod,
) -> f32 {
    let unit = side.get_unit();
    if let Some(unit) = unit {
        get_terrain_immune_break_command_unit(_this, unit, None)
    } else {
        0f32
    }
}

trait TerrainTrait {
    fn is_immune_break(&self) -> bool;
}

impl TerrainTrait for TerrainData {
    fn is_immune_break(&self) -> bool {
        unsafe { terrain_data_is_immune_break(self, None) }
    }
}

trait MapImageTerrainTrait {
    fn get_terrain(&self, x: i32, z: i32) -> Option<&TerrainData>;
}

impl MapImageTerrainTrait for MapImageTerrain {
    fn get_terrain(&self, x: i32, z: i32) -> Option<&TerrainData> {
        unsafe { map_image_terrain_get_data(self, x, z, None) }
    }
}

trait UnitTerrainTrait {
    fn is_enemy(&self) -> bool; // Need this to calculate player_def, enemy_avo, ...
    fn is_terrain_valid(&self, terrain: &TerrainData) -> bool;
    fn get_terrain_avo(&self) -> i32;
    fn get_terrain_def(&self) -> i32;
    fn get_terrain_heal(&self) -> i32;
    fn get_terrain_mov(&self) -> i32;
    fn is_terrain_immune_to_break(&self) -> bool;
}

impl UnitTerrainTrait for Unit {
    fn is_enemy(&self) -> bool {
        const ENEMY: i32 = 1;
        self.force.map_or(0, |f| f.force_type) == ENEMY
    }

    fn is_terrain_valid(&self, terrain: &TerrainData) -> bool {
        !unsafe { unit_is_terrain_invalid(self, terrain, None) }
    }

    fn get_terrain_avo(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| {
            t.avoid
                + if self.is_enemy() {
                    t.enemy_avoid
                } else {
                    t.player_avoid
                }
        }) as i32
    }

    fn get_terrain_def(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| {
            t.defense
                + if self.is_enemy() {
                    t.enemy_defense
                } else {
                    t.player_defense
                }
        }) as i32
    }

    fn get_terrain_heal(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| t.heal) as i32
    }

    fn get_terrain_mov(&self) -> i32 {
        get_terrain_and_overlap_data(self, |t| t.move_first) as i32
    }

    fn is_terrain_immune_to_break(&self) -> bool {
        get_terrain_and_overlap_data(self, |t| t.is_immune_break() as i8) > 0
    }
}

fn get_terrain_and_overlap_data<F>(unit: &Unit, getter: F) -> i8
where
    F: Fn(&TerrainData) -> i8,
{
    let x = unit.get_x();
    let z = unit.get_z();
    let terrain = get_instance::<MapImage>().terrain.get_terrain(x, z);
    let overlap = MapOverlap::get_terrain(x, z);
    get_single_terrain_data(unit, terrain, &getter)
        + get_single_terrain_data(unit, overlap, &getter)
}

fn get_single_terrain_data<F>(unit: &Unit, t: Option<&TerrainData>, getter: F) -> i8
where
    F: Fn(&TerrainData) -> i8,
{
    t.map_or(0, |t| {
        if unit.is_terrain_valid(t) {
            getter(t)
        } else {
            0
        }
    })
}

#[skyline::from_offset(0x02064ED0)]
fn map_image_terrain_get_data(
    this: &MapImageTerrain,
    x: i32,
    z: i32,
    method: OptionalMethod,
) -> Option<&TerrainData>;

#[skyline::from_offset(0x021E3380)]
fn terrain_data_is_immune_break(this: &TerrainData, method: OptionalMethod) -> bool;

#[skyline::from_offset(0x01A34C90)]
fn unit_is_terrain_invalid(this: &Unit, terrain: &TerrainData, method: OptionalMethod) -> bool;
