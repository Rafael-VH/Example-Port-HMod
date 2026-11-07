// Not ported yet: the Java @Accessor on JigsawStructure.poolAliases (JigsawStructureAccessor.setPoolAliases).
// Why it's off: No Horizon function mapped for JigsawStructure.poolAliases (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for JigsawStructure.poolAliases: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   JigsawStructure.poolAliases -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/JigsawStructureAccessor.java:12:
// | @Accessor("poolAliases")
// | @Mutable
// | void setPoolAliases(List<PoolAliasBinding> var1);
