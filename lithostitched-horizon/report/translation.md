# Java translated to Rust

Translated 42 of 1532 methods and 18 of 216 static fields to Rust; 0 of 2 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 1257 | `LoadPredicate.allOf(1)`: it uses LoadPredicate, which isn't translated (it's an interface) |
| Uses Minecraft classes (needs Horizon's game API) | 118 | `LithostitchedBiomeSourceTags.create(1)`: it uses Minecraft's `TagKey` |
| Other Java this translator doesn't handle yet | 115 | `AddBiomeInjectorsEvent.addInjectors(2)`: it has no body |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| dev/worldgen/lithostitched/impl/platform/fabric/LithostitchedFabricClient.java:25 | `level_tick_start` | not translated: it calls StructureAttributesHolder.from, which isn't translated |
| dev/worldgen/lithostitched/impl/platform/fabric/LithostitchedFabric.java:13 | `init` | not translated: it calls Lithostitched.init, which isn't translated |

## Every class


### `dev/worldgen/lithostitched/api/worldgen/util/WeightedSpawnerData.java`: couldn't be read: expected a name but found `{` (line 23)


### `AddBiomeInjectorsEvent`: it's an interface

- [ ] `AddBiomeInjectorsEvent.addInjectors(2)`: it has no body (dev/worldgen/lithostitched/api/event/AddBiomeInjectorsEvent.java:16)

### `AddRegionsEvent`: it's an interface

- [ ] `AddRegionsEvent.addRegions(2)`: it has no body (dev/worldgen/lithostitched/api/event/AddRegionsEvent.java:18)

### `AddRegionsEvent.RegionConsumer`: it's an interface

- [ ] `AddRegionsEvent.RegionConsumer.accept(4)`: it has no body (dev/worldgen/lithostitched/api/event/AddRegionsEvent.java:21)

### `AddWorldgenModifiersEvent`: it's an interface

- [ ] `AddWorldgenModifiersEvent.addModifiers(2)`: it has no body (dev/worldgen/lithostitched/api/event/AddWorldgenModifiersEvent.java:16)

### `LoadPredicate`: it's an interface

- [ ] `LoadPredicate.test(0)`: it has no body (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:22)
- [ ] `LoadPredicate.codec(0)`: it has no body (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:24)
- [ ] `LoadPredicate.allOf(1)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:26)
- [ ] `LoadPredicate.anyOf(1)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:30)
- [ ] `LoadPredicate.isFabric(0)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:34)
- [ ] `LoadPredicate.isNeoforge(0)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:38)
- [ ] `LoadPredicate.modLoaded(1)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:42)
- [ ] `LoadPredicate.not(1)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:46)
- [ ] `LoadPredicate.packFormat(1)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:50)
- [ ] `LoadPredicate.alwaysFalse(0)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:54)
- [ ] `LoadPredicate.alwaysTrue(0)`: it uses LoadPredicate, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java:58)

### `LithostitchedBuiltInRegistries`

- [ ] `LithostitchedBuiltInRegistries.MODIFIER_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:144)
- [ ] `LithostitchedBuiltInRegistries.PLACEMENT_CONDITION_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:145)
- [ ] `LithostitchedBuiltInRegistries.PROCESSOR_CONDITION_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:146)
- [ ] `LithostitchedBuiltInRegistries.BANDLANDS_BAND_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:147)
- [ ] `LithostitchedBuiltInRegistries.BIOME_INJECTOR_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:148)
- [ ] `LithostitchedBuiltInRegistries.FAST_NOISE_CONFIG_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:149)
- [ ] `LithostitchedBuiltInRegistries.LOAD_PREDICATE_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:150)
- [ ] `LithostitchedBuiltInRegistries.HOLDER_TARGET_TYPE`: it uses Minecraft's `Registry` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:151)
- [ ] `LithostitchedBuiltInRegistries.create(1)`: it's generic (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:153)
- [ ] `LithostitchedBuiltInRegistries.init(0)`: it calls LithostitchedRegistrar.registerRegistry, which isn't translated (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java:157)

### `LithostitchedRegistries`: it's an interface

- [ ] `LithostitchedRegistries.create(1)`: it's generic (dev/worldgen/lithostitched/api/registry/LithostitchedRegistries.java:37)

### `LithostitchedBiomeSourceTags`: it's an interface

- [ ] `LithostitchedBiomeSourceTags.create(1)`: it uses Minecraft's `TagKey` (dev/worldgen/lithostitched/api/tag/LithostitchedBiomeSourceTags.java:12)

### `LithostitchedBiomeTags`: it's an interface

- [ ] `LithostitchedBiomeTags.create(1)`: it uses Minecraft's `TagKey` (dev/worldgen/lithostitched/api/tag/LithostitchedBiomeTags.java:18)
- [ ] `LithostitchedBiomeTags.createVillagerTypeTag(1)`: it uses Minecraft's `TagKey` (dev/worldgen/lithostitched/api/tag/LithostitchedBiomeTags.java:22)

### `LithostitchedBlockTags`: it's an interface

- [ ] `LithostitchedBlockTags.create(1)`: it uses Minecraft's `TagKey` (dev/worldgen/lithostitched/api/tag/LithostitchedBlockTags.java:12)

### `LithostitchedProcessorListTags`: it's an interface

- [ ] `LithostitchedProcessorListTags.create(1)`: it uses Minecraft's `TagKey` (dev/worldgen/lithostitched/api/tag/LithostitchedProcessorListTags.java:11)

### `LithostitchedTemplatePoolTags`: it's an interface

- [ ] `LithostitchedTemplatePoolTags.create(1)`: it uses Minecraft's `TagKey` (dev/worldgen/lithostitched/api/tag/LithostitchedTemplatePoolTags.java:14)

### `InjectionType`: its constants carry values

- [ ] `InjectionType.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/util/InjectionType.java:13)
- [ ] `InjectionType.InjectionType(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/util/InjectionType.java:16)
- [ ] `InjectionType.apply(3)`: it's generic (dev/worldgen/lithostitched/api/util/InjectionType.java:20)
- [ ] `InjectionType.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/util/InjectionType.java:28)

### `WeightedHolderSet`: it's generic

- [ ] `WeightedHolderSet.create(1)`: it's generic (dev/worldgen/lithostitched/api/util/WeightedHolderSet.java:17)
- [ ] `WeightedHolderSet.create(1)`: it's generic (dev/worldgen/lithostitched/api/util/WeightedHolderSet.java:21)
- [ ] `WeightedHolderSet.WeightedHolderSet(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/api/util/WeightedHolderSet.java:25)
- [ ] `WeightedHolderSet.getRandom(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/api/util/WeightedHolderSet.java:30)
- [ ] `WeightedHolderSet.codec(2)`: it's generic (dev/worldgen/lithostitched/api/util/WeightedHolderSet.java:34)

### `LithostitchedEnvironmentAttributes`: it's an interface


### `Band`: it's an interface

- [ ] `Band.fill(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/bandlands/Band.java:23)
- [ ] `Band.codec(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/bandlands/Band.java:25)

### `BandlandsBuilder`: it's an interface

- [ ] `BandlandsBuilder.create(1)`: it uses BandlandsBuilder, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/bandlands/BandlandsBuilder.java:10)
- [ ] `BandlandsBuilder.baseBand(1)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/bandlands/BandlandsBuilder.java:14)
- [ ] `BandlandsBuilder.baseBand(3)`: it has no body (dev/worldgen/lithostitched/api/worldgen/bandlands/BandlandsBuilder.java:18)
- [ ] `BandlandsBuilder.repeatBand(3)`: it has no body (dev/worldgen/lithostitched/api/worldgen/bandlands/BandlandsBuilder.java:20)
- [ ] `BandlandsBuilder.wrappedBand(5)`: it has no body (dev/worldgen/lithostitched/api/worldgen/bandlands/BandlandsBuilder.java:22)
- [ ] `BandlandsBuilder.build(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/bandlands/BandlandsBuilder.java:24)

### `BiomeInjector`: it's an interface

- [ ] `BiomeInjector.predicate(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:32)
- [ ] `BiomeInjector.dimension(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:34)
- [ ] `BiomeInjector.priority(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:36)
- [ ] `BiomeInjector.possibleBiomes(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:38)
- [ ] `BiomeInjector.codec(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:40)
- [ ] `BiomeInjector.builder(1)`: it uses InjectorBuilder, which isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:42)
- [ ] `BiomeInjector.builder(2)`: it uses InjectorBuilder, which isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:46)

### `BiomeInjector.ClimateParameter`: its constants carry values

- [ ] `BiomeInjector.ClimateParameter.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:58)
- [ ] `BiomeInjector.ClimateParameter.ClimateParameter(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:62)
- [ ] `BiomeInjector.ClimateParameter.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:67)

### `BiomeInjector.InjectorBuilder`: it uses Minecraft's `ResourceKey`

- [ ] `BiomeInjector.InjectorBuilder.InjectorBuilder(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:77)
- [ ] `BiomeInjector.InjectorBuilder.priority(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:82)
- [ ] `BiomeInjector.InjectorBuilder.addPoints(1)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:87)
- [ ] `BiomeInjector.InjectorBuilder.dispatchAlternateLayout(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:91)
- [ ] `BiomeInjector.InjectorBuilder.forcePlacement(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:95)
- [ ] `BiomeInjector.InjectorBuilder.replaceFully(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:99)
- [ ] `BiomeInjector.InjectorBuilder.replaceFully(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:105)
- [ ] `BiomeInjector.InjectorBuilder.replacePartially(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:109)
- [ ] `BiomeInjector.InjectorBuilder.replacePartially(3)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java:120)

### `ParameterBuilder`: it's an interface

- [ ] `ParameterBuilder.create(0)`: it uses ParameterBuilder, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:11)
- [ ] `ParameterBuilder.densityFunctionExactly(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:15)
- [ ] `ParameterBuilder.densityFunctionMin(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:17)
- [ ] `ParameterBuilder.densityFunctionMax(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:19)
- [ ] `ParameterBuilder.densityFunctionRange(3)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:21)
- [ ] `ParameterBuilder.climateExactly(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:23)
- [ ] `ParameterBuilder.climateMin(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:25)
- [ ] `ParameterBuilder.climateMax(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:27)
- [ ] `ParameterBuilder.climateRange(3)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:29)
- [ ] `ParameterBuilder.region(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:31)
- [ ] `ParameterBuilder.build(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java:33)

### `LithostitchedBlockEntityModifiers`: it's an interface

- [ ] `LithostitchedBlockEntityModifiers.applyAll(1)`: it uses Minecraft's `RuleBlockEntityModifier` (dev/worldgen/lithostitched/api/worldgen/blockentitymodifier/LithostitchedBlockEntityModifiers.java:10)
- [ ] `LithostitchedBlockEntityModifiers.applyRandom(1)`: it uses Minecraft's `RuleBlockEntityModifier` (dev/worldgen/lithostitched/api/worldgen/blockentitymodifier/LithostitchedBlockEntityModifiers.java:14)

### `LithostitchedBlockPredicates`: it's an interface

- [ ] `LithostitchedBlockPredicates.blockState(1)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:19)
- [ ] `LithostitchedBlockPredicates.blockState(2)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:23)
- [ ] `LithostitchedBlockPredicates.grid(4)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:27)
- [ ] `LithostitchedBlockPredicates.grid(5)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:31)
- [ ] `LithostitchedBlockPredicates.inStructure(1)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:35)
- [ ] `LithostitchedBlockPredicates.inStructure(2)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:39)
- [ ] `LithostitchedBlockPredicates.inStructure(2)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:43)
- [ ] `LithostitchedBlockPredicates.inStructure(3)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:47)
- [ ] `LithostitchedBlockPredicates.multipleOf(2)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:51)
- [ ] `LithostitchedBlockPredicates.offset(2)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:55)
- [ ] `LithostitchedBlockPredicates.randomChance(1)`: it uses Minecraft's `BlockPredicate` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java:59)

### `ReturnType`: its constants carry values

- [ ] `ReturnType.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/densityfunction/cellular/ReturnType.java:15)
- [ ] `ReturnType.ReturnType(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/densityfunction/cellular/ReturnType.java:18)
- [ ] `ReturnType.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/densityfunction/cellular/ReturnType.java:22)

### `FastNoiseConfig`: it implements Minecraft's `Noise`

- [ ] `FastNoiseConfig.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:11)
- [ ] `FastNoiseConfig.getCodec(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:18)
- [ ] `FastNoiseConfig.FastNoiseConfig(2)`: its class isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:20)
- [ ] `FastNoiseConfig.frequency(0)`: its class isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:28)
- [ ] `FastNoiseConfig.salt(0)`: its class isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:32)
- [ ] `FastNoiseConfig.bind(1)`: its class isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:36)
- [ ] `FastNoiseConfig.range(0)`: its class isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:40)
- [ ] `FastNoiseConfig.get(2)`: its class isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:44)
- [ ] `FastNoiseConfig.get(3)`: its class isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java:48)

### `FNL`: it uses FractalType, which isn't translated (its constants carry values)

- [x] `FNL.Gradients2D` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:24)
- [x] `FNL.RandVecs2D` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:282)
- [x] `FNL.Gradients3D` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:796)
- [x] `FNL.RandVecs3D` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:1054)
- [x] `FNL.PrimeX` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2080)
- [x] `FNL.PrimeY` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2081)
- [x] `FNL.PrimeZ` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2082)
- [ ] `FNL.FNL(0)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2084)
- [ ] `FNL.FNL(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2087)
- [ ] `FNL.SetSeed(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2091)
- [ ] `FNL.SetFrequency(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2095)
- [ ] `FNL.SetNoiseType(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2099)
- [ ] `FNL.SetRotationType3D(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2104)
- [ ] `FNL.SetFractalType(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2110)
- [ ] `FNL.SetFractalOctaves(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2114)
- [ ] `FNL.SetFractalLacunarity(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2119)
- [ ] `FNL.SetFractalGain(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2123)
- [ ] `FNL.SetFractalWeightedStrength(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2128)
- [ ] `FNL.SetFractalPingPongStrength(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2132)
- [ ] `FNL.SetCellularDistanceFunction(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2136)
- [ ] `FNL.SetCellularReturnType(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2140)
- [ ] `FNL.SetCellularJitter(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2144)
- [ ] `FNL.SetDomainWarpType(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2148)
- [ ] `FNL.SetDomainWarpAmp(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2153)
- [ ] `FNL.GetNoise(2)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2157)
- [ ] `FNL.GetNoise(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2182)
- [ ] `FNL.DomainWarp(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2225)
- [ ] `FNL.DomainWarp(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2238)
- [x] `FNL.FastMin(2)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2251)
- [x] `FNL.FastMax(2)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2255)
- [x] `FNL.FastAbs(1)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2259)
- [x] `FNL.FastSqrt(1)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2263)
- [x] `FNL.FastFloor(1)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2267)
- [x] `FNL.FastRound(1)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2271)
- [x] `FNL.Lerp(3)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2275)
- [x] `FNL.InterpHermite(1)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2279)
- [x] `FNL.InterpQuintic(1)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2283)
- [x] `FNL.CubicLerp(5)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2287)
- [x] `FNL.PingPong(1)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2292)
- [ ] `FNL.CalculateFractalBounding(0)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2297)
- [x] `FNL.Hash(3)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2310)
- [x] `FNL.Hash(4)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2315)
- [x] `FNL.ValCoord(3)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2320)
- [x] `FNL.ValCoord(4)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2327)
- [x] `FNL.GradCoord(5)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2334)
- [x] `FNL.GradCoord(7)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2343)
- [ ] `FNL.GenNoiseSingle(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2353)
- [ ] `FNL.GenNoiseSingle(4)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2372)
- [ ] `FNL.UpdateTransformType3D(0)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2391)
- [ ] `FNL.UpdateWarpTransformType3D(0)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2411)
- [ ] `FNL.GenFractalFBm(2)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2431)
- [ ] `FNL.GenFractalFBm(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2448)
- [ ] `FNL.GenFractalRidged(2)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2466)
- [ ] `FNL.GenFractalRidged(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2483)
- [ ] `FNL.GenFractalPingPong(2)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2501)
- [ ] `FNL.GenFractalPingPong(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2518)
- [ ] `FNL.SingleSimplex(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2536)
- [ ] `FNL.SingleOpenSimplex2(4)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2590)
- [ ] `FNL.SingleOpenSimplex2S(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2654)
- [ ] `FNL.SingleOpenSimplex2S(4)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2744)
- [ ] `FNL.SingleCellular(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2885)
- [ ] `FNL.SingleCellular(4)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:2981)
- [ ] `FNL.SinglePerlin(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3109)
- [ ] `FNL.SinglePerlin(4)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3127)
- [ ] `FNL.SingleValueCubic(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3155)
- [ ] `FNL.SingleValueCubic(4)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3178)
- [ ] `FNL.SingleValue(3)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3231)
- [ ] `FNL.SingleValue(4)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3245)
- [ ] `FNL.DoSingleDomainWarp(6)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3267)
- [ ] `FNL.DoSingleDomainWarp(7)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3280)
- [ ] `FNL.DomainWarpSingle(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3293)
- [ ] `FNL.DomainWarpSingle(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3312)
- [ ] `FNL.DomainWarpFractalProgressive(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3349)
- [ ] `FNL.DomainWarpFractalProgressive(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3374)
- [ ] `FNL.DomainWarpFractalIndependent(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3417)
- [ ] `FNL.DomainWarpFractalIndependent(1)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3442)
- [ ] `FNL.SingleDomainWarpBasicGrid(6)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3485)
- [ ] `FNL.SingleDomainWarpBasicGrid(7)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3508)
- [ ] `FNL.SingleDomainWarpSimplexGradient(7)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3552)
- [ ] `FNL.SingleDomainWarpOpenSimplex2Gradient(8)`: its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3684)

### `FNL.CellularDistanceFunction`


### `FNL.CellularReturnType`


### `FNL.DomainWarpType`


### `FNL.FractalType`: its constants carry values

- [ ] `FNL.FractalType.FractalType(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3853)
- [ ] `FNL.FractalType.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3857)

### `FNL.NoiseType`


### `FNL.RotationType3D`


### `FNL.TransformType3D`


### `FNL.Vector2`

- [x] `FNL.Vector2.Vector2(2)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3888)

### `FNL.Vector3`

- [x] `FNL.Vector3.Vector3(3)` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java:3899)

### `LithostitchedFastNoiseConfigs`: it's an interface

- [ ] `LithostitchedFastNoiseConfigs.cellular(5)`: it uses FastNoiseConfig, which isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/LithostitchedFastNoiseConfigs.java:9)
- [ ] `LithostitchedFastNoiseConfigs.perlin(2)`: it uses FastNoiseConfig, which isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/LithostitchedFastNoiseConfigs.java:15)
- [ ] `LithostitchedFastNoiseConfigs.simplex(6)`: it uses FastNoiseConfig, which isn't translated (it implements Minecraft's `Noise`) (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/LithostitchedFastNoiseConfigs.java:19)

### `LithostitchedDensityFunctions`: it's an interface

- [ ] `LithostitchedDensityFunctions.cos(1)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:19)
- [ ] `LithostitchedDensityFunctions.shift(4)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:23)
- [ ] `LithostitchedDensityFunctions.sin(1)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:27)
- [ ] `LithostitchedDensityFunctions.cellular2d(4)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:31)
- [ ] `LithostitchedDensityFunctions.cellular2d(8)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:35)
- [ ] `LithostitchedDensityFunctions.cellular3d(5)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:47)
- [ ] `LithostitchedDensityFunctions.cellular3d(9)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:51)
- [ ] `LithostitchedDensityFunctions.fastNoise(3)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:63)
- [ ] `LithostitchedDensityFunctions.fastNoise(6)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:67)
- [ ] `LithostitchedDensityFunctions.wrappedMarker(0)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:73)
- [ ] `LithostitchedDensityFunctions.originalMarker(0)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java:77)

### `LithostitchedFeatures`: it's an interface

