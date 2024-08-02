#![feature(lazy_cell, ptr_sub_ptr)]
use unity::{prelude::*, il2cpp::object::Array};
use engage::gamedata::{*, item::UnitItem, unit::*, job::*, skill::SkillData};
use engage::{mess::*, force::*};

#[unity::class("App", "CapabilityInt")]
pub struct CapabilityInt {
    pub data: &'static mut Array<i32>,
}
#[unity::class("App", "BattleDetail")]
pub struct BattleDetail {
    pub capability: &'static mut CapabilityInt,
}
#[unity::class("App", "TerrainData")]
pub struct TerrainData {}

#[unity::class("App", "BattleInfoSide")]
pub struct BattleInfoSide {
    //junk : [u8; 0x48],
    info: u64,
    pub side_type : i32,
    __ : i32,
    pub unit: Option<&'static Unit>,
    pub unit_item: &'static UnitItem,
    pub specified_item: &'static UnitItem,
    pub x: i32,
    pub z: i32,
    pub terrain: &'static TerrainData,
    pub overlap: &'static TerrainData,
    pub status: &'static WeaponMask,
    pub detail: &'static BattleDetail,
    hierarchy: u64,
    support: u64,
    pub parent: &'static BattleInfoSide,
    pub reverse: &'static BattleInfoSide,
}

#[unity::class("App", "GameCalculatorCommand")]
pub struct GameCalculatorCommand {}

#[unity::class("App", "CalculatorCommand")]
pub struct CalculatorCommand {}

#[unity::class("App", "CalculatorManager")]
pub struct CalculatorManager {}

#[skyline::from_offset(0x0298d900)]
pub fn calculator_manager_add_command(this: &CalculatorManager, command: &CalculatorCommand, method_info: OptionalMethod) -> &'static CalculatorCommand;

#[skyline::from_offset(0x0298daa0)]
pub fn find_command(this: &CalculatorManager, name: &Il2CppString, method_info: OptionalMethod) -> &'static mut CalculatorCommand;

#[unity::from_offset("App", "GameCalculatorCommand", "Reverse")]
pub fn game_calculator_command_reverse(this: &CalculatorCommand, method_info: OptionalMethod) -> &'static mut CalculatorCommand;

#[unity::hook("App", "UnitCalculator", "AddCommand")]
fn add_command_hook(calculator: &CalculatorManager, method_info: OptionalMethod){
    // GameCalculator is a CalculatorManager
    call_original!(calculator, method_info);
    unsafe {
        // changing luuk command for move with new GetImpl functions defined in this plugin
        let luckc: &mut CalculatorCommand  = find_command(calculator, "幸運".into(), None);
        println!("Attempting to make move command from luuk command {}", luckc.klass.get_name());
        let luck = il2cpp::instantiate_class::<CalculatorCommand>(luckc.get_class().clone()).unwrap();
        luck.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_move_name as _);
        luck.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_move as _);
        let vtable = luck.get_class_mut().get_vtable_mut();
        let ptr = get_move_battle_info as *mut u8;
        vtable[31].method_ptr = std::mem::transmute(ptr);
        calculator_manager_add_command(calculator, luck, None);

        //Create it again for the reverse 
        let luck2 = il2cpp::instantiate_class::<CalculatorCommand>(luckc.get_class().clone()).unwrap();
        luck2.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_move_name as _);
        luck2.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_move as _);
        let vtable21 = luck2.get_class_mut().get_vtable_mut();
        let ptr21 = get_move_battle_info as *mut u8;
        vtable21[31].method_ptr = std::mem::transmute(ptr);
        calculator_manager_add_command(calculator, game_calculator_command_reverse(luck2,None), None);

        // Replacing job rank
        let job_rank: &mut CalculatorCommand  = find_command(calculator, "兵種ランク".into(), None);
        job_rank.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_job_rank as _);

        // triangle attack 
        //挟撃
        let incher: &mut CalculatorCommand  = find_command(calculator, "挟撃中".into(), None);
        let pincher = il2cpp::instantiate_class::<CalculatorCommand>(incher.get_class().clone()).unwrap();
        pincher.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_triangle_name as _);
        let vtable2 = pincher.get_class_mut().get_vtable_mut();
        let ptr2 = triangle_attack as *mut u8;
        vtable2[31].method_ptr = std::mem::transmute(ptr2); 
        calculator_manager_add_command(calculator, pincher, None);

        // Sid Range Check
        let skill: &mut CalculatorCommand  = find_command(calculator, "周囲の隣接男女数".into(), None);
        let skill_command = il2cpp::instantiate_class::<CalculatorCommand>(skill.get_class().clone()).unwrap();
        skill_command.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_sid_check_name as _);
        let vtable3 = skill_command.get_class_mut().get_vtable_mut();
        let ptr3 = sid_range_check as *mut u8; /* 34, 35, 36, 37 */
        vtable3[34].method_ptr = std::mem::transmute(ptr3);
        calculator_manager_add_command(calculator, skill_command, None);

        //do it again for the reverse 
        let skill_command2 = il2cpp::instantiate_class::<CalculatorCommand>(skill.get_class().clone()).unwrap();
        skill_command2.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_sid_check_name as _);
        let vtable31 = skill_command2.get_class_mut().get_vtable_mut();
        let ptr31 = sid_range_check as *mut u8; /* 34, 35, 36, 37 */
        vtable31[34].method_ptr = std::mem::transmute(ptr31);
        calculator_manager_add_command(calculator, game_calculator_command_reverse(skill_command2,None), None);
    }
}

