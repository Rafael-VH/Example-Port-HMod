// Not ported yet: the Java @Accessor on PathPackResources.root (PathPackResourcesAccessor.getRoot).
// Why it's off: No Horizon function mapped for PathPackResources.root (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for PathPackResources.root: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PathPackResources.root -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/PathPackResourcesAccessor.java:10:
// | @Accessor
// | Path getRoot();
