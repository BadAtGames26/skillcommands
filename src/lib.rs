#![feature(lazy_cell, ptr_sub_ptr)]
use engage::battle::BattleInfoSide;
use engage::calculator::*;
use unity::{prelude::*, il2cpp::object::Array};
use engage::gamedata::{Gamedata, unit::*, skill::SkillData};
use engage::{mess::*, force::*};

#[unity::hook("App", "UnitCalculator", "AddCommand")]
fn add_command_hook(calculator: &mut CalculatorManager, method_info: OptionalMethod){
    // GameCalculator is a CalculatorManager
    call_original!(calculator, method_info);

    // changing luuk command for move with new GetImpl functions defined in this plugin
    let luckc: &mut CalculatorCommand  = calculator.find_command("幸運");
    println!("Attempting to make move command from luuk command {}", luckc.klass.get_name());
    let luck = il2cpp::instantiate_class::<GameCalculatorCommand>(luckc.get_class().clone()).unwrap();
    luck.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_move_name as _);
    luck.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_move as _);
    luck.get_class_mut().get_vtable_mut()[31].method_ptr = get_move_battle_info as *mut u8;
    calculator.add_command(&luck.parent);

    //Create it again for the reverse 
    let luck2 = il2cpp::instantiate_class::<GameCalculatorCommand>(luckc.get_class().clone()).unwrap();
    luck2.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_move_name as _);
    luck2.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_move as _);
    luck2.get_class_mut().get_vtable_mut()[31].method_ptr = get_move_battle_info as *mut u8;
    calculator.add_command(&luck2.reverse().parent);

    // Replacing job rank
    let job_rank: &mut CalculatorCommand  = calculator.find_command("兵種ランク");
    job_rank.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_job_rank as _);

    // triangle attack 
    //挟撃
    let incher: &mut CalculatorCommand  = calculator.find_command("挟撃中");
    let pincher = il2cpp::instantiate_class::<CalculatorCommand>(incher.get_class().clone()).unwrap();
    pincher.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_triangle_name as _);
    pincher.get_class_mut().get_vtable_mut()[31].method_ptr = triangle_attack as *mut u8;
    calculator.add_command(pincher);

    // Sid Range Check
    let skill: &mut CalculatorCommand  = calculator.find_command("周囲の隣接男女数");
    let skill_command = il2cpp::instantiate_class::<CalculatorCommand>(skill.get_class().clone()).unwrap();
    skill_command.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_sid_check_name as _);
    skill_command.get_class_mut().get_vtable_mut()[34].method_ptr = sid_range_check as *mut u8; /* 34, 35, 36, 37 */
    calculator.add_command(skill_command);

    //do it again for the reverse 
    let skill_command2 = il2cpp::instantiate_class::<GameCalculatorCommand>(skill.get_class().clone()).unwrap();
    skill_command2.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_sid_check_name as _);
    skill_command2.get_class_mut().get_vtable_mut()[34].method_ptr = sid_range_check as *mut u8; /* 34, 35, 36, 37 */
    calculator.add_command(&skill_command2.reverse().parent);
}

pub fn get_move_name(_this: &GameCalculatorCommand, _unit: &Unit, _method_info: OptionalMethod) -> &'static Il2CppString {
    "Mov".into()
}

pub fn get_move(_this: &GameCalculatorCommand, unit: &Unit, _method_info: OptionalMethod) -> f32 {
    let move_stat = unit.get_capability(10, true);
    println!("move command called with return value {}", move_stat);
    move_stat as f32
}

pub fn get_move_battle_info(_this: &GameCalculatorCommand, side: &BattleInfoSide, _method_info: OptionalMethod) -> f32 {
    // Move stat
    side.detail.capability.data[10] as f32
}
pub fn get_job_rank(_this: &GameCalculatorCommand, unit: &Unit, _method_info: OptionalMethod) -> f32 {
    if unit.get_job().is_low() { 
        if unit.get_job().get_max_level() > 20 {
            2.0
        } else {
            0.0
        }
    } else {
        1.0
    }
}

pub fn get_triangle_name(_this: &GameCalculatorCommand, _unit: &Unit, _method_info: OptionalMethod) -> &'static Il2CppString {
    "Triangle".into()
}

#[unity::from_offset("App", "UnitCalculator", "HasForceUnit")]
pub fn unit_calculator_has_force_unit(x: i32, z: i32, force: i32, method_info: OptionalMethod) -> bool;

pub fn check_unit_pos_battle_style(x: i32, z: i32, _force: i32, style: &str) -> bool {
    let player_force = Force::get(ForceType::Player).unwrap();

    for unit in player_force.iter() {
        if unit.get_x() == x && unit.get_z() == z {
            if unit.get_job().get_job_style().is_none() { return false; }
            let battle_style = unit.get_job().get_job_style().unwrap().get_string().unwrap();
            return battle_style == style;
        }
    }

    return false;
}