pub fn get_move_name(this: &GameCalculatorCommand, unit: &Unit, method_info: OptionalMethod) -> &'static Il2CppString { return "Mov".into(); }

pub fn get_move(this: &GameCalculatorCommand, unit: &Unit, method_info: OptionalMethod) -> f32 {
    let move_stat = unit.get_capability(10, true);
    println!("move command called with return value {}", move_stat);
    return move_stat as f32;
}

pub fn get_move_battle_info(this: &GameCalculatorCommand, side: &BattleInfoSide, method_info: OptionalMethod) -> f32 {
    let move_stat =  side.detail.capability.data[10];
    return move_stat as f32;
}
pub fn get_job_rank(this: &GameCalculatorCommand, unit: &Unit, method_info: OptionalMethod) -> f32 {
    if unit.get_job().is_low(){ 
        if unit.get_job().get_max_level() > 20 { return 2.0; }
        else { return 0.0; }
    }
    return  1.0; 
}

pub fn get_triangle_name(this: &GameCalculatorCommand, unit: &Unit, method_info: OptionalMethod) -> &'static Il2CppString { return "Triangle".into(); }

#[unity::from_offset("App", "UnitCalculator", "HasForceUnit")]
pub fn unit_calculator_has_force_unit(x: i32, z: i32, force: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "Unit", "get_X")]
pub fn unit_get_x(this: &Unit, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "Unit", "get_Z")]
pub fn unit_get_z(this: &Unit, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x01f25ec0)]
fn get_bmap_size(this: &PersonData, method_info: OptionalMethod) -> u8;

pub fn check_unit_pos_battle_style(x: i32, z: i32, force: i32, style: &str) -> bool {
    let force_iter = Force::iter(Force::get(ForceType::Player).unwrap());
    for unit in force_iter {
        unsafe {
            if unit_get_x(unit, None) == x && unit_get_z(unit, None) == z {
                if unit.get_job().get_job_style().is_none() { return false; }
                let battle_style = unit.get_job().get_job_style().unwrap().get_string().unwrap();
                return (battle_style == style);
            }
        }
    }
    return false;
}

pub fn check_unit_pos_skill(x: i32, z: i32, force: i32, skill: &SkillData) -> bool {
    let force_iter; 
    match force {
        0 => { force_iter = Force::iter(Force::get(ForceType::Player).unwrap()); }
        1 => { force_iter = Force::iter(Force::get(ForceType::Enemy).unwrap()); }
        2 => { force_iter = Force::iter(Force::get(ForceType::Ally).unwrap()); }
        _ => { force_iter = Force::iter(Force::get(ForceType::Player).unwrap()); }
    }
    for unit in force_iter {
        unsafe {
            if unit_get_x(unit, None) == x && unit_get_z(unit, None) == z {
                return unit_has_skill(unit, skill);
            }
        }
    }
    return false;
}

pub fn triangle_attack(this: &GameCalculatorCommand, side: &BattleInfoSide, method_info: OptionalMethod) -> f32 {
    unsafe { 
        if side.unit.is_none() { return 0.0; }
        let unit = side.unit.unwrap();
        if side.reverse.unit.is_none() { return 0.0; }
        let unit2 = side.reverse.unit.unwrap();
        let force = unit.force.unwrap().force_type; 
        let target_x = unit_get_x(unit2, None);
        let target_z = unit_get_z(unit2, None);
        let mut adjacent_count = 0;
        let bmap_size = get_bmap_size(unit2.person, None) as i32; 
        let battle_style = unit.get_job().get_job_style().unwrap().get_string().unwrap();
        let dx = side.reverse.x - side.x;
        let dz = side.reverse.z - side.z;
        let mut side: [bool; 4] = [false; 4];
        // ONE RANGE
        if dx*dx + dz*dz == 1 {
            for dx_ij in 0..bmap_size {
                //Bottom
                if check_unit_pos_battle_style(target_x + dx_ij, target_z - 1, 0, &battle_style) && !side[0] {
                    side[0] = true; adjacent_count += 1;
                }
                //Top
                if check_unit_pos_battle_style(target_x + dx_ij, target_z + bmap_size, 0, &battle_style) && !side[1] {
                    side[1] = true; adjacent_count += 1;
                }
                //Left
                if check_unit_pos_battle_style(target_x - 1, target_z + dx_ij, 0, &battle_style) && !side[2] {
                    side[2] = true; adjacent_count += 1;
                }
                // Right
                if check_unit_pos_battle_style(target_x + bmap_size, target_z + dx_ij, 0, &battle_style) && !side[3] {
                    side[3] = true; adjacent_count += 1;
                }
            }
        }
        if adjacent_count >= 3 { return 1.0; }
        else {return 0.0; }
    }
}
#[skyline::main(name = "skillcmd")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };


        let err_msg = format!(
            "SkillCommand plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            42069,
            "SkillCommand plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));

    skyline::install_hooks!(add_command_hook);
}

