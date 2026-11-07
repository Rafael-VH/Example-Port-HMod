// Not ported yet: the Java @Inject(RETURN) on DebugRenderer.refreshRendererList (DebugRendererMixin.refreshRendererList).
// Why it's off: No Horizon function mapped for DebugRenderer.refreshRendererList (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for DebugRenderer.refreshRendererList: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   DebugRenderer.refreshRendererList -> (not mapped yet)
//
// Decompiled Java, com/sonicether/soundphysics/mixin/DebugRendererMixin.java:21:
// | @Inject(method = "refreshRendererList", at = @At("RETURN"))
// | private void refreshRendererList(CallbackInfo ci) {
// |    if (SoundPhysicsMod.CONFIG != null) {
// |       if (SoundPhysicsMod.CONFIG.renderSoundBounces.get() || SoundPhysicsMod.CONFIG.renderOcclusion.get()) {
// |          this.renderers.add(new RaycastRenderer());
// |       }
// |    }
// | }
