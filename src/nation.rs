use unity::prelude::*;
use engage::{battle::BattleInfoSide, calculator::*, gamedata::unit::Unit};

pub fn register_nation(calculator: &mut CalculatorManager) {
    // grab gender command to replace get_Name/GetImpl with our own defined nation functions
    let gender_command: &mut CalculatorCommand = calculator.find_command("性別");
    // Creating an instance of GenderCommand so we can edit what it does
    let nation = il2cpp::instantiate_class::<GameCalculatorCommand>(gender_command.get_class().clone()).unwrap();

    nation.get_class_mut().get_virtual_method_mut("get_Name").map(|method | method.method_ptr = get_nation_name as _);
    nation.get_class_mut().get_virtual_method_mut("GetImpl").map(|method | method.method_ptr = get_nation as _);
    nation.get_class_mut().get_vtable_mut()[31].method_ptr = get_nation_battle_info as *mut u8;

    calculator.add_command( nation );

    // reverse
    let nation2 = il2cpp::instantiate_class::<GameCalculatorCommand>(gender_command.get_class().clone()).unwrap();
    nation2.get_class_mut().get_virtual_method_mut("get_Name").map(|method | method.method_ptr = get_nation_name as _);
    nation2.get_class_mut().get_virtual_method_mut("GetImpl").map(|method | method.method_ptr = get_nation as _);
    nation2.get_class_mut().get_vtable_mut()[31].method_ptr = get_nation_battle_info as *mut u8;
    let reverse_nation = nation2.reverse();
    calculator.add_command( reverse_nation );
}

pub fn get_nation_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString {
    "Nation".into()
}

pub fn get_nation(_this: &GameCalculatorCommand, unit: &Unit, _method_info: OptionalMethod) -> f32 {
    unit.person.hometown as f32
}

pub fn get_nation_battle_info(_this: &GameCalculatorCommand, side: &BattleInfoSide, _method_info: OptionalMethod) -> f32 {
    match side.unit {
        Some(unit) => unit.person.hometown as f32,
        None => -1.0,
    }
}