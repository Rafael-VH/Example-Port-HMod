// Not ported yet: the Java @Invoker on ItemStackRenderState.callFirstLayer (ItemStackRenderStateAccessor.callFirstLayer).
// Why it's off: No Horizon function mapped for ItemStackRenderState.callFirstLayer (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for ItemStackRenderState.callFirstLayer: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemStackRenderState.callFirstLayer -> (not mapped yet)
//
// Decompiled Java, team/creative/itemphysic/mixin/ItemStackRenderStateAccessor.java:10:
// | @Invoker
// | LayerRenderState callFirstLayer();
