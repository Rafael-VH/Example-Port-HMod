// Not ported yet: the Java @Accessor on LayerRenderState.itemTransform (LayerRenderStateAccessor.getItemTransform).
// Why it's off: No Horizon function mapped for LayerRenderState.itemTransform (run `match`, or map it in a --mappings file).
// horizon-mixins kind: GET
// Note: A [[bridge]] GET (SET for a setter) on the Rust struct's field.
// Note: No Horizon function known for LayerRenderState.itemTransform: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   LayerRenderState.itemTransform -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/LayerRenderStateAccessor.java:10:
// | @Accessor
// | ItemTransform getItemTransform();
