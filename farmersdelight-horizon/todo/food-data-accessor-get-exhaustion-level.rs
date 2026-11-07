// Not ported yet: the Java @Accessor on FoodData.exhaustionLevel (FoodDataAccessor.fdrf$getExhaustionLevel).
// Why it's off: No Horizon function mapped for FoodData.exhaustionLevel (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for FoodData.exhaustionLevel: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FoodData.exhaustionLevel -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/FoodDataAccessor.java:9:
// | @Accessor("exhaustionLevel")
// | float fdrf$getExhaustionLevel();
