// Not ported yet: the Java @Inject(HEAD) on ItemStackComponentizationFix.fixItemStack (ItemStackComponentizationFixMixin.fixCustomStacks).
// Why it's off: No Horizon function mapped for ItemStackComponentizationFix.fixItemStack (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for ItemStackComponentizationFix.fixItemStack: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ItemStackComponentizationFix.fixItemStack -> (not mapped yet)
//   ItemStackData.fixSubTag -> (not mapped yet)
//   ItemStackData.is -> (not mapped yet)
//   ItemStackData.setComponent -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/datafix/ItemStackComponentizationFixMixin.java:15:
// | @Inject(method = "fixItemStack", at = @At("HEAD"))
// | private static void fixCustomStacks(ItemStackData data, Dynamic tag, CallbackInfo ci) {
// |    if (data.is("farmersdelight:cooking_pot")) {
// |       data.fixSubTag(
// |          "BlockEntityTag",
// |          false,
// |          subTag -> {
// |             Optional<? extends Dynamic<?>> container = subTag.get("Container").result();
// |             container.ifPresent(dynamic -> {
// |                Dynamic<?> result = dynamic.set("count", dynamic.createInt(dynamic.get("Count").asInt(1))).remove("Count");
// |                data.setComponent("farmersdelight:container", result);
// |             });
// |             Optional<? extends Dynamic<?>> inventory = subTag.get("Inventory").result();
// |             if (inventory.isPresent()) {
// |                List<Dynamic<?>> list = inventory.get()
// |                   .get("Items")
// |                   .asList(dynamic -> dynamic.set("count", dynamic.createInt(dynamic.get("Count").asInt(2))).remove("Count").remove("Slot"));
// |                if (!list.isEmpty()) {
// |                   data.setComponent("farmersdelight:meal", list.getFirst());
// |                }
// |             }
// |
// |             return subTag.remove("Container").remove("Inventory");
// |          }
// |       );
// |    }
// | }
