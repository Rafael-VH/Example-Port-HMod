// Not ported yet: the Java @Accessor on Channel.source (ChannelAccessor.getSource).
// Why it's off: No Horizon function mapped for Channel.source (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for Channel.source: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Channel.source -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/ChannelAccessor.java:9:
// | @Accessor
// | int getSource();