- [ ] `LithostitchedFeatures.placeAll(1)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:33)
- [ ] `LithostitchedFeatures.placeUntilFailure(1)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:37)
- [ ] `LithostitchedFeatures.placeUntilSuccess(1)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:41)
- [ ] `LithostitchedFeatures.dungeon(9)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:45)
- [ ] `LithostitchedFeatures.largeDripstone(11)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:59)
- [ ] `LithostitchedFeatures.ore(2)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:87)
- [ ] `LithostitchedFeatures.placed(1)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:91)
- [ ] `LithostitchedFeatures.select(1)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:95)
- [ ] `LithostitchedFeatures.structureTemplate(2)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:99)
- [ ] `LithostitchedFeatures.structureTemplate(5)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:103)
- [ ] `LithostitchedFeatures.structureTemplate(2)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:109)
- [ ] `LithostitchedFeatures.structureTemplate(5)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:113)
- [ ] `LithostitchedFeatures.vines(3)`: it uses Minecraft's `Feature` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java:123)

### `LithostitchedMaterialConditions`: it's an interface

- [ ] `LithostitchedMaterialConditions.allOf(1)`: it uses Minecraft's `MaterialCondition` (dev/worldgen/lithostitched/api/worldgen/material/LithostitchedMaterialConditions.java:13)
- [ ] `LithostitchedMaterialConditions.anyOf(1)`: it uses Minecraft's `MaterialCondition` (dev/worldgen/lithostitched/api/worldgen/material/LithostitchedMaterialConditions.java:17)
- [ ] `LithostitchedMaterialConditions.slope(2)`: it uses Minecraft's `MaterialCondition` (dev/worldgen/lithostitched/api/worldgen/material/LithostitchedMaterialConditions.java:21)
- [ ] `LithostitchedMaterialConditions.sampleDensity(4)`: it uses Minecraft's `MaterialCondition` (dev/worldgen/lithostitched/api/worldgen/material/LithostitchedMaterialConditions.java:25)

### `LithostitchedMaterialRules`: it's an interface

- [ ] `LithostitchedMaterialRules.bandlands(1)`: it uses Minecraft's `MaterialRule` (dev/worldgen/lithostitched/api/worldgen/material/LithostitchedMaterialRules.java:9)

### `BiomeEffectsBuilder`: it's an interface

- [ ] `BiomeEffectsBuilder.create(0)`: it uses BiomeEffectsBuilder, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java:8)
- [ ] `BiomeEffectsBuilder.waterColor(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java:12)
- [ ] `BiomeEffectsBuilder.foliageColor(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java:14)
- [ ] `BiomeEffectsBuilder.dryFoliageColor(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java:16)
- [ ] `BiomeEffectsBuilder.grassColor(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java:18)
- [ ] `BiomeEffectsBuilder.grassColorModifier(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java:20)
- [ ] `BiomeEffectsBuilder.build(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java:22)

### `HolderTarget`: it's an interface

- [ ] `HolderTarget.getHolders(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:22)
- [ ] `HolderTarget.getType(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:24)
- [ ] `HolderTarget.getHoldersAsSet(1)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:26)
- [ ] `HolderTarget.of(1)`: it's generic (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:36)
- [ ] `HolderTarget.of(1)`: it's generic (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:40)
- [ ] `HolderTarget.createCodec(1)`: it's generic (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:44)

### `HolderTarget.SimpleType`: it's an interface

- [ ] `HolderTarget.SimpleType.apply(1)`: it's generic (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:55)
- [ ] `HolderTarget.SimpleType.applySimple(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:60)

### `HolderTarget.Type`: it's an interface

- [ ] `HolderTarget.Type.apply(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java:64)

### `WorldgenModifier`: it's an interface

- [ ] `WorldgenModifier.predicate(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:75)
- [ ] `WorldgenModifier.priority(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:77)
- [ ] `WorldgenModifier.apply(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:79)
- [ ] `WorldgenModifier.codec(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:81)
- [ ] `WorldgenModifier.shouldRecompileSortedFeatures(0)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:83)
- [ ] `WorldgenModifier.resetRegistrationInfo(2)`: it's generic (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:87)
- [ ] `WorldgenModifier.builder(0)`: it uses ModifierBuilder, which isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:99)
- [ ] `WorldgenModifier.builder(1)`: it uses ModifierBuilder, which isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:103)

### `WorldgenModifier.ModifierBuilder`: it uses LoadPredicate, which isn't translated (it's an interface)

- [ ] `WorldgenModifier.ModifierBuilder.ModifierBuilder(1)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:111)
- [ ] `WorldgenModifier.ModifierBuilder.priority(1)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:115)
- [ ] `WorldgenModifier.ModifierBuilder.addBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:120)
- [ ] `WorldgenModifier.ModifierBuilder.addBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:126)
- [ ] `WorldgenModifier.ModifierBuilder.addBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:130)
- [ ] `WorldgenModifier.ModifierBuilder.addFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:134)
- [ ] `WorldgenModifier.ModifierBuilder.addFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:144)
- [ ] `WorldgenModifier.ModifierBuilder.addFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:150)
- [ ] `WorldgenModifier.ModifierBuilder.addFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:156)
- [ ] `WorldgenModifier.ModifierBuilder.addFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:162)
- [ ] `WorldgenModifier.ModifierBuilder.addFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:166)
- [ ] `WorldgenModifier.ModifierBuilder.addProcessorListProcessors(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:170)
- [ ] `WorldgenModifier.ModifierBuilder.addProcessorListProcessors(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:179)
- [ ] `WorldgenModifier.ModifierBuilder.addProcessorListProcessors(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:185)
- [ ] `WorldgenModifier.ModifierBuilder.addSpawnCosts(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:191)
- [ ] `WorldgenModifier.ModifierBuilder.addSpawnCosts(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:197)
- [ ] `WorldgenModifier.ModifierBuilder.addSpawnCosts(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:201)
- [ ] `WorldgenModifier.ModifierBuilder.addStructureSetEntries(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:205)
- [ ] `WorldgenModifier.ModifierBuilder.addStructureSetEntries(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:211)
- [ ] `WorldgenModifier.ModifierBuilder.addStructureSetEntries(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:217)
- [ ] `WorldgenModifier.ModifierBuilder.addTemplatePoolElements(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:221)
- [ ] `WorldgenModifier.ModifierBuilder.addTemplatePoolElements(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:228)
- [ ] `WorldgenModifier.ModifierBuilder.addTemplatePoolElements(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:235)
- [ ] `WorldgenModifier.ModifierBuilder.noop(0)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:240)
- [ ] `WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:244)
- [ ] `WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:253)
- [ ] `WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:259)
- [ ] `WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:265)
- [ ] `WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:271)
- [ ] `WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:275)
- [ ] `WorldgenModifier.ModifierBuilder.removeFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:279)
- [ ] `WorldgenModifier.ModifierBuilder.removeFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:289)
- [ ] `WorldgenModifier.ModifierBuilder.removeFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:295)
- [ ] `WorldgenModifier.ModifierBuilder.removeFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:301)
- [ ] `WorldgenModifier.ModifierBuilder.removeFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:307)
- [ ] `WorldgenModifier.ModifierBuilder.removeFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:311)
- [ ] `WorldgenModifier.ModifierBuilder.removeStructureSetEntries(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:315)
- [ ] `WorldgenModifier.ModifierBuilder.removeStructureSetEntries(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:322)
- [ ] `WorldgenModifier.ModifierBuilder.removeStructureSetEntries(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:329)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeAttributes(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:334)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeAttributes(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:340)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeAttributes(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:346)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeAttributes(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:352)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeClimate(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:356)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeClimate(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:362)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeClimate(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:366)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeEffects(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:370)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeEffects(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:379)
- [ ] `WorldgenModifier.ModifierBuilder.setBiomeEffects(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:388)
- [ ] `WorldgenModifier.ModifierBuilder.setMaterialRule(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:394)
- [ ] `WorldgenModifier.ModifierBuilder.setMaterialRule(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:404)
- [ ] `WorldgenModifier.ModifierBuilder.setMaterialRule(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:414)
- [ ] `WorldgenModifier.ModifierBuilder.setMaterialRule(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:420)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolAliases(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:424)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolAliases(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:434)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolAliases(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:440)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolElementProcessors(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:444)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolElementProcessors(4)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:455)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolElementProcessors(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:468)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolElementProcessors(4)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:474)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolElementProcessors(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:482)
- [ ] `WorldgenModifier.ModifierBuilder.setPoolElementProcessors(4)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:488)
- [ ] `WorldgenModifier.ModifierBuilder.setStructureSpawnCondition(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:496)
- [ ] `WorldgenModifier.ModifierBuilder.setStructureSpawnCondition(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:506)
- [ ] `WorldgenModifier.ModifierBuilder.setStructureSpawnCondition(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:512)
- [ ] `WorldgenModifier.ModifierBuilder.stackFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:518)
- [ ] `WorldgenModifier.ModifierBuilder.stackFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:528)
- [ ] `WorldgenModifier.ModifierBuilder.stackFeatures(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:534)
- [ ] `WorldgenModifier.ModifierBuilder.wrapAquifers(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:538)
- [ ] `WorldgenModifier.ModifierBuilder.wrapDensityFunction(2)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:542)
- [ ] `WorldgenModifier.ModifierBuilder.wrapNoiseRouter(3)`: its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)) (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java:546)

### `LithostitchedPlacementConditions`: it's an interface

- [ ] `LithostitchedPlacementConditions.allOf(1)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:27)
- [ ] `LithostitchedPlacementConditions.anyOf(1)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:31)
- [ ] `LithostitchedPlacementConditions.grid(4)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:35)
- [ ] `LithostitchedPlacementConditions.absoluteHeightFilter(2)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:39)
- [ ] `LithostitchedPlacementConditions.heightmapRelativeFilter(2)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:43)
- [ ] `LithostitchedPlacementConditions.inBiome(1)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:47)
- [ ] `LithostitchedPlacementConditions.inBiome(1)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:51)
- [ ] `LithostitchedPlacementConditions.multipleOf(2)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:55)
- [ ] `LithostitchedPlacementConditions.not(1)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:59)
- [ ] `LithostitchedPlacementConditions.offset(2)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:63)
- [ ] `LithostitchedPlacementConditions.sampleDensity(2)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:67)
- [ ] `LithostitchedPlacementConditions.sampleNoiseRouter(2)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:71)
- [ ] `LithostitchedPlacementConditions.alwaysTrue(0)`: it uses PlacementCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java:75)

### `PlacementCondition`: it's an interface

- [ ] `PlacementCondition.test(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:32)
- [ ] `PlacementCondition.test(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:34)
- [ ] `PlacementCondition.test(2)`: it has no body (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:38)
- [ ] `PlacementCondition.codec(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:42)

### `PlacementCondition.Context`: it uses Minecraft's `RegistryAccess`

- [ ] `PlacementCondition.Context.Context(6)`: its class isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:44)
- [ ] `PlacementCondition.Context.create(1)`: it uses Context, which isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:47)
- [ ] `PlacementCondition.Context.create(1)`: it uses Context, which isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:53)
- [ ] `PlacementCondition.Context.registries(0)`: its class isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:44)
- [ ] `PlacementCondition.Context.generator(0)`: its class isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:44)
- [ ] `PlacementCondition.Context.heightAccessor(0)`: its class isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:44)
- [ ] `PlacementCondition.Context.randomState(0)`: its class isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:44)
- [ ] `PlacementCondition.Context.biomeResolver(0)`: its class isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:44)
- [ ] `PlacementCondition.Context.seed(0)`: its class isn't translated (it uses Minecraft's `RegistryAccess`) (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java:44)

### `LithostitchedPlacementModifiers`: it's an interface

- [ ] `LithostitchedPlacementModifiers.condition(1)`: it uses Minecraft's `PlacementModifier` (dev/worldgen/lithostitched/api/worldgen/placementmodifier/LithostitchedPlacementModifiers.java:11)
- [ ] `LithostitchedPlacementModifiers.noiseSlope(5)`: it uses Minecraft's `PlacementModifier` (dev/worldgen/lithostitched/api/worldgen/placementmodifier/LithostitchedPlacementModifiers.java:15)

### `DelegatingElementBuilder`: it's an interface

- [ ] `DelegatingElementBuilder.create(1)`: it uses DelegatingElementBuilder, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:13)
- [ ] `DelegatingElementBuilder.forcedCount(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:17)
- [ ] `DelegatingElementBuilder.limitedCount(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:19)
- [ ] `DelegatingElementBuilder.allowedDepth(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:21)
- [ ] `DelegatingElementBuilder.terrainAdaptation(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:23)
- [ ] `DelegatingElementBuilder.condition(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:25)
- [ ] `DelegatingElementBuilder.named(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:27)
- [ ] `DelegatingElementBuilder.terrainMatchingHeightmap(1)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:29)
- [ ] `DelegatingElementBuilder.allowBoundingBoxCollisions(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:31)
- [ ] `DelegatingElementBuilder.otherPiecesCanIntersect(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:33)
- [ ] `DelegatingElementBuilder.build(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java:35)

### `LithostitchedPoolElements`: it's an interface

- [ ] `LithostitchedPoolElements.guaranteed(2)`: it uses Minecraft's `StructurePoolElement` (dev/worldgen/lithostitched/api/worldgen/poolelement/LithostitchedPoolElements.java:9)
- [ ] `LithostitchedPoolElements.limited(2)`: it uses Minecraft's `StructurePoolElement` (dev/worldgen/lithostitched/api/worldgen/poolelement/LithostitchedPoolElements.java:13)
- [ ] `LithostitchedPoolElements.delegating(2)`: it uses Minecraft's `StructurePoolElement` (dev/worldgen/lithostitched/api/worldgen/poolelement/LithostitchedPoolElements.java:17)

### `LithostitchedProcessorConditions`: it's an interface

- [ ] `LithostitchedProcessorConditions.allOf(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:22)
- [ ] `LithostitchedProcessorConditions.anyOf(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:26)
- [ ] `LithostitchedProcessorConditions.matchingBiomes(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:30)
- [ ] `LithostitchedProcessorConditions.matchingBiomes(2)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:34)
- [ ] `LithostitchedProcessorConditions.matchingBiomes(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:38)
- [ ] `LithostitchedProcessorConditions.matchingBiomes(2)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:42)
- [ ] `LithostitchedProcessorConditions.matchingBlocks(3)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:46)
- [ ] `LithostitchedProcessorConditions.matchingBlocks(3)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:50)
- [ ] `LithostitchedProcessorConditions.matchingBlocks(3)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:54)
- [ ] `LithostitchedProcessorConditions.not(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:58)
- [ ] `LithostitchedProcessorConditions.piecePosition(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:62)
- [ ] `LithostitchedProcessorConditions.structureStartPosition(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:66)
- [ ] `LithostitchedProcessorConditions.randomChance(1)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:70)
- [ ] `LithostitchedProcessorConditions.alwaysTrue(0)`: it uses ProcessorCondition, which isn't translated (it's an interface) (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java:74)

### `ProcessorCondition`: it's an interface

- [ ] `ProcessorCondition.test(4)`: it has no body (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java:28)
- [ ] `ProcessorCondition.codec(0)`: it has no body (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java:30)

### `ProcessorCondition.Data`: it uses Minecraft's `BlockPos`

- [ ] `ProcessorCondition.Data.Data(4)`: its class isn't translated (it uses Minecraft's `BlockPos`) (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java:32)
- [ ] `ProcessorCondition.Data.pos(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java:32)
- [ ] `ProcessorCondition.Data.pivot(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java:32)
- [ ] `ProcessorCondition.Data.relative(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java:32)
- [ ] `ProcessorCondition.Data.absolute(0)`: its class isn't translated (it uses Minecraft's `BlockPos`) (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java:32)

### `BlockType`: its constants carry values

- [ ] `BlockType.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/processor/enums/BlockType.java:13)
- [ ] `BlockType.BlockType(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/BlockType.java:17)
- [ ] `BlockType.state(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/BlockType.java:22)
- [ ] `BlockType.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/BlockType.java:26)

### `ProcessorPosition`: its constants carry values

- [ ] `ProcessorPosition.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/processor/enums/ProcessorPosition.java:14)
- [ ] `ProcessorPosition.ProcessorPosition(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/ProcessorPosition.java:17)
- [ ] `ProcessorPosition.select(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/ProcessorPosition.java:21)
- [ ] `ProcessorPosition.select(3)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/ProcessorPosition.java:25)
- [ ] `ProcessorPosition.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/ProcessorPosition.java:33)

### `RandomMode`: its constants carry values

- [ ] `RandomMode.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/processor/enums/RandomMode.java:13)
- [ ] `RandomMode.RandomMode(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/RandomMode.java:16)
- [ ] `RandomMode.select(3)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/RandomMode.java:20)
- [ ] `RandomMode.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/processor/enums/RandomMode.java:28)

### `LithostitchedProcessorLists`: it's an interface

- [ ] `LithostitchedProcessorLists.key(1)`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessorLists.java:36)
- [ ] `LithostitchedProcessorLists.pick(1)`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessorLists.java:40)

### `LithostitchedProcessors`: it's an interface

- [ ] `LithostitchedProcessors.applyRandom(2)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:26)
- [ ] `LithostitchedProcessors.applyRandom(2)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:30)
- [ ] `LithostitchedProcessors.blockSwap(1)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:34)
- [ ] `LithostitchedProcessors.condition(3)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:38)
- [ ] `LithostitchedProcessors.condition(4)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:42)
- [ ] `LithostitchedProcessors.discardInput(0)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:48)
- [ ] `LithostitchedProcessors.reference(1)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:52)
- [ ] `LithostitchedProcessors.reference(1)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:56)
- [ ] `LithostitchedProcessors.scheduleTick(0)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:60)
- [ ] `LithostitchedProcessors.setBlock(4)`: it uses Minecraft's `StructureProcessor` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java:64)

### `RandomSettings`: it uses RandomMode, which isn't translated (its constants carry values)

- [ ] `RandomSettings.FULL_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java:14)
- [ ] `RandomSettings.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java:21)
- [ ] `RandomSettings.RandomSettings(2)`: its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java:13)
- [ ] `RandomSettings.RandomSettings(1)`: its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java:23)
- [ ] `RandomSettings.create(4)`: its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java:27)
- [ ] `RandomSettings.mode(0)`: its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java:13)
- [ ] `RandomSettings.name(0)`: its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)) (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java:13)

### `LithostitchedStateProviders`: it's an interface

- [ ] `LithostitchedStateProviders.weighted(1)`: it uses Minecraft's `BlockStateProvider` (dev/worldgen/lithostitched/api/worldgen/stateprovider/LithostitchedStateProviders.java:9)

### `LithostitchedStructures`: it's an interface

- [ ] `LithostitchedStructures.jigsaw(6)`: it uses Minecraft's `Structure` (dev/worldgen/lithostitched/api/worldgen/structure/LithostitchedStructures.java:24)
- [ ] `LithostitchedStructures.jigsaw(12)`: it uses Minecraft's `Structure` (dev/worldgen/lithostitched/api/worldgen/structure/LithostitchedStructures.java:50)
- [ ] `LithostitchedStructures.delegating(2)`: it uses Minecraft's `Structure` (dev/worldgen/lithostitched/api/worldgen/structure/LithostitchedStructures.java:82)

### `MaxDistanceFromCenter`

