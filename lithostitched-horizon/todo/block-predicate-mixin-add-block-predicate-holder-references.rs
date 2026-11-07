// Not ported yet: the Java @ModifyExpressionValue(INVOKE) on BlockPredicate.<clinit> (BlockPredicateMixin.addBlockPredicateHolderReferences).
// Why it's off: No Horizon function mapped for BlockPredicate.<clinit> (run `match`, or map it in a --mappings file).
// horizon-mixins kind: MODIFY_EXPRESSION_VALUE
// Note: Selects an exact Rust expression: pick it in the target function.
// Note: No Horizon function known for BlockPredicate.<clinit>: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   Holder.direct -> (not mapped yet)
//   Direct.value -> (not mapped yet)
//   RegistryCodecs.holder -> (not mapped yet)
//   BlockPredicate.<clinit> -> (not mapped yet)
//
// Decompiled Java, dev/worldgen/lithostitched/mixin/common/BlockPredicateMixin.java:17:
// | @ModifyExpressionValue(
// |    method = "<clinit>",
// |    at = @At(
// |       value = "INVOKE",
// |       target = "Lcom/mojang/serialization/Codec;dispatch(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec;"
// |    )
// | )
// | private static Codec<BlockPredicate> addBlockPredicateHolderReferences(Codec<BlockPredicate> directCodec) {
// |    return RegistryCodecs.holder(LithostitchedRegistries.BLOCK_PREDICATE, directCodec).xmap(holder -> {
// |       return switch (holder) {
// |          case Direct<BlockPredicate> direct -> (BlockPredicate)direct.value();
// |          case Reference<BlockPredicate> reference -> new HolderHolderPredicate(reference);
// |          default -> throw new MatchException(null, null);
// |       };
// |    }, value -> {
// |       return switch (value) {
// |          case HolderHolderPredicate(Holder<BlockPredicate> holder) -> holder;
// |          default -> Holder.direct(value);
// |       };
// |    });
// | }
