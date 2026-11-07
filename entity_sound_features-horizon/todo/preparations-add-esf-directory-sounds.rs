// Not ported yet: the Java @Inject(TAIL) on Preparations.listResources (Preparations.esf$addESFDirectorySounds).
// Why it's off: No Horizon function mapped for Preparations.listResources (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for Preparations.listResources: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Preparations.listResources -> (not mapped yet)
//   FileToIdConverter.<init> -> (not mapped yet)
//   FileToIdConverter.listMatchingResources -> (not mapped yet)
