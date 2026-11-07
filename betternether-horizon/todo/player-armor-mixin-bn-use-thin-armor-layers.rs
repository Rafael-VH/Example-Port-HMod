// Not ported yet: the Java @ModifyArg(INVOKE) on AvatarRenderer.<init> (PlayerArmorMixin.bn_useThinArmorLayers).
// Why it's off: No Horizon function mapped for AvatarRenderer.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_ARG
// Note: No Horizon function known for AvatarRenderer.<init>: run `match`.
// Note: The call it hooks (ArmorModelSet.bake) isn't mapped to a Rust function yet.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   ModelLayers.PLAYER_SLIM_ARMOR -> (not mapped yet)
//   ArmorModelSet.bake -> (not mapped yet)
//   AvatarRenderer.<init> -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/client/PlayerArmorMixin.java:17:
// | @ModifyArg(
// |    method = "<init>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lnet/minecraft/client/renderer/entity/ArmorModelSet;bake(Lnet/minecraft/client/renderer/entity/ArmorModelSet;Lnet/minecraft/client/model/geom/EntityModelSet;Ljava/util/function/Function;)Lnet/minecraft/client/renderer/entity/ArmorModelSet;"
// |    ),
// |    index = 0
// | )
// | private ArmorModelSet<ModelLayerLocation> bn_useThinArmorLayers(ArmorModelSet<ModelLayerLocation> locations) {
// |    return Configs.CLIENT.thinArmor.get() ? ModelLayers.PLAYER_SLIM_ARMOR : locations;
// | }
