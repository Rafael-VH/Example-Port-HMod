// Not ported yet: the Java @Inject(RETURN) on Submit.<init> (Mixin_ModelSubmit_AddData.emf$initBackupState).
// Why it's off: No Horizon function mapped for Submit.<init> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: RETURN
// Note: Horizon's RETURN runs at every normal exit and passes the return value on.
// Note: No Horizon function known for Submit.<init>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Submit.<init> -> (not mapped yet)
//
// Decompiled Java, traben/entity_texture_features/mixin/mixins/submit/Mixin_ModelSubmit_AddData.java:27:
// | @Inject(method = "<init>", at = @At("RETURN"))
// | private void emf$initBackupState(CallbackInfo ci) {
// |    if (this.data != null) {
// |       Submit self = (Submit)this;
// |       List<BiConsumer<ETFSubmitData, Submit>> in = ETFSubmitData.DATA_IN;
// |       int i = 0;
// |
// |       for (int n = in.size(); i < n; i++) {
// |          in.get(i).accept(this.data, self);
// |       }
// |    }
// | }
