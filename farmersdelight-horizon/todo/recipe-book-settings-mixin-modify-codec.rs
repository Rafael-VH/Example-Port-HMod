// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on RecipeBookSettings.<clinit> (RecipeBookSettingsMixin.fdrf$modifyCodec).
// Why it's off: No Horizon function mapped for RecipeBookSettings.<clinit> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for RecipeBookSettings.<clinit>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RecipeBookSettings.<clinit> -> (not mapped yet)
//   RecipeBookSettings.copy -> (not mapped yet)
//   RecipeBookSettings.getSettings -> (not mapped yet)
//   RecipeBookSettings.replaceFrom -> (not mapped yet)
//   RecipeBookSettings.updateSettings -> (not mapped yet)
//   TypeSettings.DEFAULT -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeBookSettingsMixin.java:25:
// | @ModifyExpressionValue(
// |    method = "<clinit>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/serialization/codecs/RecordCodecBuilder;mapCodec(Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec;"
// |    )
// | )
// | private static MapCodec<RecipeBookSettings> fdrf$modifyCodec(MapCodec<RecipeBookSettings> original) {
// |    return RecordCodecBuilder.mapCodec(
// |       inst -> inst.group(
// |             original.forGetter(Function.identity()),
// |             RecipeBookSettingsTypeSettingsInvoker.fdrf$invokeCodec("isFarmersDelightCookingGuiOpen", "isFarmersDelightCookingFilteringCraftable")
// |                .forGetter(recipeBookSettings -> ((RecipeBookSettingsMixin)recipeBookSettings).fdrf$cooking)
// |          )
// |          .apply(inst, (recipeBookSettings, typeSettings) -> {
// |             ((RecipeBookSettingsMixin)recipeBookSettings).fdrf$cooking = typeSettings;
// |             return recipeBookSettings;
// |          })
// |    );
// | }
