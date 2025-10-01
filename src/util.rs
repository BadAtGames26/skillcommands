use engage::{
    battle::BattleInfoSide,
    calculator::{CalculatorCommand, CalculatorManager, GameCalculatorCommand},
    gamedata::unit::Unit,
};
use unity::{
    il2cpp::{self, object::Array},
    prelude::{Il2CppClass, OptionalMethod},
    system::Il2CppString,
};

#[unity::class("App", "List")]
pub struct ListFloats {
    pub items: &'static Array<f32>,
    pub size: i32,
    pub version: i32,
}

pub trait CalculatorManagerTrait {
    fn find_checked(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut CalculatorCommand>;
    fn clone_from_name(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut GameCalculatorCommand>;
}

impl CalculatorManagerTrait for CalculatorManager {
    fn find_checked(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut CalculatorCommand> {
        unsafe { calculator_manager_find_command(self, command_name.into(), None) }
    }
    fn clone_from_name(
        &mut self,
        command_name: impl AsRef<str>,
    ) -> Option<&'static mut GameCalculatorCommand> {
        let command = self
            .find_checked(command_name)
            .map(|cmd| cmd.get_class().clone());
        if let Some(command) = command {
            match il2cpp::instantiate_class::<GameCalculatorCommand>(command) {
                Ok(cmd) => Some(cmd),
                Err(_) => None,
            }
        } else {
            None
        }
    }
}

pub trait CalculatorCommandTrait {
    fn assign_virtual_method(&mut self, method_name: impl AsRef<str>, method_pointer: *mut u8);
    fn assign_vtable(&mut self, vtable_index: usize, method_pointer: *mut u8);
    fn clone(&mut self) -> Option<&'static mut GameCalculatorCommand>;
}

trait CanGetClass {
    fn class_mut(&mut self) -> &mut Il2CppClass;
    fn class(&mut self) -> &Il2CppClass;
}

impl CanGetClass for CalculatorCommand {
    fn class_mut(&mut self) -> &mut Il2CppClass {
        self.get_class_mut()
    }
    fn class(&mut self) -> &Il2CppClass {
        self.get_class()
    }
}

impl CanGetClass for GameCalculatorCommand {
    fn class_mut(&mut self) -> &mut Il2CppClass {
        self.get_class_mut()
    }
    fn class(&mut self) -> &Il2CppClass {
        self.get_class()
    }
}

impl<T> CalculatorCommandTrait for T
where
    T: CanGetClass,
{
    fn assign_virtual_method(&mut self, method_name: impl AsRef<str>, method_pointer: *mut u8) {
        self.class_mut()
            .get_virtual_method_mut(method_name)
            .map(|method| method.method_ptr = method_pointer);
    }
    fn assign_vtable(&mut self, vtable_index: usize, method_pointer: *mut u8) {
        self.class_mut().get_vtable_mut()[vtable_index].method_ptr = method_pointer;
    }
    fn clone(&mut self) -> Option<&'static mut GameCalculatorCommand> {
        match il2cpp::instantiate_class::<GameCalculatorCommand>(self.class().clone()) {
            Ok(cmd) => Some(cmd),
            Err(_) => None,
        }
    }
}

#[skyline::from_offset(0x0298daa0)]
fn calculator_manager_find_command(
    this: &CalculatorManager,
    name: &Il2CppString,
    method_info: OptionalMethod,
) -> Option<&'static mut CalculatorCommand>;

pub trait BattleInfoSideTrait {
    fn get_unit(&self) -> Option<&Unit>;
}
impl BattleInfoSideTrait for BattleInfoSide {
    fn get_unit(&self) -> Option<&Unit> {
        unsafe { battle_info_side_get_unit(self, None) }
    }
}

#[skyline::from_offset(0x01E8B250)]
fn battle_info_side_get_unit(this: &BattleInfoSide, method: OptionalMethod) -> Option<&Unit>;