- [ ] `MaxDistanceFromCenter.BASE_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:11)
- [ ] `MaxDistanceFromCenter.FULL_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:12)
- [ ] `MaxDistanceFromCenter.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:19)
- [x] `MaxDistanceFromCenter.MaxDistanceFromCenter(2)` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:10)
- [x] `MaxDistanceFromCenter.MaxDistanceFromCenter(1)` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:25)
- [x] `MaxDistanceFromCenter.of(1)` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:29)
- [x] `MaxDistanceFromCenter.of(2)` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:33)
- [x] `MaxDistanceFromCenter.horizontal(0)` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:10)
- [x] `MaxDistanceFromCenter.vertical(0)` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java:10)

### `SurfaceSnap`: its constants carry values

- [ ] `SurfaceSnap.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/structure/SurfaceSnap.java:16)
- [ ] `SurfaceSnap.SurfaceSnap(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/structure/SurfaceSnap.java:20)
- [ ] `SurfaceSnap.findY(4)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/structure/SurfaceSnap.java:25)
- [ ] `SurfaceSnap.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/structure/SurfaceSnap.java:43)

### `AquiferTarget`: its constants carry values

- [ ] `AquiferTarget.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/util/AquiferTarget.java:17)
- [ ] `AquiferTarget.AquiferTarget(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/util/AquiferTarget.java:21)
- [ ] `AquiferTarget.getDensityFunction(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/util/AquiferTarget.java:26)
- [ ] `AquiferTarget.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/util/AquiferTarget.java:30)

### `BiomeClimate`: it uses Minecraft's `TemperatureModifier`

- [ ] `BiomeClimate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/api/worldgen/util/BiomeClimate.java:12)
- [ ] `BiomeClimate.BiomeClimate(4)`: its class isn't translated (it uses Minecraft's `TemperatureModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeClimate.java:9)
- [ ] `BiomeClimate.hasPrecipitation(0)`: its class isn't translated (it uses Minecraft's `TemperatureModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeClimate.java:9)
- [ ] `BiomeClimate.temperature(0)`: its class isn't translated (it uses Minecraft's `TemperatureModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeClimate.java:9)
- [ ] `BiomeClimate.temperatureModifier(0)`: its class isn't translated (it uses Minecraft's `TemperatureModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeClimate.java:9)
- [ ] `BiomeClimate.downfall(0)`: its class isn't translated (it uses Minecraft's `TemperatureModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeClimate.java:9)

### `BiomeEffects`: it uses Minecraft's `GrassColorModifier`

- [ ] `BiomeEffects.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java:15)
- [ ] `BiomeEffects.BiomeEffects(5)`: its class isn't translated (it uses Minecraft's `GrassColorModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java:8)
- [ ] `BiomeEffects.waterColor(0)`: its class isn't translated (it uses Minecraft's `GrassColorModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java:8)
- [ ] `BiomeEffects.foliageColor(0)`: its class isn't translated (it uses Minecraft's `GrassColorModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java:8)
- [ ] `BiomeEffects.dryFoliageColor(0)`: its class isn't translated (it uses Minecraft's `GrassColorModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java:8)
- [ ] `BiomeEffects.grassColor(0)`: its class isn't translated (it uses Minecraft's `GrassColorModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java:8)
- [ ] `BiomeEffects.grassColorModifier(0)`: its class isn't translated (it uses Minecraft's `GrassColorModifier`) (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java:8)

### `DensityFunctionWrapper`: it implements Minecraft's `CompileContext`

- [ ] `DensityFunctionWrapper.DensityFunctionWrapper(2)`: its class isn't translated (it implements Minecraft's `CompileContext`) (dev/worldgen/lithostitched/api/worldgen/util/DensityFunctionWrapper.java:25)
- [ ] `DensityFunctionWrapper.DensityFunctionWrapper(4)`: its class isn't translated (it implements Minecraft's `CompileContext`) (dev/worldgen/lithostitched/api/worldgen/util/DensityFunctionWrapper.java:29)
- [ ] `DensityFunctionWrapper.newLegacyInstance(1)`: its class isn't translated (it implements Minecraft's `CompileContext`) (dev/worldgen/lithostitched/api/worldgen/util/DensityFunctionWrapper.java:36)
- [ ] `DensityFunctionWrapper.createNoiseSampler(1)`: its class isn't translated (it implements Minecraft's `CompileContext`) (dev/worldgen/lithostitched/api/worldgen/util/DensityFunctionWrapper.java:40)
- [ ] `DensityFunctionWrapper.createRandom(1)`: its class isn't translated (it implements Minecraft's `CompileContext`) (dev/worldgen/lithostitched/api/worldgen/util/DensityFunctionWrapper.java:50)
- [ ] `DensityFunctionWrapper.createEndIslandRandom(0)`: its class isn't translated (it implements Minecraft's `CompileContext`) (dev/worldgen/lithostitched/api/worldgen/util/DensityFunctionWrapper.java:54)

### `NoiseRouterTarget`: its constants carry values

- [ ] `NoiseRouterTarget.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/api/worldgen/util/NoiseRouterTarget.java:19)
- [ ] `NoiseRouterTarget.NoiseRouterTarget(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/util/NoiseRouterTarget.java:23)
- [ ] `NoiseRouterTarget.getDensityFunction(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/util/NoiseRouterTarget.java:28)
- [ ] `NoiseRouterTarget.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/api/worldgen/util/NoiseRouterTarget.java:32)

### `ConfigState`: it implements `ApollibCopyable` (dev.worldgen.apollib.config)

- [ ] `ConfigState.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/config/ConfigState.java:9)
- [ ] `ConfigState.DEFAULT`: it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/lithostitched/impl/config/ConfigState.java:23)
- [ ] `ConfigState.ConfigState(3)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/lithostitched/impl/config/ConfigState.java:28)
- [ ] `ConfigState.copy(0)`: its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)) (dev/worldgen/lithostitched/impl/config/ConfigState.java:34)

### `LithostitchedDebugging`

- [ ] `LithostitchedDebugging.init(0)`: unknown static AddWorldgenModifiersEvent.EVENT (dev/worldgen/lithostitched/impl/debug/LithostitchedDebugging.java:16)
- [x] `LithostitchedDebugging.id(1)` (dev/worldgen/lithostitched/impl/debug/LithostitchedDebugging.java:45)

### `AttributeLayerBuilderDuck`: it's an interface

- [ ] `AttributeLayerBuilderDuck.addPositionalTimeBasedLayer(2)`: it has no body (dev/worldgen/lithostitched/impl/duck/AttributeLayerBuilderDuck.java:8)

### `AttributeMapDuck`: it's an interface

- [ ] `AttributeMapDuck.clear(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/AttributeMapDuck.java:4)

### `BiomeTimelineDuck`: it's an interface

- [ ] `BiomeTimelineDuck.lithostitched$addTimeline(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/BiomeTimelineDuck.java:9)
- [ ] `BiomeTimelineDuck.lithostitched$setTimelines(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/BiomeTimelineDuck.java:11)
- [ ] `BiomeTimelineDuck.lithostitched$getTimelines(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/BiomeTimelineDuck.java:13)
- [ ] `BiomeTimelineDuck.cast(1)`: it uses BiomeTimelineDuck, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/duck/BiomeTimelineDuck.java:15)

### `BiomeWeightingDuck`: it's an interface

- [ ] `BiomeWeightingDuck.lithostitched$clear(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/BiomeWeightingDuck.java:8)
- [ ] `BiomeWeightingDuck.lithostitched$accumulate(2)`: it has no body (dev/worldgen/lithostitched/impl/duck/BiomeWeightingDuck.java:10)
- [ ] `BiomeWeightingDuck.lithostitched$getWeights(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/BiomeWeightingDuck.java:12)

### `MansionRoomDuck`: it's an interface

- [ ] `MansionRoomDuck.lithostitched$floorNumber(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/MansionRoomDuck.java:7)
- [ ] `MansionRoomDuck.lithostitched$getRandom(2)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/duck/MansionRoomDuck.java:9)

### `MaterialSystemAccessor`: it's an interface

- [ ] `MaterialSystemAccessor.getSystem(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/MaterialSystemAccessor.java:6)

### `MNBSDuck`: it's an interface

- [ ] `MNBSDuck.lithostitched$getEntries(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/mnbs/MNBSDuck.java:10)
- [ ] `MNBSDuck.lithostitched$setEntries(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/mnbs/MNBSDuck.java:12)

### `MNBSPLDuck`: it's an interface

- [ ] `MNBSPLDuck.lithostitched$setParameters(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/mnbs/MNBSPLDuck.java:9)
- [ ] `MNBSPLDuck.lithostitched$setMigrationBiome(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/mnbs/MNBSPLDuck.java:11)
- [ ] `MNBSPLDuck.lithostitched$getMigrationBiome(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/mnbs/MNBSPLDuck.java:13)

### `ReferencePosDuck`: it's an interface

- [ ] `ReferencePosDuck.lithostitched$setReferencePos(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/ReferencePosDuck.java:6)
- [ ] `ReferencePosDuck.lithostitched$getReferencePos(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/ReferencePosDuck.java:8)

### `RegistryHolder`: it's an interface

- [ ] `RegistryHolder.getRegistries(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/RegistryHolder.java:6)
- [ ] `RegistryHolder.setRegistries(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/RegistryHolder.java:8)

### `SeedAccessor`: it's an interface

- [ ] `SeedAccessor.getSeed(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/SeedAccessor.java:4)

### `StructureAttributesHolder`: it's an interface

- [ ] `StructureAttributesHolder.getPreviousStructureAttributes(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java:8)
- [ ] `StructureAttributesHolder.getStructureAttributes(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java:10)
- [ ] `StructureAttributesHolder.updateStructureAttributes(1)`: it has no body (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java:12)
- [ ] `StructureAttributesHolder.getTicksSinceUpdated(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java:14)
- [ ] `StructureAttributesHolder.incrementTicksSinceUpdated(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java:16)
- [ ] `StructureAttributesHolder.shouldFadeMusic(0)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java:18)
- [ ] `StructureAttributesHolder.from(1)`: it uses StructureAttributesHolder, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java:24)

### `StructurePoolAccess`: it's an interface

- [ ] `StructurePoolAccess.getLithostitchedTemplates(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/StructurePoolAccess.java:6)
- [ ] `StructurePoolAccess.compileRawTemplates(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/StructurePoolAccess.java:8)

### `SurfaceSystemAccessor`: it's an interface

- [ ] `SurfaceSystemAccessor.getBandOffsetNoise(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/SurfaceSystemAccessor.java:7)
- [ ] `SurfaceSystemAccessor.getNoiseRandom(0)`: it has no body (dev/worldgen/lithostitched/impl/duck/SurfaceSystemAccessor.java:9)

### `LithostitchedEvent`: it's generic

- [ ] `LithostitchedEvent.LithostitchedEvent(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/event/LithostitchedEvent.java:11)
- [ ] `LithostitchedEvent.register(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/event/LithostitchedEvent.java:15)
- [ ] `LithostitchedEvent.invoker(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/event/LithostitchedEvent.java:19)

### `Lithostitched`

- [x] `Lithostitched.MOD_ID` (dev/worldgen/lithostitched/impl/Lithostitched.java:16)
- [x] `Lithostitched.LOGGER` (dev/worldgen/lithostitched/impl/Lithostitched.java:17)
- [ ] `Lithostitched.CONFIG`: it uses `ApollibConfigHolder` (dev.worldgen.apollib.config) (dev/worldgen/lithostitched/impl/Lithostitched.java:18)
- [ ] `Lithostitched.REGISTRAR`: it uses `ApollibRegistrar` (dev.worldgen.apollib.registry) (dev/worldgen/lithostitched/impl/Lithostitched.java:21)
- [ ] `Lithostitched.init(0)`: it uses Lithostitched.CONFIG, which isn't translated (it uses `ApollibConfigHolder` (dev.worldgen.apollib.config)) (dev/worldgen/lithostitched/impl/Lithostitched.java:23)
- [ ] `Lithostitched.key(2)`: it's generic (dev/worldgen/lithostitched/impl/Lithostitched.java:36)
- [x] `Lithostitched.id(1)` (dev/worldgen/lithostitched/impl/Lithostitched.java:40)
- [x] `Lithostitched.vanillaToLithostitched(1)` (dev/worldgen/lithostitched/impl/Lithostitched.java:44)
- [ ] `Lithostitched.registry(2)`: it's generic (dev/worldgen/lithostitched/impl/Lithostitched.java:48)
- [ ] `Lithostitched.breaksSeedParity(0)`: it uses Lithostitched.CONFIG, which isn't translated (it uses `ApollibConfigHolder` (dev.worldgen.apollib.config)) (dev/worldgen/lithostitched/impl/Lithostitched.java:52)
- [ ] `Lithostitched.debug(2)`: it uses `Object` (dev/worldgen/lithostitched/impl/Lithostitched.java:56)

### `LithostitchedCodecs`: it's an interface

- [ ] `LithostitchedCodecs.holderTarget(2)`: it's generic (dev/worldgen/lithostitched/impl/LithostitchedCodecs.java:52)
- [ ] `LithostitchedCodecs.compactWeightedList(2)`: it's generic (dev/worldgen/lithostitched/impl/LithostitchedCodecs.java:56)

### `LithostitchedInternalHooks`

- [x] `LithostitchedInternalHooks.WORLD_SEED` (dev/worldgen/lithostitched/impl/LithostitchedInternalHooks.java:15)
- [ ] `LithostitchedInternalHooks.onServerAboutToStart(1)`: it uses Minecraft's `MinecraftServer` (dev/worldgen/lithostitched/impl/LithostitchedInternalHooks.java:17)
- [ ] `LithostitchedInternalHooks.applyModifiersAndInjections(4)`: it uses Minecraft's `MinecraftServer` (dev/worldgen/lithostitched/impl/LithostitchedInternalHooks.java:23)

### `ApplyStructureAttributesPacket`: it implements Minecraft's `CustomPacketPayload`

- [ ] `ApplyStructureAttributesPacket.TYPE`: it uses Type, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:17)
- [ ] `ApplyStructureAttributesPacket.CODEC`: it uses Minecraft's `StreamCodec` (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:18)
- [ ] `ApplyStructureAttributesPacket.ApplyStructureAttributesPacket(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:16)
- [ ] `ApplyStructureAttributesPacket.createEmpty(0)`: it uses ApplyStructureAttributesPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:22)
- [ ] `ApplyStructureAttributesPacket.create(2)`: it uses ApplyStructureAttributesPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`) (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:26)
- [ ] `ApplyStructureAttributesPacket.getAttributes(1)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:33)
- [ ] `ApplyStructureAttributesPacket.type(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:43)
- [ ] `ApplyStructureAttributesPacket.attributes(0)`: its class isn't translated (it implements Minecraft's `CustomPacketPayload`) (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java:16)

### `LithostitchedFabric`: it implements Fabric's `ModInitializer`

- [ ] `LithostitchedFabric.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (dev/worldgen/lithostitched/impl/platform/fabric/LithostitchedFabric.java:13)

### `LithostitchedFabricClient`: it implements Fabric's `ClientModInitializer`

- [ ] `LithostitchedFabricClient.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (dev/worldgen/lithostitched/impl/platform/fabric/LithostitchedFabricClient.java:25)

### `LithostitchedPlatform`

- [x] `LithostitchedPlatform.isModLoaded(1)` (dev/worldgen/lithostitched/impl/platform/LithostitchedPlatform.java:14)
- [ ] `LithostitchedPlatform.rebuildSettings(2)`: it uses Minecraft's `Biome` (dev/worldgen/lithostitched/impl/platform/LithostitchedPlatform.java:18)
- [ ] `LithostitchedPlatform.memoize(1)`: it's generic (dev/worldgen/lithostitched/impl/platform/LithostitchedPlatform.java:25)

### `AllOfPredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `AllOfPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/predicate/AllOfPredicate.java:9)
- [ ] `AllOfPredicate.AllOfPredicate(1)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AllOfPredicate.java:8)
- [ ] `AllOfPredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AllOfPredicate.java:13)
- [ ] `AllOfPredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AllOfPredicate.java:24)
- [ ] `AllOfPredicate.predicates(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AllOfPredicate.java:8)

### `AnyOfPredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `AnyOfPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/predicate/AnyOfPredicate.java:9)
- [ ] `AnyOfPredicate.AnyOfPredicate(1)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AnyOfPredicate.java:8)
- [ ] `AnyOfPredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AnyOfPredicate.java:13)
- [ ] `AnyOfPredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AnyOfPredicate.java:24)
- [ ] `AnyOfPredicate.predicates(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/AnyOfPredicate.java:8)

### `LoaderPredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `LoaderPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/predicate/LoaderPredicate.java:9)
- [x] `LoaderPredicate.CURRENT_LOADER` (dev/worldgen/lithostitched/impl/predicate/LoaderPredicate.java:12)
- [ ] `LoaderPredicate.LoaderPredicate(1)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/LoaderPredicate.java:8)
- [ ] `LoaderPredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/LoaderPredicate.java:14)
- [ ] `LoaderPredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/LoaderPredicate.java:19)
- [ ] `LoaderPredicate.loader(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/LoaderPredicate.java:8)

### `ModLoadedPredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `ModLoadedPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/predicate/ModLoadedPredicate.java:10)
- [ ] `ModLoadedPredicate.ModLoadedPredicate(1)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/ModLoadedPredicate.java:9)
- [ ] `ModLoadedPredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/ModLoadedPredicate.java:14)
- [ ] `ModLoadedPredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/ModLoadedPredicate.java:19)
- [ ] `ModLoadedPredicate.modId(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/ModLoadedPredicate.java:9)

### `NotPredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `NotPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/predicate/NotPredicate.java:8)
- [ ] `NotPredicate.NotPredicate(1)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/NotPredicate.java:7)
- [ ] `NotPredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/NotPredicate.java:12)
- [ ] `NotPredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/NotPredicate.java:17)
- [ ] `NotPredicate.predicate(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/NotPredicate.java:7)

### `PackFormatPredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `PackFormatPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/predicate/PackFormatPredicate.java:12)
- [ ] `PackFormatPredicate.PackFormatPredicate(1)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/PackFormatPredicate.java:11)
- [ ] `PackFormatPredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/PackFormatPredicate.java:17)
- [ ] `PackFormatPredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/PackFormatPredicate.java:22)
- [ ] `PackFormatPredicate.supportedFormats(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/PackFormatPredicate.java:11)

### `StubException`: it extends `Exception`


### `TruePredicate`: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)

- [ ] `TruePredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/predicate/TruePredicate.java:7)
- [ ] `TruePredicate.TruePredicate(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/TruePredicate.java:6)
- [ ] `TruePredicate.test(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/TruePredicate.java:9)
- [ ] `TruePredicate.codec(0)`: its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)) (dev/worldgen/lithostitched/impl/predicate/TruePredicate.java:14)

### `LithostitchedRegistrar`

- [ ] `LithostitchedRegistrar.registerRegistry(2)`: it's generic (dev/worldgen/lithostitched/impl/registry/LithostitchedRegistrar.java:11)
- [ ] `LithostitchedRegistrar.register(2)`: it's generic (dev/worldgen/lithostitched/impl/registry/LithostitchedRegistrar.java:15)

### `CodecExtender`

- [ ] `CodecExtender.extend(2)`: it's generic (dev/worldgen/lithostitched/impl/util/CodecExtender.java:14)
- [ ] `CodecExtender.extend(2)`: it's generic (dev/worldgen/lithostitched/impl/util/CodecExtender.java:22)

### `PositionalTimeBasedLayer`: it's an interface

- [ ] `PositionalTimeBasedLayer.applyPositionalTimeBased(4)`: it has no body (dev/worldgen/lithostitched/impl/worldgen/attribute/PositionalTimeBasedLayer.java:9)
- [ ] `PositionalTimeBasedLayer.applyPositional(3)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/attribute/PositionalTimeBasedLayer.java:11)

### `Bandlands`: it uses Minecraft's `BlockState`

- [ ] `Bandlands.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/bandlands/Bandlands.java:14)
- [ ] `Bandlands.Bandlands(2)`: its class isn't translated (it uses Minecraft's `BlockState`) (dev/worldgen/lithostitched/impl/worldgen/bandlands/Bandlands.java:22)
- [ ] `Bandlands.base(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (dev/worldgen/lithostitched/impl/worldgen/bandlands/Bandlands.java:27)
- [ ] `Bandlands.bands(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (dev/worldgen/lithostitched/impl/worldgen/bandlands/Bandlands.java:31)
- [ ] `Bandlands.getBand(4)`: its class isn't translated (it uses Minecraft's `BlockState`) (dev/worldgen/lithostitched/impl/worldgen/bandlands/Bandlands.java:35)
- [ ] `Bandlands.fillBands(1)`: its class isn't translated (it uses Minecraft's `BlockState`) (dev/worldgen/lithostitched/impl/worldgen/bandlands/Bandlands.java:44)

### `BandlandsBuilderImpl`: it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)

- [ ] `BandlandsBuilderImpl.BandlandsBuilderImpl(1)`: its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/BandlandsBuilderImpl.java:17)
- [ ] `BandlandsBuilderImpl.baseBand(3)`: its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/BandlandsBuilderImpl.java:22)
- [ ] `BandlandsBuilderImpl.repeatBand(3)`: its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/BandlandsBuilderImpl.java:28)
- [ ] `BandlandsBuilderImpl.wrappedBand(5)`: its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/BandlandsBuilderImpl.java:34)
- [ ] `BandlandsBuilderImpl.build(0)`: its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/BandlandsBuilderImpl.java:40)

