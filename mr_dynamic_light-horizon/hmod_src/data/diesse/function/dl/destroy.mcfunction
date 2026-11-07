##
 # destroy.mcfunction
 # 
 #
 # Created by .
##
execute at @e[tag=dyl.light_5] run fill ~ ~ ~ ~ ~1 ~ air replace light
execute at @e[tag=dyl.light] run fill ~ ~ ~ ~ ~1 ~ air replace light
kill @e[tag=dyl.light_5]
kill @e[tag=dyl.light]