// Not ported yet: the Java @ModifyVariable(HEAD) on LevelDatToSavedDataFileFix.extractWorldGenSettingsToFile (LevelStorageMixin.graveyard$dropPastDimension).
// Why it's off: No Horizon function mapped for LevelDatToSavedDataFileFix.extractWorldGenSettingsToFile (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_VARIABLE
// Note: Selects a Rust local by name: pick it in the target function.
// Note: No Horizon function known for LevelDatToSavedDataFileFix.extractWorldGenSettingsToFile: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelDatToSavedDataFileFix.extractWorldGenSettingsToFile -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/LevelStorageMixin.java:11:
// | @ModifyVariable(method = "extractWorldGenSettingsToFile", at = @At("HEAD"), argsOnly = true, ordinal = 0)
// | private Dynamic<?> graveyard$dropPastDimension(Dynamic<?> levelData) {
// |    return levelData.update("world_gen_settings", settings -> settings.update("dimensions", dimensions -> dimensions.remove("graveyard:past")));
// | }
