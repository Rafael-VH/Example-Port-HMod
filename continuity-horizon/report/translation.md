# Java translated to Rust

Translated 28 of 494 methods (2 more are done another way in Horizon) and 23 of 54 static fields to Rust; 1 of 3 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 352 | `ContinuityFeatureStates.get(0)`: it uses ContinuityFeatureStates, which isn't translated (it's an interface) |
| Other Java this translator doesn't handle yet | 82 | `CachingPredicates.affectsSprites(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 15 | `EmissiveItemGeometry.combine(2)`: it uses Minecraft's `BakedQuad` |
| Uses Fabric API | 12 | `CompactCtmQuadProcessor.Vertex.readFromQuad(2)`: it uses Fabric's `QuadView` |
| Done another way in Horizon | 2 | `ModMenuApiImpl.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) |
| Uses other libraries | 2 | `SodiumConfigImpl.asSodiumOption(3)`: it uses `BooleanOptionBuilder` (net.caffeinemc.mods.sodium.api.config.structure) |
| Uses lambdas or method references | 1 | `ModelWrappingHandler.init(0)`: a lambda |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:30 | `join` | not translated: it uses BiomeHolderManager.registryManager, which isn't translated (it uses Minecraft's `RegistryAccess`) |
| me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java:57 | `started` | not translated: `this` in a static method |
| me/pepperbell/continuity/client/ContinuityClient.java:57 | `init` | `on_init`, called from the hook, without: the call to `INSTANCE.init(..)`: it uses ProcessingDataKeyRegistryImpl.INSTANCE, which isn't translated (it uses ProcessingDataKeyRegistryImpl, which isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client))); the call to `BiomeHolderManager.init(..)`: it calls BiomeHolderManager.init, which isn't translated; the call to `ModelWrappingHandler.init(..)`: it calls ModelWrappingHandler.init, which isn't translated; the local `resourceLoader`: it uses Fabric's `ResourceLoader`; the call to `resourceLoader.addListenerOrdering(..)`: unknown name `resourceLoader`; the call to `ifPresent(..)`: it uses Fabric's `PackActivationType`.NORMAL; the local `registry`: it uses CtmLoaderRegistry, which isn't translated (it's an interface); the local `loader`: it uses CtmLoader, which isn't translated (it's an interface); an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader`; an assignment: unknown name `loader` |

## Every class


### `CachingPredicates`: it's an interface

- [ ] `CachingPredicates.affectsSprites(0)`: it has no body (me/pepperbell/continuity/api/client/CachingPredicates.java:9)
- [ ] `CachingPredicates.affectsSprite(1)`: it has no body (me/pepperbell/continuity/api/client/CachingPredicates.java:11)
- [ ] `CachingPredicates.affectsBlockStates(0)`: it has no body (me/pepperbell/continuity/api/client/CachingPredicates.java:13)
- [ ] `CachingPredicates.affectsBlockState(1)`: it has no body (me/pepperbell/continuity/api/client/CachingPredicates.java:15)
- [ ] `CachingPredicates.isValidForMultipass(0)`: it has no body (me/pepperbell/continuity/api/client/CachingPredicates.java:17)

### `CachingPredicates.Factory`: it's an interface

- [ ] `CachingPredicates.Factory.createPredicates(2)`: it has no body (me/pepperbell/continuity/api/client/CachingPredicates.java:20)

### `ContinuityFeatureStates`: it's an interface

- [ ] `ContinuityFeatureStates.get(0)`: it uses ContinuityFeatureStates, which isn't translated (it's an interface) (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java:8)
- [ ] `ContinuityFeatureStates.getConnectedTexturesState(0)`: it has no body (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java:12)
- [ ] `ContinuityFeatureStates.getEmissiveTexturesState(0)`: it has no body (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java:14)

### `ContinuityFeatureStates.FeatureState`: it's an interface

- [ ] `ContinuityFeatureStates.FeatureState.isEnabled(0)`: it has no body (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java:18)
- [ ] `ContinuityFeatureStates.FeatureState.enable(0)`: it has no body (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java:20)
- [ ] `ContinuityFeatureStates.FeatureState.disable(0)`: it has no body (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java:22)

### `CtmLoader`: it's an interface

- [ ] `CtmLoader.getPropertiesFactory(0)`: it has no body (me/pepperbell/continuity/api/client/CtmLoader.java:4)
- [ ] `CtmLoader.getProcessorFactory(0)`: it has no body (me/pepperbell/continuity/api/client/CtmLoader.java:6)
- [ ] `CtmLoader.getPredicatesFactory(0)`: it has no body (me/pepperbell/continuity/api/client/CtmLoader.java:8)

### `CtmLoaderRegistry`: it's an interface

- [ ] `CtmLoaderRegistry.get(0)`: it uses CtmLoaderRegistry, which isn't translated (it's an interface) (me/pepperbell/continuity/api/client/CtmLoaderRegistry.java:9)
- [ ] `CtmLoaderRegistry.registerLoader(2)`: it has no body (me/pepperbell/continuity/api/client/CtmLoaderRegistry.java:13)
- [ ] `CtmLoaderRegistry.getLoader(1)`: it has no body (me/pepperbell/continuity/api/client/CtmLoaderRegistry.java:15)

### `CtmProperties`: it's an interface

- [ ] `CtmProperties.getSpriteDependencies(0)`: it has no body (me/pepperbell/continuity/api/client/CtmProperties.java:11)

### `CtmProperties.Factory`: it's an interface

- [ ] `CtmProperties.Factory.createProperties(6)`: it has no body (me/pepperbell/continuity/api/client/CtmProperties.java:14)

### `EmissiveSpriteApi`: it's an interface

- [ ] `EmissiveSpriteApi.get(0)`: it uses EmissiveSpriteApi, which isn't translated (it's an interface) (me/pepperbell/continuity/api/client/EmissiveSpriteApi.java:10)
- [ ] `EmissiveSpriteApi.getEmissiveSprite(1)`: it has no body (me/pepperbell/continuity/api/client/EmissiveSpriteApi.java:14)

### `ProcessingDataKey`: it's an interface

- [ ] `ProcessingDataKey.getId(0)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataKey.java:11)
- [ ] `ProcessingDataKey.getRawId(0)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataKey.java:13)
- [ ] `ProcessingDataKey.getValueSupplier(0)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataKey.java:15)
- [ ] `ProcessingDataKey.getValueResetAction(0)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataKey.java:17)

### `ProcessingDataKeyRegistry`: it's an interface

- [ ] `ProcessingDataKeyRegistry.get(0)`: it uses ProcessingDataKeyRegistry, which isn't translated (it's an interface) (me/pepperbell/continuity/api/client/ProcessingDataKeyRegistry.java:12)
- [ ] `ProcessingDataKeyRegistry.registerKey(2)`: it's generic (me/pepperbell/continuity/api/client/ProcessingDataKeyRegistry.java:16)
- [ ] `ProcessingDataKeyRegistry.registerKey(3)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataKeyRegistry.java:20)
- [ ] `ProcessingDataKeyRegistry.getKey(1)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataKeyRegistry.java:22)
- [ ] `ProcessingDataKeyRegistry.getRegisteredAmount(0)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataKeyRegistry.java:25)

### `ProcessingDataProvider`: it's an interface

- [ ] `ProcessingDataProvider.getData(1)`: it has no body (me/pepperbell/continuity/api/client/ProcessingDataProvider.java:4)

### `QuadProcessor`: it's an interface

- [ ] `QuadProcessor.processQuad(9)`: it has no body (me/pepperbell/continuity/api/client/QuadProcessor.java:14)

### `QuadProcessor.Factory`: it's an interface

- [ ] `QuadProcessor.Factory.createProcessor(2)`: it has no body (me/pepperbell/continuity/api/client/QuadProcessor.java:27)

### `QuadProcessor.ProcessingContext`: it's an interface

- [ ] `QuadProcessor.ProcessingContext.getExtraQuadEmitter(0)`: it has no body (me/pepperbell/continuity/api/client/QuadProcessor.java:31)

### `QuadProcessor.ProcessingResult`


### `ContinuityConfig`: it uses `File` (java.io)

- [x] `ContinuityConfig.LOGGER` (me/pepperbell/continuity/client/config/ContinuityConfig.java:23)
- [x] `ContinuityConfig.GSON` (me/pepperbell/continuity/client/config/ContinuityConfig.java:24)
- [ ] `ContinuityConfig.INSTANCE`: it uses ContinuityConfig, which isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:25)
- [ ] `ContinuityConfig.ContinuityConfig(1)`: its class isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:32)
- [ ] `ContinuityConfig.load(0)`: its class isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:36)
- [ ] `ContinuityConfig.save(0)`: its class isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:48)
- [ ] `ContinuityConfig.fromJson(1)`: its class isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:56)
- [ ] `ContinuityConfig.toJson(0)`: its class isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:77)
- [ ] `ContinuityConfig.addOption(1)`: it's generic (me/pepperbell/continuity/client/config/ContinuityConfig.java:89)
- [ ] `ContinuityConfig.getOption(1)`: its class isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:98)
- [ ] `ContinuityConfig.getOptionMapView(0)`: its class isn't translated (it uses `File` (java.io)) (me/pepperbell/continuity/client/config/ContinuityConfig.java:103)

### `ContinuityConfigScreen`: it extends Minecraft's `Screen`

- [ ] `ContinuityConfigScreen.ContinuityConfigScreen(2)`: its class isn't translated (it extends Minecraft's `Screen`) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:21)
- [ ] `ContinuityConfigScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:27)
- [ ] `ContinuityConfigScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:46)
- [ ] `ContinuityConfigScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:51)
- [ ] `ContinuityConfigScreen.saveValues(0)`: its class isn't translated (it extends Minecraft's `Screen`) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:55)
- [x] `ContinuityConfigScreen.getTranslationKey(1)` (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:72)
- [x] `ContinuityConfigScreen.getTooltipKey(1)` (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:76)
- [ ] `ContinuityConfigScreen.startBooleanValueButton(1)`: its class isn't translated (it extends Minecraft's `Screen`) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:80)

### `ContinuityConfigScreen.Value`: it's generic

