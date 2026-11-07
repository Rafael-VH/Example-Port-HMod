// Not ported yet: the Java @Accessor on SinglePoolElement.processors (SinglePoolElementAccessor.setProcessors).
// Why it's off: No Horizon function mapped for SinglePoolElement.processors (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for SinglePoolElement.processors: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SinglePoolElement.processors -> (not mapped yet)
//   SinglePoolElement.template -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/SinglePoolElementAccessor.java:23:
// | @Accessor("processors")
// | @Mutable
// | void setProcessors(Holder<StructureProcessorList> var1);
