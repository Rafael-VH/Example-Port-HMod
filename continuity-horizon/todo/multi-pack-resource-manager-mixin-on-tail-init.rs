// Not ported yet: the Java @Inject(TAIL) on MultiPackResourceManager.<init> (MultiPackResourceManagerMixin.continuity$onTailInit).
// Why it's off: No Horizon function mapped for MultiPackResourceManager.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon has no TAIL: RETURN runs at every exit, early ones too.
// Note: No Horizon function known for MultiPackResourceManager.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   PackType.CLIENT_RESOURCES -> (not mapped yet)
//   MultiPackResourceManager.<init> -> (not mapped yet)
//   MultiPackResourceManager.getResource -> (not mapped yet)
//   MultiPackResourceManager.getResourceStack -> (not mapped yet)
//
// Decompiled Java, me/pepperbell/continuity/client/mixin/MultiPackResourceManagerMixin.java:21:
// | @Inject(method = "<init>(Lnet/minecraft/server/packs/PackType;Ljava/util/List;)V", at = @At("TAIL"))
// | private void continuity$onTailInit(PackType type, List<PackResources> packs, CallbackInfo ci) {
// |    this.continuity$useRedirects = type == PackType.CLIENT_RESOURCES;
// | }
