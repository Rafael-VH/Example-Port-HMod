// Not ported yet: the Java @Accessor on NoiseSettings.height (NoiseSettingsAccessor.setHeight).
// Why it's off: No Horizon function mapped for NoiseSettings.height (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for NoiseSettings.height: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NoiseSettings.height -> (not mapped yet)
//   NoiseSettings.minY -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/NoiseSettingsAccessor.java:14:
// | @Accessor("height")
// | @Mutable
// | void setHeight(int var1);
