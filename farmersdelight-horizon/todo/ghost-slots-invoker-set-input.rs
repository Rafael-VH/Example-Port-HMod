// Not ported yet: the Java @Invoker on GhostSlots.setInput (GhostSlotsInvoker.fdrf$setInput).
// Why it's off: No Horizon function mapped for GhostSlots.setInput (run `match`, or map it in a --mappings file).
// horizon-mixins kind: INVOKE
// Note: A [[bridge]] INVOKE of the Rust method.
// Note: No Horizon function known for GhostSlots.setInput: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   GhostSlots.setInput -> (not mapped yet)
//   GhostSlots.setResult -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/GhostSlotsInvoker.java:12:
// | @Invoker("setInput")
// | void fdrf$setInput(Slot var1, ContextMap var2, SlotDisplay var3);
