// Not ported yet: the Java @Invoker on GhostSlots.setResult (GhostSlotsInvoker.fdrf$setResult).
// Why it's off: No Horizon function mapped for GhostSlots.setResult (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for GhostSlots.setResult: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GhostSlots.setInput -> (not mapped yet)
//   GhostSlots.setResult -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/GhostSlotsInvoker.java:15:
// | @Invoker("setResult")
// | void fdrf$setResult(Slot var1, ContextMap var2, SlotDisplay var3);
