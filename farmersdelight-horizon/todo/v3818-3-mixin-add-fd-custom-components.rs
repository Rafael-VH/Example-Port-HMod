// Not ported yet: the Java @ModifyArg(INVOKE) on V3818_3.lambda$registerTypes$0 (V3818_3Mixin.addFDCustomComponents).
// Why it's off: No Horizon function mapped for V3818_3.lambda$registerTypes$0 (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for V3818_3.lambda$registerTypes$0: run `match`.
// Note: The call it hooks (DSL.optionalFieldsLazy) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DSL.optionalFieldsLazy -> (not mapped yet)
//   References.ITEM_STACK -> (not mapped yet)
//   V3818_3.lambda$registerTypes$0 -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/datafix/V3818_3Mixin.java:17:
// | @ModifyArg(
// |    method = "lambda$registerTypes$0",
// |    at = @At(value = "INVOKE", target = "Lcom/mojang/datafixers/DSL;optionalFieldsLazy(Ljava/util/Map;)Lcom/mojang/datafixers/types/templates/TypeTemplate;")
// | )
// | private static Map<String, Supplier<TypeTemplate>> addFDCustomComponents(Map<String, Supplier<TypeTemplate>> fields, @Local(argsOnly = true) Schema schema) {
// |    HashMap<String, Supplier<TypeTemplate>> map = new HashMap<>(fields);
// |    map.put("farmersdelight:meal", () -> References.ITEM_STACK.in(schema));
// |    map.put("farmersdelight:container", () -> References.ITEM_STACK.in(schema));
// |    return map;
// | }