#[unity::class("App", "List")]
pub struct ListFloats {
    pub items: &'static Array<f32>,
    pub size: i32,
    pub version: i32,
}
#[skyline::from_offset(0x01a35520)]
pub fn unit_has_skill_mask(this: &Unit, skill: &SkillData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a35ec0)]
pub fn unit_has_skill_equip(this: &Unit, skill: &SkillData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01a37970)]
pub fn unit_has_skill_private(this: &Unit, skill: &SkillData, method_info: OptionalMethod) -> bool;

pub fn unit_has_skill(this: &Unit, skill: &SkillData) -> bool {
    unsafe {
        if unit_has_skill_mask(this, skill, None) { return true; }
        if unit_has_skill_equip(this, skill, None) { return true; }
        if unit_has_skill_private(this, skill, None) { return true; }
    }
    return false; 
}
pub fn get_sid_check_name(this: &GameCalculatorCommand, unit: &Unit, method_info: OptionalMethod) -> &'static Il2CppString { return "SidRange".into(); }

pub fn get_skill_name(skill: &SkillData) -> String {
    if skill.name.is_some() { return format!("#{} {} ({})", skill.parent.index, mess_get(skill.name.unwrap()), skill.sid.get_string().unwrap()); }
    else {  return format!(" --- #{} ({}) ", skill.parent.index, skill.sid.get_string().unwrap()); }
}
pub fn mess_get(value: &Il2CppString) -> String { return Mess::get(value).get_string().unwrap(); }
pub fn sid_range_check(this: &GameCalculatorCommand, unit: &Unit, args: ListFloats, method_info: OptionalMethod) -> f32 {
    println!("SID Range Check with {} args", args.size);
    unsafe {
        if args.size < 2 { return 0.0; }
        let skill_list = SkillData::get_list().unwrap();
        let skill_index = args.items[1] as i32;
        if skill_index < 0 || skill_index >= skill_list.len() as i32 { 
            return 0.0;
        }
        let range = args.items[0] as i32;
        let x_pos =  unit_get_x(unit, None);
        let z_pos = unit_get_z(unit, None);
        let skill = &skill_list[skill_index as usize]; 
        //println!("Skill: {}", get_skill_name(skill));
        let r2 = range*range;
        //println!("Range: {}", range);
        let mut count: [i32; 3] = [0; 3];
        let force: i32;
        if args.size >= 3 { force =  args.items[2] as i32;}
        else { force = unit.force.unwrap().force_type; }
        //println!("Force: {}", force);
        if range == 0 {
            //println!("Range == 0, returning: {}", unit_has_skill(unit, skill));
            if unit_has_skill(unit, skill) { return 1.0; }
            else { return 0.0;}
        }
        for x in -range..range {
            let x_check = x + x_pos;
            for z in -range..range {
                let dr2 = x*x + z*z;
                let z_check = z + z_pos;
                if dr2 <= r2 {
                    for f in 0..3 {
                        if check_unit_pos_skill(x_check, z_check, f as i32, skill) {
                            count[ f as usize] += 1;
                        }
                    }
                }
            }
        }
        println!("Force {}: {} {} {}", force, count[0], count[1], count[2]);
        if force == 0 { return count[0] as f32; }
        else if force == 1 { return count[1] as f32; }
        else if force == 2 { return count[2] as f32; }
        else if force == -1 {
            let mut return_value = 0;
            for f in 0..3 {
                if f as i32 == unit.force.unwrap().force_type { continue; }
                return_value += count[f as usize];
            }
            return return_value as f32;
        }
        else if force == 3 {
            let mut return_value = 0;
            for f in 0..3 { return_value += count[f as usize]; }
            return return_value as f32;
        }
        return 0.0;
    }
}