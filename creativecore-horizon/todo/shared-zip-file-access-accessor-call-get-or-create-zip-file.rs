// Not ported yet: the Java @Invoker on SharedZipFileAccess.callGetOrCreateZipFile (SharedZipFileAccessAccessor.callGetOrCreateZipFile).
// Why it's off: No Horizon function mapped for SharedZipFileAccess.callGetOrCreateZipFile (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for SharedZipFileAccess.callGetOrCreateZipFile: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   SharedZipFileAccess.callGetOrCreateZipFile -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/SharedZipFileAccessAccessor.java:10:
// | @Invoker
// | ZipFile callGetOrCreateZipFile();