- [ ] `ContinuityConfigScreen.Value.Value(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:102)
- [ ] `ContinuityConfigScreen.Value.of(2)`: it's generic (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:109)
- [ ] `ContinuityConfigScreen.Value.getOption(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:115)
- [ ] `ContinuityConfigScreen.Value.getFlags(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:119)
- [ ] `ContinuityConfigScreen.Value.get(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:123)
- [ ] `ContinuityConfigScreen.Value.set(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:127)
- [ ] `ContinuityConfigScreen.Value.isChanged(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:131)
- [ ] `ContinuityConfigScreen.Value.saveToOption(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:135)

### `ContinuityConfigScreen.Value.Flag`: its constants carry values

- [ ] `ContinuityConfigScreen.Value.Flag.onSave(0)`: it has no body (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java:147)

### `ModMenuApiImpl`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `ModMenuApiImpl.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (me/pepperbell/continuity/client/config/ModMenuApiImpl.java:7)

### `Option`: it's an interface

- [ ] `Option.getKey(0)`: it has no body (me/pepperbell/continuity/client/config/Option.java:8)
- [ ] `Option.getDefault(0)`: it has no body (me/pepperbell/continuity/client/config/Option.java:10)
- [ ] `Option.get(0)`: it has no body (me/pepperbell/continuity/client/config/Option.java:12)
- [ ] `Option.set(1)`: it has no body (me/pepperbell/continuity/client/config/Option.java:14)
- [ ] `Option.toJson(0)`: it has no body (me/pepperbell/continuity/client/config/Option.java:16)
- [ ] `Option.fromJson(1)`: it has no body (me/pepperbell/continuity/client/config/Option.java:18)

### `Option.BaseOption`: it's generic

- [ ] `Option.BaseOption.BaseOption(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/Option.java:25)
- [ ] `Option.BaseOption.getKey(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/Option.java:31)
- [ ] `Option.BaseOption.getDefault(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/Option.java:36)
- [ ] `Option.BaseOption.get(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/Option.java:41)
- [ ] `Option.BaseOption.set(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/Option.java:46)

### `Option.BooleanOption`: it extends `BaseOption`

- [ ] `Option.BooleanOption.BooleanOption(2)`: its class isn't translated (it extends `BaseOption`) (me/pepperbell/continuity/client/config/Option.java:53)
- [ ] `Option.BooleanOption.toJson(0)`: its class isn't translated (it extends `BaseOption`) (me/pepperbell/continuity/client/config/Option.java:57)
- [ ] `Option.BooleanOption.fromJson(1)`: its class isn't translated (it extends `BaseOption`) (me/pepperbell/continuity/client/config/Option.java:62)

### `SodiumConfigImpl`: it implements `ConfigEntryPoint` (net.caffeinemc.mods.sodium.api.config)

- [ ] `SodiumConfigImpl.registerConfigLate(1)`: its class isn't translated (it implements `ConfigEntryPoint` (net.caffeinemc.mods.sodium.api.config)) (me/pepperbell/continuity/client/config/SodiumConfigImpl.java:12)
- [ ] `SodiumConfigImpl.asSodiumOption(3)`: it uses `BooleanOptionBuilder` (net.caffeinemc.mods.sodium.api.config.structure) (me/pepperbell/continuity/client/config/SodiumConfigImpl.java:24)

### `SodiumConfigImpl.OptionBindingImpl`: it's generic

- [ ] `SodiumConfigImpl.OptionBindingImpl.OptionBindingImpl(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/SodiumConfigImpl.java:39)
- [ ] `SodiumConfigImpl.OptionBindingImpl.save(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/SodiumConfigImpl.java:43)
- [ ] `SodiumConfigImpl.OptionBindingImpl.load(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/config/SodiumConfigImpl.java:47)

### `ContinuityClient`: it implements Fabric's `ClientModInitializer`

- [x] `ContinuityClient.ID` (me/pepperbell/continuity/client/ContinuityClient.java:53)
- [x] `ContinuityClient.NAME` (me/pepperbell/continuity/client/ContinuityClient.java:54)
- [x] `ContinuityClient.LOGGER` (me/pepperbell/continuity/client/ContinuityClient.java:55)
- [ ] `ContinuityClient.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook (me/pepperbell/continuity/client/ContinuityClient.java:57)
- [ ] `ContinuityClient.createLoader(3)`: its body couldn't be read: an anonymous class (me/pepperbell/continuity/client/ContinuityClient.java:168)
- [ ] `ContinuityClient.createLoader(4)`: it's generic (me/pepperbell/continuity/client/ContinuityClient.java:189)
- [ ] `ContinuityClient.createLoader(3)`: its body couldn't be read: an anonymous class (me/pepperbell/continuity/client/ContinuityClient.java:199)
- [ ] `ContinuityClient.createLoader(3)`: its body couldn't be read: an anonymous class (me/pepperbell/continuity/client/ContinuityClient.java:205)
- [ ] `ContinuityClient.createLoader(2)`: it's generic (me/pepperbell/continuity/client/ContinuityClient.java:215)
- [ ] `ContinuityClient.createCustomLoader(4)`: it's generic (me/pepperbell/continuity/client/ContinuityClient.java:219)
- [ ] `ContinuityClient.createCustomLoader(3)`: it's generic (me/pepperbell/continuity/client/ContinuityClient.java:229)
- [ ] `ContinuityClient.wrapWithOptifineOnlyCheck(1)`: it's generic (me/pepperbell/continuity/client/ContinuityClient.java:235)
- [x] `ContinuityClient.asId(1)` (me/pepperbell/continuity/client/ContinuityClient.java:241)

### `TextureAtlasSpriteExtension`: it's an interface

- [ ] `TextureAtlasSpriteExtension.continuity$getEmissiveSprite(0)`: it has no body (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:7)
- [ ] `TextureAtlasSpriteExtension.continuity$setEmissiveSprite(1)`: it has no body (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:10)
- [ ] `AtlasManagerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:25)
- [ ] `CuboidItemModelWrapperMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:32)
- [ ] `CuboidItemModelWrapperUnbakedMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:18)
- [ ] `MultiPackResourceManagerMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:17)
- [ ] `SpriteLoaderMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:31)
- [ ] `SpriteSourceListMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:32)
- [ ] `StateHolderAccessor`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:10)
- [ ] `TextureAtlasSpriteMixin`: a Mixin class: its handlers are the injections, in the bundle's helpers (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java:10)

### `CtmBlockStateModel`: it extends Fabric's `WrapperBlockStateModel`

- [x] `CtmBlockStateModel.PASSES` (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:23)
- [ ] `CtmBlockStateModel.CtmBlockStateModel(2)`: its class isn't translated (it extends Fabric's `WrapperBlockStateModel`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:27)
- [ ] `CtmBlockStateModel.emitQuads(6)`: its class isn't translated (it extends Fabric's `WrapperBlockStateModel`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:32)
- [ ] `CtmBlockStateModel.createGeometryKey(4)`: its class isn't translated (it extends Fabric's `WrapperBlockStateModel`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:58)
- [ ] `CtmBlockStateModel.getSliceFunc(1)`: its class isn't translated (it extends Fabric's `WrapperBlockStateModel`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:73)

### `CtmBlockStateModel.CtmQuadTransform`: it implements Fabric's `QuadTransform`

- [ ] `CtmBlockStateModel.CtmQuadTransform.transform(1)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:104)
- [ ] `CtmBlockStateModel.CtmQuadTransform.transformOnce(2)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:119)
- [ ] `CtmBlockStateModel.CtmQuadTransform.isActive(0)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:147)
- [ ] `CtmBlockStateModel.CtmQuadTransform.prepare(7)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:151)
- [ ] `CtmBlockStateModel.CtmQuadTransform.reset(0)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/CtmBlockStateModel.java:170)

### `EmissiveBlockStateModel`: it extends Fabric's `WrapperBlockStateModel`

- [ ] `EmissiveBlockStateModel.EmissiveBlockStateModel(1)`: its class isn't translated (it extends Fabric's `WrapperBlockStateModel`) (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java:25)
- [ ] `EmissiveBlockStateModel.emitQuads(6)`: its class isn't translated (it extends Fabric's `WrapperBlockStateModel`) (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java:29)
- [ ] `EmissiveBlockStateModel.createGeometryKey(4)`: its body couldn't be read: a class declared inside a method (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java:56)

### `EmissiveBlockStateModel.EmissiveQuadTransform`: it implements Fabric's `QuadTransform`

- [ ] `EmissiveBlockStateModel.EmissiveQuadTransform.transform(1)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java:86)
- [ ] `EmissiveBlockStateModel.EmissiveQuadTransform.isActive(0)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java:108)
- [ ] `EmissiveBlockStateModel.EmissiveQuadTransform.prepare(3)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java:112)
- [ ] `EmissiveBlockStateModel.EmissiveQuadTransform.reset(0)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java:119)

### `EmissiveItemGeometry`: it uses Minecraft's `ItemQuads`

- [ ] `EmissiveItemGeometry.EMISSIVE_GEOMETRY_MARKER`: it uses `Object` (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:28)
- [ ] `EmissiveItemGeometry.EmissiveItemGeometry(3)`: its class isn't translated (it uses Minecraft's `ItemQuads`) (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:27)
- [ ] `EmissiveItemGeometry.of(3)`: it uses EmissiveItemGeometry, which isn't translated (it uses Minecraft's `ItemQuads`) (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:30)
- [ ] `EmissiveItemGeometry.of(1)`: it uses EmissiveItemGeometry, which isn't translated (it uses Minecraft's `ItemQuads`) (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:39)
- [ ] `EmissiveItemGeometry.combine(2)`: it uses Minecraft's `BakedQuad` (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:126)
- [ ] `EmissiveItemGeometry.of(2)`: its body couldn't be read: `var` (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:135)
- [ ] `EmissiveItemGeometry.baseAndEmissiveItemQuads(0)`: its class isn't translated (it uses Minecraft's `ItemQuads`) (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:27)
- [ ] `EmissiveItemGeometry.emissiveMesh(0)`: its class isn't translated (it uses Minecraft's `ItemQuads`) (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:27)
- [ ] `EmissiveItemGeometry.emissiveAnimated(0)`: its class isn't translated (it uses Minecraft's `ItemQuads`) (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java:27)

### `ModelObjectsContainer`: it uses CtmQuadTransform, which isn't translated (it implements Fabric's `QuadTransform`)

