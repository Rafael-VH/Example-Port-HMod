# Horizon port report: Lithostitched

`lithostitched` 2.0.6 · lithostitched-2.0.6-fabric-26.3.jar · 354 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 2 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 116 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 21 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 5 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: dev.worldgen.lithostitched.impl.platform.fabric.LithostitchedFabric
- `client`: dev.worldgen.lithostitched.impl.platform.fabric.LithostitchedFabricClient

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| PayloadTypeRegistry.clientboundPlay | `LithostitchedFabric` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| PayloadTypeRegistry.register | `LithostitchedFabric` | – | unsupported | Horizon speaks the vanilla protocol; custom packets aren't supported. |
| ServerTickEvents.START_LEVEL_TICK | `LithostitchedFabric` | hook `tick` | auto | The tick hook runs in every Game, the server's too: run server logic only when `!self.native_window()`. |
| ClientPlayNetworking.registerGlobalReceiver | `LithostitchedFabricClient` | – | unsupported | Custom packets aren't supported. |
| ClientTickEvents.START_LEVEL_TICK | `LithostitchedFabricClient` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| ServerPlayNetworking.canSend | `StructureAttributeHandler` | – | unsupported | Custom packets aren't supported. |
| ServerPlayNetworking.send | `StructureAttributeHandler` | – | unsupported | Custom packets aren't supported. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| mixin into ClientLevel with no injectors (interfaces or @Shadow only) | `ClientLevelMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on EnvironmentAttributeSystem.addDynamicLayers at Lnet/minecraft/world/level/dimension/DimensionType;timelines()Lnet/minecraft/core/HolderSet; | `EnvironmentAttributeSystemMixin.applyStructureAttributes` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for EnvironmentAttributeSystem.addDynamicLayers: run `match`. The call it hooks (DimensionType.timelines) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on IntegratedServer.initServer at Lnet/minecraft/client/server/IntegratedServer;loadLevel()V | `IntegratedServerMixin.initServer` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for IntegratedServer.initServer: run `match`. The call it hooks (IntegratedServer.loadLevel) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on MusicManager.tick at Lnet/minecraft/client/Minecraft;getMusicVolume()F | `MusicManagerMixin.applyVolumeMultiplier` | WRAP_OPERATION on horizon_client::music::MusicManager::tick | needs-mapping | The call it hooks (Minecraft.getMusicVolume) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on Beardifier.forStructuresInChunk at Ljava/util/List;add(Ljava/lang/Object;)Z | `BeardifierMixin.overrideTerrainAdaptation` | WRAP_OPERATION | needs-mapping | No Horizon function known for Beardifier.forStructuresInChunk: run `match`. The call it hooks (List.add) isn't mapped to a Rust function yet. |
| @Accessor on Biome.climateSettings | `BiomeAccessor.getClimateSettings` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Biome.climateSettings: run `match`. |
| @Accessor on Biome.climateSettings | `BiomeAccessor.setClimateSettings` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Biome.climateSettings: run `match`. |
| @Accessor on Biome.specialEffects | `BiomeAccessor.getSpecialEffects` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Biome.specialEffects: run `match`. |
| @Accessor on Biome.specialEffects | `BiomeAccessor.setSpecialEffects` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Biome.specialEffects: run `match`. |
| @Accessor on Biome.generationSettings | `BiomeAccessor.setGenerationSettings` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Biome.generationSettings: run `match`. |
| @Accessor on Biome.attributes | `BiomeAccessor.setAttributes` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Biome.attributes: run `match`. |
| @Invoker on BiomeGenerationSettings.<init> | `BiomeGenerationSettingsAccessor.createGenerationSettings` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for BiomeGenerationSettings.<init>: run `match`. |
| @Accessor on BiomeGenerationSettings.carvers | `BiomeGenerationSettingsAccessor.getCarvers` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for BiomeGenerationSettings.carvers: run `match`. |
| @Invoker on BiomeSource.codec | `BiomeSourceInvoker.getCodec` | INVOKE | needs-mapping | A [[bridge]] INVOKE of the Rust method. No Horizon function known for BiomeSource.codec: run `match`. |
| @ModifyExpressionValue(INVOKE) on BlockPredicate.<clinit> at Lcom/mojang/serialization/Codec;dispatch(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/Codec; | `BlockPredicateMixin.addBlockPredicateHolderReferences` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for BlockPredicate.<clinit>: run `match`. |
| @Accessor on ChunkGenerator.biomeSource | `ChunkGeneratorAccessor.getBiomeSource` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ChunkGenerator.biomeSource: run `match`. |
| @Accessor on ChunkGenerator.biomeSource | `ChunkGeneratorAccessor.setBiomeSource` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ChunkGenerator.biomeSource: run `match`. |
| @Accessor on ChunkGenerator.generationSettingsGetter | `ChunkGeneratorAccessor.getGetter` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ChunkGenerator.generationSettingsGetter: run `match`. |
| @Accessor on ChunkGenerator.featuresPerStep | `ChunkGeneratorAccessor.setFeaturesPerStep` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ChunkGenerator.featuresPerStep: run `match`. |
| @WrapOperation(INVOKE) on ChunkMap.<init> at Lnet/minecraft/world/level/levelgen/RandomState;create(Lnet/minecraft/core/HolderGetter;JLnet/minecraft/world/level/levelgen/NoiseGeneratorSettings;)Lnet/minecraft/world/level/levelgen/RandomState; | `ChunkMapMixin.wrapNoiseRouter` | WRAP_OPERATION | needs-mapping | No Horizon function known for ChunkMap.<init>: run `match`. The call it hooks (RandomState.create) isn't mapped to a Rust function yet. |
| @Accessor on DimensionType.attributes | `DimensionTypeAccessor.setAttributes` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for DimensionType.attributes: run `match`. |
| mixin into Builder with no injectors (interfaces or @Shadow only) | `EnvironmentAttributeMapBuilderMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on GameTestServer.initServer at Lnet/minecraft/gametest/framework/GameTestServer;loadLevel()V | `GameTestServerMixin.initServer` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for GameTestServer.initServer: run `match`. The call it hooks (GameTestServer.loadLevel) isn't mapped to a Rust function yet. |
| @Accessor on Reference.value | `HolderReferenceAccessor.setValue` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Reference.value: run `match`. |
| @Accessor on JigsawStructure.poolAliases | `JigsawStructureAccessor.setPoolAliases` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for JigsawStructure.poolAliases: run `match`. |
| @Accessor on JigsawStructure.poolAliases | `JigsawStructureAccessor.getPoolAliases` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for JigsawStructure.poolAliases: run `match`. |
| @Redirect(INVOKE) on JigsawStructure.findGenerationPoint at Lnet/minecraft/world/level/levelgen/structure/pools/JigsawPlacement;addPieces(Lnet/minecraft/world/level/levelgen/structure/Structure$GenerationContext;Lnet/minecraft/core/Holder;Ljava/util/Optional;ILnet/minecraft/core/BlockPos;ZLjava/util/Optional;Lnet/minecraft/world/level/levelgen/structure/structures/JigsawStructure$MaxDistance;Lnet/minecraft/world/level/levelgen/structure/pools/alias/PoolAliasLookup;Lnet/minecraft/world/level/levelgen/structure/pools/DimensionPadding;Lnet/minecraft/world/level/levelgen/structure/templatesystem/LiquidSettings;)Ljava/util/Optional; | `JigsawStructureMixin.init` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for JigsawStructure.findGenerationPoint: run `match`. The call it hooks (JigsawPlacement.addPieces) isn't mapped to a Rust function yet. |
| @Accessor on MappedRegistry.registrationInfos | `MappedRegistryAccessor.lithostitched$getRegistrationInfos` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MappedRegistry.registrationInfos: run `match`. |
| @Accessor on MappedRegistry.byValue | `MappedRegistryAccessor.getByValue` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MappedRegistry.byValue: run `match`. |
| mixin into MaterialRuleContext with no injectors (interfaces or @Shadow only) | `MaterialRuleContextMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Accessor on MobSpawnSettings.spawnsByCategory | `MobSpawnSettingsAccessor.getSpawnsByCategory` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MobSpawnSettings.spawnsByCategory: run `match`. |
| @Accessor on MobSpawnSettings.spawnsByCategory | `MobSpawnSettingsAccessor.setSpawnsByCategory` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MobSpawnSettings.spawnsByCategory: run `match`. |
| @Accessor on MultiNoiseBiomeSource.parameters | `MultiNoiseBiomeSourceAccessor.getParameters` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MultiNoiseBiomeSource.parameters: run `match`. |
| @Accessor on MultiNoiseBiomeSource.parameters | `MultiNoiseBiomeSourceAccessor.setParameters` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for MultiNoiseBiomeSource.parameters: run `match`. |
| @Accessor on NoiseBasedChunkGenerator.settings | `NoiseBasedChunkGeneratorAccessor.setSettings` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for NoiseBasedChunkGenerator.settings: run `match`. |
| @WrapOperation(INVOKE) on NoiseBasedChunkGenerator.spawnOriginalMobs at Lnet/minecraft/core/BlockPos;atY(I)Lnet/minecraft/core/BlockPos; | `NoiseBasedChunkGeneratorMixin.fixSampledBiomeY` | WRAP_OPERATION | needs-mapping | No Horizon function known for NoiseBasedChunkGenerator.spawnOriginalMobs: run `match`. The call it hooks (BlockPos.atY) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on NoiseBasedChunkGenerator.buildTerrain | `NoiseBasedChunkGeneratorMixin.fillBands` | HEAD | needs-mapping | No Horizon function known for NoiseBasedChunkGenerator.buildTerrain: run `match`. |
| @Accessor on NoiseGeneratorSettings.noiseRouter | `NoiseGeneratorSettingsAccessor.setNoiseRouter` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for NoiseGeneratorSettings.noiseRouter: run `match`. |
| @Accessor on NoiseGeneratorSettings.aquifers | `NoiseGeneratorSettingsAccessor.setAquifers` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for NoiseGeneratorSettings.aquifers: run `match`. |
| @Accessor on PlacedFeature.feature | `PlacedFeatureAccessor.setFeature` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for PlacedFeature.feature: run `match`. |
| @Redirect(INVOKE) on PoolAliasLookup.create at Lcom/google/common/collect/ImmutableMap$Builder;build()Lcom/google/common/collect/ImmutableMap; | `PoolAliasLookupMixin.lithostitched$allowDuplicateEntries` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for PoolAliasLookup.create: run `match`. The call it hooks (Builder.build) isn't mapped to a Rust function yet. |
| @Accessor on RandomState.random | `RandomStateAccessor.getRandom` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for RandomState.random: run `match`. |
| @Accessor on SinglePoolElement.template | `SinglePoolElementAccessor.getTemplate` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for SinglePoolElement.template: run `match`. |
| @Accessor on SinglePoolElement.processors | `SinglePoolElementAccessor.getProcessors` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for SinglePoolElement.processors: run `match`. |
| @Accessor on SinglePoolElement.processors | `SinglePoolElementAccessor.setProcessors` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for SinglePoolElement.processors: run `match`. |
| @Accessor on StructureProcessorList.list | `StructureProcessorListAccessor.setProcessors` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StructureProcessorList.list: run `match`. |
| @Accessor on StructureSet.structures | `StructureSetAccessor.setStructures` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StructureSet.structures: run `match`. |
| @Accessor on StructureTemplatePool.rawTemplates | `StructureTemplatePoolAccessor.getRawTemplates` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StructureTemplatePool.rawTemplates: run `match`. |
| @Accessor on StructureTemplatePool.templates | `StructureTemplatePoolAccessor.getVanillaTemplates` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StructureTemplatePool.templates: run `match`. |
| @Accessor on StructureTemplatePool.rawTemplates | `StructureTemplatePoolAccessor.setRawTemplates` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StructureTemplatePool.rawTemplates: run `match`. |
| @Accessor on StructureTemplatePool.templates | `StructureTemplatePoolAccessor.setVanillaTemplates` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for StructureTemplatePool.templates: run `match`. |
| mixin into StructureTemplatePool with no injectors (interfaces or @Shadow only) | `StructureTemplatePoolMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Accessor on Timeline.tracks | `TimelineAccessor.getTracks` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Timeline.tracks: run `match`. |
| @Accessor on Timeline.tracks | `TimelineAccessor.setTracks` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for Timeline.tracks: run `match`. |
| @Accessor on TreeFeature.decorators | `TreeFeatureAccessor.lithostitched$setDecorators` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for TreeFeature.decorators: run `match`. |
| mixin into MaterialSystem with no injectors (interfaces or @Shadow only) | `MaterialSystemMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| mixin into NoiseBasedChunkGenerator with no injectors (interfaces or @Shadow only) | `NoiseBasedChunkGeneratorMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Redirect(INVOKE) on MultiNoiseBiomeSource.<clinit> at Lcom/mojang/serialization/MapCodec;xmap(Ljava/util/function/Function;Ljava/util/function/Function;)Lcom/mojang/serialization/MapCodec; | `MNBSMixin.wrapCodec` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for MultiNoiseBiomeSource.<clinit>: run `match`. The call it hooks (MapCodec.xmap) isn't mapped to a Rust function yet. |
| @Redirect(INVOKE) on MultiNoiseBiomeSourceParameterList.<clinit> at Lcom/mojang/serialization/codecs/RecordCodecBuilder;create(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec; | `MNBSPLMixin.wrapCodec` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for MultiNoiseBiomeSourceParameterList.<clinit>: run `match`. The call it hooks (RecordCodecBuilder.create) isn't mapped to a Rust function yet. |
| @Accessor on ParameterList.values | `ParameterListAccessor.lithostitched$setValues` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ParameterList.values: run `match`. |
| @Accessor on ParameterList.index | `ParameterListAccessor.lithostitched$index` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for ParameterList.index: run `match`. |
| mixin into MinecraftServer with no injectors (interfaces or @Shadow only) | `ServerLifecycleHooksMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(INVOKE) on PendingRegistration.loadFromResource at Lcom/mojang/serialization/Decoder;parse(Lcom/mojang/serialization/DynamicOps;Ljava/lang/Object;)Lcom/mojang/serialization/DataResult; | `PendingRegistrationMixin.loadFromResource` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for PendingRegistration.loadFromResource: run `match`. The call it hooks (Decoder.parse) isn't mapped to a Rust function yet. |
| @ModifyExpressionValue(NEW) on ResourceManagerRegistryLoadTask.lambda$load$2 at net/minecraft/resources/RegistryLoadTask$PendingRegistration | `ResourceManagerRegistryLoadTaskMixin.load` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for ResourceManagerRegistryLoadTask.lambda$load$2: run `match`. |
| @WrapOperation(INVOKE) on DesertPyramidStructure.afterPlace at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `DesertPyramidStructureMixin.applyProcessorToDesertTempleSand` | WRAP_OPERATION on horizon_world::worldgen::structure::temple::<DesertPyramidPiece as LegacyPiece>::after_place | needs-mapping | The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on DesertPyramidStructure.placeSuspiciousSand at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `DesertPyramidStructureMixin.applyProcessorToDesertTempleSand` | WRAP_OPERATION | needs-mapping | No Horizon function known for DesertPyramidStructure.placeSuspiciousSand: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @ModifyReturnValue(RETURN) on EndCityPiece.makeSettings | `EndCityPieceMixin.addShipwreckProcessor` | RETURN | needs-mapping | No Horizon function known for EndCityPiece.makeSettings: run `match`. |
| @ModifyReturnValue(RETURN) on IglooPiece.makeSettings | `IglooPieceMixin.addShipwreckProcessor` | RETURN | needs-mapping | No Horizon function known for IglooPiece.makeSettings: run `match`. |
| @WrapOperation(INVOKE) on MineShaftCorridor.postProcess at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `MineshaftCorridorMixin.applyProcessorToSpawner` | WRAP_OPERATION | needs-mapping | No Horizon function known for MineShaftCorridor.postProcess: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on MineShaftCorridor.fillPillarDownOrChainUp at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `MineshaftCorridorMixin.applyProcessorToColumn` | WRAP_OPERATION | needs-mapping | No Horizon function known for MineShaftCorridor.fillPillarDownOrChainUp: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on MineShaftCorridor.fillColumnDown at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `MineshaftCorridorMixin.applyProcessorToColumn` | WRAP_OPERATION | needs-mapping | No Horizon function known for MineShaftCorridor.fillColumnDown: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on MineShaftCorridor.fillPillarDownOrChainUp at Lnet/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftCorridor;fillColumnBetween(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/core/BlockPos$MutableBlockPos;II)V | `MineshaftCorridorMixin.applyProcessorToChains` | WRAP_OPERATION | needs-mapping | No Horizon function known for MineShaftCorridor.fillPillarDownOrChainUp: run `match`. The call it hooks (MineShaftCorridor.fillColumnBetween) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on MineShaftCorridor.placeDoubleLowerOrUpperSupport at Lnet/minecraft/world/level/block/state/BlockState;is(Ljava/lang/Object;)Z | `MineshaftCorridorMixin.useMineshaftExtensionsTag` | WRAP_OPERATION | needs-mapping | No Horizon function known for MineShaftCorridor.placeDoubleLowerOrUpperSupport: run `match`. The call it hooks (BlockState.is) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on MineShaftPiece.setPlanksBlock at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `MineshaftPieceMixin.applyProcessor` | WRAP_OPERATION | needs-mapping | No Horizon function known for MineShaftPiece.setPlanksBlock: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on MonsterThrone.postProcess at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `MonsterThroneMixin.applyProcessorToSpawner` | WRAP_OPERATION | needs-mapping | No Horizon function known for MonsterThrone.postProcess: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @ModifyReturnValue(RETURN) on NetherFossilPiece.makeSettings | `NetherFossilPieceMixin.addShipwreckProcessor` | RETURN | needs-mapping | No Horizon function known for NetherFossilPiece.makeSettings: run `match`. |
| @WrapOperation(INVOKE) on NetherFossilPiece.placeDriedGhast at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `NetherFossilPieceMixin.applyProcessorToDriedGhast` | WRAP_OPERATION | needs-mapping | No Horizon function known for NetherFossilPiece.placeDriedGhast: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @ModifyReturnValue(RETURN) on OceanRuinPiece.makeSettings | `OceanRuinPieceMixin.addShipwreckProcessor` | RETURN | needs-mapping | No Horizon function known for OceanRuinPiece.makeSettings: run `match`. |
| @WrapOperation(INVOKE) on PortalRoom.postProcess at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `PortalRoomMixin.applyProcessorToSpawner` | WRAP_OPERATION | needs-mapping | No Horizon function known for PortalRoom.postProcess: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @ModifyReturnValue(RETURN) on RuinedPortalPiece.makeSettings | `RuinedPortalPieceMixin.addRuinedPortalProcessor` | RETURN | needs-mapping | No Horizon function known for RuinedPortalPiece.makeSettings: run `match`. |
| @WrapOperation(INVOKE) on RuinedPortalPiece.maybeAddVines at Lnet/minecraft/world/level/LevelAccessor;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z | `RuinedPortalPieceMixin.applyProcessorToExtraBits` | WRAP_OPERATION | needs-mapping | No Horizon function known for RuinedPortalPiece.maybeAddVines: run `match`. The call it hooks (LevelAccessor.setBlockAndUpdate) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on RuinedPortalPiece.maybeAddLeavesAbove at Lnet/minecraft/world/level/LevelAccessor;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z | `RuinedPortalPieceMixin.applyProcessorToExtraBits` | WRAP_OPERATION | needs-mapping | No Horizon function known for RuinedPortalPiece.maybeAddLeavesAbove: run `match`. The call it hooks (LevelAccessor.setBlockAndUpdate) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on RuinedPortalPiece.placeNetherrackOrMagma at Lnet/minecraft/world/level/LevelAccessor;setBlockAndUpdate(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;)Z | `RuinedPortalPieceMixin.applyProcessorToExtraBits` | WRAP_OPERATION | needs-mapping | No Horizon function known for RuinedPortalPiece.placeNetherrackOrMagma: run `match`. The call it hooks (LevelAccessor.setBlockAndUpdate) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on RuinedPortalPiece.maybeAddLeavesAbove at Lnet/minecraft/world/level/block/state/BlockState;is(Ljava/lang/Object;)Z | `RuinedPortalPieceMixin.applyLeavesExtensionTag` | WRAP_OPERATION | needs-mapping | No Horizon function known for RuinedPortalPiece.maybeAddLeavesAbove: run `match`. The call it hooks (BlockState.is) isn't mapped to a Rust function yet. |
| @ModifyReturnValue(RETURN) on ShipwreckPiece.makeSettings | `ShipwreckPieceMixin.addShipwreckProcessor` | RETURN | needs-mapping | No Horizon function known for ShipwreckPiece.makeSettings: run `match`. |
| @WrapOperation(INVOKE) on StructurePiece.placeBlock at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `StructurePieceMixin.applyProcessor` | WRAP_OPERATION | needs-mapping | No Horizon function known for StructurePiece.placeBlock: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on StructurePiece.fillColumnDown at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `StructurePieceMixin.applyProcessor` | WRAP_OPERATION | needs-mapping | No Horizon function known for StructurePiece.fillColumnDown: run `match`. The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on StructurePiece.createChest at Lnet/minecraft/world/level/ServerLevelAccessor;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `StructurePieceMixin.applyProcessor` | WRAP_OPERATION on horizon_world::worldgen::structure::legacy::PieceFrame::create_chest | needs-mapping | The call it hooks (ServerLevelAccessor.setBlock) isn't mapped to a Rust function yet. |
| @Inject(INVOKE) on StructureStart.placeInChunk at Lnet/minecraft/world/level/levelgen/structure/StructurePiece;postProcess(Lnet/minecraft/world/level/WorldGenLevel;Lnet/minecraft/world/level/StructureManager;Lnet/minecraft/world/level/chunk/ChunkGenerator;Lnet/minecraft/util/RandomSource;Lnet/minecraft/world/level/levelgen/structure/BoundingBox;Lnet/minecraft/world/level/ChunkPos;Lnet/minecraft/core/BlockPos;)V | `StructureStartMixin.storeReferencePos` | BEFORE_CALL on horizon_world::worldgen::structure::start::place_in_chunk | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. The call it hooks (StructurePiece.postProcess) isn't mapped to a Rust function yet. |
| @ModifyReturnValue(RETURN) on WoodlandMansionPiece.makeSettings | `WoodlandMansionPieceMixin.addShipwreckProcessor` | RETURN | needs-mapping | No Horizon function known for WoodlandMansionPiece.makeSettings: run `match`. |
| @WrapOperation(INVOKE) on WoodlandMansionStructure.afterPlace at Lnet/minecraft/world/level/WorldGenLevel;setBlock(Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/state/BlockState;I)Z | `WoodlandMansionStructureMixin.applyProcessorToWoodlandMansionExtension` | WRAP_OPERATION on horizon_world::worldgen::structure::mansion::after_place | needs-mapping | The call it hooks (WorldGenLevel.setBlock) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on NetherFossilStructure.findGenerationPoint at Ljava/util/Optional;of(Ljava/lang/Object;)Ljava/util/Optional; | `NetherFossilStructureMixin.useTemplateList` | WRAP_OPERATION | needs-mapping | No Horizon function known for NetherFossilStructure.findGenerationPoint: run `match`. The call it hooks (Optional.of) isn't mapped to a Rust function yet. |
| @Redirect(INVOKE) on RuinedPortalStructure.findGenerationPoint at Lnet/minecraft/resources/Identifier;withDefaultNamespace(Ljava/lang/String;)Lnet/minecraft/resources/Identifier; | `RuinedPortalStructureMixin.useStandardTemplateList` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for RuinedPortalStructure.findGenerationPoint: run `match`. The call it hooks (Identifier.withDefaultNamespace) isn't mapped to a Rust function yet. |
| @Redirect(INVOKE) on RuinedPortalStructure.findGenerationPoint at Lnet/minecraft/resources/Identifier;withDefaultNamespace(Ljava/lang/String;)Lnet/minecraft/resources/Identifier; | `RuinedPortalStructureMixin.useGiantTemplateList` | WRAP_OPERATION | needs-mapping | A redirect is a WRAP_OPERATION that doesn't call `original`. No Horizon function known for RuinedPortalStructure.findGenerationPoint: run `match`. The call it hooks (Identifier.withDefaultNamespace) isn't mapped to a Rust function yet. |
| @WrapOperation(INVOKE) on ShipwreckStructure.generatePieces at Lnet/minecraft/world/level/levelgen/structure/structures/ShipwreckPieces;addRandomPiece(Lnet/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager;Lnet/minecraft/core/BlockPos;Lnet/minecraft/world/level/block/Rotation;Lnet/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder;Lnet/minecraft/util/RandomSource;Z)Lnet/minecraft/world/level/levelgen/structure/structures/ShipwreckPieces$ShipwreckPiece; | `ShipwreckStructureMixin.useTemplateList` | WRAP_OPERATION | needs-mapping | No Horizon function known for ShipwreckStructure.generatePieces: run `match`. The call it hooks (ShipwreckPieces.addRandomPiece) isn't mapped to a Rust function yet. |
| mixin into FloorRoomCollection with no injectors (interfaces or @Shadow only) | `FloorRoomCollectionMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @WrapOperation(NEW) on MansionPiecePlacer.createMansion at net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection | `PiecePlacerMixin.addFirstFloorRegistries` | WRAP_OPERATION | needs-mapping | No Horizon function known for MansionPiecePlacer.createMansion: run `match`. |
| @WrapOperation(NEW) on MansionPiecePlacer.createMansion at net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection | `PiecePlacerMixin.addSecondFloorRegistries` | WRAP_OPERATION | needs-mapping | No Horizon function known for MansionPiecePlacer.createMansion: run `match`. |
| @WrapOperation(NEW) on MansionPiecePlacer.createMansion at net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$ThirdFloorRoomCollection | `PiecePlacerMixin.addThirdFloorRegistries` | WRAP_OPERATION | needs-mapping | No Horizon function known for MansionPiecePlacer.createMansion: run `match`. |
| @Inject(HEAD) on WoodlandMansionPiece.makeLocation | `WoodlandMansionPieceMixin.convertStringToId` | HEAD | needs-mapping | No Horizon function known for WoodlandMansionPiece.makeLocation: run `match`. |
| @Inject(HEAD) on WoodlandMansionStructure.generatePieces | `WoodlandMansionStructureMixin.injectRegistriesForTemplateList` | HEAD | needs-mapping | No Horizon function known for WoodlandMansionStructure.generatePieces: run `match`. |
| @Inject(HEAD) on FirstFloorRoomCollection.get1x1 | `FirstFloorMixin.use1x1TemplateList` | HEAD | needs-mapping | No Horizon function known for FirstFloorRoomCollection.get1x1: run `match`. |
| @Inject(HEAD) on FirstFloorRoomCollection.get1x1Secret | `FirstFloorMixin.use1x1SecretTemplateList` | HEAD | needs-mapping | No Horizon function known for FirstFloorRoomCollection.get1x1Secret: run `match`. |
| @Inject(HEAD) on FirstFloorRoomCollection.get1x2SideEntrance | `FirstFloorMixin.use1x2SideTemplateList` | HEAD | needs-mapping | No Horizon function known for FirstFloorRoomCollection.get1x2SideEntrance: run `match`. |
| @Inject(HEAD) on FirstFloorRoomCollection.get1x2FrontEntrance | `FirstFloorMixin.use1x2FrontTemplateList` | HEAD | needs-mapping | No Horizon function known for FirstFloorRoomCollection.get1x2FrontEntrance: run `match`. |
| @Inject(HEAD) on FirstFloorRoomCollection.get1x2Secret | `FirstFloorMixin.use1x2SecretTemplateList` | HEAD | needs-mapping | No Horizon function known for FirstFloorRoomCollection.get1x2Secret: run `match`. |
| @Inject(HEAD) on FirstFloorRoomCollection.get2x2 | `FirstFloorMixin.use2x2TemplateList` | HEAD | needs-mapping | No Horizon function known for FirstFloorRoomCollection.get2x2: run `match`. |
| @Inject(HEAD) on FirstFloorRoomCollection.get2x2Secret | `FirstFloorMixin.use2x2SecretTemplateList` | HEAD | needs-mapping | No Horizon function known for FirstFloorRoomCollection.get2x2Secret: run `match`. |
| @Inject(HEAD) on SecondFloorRoomCollection.get1x1 | `SecondFloorMixin.use1x1TemplateList` | HEAD | needs-mapping | No Horizon function known for SecondFloorRoomCollection.get1x1: run `match`. |
| @Inject(HEAD) on SecondFloorRoomCollection.get1x1Secret | `SecondFloorMixin.use1x1SecretTemplateList` | HEAD | needs-mapping | No Horizon function known for SecondFloorRoomCollection.get1x1Secret: run `match`. |
| @Inject(HEAD) on SecondFloorRoomCollection.get1x2SideEntrance | `SecondFloorMixin.use1x2SideTemplateList` | HEAD | needs-mapping | No Horizon function known for SecondFloorRoomCollection.get1x2SideEntrance: run `match`. |
| @Inject(HEAD) on SecondFloorRoomCollection.get1x2FrontEntrance | `SecondFloorMixin.use1x2FrontTemplateList` | HEAD | needs-mapping | No Horizon function known for SecondFloorRoomCollection.get1x2FrontEntrance: run `match`. |
| @Inject(HEAD) on SecondFloorRoomCollection.get1x2Secret | `SecondFloorMixin.use1x2SecretTemplateList` | HEAD | needs-mapping | No Horizon function known for SecondFloorRoomCollection.get1x2Secret: run `match`. |
| @Inject(HEAD) on SecondFloorRoomCollection.get2x2 | `SecondFloorMixin.use2x2TemplateList` | HEAD | needs-mapping | No Horizon function known for SecondFloorRoomCollection.get2x2: run `match`. |
| @Inject(HEAD) on SecondFloorRoomCollection.get2x2Secret | `SecondFloorMixin.use2x2SecretTemplateList` | HEAD | needs-mapping | No Horizon function known for SecondFloorRoomCollection.get2x2Secret: run `match`. |
| mixin into ThirdFloorRoomCollection with no injectors (interfaces or @Shadow only) | `ThirdFloorMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyExpressionValue(INVOKE) on Biome.<clinit> at Lcom/mojang/serialization/codecs/RecordCodecBuilder;create(Ljava/util/function/Function;)Lcom/mojang/serialization/Codec; | `BiomeMixin.addTimelineField` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for Biome.<clinit>: run `match`. |
| @Inject(HEAD) on EnvironmentAttributeProbe.tick | `EnvironmentAttributeProbeMixin.clearTimelineWeights` | HEAD | needs-mapping | No Horizon function known for EnvironmentAttributeProbe.tick: run `match`. |
| @Inject(TAIL) on EnvironmentAttributeProbe.lambda$tick$0 | `EnvironmentAttributeProbeMixin.collectTimelineWeights` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for EnvironmentAttributeProbe.lambda$tick$0: run `match`. |
| mixin into Builder with no injectors (interfaces or @Shadow only) | `EnvironmentAttributeSystemBuilderMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(TAIL) on EnvironmentAttributeSystem.addDynamicLayers | `EnvironmentAttributeSystemMixin.addBiomeTimelines` | RETURN | needs-mapping | Horizon has no TAIL: RETURN runs at every exit, early ones too. No Horizon function known for EnvironmentAttributeSystem.addDynamicLayers: run `match`. |
| @ModifyExpressionValue(MIXINEXTRAS:EXPRESSION) on ValueSampler.computeValuePositional | `EnvironmentAttributeSystemValueSamplerMixin.removeBiomeTimelineLayers` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for ValueSampler.computeValuePositional: run `match`. |
| @ModifyExpressionValue(MIXINEXTRAS:EXPRESSION) on ValueSampler.computeValueNotPositional | `EnvironmentAttributeSystemValueSamplerMixin.removeBiomeTimelineLayers` | MODIFY_EXPRESSION_VALUE | needs-mapping | Selects an exact Rust expression: pick it in the target function. No Horizon function known for ValueSampler.computeValueNotPositional: run `match`. |
| @WrapOperation(INVOKE) on ValueSampler.computeValuePositional at Lnet/minecraft/world/attribute/EnvironmentAttribute;sanitizeValue(Ljava/lang/Object;)Ljava/lang/Object; | `EnvironmentAttributeSystemValueSamplerMixin.applyBiomeTimelineLayers` | WRAP_OPERATION | needs-mapping | No Horizon function known for ValueSampler.computeValuePositional: run `match`. The call it hooks (EnvironmentAttribute.sanitizeValue) isn't mapped to a Rust function yet. |
| mixin into SpatialAttributeInterpolator with no injectors (interfaces or @Shadow only) | `SpatialAttributeInterpolatorMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @ModifyReturnValue(RETURN) on VillagerType.byBiome | `VillagerTypeMixin.useBiomeTags` | RETURN | needs-mapping | No Horizon function known for VillagerType.byBiome: run `match`. |
| @Inject(INVOKE) on DedicatedServer.initServer at Lnet/minecraft/server/dedicated/DedicatedServer;loadLevel()V | `DedicatedServerMixin.initServer` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for DedicatedServer.initServer: run `match`. The call it hooks (DedicatedServer.loadLevel) isn't mapped to a Rust function yet. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends BiomeSource | `InjectorBiomeSource` | needs-translation | No mapping for this superclass yet. |
| extends StateTestingPredicate | `BlockStatePredicate` | needs-translation | No mapping for this superclass yet. |
| extends LazyXZCondition | `SlopeCondition$1` | needs-translation | No mapping for this superclass yet. |
| extends StructurePoolElement | `DelegatingPoolElement` | needs-translation | No mapping for this superclass yet. |
| extends Structure | `AlternateJigsawStructure` | needs-translation | No mapping for this superclass yet. |
| extends Structure | `DelegatingStructure` | needs-translation | No mapping for this superclass yet. |
| extends MegaJungleFoliagePlacer | `BranchedMegaJungleFoliagePlacer` | needs-translation | No mapping for this superclass yet. |
| extends MegaPineFoliagePlacer | `BranchedMegaPineFoliagePlacer` | needs-translation | No mapping for this superclass yet. |
| extends MangroveRootPlacer | `LargeMangroveRootPlacer` | needs-translation | No mapping for this superclass yet. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `lithostitched.ct` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 0 asset files, 80 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

26 of 814 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/sounds/MusicManager#tick` | horizon_client::music::MusicManager::tick (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK` | content |
| `net/minecraft/server/level/ServerLevel#scheduleTick` | horizon_world::level::Behaviours::schedule_tick (suggested) |
| `net/minecraft/util/ExtraCodecs#nonEmptyList` | horizon_command::component::non_empty_list (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/util/Mth#lerp` | horizon_world::stepped_interpolation::lerp (suggested) |
| `net/minecraft/world/entity/EntityType#getCategory` | horizon_client::mobs::Mob::category (suggested) |
| `net/minecraft/world/level/WorldGenLevel#getSeed` | horizon_world::worldgen::structure::legacy::seed (suggested) |
| `net/minecraft/world/level/block/Block#withPropertiesOf` | horizon_world::worldgen::structure::processor::with_properties_of (suggested) |
| `net/minecraft/world/level/block/Rotation#getRandom` | horizon_world::worldgen::structure::Rotation::random (suggested) |
| `net/minecraft/world/level/chunk/ChunkGenerator#getBaseColumn` | horizon_world::worldgen::generator::Pipeline<R>::base_column (suggested) |
| `net/minecraft/world/level/chunk/ChunkGenerator#getFirstFreeHeight` | horizon_world::worldgen::structure::start_checks::first_free_height (suggested) |
| `net/minecraft/world/level/levelgen/structure/Structure#afterPlace` | horizon_world::worldgen::structure::legacy::after_place (suggested) |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#createChest` | horizon_world::worldgen::structure::legacy::PieceFrame::create_chest (suggested) |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#reorient` | horizon_world::worldgen::structure::legacy::reorient (suggested) |
| `net/minecraft/world/level/levelgen/structure/StructureStart#placeInChunk` | horizon_world::worldgen::structure::start::place_in_chunk (suggested) |
| `net/minecraft/world/level/levelgen/structure/structures/DesertPyramidStructure#afterPlace` | horizon_world::worldgen::structure::temple::<DesertPyramidPiece as LegacyPiece>::after_place (suggested) |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionStructure#afterPlace` | horizon_world::worldgen::structure::mansion::after_place (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/PosRuleTest#test` | horizon_world::worldgen::structure::processor::PosRule::test (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessor#processBlock` | horizon_world::worldgen::structure::processor::process_block (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#placeInWorld` | horizon_world::worldgen::structure::processor::place_in_world (suggested) |
| `net/minecraft/world/level/levelgen/structure/templatesystem/rule/blockentity/RuleBlockEntityModifier#apply` | horizon_world::worldgen::structure::processor::Modifier::apply (suggested) |
| `net/minecraft/world/phys/AABB#intersects` | horizon_core::geometry::Aabb::intersects_corners (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/google/common/collect/ImmutableMap$Builder#build` | method | 1 |
| `com/mojang/serialization/Decoder#parse` | method | 1 |
| `com/mojang/serialization/MapCodec#xmap` | method | 1 |
| `com/mojang/serialization/codecs/RecordCodecBuilder#create` | method | 1 |
| `java/util/List#add` | method | 1 |
| `java/util/Optional#of` | method | 1 |
| `net/minecraft/SharedConstants#getCurrentVersion` | method | 1 |
| `net/minecraft/WorldVersion#packVersion` | method | 1 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate#<init>` | method | 1 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate#CODEC` | field | 2 |
| `net/minecraft/advancements/predicates/StatePropertiesPredicate#matches` | method | 2 |
| `net/minecraft/client/Minecraft#getCameraEntity` | method | 1 |
| `net/minecraft/client/Minecraft#getMusicVolume` | method | 1 |
| `net/minecraft/client/Minecraft#level` | field | 2 |
| `net/minecraft/client/gui/components/debug/DebugScreenDisplayer#addLine` | method | 1 |
| `net/minecraft/client/gui/components/debug/DebugScreenEntries#register` | method | 1 |
| `net/minecraft/client/multiplayer/ClientLevel#registryAccess` | method | 1 |
| `net/minecraft/client/server/IntegratedServer#initServer` | method | 1 |
| `net/minecraft/client/server/IntegratedServer#loadLevel` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 10 |
| `net/minecraft/core/BlockPos#CODEC` | field | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 2 |
| `net/minecraft/core/BlockPos#above` | method | 2 |
| `net/minecraft/core/BlockPos#atY` | method | 3 |
| `net/minecraft/core/BlockPos#below` | method | 2 |
| `net/minecraft/core/BlockPos#getX` | method | 20 |
| `net/minecraft/core/BlockPos#getY` | method | 22 |
| `net/minecraft/core/BlockPos#getZ` | method | 20 |
| `net/minecraft/core/BlockPos#mutable` | method | 4 |
| `net/minecraft/core/BlockPos#offset` | method | 11 |
| `net/minecraft/core/BlockPos#relative` | method | 2 |
| `net/minecraft/core/BlockPos#subtract` | method | 3 |
| `net/minecraft/core/BlockPos#toShortString` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#above` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 2 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#immutable` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 3 |
| `net/minecraft/core/BlockPos$MutableBlockPos#relative` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#set` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setX` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#setZ` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#byNameCodec` | method | 4 |
| `net/minecraft/core/DefaultedRegistry#registryKeySet` | method | 1 |
| `net/minecraft/core/Direction#DOWN` | field | 2 |
| `net/minecraft/core/Direction#EAST` | field | 1 |
| `net/minecraft/core/Direction#NORTH` | field | 1 |
| `net/minecraft/core/Direction#SOUTH` | field | 1 |
| `net/minecraft/core/Direction#UP` | field | 2 |
| `net/minecraft/core/Direction#WEST` | field | 1 |
| `net/minecraft/core/Direction#getStepY` | method | 1 |
| `net/minecraft/core/Direction#getUnitVec3i` | method | 1 |
| `net/minecraft/core/Direction#values` | method | 1 |
| `net/minecraft/core/Direction$Axis#X` | field | 3 |
| `net/minecraft/core/Direction$Axis#Z` | field | 3 |
| `net/minecraft/core/Direction$Axis#getNegative` | method | 1 |
| `net/minecraft/core/Direction$Plane#HORIZONTAL` | field | 1 |
| `net/minecraft/core/Direction$Plane#stream` | method | 1 |
| `net/minecraft/core/Holder#direct` | method | 7 |
| `net/minecraft/core/Holder#is` | method | 4 |
| `net/minecraft/core/Holder#unwrapKey` | method | 6 |
| `net/minecraft/core/Holder#value` | method | 55 |
| `net/minecraft/core/Holder$Direct#value` | method | 1 |
| `net/minecraft/core/Holder$Reference#isBound` | method | 1 |
| `net/minecraft/core/Holder$Reference#key` | method | 4 |
| `net/minecraft/core/Holder$Reference#value` | method | 10 |
| `net/minecraft/core/HolderGetter#get` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#get` | method | 1 |
| `net/minecraft/core/HolderLookup$RegistryLookup#listElements` | method | 1 |
| `net/minecraft/core/HolderSet#contains` | method | 6 |
| `net/minecraft/core/HolderSet#direct` | method | 14 |
| `net/minecraft/core/HolderSet#get` | method | 2 |
| `net/minecraft/core/HolderSet#iterator` | method | 5 |
| `net/minecraft/core/HolderSet#size` | method | 1 |
| `net/minecraft/core/HolderSet#stream` | method | 6 |
| `net/minecraft/core/IdMap#iterator` | method | 1 |
| `net/minecraft/core/MappedRegistry#byValue` | method | 1 |
| `net/minecraft/core/MappedRegistry#registrationInfos` | method | 1 |
| `net/minecraft/core/QuartPos#fromBlock` | method | 4 |
| `net/minecraft/core/QuartPos#toBlock` | method | 1 |
| `net/minecraft/core/RegistrationInfo#<init>` | method | 1 |
| `net/minecraft/core/RegistrationInfo#lifecycle` | method | 1 |
| `net/minecraft/core/Registry#asHolderIdMap` | method | 1 |
| `net/minecraft/core/Registry#byNameCodec` | method | 8 |
| `net/minecraft/core/Registry#entrySet` | method | 1 |
| `net/minecraft/core/Registry#get` | method | 1 |
| `net/minecraft/core/Registry#getOptional` | method | 8 |
| `net/minecraft/core/Registry#getOrThrow` | method | 2 |
| `net/minecraft/core/Registry#getTagOrEmpty` | method | 1 |
| `net/minecraft/core/Registry#listElements` | method | 9 |
| `net/minecraft/core/Registry#registrationInfo` | method | 1 |
| `net/minecraft/core/Registry#stream` | method | 2 |
| `net/minecraft/core/RegistryAccess#createSerializationContext` | method | 2 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 15 |
| `net/minecraft/core/SectionPos#sectionRelative` | method | 1 |
| `net/minecraft/core/Vec3i#<init>` | method | 2 |
| `net/minecraft/core/Vec3i#ZERO` | field | 1 |
| `net/minecraft/core/Vec3i#get` | method | 1 |
| `net/minecraft/core/Vec3i#getX` | method | 1 |
| `net/minecraft/core/Vec3i#getY` | method | 2 |
| `net/minecraft/core/Vec3i#getZ` | method | 1 |
| `net/minecraft/core/Vec3i#multiply` | method | 1 |
| `net/minecraft/core/Vec3i#offsetCodec` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#BIOME_SOURCE` | field | 2 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_PREDICATE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#BLOCK_STATE_PROVIDER_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#DENSITY_FUNCTION_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#ENTITY_TYPE` | field | 3 |
| `net/minecraft/core/registries/BuiltInRegistries#ENVIRONMENT_ATTRIBUTE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#FEATURE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#FOLIAGE_PLACER_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MATERIAL_CONDITION_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#MATERIAL_RULE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#PLACEMENT_MODIFIER_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#POOL_ALIAS_BINDING_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#REGISTRY` | field | 3 |
| `net/minecraft/core/registries/BuiltInRegistries#ROOT_PLACER_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#RULE_BLOCK_ENTITY_MODIFIER` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#STRUCTURE_POOL_ELEMENT` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#STRUCTURE_PROCESSOR` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#STRUCTURE_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#VILLAGER_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#BIOME` | field | 15 |
| `net/minecraft/core/registries/Registries#BIOME_SOURCE` | field | 1 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 3 |
| `net/minecraft/core/registries/Registries#DENSITY_FUNCTION` | field | 2 |
| `net/minecraft/core/registries/Registries#DIMENSION` | field | 2 |
| `net/minecraft/core/registries/Registries#DIMENSION_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ENTITY_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#ENVIRONMENT_ATTRIBUTE` | field | 1 |
| `net/minecraft/core/registries/Registries#FEATURE` | field | 2 |
| `net/minecraft/core/registries/Registries#LEVEL_STEM` | field | 2 |
| `net/minecraft/core/registries/Registries#LOOT_TABLE` | field | 1 |
| `net/minecraft/core/registries/Registries#MATERIAL_RULE` | field | 1 |
| `net/minecraft/core/registries/Registries#MULTI_NOISE_BIOME_SOURCE_PARAMETER_LIST` | field | 1 |
| `net/minecraft/core/registries/Registries#NOISE` | field | 2 |
| `net/minecraft/core/registries/Registries#PLACED_FEATURE` | field | 2 |
| `net/minecraft/core/registries/Registries#PROCESSOR_LIST` | field | 7 |
| `net/minecraft/core/registries/Registries#STRUCTURE` | field | 3 |
| `net/minecraft/core/registries/Registries#STRUCTURE_SET` | field | 2 |
| `net/minecraft/core/registries/Registries#TEMPLATE_POOL` | field | 6 |
| `net/minecraft/core/registries/Registries#TIMELINE` | field | 2 |
| `net/minecraft/core/registries/Registries#VILLAGER_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#levelStemToLevel` | method | 1 |
| `net/minecraft/core/registries/Registries#levelToLevelStem` | method | 2 |
| `net/minecraft/core/registries/codec/RegistryCodecs#holder` | method | 5 |
| `net/minecraft/core/registries/codec/RegistryCodecs#holderSet` | method | 8 |
| `net/minecraft/data/worldgen/Pools#EMPTY` | field | 1 |
| `net/minecraft/data/worldgen/placement/NetherPlacements#LARGE_BASALT_COLUMNS` | field | 1 |
| `net/minecraft/gametest/framework/GameTestServer#initServer` | method | 1 |
| `net/minecraft/gametest/framework/GameTestServer#loadLevel` | method | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 1 |
| `net/minecraft/nbt/NbtOps#INSTANCE` | field | 2 |
| `net/minecraft/network/codec/ByteBufCodecs#TAG` | field | 1 |
| `net/minecraft/network/codec/StreamCodec#composite` | method | 1 |
| `net/minecraft/network/protocol/common/custom/CustomPacketPayload$Type#<init>` | method | 1 |
| `net/minecraft/resources/Identifier#CODEC` | field | 8 |
| `net/minecraft/resources/Identifier#equals` | method | 2 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 2 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 1 |
| `net/minecraft/resources/Identifier#getPath` | method | 1 |
| `net/minecraft/resources/Identifier#hashCode` | method | 1 |
| `net/minecraft/resources/Identifier#parse` | method | 1 |
| `net/minecraft/resources/Identifier#toString` | method | 3 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 6 |
| `net/minecraft/resources/Identifier#withPrefix` | method | 2 |
| `net/minecraft/resources/RegistryLoadTask$PendingRegistration#loadFromResource` | method | 1 |
| `net/minecraft/resources/RegistryLoadTask$PendingRegistration#value` | method | 1 |
| `net/minecraft/resources/RegistryOps#retrieveGetter` | method | 1 |
| `net/minecraft/resources/ResourceKey#codec` | method | 9 |
| `net/minecraft/resources/ResourceKey#create` | method | 5 |
| `net/minecraft/resources/ResourceKey#createRegistryKey` | method | 1 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 12 |
| `net/minecraft/resources/ResourceKey#registryKey` | method | 1 |
| `net/minecraft/resources/ResourceManagerRegistryLoadTask#lambda$load$2` | method | 1 |
| `net/minecraft/server/MinecraftServer#getLevel` | method | 1 |
| `net/minecraft/server/MinecraftServer#getStructureTemplateManager` | method | 1 |
| `net/minecraft/server/MinecraftServer#getWorldGenSettings` | method | 1 |
| `net/minecraft/server/MinecraftServer#registryAccess` | method | 1 |
| `net/minecraft/server/dedicated/DedicatedServer#initServer` | method | 1 |
| `net/minecraft/server/dedicated/DedicatedServer#loadLevel` | method | 1 |
| `net/minecraft/server/level/ChunkMap#<init>` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#getGenerator` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#getLevel` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#randomState` | method | 3 |
| `net/minecraft/server/level/ServerLevel#dimension` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunk` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 3 |
| `net/minecraft/server/level/ServerLevel#getServer` | method | 1 |
| `net/minecraft/server/level/ServerLevel#isLoaded` | method | 1 |
| `net/minecraft/server/level/ServerLevel#players` | method | 1 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 1 |
| `net/minecraft/server/level/ServerLevel#structureManager` | method | 2 |
| `net/minecraft/server/level/ServerPlayer#blockPosition` | method | 1 |
| `net/minecraft/server/level/WorldGenRegion#getChunk` | method | 1 |
| `net/minecraft/server/level/WorldGenRegion#registryAccess` | method | 1 |
| `net/minecraft/server/network/ServerGamePacketListenerImpl#getPlayer` | method | 1 |
| `net/minecraft/server/packs/PackType#SERVER_DATA` | field | 1 |
| `net/minecraft/server/packs/metadata/pack/PackFormat#major` | method | 1 |
| `net/minecraft/tags/BiomeTags#IS_OVERWORLD` | field | 1 |
| `net/minecraft/tags/BlockTags#BASE_STONE_OVERWORLD` | field | 1 |
| `net/minecraft/tags/BlockTags#FEATURES_CANNOT_REPLACE` | field | 1 |
| `net/minecraft/tags/TagKey#create` | method | 5 |
| `net/minecraft/util/ExtraCodecs#NON_NEGATIVE_INT` | field | 5 |
| `net/minecraft/util/ExtraCodecs#PATTERN` | field | 1 |
| `net/minecraft/util/ExtraCodecs#POSITIVE_INT` | field | 4 |
| `net/minecraft/util/ExtraCodecs#compactListCodec` | method | 4 |
| `net/minecraft/util/ExtraCodecs#intRange` | method | 2 |
| `net/minecraft/util/InclusiveRange#<init>` | method | 12 |
| `net/minecraft/util/InclusiveRange#INT` | field | 1 |
| `net/minecraft/util/InclusiveRange#codec` | method | 1 |
| `net/minecraft/util/InclusiveRange#isValueInRange` | method | 15 |
| `net/minecraft/util/InclusiveRange#maxInclusive` | method | 8 |
| `net/minecraft/util/InclusiveRange#minInclusive` | method | 5 |
| `net/minecraft/util/Interval#INFINITE` | field | 1 |
| `net/minecraft/util/Interval#of` | method | 3 |
| `net/minecraft/util/Interval#ofExact` | method | 1 |
| `net/minecraft/util/Mth#cos` | method | 2 |
| `net/minecraft/util/Mth#length` | method | 2 |
| `net/minecraft/util/Mth#randomBetween` | method | 2 |
| `net/minecraft/util/Mth#randomBetweenInclusive` | method | 1 |
| `net/minecraft/util/Mth#sin` | method | 3 |
| `net/minecraft/util/Mth#sqrt` | method | 1 |
| `net/minecraft/util/RandomSource#create` | method | 2 |
| `net/minecraft/util/RandomSource#forkPositional` | method | 3 |
| `net/minecraft/util/RandomSource#nextDouble` | method | 1 |
| `net/minecraft/util/RandomSource#nextFloat` | method | 6 |
| `net/minecraft/util/RandomSource#nextInt` | method | 7 |
| `net/minecraft/util/RandomSource#nextLong` | method | 1 |
| `net/minecraft/util/SequencedPriorityIterator#<init>` | method | 1 |
| `net/minecraft/util/SequencedPriorityIterator#add` | method | 1 |
| `net/minecraft/util/SequencedPriorityIterator#hasNext` | method | 1 |
| `net/minecraft/util/SequencedPriorityIterator#next` | method | 1 |
| `net/minecraft/util/StringRepresentable#fromEnum` | method | 11 |
| `net/minecraft/util/StringRepresentable#fromValues` | method | 3 |
| `net/minecraft/util/Util#memoize` | method | 1 |
| `net/minecraft/util/random/Weighted#value` | method | 1 |
| `net/minecraft/util/random/Weighted#weight` | method | 1 |
| `net/minecraft/util/random/WeightedList#builder` | method | 3 |
| `net/minecraft/util/random/WeightedList#codec` | method | 4 |
| `net/minecraft/util/random/WeightedList#getRandom` | method | 5 |
| `net/minecraft/util/random/WeightedList#getRandomOrThrow` | method | 1 |
| `net/minecraft/util/random/WeightedList#nonEmptyCodec` | method | 2 |
| `net/minecraft/util/random/WeightedList#of` | method | 2 |
| `net/minecraft/util/random/WeightedList#unwrap` | method | 1 |
| `net/minecraft/util/random/WeightedList$Builder#add` | method | 3 |
| `net/minecraft/util/random/WeightedList$Builder#build` | method | 3 |
| `net/minecraft/util/valueproviders/ConstantInt#of` | method | 2 |
| `net/minecraft/util/valueproviders/FloatProvider#sample` | method | 2 |
| `net/minecraft/util/valueproviders/FloatProviders#codec` | method | 1 |
| `net/minecraft/util/valueproviders/IntProvider#maxInclusive` | method | 1 |
| `net/minecraft/util/valueproviders/IntProvider#minInclusive` | method | 1 |
| `net/minecraft/util/valueproviders/IntProvider#sample` | method | 7 |
| `net/minecraft/util/valueproviders/IntProviders#POSITIVE_CODEC` | field | 1 |
| `net/minecraft/util/valueproviders/IntProviders#codec` | method | 8 |
| `net/minecraft/util/valueproviders/UniformInt#of` | method | 2 |
| `net/minecraft/world/attribute/AttributeType#spatialLerp` | method | 1 |
| `net/minecraft/world/attribute/AttributeType#stateChangeLerp` | method | 1 |
| `net/minecraft/world/attribute/AttributeTypes#BOOLEAN` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttribute#builder` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttribute#isPositional` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttribute#isSpatiallyInterpolated` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttribute#sanitizeValue` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttribute#type` | method | 2 |
| `net/minecraft/world/attribute/EnvironmentAttribute$Builder#build` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttribute$Builder#defaultValue` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttribute$Builder#syncable` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeLayer$TimeBased#applyTimeBased` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#CODEC` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#CODEC_ONLY_POSITIONAL` | field | 3 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#EMPTY` | field | 3 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#NETWORK_CODEC` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#builder` | method | 7 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#contains` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#get` | method | 4 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap#keySet` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap$Builder#build` | method | 7 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap$Builder#modify` | method | 3 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap$Builder#putAll` | method | 6 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap$Builder#set` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap$Entry#applyModifier` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap$Entry#argument` | method | 3 |
| `net/minecraft/world/attribute/EnvironmentAttributeMap$Entry#modifier` | method | 3 |
| `net/minecraft/world/attribute/EnvironmentAttributeProbe#lambda$tick$0` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeProbe#tick` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem#addDynamicLayers` | method | 2 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$Builder#addTimeBasedLayer` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$ValueSampler#computeValueNotPositional` | method | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributeSystem$ValueSampler#computeValuePositional` | method | 2 |
| `net/minecraft/world/attribute/EnvironmentAttributes#CODEC` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributes#FOG_END_DISTANCE` | field | 1 |
| `net/minecraft/world/attribute/EnvironmentAttributes#NATURAL_MOB_SPAWNS` | field | 3 |
| `net/minecraft/world/attribute/LerpFunction#apply` | method | 2 |
| `net/minecraft/world/attribute/modifier/MobSpawnSettingsModifier#overlay` | method | 1 |
| `net/minecraft/world/entity/Entity#blockPosition` | method | 1 |
| `net/minecraft/world/entity/EntityType#builtInRegistryHolder` | method | 1 |
| `net/minecraft/world/entity/EntityType#getKey` | method | 1 |
| `net/minecraft/world/entity/EntityTypes#PIG` | field | 2 |
| `net/minecraft/world/entity/EntityTypes#SKELETON` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#SPIDER` | field | 1 |
| `net/minecraft/world/entity/EntityTypes#ZOMBIE` | field | 1 |
| `net/minecraft/world/entity/MobCategory#MISC` | field | 1 |
| `net/minecraft/world/entity/npc/villager/VillagerType#byBiome` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMinBlockX` | method | 1 |
| `net/minecraft/world/level/ChunkPos#getMinBlockZ` | method | 1 |
| `net/minecraft/world/level/ChunkPos#x` | method | 1 |
| `net/minecraft/world/level/ChunkPos#z` | method | 1 |
| `net/minecraft/world/level/Level#clockManager` | method | 1 |
| `net/minecraft/world/level/Level#getBiomeManager` | method | 1 |
| `net/minecraft/world/level/Level#getUncachedNoiseBiome` | method | 1 |
| `net/minecraft/world/level/Level#registryAccess` | method | 2 |
| `net/minecraft/world/level/LevelAccessor#getRandom` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#isStateAtPosition` | method | 1 |
| `net/minecraft/world/level/LevelAccessor#setBlockAndUpdate` | method | 3 |
| `net/minecraft/world/level/LevelHeightAccessor#getMaxY` | method | 1 |
| `net/minecraft/world/level/LevelHeightAccessor#getMinY` | method | 1 |
| `net/minecraft/world/level/LevelHeightAccessor#isOutsideBuildHeight` | method | 1 |
| `net/minecraft/world/level/LevelReader#registryAccess` | method | 3 |
| `net/minecraft/world/level/NoiseColumn#getBlock` | method | 1 |
| `net/minecraft/world/level/ServerLevelAccessor#setBlock` | method | 1 |
| `net/minecraft/world/level/StructureManager#fillStartsForStructure` | method | 1 |
| `net/minecraft/world/level/StructureManager#getStructureWithPieceAt` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#ensureCanWrite` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#getBlockEntity` | method | 2 |
| `net/minecraft/world/level/WorldGenLevel#getBlockState` | method | 4 |
| `net/minecraft/world/level/WorldGenLevel#getChunkSource` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#getHeight` | method | 2 |
| `net/minecraft/world/level/WorldGenLevel#getLevel` | method | 5 |
| `net/minecraft/world/level/WorldGenLevel#isEmptyBlock` | method | 2 |
| `net/minecraft/world/level/WorldGenLevel#isOutsideBuildHeight` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#registryAccess` | method | 1 |
| `net/minecraft/world/level/WorldGenLevel#setBlock` | method | 16 |
| `net/minecraft/world/level/biome/Biome#<clinit>` | method | 1 |
| `net/minecraft/world/level/biome/Biome#CODEC` | field | 6 |
| `net/minecraft/world/level/biome/Biome#LIST_CODEC` | field | 5 |
| `net/minecraft/world/level/biome/Biome#attributes` | method | 1 |
| `net/minecraft/world/level/biome/Biome#climateSettings` | method | 2 |
| `net/minecraft/world/level/biome/Biome#generationSettings` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getAttributes` | method | 4 |
| `net/minecraft/world/level/biome/Biome#getGenerationSettings` | method | 4 |
| `net/minecraft/world/level/biome/Biome#specialEffects` | method | 2 |
| `net/minecraft/world/level/biome/Biome$ClimateSettings#<init>` | method | 1 |
| `net/minecraft/world/level/biome/Biome$ClimateSettings#downfall` | method | 2 |
| `net/minecraft/world/level/biome/Biome$ClimateSettings#hasPrecipitation` | method | 2 |
| `net/minecraft/world/level/biome/Biome$ClimateSettings#temperature` | method | 2 |
| `net/minecraft/world/level/biome/Biome$ClimateSettings#temperatureModifier` | method | 2 |
| `net/minecraft/world/level/biome/Biome$TemperatureModifier#CODEC` | field | 2 |
| `net/minecraft/world/level/biome/BiomeGenerationSettings#<init>` | method | 1 |
| `net/minecraft/world/level/biome/BiomeGenerationSettings#carvers` | method | 1 |
| `net/minecraft/world/level/biome/BiomeGenerationSettings#features` | method | 4 |
| `net/minecraft/world/level/biome/BiomeGenerationSettings#hasFeature` | method | 1 |
| `net/minecraft/world/level/biome/BiomeManager#getNoiseBiomeAtPosition` | method | 1 |
| `net/minecraft/world/level/biome/BiomeResolver#getNoiseBiome` | method | 3 |
| `net/minecraft/world/level/biome/BiomeSource#<init>` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSource#CODEC` | field | 1 |
| `net/minecraft/world/level/biome/BiomeSource#addDebugInfo` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSource#codec` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSource#createResolver` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSource#possibleBiomes` | method | 2 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects#dryFoliageColorOverride` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects#foliageColorOverride` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects#grassColorModifier` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects#grassColorOverride` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects#waterColor` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$Builder#<init>` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$Builder#build` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$Builder#dryFoliageColorOverride` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$Builder#foliageColorOverride` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$Builder#grassColorModifier` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$Builder#grassColorOverride` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$Builder#waterColor` | method | 1 |
| `net/minecraft/world/level/biome/BiomeSpecialEffects$GrassColorModifier#CODEC` | field | 1 |
| `net/minecraft/world/level/biome/Biomes#PALE_GARDEN` | field | 1 |
| `net/minecraft/world/level/biome/Climate$ParameterList#codec` | method | 3 |
| `net/minecraft/world/level/biome/Climate$ParameterList#findValue` | method | 1 |
| `net/minecraft/world/level/biome/Climate$ParameterList#index` | method | 1 |
| `net/minecraft/world/level/biome/Climate$ParameterList#values` | method | 5 |
| `net/minecraft/world/level/biome/Climate$RTree#create` | method | 1 |
| `net/minecraft/world/level/biome/Climate$Sampler#sample` | method | 1 |
| `net/minecraft/world/level/biome/Climate$TargetPoint#continentalness` | method | 1 |
| `net/minecraft/world/level/biome/Climate$TargetPoint#depth` | method | 1 |
| `net/minecraft/world/level/biome/Climate$TargetPoint#erosion` | method | 1 |
| `net/minecraft/world/level/biome/Climate$TargetPoint#humidity` | method | 1 |
| `net/minecraft/world/level/biome/Climate$TargetPoint#temperature` | method | 1 |
| `net/minecraft/world/level/biome/Climate$TargetPoint#weirdness` | method | 1 |
| `net/minecraft/world/level/biome/FeatureSorter#buildFeaturesPerStep` | method | 2 |
| `net/minecraft/world/level/biome/MobSpawnSettings#allSpawnCosts` | method | 2 |
| `net/minecraft/world/level/biome/MobSpawnSettings#definedCategories` | method | 3 |
| `net/minecraft/world/level/biome/MobSpawnSettings#getMobsInCategory` | method | 3 |
| `net/minecraft/world/level/biome/MobSpawnSettings#spawnsByCategory` | method | 2 |
| `net/minecraft/world/level/biome/MobSpawnSettings$Builder#<init>` | method | 3 |
| `net/minecraft/world/level/biome/MobSpawnSettings$Builder#addAllCosts` | method | 2 |
| `net/minecraft/world/level/biome/MobSpawnSettings$Builder#addAllSpawns` | method | 2 |
| `net/minecraft/world/level/biome/MobSpawnSettings$Builder#addSpawn` | method | 2 |
| `net/minecraft/world/level/biome/MobSpawnSettings$Builder#build` | method | 3 |
| `net/minecraft/world/level/biome/MobSpawnSettings$MobSpawnCost#CODEC` | field | 1 |
| `net/minecraft/world/level/biome/MobSpawnSettings$SpawnerData#count` | method | 1 |
| `net/minecraft/world/level/biome/MobSpawnSettings$SpawnerData#type` | method | 1 |
| `net/minecraft/world/level/biome/MultiNoiseBiomeSource#<clinit>` | method | 1 |
| `net/minecraft/world/level/biome/MultiNoiseBiomeSource#parameters` | method | 2 |
| `net/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList#<clinit>` | method | 1 |
| `net/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterList#parameters` | method | 2 |
| `net/minecraft/world/level/biome/MultiNoiseBiomeSourceParameterLists#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/block/Block#builtInRegistryHolder` | method | 2 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 3 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CAVE_AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#CHEST` | field | 1 |
| `net/minecraft/world/level/block/Blocks#COBBLESTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAVA` | field | 2 |
| `net/minecraft/world/level/block/Blocks#MOSSY_COBBLESTONE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#SPAWNER` | field | 1 |
| `net/minecraft/world/level/block/Blocks#VINE` | field | 1 |
| `net/minecraft/world/level/block/Blocks#WATER` | field | 1 |
| `net/minecraft/world/level/block/JigsawBlock#canAttach` | method | 1 |
| `net/minecraft/world/level/block/JigsawBlock#getFrontFacing` | method | 1 |
| `net/minecraft/world/level/block/Rotation#CODEC` | field | 1 |
| `net/minecraft/world/level/block/Rotation#NONE` | field | 1 |
| `net/minecraft/world/level/block/Rotation#getShuffled` | method | 1 |
| `net/minecraft/world/level/block/Rotation#rotate` | method | 1 |
| `net/minecraft/world/level/block/VineBlock#EAST` | field | 1 |
| `net/minecraft/world/level/block/VineBlock#NORTH` | field | 1 |
| `net/minecraft/world/level/block/VineBlock#SOUTH` | field | 1 |
| `net/minecraft/world/level/block/VineBlock#UP` | field | 1 |
| `net/minecraft/world/level/block/VineBlock#WEST` | field | 1 |
| `net/minecraft/world/level/block/VineBlock#getPropertyForFace` | method | 1 |
| `net/minecraft/world/level/block/VineBlock#isAcceptableNeighbour` | method | 1 |
| `net/minecraft/world/level/block/entity/BlockEntityTypes#CHEST` | field | 1 |
| `net/minecraft/world/level/block/entity/ChestBlockEntity#setLootTable` | method | 1 |
| `net/minecraft/world/level/block/entity/SpawnerBlockEntity#setEntityId` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#CODEC` | field | 4 |
| `net/minecraft/world/level/block/state/BlockState#getBlock` | method | 4 |
| `net/minecraft/world/level/block/state/BlockState#getFluidState` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#getValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 13 |
| `net/minecraft/world/level/block/state/BlockState#isAir` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#isSolid` | method | 2 |
| `net/minecraft/world/level/block/state/BlockState#setValue` | method | 1 |
| `net/minecraft/world/level/block/state/BlockState#trySetValue` | method | 2 |
| `net/minecraft/world/level/block/state/properties/BlockStateProperties#AXIS` | field | 2 |
| `net/minecraft/world/level/chunk/BulkSectionAccess#<init>` | method | 1 |
| `net/minecraft/world/level/chunk/BulkSectionAccess#close` | method | 1 |
| `net/minecraft/world/level/chunk/BulkSectionAccess#getSection` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getAllReferences` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getHeight` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkGenerator#biomeSource` | method | 2 |
| `net/minecraft/world/level/chunk/ChunkGenerator#featuresPerStep` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkGenerator#generationSettingsGetter` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getBiomeSource` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkGenerator#getMinY` | method | 1 |
| `net/minecraft/world/level/chunk/LevelChunkSection#setBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#STRUCTURE_REFERENCES` | field | 1 |
| `net/minecraft/world/level/dimension/DimensionType#Y_SIZE` | field | 2 |
| `net/minecraft/world/level/dimension/DimensionType#attributes` | method | 2 |
| `net/minecraft/world/level/dimension/DimensionType#timelines` | method | 1 |
| `net/minecraft/world/level/dimension/LevelStem#generator` | method | 2 |
| `net/minecraft/world/level/levelgen/Aquifer$Config#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/Aquifer$Config#barrierNoise` | method | 2 |
| `net/minecraft/world/level/levelgen/Aquifer$Config#exclusion` | method | 2 |
| `net/minecraft/world/level/levelgen/Aquifer$Config#fluidLevelFloodednessNoise` | method | 2 |
| `net/minecraft/world/level/levelgen/Aquifer$Config#fluidLevelSpreadNoise` | method | 2 |
| `net/minecraft/world/level/levelgen/Aquifer$Config#lavaNoise` | method | 2 |
| `net/minecraft/world/level/levelgen/Aquifer$Config#surfaceLevel` | method | 2 |
| `net/minecraft/world/level/levelgen/Beardifier#forStructuresInChunk` | method | 1 |
| `net/minecraft/world/level/levelgen/Beardifier$Rigid#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/Column#scan` | method | 1 |
| `net/minecraft/world/level/levelgen/Column$Range#ceiling` | method | 1 |
| `net/minecraft/world/level/levelgen/Column$Range#floor` | method | 1 |
| `net/minecraft/world/level/levelgen/Column$Range#height` | method | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#CODEC` | field | 2 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#RAW_GENERATION` | field | 1 |
| `net/minecraft/world/level/levelgen/GenerationStep$Decoration#ordinal` | method | 2 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#CODEC` | field | 3 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#MOTION_BLOCKING_NO_LEAVES` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#OCEAN_FLOOR_WG` | field | 1 |
| `net/minecraft/world/level/levelgen/Heightmap$Types#WORLD_SURFACE_WG` | field | 4 |
| `net/minecraft/world/level/levelgen/LegacyRandomSource#<init>` | method | 3 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#buildTerrain` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#generatorSettings` | method | 3 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#settings` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#spawnOriginalMobs` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#aquifers` | method | 3 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#noiseRouter` | method | 3 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#useLegacyRandomSource` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseRouter#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseRouter#chunkSurfaceLevel` | method | 2 |
| `net/minecraft/world/level/levelgen/NoiseRouter#continents` | method | 2 |
| `net/minecraft/world/level/levelgen/NoiseRouter#depth` | method | 2 |
| `net/minecraft/world/level/levelgen/NoiseRouter#erosion` | method | 2 |
| `net/minecraft/world/level/levelgen/NoiseRouter#finalDensity` | method | 2 |
| `net/minecraft/world/level/levelgen/NoiseRouter#ridges` | method | 2 |
| `net/minecraft/world/level/levelgen/NoiseRouter#temperature` | method | 2 |
| `net/minecraft/world/level/levelgen/NoiseRouter#vegetation` | method | 2 |
| `net/minecraft/world/level/levelgen/Noises#TEMPERATURE_NETHER` | field | 1 |
| `net/minecraft/world/level/levelgen/Noises#VEGETATION_NETHER` | field | 1 |
| `net/minecraft/world/level/levelgen/PositionalRandomFactory#at` | method | 3 |
| `net/minecraft/world/level/levelgen/PositionalRandomFactory#fromHashOf` | method | 2 |
| `net/minecraft/world/level/levelgen/RandomState#create` | method | 2 |
| `net/minecraft/world/level/levelgen/RandomState#createClimateSampler` | method | 1 |
| `net/minecraft/world/level/levelgen/RandomState#getOrCreateNoise` | method | 2 |
| `net/minecraft/world/level/levelgen/RandomState#random` | method | 1 |
| `net/minecraft/world/level/levelgen/RandomState#samplersWithContext` | method | 5 |
| `net/minecraft/world/level/levelgen/RandomState#surfaceSystem` | method | 1 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#BOTTOM` | field | 1 |
| `net/minecraft/world/level/levelgen/WorldGenSettings#options` | method | 1 |
| `net/minecraft/world/level/levelgen/WorldGenerationContext#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/WorldOptions#seed` | method | 1 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#nextFloat` | method | 2 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#nextIntBetweenInclusive` | method | 2 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#setLargeFeatureWithSalt` | method | 1 |
| `net/minecraft/world/level/levelgen/WorldgenRandom#setSeed` | method | 1 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#<clinit>` | method | 1 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#CODEC` | field | 6 |
| `net/minecraft/world/level/levelgen/blockpredicates/BlockPredicate#test` | method | 6 |
| `net/minecraft/world/level/levelgen/blockpredicates/StateTestingPredicate#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityBuffer#fill` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityBuffer#get` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityBuffer#set` | method | 7 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityBuffer#size` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#CODEC` | field | 12 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#compileSampler` | method | 6 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#domainAxes` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#equals` | method | 4 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#range` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#rewriteChildren` | method | 3 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunctions#zero` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunctions$HolderHolder#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunctions$HolderHolder#function` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensitySampler#sampleValue` | method | 6 |
| `net/minecraft/world/level/levelgen/densityfunction/DensitySampler#sampleVolume` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensitySamplerSet#sampleValue` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockX` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockY` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockZ` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeX` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeY` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeZ` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/DfRewriteRule#rewrite` | method | 6 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext#acquireBuffer` | method | 3 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext#builder` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext$Builder#build` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext$Builder#enableCaches` | method | 5 |
| `net/minecraft/world/level/levelgen/densityfunction/ScopedDensityBuffer#close` | method | 3 |
| `net/minecraft/world/level/levelgen/densityfunction/ScopedDensityBuffer#get` | method | 3 |
| `net/minecraft/world/level/levelgen/feature/TreeFeature#decorators` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/TreeFeature#trunkProvider` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/foliageplacers/FoliagePlacer$FoliageAttachment#doubleTrunk` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/foliageplacers/FoliagePlacer$FoliageAttachment#pos` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/foliageplacers/FoliagePlacer$FoliageAttachment#radiusOffsetXZ` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/foliageplacers/FoliagePlacer$FoliageSetter#set` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/foliageplacers/FoliagePlacerType#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/feature/foliageplacers/MegaJungleFoliagePlacer#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/foliageplacers/MegaPineFoliagePlacer#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/rootplacers/MangroveRootPlacement#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/feature/rootplacers/MangroveRootPlacer#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/rootplacers/RootPlacerType#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#CODEC` | field | 5 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#getState` | method | 7 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#holderOf` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/stateproviders/BlockStateProvider#of` | method | 1 |
| `net/minecraft/world/level/levelgen/feature/treedecorators/TreeDecorator#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/heightproviders/ConstantHeight#of` | method | 1 |
| `net/minecraft/world/level/levelgen/heightproviders/HeightProvider#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/heightproviders/HeightProvider#sample` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRuleContext#getDensitiesInChunk` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRuleContext#surfaceGradientX` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRuleContext#surfaceGradientZ` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRuleContext$LazyXZCondition#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/material/MaterialRules#sequence` | method | 2 |
| `net/minecraft/world/level/levelgen/material/MaterialRules$DensityGetter#get` | method | 1 |
| `net/minecraft/world/level/levelgen/material/condition/ConditionEvaluator#test` | method | 2 |
| `net/minecraft/world/level/levelgen/material/condition/MaterialCondition#CODEC` | field | 2 |
| `net/minecraft/world/level/levelgen/material/condition/MaterialCondition#compile` | method | 2 |
| `net/minecraft/world/level/levelgen/material/rule/MaterialRule#HOLDER_CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/material/rule/SequenceRule#sequence` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#CODEC` | field | 4 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#LIST_CODEC` | field | 3 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#feature` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacedFeature#place` | method | 3 |
| `net/minecraft/world/level/levelgen/placement/PlacementContext#generator` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacementContext#getLevel` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#encapsulate` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#getCenter` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#getYSpan` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#intersectingChunks` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#intersects` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#isInside` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxX` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxY` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#maxZ` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minX` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minY` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#minZ` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#moved` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#addJunction` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#getBoundingBox` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#getElement` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#getGroundLevelDelta` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#getPosition` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#getRotation` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/PoolElementStructurePiece#move` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/Structure#CODEC` | field | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure#biomes` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#findValidGenerationPoint` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#spawnOverrides` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#step` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure#terrainAdaptation` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#biomeResolver` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkGenerator` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#chunkPos` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#couldStructureExistInColumn` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#heightAccessor` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#random` | method | 5 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#randomState` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#registryAccess` | method | 6 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#seed` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#structureTemplateManager` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationContext#validBiome` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationStub#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/Structure$GenerationStub#position` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/Structure$StructureSettings#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#<init>` | method | 5 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#fillColumnDown` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#getBoundingBox` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#placeBlock` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#postProcess` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureSet#structures` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/StructureSet$StructureSelectionEntry#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureSet$StructureSelectionEntry#structure` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureStart#getPieces` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructureStart#getStructure` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#BEARD_BOX` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#BEARD_THIN` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#BURY` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#ENCAPSULATE` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#NONE` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#ordinal` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/TerrainAdjustment#values` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pieces/StructurePiecesBuilder#addPiece` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/pools/DimensionPadding#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/DimensionPadding#ZERO` | field | 3 |
| `net/minecraft/world/level/levelgen/structure/pools/DimensionPadding#bottom` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/DimensionPadding#top` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/EmptyPoolElement#INSTANCE` | field | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/JigsawJunction#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/JigsawPlacement#addPieces` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/ListPoolElement#getElements` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/SinglePoolElement#processors` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/SinglePoolElement#template` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#CODEC` | field | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#getBoundingBox` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#getGroundLevelDelta` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#getProjection` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#getShuffledJigsawBlocks` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#getSize` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#place` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructurePoolElement#setProjection` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#getFallback` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#getMaxSize` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#getRandomTemplate` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#getShuffledTemplates` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#rawTemplates` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#size` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool#templates` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/StructureTemplatePool$Projection#RIGID` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/pools/alias/PoolAliasBinding#CODEC` | field | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/alias/PoolAliasLookup#create` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/pools/alias/PoolAliasLookup#lookup` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/structures/DesertPyramidStructure#placeSuspiciousSand` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/EndCityPieces$EndCityPiece#makeSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/IglooPieces$IglooPiece#makeSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/JigsawStructure#findGenerationPoint` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/JigsawStructure#poolAliases` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/structures/JigsawStructure$MaxDistance#horizontal` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/JigsawStructure$MaxDistance#vertical` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftCorridor#fillColumnBetween` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftCorridor#fillColumnDown` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftCorridor#fillPillarDownOrChainUp` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftCorridor#placeDoubleLowerOrUpperSupport` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftCorridor#postProcess` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/MineshaftPieces$MineShaftPiece#setPlanksBlock` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/NetherFortressPieces$MonsterThrone#postProcess` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/NetherFossilPieces$NetherFossilPiece#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/NetherFossilPieces$NetherFossilPiece#makeSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/NetherFossilPieces$NetherFossilPiece#placeDriedGhast` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/NetherFossilStructure#findGenerationPoint` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/OceanRuinPieces$OceanRuinPiece#makeSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/OceanRuinStructure$Type#getName` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/RuinedPortalPiece#makeSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/RuinedPortalPiece#maybeAddLeavesAbove` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/structures/RuinedPortalPiece#maybeAddVines` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/RuinedPortalPiece#placeNetherrackOrMagma` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/RuinedPortalStructure#findGenerationPoint` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/structures/ShipwreckPieces#addRandomPiece` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/ShipwreckPieces$ShipwreckPiece#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/ShipwreckPieces$ShipwreckPiece#makeSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/ShipwreckStructure#generatePieces` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/StrongholdPieces$PortalRoom#postProcess` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection#get1x1` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection#get1x1Secret` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection#get1x2FrontEntrance` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection#get1x2Secret` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection#get1x2SideEntrance` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection#get2x2` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$FirstFloorRoomCollection#get2x2Secret` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$MansionGrid#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$MansionPiecePlacer#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$MansionPiecePlacer#createMansion` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection#get1x1` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection#get1x1Secret` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection#get1x2FrontEntrance` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection#get1x2Secret` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection#get1x2SideEntrance` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection#get2x2` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$SecondFloorRoomCollection#get2x2Secret` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$WoodlandMansionPiece#makeLocation` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionPieces$WoodlandMansionPiece#makeSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/structures/WoodlandMansionStructure#generatePieces` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/LiquidSettings#APPLY_WATERLOGGING` | field | 4 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/LiquidSettings#CODEC` | field | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/PosRuleTest#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#addProcessor` | method | 8 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setLiquidSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setRandom` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructurePlaceSettings#setRotation` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorList#<init>` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorList#list` | method | 8 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorType#DIRECT_CODEC` | field | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorType#LIST_CODEC` | field | 3 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorType#LIST_OBJECT_CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureProcessorType#SINGLE_CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate#getSize` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#name` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#placementPriority` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#pool` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#pos` | method | 2 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$JigsawBlockInfo#state` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#<init>` | method | 4 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#nbt` | method | 3 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#pos` | method | 8 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplate$StructureBlockInfo#state` | method | 6 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/StructureTemplateManager#getOrCreate` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/rule/blockentity/Passthrough#INSTANCE` | field | 1 |
| `net/minecraft/world/level/levelgen/structure/templatesystem/rule/blockentity/RuleBlockEntityModifier#CODEC` | field | 3 |
| `net/minecraft/world/level/levelgen/synth/BlendedNoise#NOISE_SEED` | field | 1 |
| `net/minecraft/world/level/levelgen/synth/Noise#addToVolume` | method | 1 |
| `net/minecraft/world/level/levelgen/synth/Noise#get` | method | 5 |
| `net/minecraft/world/level/levelgen/synth/NormalNoise#createForLegacyNetherBiome` | method | 1 |
| `net/minecraft/world/level/material/FluidState#getType` | method | 1 |
| `net/minecraft/world/level/material/FluidState#isEmpty` | method | 1 |
| `net/minecraft/world/level/storage/loot/BuiltInLootTables#SIMPLE_DUNGEON` | field | 1 |
| `net/minecraft/world/phys/AABB#<init>` | method | 2 |
| `net/minecraft/world/phys/AABB#contains` | method | 1 |
| `net/minecraft/world/phys/AABB#deflate` | method | 1 |
| `net/minecraft/world/phys/AABB#equals` | method | 1 |
| `net/minecraft/world/phys/AABB#getXsize` | method | 1 |
| `net/minecraft/world/phys/AABB#getYsize` | method | 1 |
| `net/minecraft/world/phys/AABB#getZsize` | method | 1 |
| `net/minecraft/world/phys/AABB#maxX` | field | 1 |
| `net/minecraft/world/phys/AABB#maxY` | field | 1 |
| `net/minecraft/world/phys/AABB#maxZ` | field | 1 |
| `net/minecraft/world/phys/AABB#minX` | field | 1 |
| `net/minecraft/world/phys/AABB#minY` | field | 1 |
| `net/minecraft/world/phys/AABB#minZ` | field | 1 |
| `net/minecraft/world/phys/AABB#move` | method | 1 |
| `net/minecraft/world/phys/AABB#of` | method | 2 |
| `net/minecraft/world/phys/Vec3#<init>` | method | 1 |
| `net/minecraft/world/phys/Vec3#scale` | method | 1 |
| `net/minecraft/world/phys/Vec3#x` | field | 2 |
| `net/minecraft/world/phys/Vec3#y` | field | 1 |
| `net/minecraft/world/phys/Vec3#z` | field | 2 |
| `net/minecraft/world/timeline/AttributeTrack#createCodec` | method | 1 |
| `net/minecraft/world/timeline/Timeline#CODEC` | field | 1 |
| `net/minecraft/world/timeline/Timeline#attributes` | method | 1 |
| `net/minecraft/world/timeline/Timeline#createTrackSampler` | method | 1 |
| `net/minecraft/world/timeline/Timeline#tracks` | method | 2 |

</details>

