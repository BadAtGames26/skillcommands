use unity::prelude::*;
use engage::{calculator::*, gamedata::unit::Unit};

use crate::ListFloats;

pub fn register_unit_status(calculator: &mut CalculatorManager) {
    let skill: &mut CalculatorCommand  = calculator.find_command("周囲の隣接男女数");   // grabing 周囲の隣接男女数 command
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

//Unit Status Name
pub fn get_unit_status_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString { "UnitStatus".into() }

pub fn unit_status_check(_this: &GameCalculatorCommand, unit: &Unit, args: ListFloats, _method_info: OptionalMethod) -> f32 {
    if args.items.len() == 0 { return 0.0; }
    let status = unit.status.value;
    let flag = args.items[0] as u64;
    println!("UnitStatus Command: {} & {} is {}", status, flag, status & flag != 0);
    if status & flag != 0 {
        1.0
    }
    else {
        0.0
    }
}