### `BaseBand`: it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)

- [ ] `BaseBand.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java:12)
- [ ] `BaseBand.BaseBand(3)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java:11)
- [ ] `BaseBand.fill(2)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java:21)
- [ ] `BaseBand.codec(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java:35)
- [ ] `BaseBand.count(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java:11)
- [ ] `BaseBand.size(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java:11)
- [ ] `BaseBand.state(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java:11)

### `RepeatingBand`: it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)

- [ ] `RepeatingBand.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java:12)
- [ ] `RepeatingBand.RepeatingBand(3)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java:11)
- [ ] `RepeatingBand.fill(2)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java:21)
- [ ] `RepeatingBand.codec(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java:34)
- [ ] `RepeatingBand.interval(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java:11)
- [ ] `RepeatingBand.size(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java:11)
- [ ] `RepeatingBand.state(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java:11)

### `WrappedBand`: it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)

- [ ] `WrappedBand.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:13)
- [ ] `WrappedBand.WrappedBand(5)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:12)
- [ ] `WrappedBand.fill(2)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:24)
- [ ] `WrappedBand.codec(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:43)
- [ ] `WrappedBand.interval(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:12)
- [ ] `WrappedBand.maxCount(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:12)
- [ ] `WrappedBand.wrapperChance(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:12)
- [ ] `WrappedBand.wrapperState(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:12)
- [ ] `WrappedBand.wrappedState(0)`: its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)) (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java:12)

### `AddPoints`: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)

- [ ] `AddPoints.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:20)
- [ ] `AddPoints.AddPoints(4)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:18)
- [ ] `AddPoints.apply(2)`: it uses Minecraft's `Pair` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:30)
- [ ] `AddPoints.possibleBiomes(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:36)
- [ ] `AddPoints.codec(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:41)
- [ ] `AddPoints.predicate(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:18)
- [ ] `AddPoints.dimension(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:18)
- [ ] `AddPoints.priority(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:18)
- [ ] `AddPoints.points(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java:18)

### `DispatchAlternateLayout`: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)

- [ ] `DispatchAlternateLayout.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:23)
- [ ] `DispatchAlternateLayout.DispatchAlternateLayout(5)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:20)
- [ ] `DispatchAlternateLayout.matches(6)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:34)
- [ ] `DispatchAlternateLayout.possibleBiomes(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:38)
- [ ] `DispatchAlternateLayout.codec(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:43)
- [ ] `DispatchAlternateLayout.predicate(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:20)
- [ ] `DispatchAlternateLayout.dimension(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:20)
- [ ] `DispatchAlternateLayout.priority(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:20)
- [ ] `DispatchAlternateLayout.parameters(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:20)
- [ ] `DispatchAlternateLayout.points(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java:20)

### `ForcePlacement`: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)

- [ ] `ForcePlacement.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:20)
- [ ] `ForcePlacement.ForcePlacement(5)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:18)
- [ ] `ForcePlacement.matches(6)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:31)
- [ ] `ForcePlacement.possibleBiomes(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:35)
- [ ] `ForcePlacement.codec(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:40)
- [ ] `ForcePlacement.predicate(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:18)
- [ ] `ForcePlacement.dimension(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:18)
- [ ] `ForcePlacement.priority(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:18)
- [ ] `ForcePlacement.biome(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:18)
- [ ] `ForcePlacement.parameters(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java:18)

### `BiomeInjectorManager`

- [ ] `BiomeInjectorManager.applyBiomeInjectors(4)`: it uses Minecraft's `MinecraftServer` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/BiomeInjectorManager.java:38)
- [ ] `BiomeInjectorManager.createRegionId(1)`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/BiomeInjectorManager.java:104)

### `InjectorBiomeSource`: it extends Minecraft's `BiomeSource`

- [ ] `InjectorBiomeSource.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:52)
- [ ] `InjectorBiomeSource.InjectorBiomeSource(1)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:63)
- [ ] `InjectorBiomeSource.rootDelegate(0)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:68)
- [ ] `InjectorBiomeSource.directDelegate(0)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:72)
- [ ] `InjectorBiomeSource.applyInjectors(4)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:76)
- [ ] `InjectorBiomeSource.updateParameters(2)`: it uses Minecraft's `ParameterList` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:118)
- [ ] `InjectorBiomeSource.codec(0)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:146)
- [ ] `InjectorBiomeSource.collectPossibleBiomes(0)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:150)
- [ ] `InjectorBiomeSource.createResolver(1)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:154)
- [ ] `InjectorBiomeSource.getNoiseBiome(6)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:160)
- [ ] `InjectorBiomeSource.addDebugInfo(3)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:210)
- [ ] `InjectorBiomeSource.getRegionLine(3)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:214)
- [ ] `InjectorBiomeSource.getRootSource(1)`: it uses Minecraft's `BiomeSource` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:224)
- [ ] `InjectorBiomeSource.instanceOfBlueprintSource(1)`: it uses Minecraft's `BiomeSource` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:241)
- [ ] `InjectorBiomeSource.clone(0)`: its class isn't translated (it extends Minecraft's `BiomeSource`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java:245)

### `ParameterBuilderImpl`: it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)

- [ ] `ParameterBuilderImpl.densityFunctionExactly(2)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:20)
- [ ] `ParameterBuilderImpl.densityFunctionMin(2)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:26)
- [ ] `ParameterBuilderImpl.densityFunctionMax(2)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:32)
- [ ] `ParameterBuilderImpl.densityFunctionRange(3)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:38)
- [ ] `ParameterBuilderImpl.climateExactly(2)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:44)
- [ ] `ParameterBuilderImpl.climateMin(2)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:50)
- [ ] `ParameterBuilderImpl.climateMax(2)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:56)
- [ ] `ParameterBuilderImpl.climateRange(3)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:62)
- [ ] `ParameterBuilderImpl.region(1)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:68)
- [ ] `ParameterBuilderImpl.build(0)`: its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java:74)

### `ParameterMap`: it uses Minecraft's `Either`

- [ ] `ParameterMap.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterMap.java:21)
- [ ] `ParameterMap.ParameterMap(2)`: its class isn't translated (it uses Minecraft's `Either`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterMap.java:33)
- [ ] `ParameterMap.matches(6)`: its class isn't translated (it uses Minecraft's `Either`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterMap.java:38)

### `Region`: it uses Minecraft's `ResourceKey`

- [ ] `Region.KEY_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:17)
- [ ] `Region.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:18)
- [ ] `Region.Region(4)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:16)
- [ ] `Region.create(3)`: it uses Region, which isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:27)
- [ ] `Region.create(4)`: it uses Region, which isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:31)
- [ ] `Region.name(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:16)
- [ ] `Region.dimension(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:16)
- [ ] `Region.biomes(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:16)
- [ ] `Region.weight(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java:16)

### `RegionManager`: it uses Minecraft's `DensityFunction`

- [ ] `RegionManager.NO_REGIONS`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:19)
- [ ] `RegionManager.NO_REGIONS_IN_RANGE`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:20)
- [ ] `RegionManager.RegionManager(3)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:24)
- [ ] `RegionManager.getRegion(5)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:50)
- [ ] `RegionManager.getRegionValue(5)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:55)
- [ ] `RegionManager.error(1)`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:62)

### `RegionManager.BiomeRegions`: it uses Minecraft's `ResourceKey`

- [ ] `RegionManager.BiomeRegions.BiomeRegions(2)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:66)
- [ ] `RegionManager.BiomeRegions.getRegionValue(5)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:67)
- [ ] `RegionManager.BiomeRegions.getRegion(5)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:72)
- [ ] `RegionManager.BiomeRegions.regionsByOutputs(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:66)
- [ ] `RegionManager.BiomeRegions.totalWeight(0)`: its class isn't translated (it uses Minecraft's `ResourceKey`) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java:66)

### `ReplaceFully`: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)

- [ ] `ReplaceFully.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:18)
- [ ] `ReplaceFully.ReplaceFully(5)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:15)
- [ ] `ReplaceFully.possibleBiomes(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:29)
- [ ] `ReplaceFully.codec(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:34)
- [ ] `ReplaceFully.predicate(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:15)
- [ ] `ReplaceFully.dimension(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:15)
- [ ] `ReplaceFully.priority(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:15)
- [ ] `ReplaceFully.targets(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:15)
- [ ] `ReplaceFully.replacement(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java:15)

### `ReplacePartially`: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)

- [ ] `ReplacePartially.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:27)
- [ ] `ReplacePartially.ReplacePartially(6)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:19)
- [ ] `ReplacePartially.matches(7)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:39)
- [ ] `ReplacePartially.possibleBiomes(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:45)
- [ ] `ReplacePartially.codec(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:50)
- [ ] `ReplacePartially.predicate(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:19)
- [ ] `ReplacePartially.dimension(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:19)
- [ ] `ReplacePartially.priority(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:19)
- [ ] `ReplacePartially.targets(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:19)
- [ ] `ReplacePartially.replacement(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:19)
- [ ] `ReplacePartially.parameters(0)`: its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)) (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java:19)

### `ApplyAll`: it implements Minecraft's `RuleBlockEntityModifier`

- [ ] `ApplyAll.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyAll.java:14)
- [ ] `ApplyAll.TYPE`: it uses Minecraft's `RuleBlockEntityModifierType` (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyAll.java:17)
- [ ] `ApplyAll.ApplyAll(1)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyAll.java:13)
- [ ] `ApplyAll.apply(2)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyAll.java:19)
- [ ] `ApplyAll.getType(0)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyAll.java:28)
- [ ] `ApplyAll.modifiers(0)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyAll.java:13)

### `ApplyRandom`: it implements Minecraft's `RuleBlockEntityModifier`

- [ ] `ApplyRandom.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyRandom.java:15)
- [ ] `ApplyRandom.TYPE`: it uses Minecraft's `RuleBlockEntityModifierType` (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyRandom.java:19)
- [ ] `ApplyRandom.ApplyRandom(1)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyRandom.java:14)
- [ ] `ApplyRandom.apply(2)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyRandom.java:21)
- [ ] `ApplyRandom.getType(0)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyRandom.java:27)
- [ ] `ApplyRandom.modifiers(0)`: its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`) (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyRandom.java:14)

### `BlockStatePredicate`: it extends Minecraft's `StateTestingPredicate`

- [ ] `BlockStatePredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/BlockStatePredicate.java:12)
- [ ] `BlockStatePredicate.TYPE`: it uses Minecraft's `BlockPredicateType` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/BlockStatePredicate.java:17)
- [ ] `BlockStatePredicate.BlockStatePredicate(2)`: its class isn't translated (it extends Minecraft's `StateTestingPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/BlockStatePredicate.java:20)
- [ ] `BlockStatePredicate.properties(0)`: its class isn't translated (it extends Minecraft's `StateTestingPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/BlockStatePredicate.java:25)
- [ ] `BlockStatePredicate.test(1)`: its class isn't translated (it extends Minecraft's `StateTestingPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/BlockStatePredicate.java:29)
- [ ] `BlockStatePredicate.type(0)`: its class isn't translated (it extends Minecraft's `StateTestingPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/BlockStatePredicate.java:33)

### `GridPredicate`: it implements Minecraft's `BlockPredicate`

- [ ] `GridPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:15)
- [ ] `GridPredicate.TYPE`: it uses Minecraft's `BlockPredicateType` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:24)
- [ ] `GridPredicate.GridPredicate(4)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:14)
- [ ] `GridPredicate.test(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:26)
- [ ] `GridPredicate.type(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:43)
- [ ] `GridPredicate.radius(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:14)
- [ ] `GridPredicate.distBetweenPoints(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:14)
- [ ] `GridPredicate.predicate(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:14)
- [ ] `GridPredicate.allowedCount(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java:14)

### `HolderHolderPredicate`: it implements Minecraft's `BlockPredicate`

- [ ] `HolderHolderPredicate.HolderHolderPredicate(1)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/HolderHolderPredicate.java:9)
- [ ] `HolderHolderPredicate.type(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/HolderHolderPredicate.java:10)
- [ ] `HolderHolderPredicate.test(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/HolderHolderPredicate.java:14)
- [ ] `HolderHolderPredicate.predicate(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/HolderHolderPredicate.java:9)

### `InStructurePredicate`: it implements Minecraft's `BlockPredicate`

- [ ] `InStructurePredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:31)
- [ ] `InStructurePredicate.TYPE`: it uses Minecraft's `BlockPredicateType` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:38)
- [ ] `InStructurePredicate.InStructurePredicate(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:30)
- [ ] `InStructurePredicate.type(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:40)
- [ ] `InStructurePredicate.test(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:44)
- [ ] `InStructurePredicate.structure(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:30)
- [ ] `InStructurePredicate.searchRange(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:30)

### `InStructurePredicate.SearchRange`

- [ ] `InStructurePredicate.SearchRange.BASE_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:83)
- [ ] `InStructurePredicate.SearchRange.FULL_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:84)
- [ ] `InStructurePredicate.SearchRange.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:93)
- [x] `InStructurePredicate.SearchRange.SearchRange(2)` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:82)
- [x] `InStructurePredicate.SearchRange.SearchRange(1)` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:99)
- [ ] `InStructurePredicate.SearchRange.box(0)`: it uses Minecraft's `BoundingBox` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:103)
- [x] `InStructurePredicate.SearchRange.horizontal(0)` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:82)
- [x] `InStructurePredicate.SearchRange.vertical(0)` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java:82)

### `MultipleOfPredicate`: it implements Minecraft's `BlockPredicate`

- [ ] `MultipleOfPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java:14)
- [ ] `MultipleOfPredicate.TYPE`: it uses Minecraft's `BlockPredicateType` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java:21)
- [ ] `MultipleOfPredicate.MultipleOfPredicate(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java:13)
- [ ] `MultipleOfPredicate.test(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java:23)
- [ ] `MultipleOfPredicate.type(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java:37)
- [ ] `MultipleOfPredicate.predicates(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java:13)
- [ ] `MultipleOfPredicate.allowedCount(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java:13)

### `OffsetPredicate`: it implements Minecraft's `BlockPredicate`

- [ ] `OffsetPredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java:12)
- [ ] `OffsetPredicate.TYPE`: it uses Minecraft's `BlockPredicateType` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java:19)
- [ ] `OffsetPredicate.OffsetPredicate(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java:11)
- [ ] `OffsetPredicate.test(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java:21)
- [ ] `OffsetPredicate.type(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java:25)
- [ ] `OffsetPredicate.predicate(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java:11)
- [ ] `OffsetPredicate.offset(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java:11)

### `RandomChancePredicate`: it implements Minecraft's `BlockPredicate`

- [ ] `RandomChancePredicate.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/RandomChancePredicate.java:12)
- [ ] `RandomChancePredicate.TYPE`: it uses Minecraft's `BlockPredicateType` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/RandomChancePredicate.java:15)
- [ ] `RandomChancePredicate.RandomChancePredicate(1)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/RandomChancePredicate.java:11)
- [ ] `RandomChancePredicate.test(2)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/RandomChancePredicate.java:17)
- [ ] `RandomChancePredicate.type(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/RandomChancePredicate.java:22)
- [ ] `RandomChancePredicate.chance(0)`: its class isn't translated (it implements Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/RandomChancePredicate.java:11)

### `CellularDensityFunction`: it implements Minecraft's `DensityFunction`

- [ ] `CellularDensityFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:33)
- [ ] `CellularDensityFunction.CellularDensityFunction(5)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:26)
- [ ] `CellularDensityFunction.isCellPositionInvalid(3)`: it uses Sampler, which isn't translated (it uses Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:44)
- [ ] `CellularDensityFunction.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:48)
- [ ] `CellularDensityFunction.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:58)
- [ ] `CellularDensityFunction.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:63)
- [ ] `CellularDensityFunction.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:67)
- [ ] `CellularDensityFunction.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:71)
- [ ] `CellularDensityFunction.condition(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:26)
- [ ] `CellularDensityFunction.gridDimensions(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:26)
- [ ] `CellularDensityFunction.jitter(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:26)
- [ ] `CellularDensityFunction.returnType(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:26)
- [ ] `CellularDensityFunction.salt(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:26)

### `CellularDensityFunction.CellCondition`: it uses Minecraft's `DensityFunction`

- [ ] `CellularDensityFunction.CellCondition.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:76)
- [ ] `CellularDensityFunction.CellCondition.CellCondition(3)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:75)
- [ ] `CellularDensityFunction.CellCondition.rewriteChildren(1)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:85)
- [ ] `CellularDensityFunction.CellCondition.compileSampler(1)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:90)
- [ ] `CellularDensityFunction.CellCondition.function(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:75)
- [ ] `CellularDensityFunction.CellCondition.range(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:75)
- [ ] `CellularDensityFunction.CellCondition.fallbackValue(0)`: its class isn't translated (it uses Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:75)

### `CellularDensityFunction.CellCondition.Sampler`: it uses Minecraft's `DensitySampler`

- [ ] `CellularDensityFunction.CellCondition.Sampler.Sampler(3)`: its class isn't translated (it uses Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:94)
- [ ] `CellularDensityFunction.CellCondition.Sampler.function(0)`: its class isn't translated (it uses Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:94)
- [ ] `CellularDensityFunction.CellCondition.Sampler.range(0)`: its class isn't translated (it uses Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:94)
- [ ] `CellularDensityFunction.CellCondition.Sampler.fallbackValue(0)`: its class isn't translated (it uses Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:94)

### `CellularDensityFunction.GridDimensions`

- [ ] `CellularDensityFunction.GridDimensions.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:99)
- [x] `CellularDensityFunction.GridDimensions.GridDimensions(2)` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:98)
- [x] `CellularDensityFunction.GridDimensions.is2d(0)` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:111)
- [x] `CellularDensityFunction.GridDimensions.horizontal(0)` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:98)
- [x] `CellularDensityFunction.GridDimensions.vertical(0)` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:98)

### `CellularDensityFunction.XYZSampler`: it implements Minecraft's `DensitySampler`

- [ ] `CellularDensityFunction.XYZSampler.XYZSampler(5)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:116)
- [ ] `CellularDensityFunction.XYZSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:123)
- [ ] `CellularDensityFunction.XYZSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:140)
- [ ] `CellularDensityFunction.XYZSampler.condition(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:116)
- [ ] `CellularDensityFunction.XYZSampler.gridSize(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:116)
- [ ] `CellularDensityFunction.XYZSampler.jitter(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:116)
- [ ] `CellularDensityFunction.XYZSampler.returnType(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:116)
- [ ] `CellularDensityFunction.XYZSampler.salt(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:116)

### `CellularDensityFunction.XZSampler`: it implements Minecraft's `DensitySampler`

- [ ] `CellularDensityFunction.XZSampler.XZSampler(5)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:210)
- [ ] `CellularDensityFunction.XZSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:212)
- [ ] `CellularDensityFunction.XZSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:229)
- [ ] `CellularDensityFunction.XZSampler.toGridPos(1)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:285)
- [ ] `CellularDensityFunction.XZSampler.condition(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:210)
- [ ] `CellularDensityFunction.XZSampler.gridWidth(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:210)
- [ ] `CellularDensityFunction.XZSampler.jitter(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:210)
- [ ] `CellularDensityFunction.XZSampler.returnType(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:210)
- [ ] `CellularDensityFunction.XZSampler.salt(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java:210)

### `CosFunction`: it implements Minecraft's `DensityFunction`

- [ ] `CosFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:15)
- [ ] `CosFunction.CosFunction(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:14)
- [ ] `CosFunction.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:17)
- [ ] `CosFunction.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:21)
- [ ] `CosFunction.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:26)
- [ ] `CosFunction.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:30)
- [ ] `CosFunction.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:34)
- [ ] `CosFunction.input(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:14)

### `CosFunction.CosSampler`: it implements Minecraft's `DensitySampler`

- [ ] `CosFunction.CosSampler.CosSampler(1)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:38)
- [ ] `CosFunction.CosSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:39)
- [ ] `CosFunction.CosSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:47)
- [ ] `CosFunction.CosSampler.input(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java:38)

### `FastNoiseFunction`: it implements Minecraft's `DensityFunction`

