// Not ported yet: the Java @Accessor on StructureProcessorList.list (StructureProcessorListAccessor.setProcessors).
// Why it's off: No Horizon function mapped for StructureProcessorList.list (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for StructureProcessorList.list: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   StructureProcessorList.list -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/StructureProcessorListAccessor.java:12:
// | @Accessor("list")
// | @Mutable
// | void setProcessors(List<StructureProcessor> var1);
