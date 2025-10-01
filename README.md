# Skill Commands Plugin
Adds new skill commands for use in Skill.xml. Does nothing on its own on the user side and is intended for mod authors.

## New Skill Commands
Mov - The target's movement stat.

Triangle - Checks if target is surrounded by 3 or more units of the same battle style.

SidRange - Checks if a unit has skill in a range. Usage is SidRange( Range, スキル(&quot;SID&quot;), Force (If none, checks force of unit) ).
  - Range 99 checks all units in force.
  - Range 0 check self.

UnitStatus - Checks if unit has a certain status. Usage is UnitStatus( Status ).

Gold - The current gold an army has, can increase or decrease.

Nation - Returns unit's nation/hometown. Returns -1 if no unit.  

TerrainHeal - Returns unit's terrain heal, negative for damage.

TerrainMov - Returns unit's terrain move_first.

TerrainImmuneBreak - Returns 1 if unit's terrain grants immunity to break.  

## Adjusted Skill Commands
兵種ランク - Special Classes now return 2 instead of being treated as a base class.  

地形回避 - Now is accessible outside a combat, for example, it's accessible at phase start.

地形防御 - Same as 地形回避.