- [ ] `FastNoiseFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:26)
- [ ] `FastNoiseFunction.FastNoiseFunction(6)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:23)
- [ ] `FastNoiseFunction.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:38)
- [ ] `FastNoiseFunction.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:54)
- [ ] `FastNoiseFunction.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:63)
- [ ] `FastNoiseFunction.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:67)
- [ ] `FastNoiseFunction.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:80)
- [ ] `FastNoiseFunction.config(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:23)
- [ ] `FastNoiseFunction.xzScale(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:23)
- [ ] `FastNoiseFunction.yScale(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:23)
- [ ] `FastNoiseFunction.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:23)
- [ ] `FastNoiseFunction.shiftY(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:23)
- [ ] `FastNoiseFunction.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:23)

### `FastNoiseFunction.Sampler`: it implements Minecraft's `DensitySampler`

- [ ] `FastNoiseFunction.Sampler.Sampler(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:84)
- [ ] `FastNoiseFunction.Sampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:85)
- [ ] `FastNoiseFunction.Sampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:90)
- [ ] `FastNoiseFunction.Sampler.noise(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:84)
- [ ] `FastNoiseFunction.Sampler.xzScale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:84)
- [ ] `FastNoiseFunction.Sampler.yScale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:84)

### `FastNoiseFunction.ShiftedXyzSampler`: it implements Minecraft's `DensitySampler`

- [ ] `FastNoiseFunction.ShiftedXyzSampler.ShiftedXyzSampler(6)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:95)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:97)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:156)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:95)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.shiftY(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:95)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:95)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.noise(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:95)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.xzScale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:95)
- [ ] `FastNoiseFunction.ShiftedXyzSampler.yScale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:95)

### `FastNoiseFunction.ShiftedXzSampler`: it implements Minecraft's `DensitySampler`

- [ ] `FastNoiseFunction.ShiftedXzSampler.ShiftedXzSampler(5)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:164)
- [ ] `FastNoiseFunction.ShiftedXzSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:165)
- [ ] `FastNoiseFunction.ShiftedXzSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:205)
- [ ] `FastNoiseFunction.ShiftedXzSampler.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:164)
- [ ] `FastNoiseFunction.ShiftedXzSampler.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:164)
- [ ] `FastNoiseFunction.ShiftedXzSampler.noise(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:164)
- [ ] `FastNoiseFunction.ShiftedXzSampler.xzScale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:164)
- [ ] `FastNoiseFunction.ShiftedXzSampler.yScale(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java:164)

### `MarkerFunction`: it's an interface

- [ ] `MarkerFunction.compileSampler(1)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MarkerFunction.java:11)
- [ ] `MarkerFunction.rewriteChildren(1)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MarkerFunction.java:15)
- [ ] `MarkerFunction.range(0)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MarkerFunction.java:19)
- [ ] `MarkerFunction.domainAxes(0)`: its class isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MarkerFunction.java:23)

### `MergedFunction`: it implements Minecraft's `DensityFunction`

- [ ] `MergedFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:14)
- [ ] `MergedFunction.MergedFunction(3)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:13)
- [ ] `MergedFunction.unwrappedOriginal(1)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:35)
- [ ] `MergedFunction.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:39)
- [ ] `MergedFunction.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:43)
- [ ] `MergedFunction.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:48)
- [ ] `MergedFunction.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:52)
- [ ] `MergedFunction.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:56)
- [ ] `MergedFunction.original(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:13)
- [ ] `MergedFunction.wrapped(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:13)
- [ ] `MergedFunction.full(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java:13)

### `OriginalMarkerFunction`: it implements `MarkerFunction`

- [ ] `OriginalMarkerFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/OriginalMarkerFunction.java:8)
- [ ] `OriginalMarkerFunction.codec(0)`: its class isn't translated (it implements `MarkerFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/OriginalMarkerFunction.java:10)

### `WrappedMarkerFunction`: it implements `MarkerFunction`

- [ ] `WrappedMarkerFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/WrappedMarkerFunction.java:8)
- [ ] `WrappedMarkerFunction.codec(0)`: its class isn't translated (it implements `MarkerFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/WrappedMarkerFunction.java:10)

### `ShiftFunction`: it implements Minecraft's `DensityFunction`

- [ ] `ShiftFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:17)
- [ ] `ShiftFunction.ShiftFunction(4)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:16)
- [ ] `ShiftFunction.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:27)
- [ ] `ShiftFunction.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:33)
- [ ] `ShiftFunction.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:43)
- [ ] `ShiftFunction.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:47)
- [ ] `ShiftFunction.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:51)
- [ ] `ShiftFunction.input(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:16)
- [ ] `ShiftFunction.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:16)
- [ ] `ShiftFunction.shiftY(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:16)
- [ ] `ShiftFunction.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:16)

### `ShiftFunction.Sampler`: it implements Minecraft's `DensitySampler`

- [ ] `ShiftFunction.Sampler.Sampler(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:55)
- [ ] `ShiftFunction.Sampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:56)
- [ ] `ShiftFunction.Sampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:115)
- [ ] `ShiftFunction.Sampler.input(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:55)
- [ ] `ShiftFunction.Sampler.shiftX(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:55)
- [ ] `ShiftFunction.Sampler.shiftY(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:55)
- [ ] `ShiftFunction.Sampler.shiftZ(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java:55)

### `SinFunction`: it implements Minecraft's `DensityFunction`

- [ ] `SinFunction.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:15)
- [ ] `SinFunction.SinFunction(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:14)
- [ ] `SinFunction.compileSampler(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:17)
- [ ] `SinFunction.rewriteChildren(1)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:21)
- [ ] `SinFunction.range(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:26)
- [ ] `SinFunction.domainAxes(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:30)
- [ ] `SinFunction.codec(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:34)
- [ ] `SinFunction.input(0)`: its class isn't translated (it implements Minecraft's `DensityFunction`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:14)

### `SinFunction.SinSampler`: it implements Minecraft's `DensitySampler`

- [ ] `SinFunction.SinSampler.SinSampler(1)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:38)
- [ ] `SinFunction.SinSampler.sampleVolume(3)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:39)
- [ ] `SinFunction.SinSampler.sampleValue(4)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:47)
- [ ] `SinFunction.SinSampler.input(0)`: its class isn't translated (it implements Minecraft's `DensitySampler`) (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java:38)

### `CellularNoiseType`: it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)

- [ ] `CellularNoiseType.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:11)
- [ ] `CellularNoiseType.CellularNoiseType(5)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:27)
- [ ] `CellularNoiseType.distanceFunction(0)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:40)
- [ ] `CellularNoiseType.returnType(0)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:44)
- [ ] `CellularNoiseType.jitter(0)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:48)
- [ ] `CellularNoiseType.getCodec(0)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:52)

### `CellularNoiseType.DistanceFunction`: its constants carry values

- [ ] `CellularNoiseType.DistanceFunction.DistanceFunction(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:66)
- [ ] `CellularNoiseType.DistanceFunction.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:71)

### `CellularNoiseType.ReturnType`: its constants carry values

- [ ] `CellularNoiseType.ReturnType.ReturnType(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:88)
- [ ] `CellularNoiseType.ReturnType.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java:93)

### `PerlinNoiseType`: it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)

- [ ] `PerlinNoiseType.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/PerlinNoiseType.java:10)
- [ ] `PerlinNoiseType.PerlinNoiseType(2)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/PerlinNoiseType.java:17)
- [ ] `PerlinNoiseType.getCodec(0)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/PerlinNoiseType.java:22)

### `SimplexNoiseType`: it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)

- [ ] `SimplexNoiseType.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/SimplexNoiseType.java:13)
- [ ] `SimplexNoiseType.SimplexNoiseType(6)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/SimplexNoiseType.java:29)
- [ ] `SimplexNoiseType.getCodec(0)`: its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)) (dev/worldgen/lithostitched/impl/worldgen/fastnoise/SimplexNoiseType.java:42)

### `CompositeFeature`: it implements Minecraft's `Feature`

- [ ] `CompositeFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:19)
- [ ] `CompositeFeature.CompositeFeature(2)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:18)
- [ ] `CompositeFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:27)
- [ ] `CompositeFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:44)
- [ ] `CompositeFeature.features(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:18)
- [ ] `CompositeFeature.placementType(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:18)

### `CompositeFeature.Type`: its constants carry values

- [ ] `CompositeFeature.Type.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:53)
- [ ] `CompositeFeature.Type.Type(2)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:57)
- [ ] `CompositeFeature.Type.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:62)
- [ ] `CompositeFeature.Type.shouldContinue(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java:67)

### `DungeonFeature`: it implements Minecraft's `Feature`

- [ ] `DungeonFeature.DEFAULT_MOBS`: it uses Minecraft's `WeightedList` (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:52)
- [ ] `DungeonFeature.DEFAULT_FLOOR`: it uses Minecraft's `WeightedList` (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:57)
- [ ] `DungeonFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:61)
- [ ] `DungeonFeature.DungeonFeature(9)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:85)
- [ ] `DungeonFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:182)
- [ ] `DungeonFeature.minOpenings(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.maxOpenings(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.radius(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.maxChests(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.spawnerMobs(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.floorProvider(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.wallProvider(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.dungeonInvalidBlocks(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)
- [ ] `DungeonFeature.lootTable(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java:41)

### `LargeDripstoneFeature`: it implements Minecraft's `Feature`

- [ ] `LargeDripstoneFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:45)
- [ ] `LargeDripstoneFeature.LargeDripstoneFeature(11)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:62)
- [ ] `LargeDripstoneFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:66)
- [ ] `LargeDripstoneFeature.makeDripstone(7)`: it uses LargeDripstone, which isn't translated (it uses Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:123)
- [ ] `LargeDripstoneFeature.stateProvider(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.replaceableBlocks(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.floorToCeilingSearchRange(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.columnRadius(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.heightScale(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.maxColumnRadiusToCaveHeightRatio(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.stalactiteBluntness(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.stalagmiteBluntness(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.windSpeed(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.minRadiusForWind(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)
- [ ] `LargeDripstoneFeature.minBluntnessForWind(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:32)

### `LargeDripstoneFeature.LargeDripstone`: it uses Minecraft's `BlockStateProvider`

- [ ] `LargeDripstoneFeature.LargeDripstone.LargeDripstone(6)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:137)
- [ ] `LargeDripstoneFeature.LargeDripstone.getHeight(0)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:146)
- [ ] `LargeDripstoneFeature.LargeDripstone.moveBackUntilBaseIsInsideStoneAndShrinkRadiusIfNecessary(2)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:150)
- [ ] `LargeDripstoneFeature.LargeDripstone.getHeightAtRadius(1)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:174)
- [ ] `LargeDripstoneFeature.LargeDripstone.placeBlocks(3)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:178)
- [ ] `LargeDripstoneFeature.LargeDripstone.isSuitableForWind(1)`: its class isn't translated (it uses Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:210)

### `LargeDripstoneFeature.WindOffsetter`: it uses Minecraft's `Vec3`

- [ ] `LargeDripstoneFeature.WindOffsetter.WindOffsetter(3)`: its class isn't translated (it uses Minecraft's `Vec3`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:219)
- [ ] `LargeDripstoneFeature.WindOffsetter.WindOffsetter(0)`: its class isn't translated (it uses Minecraft's `Vec3`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:226)
- [ ] `LargeDripstoneFeature.WindOffsetter.noWind(0)`: it uses WindOffsetter, which isn't translated (it uses Minecraft's `Vec3`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:231)
- [ ] `LargeDripstoneFeature.WindOffsetter.offset(1)`: its class isn't translated (it uses Minecraft's `Vec3`) (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java:235)

### `OreFeature`: it implements Minecraft's `Feature`

- [ ] `OreFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:25)
- [ ] `OreFeature.OreFeature(2)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:24)
- [ ] `OreFeature.create(2)`: it uses OreFeature, which isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:33)
- [ ] `OreFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:39)
- [ ] `OreFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:43)
- [ ] `OreFeature.doPlace(13)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:70)
- [ ] `OreFeature.size(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:24)
- [ ] `OreFeature.targets(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:24)

### `OreFeature.Target`: it uses Minecraft's `BlockPredicate`

- [ ] `OreFeature.Target.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:199)
- [ ] `OreFeature.Target.Target(2)`: its class isn't translated (it uses Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:198)
- [ ] `OreFeature.Target.predicate(0)`: its class isn't translated (it uses Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:198)
- [ ] `OreFeature.Target.stateProvider(0)`: its class isn't translated (it uses Minecraft's `BlockPredicate`) (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java:198)

### `SelectFeature`: it implements Minecraft's `Feature`

- [ ] `SelectFeature.PAIR_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/feature/SelectFeature.java:17)
- [ ] `SelectFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/SelectFeature.java:20)
- [ ] `SelectFeature.SelectFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SelectFeature.java:16)
- [ ] `SelectFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SelectFeature.java:22)
- [ ] `SelectFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SelectFeature.java:33)
- [ ] `SelectFeature.features(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SelectFeature.java:16)

### `SimplePlacedFeature`: it implements Minecraft's `Feature`

- [ ] `SimplePlacedFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/SimplePlacedFeature.java:13)
- [ ] `SimplePlacedFeature.SimplePlacedFeature(1)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SimplePlacedFeature.java:12)
- [ ] `SimplePlacedFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SimplePlacedFeature.java:17)
- [ ] `SimplePlacedFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SimplePlacedFeature.java:21)
- [ ] `SimplePlacedFeature.feature(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/SimplePlacedFeature.java:12)

### `StructureTemplateFeature`: it implements Minecraft's `Feature`

- [ ] `StructureTemplateFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:34)
- [ ] `StructureTemplateFeature.StructureTemplateFeature(5)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:27)
- [ ] `StructureTemplateFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:45)
- [ ] `StructureTemplateFeature.getRotatedOffset(3)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:62)
- [ ] `StructureTemplateFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:66)
- [ ] `StructureTemplateFeature.template(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:27)
- [ ] `StructureTemplateFeature.processors(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:27)
- [ ] `StructureTemplateFeature.rotation(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:27)
- [ ] `StructureTemplateFeature.liquidSettings(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:27)
- [ ] `StructureTemplateFeature.placeOnCenter(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java:27)

### `DripstoneUtils`

- [x] `DripstoneUtils.getDripstoneHeight(4)` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:13)
- [ ] `DripstoneUtils.isCircleMostlyEmbeddedInStone(3)`: it uses Minecraft's `WorldGenLevel` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:27)
- [ ] `DripstoneUtils.isEmptyOrWater(2)`: it uses Minecraft's `LevelAccessor` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:45)
- [ ] `DripstoneUtils.isEmptyOrWaterOrLava(2)`: it uses Minecraft's `LevelAccessor` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:49)
- [ ] `DripstoneUtils.isReplaceableOrLava(2)`: it uses Minecraft's `BlockState` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:53)
- [ ] `DripstoneUtils.isReplaceable(2)`: it uses Minecraft's `BlockState` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:57)
- [ ] `DripstoneUtils.isEmptyOrWater(1)`: it uses Minecraft's `BlockState` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:61)
- [ ] `DripstoneUtils.isEmptyOrWaterOrLava(1)`: it uses Minecraft's `BlockState` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java:65)

### `VinesFeature`: it implements Minecraft's `Feature`

- [ ] `VinesFeature.DEFAULT_BLOCK`: it uses Minecraft's `WeightedList` (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:26)
- [ ] `VinesFeature.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:27)
- [ ] `VinesFeature.VinesFeature(3)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:25)
- [ ] `VinesFeature.canPlaceOn(1)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:39)
- [ ] `VinesFeature.place(4)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:43)
- [ ] `VinesFeature.codec(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:88)
- [ ] `VinesFeature.blocks(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:25)
- [ ] `VinesFeature.canPlaceOn(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:25)
- [ ] `VinesFeature.maxLength(0)`: its class isn't translated (it implements Minecraft's `Feature`) (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java:25)

### `AllOfCondition`: it implements Minecraft's `MaterialCondition`

- [ ] `AllOfCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/material/condition/AllOfCondition.java:11)
- [ ] `AllOfCondition.AllOfCondition(1)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AllOfCondition.java:10)
- [ ] `AllOfCondition.codec(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AllOfCondition.java:16)
- [ ] `AllOfCondition.compile(1)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AllOfCondition.java:20)
- [ ] `AllOfCondition.conditions(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AllOfCondition.java:10)

### `AnyOfCondition`: it implements Minecraft's `MaterialCondition`

- [ ] `AnyOfCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/material/condition/AnyOfCondition.java:11)
- [ ] `AnyOfCondition.AnyOfCondition(1)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AnyOfCondition.java:10)
- [ ] `AnyOfCondition.codec(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AnyOfCondition.java:16)
- [ ] `AnyOfCondition.compile(1)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AnyOfCondition.java:20)
- [ ] `AnyOfCondition.conditions(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/AnyOfCondition.java:10)

### `SampleDensityCondition`: it implements Minecraft's `MaterialCondition`

- [ ] `SampleDensityCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java:15)
- [ ] `SampleDensityCondition.SampleDensityCondition(3)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java:14)
- [ ] `SampleDensityCondition.compile(1)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java:24)
- [ ] `SampleDensityCondition.codec(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java:29)
- [ ] `SampleDensityCondition.function(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java:14)
- [ ] `SampleDensityCondition.range(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java:14)
- [ ] `SampleDensityCondition.prefill(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java:14)

### `SlopeCondition`: it implements Minecraft's `MaterialCondition`

- [ ] `SlopeCondition.BASE_DIFFERENCE`: it uses Minecraft's `InclusiveRange` (dev/worldgen/lithostitched/impl/worldgen/material/condition/SlopeCondition.java:13)
- [ ] `SlopeCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/material/condition/SlopeCondition.java:14)
- [ ] `SlopeCondition.SlopeCondition(1)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SlopeCondition.java:12)
- [ ] `SlopeCondition.compile(1)`: its body couldn't be read: an anonymous class (dev/worldgen/lithostitched/impl/worldgen/material/condition/SlopeCondition.java:19)
- [ ] `SlopeCondition.codec(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SlopeCondition.java:28)
- [ ] `SlopeCondition.threshold(0)`: its class isn't translated (it implements Minecraft's `MaterialCondition`) (dev/worldgen/lithostitched/impl/worldgen/material/condition/SlopeCondition.java:12)

### `BandlandsRule`: it implements Minecraft's `MaterialRule`

