// Not ported yet: the Java @Accessor on MouseHandler.lastHandleMovementTime (MouseHandlerAccessor.getLastHandleMovementTime).
// Why it's off: No Horizon function mapped for MouseHandler.lastHandleMovementTime (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for MouseHandler.lastHandleMovementTime: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   MouseHandler.lastHandleMovementTime -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/MouseHandlerAccessor.java:9:
// | @Accessor
// | double getLastHandleMovementTime();
