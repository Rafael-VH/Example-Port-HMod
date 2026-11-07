// Not ported yet: the Java @Invoker on LevelResource.<init> (LevelResourceMixin.invokeInit).
// Why it's off: No Horizon function mapped for LevelResource.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for LevelResource.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LevelResource.<init> -> (not mapped yet)
//
// Decompiled Java, com/tiviacz/travelersbackpack/mixin/LevelResourceMixin.java:9:
// | @Invoker("<init>")
// | static LevelResource invokeInit(String path) {
// |    throw new AssertionError();
// | }