- [ ] `ModelObjectsContainer.THREAD_LOCAL`: it uses `ThreadLocal` (me/pepperbell/continuity/client/model/ModelObjectsContainer.java:8)
- [ ] `ModelObjectsContainer.get(0)`: it uses ModelObjectsContainer, which isn't translated (it uses CtmQuadTransform, which isn't translated (it implements Fabric's `QuadTransform`)) (me/pepperbell/continuity/client/model/ModelObjectsContainer.java:14)

### `QuadProcessors`

- [ ] `QuadProcessors.processorHolders`: it uses ProcessorHolder, which isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:14)
- [ ] `QuadProcessors.CACHE`: it uses BlockStateKeyCache, which isn't translated (it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects)) (me/pepperbell/continuity/client/model/QuadProcessors.java:15)
- [ ] `QuadProcessors.getCache(1)`: it uses Minecraft's `TextureAtlasSprite` (me/pepperbell/continuity/client/model/QuadProcessors.java:17)
- [ ] `QuadProcessors.computeSlice(2)`: it uses Slice, which isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:21)
- [ ] `QuadProcessors.reload(1)`: it uses ProcessorHolder, which isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:47)

### `QuadProcessors.BlockStateKeyCache`: it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects)

- [ ] `QuadProcessors.BlockStateKeyCache.apply(1)`: its class isn't translated (it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects)) (me/pepperbell/continuity/client/model/QuadProcessors.java:56)
- [ ] `QuadProcessors.BlockStateKeyCache.clear(0)`: its class isn't translated (it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects)) (me/pepperbell/continuity/client/model/QuadProcessors.java:94)

### `QuadProcessors.ProcessorHolder`: it uses QuadProcessor, which isn't translated (it's an interface)

- [ ] `QuadProcessors.ProcessorHolder.ProcessorHolder(2)`: its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:105)
- [ ] `QuadProcessors.ProcessorHolder.processor(0)`: its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:105)
- [ ] `QuadProcessors.ProcessorHolder.predicates(0)`: its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:105)

### `QuadProcessors.Slice`: it uses QuadProcessor, which isn't translated (it's an interface)

- [ ] `QuadProcessors.Slice.Slice(2)`: its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:108)
- [ ] `QuadProcessors.Slice.processors(0)`: its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:108)
- [ ] `QuadProcessors.Slice.multipassProcessors(0)`: its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)) (me/pepperbell/continuity/client/model/QuadProcessors.java:108)

### `QuadProcessors.SpriteKeyCache`: it implements `Function` (java.util.function)

- [ ] `QuadProcessors.SpriteKeyCache.SpriteKeyCache(1)`: its class isn't translated (it implements `Function` (java.util.function)) (me/pepperbell/continuity/client/model/QuadProcessors.java:116)
- [ ] `QuadProcessors.SpriteKeyCache.apply(1)`: its class isn't translated (it implements `Function` (java.util.function)) (me/pepperbell/continuity/client/model/QuadProcessors.java:120)
- [ ] `QuadProcessors.SpriteKeyCache.clear(0)`: its class isn't translated (it implements `Function` (java.util.function)) (me/pepperbell/continuity/client/model/QuadProcessors.java:158)

### `AbstractQuadProcessor`: it implements `QuadProcessor` (me.pepperbell.continuity.api.client)

- [ ] `AbstractQuadProcessor.AbstractQuadProcessor(2)`: its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/AbstractQuadProcessor.java:15)
- [ ] `AbstractQuadProcessor.processQuad(9)`: its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/AbstractQuadProcessor.java:20)
- [ ] `AbstractQuadProcessor.processQuadInner(9)`: it has no body (me/pepperbell/continuity/client/processor/AbstractQuadProcessor.java:37)

### `AbstractQuadProcessorFactory`: it's generic

- [ ] `AbstractQuadProcessorFactory.createProcessor(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/AbstractQuadProcessorFactory.java:13)
- [ ] `AbstractQuadProcessorFactory.createProcessor(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/AbstractQuadProcessorFactory.java:78)
- [ ] `AbstractQuadProcessorFactory.getSpriteAmount(1)`: it has no body (me/pepperbell/continuity/client/processor/AbstractQuadProcessorFactory.java:80)
- [ ] `AbstractQuadProcessorFactory.supportsNullSprites(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/AbstractQuadProcessorFactory.java:82)

### `BaseCachingPredicates`: it implements `CachingPredicates` (me.pepperbell.continuity.api.client)

- [ ] `BaseCachingPredicates.BaseCachingPredicates(3)`: its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:20)
- [ ] `BaseCachingPredicates.affectsSprites(0)`: its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:26)
- [ ] `BaseCachingPredicates.affectsSprite(1)`: its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:31)
- [ ] `BaseCachingPredicates.affectsBlockStates(0)`: its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:36)
- [ ] `BaseCachingPredicates.affectsBlockState(1)`: its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:41)
- [ ] `BaseCachingPredicates.isValidForMultipass(0)`: its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:46)

### `BaseCachingPredicates.Factory`: it's generic

- [ ] `BaseCachingPredicates.Factory.Factory(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:54)
- [ ] `BaseCachingPredicates.Factory.createPredicates(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java:58)

### `BaseProcessingPredicate`: it implements `ProcessingPredicate`

- [ ] `BaseProcessingPredicate.BaseProcessingPredicate(4)`: its class isn't translated (it implements `ProcessingPredicate`) (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java:30)
- [ ] `BaseProcessingPredicate.shouldProcessQuad(7)`: its class isn't translated (it implements `ProcessingPredicate`) (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java:42)
- [ ] `BaseProcessingPredicate.fromProperties(1)`: it uses BaseProcessingPredicate, which isn't translated (it implements `ProcessingPredicate`) (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java:89)

### `BaseProcessingPredicate.BiomeCache`: it uses Minecraft's `Biome`

- [ ] `BaseProcessingPredicate.BiomeCache.get(2)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java:100)
- [ ] `BaseProcessingPredicate.BiomeCache.reset(0)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java:110)

### `BaseProcessingPredicate.BlockEntityNameCache`

- [ ] `BaseProcessingPredicate.BlockEntityNameCache.get(2)`: it uses Minecraft's `BlockAndTintGetter` (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java:120)
- [x] `BaseProcessingPredicate.BlockEntityNameCache.reset(0)` (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java:139)

### `CompactCtmQuadProcessor`: it extends `AbstractQuadProcessor`

- [x] `CompactCtmQuadProcessor.QUADRANT_INDEX_MAPS` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:34)
- [ ] `CompactCtmQuadProcessor.CompactCtmQuadProcessor(6)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:41)
- [ ] `CompactCtmQuadProcessor.processQuadInner(9)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:56)
- [x] `CompactCtmQuadProcessor.shouldSplitUV(2)` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:409)
- [ ] `CompactCtmQuadProcessor.getSpriteIndex(2)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:413)
- [ ] `CompactCtmQuadProcessor.tryInterpolate(3)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:427)
- [ ] `CompactCtmQuadProcessor.splitHalf(6)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:434)
- [ ] `CompactCtmQuadProcessor.splitQuadrant(6)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:445)

### `CompactCtmQuadProcessor.Factory`: it implements `Factory`

- [ ] `CompactCtmQuadProcessor.Factory.createProcessor(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:477)
- [ ] `CompactCtmQuadProcessor.Factory.createProcessor(3)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:594)
- [ ] `CompactCtmQuadProcessor.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:607)
- [ ] `CompactCtmQuadProcessor.Factory.getReplacementSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:611)
- [ ] `CompactCtmQuadProcessor.Factory.supportsNullSprites(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:615)

### `CompactCtmQuadProcessor.Vertex`

- [ ] `CompactCtmQuadProcessor.Vertex.readFromQuad(2)`: it uses Fabric's `QuadView` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:633)
- [ ] `CompactCtmQuadProcessor.Vertex.writeToQuad(2)`: it uses Fabric's `MutableQuadView` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:649)
- [x] `CompactCtmQuadProcessor.Vertex.set(1)` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:659)
- [x] `CompactCtmQuadProcessor.Vertex.setLerped(3)` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:675)

### `CompactCtmQuadProcessor.VertexContainer`

- [ ] `CompactCtmQuadProcessor.VertexContainer.fillBaseVertices(1)`: it uses Fabric's `QuadView` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java:712)

### `ConnectionPredicate`: it's an interface

- [ ] `ConnectionPredicate.shouldConnect(9)`: it has no body (me/pepperbell/continuity/client/processor/ConnectionPredicate.java:11)
- [ ] `ConnectionPredicate.shouldConnect(7)`: its class isn't translated (it's an interface) (me/pepperbell/continuity/client/processor/ConnectionPredicate.java:23)
- [ ] `ConnectionPredicate.shouldConnect(8)`: its class isn't translated (it's an interface) (me/pepperbell/continuity/client/processor/ConnectionPredicate.java:31)

### `DirectionMaps`

- [ ] `DirectionMaps.DIRECTION_MAPS`: it uses Minecraft's `Direction` (me/pepperbell/continuity/client/processor/DirectionMaps.java:10)
- [ ] `DirectionMaps.getMap(1)`: it uses Minecraft's `Direction` (me/pepperbell/continuity/client/processor/DirectionMaps.java:12)
- [ ] `DirectionMaps.getDirections(3)`: it uses Minecraft's `Direction` (me/pepperbell/continuity/client/processor/DirectionMaps.java:16)

### `OrientationMode`

- [x] `OrientationMode.AXIS_ORIENTATIONS` (me/pepperbell/continuity/client/processor/OrientationMode.java:14)
- [ ] `OrientationMode.getOrientation(2)`: it uses Fabric's `QuadView` (me/pepperbell/continuity/client/processor/OrientationMode.java:16)

### `OverlayProcessingPredicate`: it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor)

- [ ] `OverlayProcessingPredicate.OverlayProcessingPredicate(4)`: its class isn't translated (it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/OverlayProcessingPredicate.java:20)
- [ ] `OverlayProcessingPredicate.shouldProcessQuad(7)`: its class isn't translated (it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/OverlayProcessingPredicate.java:29)
- [ ] `OverlayProcessingPredicate.fromProperties(1)`: it uses OverlayProcessingPredicate, which isn't translated (it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/OverlayProcessingPredicate.java:42)

