// Not ported yet: the Java @Accessor on MappedRegistry.byValue (MappedRegistryAccessor.getByValue).
// Why it's off: No Horizon function mapped for MappedRegistry.byValue (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for MappedRegistry.byValue: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MappedRegistry.byValue -> (not mapped yet)
//   MappedRegistry.registrationInfos -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/MappedRegistryAccessor.java:18:
// | @Accessor("byValue")
// | Map<T, Reference<T>> getByValue();
