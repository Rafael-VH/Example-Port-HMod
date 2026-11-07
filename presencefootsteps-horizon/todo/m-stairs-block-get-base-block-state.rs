// Not ported yet: the Java @Accessor on StairBlock.baseState (MStairsBlock.getBaseBlockState).
// Why it's off: No Horizon function mapped for StairBlock.baseState (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for StairBlock.baseState: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   StairBlock.baseState -> (not mapped yet)
//
// Decompiled Java, eu/ha3/presencefootsteps/mixins/MStairsBlock.java:11:
// | @Accessor("baseState")
// | @Override
// | public abstract BlockState getBaseBlockState();
