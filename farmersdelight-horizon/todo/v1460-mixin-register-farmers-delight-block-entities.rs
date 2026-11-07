// Not ported yet: the Java @Inject(RETURN) on V1460.registerBlockEntities (V1460Mixin.registerFarmersDelightBlockEntities).
// Why it's off: No Horizon function mapped for V1460.registerBlockEntities (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for V1460.registerBlockEntities: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   References.ITEM_STACK -> (not mapped yet)
//   V1460.registerBlockEntities -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/datafix/V1460Mixin.java:27:
// | @Inject(method = "registerBlockEntities", at = @At("RETURN"))
// | private void registerFarmersDelightBlockEntities(Schema schema, CallbackInfoReturnable<Map<String, Supplier<TypeTemplate>>> cir) {
// |    Map<String, Supplier<TypeTemplate>> map = (Map<String, Supplier<TypeTemplate>>)cir.getReturnValue();
// |    farmersdelight$registerHandlerInventory(schema, map, farmersdelight$id("stove"));
// |    farmersdelight$registerHandlerInventory(schema, map, farmersdelight$id("cutting_board"));
// |    farmersdelight$registerHandlerInventory(schema, map, farmersdelight$id("skillet"));
// |    schema.register(
// |       map,
// |       farmersdelight$id("cooking_pot"),
// |       p_17586_ -> DSL.optionalFields(
// |          "Inventory", DSL.optionalFields("Items", DSL.list(References.ITEM_STACK.in(schema))), "Container", References.ITEM_STACK.in(schema)
// |       )
// |    );
// |    registerInventory(schema, map, farmersdelight$id("cabinet"));
// |    registerInventory(schema, map, farmersdelight$id("basket"));
// | }
