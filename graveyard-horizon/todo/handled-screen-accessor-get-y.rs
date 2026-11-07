// Not ported yet: the Java @Accessor on AbstractContainerScreen.topPos (HandledScreenAccessor.graveyard$getY).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.topPos (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for AbstractContainerScreen.topPos: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AbstractContainerScreen.imageWidth -> (not mapped yet)
//   AbstractContainerScreen.leftPos -> (not mapped yet)
//   AbstractContainerScreen.topPos -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/client/HandledScreenAccessor.java:12:
// | @Accessor("topPos")
// | int graveyard$getY();