- [ ] `BandlandsRule.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/material/rule/BandlandsRule.java:15)
- [ ] `BandlandsRule.BandlandsRule(1)`: its class isn't translated (it implements Minecraft's `MaterialRule`) (dev/worldgen/lithostitched/impl/worldgen/material/rule/BandlandsRule.java:14)
- [ ] `BandlandsRule.compile(1)`: its class isn't translated (it implements Minecraft's `MaterialRule`) (dev/worldgen/lithostitched/impl/worldgen/material/rule/BandlandsRule.java:19)
- [ ] `BandlandsRule.codec(0)`: its class isn't translated (it implements Minecraft's `MaterialRule`) (dev/worldgen/lithostitched/impl/worldgen/material/rule/BandlandsRule.java:24)
- [ ] `BandlandsRule.options(0)`: its class isn't translated (it implements Minecraft's `MaterialRule`) (dev/worldgen/lithostitched/impl/worldgen/material/rule/BandlandsRule.java:14)

### `AddBiomeSpawnsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `AddBiomeSpawnsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:31)
- [ ] `AddBiomeSpawnsModifier.AddBiomeSpawnsModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:29)
- [ ] `AddBiomeSpawnsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:41)
- [ ] `AddBiomeSpawnsModifier.applyModifier(2)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:48)
- [ ] `AddBiomeSpawnsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:83)
- [ ] `AddBiomeSpawnsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:29)
- [ ] `AddBiomeSpawnsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:29)
- [ ] `AddBiomeSpawnsModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:29)
- [ ] `AddBiomeSpawnsModifier.biomeSpawns(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java:29)

### `AddFeaturesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `AddFeaturesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:24)
- [ ] `AddFeaturesModifier.AddFeaturesModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:35)
- [ ] `AddFeaturesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:39)
- [ ] `AddFeaturesModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:46)
- [ ] `AddFeaturesModifier.shouldRecompileSortedFeatures(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:65)
- [ ] `AddFeaturesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:70)
- [ ] `AddFeaturesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:21)
- [ ] `AddFeaturesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:21)
- [ ] `AddFeaturesModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:21)
- [ ] `AddFeaturesModifier.features(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:21)
- [ ] `AddFeaturesModifier.step(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java:21)

### `AddProcessorListProcessorsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `AddProcessorListProcessorsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:22)
- [ ] `AddProcessorListProcessorsModifier.AddProcessorListProcessorsModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:19)
- [ ] `AddProcessorListProcessorsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:32)
- [ ] `AddProcessorListProcessorsModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:39)
- [ ] `AddProcessorListProcessorsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:46)
- [ ] `AddProcessorListProcessorsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:19)
- [ ] `AddProcessorListProcessorsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:19)
- [ ] `AddProcessorListProcessorsModifier.processorLists(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:19)
- [ ] `AddProcessorListProcessorsModifier.processors(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java:19)

### `AddSpawnCostsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `AddSpawnCostsModifier.SPAWN_COST_CODEC`: it uses Minecraft's `SimpleMapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:33)
- [ ] `AddSpawnCostsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:36)
- [ ] `AddSpawnCostsModifier.AddSpawnCostsModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:31)
- [ ] `AddSpawnCostsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:46)
- [ ] `AddSpawnCostsModifier.applyModifier(2)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:53)
- [ ] `AddSpawnCostsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:77)
- [ ] `AddSpawnCostsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:31)
- [ ] `AddSpawnCostsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:31)
- [ ] `AddSpawnCostsModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:31)
- [ ] `AddSpawnCostsModifier.spawnCosts(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java:31)

### `AddStructureSetEntriesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `AddStructureSetEntriesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:22)
- [ ] `AddStructureSetEntriesModifier.AddStructureSetEntriesModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:19)
- [ ] `AddStructureSetEntriesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:32)
- [ ] `AddStructureSetEntriesModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:39)
- [ ] `AddStructureSetEntriesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:46)
- [ ] `AddStructureSetEntriesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:19)
- [ ] `AddStructureSetEntriesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:19)
- [ ] `AddStructureSetEntriesModifier.structureSets(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:19)
- [ ] `AddStructureSetEntriesModifier.entries(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java:19)

### `AddStructureTemplatesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `AddStructureTemplatesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:19)
- [ ] `AddStructureTemplatesModifier.AddStructureTemplatesModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:17)
- [ ] `AddStructureTemplatesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:29)
- [ ] `AddStructureTemplatesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:36)
- [ ] `AddStructureTemplatesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:17)
- [ ] `AddStructureTemplatesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:17)
- [ ] `AddStructureTemplatesModifier.targets(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:17)
- [ ] `AddStructureTemplatesModifier.templates(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java:17)

### `AddTemplatePoolElementsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `AddTemplatePoolElementsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:25)
- [ ] `AddTemplatePoolElementsModifier.AddTemplatePoolElementsModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:22)
- [ ] `AddTemplatePoolElementsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:39)
- [ ] `AddTemplatePoolElementsModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:46)
- [ ] `AddTemplatePoolElementsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:62)
- [ ] `AddTemplatePoolElementsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:22)
- [ ] `AddTemplatePoolElementsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:22)
- [ ] `AddTemplatePoolElementsModifier.templatePools(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:22)
- [ ] `AddTemplatePoolElementsModifier.elements(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java:22)

### `BiomeEffectsBuilderImpl`: it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `BiomeEffectsBuilderImpl.waterColor(1)`: its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/BiomeEffectsBuilderImpl.java:15)
- [ ] `BiomeEffectsBuilderImpl.foliageColor(1)`: its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/BiomeEffectsBuilderImpl.java:20)
- [ ] `BiomeEffectsBuilderImpl.dryFoliageColor(1)`: its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/BiomeEffectsBuilderImpl.java:25)
- [ ] `BiomeEffectsBuilderImpl.grassColor(1)`: its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/BiomeEffectsBuilderImpl.java:30)
- [ ] `BiomeEffectsBuilderImpl.grassColorModifier(1)`: its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/BiomeEffectsBuilderImpl.java:35)
- [ ] `BiomeEffectsBuilderImpl.build(0)`: its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/BiomeEffectsBuilderImpl.java:40)

### `CompileRawTemplatesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `CompileRawTemplatesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/CompileRawTemplatesModifier.java:14)
- [ ] `CompileRawTemplatesModifier.CompileRawTemplatesModifier(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/CompileRawTemplatesModifier.java:13)
- [ ] `CompileRawTemplatesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/CompileRawTemplatesModifier.java:16)
- [ ] `CompileRawTemplatesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/CompileRawTemplatesModifier.java:21)
- [ ] `CompileRawTemplatesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/CompileRawTemplatesModifier.java:28)
- [ ] `CompileRawTemplatesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/CompileRawTemplatesModifier.java:33)

### `DensityFunctionInjectorHelper`

- [ ] `DensityFunctionInjectorHelper.wrap(2)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:12)
- [ ] `DensityFunctionInjectorHelper.isMarker(1)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:21)

### `DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule`: it implements Minecraft's `DfRewriteRule`

- [ ] `DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.ApplyWrappedAndOriginalRule(2)`: its class isn't translated (it implements Minecraft's `DfRewriteRule`) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:39)
- [ ] `DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.rewrite(1)`: its class isn't translated (it implements Minecraft's `DfRewriteRule`) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:40)
- [ ] `DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.original(0)`: its class isn't translated (it implements Minecraft's `DfRewriteRule`) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:39)
- [ ] `DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.wrapped(0)`: its class isn't translated (it implements Minecraft's `DfRewriteRule`) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:39)

### `DensityFunctionInjectorHelper.ApplyWrappedRule`: it implements Minecraft's `DfRewriteRule`

- [ ] `DensityFunctionInjectorHelper.ApplyWrappedRule.ApplyWrappedRule(1)`: its class isn't translated (it implements Minecraft's `DfRewriteRule`) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:49)
- [ ] `DensityFunctionInjectorHelper.ApplyWrappedRule.rewrite(1)`: its class isn't translated (it implements Minecraft's `DfRewriteRule`) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:50)
- [ ] `DensityFunctionInjectorHelper.ApplyWrappedRule.wrapped(0)`: its class isn't translated (it implements Minecraft's `DfRewriteRule`) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java:49)

### `ModifierManager`

- [ ] `ModifierManager.applyModifiers(2)`: it uses Minecraft's `RegistryAccess` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/ModifierManager.java:23)
- [ ] `ModifierManager.sortByPriority(1)`: it uses WorldgenModifier, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/ModifierManager.java:48)
- [ ] `ModifierManager.getAllModifiers(1)`: it uses WorldgenModifier, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/ModifierManager.java:52)
- [ ] `ModifierManager.getModifiersOfType(2)`: it's generic (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/ModifierManager.java:65)

### `NoOpModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `NoOpModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/NoOpModifier.java:10)
- [ ] `NoOpModifier.NoOpModifier(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/NoOpModifier.java:9)
- [ ] `NoOpModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/NoOpModifier.java:12)
- [ ] `NoOpModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/NoOpModifier.java:17)
- [ ] `NoOpModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/NoOpModifier.java:21)
- [ ] `NoOpModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/NoOpModifier.java:26)

### `RemoveBiomeSpawnsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `RemoveBiomeSpawnsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:30)
- [ ] `RemoveBiomeSpawnsModifier.RemoveBiomeSpawnsModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:28)
- [ ] `RemoveBiomeSpawnsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:40)
- [ ] `RemoveBiomeSpawnsModifier.applyModifier(2)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:47)
- [ ] `RemoveBiomeSpawnsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:74)
- [ ] `RemoveBiomeSpawnsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:28)
- [ ] `RemoveBiomeSpawnsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:28)
- [ ] `RemoveBiomeSpawnsModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:28)
- [ ] `RemoveBiomeSpawnsModifier.mobs(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java:28)

### `RemoveFeaturesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `RemoveFeaturesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:22)
- [ ] `RemoveFeaturesModifier.RemoveFeaturesModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:19)
- [ ] `RemoveFeaturesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:33)
- [ ] `RemoveFeaturesModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:40)
- [ ] `RemoveFeaturesModifier.shouldRecompileSortedFeatures(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:53)
- [ ] `RemoveFeaturesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:58)
- [ ] `RemoveFeaturesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:19)
- [ ] `RemoveFeaturesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:19)
- [ ] `RemoveFeaturesModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:19)
- [ ] `RemoveFeaturesModifier.features(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:19)
- [ ] `RemoveFeaturesModifier.step(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java:19)

### `RemoveStructureSetEntriesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `RemoveStructureSetEntriesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:24)
- [ ] `RemoveStructureSetEntriesModifier.RemoveStructureSetEntriesModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:21)
- [ ] `RemoveStructureSetEntriesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:34)
- [ ] `RemoveStructureSetEntriesModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:41)
- [ ] `RemoveStructureSetEntriesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:49)
- [ ] `RemoveStructureSetEntriesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:21)
- [ ] `RemoveStructureSetEntriesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:21)
- [ ] `RemoveStructureSetEntriesModifier.structureSets(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:21)
- [ ] `RemoveStructureSetEntriesModifier.entries(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java:21)

### `SetBiomeAttributesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetBiomeAttributesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:22)
- [ ] `SetBiomeAttributesModifier.SetBiomeAttributesModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:19)
- [ ] `SetBiomeAttributesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:33)
- [ ] `SetBiomeAttributesModifier.applyModifier(2)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:40)
- [ ] `SetBiomeAttributesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:51)
- [ ] `SetBiomeAttributesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:19)
- [ ] `SetBiomeAttributesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:19)
- [ ] `SetBiomeAttributesModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:19)
- [ ] `SetBiomeAttributesModifier.attributes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:19)
- [ ] `SetBiomeAttributesModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java:19)

### `SetBiomeClimateModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetBiomeClimateModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:22)
- [ ] `SetBiomeClimateModifier.SetBiomeClimateModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:20)
- [ ] `SetBiomeClimateModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:32)
- [ ] `SetBiomeClimateModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:42)
- [ ] `SetBiomeClimateModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:51)
- [ ] `SetBiomeClimateModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:20)
- [ ] `SetBiomeClimateModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:20)
- [ ] `SetBiomeClimateModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:20)
- [ ] `SetBiomeClimateModifier.climateSettings(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java:20)

### `SetBiomeEffectsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetBiomeEffectsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:25)
- [ ] `SetBiomeEffectsModifier.SetBiomeEffectsModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:23)
- [ ] `SetBiomeEffectsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:35)
- [ ] `SetBiomeEffectsModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:45)
- [ ] `SetBiomeEffectsModifier.applyRequiredEffect(3)`: it's generic (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:57)
- [ ] `SetBiomeEffectsModifier.applyOptionalEffect(3)`: it's generic (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:61)
- [ ] `SetBiomeEffectsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:68)
- [ ] `SetBiomeEffectsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:23)
- [ ] `SetBiomeEffectsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:23)
- [ ] `SetBiomeEffectsModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:23)
- [ ] `SetBiomeEffectsModifier.effects(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java:23)

### `SetBiomeTimelineModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetBiomeTimelineModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:19)
- [ ] `SetBiomeTimelineModifier.SetBiomeTimelineModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:17)
- [ ] `SetBiomeTimelineModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:30)
- [ ] `SetBiomeTimelineModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:41)
- [ ] `SetBiomeTimelineModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:17)
- [ ] `SetBiomeTimelineModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:17)
- [ ] `SetBiomeTimelineModifier.biomes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:17)
- [ ] `SetBiomeTimelineModifier.timeline(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:17)
- [ ] `SetBiomeTimelineModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java:17)

### `SetDimensionAttributesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetDimensionAttributesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:22)
- [ ] `SetDimensionAttributesModifier.SetDimensionAttributesModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:19)
- [ ] `SetDimensionAttributesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:33)
- [ ] `SetDimensionAttributesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:47)
- [ ] `SetDimensionAttributesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:19)
- [ ] `SetDimensionAttributesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:19)
- [ ] `SetDimensionAttributesModifier.dimensionTypes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:19)
- [ ] `SetDimensionAttributesModifier.attributes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:19)
- [ ] `SetDimensionAttributesModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java:19)

### `SetMaterialRuleModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetMaterialRuleModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:24)
- [ ] `SetMaterialRuleModifier.SetMaterialRuleModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:21)
- [ ] `SetMaterialRuleModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:35)
- [ ] `SetMaterialRuleModifier.wrap(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:45)
- [ ] `SetMaterialRuleModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:81)
- [ ] `SetMaterialRuleModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:21)
- [ ] `SetMaterialRuleModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:21)
- [ ] `SetMaterialRuleModifier.targetRules(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:21)
- [ ] `SetMaterialRuleModifier.materialRule(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:21)
- [ ] `SetMaterialRuleModifier.injectionType(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java:21)

### `SetPoolAliasesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetPoolAliasesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:27)
- [ ] `SetPoolAliasesModifier.SetPoolAliasesModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:24)
- [ ] `SetPoolAliasesModifier.validate(1)`: it uses Minecraft's `DataResult` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:39)
- [ ] `SetPoolAliasesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:43)
- [ ] `SetPoolAliasesModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:50)
- [ ] `SetPoolAliasesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:68)
- [ ] `SetPoolAliasesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:24)
- [ ] `SetPoolAliasesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:24)
- [ ] `SetPoolAliasesModifier.structures(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:24)
- [ ] `SetPoolAliasesModifier.poolAliases(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:24)
- [ ] `SetPoolAliasesModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java:24)

### `SetPoolElementProcessorsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetPoolElementProcessorsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:38)
- [ ] `SetPoolElementProcessorsModifier.SetPoolElementProcessorsModifier(6)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:30)
- [ ] `SetPoolElementProcessorsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:50)
- [ ] `SetPoolElementProcessorsModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:61)
- [ ] `SetPoolElementProcessorsModifier.addProcessor(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:77)
- [ ] `SetPoolElementProcessorsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:87)
- [ ] `SetPoolElementProcessorsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:30)
- [ ] `SetPoolElementProcessorsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:30)
- [ ] `SetPoolElementProcessorsModifier.templatePools(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:30)
- [ ] `SetPoolElementProcessorsModifier.locations(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:30)
- [ ] `SetPoolElementProcessorsModifier.processorList(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:30)
- [ ] `SetPoolElementProcessorsModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java:30)

### `SetStructureAttributesModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetStructureAttributesModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:25)
- [ ] `SetStructureAttributesModifier.SetStructureAttributesModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:22)
- [ ] `SetStructureAttributesModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:36)
- [ ] `SetStructureAttributesModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:50)
- [ ] `SetStructureAttributesModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:22)
- [ ] `SetStructureAttributesModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:22)
- [ ] `SetStructureAttributesModifier.structures(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:22)
- [ ] `SetStructureAttributesModifier.attributes(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:22)
- [ ] `SetStructureAttributesModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java:22)

### `SetStructureSpawnConditionModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetStructureSpawnConditionModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:26)
- [ ] `SetStructureSpawnConditionModifier.SetStructureSpawnConditionModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:23)
- [ ] `SetStructureSpawnConditionModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:37)
- [ ] `SetStructureSpawnConditionModifier.applyModifier(2)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:44)
- [ ] `SetStructureSpawnConditionModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:54)
- [ ] `SetStructureSpawnConditionModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:23)
- [ ] `SetStructureSpawnConditionModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:23)
- [ ] `SetStructureSpawnConditionModifier.structures(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:23)
- [ ] `SetStructureSpawnConditionModifier.spawnCondition(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:23)
- [ ] `SetStructureSpawnConditionModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java:23)

### `SetTimelineTracksModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetTimelineTracksModifier.TRACKS_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:26)
- [ ] `SetTimelineTracksModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:29)
- [ ] `SetTimelineTracksModifier.SetTimelineTracksModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:23)
- [ ] `SetTimelineTracksModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:40)
- [ ] `SetTimelineTracksModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:55)
- [ ] `SetTimelineTracksModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:23)
- [ ] `SetTimelineTracksModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:23)
- [ ] `SetTimelineTracksModifier.timelines(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:23)
- [ ] `SetTimelineTracksModifier.tracks(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:23)
- [ ] `SetTimelineTracksModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java:23)

### `SetTreeDecoratorsModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `SetTreeDecoratorsModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:23)
- [ ] `SetTreeDecoratorsModifier.SetTreeDecoratorsModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:20)
- [ ] `SetTreeDecoratorsModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:34)
- [ ] `SetTreeDecoratorsModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:41)
- [ ] `SetTreeDecoratorsModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:53)
- [ ] `SetTreeDecoratorsModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:20)
- [ ] `SetTreeDecoratorsModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:20)
- [ ] `SetTreeDecoratorsModifier.features(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:20)
- [ ] `SetTreeDecoratorsModifier.decorators(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:20)
- [ ] `SetTreeDecoratorsModifier.append(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java:20)

### `StackFeatureModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `StackFeatureModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:28)
- [ ] `StackFeatureModifier.StackFeatureModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:21)
- [ ] `StackFeatureModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:42)
- [ ] `StackFeatureModifier.applyModifier(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:49)
- [ ] `StackFeatureModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:61)
- [ ] `StackFeatureModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:21)
- [ ] `StackFeatureModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:21)
- [ ] `StackFeatureModifier.baseFeatures(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:21)
- [ ] `StackFeatureModifier.stackedFeature(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:21)
- [ ] `StackFeatureModifier.placementType(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java:21)

### `AnyTarget`: it's generic

- [ ] `AnyTarget.TYPE`: its value couldn't be read: an anonymous class (dev/worldgen/lithostitched/impl/worldgen/modifier/target/AnyTarget.java:11)
- [ ] `AnyTarget.AnyTarget(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/AnyTarget.java:10)
- [ ] `AnyTarget.getHolders(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/AnyTarget.java:18)
- [ ] `AnyTarget.getType(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/AnyTarget.java:23)
- [ ] `AnyTarget.key(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/AnyTarget.java:10)

### `ClimateTarget`: it's generic

- [ ] `ClimateTarget.TYPE`: its value couldn't be read: an anonymous class (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:28)
- [ ] `ClimateTarget.ClimateTarget(5)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:21)
- [ ] `ClimateTarget.getHolders(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:47)
- [ ] `ClimateTarget.test(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:56)
- [ ] `ClimateTarget.getType(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:69)
- [ ] `ClimateTarget.key(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:21)
- [ ] `ClimateTarget.temperature(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:21)
- [ ] `ClimateTarget.downfall(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:21)
- [ ] `ClimateTarget.hasPrecipitation(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:21)
- [ ] `ClimateTarget.temperatureModifier(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java:21)

### `HasFeatureTarget`: it's generic

- [ ] `HasFeatureTarget.TYPE`: its value couldn't be read: an anonymous class (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java:17)
- [ ] `HasFeatureTarget.HasFeatureTarget(2)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java:16)
- [ ] `HasFeatureTarget.getHolders(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java:31)
- [ ] `HasFeatureTarget.test(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java:40)
- [ ] `HasFeatureTarget.getType(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java:44)
- [ ] `HasFeatureTarget.key(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java:16)
- [ ] `HasFeatureTarget.feature(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java:16)

### `HolderSetTarget`: it's generic

- [ ] `HolderSetTarget.TYPE`: its value couldn't be read: an anonymous class (dev/worldgen/lithostitched/impl/worldgen/modifier/target/HolderSetTarget.java:13)
- [ ] `HolderSetTarget.HolderSetTarget(2)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/HolderSetTarget.java:12)
- [ ] `HolderSetTarget.getHolders(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/HolderSetTarget.java:20)
- [ ] `HolderSetTarget.getType(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/HolderSetTarget.java:25)
- [ ] `HolderSetTarget.set(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/HolderSetTarget.java:12)
- [ ] `HolderSetTarget.key(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/HolderSetTarget.java:12)

### `RegexTarget`: it's generic

- [ ] `RegexTarget.TYPE`: its value couldn't be read: an anonymous class (dev/worldgen/lithostitched/impl/worldgen/modifier/target/RegexTarget.java:13)
- [ ] `RegexTarget.RegexTarget(2)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/RegexTarget.java:12)
- [ ] `RegexTarget.getHolders(1)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/RegexTarget.java:22)
- [ ] `RegexTarget.getType(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/RegexTarget.java:27)
- [ ] `RegexTarget.key(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/RegexTarget.java:12)
- [ ] `RegexTarget.regex(0)`: its class isn't translated (it's generic) (dev/worldgen/lithostitched/impl/worldgen/modifier/target/RegexTarget.java:12)

### `TemplateList`

- [ ] `TemplateList.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateList.java:11)
- [x] `TemplateList.TemplateList(1)` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateList.java:13)
- [x] `TemplateList.getRandom(1)` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateList.java:17)
- [x] `TemplateList.addAll(1)` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateList.java:21)
- [x] `TemplateList.templates(0)` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateList.java:10)

