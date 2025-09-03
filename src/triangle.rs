use unity::prelude::*;
use engage::{battle::BattleInfoSide, calculator::*, force::{Force, ForceType}, gamedata::skill::SkillData};

use crate::sidrange::check_has_skill;

pub fn register_triangle(calculator: &mut CalculatorManager) {
    // This requires writing a function to replace either GetImpl(Unit) or GetImpl(BattleInfoSide) see the triangle_attack function below

    // triangle attack - using the pincher command as the base.
    let incher: &mut CalculatorCommand  = calculator.find_command("挟撃中");   // grabbing 挟撃中 command
    // making an instance of 挟撃中 command to edit what it does
    let pincher = il2cpp::instantiate_class::<CalculatorCommand>(incher.get_class().clone()).unwrap();  

    // replacing get_Name so it will return "Triangle" instead of "挟撃中". Triangle will be used in condition/actvalue
    pincher.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_triangle_name as _);
    // replacing GetImpl(BattleInfoSide) so it does our custom check made for triangle attack.
    // this command doesn't use unit version but you can defined it if you want
    pincher.get_class_mut().get_vtable_mut()[31].method_ptr = triangle_attack as *mut u8;
    // adding our edited pincher command, which functions as our triangle command now to the calculator manager

    calculator.add_command(pincher);
}

// Triangle is what is used in condition/actvalue for our triangle attack command
pub fn get_triangle_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString {
    "Triangle".into()
}

#[unity::from_offset("App", "UnitCalculator", "HasForceUnit")]
pub fn unit_calculator_has_force_unit(x: i32, z: i32, force: i32, method_info: OptionalMethod) -> bool;

// function to check if player force units at a position has battle style (Class Type) for triangle attack command
pub fn check_unit_pos_battle_style(x: i32, z: i32, _force: i32, style: &str) -> bool {
    let player_force = Force::get(ForceType::Player).unwrap();

    for unit in player_force.iter() {
        if unit.get_x() == x && unit.get_z() == z {
            if unit.get_job().get_job_style().is_none() { return false; }
            let battle_style = unit.get_job().get_job_style().unwrap().to_string();
            return battle_style == style;
        }
    }

    return false;
}

// function to check if a unit of a given force at position x, z that has skill for SidRange
pub fn check_unit_pos_skill(x: i32, z: i32, force: i32, skill: &SkillData) -> bool {
    let force = match force {
        0 => { Force::get(ForceType::Player) }
        1 => { Force::get(ForceType::Enemy) }
        2 => { Force::get(ForceType::Ally) }
        _ => { Force::get(ForceType::Player) }
    };

    for unit in force.unwrap().iter() {
        if unit.get_x() == x && unit.get_z() == z {
            return check_has_skill(unit, skill);    // return true if unit has skill
        }
    }

    return false;
}
// how we implement the check for triangle attack. The meat of the our custom triangle attack command
pub fn triangle_attack(_this: &GameCalculatorCommand, side: &BattleInfoSide, _method_info: OptionalMethod) -> f32 {
// Since this is a custom command, you could just panic here and let the user know they messed up and how instead of letting it silently fail
    let unit = side.unit.unwrap();
    let target = side.reverse.unit.unwrap();

    let target_x = target.get_x();
    let target_z = target.get_z();

    let dx = side.reverse.x - side.x;
    let dz = side.reverse.z - side.z;

    let mut adjacent_count = 0;

    // ONE RANGE Triangle Attack check only. Can easily adapt it to long range if desired
    if dx*dx + dz*dz == 1 {
        let battle_style = unit.get_job().get_job_style().unwrap().to_string();
        // since targets can be greater than 1x1, need to iterate the entire width of the target
        let bmap_size = target.person.get_bmap_size() as i32;
        let mut side: [bool; 4] = [false; 4];

        // Seaching for all sides of the target to check for allies with the same battle style and adding to the adjacent_count
        // while also counting one side once if the multiple allies are the same side of a 2x2 or bigger target
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
    // triangle attack condition is true if adjacent count is greater/equal to 3, else false
    if adjacent_count >= 3 {
        1.0
    } else {
        0.0
    }
}
