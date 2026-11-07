// Not ported yet: the Java @Accessor on MinecraftServer.emptyTicks (MinecraftServerAccess.setEmptyTicks).
// Why it's off: No Horizon function mapped for MinecraftServer.emptyTicks (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for MinecraftServer.emptyTicks: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MinecraftServer.emptyTicks -> (not mapped yet)
//
// Decompiled Java, org/popcraft/chunky/mixin/MinecraftServerAccess.java:9:
// | @Accessor
// | void setEmptyTicks(int var1);