### `TemplateLists`: it's an interface

- [ ] `TemplateLists.key(1)`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateLists.java:20)
- [ ] `TemplateLists.mansion(2)`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateLists.java:24)
- [ ] `TemplateLists.getRandom(3)`: it uses Minecraft's `RegistryAccess` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateLists.java:28)

### `TemplateLists.Mansion`: it's an interface


### `WrapAquifersModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `WrapAquifersModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:21)
- [ ] `WrapAquifersModifier.WrapAquifersModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:18)
- [ ] `WrapAquifersModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:32)
- [ ] `WrapAquifersModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:36)
- [ ] `WrapAquifersModifier.modifyAquiferFunction(3)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:41)
- [ ] `WrapAquifersModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:18)
- [ ] `WrapAquifersModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:18)
- [ ] `WrapAquifersModifier.dimension(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:18)
- [ ] `WrapAquifersModifier.target(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:18)
- [ ] `WrapAquifersModifier.wrapperFunction(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java:18)

### `WrapDensityFunctionModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `WrapDensityFunctionModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:20)
- [ ] `WrapDensityFunctionModifier.WrapDensityFunctionModifier(4)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:17)
- [ ] `WrapDensityFunctionModifier.create(4)`: it uses WorldgenModifier, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:30)
- [ ] `WrapDensityFunctionModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:36)
- [ ] `WrapDensityFunctionModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:44)
- [ ] `WrapDensityFunctionModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:17)
- [ ] `WrapDensityFunctionModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:17)
- [ ] `WrapDensityFunctionModifier.targetFunction(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:17)
- [ ] `WrapDensityFunctionModifier.wrapperFunction(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java:17)

### `WrapNoiseRouterModifier`: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)

- [ ] `WrapNoiseRouterModifier.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:21)
- [ ] `WrapNoiseRouterModifier.WrapNoiseRouterModifier(5)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:18)
- [ ] `WrapNoiseRouterModifier.apply(1)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:32)
- [ ] `WrapNoiseRouterModifier.codec(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:36)
- [ ] `WrapNoiseRouterModifier.modifyRouterFunction(3)`: it uses Minecraft's `DensityFunction` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:41)
- [ ] `WrapNoiseRouterModifier.predicate(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:18)
- [ ] `WrapNoiseRouterModifier.priority(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:18)
- [ ] `WrapNoiseRouterModifier.dimension(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:18)
- [ ] `WrapNoiseRouterModifier.target(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:18)
- [ ] `WrapNoiseRouterModifier.wrapperFunction(0)`: its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)) (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java:18)

### `AllOfPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `AllOfPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AllOfPlacementCondition.java:10)
- [ ] `AllOfPlacementCondition.AllOfPlacementCondition(1)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AllOfPlacementCondition.java:15)
- [ ] `AllOfPlacementCondition.appendCondition(1)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AllOfPlacementCondition.java:19)
- [ ] `AllOfPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AllOfPlacementCondition.java:23)
- [ ] `AllOfPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AllOfPlacementCondition.java:34)
- [ ] `AllOfPlacementCondition.conditions(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AllOfPlacementCondition.java:9)

### `AnyOfPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `AnyOfPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AnyOfPlacementCondition.java:9)
- [ ] `AnyOfPlacementCondition.AnyOfPlacementCondition(1)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AnyOfPlacementCondition.java:8)
- [ ] `AnyOfPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AnyOfPlacementCondition.java:14)
- [ ] `AnyOfPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AnyOfPlacementCondition.java:25)
- [ ] `AnyOfPlacementCondition.conditions(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AnyOfPlacementCondition.java:8)

### `GridPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `GridPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:13)
- [ ] `GridPlacementCondition.GridPlacementCondition(4)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:11)
- [ ] `GridPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:23)
- [ ] `GridPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:40)
- [ ] `GridPlacementCondition.radius(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:11)
- [ ] `GridPlacementCondition.distBetweenPoints(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:11)
- [ ] `GridPlacementCondition.condition(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:11)
- [ ] `GridPlacementCondition.allowedCount(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java:11)

### `HeightFilterPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `HeightFilterPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:18)
- [ ] `HeightFilterPlacementCondition.HeightFilterPlacementCondition(3)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:15)
- [ ] `HeightFilterPlacementCondition.validate(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:28)
- [ ] `HeightFilterPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:34)
- [ ] `HeightFilterPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:45)
- [ ] `HeightFilterPlacementCondition.rangeType(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:15)
- [ ] `HeightFilterPlacementCondition.heightmap(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:15)
- [ ] `HeightFilterPlacementCondition.permittedRange(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:15)

### `HeightFilterPlacementCondition.RangeType`: its constants carry values

- [ ] `HeightFilterPlacementCondition.RangeType.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:54)
- [ ] `HeightFilterPlacementCondition.RangeType.RangeType(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:57)
- [ ] `HeightFilterPlacementCondition.RangeType.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java:61)

### `InBiomePlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `InBiomePlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/InBiomePlacementCondition.java:12)
- [ ] `InBiomePlacementCondition.InBiomePlacementCondition(1)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/InBiomePlacementCondition.java:11)
- [ ] `InBiomePlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/InBiomePlacementCondition.java:16)
- [ ] `InBiomePlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/InBiomePlacementCondition.java:23)
- [ ] `InBiomePlacementCondition.biomes(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/InBiomePlacementCondition.java:11)

### `MultipleOfPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `MultipleOfPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/MultipleOfPlacementCondition.java:12)
- [ ] `MultipleOfPlacementCondition.MultipleOfPlacementCondition(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/MultipleOfPlacementCondition.java:11)
- [ ] `MultipleOfPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/MultipleOfPlacementCondition.java:20)
- [ ] `MultipleOfPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/MultipleOfPlacementCondition.java:35)
- [ ] `MultipleOfPlacementCondition.conditions(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/MultipleOfPlacementCondition.java:11)
- [ ] `MultipleOfPlacementCondition.allowedCount(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/MultipleOfPlacementCondition.java:11)

### `NotPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `NotPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/NotPlacementCondition.java:8)
- [ ] `NotPlacementCondition.NotPlacementCondition(1)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/NotPlacementCondition.java:7)
- [ ] `NotPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/NotPlacementCondition.java:12)
- [ ] `NotPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/NotPlacementCondition.java:17)
- [ ] `NotPlacementCondition.condition(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/NotPlacementCondition.java:7)

### `OffsetPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `OffsetPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/OffsetPlacementCondition.java:9)
- [ ] `OffsetPlacementCondition.OffsetPlacementCondition(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/OffsetPlacementCondition.java:8)
- [ ] `OffsetPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/OffsetPlacementCondition.java:17)
- [ ] `OffsetPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/OffsetPlacementCondition.java:22)
- [ ] `OffsetPlacementCondition.condition(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/OffsetPlacementCondition.java:8)
- [ ] `OffsetPlacementCondition.offset(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/OffsetPlacementCondition.java:8)

### `SampleAquifersPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `SampleAquifersPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java:17)
- [ ] `SampleAquifersPlacementCondition.SampleAquifersPlacementCondition(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java:16)
- [ ] `SampleAquifersPlacementCondition.SampleAquifersPlacementCondition(3)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java:26)
- [ ] `SampleAquifersPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java:30)
- [ ] `SampleAquifersPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java:47)
- [ ] `SampleAquifersPlacementCondition.target(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java:16)
- [ ] `SampleAquifersPlacementCondition.range(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java:16)

### `SampleDensityPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `SampleDensityPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java:14)
- [ ] `SampleDensityPlacementCondition.SampleDensityPlacementCondition(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java:13)
- [ ] `SampleDensityPlacementCondition.SampleDensityPlacementCondition(3)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java:23)
- [ ] `SampleDensityPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java:27)
- [ ] `SampleDensityPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java:39)
- [ ] `SampleDensityPlacementCondition.densityFunction(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java:13)
- [ ] `SampleDensityPlacementCondition.range(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java:13)

### `SampleNoiseRouterPlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `SampleNoiseRouterPlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java:15)
- [ ] `SampleNoiseRouterPlacementCondition.SampleNoiseRouterPlacementCondition(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java:14)
- [ ] `SampleNoiseRouterPlacementCondition.SampleNoiseRouterPlacementCondition(3)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java:24)
- [ ] `SampleNoiseRouterPlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java:28)
- [ ] `SampleNoiseRouterPlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java:45)
- [ ] `SampleNoiseRouterPlacementCondition.target(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java:14)
- [ ] `SampleNoiseRouterPlacementCondition.range(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java:14)

### `TruePlacementCondition`: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)

- [ ] `TruePlacementCondition.INSTANCE`: it uses TruePlacementCondition, which isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/TruePlacementCondition.java:8)
- [ ] `TruePlacementCondition.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/TruePlacementCondition.java:9)
- [ ] `TruePlacementCondition.test(2)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/TruePlacementCondition.java:11)
- [ ] `TruePlacementCondition.codec(0)`: its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)) (dev/worldgen/lithostitched/impl/worldgen/placementcondition/TruePlacementCondition.java:16)

### `ConditionPlacement`: it implements Minecraft's `PlacementFilter`

- [ ] `ConditionPlacement.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/ConditionPlacement.java:11)
- [ ] `ConditionPlacement.ConditionPlacement(1)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/ConditionPlacement.java:10)
- [ ] `ConditionPlacement.shouldPlace(3)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/ConditionPlacement.java:15)
- [ ] `ConditionPlacement.codec(0)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/ConditionPlacement.java:19)
- [ ] `ConditionPlacement.condition(0)`: its class isn't translated (it implements Minecraft's `PlacementFilter`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/ConditionPlacement.java:10)

### `NoiseSlopePlacement`: it implements Minecraft's `PlacementModifier`

- [ ] `NoiseSlopePlacement.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:17)
- [ ] `NoiseSlopePlacement.NoiseSlopePlacement(5)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:16)
- [ ] `NoiseSlopePlacement.modify(4)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:28)
- [ ] `NoiseSlopePlacement.count(2)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:36)
- [ ] `NoiseSlopePlacement.codec(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:42)
- [ ] `NoiseSlopePlacement.noise(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:16)
- [ ] `NoiseSlopePlacement.slope(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:16)
- [ ] `NoiseSlopePlacement.offset(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:16)
- [ ] `NoiseSlopePlacement.xzScale(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:16)
- [ ] `NoiseSlopePlacement.yScale(0)`: its class isn't translated (it implements Minecraft's `PlacementModifier`) (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java:16)

### `RandomEntries`: it implements Minecraft's `PoolAliasBinding`

- [ ] `RandomEntries.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:18)
- [ ] `RandomEntries.RandomEntries(2)`: its class isn't translated (it implements Minecraft's `PoolAliasBinding`) (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:17)
- [ ] `RandomEntries.validate(1)`: it uses Minecraft's `DataResult` (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:27)
- [ ] `RandomEntries.forEachResolved(2)`: its class isn't translated (it implements Minecraft's `PoolAliasBinding`) (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:31)
- [ ] `RandomEntries.allTargets(0)`: its class isn't translated (it implements Minecraft's `PoolAliasBinding`) (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:39)
- [ ] `RandomEntries.codec(0)`: its class isn't translated (it implements Minecraft's `PoolAliasBinding`) (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:43)
- [ ] `RandomEntries.aliases(0)`: its class isn't translated (it implements Minecraft's `PoolAliasBinding`) (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:17)
- [ ] `RandomEntries.pools(0)`: its class isn't translated (it implements Minecraft's `PoolAliasBinding`) (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java:17)

### `DelegatingConfig`: another class has the same name, in a different package

- [ ] `DelegatingConfig.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:32)
- [ ] `DelegatingConfig.DelegatingConfig(10)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.DelegatingConfig(1)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:49)
- [ ] `DelegatingConfig.validate(1)`: it uses Minecraft's `DataResult` (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:64)
- [ ] `DelegatingConfig.getName(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:70)
- [ ] `DelegatingConfig.shouldCancelPlacement(4)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:74)
- [ ] `DelegatingConfig.delegate(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.name(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.placementCondition(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.allowedDepth(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.forcedCount(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.maxCount(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.allowBoundingBoxCollisions(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.otherPiecesCanIntersect(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.terrainMatchingHeightmap(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)
- [ ] `DelegatingConfig.overrideTerrainAdaptation(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java:20)

### `DelegatingElementBuilderImpl`: it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)

- [ ] `DelegatingElementBuilderImpl.DelegatingElementBuilderImpl(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:24)
- [ ] `DelegatingElementBuilderImpl.forcedCount(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:28)
- [ ] `DelegatingElementBuilderImpl.limitedCount(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:38)
- [ ] `DelegatingElementBuilderImpl.allowedDepth(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:48)
- [ ] `DelegatingElementBuilderImpl.terrainAdaptation(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:54)
- [ ] `DelegatingElementBuilderImpl.condition(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:60)
- [ ] `DelegatingElementBuilderImpl.named(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:66)
- [ ] `DelegatingElementBuilderImpl.terrainMatchingHeightmap(1)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:72)
- [ ] `DelegatingElementBuilderImpl.allowBoundingBoxCollisions(0)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:78)
- [ ] `DelegatingElementBuilderImpl.otherPiecesCanIntersect(0)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:84)
- [ ] `DelegatingElementBuilderImpl.build(0)`: its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java:90)

### `DelegatingPoolElement`: it extends Minecraft's `StructurePoolElement`

- [ ] `DelegatingPoolElement.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:23)
- [ ] `DelegatingPoolElement.TYPE`: it uses Minecraft's `StructurePoolElementType` (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:24)
- [ ] `DelegatingPoolElement.DelegatingPoolElement(1)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:27)
- [ ] `DelegatingPoolElement.DelegatingPoolElement(4)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:32)
- [ ] `DelegatingPoolElement.config(0)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:49)
- [ ] `DelegatingPoolElement.delegate(0)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:53)
- [ ] `DelegatingPoolElement.minDepth(0)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:57)
- [ ] `DelegatingPoolElement.prioritized(0)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:61)
- [ ] `DelegatingPoolElement.getSize(2)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:65)
- [ ] `DelegatingPoolElement.getShuffledJigsawBlocks(4)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:69)
- [ ] `DelegatingPoolElement.getBoundingBox(3)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:73)
- [ ] `DelegatingPoolElement.place(11)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:77)
- [ ] `DelegatingPoolElement.setProjection(1)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:107)
- [ ] `DelegatingPoolElement.getType(0)`: its class isn't translated (it extends Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java:113)

### `LithostitchedFeatureConfig`: it uses Minecraft's `Holder`

- [x] `LithostitchedFeatureConfig.DEFAULT_JIGSAW_NAME` (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java:10)
- [x] `LithostitchedFeatureConfig.DEFAULT_TARGET_NAME` (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java:11)
- [ ] `LithostitchedFeatureConfig.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java:12)
- [ ] `LithostitchedFeatureConfig.LithostitchedFeatureConfig(3)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java:9)
- [ ] `LithostitchedFeatureConfig.feature(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java:9)
- [ ] `LithostitchedFeatureConfig.jigsawName(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java:9)
- [ ] `LithostitchedFeatureConfig.targetName(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java:9)

### `ApplyRandomStructureProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `ApplyRandomStructureProcessor.SET_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java:25)
- [ ] `ApplyRandomStructureProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java:28)
- [ ] `ApplyRandomStructureProcessor.ApplyRandomStructureProcessor(2)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java:23)
- [ ] `ApplyRandomStructureProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java:38)
- [ ] `ApplyRandomStructureProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java:61)
- [ ] `ApplyRandomStructureProcessor.processorLists(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java:23)
- [ ] `ApplyRandomStructureProcessor.randomSettings(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java:23)

### `BlockSwapStructureProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `BlockSwapStructureProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/BlockSwapStructureProcessor.java:20)
- [ ] `BlockSwapStructureProcessor.BlockSwapStructureProcessor(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/BlockSwapStructureProcessor.java:19)
- [ ] `BlockSwapStructureProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/BlockSwapStructureProcessor.java:29)
- [ ] `BlockSwapStructureProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/BlockSwapStructureProcessor.java:51)
- [ ] `BlockSwapStructureProcessor.blockSwapMap(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/BlockSwapStructureProcessor.java:19)

### `ConditionProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `ConditionProcessor.PROCESSOR_CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:21)
- [ ] `ConditionProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:22)
- [ ] `ConditionProcessor.ConditionProcessor(4)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:36)
- [ ] `ConditionProcessor.randomSettings(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:43)
- [ ] `ConditionProcessor.condition(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:47)
- [ ] `ConditionProcessor.thenRun(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:51)
- [ ] `ConditionProcessor.elseRun(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:55)
- [ ] `ConditionProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:59)
- [ ] `ConditionProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java:82)

### `AllOf`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `AllOf.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AllOf.java:11)
- [ ] `AllOf.AllOf(1)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AllOf.java:10)
- [ ] `AllOf.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AllOf.java:13)
- [ ] `AllOf.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AllOf.java:24)
- [ ] `AllOf.conditions(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AllOf.java:10)

### `AnyOf`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `AnyOf.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AnyOf.java:11)
- [ ] `AnyOf.AnyOf(1)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AnyOf.java:10)
- [ ] `AnyOf.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AnyOf.java:13)
- [ ] `AnyOf.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AnyOf.java:24)
- [ ] `AnyOf.conditions(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AnyOf.java:10)

### `MatchingBiomes`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `MatchingBiomes.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBiomes.java:18)
- [ ] `MatchingBiomes.MatchingBiomes(2)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBiomes.java:17)
- [ ] `MatchingBiomes.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBiomes.java:26)
- [ ] `MatchingBiomes.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBiomes.java:38)
- [ ] `MatchingBiomes.biomes(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBiomes.java:17)
- [ ] `MatchingBiomes.position(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBiomes.java:17)

### `MatchingBlocks`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `MatchingBlocks.DEFAULT_PREDICATE`: it uses Minecraft's `StatePropertiesPredicate` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:18)
- [ ] `MatchingBlocks.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:19)
- [ ] `MatchingBlocks.MatchingBlocks(3)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:17)
- [ ] `MatchingBlocks.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:28)
- [ ] `MatchingBlocks.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:34)
- [ ] `MatchingBlocks.blocks(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:17)
- [ ] `MatchingBlocks.properties(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:17)
- [ ] `MatchingBlocks.matchType(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java:17)

### `Not`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `Not.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Not.java:10)
- [ ] `Not.Not(1)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Not.java:9)
- [ ] `Not.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Not.java:12)
- [ ] `Not.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Not.java:17)
- [ ] `Not.condition(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Not.java:9)

### `Position`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `Position.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:15)
- [ ] `Position.Position(2)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:14)
- [ ] `Position.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:23)
- [ ] `Position.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:28)
- [ ] `Position.predicate(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:14)
- [ ] `Position.anchor(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:14)

### `Position.PosAnchor`: its constants carry values

- [ ] `Position.PosAnchor.CODEC`: it uses Minecraft's `Codec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:37)
- [ ] `Position.PosAnchor.PosAnchor(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:40)
- [ ] `Position.PosAnchor.get(1)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:44)
- [ ] `Position.PosAnchor.getSerializedName(0)`: its class isn't translated (its constants carry values) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java:48)

### `RandomChance`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `RandomChance.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/RandomChance.java:11)
- [ ] `RandomChance.RandomChance(1)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/RandomChance.java:10)
- [ ] `RandomChance.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/RandomChance.java:13)
- [ ] `RandomChance.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/RandomChance.java:18)
- [ ] `RandomChance.chance(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/RandomChance.java:10)

