// Not ported yet: the Java @Accessor on ChunkHolder.pendingFullStateConfirmation (ChunkHolderAccess.chunky$getPendingFullStateConfirmation).
// Why it's off: No Horizon function mapped for ChunkHolder.pendingFullStateConfirmation (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for ChunkHolder.pendingFullStateConfirmation: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ChunkHolder.pendingFullStateConfirmation -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/ChunkHolderAccess.java:10:
// | @Accessor("pendingFullStateConfirmation")
// | CompletableFuture<?> chunky$getPendingFullStateConfirmation();
