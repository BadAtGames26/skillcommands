#![feature(ptr_sub_ptr)]
use engage::battle::BattleInfoSide;
use engage::{calculator::*, force};
use engage::unitpool::UnitPool;
use unity::{prelude::*, il2cpp::object::Array};
use engage::gamedata::{Gamedata, unit::*, skill::SkillData};
use engage::{mess::*, force::*};

mod gold;

#[unity::hook("App", "UnitCalculator", "AddCommand")]
fn add_command_hook(calculator: &mut CalculatorManager, method_info: OptionalMethod){
    // GameCalculator is a CalculatorManager
    call_original!(calculator, method_info);

// Example 1: Grabing the movement stat of a unit does not exist, so lets create it by editing the command for luck
    // changing luuk command for move with new GetImpl functions defined in this plugin
    // grab luuk command to replace get_Name/GetImpl with our defined move functions
    let luckc: &mut CalculatorCommand  = calculator.find_command("幸運");   
     // Creating an instance of LuukCommand so we can edit what it does
    let luck = il2cpp::instantiate_class::<GameCalculatorCommand>(luckc.get_class().clone()).unwrap();  

    // replacing get_Name function "Mov" would be used in actvalue/condition as that's what get_move_name returns
    luck.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_move_name as _); // get_move_name() returns "Mov"
    // replacing what the luuk command grabs, get_move returns the move stat of the unit as defined below. This is for the unit version, which is vtable function 30
    luck.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_move_stat_unit as _); 
    // replacing what the luuk command grabs, get_move returns the move stat of the unit as defined below. This for the BattleInfoSide version.
    luck.get_class_mut().get_vtable_mut()[31].method_ptr = get_move_stat_battle_info as *mut u8; 

    // adding our move command (which is an edited luuk command) to the calculator manager
    calculator.add_command( luck ); 

    //Create it again for the reverse. Need to edit another instance of luuk command for the reverse separately.
    let luck2 = il2cpp::instantiate_class::<GameCalculatorCommand>(luckc.get_class().clone()).unwrap();
    luck2.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_move_name as _);
    luck2.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_move_stat_unit as _);
    luck2.get_class_mut().get_vtable_mut()[31].method_ptr = get_move_stat_battle_info as *mut u8;
    // this creates the reverse version so "相手のMov" can be used. Calling reverse automatically attaches 相手の to the name to the new created command
    let reverse_mov = luck2.reverse();  // This gives as a new GameCalculatorCommand 
    // Adding it to the calculator manager
    calculator.add_command( reverse_mov ); 

// Example 2: Rewriting what an already existing command does. 
//  JobRankCommand (兵種ランク) treats special classes as base classes, so let's change it so special classes are treated differently
    // Replacing what job rank command (兵種ランク) so that special classes will return 2 instead 0.
    let job_rank: &mut CalculatorCommand  = calculator.find_command("兵種ランク");
    // replacing the GetImpl with our custom one
    job_rank.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_job_rank as _);
    // no need to add it as it's already in the calculator manager

// Example 3: Triangle Attack condition. This example highlights how you can create a command that does something completely new
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

// Example 4: SID Skill Range Skill: most involved custom skill command that will return the number of units (from a particular force) within a range that has a skil;
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


    // UnitStatus Check
    let status_command = il2cpp::instantiate_class::<CalculatorCommand>(skill.get_class().clone()).unwrap();
    status_command.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_unit_status_name as _);
    status_command.get_class_mut().get_vtable_mut()[34].method_ptr = unit_status_check as *mut u8; 

    let status_command2 = il2cpp::instantiate_class::<GameCalculatorCommand>(skill.get_class().clone()).unwrap();
    status_command2.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_unit_status_name as _);
    status_command2.get_class_mut().get_vtable_mut()[34].method_ptr = unit_status_check as *mut u8; 
    let reserve_status = status_command2.reverse();

    calculator.add_command( status_command );
    calculator.add_command( reserve_status );
}


#[unity::hook("App", "SystemCalculator", ".ctor")]
pub fn gamecalculator_ctor(this: &mut CalculatorManager, method_info: OptionalMethod) {
    // GameCalculator is a CalculatorManager
    call_original!(this, method_info);

    // Get Gold
    gold::register_get_gold(this);

}

//Unit Status Name
pub fn get_unit_status_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString { "UnitStatus".into() }

pub fn unit_status_check(_this: &GameCalculatorCommand, unit: &Unit, args: ListFloats, _method_info: OptionalMethod) -> f32 {
    if args.items.len() == 0 { return 0.0; }
    let status = unit.status.value;
    let flag = args.items[0] as u64;
    //println!("UnitStatus Command: {} & {} is {}", status, flag, status & flag != 0);
    if status & flag != 0 {
        1.0
    }
    else {
        0.0
    }
}

// Mov is what you use in actvalue/condition for this command when replacing it in the vtable 
pub fn get_move_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString {
    "Mov".into()
}

// Replacing GetImpl functions with these
// GetImpl Unit Function, this will probably get called for non-battle timings
pub fn get_move_stat_unit(_this: &GameCalculatorCommand, unit: &Unit, _method_info: OptionalMethod) -> f32 {
    // Move stat is the 10th index
    unit.get_capability(10, true) as f32
}

// GetImpl(BattleInfoSide) This will probably get called during battle timings (2-18)
pub fn get_move_stat_battle_info(_this: &GameCalculatorCommand, side: &BattleInfoSide, _method_info: OptionalMethod) -> f32 {
    // Move stat is the 10th index
    side.detail.capability.data[10] as f32
}

// Changing what JobRankCommand does to handle special classes differently than base classes by checking if class max level is greater than 20
pub fn get_job_rank(_this: &GameCalculatorCommand, unit: &Unit, _method_info: OptionalMethod) -> f32 {
    if unit.get_job().is_low() { 
        if unit.get_job().get_max_level() > 20 {
            2.0 //unpromoted and it's a special class: return 2
        } else {    // unpromoted and not a special class: return 0
            0.0
        }
    } else {    // promoted class returns 1
        1.0
    }
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


#[unity::class("App", "List")]
pub struct ListFloats {
    pub items: &'static Array<f32>,
    pub size: i32,
    pub version: i32,
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

    //println!("SID Range Check with {} args", args.size);
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

    skyline::install_hooks!(add_command_hook, gamecalculator_ctor);
}