use unity::prelude::*;
use engage::{calculator::*, gamedata::{skill::SkillData, unit::Unit, Gamedata}, mess::Mess, unitpool::UnitPool};

use crate::{triangle::check_unit_pos_skill, ListFloats};

pub fn register_sid_range(calculator: &mut CalculatorManager) {
    // Sid Range Check - custom command that takes in the skill index, range, and force, by editing "周囲の隣接男女数"
    let skill: &mut CalculatorCommand  = calculator.find_command("周囲の隣接男女数");   // grabing 周囲の隣接男女数 command
    // creating an instance of 周囲の隣接男女数 command
    let skill_command = il2cpp::instantiate_class::<CalculatorCommand>(skill.get_class().clone()).unwrap();
    // replacing the name of "周囲の隣接男女数" with "SidRange"
    skill_command.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_sid_check_name as _);
    // vtable 34 function is the FuncImpl which does the whole ""周囲の隣接男女数(number, number, number)" thing
    skill_command.get_class_mut().get_vtable_mut()[34].method_ptr = sid_range_check as *mut u8; 

    // adding our edited "周囲の隣接男女数" command that does our skill check.
    calculator.add_command(skill_command);

    //do it again for the reverse 
    let skill_command2 = il2cpp::instantiate_class::<GameCalculatorCommand>(skill.get_class().clone()).unwrap();
    skill_command2.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_sid_check_name as _);
    // vtable 34 function is the FuncImpl which does the whole "command(number, number, number)" thing
    skill_command2.get_class_mut().get_vtable_mut()[34].method_ptr = sid_range_check as *mut u8; 
    let reverse_skill_check = skill_command2.reverse();

    // adding our new "相手のSIDRange" command
    calculator.add_command( reverse_skill_check); 


}

// used to check if unit has skill 
pub fn check_has_skill(this: &Unit, skill: &SkillData) -> bool {
    this.has_skill(skill) || this.has_skill_equip(skill) || this.has_skill_private(skill)
}
// name of Skill range check that will be used in condition/actvalues
pub fn get_sid_check_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString {
    "SidRange".into()
}
// for debugging purpose to print the skill name 
pub fn get_skill_name(skill: &SkillData) -> String {
    if let Some(name) = skill.name {
        format!("#{} {} ({})", skill.parent.index, mess_get(name), skill.sid.to_string())
    } else {
        format!(" --- #{} ({}) ", skill.parent.index, skill.sid.to_string())
    }
}

pub fn mess_get<'a>(value: impl Into<&'a Il2CppString>) -> String {
    Mess::get(value).to_string()
}

pub fn sid_range_check(_this: &GameCalculatorCommand, unit: &Unit, args: ListFloats, _method_info: OptionalMethod) -> f32 {
    // List arguments
    // 1st argument - range of skill check (if 0, check for unit for skill instead)
    // 2st argument - Skill Index, can use スキル("SID_XXX")
    // 3rd argument - which force of units to check 
    //              -1: all force but unit's current force, 
    //               0: player force only
    //               1: enemy force only
    //               2: ally force only
    //               3: player + enemy + ally force
    // returns number of units that has skill index at a given range and of a given force
    // SidRange( 1, スキル("SID_平和の花効果"), 1) will check for enemy (force 1) units at 1 range with skill SID_平和の花効果 

    println!("SID Range Check with {} args", args.size);
    // if only 1 argument is given, return false
    if args.size < 2 {
        return 0.0;
    }

    let skill_list = SkillData::get_list().unwrap();

    let skill_index = args.items[1] as i32;
    // if not valid skill index return false
    if skill_index < 0 || skill_index >= skill_list.len() as i32 { 
        return 0.0;
    }

    let range = args.items[0] as i32;
    let skill = &skill_list[skill_index as usize]; 
    
    // keeping track of the count of units in each force that has the skill
    let mut force_unit_count: [i32; 3] = [0; 3];

    if range == 0 { // Self
        if check_has_skill(unit, skill) {
            return 1.0;
        }
        else {
            return 0.0;
        }
    }
    else if range == 99 {    // all units, includes unit
        for x in 1..250 {
            if let Some(unit) = UnitPool::get_by_index(x).filter(|unit| unit.force.is_some_and(|f| ( 1 << f.force_type) & 7 != 0)) {
                if check_has_skill(unit, skill) { force_unit_count[ unit.force.unwrap().force_type as usize] += 1;}
            }
        }
    }
    else {
        let x_pos = unit.get_x();
        let z_pos = unit.get_z();
        for x in -range..range+1 {
            let x_check = x + x_pos;    // x position to check for unit
            for z in -range..range+1 {
                let r2 = range * range;
                let dr2 = x*x + z*z;
                let z_check = z + z_pos;   // z position to check for unit

                if dr2 > r2 { continue; }   // if out of range

                for f in 0..3 {
                    if check_unit_pos_skill(x_check, z_check, f as i32, skill) {
                        // add to the force's unit counter 
                        force_unit_count[ f as usize] += 1;
                    }
                }
            }
        }
    }
    // getting which force to check.
    // if no force argument is given, then use unit's current force
    let force = if args.size >= 3 {
        args.items[2] as i32
    } else {
        unit.force.unwrap().force_type
    };


    match force {
        -1 => { // all forces but the unit's current force
            let mut return_value = 0;
            let unit_force_type = unit.force.unwrap().force_type;

            for f in 0..3 {
                if f == unit_force_type {   // ignore the unit's force
                    continue;
                }
                return_value += force_unit_count[f as usize];
            }
            
            return_value as f32
        }
        // 0, 1, and 2, return unit count of that force (player or enemy or ally)
        idx @ (0 | 1 | 2) => force_unit_count[idx as usize] as f32,
        3 => { // 3 - return unit count of all forces (player + enemy + ally)
            let mut return_value = 0;

            for f in 0..3 {
                return_value += force_unit_count[f as usize];
            }
            
            return_value as f32
        }
        _ => 0.0
    }
}
