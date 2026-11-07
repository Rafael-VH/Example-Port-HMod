// Not ported yet: the Java @Accessor on VoxelShape.shape (VoxelShapeAccessor.setShape).
// Why it's off: No Horizon function mapped for VoxelShape.shape (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for VoxelShape.shape: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   VoxelShape.shape -> (not mapped yet)
//
// Decompiled Java, team/creative/creativecore/mixin/VoxelShapeAccessor.java:11:
// | @Accessor
// | @Mutable
// | void setShape(DiscreteVoxelShape var1);
