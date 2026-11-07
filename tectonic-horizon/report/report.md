# Horizon port report: Tectonic

`tectonic` 3.0.31 · tectonic-3.0.31-fabric-26.3.jar · 55 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 1 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 15 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 6 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `main`: dev.worldgen.tectonic.platform.fabric.TectonicFabric
- `modmenu`: dev.worldgen.tectonic.compat.TectonicModMenuCompat

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| CommandRegistrationCallback.EVENT | `TectonicFabric` | hook `command` | needs-translation | Server commands. Horizon's command hook is client-side. |

## Mixins

| Injection | In | Horizon | Status | Note |
|---|---|---|---|---|
| @ModifyVariable(HEAD) on Biome.getHeightAdjustedTemperature | `BiomeMixin.tectonic$adjustSnowStart` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for Biome.getHeightAdjustedTemperature: run `match`. |
| @Inject(INVOKE) on BlendingData.getHeightAtXZ at Lnet/minecraft/core/BlockPos$MutableBlockPos;move(Lnet/minecraft/core/Direction;)Lnet/minecraft/core/BlockPos$MutableBlockPos; | `BlendingDataMixin.checkTerralithSurfaceBlocks` | BEFORE_CALL | needs-mapping | Only whole statements, plain let initializers and block tails can be selected. No Horizon function known for BlendingData.getHeightAtXZ: run `match`. The call it hooks (MutableBlockPos.move) isn't mapped to a Rust function yet. |
| @Inject(HEAD) on ChunkAccess.getOrCreateOffsetList | `ChunkAccessMixin.tectonic$stopHeightDecreasingLogSpam` | HEAD | needs-mapping | No Horizon function known for ChunkAccess.getOrCreateOffsetList: run `match`. |
| @Inject(HEAD) on SerializableChunkData.parse | `ChunkDataMixin.tectonic$parse` | HEAD | needs-mapping | No Horizon function known for SerializableChunkData.parse: run `match`. |
| @Inject(RETURN) on SerializableChunkData.write | `ChunkDataMixin.tectonic$write` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for SerializableChunkData.write: run `match`. |
| @Accessor on DimensionType.minY | `DimensionTypeAccessor.setMinY` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for DimensionType.minY: run `match`. |
| @Accessor on DimensionType.height | `DimensionTypeAccessor.setHeight` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for DimensionType.height: run `match`. |
| @Accessor on DimensionType.logicalHeight | `DimensionTypeAccessor.setLogicalHeight` | GET on horizon_client::game::Dim::logical_height | auto | A [[bridge]] GET (SET for a setter) on the Rust struct's field. |
| mixin into Heightmap with no injectors (interfaces or @Shadow only) | `HeightmapMixin` | – | needs-translation | Usually adds an interface or fields to a game class: needs a manual change. |
| @Inject(HEAD) on IOWorker.isOldChunk | `IOWorkerMixin.tectonic$needsBlending` | HEAD | needs-mapping | No Horizon function known for IOWorker.isOldChunk: run `match`. |
| @Inject(RETURN) on NoiseBasedChunkGenerator.<init> | `NoiseBasedChunkGeneratorMixin.tectonic$fixLavaLevel` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for NoiseBasedChunkGenerator.<init>: run `match`. |
| @Accessor on NoiseSettings.minY | `NoiseSettingsAccessor.setMinY` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for NoiseSettings.minY: run `match`. |
| @Accessor on NoiseSettings.height | `NoiseSettingsAccessor.setHeight` | GET | needs-mapping | A [[bridge]] GET (SET for a setter) on the Rust struct's field. No Horizon function known for NoiseSettings.height: run `match`. |
| @ModifyArg(INVOKE) on Noises.instantiate at Lnet/minecraft/world/level/levelgen/PositionalRandomFactory;fromHashOf(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/util/RandomSource; | `NoisesMixin.tectonic$fixTectonicNoiseSeeds` | MODIFY_ARG | needs-mapping | No Horizon function known for Noises.instantiate: run `match`. The call it hooks (PositionalRandomFactory.fromHashOf) isn't mapped to a Rust function yet. |
| @ModifyVariable(HEAD) on StructurePiece.<init> | `StructurePieceMixin.tectonic$lowerOceanMonuments` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for StructurePiece.<init>: run `match`. |
| @ModifyVariable(HEAD) on 2.modifyTemperature | `TemperatureModifierMixin.tectonic$adjustSnowStart` | MODIFY_VARIABLE | needs-mapping | Selects a Rust local by name: pick it in the target function. No Horizon function known for 2.modifyTemperature: run `match`. |
| @Inject(RETURN) on WorldCreationUiState.getNonEmptyList | `WorldCreationUiStateMixin.tectonic$removeUnusablePresets` | RETURN | needs-mapping | Horizon's RETURN runs at every normal exit and passes the return value on. No Horizon function known for WorldCreationUiState.getNonEmptyList: run `match`. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends Entry | `PresetSelectorScreen$PresetList$Entry` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `PresetSelectorScreen$PresetList` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `PresetSelectorScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |

## Access wideners

| File | In | Status | Note |
|---|---|---|---|
| access widener (opens private game fields) | `tectonic.ct` | needs-translation | Native helpers already see the game's fields, once each field is mapped. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 1 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

6 of 186 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/core/BlockPos#spiralAround` | horizon_world::worldgen::generator::spiral_around (suggested) |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/util/Mth#floor` | horizon_client_render::gfx::occlusion::floor (suggested) |
| `net/minecraft/world/level/dimension/DimensionType#logicalHeight` | horizon_client::game::Dim::logical_height (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `net/minecraft/ChatFormatting#BOLD` | field | 1 |
| `net/minecraft/ChatFormatting#DARK_GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 1 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 1 |
| `net/minecraft/SharedConstants#DEBUG_DISABLE_FLUID_GENERATION` | field | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 1 |
| `net/minecraft/client/gui/components/Button#builder` | method | 3 |
| `net/minecraft/client/gui/components/Button#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/Button#setX` | method | 1 |
| `net/minecraft/client/gui/components/Button#setY` | method | 1 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 3 |
| `net/minecraft/client/gui/components/Button$Builder#width` | method | 2 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#<init>` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#updateSize` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList$Entry#<init>` | method | 1 |
| `net/minecraft/client/gui/components/Tooltip#create` | method | 1 |
| `net/minecraft/client/gui/layouts/GridLayout$RowHelper#addChild` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#<init>` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#addTitleHeader` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#addToContents` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#addToFooter` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#arrangeElements` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#getContentHeight` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#getHeaderHeight` | method | 1 |
| `net/minecraft/client/gui/layouts/HeaderAndFooterLayout#visitWidgets` | method | 1 |
| `net/minecraft/client/gui/layouts/LinearLayout#addChild` | method | 2 |
| `net/minecraft/client/gui/layouts/LinearLayout#horizontal` | method | 2 |
| `net/minecraft/client/gui/layouts/LinearLayout#spacing` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/worldselection/WorldCreationUiState#getNonEmptyList` | method | 1 |
| `net/minecraft/client/gui/screens/worldselection/WorldCreationUiState$WorldTypeEntry#preset` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getLevel` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#getPosition` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendFailure` | method | 1 |
| `net/minecraft/commands/CommandSourceStack#sendSystemMessage` | method | 1 |
| `net/minecraft/commands/Commands#LEVEL_GAMEMASTERS` | field | 1 |
| `net/minecraft/commands/Commands#hasPermission` | method | 1 |
| `net/minecraft/commands/Commands#literal` | method | 1 |
| `net/minecraft/core/BlockPos#<init>` | method | 1 |
| `net/minecraft/core/BlockPos#ZERO` | field | 1 |
| `net/minecraft/core/BlockPos#above` | method | 1 |
| `net/minecraft/core/BlockPos#atY` | method | 1 |
| `net/minecraft/core/BlockPos#below` | method | 1 |
| `net/minecraft/core/BlockPos#containing` | method | 1 |
| `net/minecraft/core/BlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getX` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getY` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#getZ` | method | 1 |
| `net/minecraft/core/BlockPos$MutableBlockPos#move` | method | 1 |
| `net/minecraft/core/Direction#EAST` | field | 1 |
| `net/minecraft/core/Direction#SOUTH` | field | 1 |
| `net/minecraft/core/Holder#is` | method | 1 |
| `net/minecraft/core/Holder#unwrapKey` | method | 1 |
| `net/minecraft/core/Holder#value` | method | 4 |
| `net/minecraft/core/HolderLookup$RegistryLookup#getOrThrow` | method | 1 |
| `net/minecraft/core/Registry#getOrThrow` | method | 1 |
| `net/minecraft/core/RegistryAccess#lookupOrThrow` | method | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#DENSITY_FUNCTION_TYPE` | field | 1 |
| `net/minecraft/core/registries/BuiltInRegistries#PLACEMENT_MODIFIER_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#BLOCK` | field | 1 |
| `net/minecraft/core/registries/Registries#DENSITY_FUNCTION` | field | 1 |
| `net/minecraft/core/registries/Registries#DIMENSION_TYPE` | field | 1 |
| `net/minecraft/core/registries/Registries#NOISE` | field | 1 |
| `net/minecraft/core/registries/Registries#WORLD_PRESET` | field | 1 |
| `net/minecraft/nbt/CompoundTag#<init>` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getIntOr` | method | 2 |
| `net/minecraft/nbt/CompoundTag#getListOrEmpty` | method | 1 |
| `net/minecraft/nbt/CompoundTag#getStringOr` | method | 1 |
| `net/minecraft/nbt/CompoundTag#put` | method | 1 |
| `net/minecraft/nbt/CompoundTag#putInt` | method | 1 |
| `net/minecraft/nbt/CompoundTag#remove` | method | 1 |
| `net/minecraft/nbt/IntTag#intValue` | method | 1 |
| `net/minecraft/nbt/ListTag#iterator` | method | 1 |
| `net/minecraft/network/chat/ClickEvent$SuggestCommand#<init>` | method | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_CANCEL` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#NEW_LINE` | field | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 3 |
| `net/minecraft/network/chat/Component#translatable` | method | 4 |
| `net/minecraft/network/chat/Component#translatableWithFallback` | method | 2 |
| `net/minecraft/network/chat/ComponentUtils#wrapInSquareBrackets` | method | 1 |
| `net/minecraft/network/chat/HoverEvent$ShowText#<init>` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#withColor` | method | 1 |
| `net/minecraft/network/chat/Style#withClickEvent` | method | 1 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/network/chat/Style#withHoverEvent` | method | 1 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 1 |
| `net/minecraft/resources/Identifier#getNamespace` | method | 1 |
| `net/minecraft/resources/Identifier#getPath` | method | 2 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 1 |
| `net/minecraft/resources/ResourceKey#create` | method | 1 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 1 |
| `net/minecraft/server/level/ServerChunkCache#getGenerator` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getChunkSource` | method | 1 |
| `net/minecraft/server/level/ServerLevel#getSeed` | method | 1 |
| `net/minecraft/server/level/ServerLevel#registryAccess` | method | 1 |
| `net/minecraft/tags/TagKey#create` | method | 2 |
| `net/minecraft/util/ExtraCodecs#POSITIVE_FLOAT` | field | 1 |
| `net/minecraft/util/InclusiveRange#<init>` | method | 1 |
| `net/minecraft/util/InclusiveRange#isValueInRange` | method | 1 |
| `net/minecraft/util/Interval#clamp` | method | 1 |
| `net/minecraft/util/Interval#max` | method | 1 |
| `net/minecraft/util/Interval#min` | method | 1 |
| `net/minecraft/util/Interval#ofExact` | method | 1 |
| `net/minecraft/util/Mth#sqrt` | method | 1 |
| `net/minecraft/util/RandomSource#nextInt` | method | 1 |
| `net/minecraft/util/RandomSource#nextIntBetweenInclusive` | method | 1 |
| `net/minecraft/world/level/Level#OVERWORLD` | field | 1 |
| `net/minecraft/world/level/Level#dimension` | method | 1 |
| `net/minecraft/world/level/biome/Biome#getHeightAdjustedTemperature` | method | 1 |
| `net/minecraft/world/level/biome/Biome$TemperatureModifier$2#modifyTemperature` | method | 1 |
| `net/minecraft/world/level/block/Block#defaultBlockState` | method | 1 |
| `net/minecraft/world/level/block/Blocks#AIR` | field | 1 |
| `net/minecraft/world/level/block/Blocks#LAVA` | field | 1 |
| `net/minecraft/world/level/block/state/BlockState#is` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getBlockState` | method | 1 |
| `net/minecraft/world/level/chunk/ChunkAccess#getOrCreateOffsetList` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#byName` | method | 1 |
| `net/minecraft/world/level/chunk/status/ChunkStatus#toString` | method | 1 |
| `net/minecraft/world/level/chunk/storage/IOWorker#isOldChunk` | method | 1 |
| `net/minecraft/world/level/chunk/storage/SerializableChunkData#parse` | method | 1 |
| `net/minecraft/world/level/chunk/storage/SerializableChunkData#write` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#CODEC` | field | 1 |
| `net/minecraft/world/level/dimension/DimensionType#MIN_Y` | field | 1 |
| `net/minecraft/world/level/dimension/DimensionType#height` | method | 1 |
| `net/minecraft/world/level/dimension/DimensionType#minY` | method | 1 |
| `net/minecraft/world/level/levelgen/Aquifer$FluidStatus#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseBasedChunkGenerator#generatorSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#defaultFluid` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#noiseSettings` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseGeneratorSettings#seaLevel` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseSettings#height` | method | 1 |
| `net/minecraft/world/level/levelgen/NoiseSettings#minY` | method | 1 |
| `net/minecraft/world/level/levelgen/Noises#instantiate` | method | 1 |
| `net/minecraft/world/level/levelgen/PositionalRandomFactory#fromHashOf` | method | 1 |
| `net/minecraft/world/level/levelgen/RandomState#create` | method | 1 |
| `net/minecraft/world/level/levelgen/RandomState#getSampler` | method | 1 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/VerticalAnchor#resolveY` | method | 1 |
| `net/minecraft/world/level/levelgen/blending/BlendingData#getHeightAtXZ` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityBuffer#get` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityBuffer#set` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#CODEC` | field | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#clamp` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#compileSampler` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#domainAxes` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#range` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction#rewriteChildren` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunction$CompileContext#createNoiseSampler` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityFunctions#constant` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/DensitySampler#sampleValue` | method | 3 |
| `net/minecraft/world/level/levelgen/densityfunction/DensitySampler#sampleVolume` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockX` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#blockZ` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeX` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeY` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DensityVolume#sizeZ` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/DfRewriteRule#rewrite` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext#acquireBuffer` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext#builder` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext$Builder#build` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/SamplerContext$Builder#enableCaches` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/ScopedDensityBuffer#close` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/ScopedDensityBuffer#get` | method | 2 |
| `net/minecraft/world/level/levelgen/densityfunction/generator/ConstantFunction$Sampler#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/densityfunction/op/ClampFunction$Sampler#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/placement/PlacementContext#getMinGenY` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/BoundingBox#moved` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/StructurePiece#<init>` | method | 1 |
| `net/minecraft/world/level/levelgen/structure/pieces/StructurePieceType#OCEAN_MONUMENT_BUILDING` | field | 1 |
| `net/minecraft/world/level/levelgen/synth/Noise#get` | method | 2 |
| `net/minecraft/world/level/levelgen/synth/NormalNoise#CODEC` | field | 1 |
| `net/minecraft/world/level/levelgen/synth/NormalNoise#range` | method | 1 |

</details>

