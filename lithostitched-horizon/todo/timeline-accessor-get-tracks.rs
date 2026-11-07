// Not ported yet: the Java @Accessor on Timeline.tracks (TimelineAccessor.getTracks).
// Why it's off: No Horizon function mapped for Timeline.tracks (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Timeline.tracks: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Timeline.tracks -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/TimelineAccessor.java:13:
// | @Accessor("tracks")
// | Map<EnvironmentAttribute<?>, AttributeTrack<?, ?>> getTracks();
