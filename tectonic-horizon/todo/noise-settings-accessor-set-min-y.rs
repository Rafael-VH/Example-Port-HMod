// Not ported yet: the Java @Accessor on NoiseSettings.minY (NoiseSettingsAccessor.setMinY).
// Why it's off: No Horizon function mapped for NoiseSettings.minY (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for NoiseSettings.minY: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   NoiseSettings.height -> (not mapped yet)
//   NoiseSettings.minY -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/tectonic/mixin/NoiseSettingsAccessor.java:10:
// | @Accessor("minY")
// | @Mutable
// | void setMinY(int var1);
