// Not ported yet: the Java @Inject(HEAD) on RecipeBookSettings.getSettings (RecipeBookSettingsMixin.fdrf$getCookingBookSettings).
// Why it's off: No Horizon function mapped for RecipeBookSettings.getSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for RecipeBookSettings.getSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RecipeBookSettings.<clinit> -> (not mapped yet)
//   RecipeBookSettings.copy -> (not mapped yet)
//   RecipeBookSettings.getSettings -> (not mapped yet)
//   RecipeBookSettings.replaceFrom -> (not mapped yet)
//   RecipeBookSettings.updateSettings -> (not mapped yet)
//   TypeSettings.DEFAULT -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeBookSettingsMixin.java:57:
// | @Inject(method = "getSettings", at = @At("HEAD"), cancellable = true)
// | private void fdrf$getCookingBookSettings(RecipeBookType type, CallbackInfoReturnable<TypeSettings> cir) {
// |    if (type == FDRecipeBookTypes.COOKING) {
// |       cir.setReturnValue(this.fdrf$cooking);
// |    }
// | }
