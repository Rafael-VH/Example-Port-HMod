// Not ported yet: the Java @Accessor on AbstractContainerScreen.leftPos (HandledScreenAccessor.graveyard$getX).
// Why it's off: No Horizon function mapped for AbstractContainerScreen.leftPos (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for AbstractContainerScreen.leftPos: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   AbstractContainerScreen.imageWidth -> (not mapped yet)
//   AbstractContainerScreen.leftPos -> (not mapped yet)
//   AbstractContainerScreen.topPos -> (not mapped yet)
//
// Decompiled Java, com/lion/graveyard/mixin/client/HandledScreenAccessor.java:9:
// | @Accessor("leftPos")
// | int graveyard$getX();