### `SimpleOverlayQuadProcessor`: it extends `SimpleQuadProcessor` (me.pepperbell.continuity.client.processor.simple)

- [ ] `SimpleOverlayQuadProcessor.SimpleOverlayQuadProcessor(5)`: its class isn't translated (it extends `SimpleQuadProcessor` (me.pepperbell.continuity.client.processor.simple)) (me/pepperbell/continuity/client/processor/overlay/SimpleOverlayQuadProcessor.java:28)
- [ ] `SimpleOverlayQuadProcessor.processQuad(9)`: its class isn't translated (it extends `SimpleQuadProcessor` (me.pepperbell.continuity.client.processor.simple)) (me/pepperbell/continuity/client/processor/overlay/SimpleOverlayQuadProcessor.java:38)

### `SimpleOverlayQuadProcessor.Factory`: it's generic

- [ ] `SimpleOverlayQuadProcessor.Factory.Factory(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/overlay/SimpleOverlayQuadProcessor.java:68)
- [ ] `SimpleOverlayQuadProcessor.Factory.createProcessor(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/overlay/SimpleOverlayQuadProcessor.java:72)
- [ ] `SimpleOverlayQuadProcessor.Factory.supportsNullSprites(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/overlay/SimpleOverlayQuadProcessor.java:84)

### `StandardOverlayQuadProcessor`: it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)

- [ ] `StandardOverlayQuadProcessor.StandardOverlayQuadProcessor(10)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:48)
- [ ] `StandardOverlayQuadProcessor.processQuadInner(9)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:79)
- [ ] `StandardOverlayQuadProcessor.matchesAny(2)`: it uses Minecraft's `TextureAtlasSprite` (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:109)
- [ ] `StandardOverlayQuadProcessor.appliesOverlay(9)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:119)
- [ ] `StandardOverlayQuadProcessor.hasSameOverlay(2)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:141)
- [ ] `StandardOverlayQuadProcessor.appliesOverlayCorner(9)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:151)
- [ ] `StandardOverlayQuadProcessor.fromTwoSidesAdj(14)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:173)
- [ ] `StandardOverlayQuadProcessor.fromOneSide(17)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:198)
- [ ] `StandardOverlayQuadProcessor.getCollector(1)`: it uses SpriteCollector, which isn't translated (it uses Minecraft's `TextureAtlasSprite`) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:239)
- [ ] `StandardOverlayQuadProcessor.prepareCollector(2)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:243)
- [ ] `StandardOverlayQuadProcessor.prepareCollector(3)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:248)
- [ ] `StandardOverlayQuadProcessor.getSprites(8)`: its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:254)

### `StandardOverlayQuadProcessor.Factory`: it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor)

- [ ] `StandardOverlayQuadProcessor.Factory.createProcessor(2)`: its class isn't translated (it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:507)
- [ ] `StandardOverlayQuadProcessor.Factory.getSpriteAmount(1)`: its class isn't translated (it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:523)
- [ ] `StandardOverlayQuadProcessor.Factory.supportsNullSprites(1)`: its class isn't translated (it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor)) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:527)

### `StandardOverlayQuadProcessor.SpriteCollector`: it uses Minecraft's `TextureAtlasSprite`

- [ ] `StandardOverlayQuadProcessor.SpriteCollector.EMPTY_SPRITES`: it uses Minecraft's `TextureAtlasSprite` (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:533)
- [ ] `StandardOverlayQuadProcessor.SpriteCollector.add(1)`: its class isn't translated (it uses Minecraft's `TextureAtlasSprite`) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:537)
- [ ] `StandardOverlayQuadProcessor.SpriteCollector.clear(0)`: its class isn't translated (it uses Minecraft's `TextureAtlasSprite`) (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java:543)

### `ProcessingDataKeys`

- [ ] `ProcessingDataKeys.MUTABLE_POS`: it uses ProcessingDataKey, which isn't translated (it's an interface) (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:12)
- [ ] `ProcessingDataKeys.BIOME_CACHE`: it uses ProcessingDataKey, which isn't translated (it's an interface) (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:13)
- [ ] `ProcessingDataKeys.BLOCK_ENTITY_NAME_CACHE`: it uses ProcessingDataKey, which isn't translated (it's an interface) (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:16)
- [ ] `ProcessingDataKeys.VERTEX_CONTAINER`: it uses ProcessingDataKey, which isn't translated (it's an interface) (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:19)
- [ ] `ProcessingDataKeys.SPRITE_COLLECTOR`: it uses ProcessingDataKey, which isn't translated (it's an interface) (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:22)
- [ ] `ProcessingDataKeys.create(2)`: it's generic (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:26)
- [ ] `ProcessingDataKeys.create(3)`: it's generic (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:30)
- [x] `ProcessingDataKeys.init(0)` (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java:34)

### `ProcessingPredicate`: it's an interface

- [ ] `ProcessingPredicate.shouldProcessQuad(7)`: it has no body (me/pepperbell/continuity/client/processor/ProcessingPredicate.java:11)

### `CtmSpriteProvider`: it implements `SpriteProvider`

- [x] `CtmSpriteProvider.SPRITE_INDEX_MAP` (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java:20)
- [ ] `CtmSpriteProvider.CtmSpriteProvider(4)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java:283)
- [ ] `CtmSpriteProvider.getSprite(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java:290)
- [ ] `CtmSpriteProvider.getConnections(10)`: it uses Minecraft's `Direction` (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java:310)

### `CtmSpriteProvider.Factory`: it implements `Factory`

- [ ] `CtmSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java:346)
- [ ] `CtmSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java:350)

### `FixedSpriteProvider`: it implements `SpriteProvider`

- [ ] `FixedSpriteProvider.FixedSpriteProvider(1)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/FixedSpriteProvider.java:16)
- [ ] `FixedSpriteProvider.getSprite(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/FixedSpriteProvider.java:20)

### `FixedSpriteProvider.Factory`: it implements `Factory`

- [ ] `FixedSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/FixedSpriteProvider.java:36)
- [ ] `FixedSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/FixedSpriteProvider.java:41)

### `HorizontalSpriteProvider`: it implements `SpriteProvider`

- [x] `HorizontalSpriteProvider.SPRITE_INDEX_MAP` (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java:20)
- [ ] `HorizontalSpriteProvider.HorizontalSpriteProvider(4)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java:26)
- [ ] `HorizontalSpriteProvider.getSprite(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java:33)
- [ ] `HorizontalSpriteProvider.getConnections(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java:51)

### `HorizontalSpriteProvider.Factory`: it implements `Factory`

- [ ] `HorizontalSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java:74)
- [ ] `HorizontalSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java:78)

### `HorizontalVerticalSpriteProvider`: it extends `HorizontalSpriteProvider`

- [x] `HorizontalVerticalSpriteProvider.SECONDARY_SPRITE_INDEX_MAP` (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java:20)
- [ ] `HorizontalVerticalSpriteProvider.HorizontalVerticalSpriteProvider(4)`: its class isn't translated (it extends `HorizontalSpriteProvider`) (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java:87)
- [ ] `HorizontalVerticalSpriteProvider.getSprite(8)`: its class isn't translated (it extends `HorizontalSpriteProvider`) (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java:93)
- [ ] `HorizontalVerticalSpriteProvider.getSecondaryConnections(8)`: its class isn't translated (it extends `HorizontalSpriteProvider`) (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java:116)

### `HorizontalVerticalSpriteProvider.Factory`: it implements `Factory`

- [ ] `HorizontalVerticalSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java:147)
- [ ] `HorizontalVerticalSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java:151)

### `RandomSpriteProvider`: it implements `SpriteProvider`

- [ ] `RandomSpriteProvider.RandomSpriteProvider(5)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/RandomSpriteProvider.java:27)
- [ ] `RandomSpriteProvider.getSprite(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/RandomSpriteProvider.java:35)

### `RandomSpriteProvider.Factory`: it implements `Factory`

- [ ] `RandomSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/RandomSpriteProvider.java:68)
- [ ] `RandomSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/RandomSpriteProvider.java:80)

### `RepeatSpriteProvider`: it implements `SpriteProvider`

- [ ] `RepeatSpriteProvider.RepeatSpriteProvider(5)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/RepeatSpriteProvider.java:23)
- [ ] `RepeatSpriteProvider.getSprite(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/RepeatSpriteProvider.java:31)

### `RepeatSpriteProvider.Factory`: it implements `Factory`

- [ ] `RepeatSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/RepeatSpriteProvider.java:129)
- [ ] `RepeatSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/RepeatSpriteProvider.java:133)

### `SimpleQuadProcessor`: it implements `QuadProcessor` (me.pepperbell.continuity.api.client)

- [ ] `SimpleQuadProcessor.SimpleQuadProcessor(2)`: its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java:22)
- [ ] `SimpleQuadProcessor.processQuad(9)`: its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java:27)
- [ ] `SimpleQuadProcessor.process(3)`: it uses Fabric's `MutableQuadView` (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java:47)

### `SimpleQuadProcessor.Factory`: it's generic

- [ ] `SimpleQuadProcessor.Factory.Factory(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java:63)
- [ ] `SimpleQuadProcessor.Factory.createProcessor(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java:67)
- [ ] `SimpleQuadProcessor.Factory.getSpriteAmount(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java:74)

### `SpriteProvider`: it's an interface

- [ ] `SpriteProvider.getSprite(8)`: it has no body (me/pepperbell/continuity/client/processor/simple/SpriteProvider.java:14)

### `SpriteProvider.Factory`: it's an interface

- [ ] `SpriteProvider.Factory.createSpriteProvider(2)`: it has no body (me/pepperbell/continuity/client/processor/simple/SpriteProvider.java:27)
- [ ] `SpriteProvider.Factory.getSpriteAmount(1)`: it has no body (me/pepperbell/continuity/client/processor/simple/SpriteProvider.java:29)

### `VerticalHorizontalSpriteProvider`: it extends `VerticalSpriteProvider`

- [x] `VerticalHorizontalSpriteProvider.SECONDARY_SPRITE_INDEX_MAP` (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java:20)
- [ ] `VerticalHorizontalSpriteProvider.VerticalHorizontalSpriteProvider(4)`: its class isn't translated (it extends `VerticalSpriteProvider`) (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java:87)
- [ ] `VerticalHorizontalSpriteProvider.getSprite(8)`: its class isn't translated (it extends `VerticalSpriteProvider`) (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java:93)
- [ ] `VerticalHorizontalSpriteProvider.getSecondaryConnections(8)`: its class isn't translated (it extends `VerticalSpriteProvider`) (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java:116)

