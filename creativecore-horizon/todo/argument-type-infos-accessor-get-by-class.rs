// Not ported yet: the Java @Accessor on ArgumentTypeInfos.BY_CLASS (ArgumentTypeInfosAccessor.getByClass).
// Why it's off: No Horizon function mapped for ArgumentTypeInfos.BY_CLASS (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ArgumentTypeInfos.BY_CLASS: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ArgumentTypeInfos.BY_CLASS -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ArgumentTypeInfosAccessor.java:11:
// | @Accessor("BY_CLASS")
// | static Map<Class<?>, ArgumentTypeInfo<?, ?>> getByClass() {
// |    throw new AssertionError();
// | }
