use unity::prelude::*;
use engage::{calculator::*, gamedata::unit::Unit};

pub fn register_rank(calculator: &mut CalculatorManager) {
    //  JobRankCommand (兵種ランク) treats special classes as base classes, so let's change it so special classes are treated differently
    // Replacing what job rank command (兵種ランク) so that special classes will return 2 instead 0.
    let job_rank: &mut CalculatorCommand  = calculator.find_command("兵種ランク");
    // replacing the GetImpl with our custom one
    job_rank.get_class_mut().get_virtual_method_mut("GetImpl").map(|method| method.method_ptr = get_job_rank as _);
    // no need to add it as it's already in the calculator manager
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