### `VerticalHorizontalSpriteProvider.Factory`: it implements `Factory`

- [ ] `VerticalHorizontalSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java:147)
- [ ] `VerticalHorizontalSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java:151)

### `VerticalSpriteProvider`: it implements `SpriteProvider`

- [x] `VerticalSpriteProvider.SPRITE_INDEX_MAP` (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java:20)
- [ ] `VerticalSpriteProvider.VerticalSpriteProvider(4)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java:26)
- [ ] `VerticalSpriteProvider.getSprite(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java:33)
- [ ] `VerticalSpriteProvider.getConnections(8)`: its class isn't translated (it implements `SpriteProvider`) (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java:51)

### `VerticalSpriteProvider.Factory`: it implements `Factory`

- [ ] `VerticalSpriteProvider.Factory.createSpriteProvider(2)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java:74)
- [ ] `VerticalSpriteProvider.Factory.getSpriteAmount(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java:78)

### `Symmetry`

- [ ] `Symmetry.apply(1)`: it uses Minecraft's `Direction` (me/pepperbell/continuity/client/processor/Symmetry.java:11)

### `TopQuadProcessor`: it extends `AbstractQuadProcessor`

- [ ] `TopQuadProcessor.TopQuadProcessor(4)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/TopQuadProcessor.java:22)
- [ ] `TopQuadProcessor.processQuadInner(9)`: its class isn't translated (it extends `AbstractQuadProcessor`) (me/pepperbell/continuity/client/processor/TopQuadProcessor.java:28)

### `TopQuadProcessor.Factory`: it extends `AbstractQuadProcessorFactory`

- [ ] `TopQuadProcessor.Factory.createProcessor(2)`: its class isn't translated (it extends `AbstractQuadProcessorFactory`) (me/pepperbell/continuity/client/processor/TopQuadProcessor.java:60)
- [ ] `TopQuadProcessor.Factory.getSpriteAmount(1)`: its class isn't translated (it extends `AbstractQuadProcessorFactory`) (me/pepperbell/continuity/client/processor/TopQuadProcessor.java:66)

### `BaseCtmProperties`: it implements `CtmProperties` (me.pepperbell.continuity.api.client)

- [x] `BaseCtmProperties.SPECIAL_SKIP_ID` (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:39)
- [x] `BaseCtmProperties.SPECIAL_DEFAULT_ID` (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:40)
- [ ] `BaseCtmProperties.DIRECTION_AMOUNT`: it calls Minecraft's `Direction`.values (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:41)
- [ ] `BaseCtmProperties.BaseCtmProperties(6)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:65)
- [ ] `BaseCtmProperties.getSpriteDependencies(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:74)
- [ ] `BaseCtmProperties.compareTo(1)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:78)
- [ ] `BaseCtmProperties.init(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:95)
- [ ] `BaseCtmProperties.parseMatchTiles(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:110)
- [ ] `BaseCtmProperties.parseMatchBlocks(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:117)
- [ ] `BaseCtmProperties.detectMatches(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:124)
- [ ] `BaseCtmProperties.validateMatches(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:138)
- [ ] `BaseCtmProperties.parseTiles(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:145)
- [ ] `BaseCtmProperties.parseFaces(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:314)
- [ ] `BaseCtmProperties.parseBiomes(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:358)
- [ ] `BaseCtmProperties.parseHeights(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:407)
- [ ] `BaseCtmProperties.parseLegacyHeights(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:491)
- [ ] `BaseCtmProperties.parseName(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:539)
- [ ] `BaseCtmProperties.parsePrioritize(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:580)
- [ ] `BaseCtmProperties.parseResourceCondition(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:589)
- [ ] `BaseCtmProperties.isValid(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:678)
- [ ] `BaseCtmProperties.getProperties(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:682)
- [ ] `BaseCtmProperties.getResourceId(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:686)
- [ ] `BaseCtmProperties.getPackId(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:690)
- [ ] `BaseCtmProperties.getPackPriority(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:694)
- [ ] `BaseCtmProperties.getMethod(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:698)
- [ ] `BaseCtmProperties.getMatchTilesSet(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:702)
- [ ] `BaseCtmProperties.getMatchBlocksPredicate(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:707)
- [ ] `BaseCtmProperties.getSpriteIds(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:712)
- [ ] `BaseCtmProperties.getFaces(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:716)
- [ ] `BaseCtmProperties.getBiomePredicate(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:721)
- [ ] `BaseCtmProperties.getHeightPredicate(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:726)
- [ ] `BaseCtmProperties.getBlockEntityNamePredicate(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:731)
- [ ] `BaseCtmProperties.isPrioritized(0)`: its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:736)
- [ ] `BaseCtmProperties.wrapFactory(1)`: it's generic (me/pepperbell/continuity/client/properties/BaseCtmProperties.java:740)

### `BasicConnectingCtmProperties`: it extends `BaseCtmProperties`

- [ ] `BasicConnectingCtmProperties.BasicConnectingCtmProperties(6)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java:20)
- [ ] `BasicConnectingCtmProperties.init(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java:26)
- [ ] `BasicConnectingCtmProperties.parseConnect(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java:34)
- [ ] `BasicConnectingCtmProperties.detectConnect(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java:45)
- [ ] `BasicConnectingCtmProperties.validateConnect(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java:55)
- [ ] `BasicConnectingCtmProperties.getConnectionPredicate(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java:62)

### `BasicConnectingCtmProperties.ConnectionType`: its constants carry values


### `CompactConnectingCtmProperties`: it extends `OrientedConnectingCtmProperties`

- [ ] `CompactConnectingCtmProperties.CompactConnectingCtmProperties(7)`: its class isn't translated (it extends `OrientedConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/CompactConnectingCtmProperties.java:18)
- [ ] `CompactConnectingCtmProperties.CompactConnectingCtmProperties(6)`: its class isn't translated (it extends `OrientedConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/CompactConnectingCtmProperties.java:30)
- [ ] `CompactConnectingCtmProperties.init(0)`: its class isn't translated (it extends `OrientedConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/CompactConnectingCtmProperties.java:36)
- [ ] `CompactConnectingCtmProperties.parseTileReplacements(0)`: its class isn't translated (it extends `OrientedConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/CompactConnectingCtmProperties.java:42)
- [ ] `CompactConnectingCtmProperties.getTileReplacementMap(0)`: its class isn't translated (it extends `OrientedConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/CompactConnectingCtmProperties.java:89)

### `ConnectingCtmProperties`: it extends `BasicConnectingCtmProperties`

- [ ] `ConnectingCtmProperties.ConnectingCtmProperties(6)`: its class isn't translated (it extends `BasicConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/ConnectingCtmProperties.java:11)
- [ ] `ConnectingCtmProperties.init(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/ConnectingCtmProperties.java:17)
- [ ] `ConnectingCtmProperties.parseInnerSeams(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/ConnectingCtmProperties.java:23)
- [ ] `ConnectingCtmProperties.getInnerSeams(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/ConnectingCtmProperties.java:30)

### `OrientedConnectingCtmProperties`: it extends `ConnectingCtmProperties`

- [ ] `OrientedConnectingCtmProperties.OrientedConnectingCtmProperties(7)`: its class isn't translated (it extends `ConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/OrientedConnectingCtmProperties.java:12)
- [ ] `OrientedConnectingCtmProperties.OrientedConnectingCtmProperties(6)`: its class isn't translated (it extends `ConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/OrientedConnectingCtmProperties.java:25)
- [ ] `OrientedConnectingCtmProperties.init(0)`: its class isn't translated (it extends `ConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/OrientedConnectingCtmProperties.java:31)
- [ ] `OrientedConnectingCtmProperties.parseOrient(0)`: its class isn't translated (it extends `ConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/OrientedConnectingCtmProperties.java:37)
- [ ] `OrientedConnectingCtmProperties.getOrientationMode(0)`: its class isn't translated (it extends `ConnectingCtmProperties`) (me/pepperbell/continuity/client/properties/OrientedConnectingCtmProperties.java:44)

### `BaseOverlayCtmProperties`: it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties)

- [ ] `BaseOverlayCtmProperties.BaseOverlayCtmProperties(6)`: its class isn't translated (it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/BaseOverlayCtmProperties.java:12)
- [ ] `BaseOverlayCtmProperties.init(0)`: its class isn't translated (it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/BaseOverlayCtmProperties.java:19)
- [ ] `BaseOverlayCtmProperties.getOverlayPropertiesSection(0)`: its class isn't translated (it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/BaseOverlayCtmProperties.java:25)

### `OrientedConnectingOverlayCtmProperties`: it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)

- [ ] `OrientedConnectingOverlayCtmProperties.OrientedConnectingOverlayCtmProperties(7)`: its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/OrientedConnectingOverlayCtmProperties.java:13)
- [ ] `OrientedConnectingOverlayCtmProperties.OrientedConnectingOverlayCtmProperties(6)`: its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/OrientedConnectingOverlayCtmProperties.java:26)
- [ ] `OrientedConnectingOverlayCtmProperties.init(0)`: its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/OrientedConnectingOverlayCtmProperties.java:32)
- [ ] `OrientedConnectingOverlayCtmProperties.getOverlayPropertiesSection(0)`: its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/OrientedConnectingOverlayCtmProperties.java:38)

### `OverlayPropertiesSection`: it uses Minecraft's `BlockState`

