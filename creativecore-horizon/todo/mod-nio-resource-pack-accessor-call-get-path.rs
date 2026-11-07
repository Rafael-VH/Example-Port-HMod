// Not ported yet: the Java @Invoker on ModNioPackResources.callGetPath (ModNioResourcePackAccessor.callGetPath).
// Why it's off: No Horizon function mapped for ModNioPackResources.callGetPath (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ModNioPackResources.callGetPath: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModNioPackResources.callGetFilename -> (not mapped yet)
//   ModNioPackResources.callGetPath -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/ModNioResourcePackAccessor.java:17:
// | @Invoker
// | Path callGetPath(String var1);
