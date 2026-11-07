// Not ported yet: the Java @Accessor on MobSpawnSettings.spawnsByCategory (MobSpawnSettingsAccessor.setSpawnsByCategory).
// Why it's off: No Horizon function mapped for MobSpawnSettings.spawnsByCategory (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for MobSpawnSettings.spawnsByCategory: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MobSpawnSettings.spawnsByCategory -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/MobSpawnSettingsAccessor.java:16:
// | @Accessor("spawnsByCategory")
// | void setSpawnsByCategory(Map<MobCategory, WeightedList<SpawnerData>> var1);
