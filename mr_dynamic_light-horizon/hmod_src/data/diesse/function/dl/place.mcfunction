##
 # place.mcfunction
 # 
 #
 # Created by .
##
execute at @e[predicate=diesse:on_fire] run function diesse:dl/light
execute as @e[type=minecraft:creeper] store result score @s dyl.ignited run data get entity @s ignited
execute at @e[type=creeper,scores={dyl.ignited=1}] run function diesse:dl/light
execute at @e[type=tnt] run function diesse:dl/light
execute at @e[predicate=diesse:glowing] run function diesse:dl/light
execute at @e[predicate=diesse:light_o] run function diesse:dl/light
execute at @e[predicate=diesse:light_m] run function diesse:dl/light

execute at @e[predicate=diesse:light_5_o] run function diesse:dl/light_5
execute at @e[predicate=diesse:light_5_m] run function diesse:dl/light_5
execute at @e[type=minecraft:copper_golem] run function diesse:dl/light_5