pub fn check_unit_pos_skill(x: i32, z: i32, force: i32, skill: &SkillData) -> bool {
    let force = match force {
        0 => { Force::get(ForceType::Player) }
        1 => { Force::get(ForceType::Enemy) }
        2 => { Force::get(ForceType::Ally) }
        _ => { Force::get(ForceType::Player) }
    };

    for unit in force.unwrap().iter() {
        if unit.get_x() == x && unit.get_z() == z {
            return check_has_skill(unit, skill);
        }
    }

    return false;
}

pub fn triangle_attack(_this: &GameCalculatorCommand, side: &BattleInfoSide, _method_info: OptionalMethod) -> f32 {
        // Since this is a custom command, you could just panic here and let the user know they messed up and how instead of letting it silently fail
        let unit = side.unit.unwrap();
        let target = side.reverse.unit.unwrap();

        let target_x = target.get_x();
        let target_z = target.get_z();

        let dx = side.reverse.x - side.x;
        let dz = side.reverse.z - side.z;

        let mut adjacent_count = 0;

        // ONE RANGE
        if dx*dx + dz*dz == 1 {
            let battle_style = unit.get_job().get_job_style().unwrap().get_string().unwrap();
            let bmap_size = target.person.get_bmap_size() as i32;
            let mut side: [bool; 4] = [false; 4];

            for dx_ij in 0..bmap_size {
                //Bottom
                if check_unit_pos_battle_style(target_x + dx_ij, target_z - 1, 0, &battle_style) && !side[0] {
                    side[0] = true;
                    adjacent_count += 1;
                }
                //Top
                if check_unit_pos_battle_style(target_x + dx_ij, target_z + bmap_size, 0, &battle_style) && !side[1] {
                    side[1] = true;
                    adjacent_count += 1;
                }
                //Left
                if check_unit_pos_battle_style(target_x - 1, target_z + dx_ij, 0, &battle_style) && !side[2] {
                    side[2] = true;
                    adjacent_count += 1;
                }
                // Right
                if check_unit_pos_battle_style(target_x + bmap_size, target_z + dx_ij, 0, &battle_style) && !side[3] {
                    side[3] = true;
                    adjacent_count += 1;
                }
            }
        }

        if adjacent_count >= 3 {
            1.0
        } else {
            0.0
        }
}


#[unity::class("App", "List")]
pub struct ListFloats {
    pub items: &'static Array<f32>,
    pub size: i32,
    pub version: i32,
}

pub fn check_has_skill(this: &Unit, skill: &SkillData) -> bool {
    if this.has_skill(skill) || this.has_skill_equip(skill) || this.has_skill_private(skill) {
        true
    } else {
        false
    }
}

pub fn get_sid_check_name(_this: &GameCalculatorCommand, _unit: &Unit, _method_info: OptionalMethod) -> &'static Il2CppString {
    "SidRange".into()
}

pub fn get_skill_name(skill: &SkillData) -> String {
    if let Some(name) = skill.name {
        format!("#{} {} ({})", skill.parent.index, mess_get(name), skill.sid.get_string().unwrap())
    } else {
        format!(" --- #{} ({}) ", skill.parent.index, skill.sid.get_string().unwrap())
    }
}

pub fn mess_get<'a>(value: impl Into<&'a Il2CppString>) -> String {
    Mess::get(value).get_string().unwrap()
}

pub fn sid_range_check(_this: &GameCalculatorCommand, unit: &Unit, args: ListFloats, _method_info: OptionalMethod) -> f32 {
    println!("SID Range Check with {} args", args.size);
    
    if args.size < 2 {
        return 0.0;
    }

    let skill_list = SkillData::get_list().unwrap();

    let skill_index = args.items[1] as i32;

    if skill_index < 0 || skill_index >= skill_list.len() as i32 { 
        return 0.0;
    }

    
    //println!("Skill: {}", get_skill_name(skill));
    let range = args.items[0] as i32;
    let skill = &skill_list[skill_index as usize]; 
    
    if range == 0 {
        //println!("Range == 0, returning: {}", unit_has_skill(unit, skill));
        if check_has_skill(unit, skill) {
            return 1.0;
        }
        else {
            return 0.0;
        }
    }

    let x_pos =  unit.get_x();
    let z_pos = unit.get_z();

    let mut count: [i32; 3] = [0; 3];

    for x in -range..range {
        let x_check = x + x_pos;

        for z in -range..range {
            let r2 = range * range;
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

    let force = if args.size >= 3 {
        args.items[2] as i32
    } else {
        unit.force.unwrap().force_type
    };

    println!("Force {}: {} {} {}", force, count[0], count[1], count[2]);

    match force {
        -1 => {
            let mut return_value = 0;
            let unit_force_type = unit.force.unwrap().force_type;

            for f in 0..3 {
                if f == unit_force_type {
                    continue;
                }

                return_value += count[f as usize];
            }
            
            return_value as f32
        }
        idx @ (0 | 1 | 2) => count[idx as usize] as f32,
        3 => {
            let mut return_value = 0;

            for f in 0..3 {
                return_value += count[f as usize];
            }
            
            return_value as f32
        }
        _ => 0.0
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