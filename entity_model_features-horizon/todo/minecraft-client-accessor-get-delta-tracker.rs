// Not ported yet: the Java @Accessor on Minecraft.deltaTracker (MinecraftClientAccessor.getDeltaTracker).
// Why it's off: No Horizon function mapped for Minecraft.deltaTracker (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Minecraft.deltaTracker: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Minecraft.deltaTracker -> (not mapped yet)
//
// Decompiled Java, traben/entity_model_features/mixin/mixins/accessor/MinecraftClientAccessor.java:10:
// | @Accessor
// | Timer getDeltaTracker();
