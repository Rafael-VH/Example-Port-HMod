// Not ported yet: the Java @Accessor on StructureSet.structures (StructureSetAccessor.setStructures).
// Why it's off: No Horizon function mapped for StructureSet.structures (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for StructureSet.structures: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   StructureSet.structures -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/StructureSetAccessor.java:12:
// | @Accessor("structures")
// | @Mutable
// | void setStructures(List<StructureSelectionEntry> var1);