- [ ] `OverlayPropertiesSection.OverlayPropertiesSection(3)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:23)
- [ ] `OverlayPropertiesSection.init(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:29)
- [ ] `OverlayPropertiesSection.parseTintIndex(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:35)
- [ ] `OverlayPropertiesSection.parseTintBlock(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:51)
- [ ] `OverlayPropertiesSection.parseLayer(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:92)
- [ ] `OverlayPropertiesSection.getTintIndex(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:109)
- [ ] `OverlayPropertiesSection.getTintBlock(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:113)
- [ ] `OverlayPropertiesSection.getLayer(0)`: its class isn't translated (it uses Minecraft's `BlockState`) (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:118)

### `OverlayPropertiesSection.Provider`: it's an interface

- [ ] `OverlayPropertiesSection.Provider.getOverlayPropertiesSection(0)`: it has no body (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java:123)

### `RandomOverlayCtmProperties`: it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties)

- [ ] `RandomOverlayCtmProperties.RandomOverlayCtmProperties(6)`: its class isn't translated (it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/RandomOverlayCtmProperties.java:12)
- [ ] `RandomOverlayCtmProperties.init(0)`: its class isn't translated (it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/RandomOverlayCtmProperties.java:19)
- [ ] `RandomOverlayCtmProperties.getOverlayPropertiesSection(0)`: its class isn't translated (it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/RandomOverlayCtmProperties.java:25)

### `RepeatOverlayCtmProperties`: it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties)

- [ ] `RepeatOverlayCtmProperties.RepeatOverlayCtmProperties(6)`: its class isn't translated (it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/RepeatOverlayCtmProperties.java:12)
- [ ] `RepeatOverlayCtmProperties.init(0)`: its class isn't translated (it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/RepeatOverlayCtmProperties.java:19)
- [ ] `RepeatOverlayCtmProperties.getOverlayPropertiesSection(0)`: its class isn't translated (it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/RepeatOverlayCtmProperties.java:25)

### `StandardOverlayCtmProperties`: it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)

- [ ] `StandardOverlayCtmProperties.StandardOverlayCtmProperties(6)`: its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java:21)
- [ ] `StandardOverlayCtmProperties.init(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java:28)
- [ ] `StandardOverlayCtmProperties.getOverlayPropertiesSection(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java:36)
- [ ] `StandardOverlayCtmProperties.parseConnectTiles(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java:41)
- [ ] `StandardOverlayCtmProperties.parseConnectBlocks(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java:45)
- [ ] `StandardOverlayCtmProperties.getConnectTilesSet(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java:49)
- [ ] `StandardOverlayCtmProperties.getConnectBlocksPredicate(0)`: its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)) (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java:54)

### `PropertiesParsingHelper`

- [ ] `PropertiesParsingHelper.EMPTY_BLOCK_STATE_PREDICATE`: it uses Minecraft's `BlockState` (me/pepperbell/continuity/client/properties/PropertiesParsingHelper.java:29)
- [ ] `PropertiesParsingHelper.parseMatchTiles(4)`: it calls `FilenameUtils` (org.apache.commons.io).getPath (me/pepperbell/continuity/client/properties/PropertiesParsingHelper.java:31)
- [ ] `PropertiesParsingHelper.parseBlockStates(4)`: its body couldn't be read: a labelled statement (me/pepperbell/continuity/client/properties/PropertiesParsingHelper.java:134)
- [x] `PropertiesParsingHelper.parseSymmetry(4)` (me/pepperbell/continuity/client/properties/PropertiesParsingHelper.java:397)
- [x] `PropertiesParsingHelper.parseOrientationMode(4)` (me/pepperbell/continuity/client/properties/PropertiesParsingHelper.java:412)
- [x] `PropertiesParsingHelper.parseOptifineOnly(2)` (me/pepperbell/continuity/client/properties/PropertiesParsingHelper.java:428)

### `RandomCtmProperties`: it extends `BaseCtmProperties`

- [ ] `RandomCtmProperties.RandomCtmProperties(6)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:19)
- [ ] `RandomCtmProperties.init(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:25)
- [ ] `RandomCtmProperties.parseWeights(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:34)
- [ ] `RandomCtmProperties.parseRandomLoops(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:78)
- [ ] `RandomCtmProperties.parseSymmetry(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:94)
- [ ] `RandomCtmProperties.parseLinked(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:101)
- [ ] `RandomCtmProperties.getIndexProviderFactory(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:108)
- [ ] `RandomCtmProperties.getRandomLoops(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:112)
- [ ] `RandomCtmProperties.getSymmetry(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:116)
- [ ] `RandomCtmProperties.getLinked(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RandomCtmProperties.java:120)

### `RepeatCtmProperties`: it extends `BaseCtmProperties`

- [ ] `RepeatCtmProperties.RepeatCtmProperties(6)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:17)
- [ ] `RepeatCtmProperties.init(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:23)
- [ ] `RepeatCtmProperties.parseWidth(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:32)
- [ ] `RepeatCtmProperties.parseHeight(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:52)
- [ ] `RepeatCtmProperties.parseSymmetry(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:72)
- [ ] `RepeatCtmProperties.parseOrient(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:79)
- [ ] `RepeatCtmProperties.getWidth(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:86)
- [ ] `RepeatCtmProperties.getHeight(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:90)
- [ ] `RepeatCtmProperties.getSymmetry(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:94)
- [ ] `RepeatCtmProperties.getOrientationMode(0)`: its class isn't translated (it extends `BaseCtmProperties`) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:98)

### `RepeatCtmProperties.Validator`: it's generic

- [ ] `RepeatCtmProperties.Validator.validateTileAmount(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java:103)

### `TileAmountValidator`: it's an interface

- [ ] `TileAmountValidator.validateTileAmount(2)`: it has no body (me/pepperbell/continuity/client/properties/TileAmountValidator.java:7)
- [ ] `TileAmountValidator.wrapFactory(2)`: it's generic (me/pepperbell/continuity/client/properties/TileAmountValidator.java:9)

### `TileAmountValidator.AtLeast`: it's generic

- [ ] `TileAmountValidator.AtLeast.AtLeast(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/properties/TileAmountValidator.java:23)
- [ ] `TileAmountValidator.AtLeast.validateTileAmount(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/properties/TileAmountValidator.java:27)

### `TileAmountValidator.Exactly`: it's generic

- [ ] `TileAmountValidator.Exactly.Exactly(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/properties/TileAmountValidator.java:54)
- [ ] `TileAmountValidator.Exactly.validateTileAmount(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/properties/TileAmountValidator.java:58)

### `CtmPropertiesLoader`: it uses Minecraft's `ResourceManager`

- [ ] `CtmPropertiesLoader.CtmPropertiesLoader(1)`: its class isn't translated (it uses Minecraft's `ResourceManager`) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:32)
- [ ] `CtmPropertiesLoader.loadAllWithState(1)`: it uses LoadingResult, which isn't translated (it uses LoadingContainer, which isn't translated (it's generic)) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:36)
- [ ] `CtmPropertiesLoader.loadAll(1)`: it uses LoadingResult, which isn't translated (it uses LoadingContainer, which isn't translated (it's generic)) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:43)
- [ ] `CtmPropertiesLoader.loadAll(0)`: its class isn't translated (it uses Minecraft's `ResourceManager`) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:47)
- [ ] `CtmPropertiesLoader.loadAll(2)`: its class isn't translated (it uses Minecraft's `ResourceManager`) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:59)
- [ ] `CtmPropertiesLoader.load(4)`: its class isn't translated (it uses Minecraft's `ResourceManager`) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:75)
- [ ] `CtmPropertiesLoader.load(6)`: it's generic (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:85)

### `CtmPropertiesLoader.LoadingContainer`: it's generic

