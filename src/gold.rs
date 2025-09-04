use engage::gameuserdata::GameUserData;
use unity::prelude::*;
use engage::calculator::*;

pub fn register_gold(calculator: &mut CalculatorManager) {
    let difficulty: &mut CalculatorCommand = calculator.find_command("幸運");  

    let gold = il2cpp::instantiate_class::<GameCalculatorCommand>(difficulty.get_class().clone()).unwrap();

    gold.get_class_mut().get_virtual_method_mut("get_Name").map(|method| method.method_ptr = get_gold_name as _);
    gold.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_gold as _);
    gold.get_class_mut().get_virtual_method_mut("SetImpl").map(|method| method.method_ptr = set_gold as _);

    gold.get_class_mut().get_vtable_mut()[31].method_ptr = get_gold as *mut u8;
    gold.get_class_mut().get_vtable_mut()[33].method_ptr = set_gold as *mut u8;  
    
    calculator.add_command( gold ); 
}

pub fn get_gold_name(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> &'static Il2CppString { "Gold".into() }

pub fn get_gold(_this: &GameCalculatorCommand, _method_info: OptionalMethod) -> f32 {
    //println!("Get_Gold FN");
    let userdata = GameUserData::get_instance();
    let gold = unsafe { userdata_get_gold(userdata, None) };
    //println!("Gold: {}G", gold);
    gold as f32
}

pub fn set_gold(_this: &GameCalculatorCommand, value: f32, _method_info: OptionalMethod) {
    // println!("Set_Gold FN");
    let userdata = GameUserData::get_instance();
    let gold = unsafe { userdata_get_gold(userdata, None) as f32 };
    let new_gold = ( gold + value ) as i32;
    unsafe {
        maphistory_gold(gold as i32, None)
    };
    unsafe { 
        userdata_set_gold(userdata, new_gold, None) 
    };
    //println!("Gold: {}G", new_gold);
}

#[unity::from_offset("App", "GameUserData", "get_Gold")]
pub fn userdata_get_gold(this: &GameUserData, method_info: OptionalMethod) -> i32;
#[unity::from_offset("App", "GameUserData", "set_Gold")]
pub fn userdata_set_gold(this: &GameUserData, value: i32, method_info: OptionalMethod) -> i32;
#[unity::from_offset("App", "MapHistory", "Gold")]
pub fn maphistory_gold(gold: i32, method_info: OptionalMethod) -> i32;
