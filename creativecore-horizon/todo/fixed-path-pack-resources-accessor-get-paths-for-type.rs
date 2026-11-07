// Not ported yet: the Java @Accessor on FixedPathPackResources.pathsForType (FixedPathPackResourcesAccessor.getPathsForType).
// Why it's off: No Horizon function mapped for FixedPathPackResources.pathsForType (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for FixedPathPackResources.pathsForType: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FixedPathPackResources.pathsForType -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/FixedPathPackResourcesAccessor.java:13:
// | @Accessor
// | Map<PackType, List<Path>> getPathsForType();