- [ ] `CtmPropertiesLoader.LoadingContainer.LoadingContainer(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:96)
- [ ] `CtmPropertiesLoader.LoadingContainer.toProcessorHolder(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:97)
- [ ] `CtmPropertiesLoader.LoadingContainer.compareTo(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:103)
- [ ] `CtmPropertiesLoader.LoadingContainer.loader(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:96)
- [ ] `CtmPropertiesLoader.LoadingContainer.properties(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:96)

### `CtmPropertiesLoader.LoadingResult`: it uses LoadingContainer, which isn't translated (it's generic)

- [ ] `CtmPropertiesLoader.LoadingResult.LoadingResult(2)`: its class isn't translated (it uses LoadingContainer, which isn't translated (it's generic)) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:112)
- [ ] `CtmPropertiesLoader.LoadingResult.createProcessorHolders(1)`: its class isn't translated (it uses LoadingContainer, which isn't translated (it's generic)) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:117)
- [ ] `CtmPropertiesLoader.LoadingResult.getBlockAtlasSpriteDependencies(0)`: its class isn't translated (it uses LoadingContainer, which isn't translated (it's generic)) (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java:127)

### `CtmResourceReloader`: it implements Minecraft's `PreparableReloadListener`

- [ ] `CtmResourceReloader.BLOCK_ATLAS_EXTRA_IDS_FUTURE_KEY`: it uses Minecraft's `StateKey` (me/pepperbell/continuity/client/resource/CtmResourceReloader.java:22)
- [x] `CtmResourceReloader.ID` (me/pepperbell/continuity/client/resource/CtmResourceReloader.java:23)
- [ ] `CtmResourceReloader.INSTANCE`: it uses CtmResourceReloader, which isn't translated (it implements Minecraft's `PreparableReloadListener`) (me/pepperbell/continuity/client/resource/CtmResourceReloader.java:24)
- [ ] `CtmResourceReloader.prepareSharedState(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (me/pepperbell/continuity/client/resource/CtmResourceReloader.java:26)
- [ ] `CtmResourceReloader.reload(4)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (me/pepperbell/continuity/client/resource/CtmResourceReloader.java:31)
- [ ] `CtmResourceReloader.apply(1)`: its class isn't translated (it implements Minecraft's `PreparableReloadListener`) (me/pepperbell/continuity/client/resource/CtmResourceReloader.java:66)

### `CuboidItemModelWrapperInitContext`

- [ ] `CuboidItemModelWrapperInitContext.SPRITE_FINDER_GETTER`: it uses `ThreadLocal` (me/pepperbell/continuity/client/resource/CuboidItemModelWrapperInitContext.java:6)
- [x] `CuboidItemModelWrapperInitContext.CuboidItemModelWrapperInitContext(0)` (me/pepperbell/continuity/client/resource/CuboidItemModelWrapperInitContext.java:8)

### `EmissiveSuffixLoader`

- [x] `EmissiveSuffixLoader.LOCATION` (me/pepperbell/continuity/client/resource/EmissiveSuffixLoader.java:14)
- [x] `EmissiveSuffixLoader.emissiveSuffix` (me/pepperbell/continuity/client/resource/EmissiveSuffixLoader.java:15)
- [x] `EmissiveSuffixLoader.getEmissiveSuffix(0)` (me/pepperbell/continuity/client/resource/EmissiveSuffixLoader.java:17)
- [ ] `EmissiveSuffixLoader.load(1)`: it uses Minecraft's `ResourceManager` (me/pepperbell/continuity/client/resource/EmissiveSuffixLoader.java:22)

### `ModelWrappingHandler`

- [ ] `ModelWrappingHandler.WRAP_CTM_FUTURE_KEY`: it uses Minecraft's `StateKey` (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java:12)
- [ ] `ModelWrappingHandler.WRAP_EMISSIVE_FUTURE_KEY`: it uses Minecraft's `StateKey` (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java:13)
- [ ] `ModelWrappingHandler.init(0)`: a lambda (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java:15)

### `ModelWrappingHandler.Data`

- [x] `ModelWrappingHandler.Data.Data(2)` (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java:43)
- [x] `ModelWrappingHandler.Data.wrapCtm(0)` (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java:43)
- [x] `ModelWrappingHandler.Data.wrapEmissive(0)` (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java:43)

### `ResourceRedirectHandler`

- [x] `ResourceRedirectHandler.SPRITE_PATH_START` (me/pepperbell/continuity/client/resource/ResourceRedirectHandler.java:6)
- [x] `ResourceRedirectHandler.PATH_START` (me/pepperbell/continuity/client/resource/ResourceRedirectHandler.java:7)
- [x] `ResourceRedirectHandler.PATH_START_LENGTH` (me/pepperbell/continuity/client/resource/ResourceRedirectHandler.java:8)
- [x] `ResourceRedirectHandler.redirect(1)` (me/pepperbell/continuity/client/resource/ResourceRedirectHandler.java:10)

### `SpriteLoaderLoadContext`: it's an interface

- [ ] `SpriteLoaderLoadContext.getExtraIdsFuture(1)`: it has no body (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContext.java:12)
- [ ] `SpriteLoaderLoadContext.getEmissiveControl(1)`: it has no body (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContext.java:15)

### `SpriteLoaderLoadContext.EmissiveControl`: it's an interface

- [ ] `SpriteLoaderLoadContext.EmissiveControl.getEmissiveIdMap(0)`: it has no body (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContext.java:19)
- [ ] `SpriteLoaderLoadContext.EmissiveControl.setEmissiveIdMap(1)`: it has no body (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContext.java:22)
- [ ] `SpriteLoaderLoadContext.EmissiveControl.setHasEmissives(1)`: it has no body (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContext.java:24)

### `SpriteLoaderLoadContextImpl`: it implements `SpriteLoaderLoadContext`

- [ ] `SpriteLoaderLoadContextImpl.SpriteLoaderLoadContextImpl(2)`: its class isn't translated (it implements `SpriteLoaderLoadContext`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:15)
- [ ] `SpriteLoaderLoadContextImpl.getExtraIdsFuture(1)`: its class isn't translated (it implements `SpriteLoaderLoadContext`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:21)
- [ ] `SpriteLoaderLoadContextImpl.getEmissiveControl(1)`: its class isn't translated (it implements `SpriteLoaderLoadContext`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:27)

### `SpriteLoaderLoadContextImpl.EmissiveControlImpl`: it implements `EmissiveControl`

- [ ] `SpriteLoaderLoadContextImpl.EmissiveControlImpl.EmissiveControlImpl(1)`: its class isn't translated (it implements `EmissiveControl`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:43)
- [ ] `SpriteLoaderLoadContextImpl.EmissiveControlImpl.EmissiveControlImpl(0)`: its class isn't translated (it implements `EmissiveControl`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:47)
- [ ] `SpriteLoaderLoadContextImpl.EmissiveControlImpl.getEmissiveIdMap(0)`: its class isn't translated (it implements `EmissiveControl`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:51)
- [ ] `SpriteLoaderLoadContextImpl.EmissiveControlImpl.setEmissiveIdMap(1)`: its class isn't translated (it implements `EmissiveControl`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:57)
- [ ] `SpriteLoaderLoadContextImpl.EmissiveControlImpl.setHasEmissives(1)`: its class isn't translated (it implements `EmissiveControl`) (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java:68)

### `SpriteLoaderStitchContext`: it's an interface

- [ ] `SpriteLoaderStitchContext.getEmissiveIdMap(0)`: it has no body (me/pepperbell/continuity/client/resource/SpriteLoaderStitchContext.java:9)
- [ ] `SpriteLoaderStitchContext.setHasEmissives(1)`: it has no body (me/pepperbell/continuity/client/resource/SpriteLoaderStitchContext.java:11)

### `SpriteSourceListInitContext`: it's an interface

- [ ] `SpriteSourceListInitContext.getExtraIds(0)`: it has no body (me/pepperbell/continuity/client/resource/SpriteSourceListInitContext.java:9)

### `SpriteSourceListListContext`: it's an interface

- [ ] `SpriteSourceListListContext.setEmissiveIdMap(1)`: it has no body (me/pepperbell/continuity/client/resource/SpriteSourceListListContext.java:9)

### `BiomeHolder`: it uses Minecraft's `Biome`

- [ ] `BiomeHolder.BiomeHolder(1)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolder.java:15)
- [ ] `BiomeHolder.getId(0)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolder.java:19)
- [ ] `BiomeHolder.getBiome(0)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolder.java:23)
- [ ] `BiomeHolder.refresh(2)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolder.java:28)
- [ ] `BiomeHolder.equals(1)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolder.java:41)
- [ ] `BiomeHolder.hashCode(0)`: its class isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolder.java:53)

### `BiomeHolderManager`

- [ ] `BiomeHolderManager.HOLDER_CACHE`: it uses BiomeHolder, which isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:17)
- [ ] `BiomeHolderManager.REFRESH_CALLBACKS`: a Map or Set keyed by a fn#0 (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:18)
- [ ] `BiomeHolderManager.registryManager`: it uses Minecraft's `RegistryAccess` (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:20)
- [ ] `BiomeHolderManager.getOrCreateHolder(1)`: it uses BiomeHolder, which isn't translated (it uses Minecraft's `Biome`) (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:22)
- [ ] `BiomeHolderManager.addRefreshCallback(1)`: it uses BiomeHolderManager.REFRESH_CALLBACKS, which isn't translated (a Map or Set keyed by a fn#0) (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:26)
- [ ] `BiomeHolderManager.init(0)`: it uses Fabric's `ClientPlayConnectionEvents`.JOIN (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:30)
- [ ] `BiomeHolderManager.refreshHolders(0)`: it uses BiomeHolderManager.registryManager, which isn't translated (it uses Minecraft's `RegistryAccess`) (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:37)
- [ ] `BiomeHolderManager.clearCache(0)`: it uses BiomeHolderManager.HOLDER_CACHE, which isn't translated (it uses BiomeHolder, which isn't translated (it uses Minecraft's `Biome`)) (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java:63)

### `BiomeSetPredicate`: it implements `Predicate` (java.util.function)

- [ ] `BiomeSetPredicate.BiomeSetPredicate(1)`: its class isn't translated (it implements `Predicate` (java.util.function)) (me/pepperbell/continuity/client/util/biome/BiomeSetPredicate.java:13)
- [ ] `BiomeSetPredicate.test(1)`: its class isn't translated (it implements `Predicate` (java.util.function)) (me/pepperbell/continuity/client/util/biome/BiomeSetPredicate.java:18)
- [ ] `BiomeSetPredicate.refresh(0)`: its class isn't translated (it implements `Predicate` (java.util.function)) (me/pepperbell/continuity/client/util/biome/BiomeSetPredicate.java:22)

### `BooleanState`

- [x] `BooleanState.isEnabled(0)` (me/pepperbell/continuity/client/util/BooleanState.java:6)
- [x] `BooleanState.enable(0)` (me/pepperbell/continuity/client/util/BooleanState.java:10)
- [x] `BooleanState.disable(0)` (me/pepperbell/continuity/client/util/BooleanState.java:14)

### `ListPairView`: it's generic

- [ ] `ListPairView.ListPairView(2)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/util/ListPairView.java:12)
- [ ] `ListPairView.get(1)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/util/ListPairView.java:17)
- [ ] `ListPairView.size(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/client/util/ListPairView.java:24)

### `MathUtil`

- [x] `MathUtil.GOLDEN_GAMMA` (me/pepperbell/continuity/client/util/MathUtil.java:6)
- [x] `MathUtil.signum(1)` (me/pepperbell/continuity/client/util/MathUtil.java:8)
- [x] `MathUtil.lerp(3)` (me/pepperbell/continuity/client/util/MathUtil.java:16)
- [x] `MathUtil.lerpColor(3)` (me/pepperbell/continuity/client/util/MathUtil.java:20)
- [x] `MathUtil.lerpLight(3)` (me/pepperbell/continuity/client/util/MathUtil.java:27)
- [x] `MathUtil.mix64(1)` (me/pepperbell/continuity/client/util/MathUtil.java:31)
- [x] `MathUtil.mix32(1)` (me/pepperbell/continuity/client/util/MathUtil.java:37)
- [ ] `MathUtil.mix(5)`: it calls Minecraft's `Mth`.getSeed (me/pepperbell/continuity/client/util/MathUtil.java:42)
- [x] `MathUtil.removeSignBit(1)` (me/pepperbell/continuity/client/util/MathUtil.java:46)

### `QuadUtil`

- [ ] `QuadUtil.interpolate(3)`: it uses Fabric's `MutableQuadView` (me/pepperbell/continuity/client/util/QuadUtil.java:16)
- [ ] `QuadUtil.interpolate(4)`: it uses Minecraft's `BakedQuad` (me/pepperbell/continuity/client/util/QuadUtil.java:31)
- [ ] `QuadUtil.emitOverlayQuad(6)`: it uses Fabric's `QuadEmitter` (me/pepperbell/continuity/client/util/QuadUtil.java:45)
- [ ] `QuadUtil.isQuadUnitSquare(1)`: it uses Fabric's `QuadView` (me/pepperbell/continuity/client/util/QuadUtil.java:60)
- [ ] `QuadUtil.getTextureOrientation(1)`: it uses Fabric's `QuadView` (me/pepperbell/continuity/client/util/QuadUtil.java:100)

### `QuadUtil.PackedUvContainer`

- [x] `QuadUtil.PackedUvContainer.packedUV(2)` (me/pepperbell/continuity/client/util/QuadUtil.java:173)

### `RandomIndexProvider`: it's an interface

- [ ] `RandomIndexProvider.getRandomIndex(1)`: it has no body (me/pepperbell/continuity/client/util/RandomIndexProvider.java:4)

### `RandomIndexProvider.Factory`: it's an interface

- [ ] `RandomIndexProvider.Factory.createIndexProvider(1)`: it has no body (me/pepperbell/continuity/client/util/RandomIndexProvider.java:7)

### `RandomIndexProvider.Unweighted`: it implements `RandomIndexProvider`

- [ ] `RandomIndexProvider.Unweighted.Unweighted(1)`: its class isn't translated (it implements `RandomIndexProvider`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:13)
- [ ] `RandomIndexProvider.Unweighted.getRandomIndex(1)`: its class isn't translated (it implements `RandomIndexProvider`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:17)

### `RandomIndexProvider.UnweightedFactory`: it implements `Factory`

- [ ] `RandomIndexProvider.UnweightedFactory.INSTANCE`: it uses UnweightedFactory, which isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:24)
- [ ] `RandomIndexProvider.UnweightedFactory.createIndexProvider(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:26)

