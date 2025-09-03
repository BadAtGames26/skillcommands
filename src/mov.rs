use unity::prelude::*;
use engage::{battle::BattleInfoSide, calculator::*, gamedata::unit::Unit};

pub fn register_move(calculator: &mut CalculatorManager) {
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
