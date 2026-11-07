// Not ported yet: the Java @Inject(HEAD) on RecipeBookSettings.updateSettings (RecipeBookSettingsMixin.fdrf$updateCookingBookSettings).
// Why it's off: No Horizon function mapped for RecipeBookSettings.updateSettings (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for RecipeBookSettings.updateSettings: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   RecipeBookSettings.<clinit> -> (not mapped yet)
//   RecipeBookSettings.copy -> (not mapped yet)
//   RecipeBookSettings.getSettings -> (not mapped yet)
//   RecipeBookSettings.replaceFrom -> (not mapped yet)
//   RecipeBookSettings.updateSettings -> (not mapped yet)
//   TypeSettings.DEFAULT -> (not mapped yet)
//
// Decompiled Java, vectorwing/farmersdelight/common/mixin/refabricated/RecipeBookSettingsMixin.java:64:
// | @Inject(method = "updateSettings", at = @At("HEAD"), cancellable = true)
// | private void fdrf$updateCookingBookSettings(RecipeBookType type, UnaryOperator<TypeSettings> updater, CallbackInfo ci) {
// |    if (type == FDRecipeBookTypes.COOKING) {
// |       this.fdrf$cooking = updater.apply(this.fdrf$cooking);
// |       ci.cancel();
// |    }
// | }
