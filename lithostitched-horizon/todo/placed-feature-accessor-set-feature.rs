// Not ported yet: the Java @Accessor on PlacedFeature.feature (PlacedFeatureAccessor.setFeature).
// Why it's off: No Horizon function mapped for PlacedFeature.feature (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for PlacedFeature.feature: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PlacedFeature.feature -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/PlacedFeatureAccessor.java:12:
// | @Accessor("feature")
// | @Mutable
// | void setFeature(Holder<Feature> var1);