### `RandomIndexProvider.Weighted`: it implements `RandomIndexProvider`

- [ ] `RandomIndexProvider.Weighted.Weighted(2)`: its class isn't translated (it implements `RandomIndexProvider`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:37)
- [ ] `RandomIndexProvider.Weighted.getRandomIndex(1)`: its class isn't translated (it implements `RandomIndexProvider`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:43)

### `RandomIndexProvider.WeightedFactory`: it implements `Factory`

- [ ] `RandomIndexProvider.WeightedFactory.WeightedFactory(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:59)
- [ ] `RandomIndexProvider.WeightedFactory.createIndexProvider(1)`: its class isn't translated (it implements `Factory`) (me/pepperbell/continuity/client/util/RandomIndexProvider.java:63)

### `RenderUtil`

- [ ] `RenderUtil.BLOCK_COLORS`: it uses Minecraft's `BlockColors` (me/pepperbell/continuity/client/util/RenderUtil.java:21)
- [ ] `RenderUtil.blockAtlasSpriteFinder`: it uses Fabric's `SpriteFinder` (me/pepperbell/continuity/client/util/RenderUtil.java:22)
- [ ] `RenderUtil.getTintColor(4)`: it uses Minecraft's `BlockState` (me/pepperbell/continuity/client/util/RenderUtil.java:24)
- [ ] `RenderUtil.aoFromTintBlock(1)`: it uses Fabric's `TriState` (me/pepperbell/continuity/client/util/RenderUtil.java:33)
- [ ] `RenderUtil.canHaveAO(1)`: it uses Minecraft's `BlockState` (me/pepperbell/continuity/client/util/RenderUtil.java:37)
- [ ] `RenderUtil.isMissingSprite(1)`: it uses Minecraft's `TextureAtlasSprite` (me/pepperbell/continuity/client/util/RenderUtil.java:41)
- [ ] `RenderUtil.getSpriteFinder(0)`: it uses Fabric's `SpriteFinder` (me/pepperbell/continuity/client/util/RenderUtil.java:45)

### `RenderUtil.ReloadListener`: it implements Minecraft's `ResourceManagerReloadListener`

- [x] `RenderUtil.ReloadListener.ID` (me/pepperbell/continuity/client/util/RenderUtil.java:50)
- [ ] `RenderUtil.ReloadListener.INSTANCE`: it uses ReloadListener, which isn't translated (it implements Minecraft's `ResourceManagerReloadListener`) (me/pepperbell/continuity/client/util/RenderUtil.java:51)
- [ ] `RenderUtil.ReloadListener.onResourceManagerReload(1)`: its class isn't translated (it implements Minecraft's `ResourceManagerReloadListener`) (me/pepperbell/continuity/client/util/RenderUtil.java:53)

### `SpriteCalculator`

- [ ] `SpriteCalculator.MODEL_MANAGER`: it uses Minecraft's `ModelManager` (me/pepperbell/continuity/client/util/SpriteCalculator.java:27)
- [ ] `SpriteCalculator.SPRITE_CACHES`: it uses `EnumMap` (java.util) (me/pepperbell/continuity/client/util/SpriteCalculator.java:28)
- [ ] `SpriteCalculator.getSprites(2)`: it uses Minecraft's `TextureAtlasSprite` (me/pepperbell/continuity/client/util/SpriteCalculator.java:30)
- [ ] `SpriteCalculator.clearCache(0)`: it uses SpriteCalculator.SPRITE_CACHES, which isn't translated (it uses `EnumMap` (java.util)) (me/pepperbell/continuity/client/util/SpriteCalculator.java:34)

### `SpriteCalculator.SpriteCache`: it uses Minecraft's `Direction`

- [ ] `SpriteCalculator.SpriteCache.SpriteCache(1)`: its class isn't translated (it uses Minecraft's `Direction`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:56)
- [ ] `SpriteCalculator.SpriteCache.getSprites(1)`: its class isn't translated (it uses Minecraft's `Direction`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:61)
- [ ] `SpriteCalculator.SpriteCache.calculateSprites(1)`: its class isn't translated (it uses Minecraft's `Direction`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:99)
- [ ] `SpriteCalculator.SpriteCache.clear(0)`: its class isn't translated (it uses Minecraft's `Direction`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:116)

### `SpriteCalculator.SpriteCache.CollectingQuadTransform`: it implements Fabric's `QuadTransform`

- [ ] `SpriteCalculator.SpriteCache.CollectingQuadTransform.CollectingQuadTransform(1)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:131)
- [ ] `SpriteCalculator.SpriteCache.CollectingQuadTransform.transform(1)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:135)
- [ ] `SpriteCalculator.SpriteCache.CollectingQuadTransform.clear(0)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:143)
- [ ] `SpriteCalculator.SpriteCache.CollectingQuadTransform.result(0)`: its class isn't translated (it implements Fabric's `QuadTransform`) (me/pepperbell/continuity/client/util/SpriteCalculator.java:147)

### `ContinuityFeatureStatesImpl`: it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)

- [ ] `ContinuityFeatureStatesImpl.ContinuityFeatureStatesImpl(0)`: its class isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ContinuityFeatureStatesImpl.java:11)
- [ ] `ContinuityFeatureStatesImpl.get(0)`: it uses ContinuityFeatureStatesImpl, which isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ContinuityFeatureStatesImpl.java:16)
- [ ] `ContinuityFeatureStatesImpl.getConnectedTexturesState(0)`: its class isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ContinuityFeatureStatesImpl.java:20)
- [ ] `ContinuityFeatureStatesImpl.getEmissiveTexturesState(0)`: its class isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ContinuityFeatureStatesImpl.java:25)

### `ContinuityFeatureStatesImpl.FeatureStateImpl`: it extends `BooleanState` (me.pepperbell.continuity.client.util)


### `CtmLoaderRegistryImpl`: it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client)

- [ ] `CtmLoaderRegistryImpl.INSTANCE`: it uses CtmLoaderRegistryImpl, which isn't translated (it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/CtmLoaderRegistryImpl.java:10)
- [ ] `CtmLoaderRegistryImpl.registerLoader(2)`: its class isn't translated (it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/CtmLoaderRegistryImpl.java:13)
- [ ] `CtmLoaderRegistryImpl.getLoader(1)`: its class isn't translated (it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/CtmLoaderRegistryImpl.java:18)

### `EmissiveSpriteApiImpl`: it implements `EmissiveSpriteApi` (me.pepperbell.continuity.api.client)

- [ ] `EmissiveSpriteApiImpl.INSTANCE`: it uses EmissiveSpriteApiImpl, which isn't translated (it implements `EmissiveSpriteApi` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/EmissiveSpriteApiImpl.java:9)
- [ ] `EmissiveSpriteApiImpl.getEmissiveSprite(1)`: its class isn't translated (it implements `EmissiveSpriteApi` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/EmissiveSpriteApiImpl.java:11)

### `ProcessingContextImpl`: it implements `ProcessingContext`

- [ ] `ProcessingContextImpl.getExtraQuadEmitter(0)`: its class isn't translated (it implements `ProcessingContext`) (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java:16)
- [ ] `ProcessingContextImpl.getData(1)`: it's generic (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java:21)
- [ ] `ProcessingContextImpl.getDataOrNull(1)`: it's generic (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java:33)
- [ ] `ProcessingContextImpl.outputTo(1)`: its class isn't translated (it implements `ProcessingContext`) (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java:38)
- [ ] `ProcessingContextImpl.reset(0)`: its class isn't translated (it implements `ProcessingContext`) (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java:42)
- [ ] `ProcessingContextImpl.resetData(0)`: its class isn't translated (it implements `ProcessingContext`) (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java:47)
- [ ] `ProcessingContextImpl.resetData(1)`: it's generic (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java:56)

### `ProcessingDataKeyImpl`: it's generic

- [ ] `ProcessingDataKeyImpl.ProcessingDataKeyImpl(4)`: its class isn't translated (it's generic) (me/pepperbell/continuity/impl/client/ProcessingDataKeyImpl.java:15)
- [ ] `ProcessingDataKeyImpl.getId(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/impl/client/ProcessingDataKeyImpl.java:22)
- [ ] `ProcessingDataKeyImpl.getRawId(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/impl/client/ProcessingDataKeyImpl.java:27)
- [ ] `ProcessingDataKeyImpl.getValueSupplier(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/impl/client/ProcessingDataKeyImpl.java:32)
- [ ] `ProcessingDataKeyImpl.getValueResetAction(0)`: its class isn't translated (it's generic) (me/pepperbell/continuity/impl/client/ProcessingDataKeyImpl.java:37)

### `ProcessingDataKeyRegistryImpl`: it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)

- [ ] `ProcessingDataKeyRegistryImpl.INSTANCE`: it uses ProcessingDataKeyRegistryImpl, which isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java:18)
- [ ] `ProcessingDataKeyRegistryImpl.registerKey(3)`: it's generic (me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java:25)
- [ ] `ProcessingDataKeyRegistryImpl.getKey(1)`: its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java:46)
- [ ] `ProcessingDataKeyRegistryImpl.getRegisteredAmount(0)`: its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java:52)
- [ ] `ProcessingDataKeyRegistryImpl.init(0)`: its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java:57)
- [ ] `ProcessingDataKeyRegistryImpl.getAllResettable(0)`: its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)) (me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java:61)
