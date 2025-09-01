use engage::gameuserdata::GameUserData;
use unity::prelude::*;
use engage::calculator::*;

pub fn get_gold(_this: &CalculatorCommand, _method_info: OptionalMethod) -> f32 {
    println!("Get_Gold FN");
    let userdata = GameUserData::get_instance();
    let gold = unsafe { userdata_get_gold(userdata, None) };
    println!("Gold: {}G", gold);
    gold as f32
}

pub fn get_gold_name(_this: &CalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString { "CurrentGold".into() }

pub fn register_get_gold(calculator: &mut CalculatorManager) {
    println!("Trying to register gold command");
    let difficulty: &mut CalculatorCommand  = calculator.find_command("難易度");  
    println!("Got difficulty command");
    let gold = il2cpp::instantiate_class::<CalculatorCommand>(difficulty.get_class().clone()).unwrap();
    println!("Made gold command");
    gold.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_gold_name as _);
    gold.get_class_mut().get_virtual_method_mut("Fer").map(|method| method.method_ptr = get_gold as _);
    println!("Set get and name function");
    gold.get_class_mut().get_vtable_mut()[9].method_ptr = get_gold as *mut u8; 
     println!("Trying to add gold command");
    calculator.add_command( gold ); 
}

#[unity::from_offset("App", "GameUserData", "get_Gold")]
pub fn userdata_get_gold(this: &GameUserData, method_info: OptionalMethod) -> i32;

