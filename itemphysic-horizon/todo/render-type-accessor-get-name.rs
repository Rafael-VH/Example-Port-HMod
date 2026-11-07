// Not ported yet: the Java @Accessor on RenderType.name (RenderTypeAccessor.getName).
// Why it's off: No Horizon function mapped for RenderType.name (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for RenderType.name: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RenderType.name -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/RenderTypeAccessor.java:9:
// | @Accessor
// | String getName();
