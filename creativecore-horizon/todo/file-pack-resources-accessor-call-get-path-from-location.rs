// Not ported yet: the Java @Invoker on FilePackResources.callGetPathFromLocation (FilePackResourcesAccessor.callGetPathFromLocation).
// Why it's off: No Horizon function mapped for FilePackResources.callGetPathFromLocation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for FilePackResources.callGetPathFromLocation: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FilePackResources.callGetPathFromLocation -> (not mapped yet)
//   FilePackResources.zipFileAccess -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/FilePackResourcesAccessor.java:13:
// | @Invoker
// | static String callGetPathFromLocation(PackType type, Identifier location) {
// |    throw new UnsupportedOperationException();
// | }