### `True`: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)

- [ ] `True.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/True.java:10)
- [ ] `True.True(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/True.java:9)
- [ ] `True.test(4)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/True.java:12)
- [ ] `True.codec(0)`: its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)) (dev/worldgen/lithostitched/impl/worldgen/processor/condition/True.java:17)

### `DiscardInputProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `DiscardInputProcessor.INSTANCE`: it uses DiscardInputProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/DiscardInputProcessor.java:11)
- [ ] `DiscardInputProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/DiscardInputProcessor.java:12)
- [ ] `DiscardInputProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/DiscardInputProcessor.java:14)
- [ ] `DiscardInputProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/DiscardInputProcessor.java:20)

### `NonTemplateProcessorHelper`

- [ ] `NonTemplateProcessorHelper.getProcessor(2)`: it uses Minecraft's `StructureProcessorList` (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java:21)
- [ ] `NonTemplateProcessorHelper.getProcessor(2)`: it uses Minecraft's `StructureProcessorList` (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java:26)
- [ ] `NonTemplateProcessorHelper.processBlock(6)`: it uses Minecraft's `BlockState` (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java:31)
- [ ] `NonTemplateProcessorHelper.processBlock(7)`: it uses Minecraft's `BlockState` (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java:37)
- [ ] `NonTemplateProcessorHelper.processBlock(5)`: it uses Minecraft's `BlockState` (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java:60)
- [ ] `NonTemplateProcessorHelper.getProcessorAndApply(7)`: it uses ReferencePosDuck, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java:73)
- [ ] `NonTemplateProcessorHelper.getProcessorAndApply(10)`: it uses ReferencePosDuck, which isn't translated (it's an interface) (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java:79)

### `ReferenceStructureProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `ReferenceStructureProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/ReferenceStructureProcessor.java:18)
- [ ] `ReferenceStructureProcessor.ReferenceStructureProcessor(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ReferenceStructureProcessor.java:17)
- [ ] `ReferenceStructureProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ReferenceStructureProcessor.java:27)
- [ ] `ReferenceStructureProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ReferenceStructureProcessor.java:44)
- [ ] `ReferenceStructureProcessor.processorLists(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ReferenceStructureProcessor.java:17)

### `ScheduleTickProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `ScheduleTickProcessor.INSTANCE`: it uses ScheduleTickProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ScheduleTickProcessor.java:13)
- [ ] `ScheduleTickProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/ScheduleTickProcessor.java:14)
- [ ] `ScheduleTickProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ScheduleTickProcessor.java:16)
- [ ] `ScheduleTickProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/ScheduleTickProcessor.java:30)

### `SetBlockProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `SetBlockProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:22)
- [ ] `SetBlockProcessor.SetBlockProcessor(4)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:20)
- [ ] `SetBlockProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:32)
- [ ] `SetBlockProcessor.withState(3)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:52)
- [ ] `SetBlockProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:56)
- [ ] `SetBlockProcessor.stateProvider(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:20)
- [ ] `SetBlockProcessor.preserveState(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:20)
- [ ] `SetBlockProcessor.randomMode(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:20)
- [ ] `SetBlockProcessor.modifier(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java:20)

### `UnboundReferenceProcessor`: it implements Minecraft's `StructureProcessor`

- [ ] `UnboundReferenceProcessor.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java:19)
- [ ] `UnboundReferenceProcessor.UnboundReferenceProcessor(1)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java:25)
- [ ] `UnboundReferenceProcessor.of(1)`: it uses UnboundReferenceProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java:29)
- [ ] `UnboundReferenceProcessor.key(1)`: it uses Minecraft's `ResourceKey` (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java:33)
- [ ] `UnboundReferenceProcessor.name(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java:37)
- [ ] `UnboundReferenceProcessor.processBlock(6)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java:41)
- [ ] `UnboundReferenceProcessor.codec(0)`: its class isn't translated (it implements Minecraft's `StructureProcessor`) (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java:59)

### `WeightedProvider`: it implements Minecraft's `BlockStateProvider`

- [ ] `WeightedProvider.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/stateprovider/WeightedProvider.java:14)
- [ ] `WeightedProvider.WeightedProvider(1)`: its class isn't translated (it implements Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/stateprovider/WeightedProvider.java:13)
- [ ] `WeightedProvider.getState(3)`: its class isn't translated (it implements Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/stateprovider/WeightedProvider.java:18)
- [ ] `WeightedProvider.getOptionalState(3)`: its class isn't translated (it implements Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/stateprovider/WeightedProvider.java:25)
- [ ] `WeightedProvider.codec(0)`: its class isn't translated (it implements Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/stateprovider/WeightedProvider.java:30)
- [ ] `WeightedProvider.providers(0)`: its class isn't translated (it implements Minecraft's `BlockStateProvider`) (dev/worldgen/lithostitched/impl/worldgen/stateprovider/WeightedProvider.java:13)

### `AlternateJigsawConfig`: it uses Minecraft's `Holder`

- [ ] `AlternateJigsawConfig.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:36)
- [ ] `AlternateJigsawConfig.AlternateJigsawConfig(11)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.create(12)`: it uses AlternateJigsawConfig, which isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:54)
- [ ] `AlternateJigsawConfig.setPoolAliases(2)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:83)
- [ ] `AlternateJigsawConfig.projectStartToHeightmap(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:105)
- [ ] `AlternateJigsawConfig.startPool(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.startJigsawName(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.size(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.fixedRotation(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.startHeight(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.useExpansionHack(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.startProjection(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.maxDistanceFromCenter(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.poolAliases(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.dimensionPadding(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)
- [ ] `AlternateJigsawConfig.liquidSettings(0)`: its class isn't translated (it uses Minecraft's `Holder`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java:23)

### `AlternateJigsawGenerator`

- [ ] `AlternateJigsawGenerator.generate(6)`: it uses Minecraft's `GenerationStub` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:48)
- [ ] `AlternateJigsawGenerator.pieceWithinPaddingBounds(3)`: it uses Minecraft's `LevelHeightAccessor` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:167)
- [ ] `AlternateJigsawGenerator.findNamedJigsaw(6)`: it uses Minecraft's `BlockPos` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:177)
- [ ] `AlternateJigsawGenerator.generatePieces(14)`: it uses Minecraft's `GenerationContext` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:189)
- [ ] `AlternateJigsawGenerator.getConfig(1)`: it uses `DelegatingConfig` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:216)

### `AlternateJigsawGenerator.PieceState`: it uses Minecraft's `PoolElementStructurePiece`

- [ ] `AlternateJigsawGenerator.PieceState.PieceState(3)`: its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:222)
- [ ] `AlternateJigsawGenerator.PieceState.piece(0)`: its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:222)
- [ ] `AlternateJigsawGenerator.PieceState.octree(0)`: its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:222)
- [ ] `AlternateJigsawGenerator.PieceState.currentSize(0)`: its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:222)

### `AlternateJigsawGenerator.StructurePoolGenerator`: it uses Minecraft's `GenerationContext`

- [ ] `AlternateJigsawGenerator.StructurePoolGenerator.StructurePoolGenerator(8)`: its class isn't translated (it uses Minecraft's `GenerationContext`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:237)
- [ ] `AlternateJigsawGenerator.StructurePoolGenerator.generatePiece(7)`: its class isn't translated (it uses Minecraft's `GenerationContext`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:257)
- [ ] `AlternateJigsawGenerator.StructurePoolGenerator.findAndTestChildCandidates(12)`: its class isn't translated (it uses Minecraft's `GenerationContext`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:296)
- [ ] `AlternateJigsawGenerator.StructurePoolGenerator.getPoolElements(4)`: its class isn't translated (it uses Minecraft's `GenerationContext`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:334)
- [ ] `AlternateJigsawGenerator.StructurePoolGenerator.findValidChildPiece(10)`: its class isn't translated (it uses Minecraft's `GenerationContext`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:378)
- [ ] `AlternateJigsawGenerator.StructurePoolGenerator.getTemplatePoolHolder(2)`: its class isn't translated (it uses Minecraft's `GenerationContext`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:525)
- [ ] `AlternateJigsawGenerator.StructurePoolGenerator.adjustJigsawPos(1)`: it uses Minecraft's `BlockPos` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java:543)

### `AlternateJigsawStructure`: it extends Minecraft's `Structure`

- [ ] `AlternateJigsawStructure.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:21)
- [ ] `AlternateJigsawStructure.TYPE`: it uses Minecraft's `StructureType` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:26)
- [ ] `AlternateJigsawStructure.validate(1)`: it uses Minecraft's `DataResult` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:29)
- [ ] `AlternateJigsawStructure.AlternateJigsawStructure(2)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:40)
- [ ] `AlternateJigsawStructure.setPoolAliases(2)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:45)
- [ ] `AlternateJigsawStructure.config(0)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:49)
- [ ] `AlternateJigsawStructure.findGenerationPoint(1)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:53)
- [ ] `AlternateJigsawStructure.generate(6)`: it uses Minecraft's `GenerationStub` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:68)
- [ ] `AlternateJigsawStructure.type(0)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java:74)

### `BoxOctree`: it uses Minecraft's `AABB`

- [x] `BoxOctree.SUBDIVIDE_THRESHOLD` (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:10)
- [x] `BoxOctree.MAXIMUM_DEPTH` (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:11)
- [ ] `BoxOctree.BoxOctree(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:18)
- [ ] `BoxOctree.BoxOctree(2)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:22)
- [ ] `BoxOctree.roundAwayFromZero(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:30)
- [ ] `BoxOctree.subdivide(0)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:34)
- [ ] `BoxOctree.addBox(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:166)
- [ ] `BoxOctree.boundaryEntirelyContains(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:188)
- [ ] `BoxOctree.boundaryIntersects(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:193)
- [ ] `BoxOctree.withinBoundsButNotIntersectingChildren(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:197)
- [ ] `BoxOctree.intersectsAnyBox(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:201)
- [ ] `BoxOctree.boundaryContains(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:219)
- [ ] `BoxOctree.withinAnyBox(1)`: its class isn't translated (it uses Minecraft's `AABB`) (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java:223)

### `DelegatingConfig`: another class has the same name, in a different package

- [ ] `DelegatingConfig.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:17)
- [ ] `DelegatingConfig.DelegatingConfig(2)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:30)
- [ ] `DelegatingConfig.DelegatingConfig(3)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:34)
- [ ] `DelegatingConfig.delegate(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:40)
- [ ] `DelegatingConfig.spawnCondition(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:44)
- [ ] `DelegatingConfig.attributes(0)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:48)
- [ ] `DelegatingConfig.setSpawnCondition(2)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:52)
- [ ] `DelegatingConfig.setAttributes(2)`: its class isn't translated (another class has the same name, in a different package) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java:68)

### `DelegatingStructure`: it extends Minecraft's `Structure`

- [ ] `DelegatingStructure.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:21)
- [ ] `DelegatingStructure.TYPE`: it uses Minecraft's `StructureType` (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:22)
- [ ] `DelegatingStructure.DelegatingStructure(1)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:25)
- [ ] `DelegatingStructure.config(0)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:30)
- [ ] `DelegatingStructure.delegate(0)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:34)
- [ ] `DelegatingStructure.findValidGenerationPoint(1)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:38)
- [ ] `DelegatingStructure.isValid(2)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:42)
- [ ] `DelegatingStructure.findGenerationPoint(1)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:50)
- [ ] `DelegatingStructure.afterPlace(7)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:54)
- [ ] `DelegatingStructure.type(0)`: its class isn't translated (it extends Minecraft's `Structure`) (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:66)
- [ ] `DelegatingStructure.createSettings(1)`: it uses Minecraft's `StructureSettings` (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java:70)

### `LithostitchedTemplates`: it implements `Iterable`

- [ ] `LithostitchedTemplates.add(2)`: its class isn't translated (it implements `Iterable`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:17)
- [ ] `LithostitchedTemplates.shuffle(1)`: its class isn't translated (it implements `Iterable`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:22)
- [ ] `LithostitchedTemplates.stream(0)`: its class isn't translated (it implements `Iterable`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:29)
- [ ] `LithostitchedTemplates.iterator(0)`: its class isn't translated (it implements `Iterable`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:33)

### `LithostitchedTemplates.WeightedEntry`: it uses Minecraft's `StructurePoolElement`

- [ ] `LithostitchedTemplates.WeightedEntry.WeightedEntry(3)`: its class isn't translated (it uses Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:46)
- [ ] `LithostitchedTemplates.WeightedEntry.copy(0)`: its class isn't translated (it uses Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:53)
- [ ] `LithostitchedTemplates.WeightedEntry.getRandWeight(0)`: its class isn't translated (it uses Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:57)
- [ ] `LithostitchedTemplates.WeightedEntry.setRandom(1)`: its class isn't translated (it uses Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:61)
- [ ] `LithostitchedTemplates.WeightedEntry.getElement(0)`: its class isn't translated (it uses Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:65)
- [ ] `LithostitchedTemplates.WeightedEntry.getIndex(0)`: its class isn't translated (it uses Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:69)
- [ ] `LithostitchedTemplates.WeightedEntry.toString(0)`: its class isn't translated (it uses Minecraft's `StructurePoolElement`) (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java:73)

### `StructureAttributeHandler`

- [x] `StructureAttributeHandler.DELAY_BETWEEN_STRUCTURE_CHECKS` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:18)
- [x] `StructureAttributeHandler.STRUCTURE_ATTRIBUTE_LERP` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:19)
- [ ] `StructureAttributeHandler.playersToStructures`: it uses Minecraft's `ServerPlayer` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:20)
- [x] `StructureAttributeHandler.delay` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:21)
- [ ] `StructureAttributeHandler.tick(1)`: it uses Minecraft's `ServerLevel` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:23)
- [ ] `StructureAttributeHandler.disconnect(1)`: it uses Minecraft's `ServerPlayer` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:49)
- [ ] `StructureAttributeHandler.hasStructureNotChanged(2)`: it uses Minecraft's `ServerPlayer` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:53)
- [ ] `StructureAttributeHandler.updateAttributes(3)`: it uses Minecraft's `ServerPlayer` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:57)
- [ ] `StructureAttributeHandler.sendPacket(2)`: it uses Minecraft's `ServerPlayer` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java:68)

### `BranchedMegaJungleFoliagePlacer`: it extends Minecraft's `MegaJungleFoliagePlacer`

- [ ] `BranchedMegaJungleFoliagePlacer.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaJungleFoliagePlacer.java:21)
- [ ] `BranchedMegaJungleFoliagePlacer.TYPE`: it uses Minecraft's `FoliagePlacerType` (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaJungleFoliagePlacer.java:24)
- [ ] `BranchedMegaJungleFoliagePlacer.BranchedMegaJungleFoliagePlacer(3)`: its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaJungleFoliagePlacer.java:26)
- [ ] `BranchedMegaJungleFoliagePlacer.type(0)`: its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaJungleFoliagePlacer.java:30)
- [ ] `BranchedMegaJungleFoliagePlacer.createFoliage(9)`: its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaJungleFoliagePlacer.java:34)
- [ ] `BranchedMegaJungleFoliagePlacer.shouldSkipLocation(6)`: its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaJungleFoliagePlacer.java:82)

### `BranchedMegaPineFoliagePlacer`: it extends Minecraft's `MegaPineFoliagePlacer`

- [ ] `BranchedMegaPineFoliagePlacer.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaPineFoliagePlacer.java:22)
- [ ] `BranchedMegaPineFoliagePlacer.TYPE`: it uses Minecraft's `FoliagePlacerType` (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaPineFoliagePlacer.java:27)
- [ ] `BranchedMegaPineFoliagePlacer.BranchedMegaPineFoliagePlacer(3)`: its class isn't translated (it extends Minecraft's `MegaPineFoliagePlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaPineFoliagePlacer.java:29)
- [ ] `BranchedMegaPineFoliagePlacer.type(0)`: its class isn't translated (it extends Minecraft's `MegaPineFoliagePlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaPineFoliagePlacer.java:33)
- [ ] `BranchedMegaPineFoliagePlacer.createFoliage(9)`: its class isn't translated (it extends Minecraft's `MegaPineFoliagePlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaPineFoliagePlacer.java:37)

### `LargeMangroveRootPlacer`: it extends Minecraft's `MangroveRootPlacer`

- [ ] `LargeMangroveRootPlacer.CODEC`: it uses Minecraft's `MapCodec` (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:37)
- [ ] `LargeMangroveRootPlacer.TYPE`: it uses Minecraft's `RootPlacerType` (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:42)
- [ ] `LargeMangroveRootPlacer.LargeMangroveRootPlacer(4)`: its class isn't translated (it extends Minecraft's `MangroveRootPlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:45)
- [ ] `LargeMangroveRootPlacer.type(0)`: its class isn't translated (it extends Minecraft's `MangroveRootPlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:52)
- [ ] `LargeMangroveRootPlacer.placeRoots(6)`: its class isn't translated (it extends Minecraft's `MangroveRootPlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:56)
- [ ] `LargeMangroveRootPlacer.getTrunkOrigin(2)`: its class isn't translated (it extends Minecraft's `MangroveRootPlacer`) (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:91)
- [ ] `ClientLevelMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:9)
- [ ] `EnvironmentAttributeSystemMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:20)
- [ ] `IntegratedServerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `MusicManagerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `MaterialSystemMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `BeardifierMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:18)
- [ ] `BiomeAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `BiomeGenerationSettingsAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:14)
- [ ] `BiomeSourceInvoker`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:9)
- [ ] `BlockPredicateMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:16)
- [ ] `ChunkGeneratorAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:17)
- [ ] `ChunkMapMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:27)
- [ ] `NoiseBasedChunkGeneratorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:7)
- [ ] `DimensionTypeAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:10)
- [ ] `EnvironmentAttributeMapBuilderMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `GameTestServerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `HolderReferenceAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:8)
- [ ] `JigsawStructureAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `JigsawStructureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:32)
- [ ] `MappedRegistryAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `MaterialRuleContextMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `MNBSMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:30)
- [ ] `MNBSPLMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:25)
- [ ] `ParameterListAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:14)
- [ ] `MobSpawnSettingsAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `MultiNoiseBiomeSourceAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `ServerLifecycleHooksMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:7)
- [ ] `NoiseBasedChunkGeneratorAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `NoiseBasedChunkGeneratorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:31)
- [ ] `NoiseGeneratorSettingsAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `PlacedFeatureAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `PoolAliasLookupMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `PendingRegistrationMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:20)
- [ ] `ResourceManagerRegistryLoadTaskMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `DesertPyramidStructureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:19)
- [ ] `EndCityPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `IglooPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `MineshaftCorridorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:21)
- [ ] `MineshaftPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:17)
- [ ] `MonsterThroneMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:16)
- [ ] `NetherFossilPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:23)
- [ ] `OceanRuinPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:14)
- [ ] `PortalRoomMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:16)
- [ ] `RuinedPortalPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:23)
- [ ] `ShipwreckPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `StructurePieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:20)
- [ ] `StructureStartMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:20)
- [ ] `WoodlandMansionPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `WoodlandMansionStructureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:19)
- [ ] `RandomStateAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:10)
- [ ] `SinglePoolElementAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:14)
- [ ] `StructureProcessorListAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `StructureSetAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `StructureTemplatePoolAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `StructureTemplatePoolMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:15)
- [ ] `FloorRoomCollectionMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:9)
- [ ] `FirstFloorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `SecondFloorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `ThirdFloorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:8)
- [ ] `PiecePlacerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:15)
- [ ] `WoodlandMansionPieceMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `WoodlandMansionStructureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:21)
- [ ] `NetherFossilStructureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:20)
- [ ] `RuinedPortalStructureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
- [ ] `ShipwreckStructureMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:21)
- [ ] `TimelineAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `BiomeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:20)
- [ ] `EnvironmentAttributeProbeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:18)
- [ ] `EnvironmentAttributeSystemBuilderMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `EnvironmentAttributeSystemMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:36)
- [ ] `EnvironmentAttributeSystemValueSamplerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:22)
- [ ] `SpatialAttributeInterpolatorMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:12)
- [ ] `TreeFeatureAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:11)
- [ ] `VillagerTypeMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:21)
- [ ] `DedicatedServerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java:13)
