#![feature(ptr_sub_ptr)]
use engage::calculator::*;
use unity::{prelude::*, il2cpp::object::Array};


mod gold;
mod mov;
mod rank;
mod sidrange;
mod triangle;
mod unitstatus;


#[unity::class("App", "List")]
pub struct ListFloats {
    pub items: &'static Array<f32>,
    pub size: i32,
    pub version: i32,
}

#[unity::hook("App", "UnitCalculator", "AddCommand")]
fn add_command_hook(calculator: &mut CalculatorManager, method_info: OptionalMethod) {
    // GameCalculator is a CalculatorManager
    call_original!(calculator, method_info);

    // Example 1: Grabing the movement stat of a unit does not exist, so lets create it by editing the command for luck
    mov::register_move(calculator);

    // Example 2: Rewriting what an already existing command does. 
    rank::register_rank(calculator);

    // Example 3: Triangle Attack condition. This example highlights how you can create a command that does something completely new
    triangle::register_triangle(calculator);

    // Example 4: SID Skill Range Skill: most involved custom skill command that will return the number of units (from a particular force) within a range that has a skil;
    sidrange::register_sid_range(calculator);

    // Example 5: UnitStatus: Checks the status on a unit
    unitstatus::register_unit_status(calculator);
    
    // Example 6: Grabs the current gold a player has and can set it to another value.
    gold::register_gold(calculator);
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