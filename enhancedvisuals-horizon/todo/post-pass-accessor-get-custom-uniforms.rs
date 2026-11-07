// Not ported yet: the Java @Accessor on PostPass.customUniforms (PostPassAccessor.getCustomUniforms).
// Why it's off: No Horizon function mapped for PostPass.customUniforms (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for PostPass.customUniforms: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PostPass.customUniforms -> (not mapped yet)
//
// Decompiled Java, team/creative/enhancedvisuals/mixin/PostPassAccessor.java:11:
// | @Accessor
// | Map<String, GpuBuffer> getCustomUniforms();
