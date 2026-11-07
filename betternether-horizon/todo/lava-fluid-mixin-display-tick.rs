// Not ported yet: the Java @Inject(HEAD) on LavaFluid.animateTick (LavaFluidMixin.displayTick).
// Why it's off: No Horizon function mapped for LavaFluid.animateTick (run `match`, or map it in a --mappings file).
// horizon-mixins kind: HEAD
// Note: No Horizon function known for LavaFluid.animateTick: run `match`.
//
// Game calls the Java code makes here, and what they are in Horizon:
//   BlockPos.above -> (not mapped yet)
//   BlockPos.below -> (not mapped yet)
//   BlockPos.getX -> (not mapped yet)
//   BlockPos.getY -> (not mapped yet)
//   BlockPos.getZ -> (not mapped yet)
//   ParticleTypes.LARGE_SMOKE -> (not mapped yet)
//   ParticleTypes.LAVA -> (not mapped yet)
//   ParticleTypes.SMOKE -> (not mapped yet)
//   RandomSource.nextDouble -> (not mapped yet)
//   Level.addParticle -> (not mapped yet)
//   Level.getFluidState -> (not mapped yet)
//   FluidState.isEmpty -> (not mapped yet)
//   FluidState.isSource -> (not mapped yet)
//   LavaFluid.animateTick -> (not mapped yet)
//
// Decompiled Java, org/betterx/betternether/mixin/client/LavaFluidMixin.java:19:
// | @Inject(method = "animateTick", at = @At("HEAD"))
// | private void displayTick(Level world, BlockPos blockPos, FluidState fluidState, RandomSource random, CallbackInfo info) {
// |    if (BetterNether.hasLavafallParticles() && !fluidState.isSource()) {
// |       FluidState state = world.getFluidState(blockPos.below());
// |       if (state.isEmpty() || state.isSource()) {
// |          state = world.getFluidState(blockPos.above(3));
// |          if (!state.isEmpty() && !state.isSource()) {
// |             for (int i = 0; i < 10; i++) {
// |                this.spawnParticle(ParticleTypes.LARGE_SMOKE, world, random, blockPos);
// |                this.spawnParticle(ParticleTypes.SMOKE, world, random, blockPos);
// |                this.spawnParticle(ParticleTypes.LAVA, world, random, blockPos);
// |             }
// |          }
// |       }
// |    }
// | }
