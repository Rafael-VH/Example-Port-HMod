// Not ported yet: the Java @Invoker on ModNioPackResources.callGetFilename (ModNioResourcePackAccessor.callGetFilename).
// Why it's off: No Horizon function mapped for ModNioPackResources.callGetFilename (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ModNioPackResources.callGetFilename: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModNioPackResources.callGetFilename -> (not mapped yet)
//   ModNioPackResources.callGetPath -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ModNioResourcePackAccessor.java:12:
// | @Invoker
// | static String callGetFilename(PackType type, Identifier id) {
// |    throw new UnsupportedOperationException();
// | }
