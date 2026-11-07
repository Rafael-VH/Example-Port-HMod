// Not ported yet: the Java @Accessor on FilePackResources.zipFileAccess (FilePackResourcesAccessor.getZipFileAccess).
// Why it's off: No Horizon function mapped for FilePackResources.zipFileAccess (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for FilePackResources.zipFileAccess: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   FilePackResources.callGetPathFromLocation -> (not mapped yet)
//   FilePackResources.zipFileAccess -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/FilePackResourcesAccessor.java:18:
// | @Accessor
// | SharedZipFileAccess getZipFileAccess();
