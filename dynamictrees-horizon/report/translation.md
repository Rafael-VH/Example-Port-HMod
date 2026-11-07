# Java translated to Rust

Translated 132 of 4210 methods and 203 of 733 static fields to Rust (2 of those call the game, through Horizon's helpers); 0 of 3 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 3272 | `CellKit.CellKit(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) |
| Other Java this translator doesn't handle yet | 558 | `Cell.getValue(0)`: it has no body |
| Uses Minecraft classes (needs Horizon's game API) | 221 | `BranchDestructionData.encodeRelBlockPos(1)`: it uses Minecraft's `class_2338` |
| Uses Fabric API | 8 | `DynamicTreesFabric.getEntryPointContainers(0)`: it uses Fabric's `EntrypointContainer` |
| Uses Java collections other than List (Map, Set...) | 5 | `TextureHelper.PixelBuffer.fill(1)`: it calls `Arrays` (java.util).fill |
| Uses other libraries | 4 | `BranchBlock.getCachedSoilState(3)`: it uses `Pair` (org.apache.commons.lang3.tuple) |
| Shares one object between names (Java references) | 3 | `PropertyApplierResult.addWarnings(1)`: it changes an object through a second name |
| Uses files (java.nio / java.io) | 3 | `JsonHelper.load(1)`: it uses `File` (java.io) |
| Calls the game where Horizon has no helper yet | 1 | `HexColorArgument.getHexCode(2)`: it calls Minecraft's FabricClientCommandSource.getArgument (no Horizon helper for it yet) |
| Uses Gson (JSON) | 1 | `ExpressionParser.error(1)`: it uses `JsonParseException` (com.google.gson) |
| Uses lambdas or method references | 1 | `JsonHelper.throwIfShouldNotLoad(1)`: a lambda |
| Uses try/catch | 1 | `BiomeDatabases.tryBlacklist(1)`: a try/catch (class_151) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| com/dtteam/dynamictrees/DynamicTreesFabricClient.java:75 | `tick_start` | not translated: it uses Minecraft's Minecraft.field_1687 (no Horizon helper for it yet) |
| com/dtteam/dynamictrees/DynamicTreesFabricClient.java:190 | `level_tick_start` | not translated: it calls SeasonHelper.updateTick, which isn't translated |
| com/dtteam/dynamictrees/DynamicTreesFabric.java:24 | `init` | not translated: it calls FabricRegistryHandler.setup, which isn't translated |

## Every class


### `com/dtteam/dynamictrees/api/configuration/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/block/branch/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/block/leaves/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/block/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/block/soil/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/command/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/deserialization/deserializer/worldgen/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/deserialization/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/item/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/loot/condition/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/loot/entry/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/loot/function/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/systems/genfeature/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `com/dtteam/dynamictrees/treepack/package-info.java`: couldn't be read: expected a class at `package` (line 2)


### `Cell`: it's an interface

- [ ] `Cell.getValue(0)`: it has no body (com/dtteam/dynamictrees/api/cell/Cell.java:6)
- [ ] `Cell.getValueFromSide(1)`: it has no body (com/dtteam/dynamictrees/api/cell/Cell.java:8)

### `CellKit`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `CellKit.NULL_CELL_SOLVER`: it uses CellSolver, which isn't translated (it's an interface) (com/dtteam/dynamictrees/api/cell/CellKit.java:11)
- [ ] `CellKit.NULL_CELL_KIT`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/api/cell/CellKit.java:12)
- [ ] `CellKit.REGISTRY`: it uses SimpleRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/cell/CellKit.java:38)
- [ ] `CellKit.CellKit(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/api/cell/CellKit.java:40)
- [ ] `CellKit.getCellForLeaves(1)`: it has no body (com/dtteam/dynamictrees/api/cell/CellKit.java:44)
- [ ] `CellKit.getCellForBranch(2)`: it has no body (com/dtteam/dynamictrees/api/cell/CellKit.java:46)
- [ ] `CellKit.getCellSolver(0)`: it has no body (com/dtteam/dynamictrees/api/cell/CellKit.java:48)
- [ ] `CellKit.getLeafCluster(0)`: it has no body (com/dtteam/dynamictrees/api/cell/CellKit.java:50)
- [ ] `CellKit.getDefaultHydration(0)`: it has no body (com/dtteam/dynamictrees/api/cell/CellKit.java:52)
- [ ] `CellKit.findCellKit(1)`: it uses CellKit, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/api/cell/CellKit.java:54)
- [ ] `CellKit.findCellKit(1)`: it uses CellKit, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/api/cell/CellKit.java:58)

### `CellNull`: it implements `Cell`

- [ ] `CellNull.NULL_CELL`: it uses CellNull, which isn't translated (it implements `Cell`) (com/dtteam/dynamictrees/api/cell/CellNull.java:6)
- [ ] `CellNull.getValue(0)`: its class isn't translated (it implements `Cell`) (com/dtteam/dynamictrees/api/cell/CellNull.java:8)
- [ ] `CellNull.getValueFromSide(1)`: its class isn't translated (it implements `Cell`) (com/dtteam/dynamictrees/api/cell/CellNull.java:13)

### `CellSolver`: it's an interface

- [ ] `CellSolver.solve(1)`: it has no body (com/dtteam/dynamictrees/api/cell/CellSolver.java:4)

### `Configurable`: it's an interface

- [ ] `Configurable.isPropertyRegistered(1)`: it has no body (com/dtteam/dynamictrees/api/configuration/Configurable.java:6)
- [ ] `Configurable.getRegisteredProperties(0)`: it has no body (com/dtteam/dynamictrees/api/configuration/Configurable.java:8)
- [ ] `Configurable.getDefaultConfiguration(0)`: it has no body (com/dtteam/dynamictrees/api/configuration/Configurable.java:10)

### `ConfigurableRegistry`: it's generic

- [ ] `ConfigurableRegistry.ConfigurableRegistry(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java:8)
- [ ] `ConfigurableRegistry.ConfigurableRegistry(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java:13)
- [ ] `ConfigurableRegistry.ConfigurableRegistry(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java:18)
- [ ] `ConfigurableRegistry.ConfigurableRegistry(5)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java:23)
- [ ] `ConfigurableRegistry.register(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java:28)
- [ ] `ConfigurableRegistry.registerTemplate(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java:37)
- [ ] `ConfigurableRegistry.getTemplates(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistry.java:41)

### `ConfigurableRegistryEntry`: it's generic

- [ ] `ConfigurableRegistryEntry.ConfigurableRegistryEntry(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:17)
- [ ] `ConfigurableRegistryEntry.ConfigurableRegistryEntry(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:21)
- [ ] `ConfigurableRegistryEntry.createDefaultConfiguration(0)`: it has no body (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:27)
- [ ] `ConfigurableRegistryEntry.registerProperties(0)`: it has no body (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:29)
- [ ] `ConfigurableRegistryEntry.register(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:31)
- [ ] `ConfigurableRegistryEntry.isPropertyRegistered(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:36)
- [ ] `ConfigurableRegistryEntry.getRegisteredProperties(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:41)
- [ ] `ConfigurableRegistryEntry.getDefaultConfiguration(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:46)
- [ ] `ConfigurableRegistryEntry.with(2)`: it's generic (com/dtteam/dynamictrees/api/configuration/ConfigurableRegistryEntry.java:51)

### `Configuration`: it's generic

- [ ] `Configuration.Configuration(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/Configuration.java:15)
- [ ] `Configuration.with(2)`: it's generic (com/dtteam/dynamictrees/api/configuration/Configuration.java:19)
- [ ] `Configuration.withAll(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/Configuration.java:39)
- [ ] `Configuration.has(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/Configuration.java:44)
- [ ] `Configuration.get(1)`: it's generic (com/dtteam/dynamictrees/api/configuration/Configuration.java:48)
- [ ] `Configuration.getAsOptional(1)`: it's generic (com/dtteam/dynamictrees/api/configuration/Configuration.java:62)
- [ ] `Configuration.getOrInvalidDefault(3)`: it's generic (com/dtteam/dynamictrees/api/configuration/Configuration.java:74)
- [ ] `Configuration.copy(0)`: it has no body (com/dtteam/dynamictrees/api/configuration/Configuration.java:78)
- [ ] `Configuration.getConfigurable(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/Configuration.java:80)
- [ ] `Configuration.toString(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/Configuration.java:84)

### `ConfigurationProperty`: it's generic

- [ ] `ConfigurationProperty.NULL`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:15)
- [ ] `ConfigurationProperty.ConfigurationProperty(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:19)
- [ ] `ConfigurationProperty.getKey(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:24)
- [ ] `ConfigurationProperty.getType(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:28)
- [ ] `ConfigurationProperty.deserialise(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:32)
- [ ] `ConfigurationProperty.property(2)`: it's generic (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:44)
- [ ] `ConfigurationProperty.string(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:49)
- [ ] `ConfigurationProperty.bool(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:53)
- [ ] `ConfigurationProperty.integer(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:57)
- [ ] `ConfigurationProperty.longProperty(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:61)
- [ ] `ConfigurationProperty.doubleProperty(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:65)
- [ ] `ConfigurationProperty.floatProperty(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:69)
- [ ] `ConfigurationProperty.block(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:73)
- [ ] `ConfigurationProperty.item(1)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:77)
- [ ] `ConfigurationProperty.equals(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:81)
- [ ] `ConfigurationProperty.hashCode(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:93)
- [ ] `ConfigurationProperty.toString(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationProperty.java:98)

### `ConfigurationTemplate`: it's an interface

- [ ] `ConfigurationTemplate.apply(1)`: it has no body (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplate.java:6)
- [ ] `ConfigurationTemplate.getRegisteredProperties(0)`: it has no body (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplate.java:8)

### `ConfigurationTemplateResourceLoader`: it's generic

- [ ] `ConfigurationTemplateResourceLoader.ConfigurationTemplateResourceLoader(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:22)
- [ ] `ConfigurationTemplateResourceLoader.applyOnReload(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:28)
- [ ] `ConfigurationTemplateResourceLoader.register(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:42)
- [ ] `ConfigurationTemplateResourceLoader.createTemplate(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:50)
- [ ] `ConfigurationTemplateResourceLoader.getConfigurable(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:56)
- [ ] `ConfigurationTemplateResourceLoader.getConfigurableName(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:60)
- [ ] `ConfigurationTemplateResourceLoader.deserialisePropertyDefinitions(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:64)
- [ ] `ConfigurationTemplateResourceLoader.assertNameNotReserved(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/ConfigurationTemplateResourceLoader.java:77)

### `CustomConfigurationProperty`: it's generic

- [ ] `CustomConfigurationProperty.CustomConfigurationProperty(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/CustomConfigurationProperty.java:4)

### `CustomConfigurationTemplate`: it's generic

- [ ] `CustomConfigurationTemplate.CustomConfigurationTemplate(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/CustomConfigurationTemplate.java:19)
- [ ] `CustomConfigurationTemplate.collectAllProperties(2)`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/CustomConfigurationTemplate.java:26)
- [ ] `CustomConfigurationTemplate.apply(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/CustomConfigurationTemplate.java:31)
- [ ] `CustomConfigurationTemplate.deserialiseJson(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/CustomConfigurationTemplate.java:37)
- [ ] `CustomConfigurationTemplate.processJson(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/CustomConfigurationTemplate.java:45)
- [ ] `CustomConfigurationTemplate.getRegisteredProperties(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/CustomConfigurationTemplate.java:55)

### `DefaultConfigurationTemplate`: it's generic

- [ ] `DefaultConfigurationTemplate.DefaultConfigurationTemplate(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/DefaultConfigurationTemplate.java:12)
- [ ] `DefaultConfigurationTemplate.apply(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/DefaultConfigurationTemplate.java:17)
- [ ] `DefaultConfigurationTemplate.getRegisteredProperties(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/DefaultConfigurationTemplate.java:22)

### `Properties`: it implements `PropertiesAccessor`

- [x] `Properties.NONE` (com/dtteam/dynamictrees/api/configuration/Properties.java:8)
- [ ] `Properties.put(2)`: it's generic (com/dtteam/dynamictrees/api/configuration/Properties.java:11)
- [ ] `Properties.putAll(1)`: its class isn't translated (it implements `PropertiesAccessor`) (com/dtteam/dynamictrees/api/configuration/Properties.java:15)
- [ ] `Properties.get(1)`: it's generic (com/dtteam/dynamictrees/api/configuration/Properties.java:19)
- [ ] `Properties.has(1)`: its class isn't translated (it implements `PropertiesAccessor`) (com/dtteam/dynamictrees/api/configuration/Properties.java:25)
- [ ] `Properties.forEach(1)`: its class isn't translated (it implements `PropertiesAccessor`) (com/dtteam/dynamictrees/api/configuration/Properties.java:30)
- [ ] `Properties.applyIteration(3)`: it's generic (com/dtteam/dynamictrees/api/configuration/Properties.java:35)
- [ ] `Properties.toString(0)`: its class isn't translated (it implements `PropertiesAccessor`) (com/dtteam/dynamictrees/api/configuration/Properties.java:39)

### `PropertiesAccessor`: it's an interface

- [ ] `PropertiesAccessor.get(1)`: it has no body (com/dtteam/dynamictrees/api/configuration/PropertiesAccessor.java:6)
- [ ] `PropertiesAccessor.has(1)`: it has no body (com/dtteam/dynamictrees/api/configuration/PropertiesAccessor.java:9)
- [ ] `PropertiesAccessor.forEach(1)`: it has no body (com/dtteam/dynamictrees/api/configuration/PropertiesAccessor.java:11)

### `PropertiesAccessor.IterationAction`: it's an interface

- [ ] `PropertiesAccessor.IterationAction.apply(2)`: it has no body (com/dtteam/dynamictrees/api/configuration/PropertiesAccessor.java:14)

### `PropertyDefinition`: it's generic

- [x] `PropertyDefinition.PREFIX` (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:4)
- [ ] `PropertyDefinition.NULL`: it uses PropertyDefinition, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:5)
- [ ] `PropertyDefinition.PropertyDefinition(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:11)
- [ ] `PropertyDefinition.PropertyDefinition(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:15)
- [ ] `PropertyDefinition.getProperty(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:22)
- [ ] `PropertyDefinition.process(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:26)
- [ ] `PropertyDefinition.getOrDefault(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:30)
- [ ] `PropertyDefinition.captureClass(0)`: it uses `Class` (com/dtteam/dynamictrees/api/configuration/PropertyDefinition.java:34)

### `TemplateRegistry`: it's generic

- [ ] `TemplateRegistry.register(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/TemplateRegistry.java:11)
- [ ] `TemplateRegistry.get(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/configuration/TemplateRegistry.java:15)

### `DynamicTreesAddonEntrypoint`: it's an interface

- [ ] `DynamicTreesAddonEntrypoint.onDynamicTreesPreSetup(0)`: it has no body (com/dtteam/dynamictrees/api/DynamicTreesAddonEntrypoint.java:8)
- [ ] `DynamicTreesAddonEntrypoint.onAddResourceLoaders(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/DynamicTreesAddonEntrypoint.java:10)
- [ ] `DynamicTreesAddonEntrypoint.onRegisterStagedApplier(3)`: it's generic (com/dtteam/dynamictrees/api/DynamicTreesAddonEntrypoint.java:13)

### `IgnoreThrowable`: it extends `Throwable`

- [ ] `IgnoreThrowable.INSTANCE`: it uses IgnoreThrowable, which isn't translated (it extends `Throwable`) (com/dtteam/dynamictrees/api/function/IgnoreThrowable.java:4)
- [ ] `IgnoreThrowable.IgnoreThrowable(0)`: its class isn't translated (it extends `Throwable`) (com/dtteam/dynamictrees/api/function/IgnoreThrowable.java:6)

### `TetraFunction`: it's an interface

- [ ] `TetraFunction.apply(4)`: it has no body (com/dtteam/dynamictrees/api/function/TetraFunction.java:5)

### `ThrowableBiFunction`: it's an interface

- [ ] `ThrowableBiFunction.apply(2)`: it has no body (com/dtteam/dynamictrees/api/function/ThrowableBiFunction.java:5)

### `ThrowableFunction`: it's an interface

- [ ] `ThrowableFunction.apply(1)`: it has no body (com/dtteam/dynamictrees/api/function/ThrowableFunction.java:5)

### `ThrowableRunnable`: it's an interface

- [ ] `ThrowableRunnable.run(0)`: it has no body (com/dtteam/dynamictrees/api/function/ThrowableRunnable.java:5)

### `TriPredicate`: it's an interface

- [ ] `TriPredicate.test(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/function/TriPredicate.java:7)

### `LazyValue`: it's an interface

- [ ] `LazyValue.get(0)`: it has no body (com/dtteam/dynamictrees/api/lazyvalue/LazyValue.java:7)
- [ ] `LazyValue.of(1)`: it's generic (com/dtteam/dynamictrees/api/lazyvalue/LazyValue.java:9)
- [ ] `LazyValue.supplied(1)`: it's generic (com/dtteam/dynamictrees/api/lazyvalue/LazyValue.java:13)

### `MutableLazyValue`: it's an interface

- [ ] `MutableLazyValue.get(0)`: it has no body (com/dtteam/dynamictrees/api/lazyvalue/MutableLazyValue.java:7)
- [ ] `MutableLazyValue.reset(1)`: it has no body (com/dtteam/dynamictrees/api/lazyvalue/MutableLazyValue.java:9)
- [ ] `MutableLazyValue.set(1)`: it has no body (com/dtteam/dynamictrees/api/lazyvalue/MutableLazyValue.java:11)
- [ ] `MutableLazyValue.supplied(1)`: it's generic (com/dtteam/dynamictrees/api/lazyvalue/MutableLazyValue.java:13)

### `MutableSuppliedLazyValue`: it's generic

- [ ] `MutableSuppliedLazyValue.MutableSuppliedLazyValue(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/lazyvalue/MutableSuppliedLazyValue.java:8)
- [ ] `MutableSuppliedLazyValue.reset(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/lazyvalue/MutableSuppliedLazyValue.java:12)
- [ ] `MutableSuppliedLazyValue.set(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/lazyvalue/MutableSuppliedLazyValue.java:18)

### `SuppliedLazyValue`: it's generic

- [ ] `SuppliedLazyValue.SuppliedLazyValue(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/lazyvalue/SuppliedLazyValue.java:9)
- [ ] `SuppliedLazyValue.get(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/lazyvalue/SuppliedLazyValue.java:13)

### `BranchConnectionData`: it uses Minecraft's `class_2680`

- [ ] `BranchConnectionData.BranchConnectionData(2)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/api/network/BranchConnectionData.java:9)
- [ ] `BranchConnectionData.getBlockState(0)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/api/network/BranchConnectionData.java:14)
- [ ] `BranchConnectionData.getConnections(0)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/api/network/BranchConnectionData.java:18)

### `BranchDestructionData`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))

- [ ] `BranchDestructionData.bounds`: it uses BlockPosBounds, which isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:46)
- [ ] `BranchDestructionData.BranchDestructionData(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:51)
- [ ] `BranchDestructionData.BranchDestructionData(12)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:69)
- [ ] `BranchDestructionData.merge(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:105)
- [ ] `BranchDestructionData.BranchDestructionData(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:126)
- [ ] `BranchDestructionData.writeToNBT(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:144)
- [ ] `BranchDestructionData.convertBranchesToIntArrays(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:170)
- [ ] `BranchDestructionData.encodeBranchesRadiusPos(3)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:203)
- [ ] `BranchDestructionData.encodeBranchesConnections(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:207)
- [ ] `BranchDestructionData.encodeBranchBlocks(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:220)
- [ ] `BranchDestructionData.getNumBranches(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:224)
- [ ] `BranchDestructionData.getBranchRelPos(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:228)
- [ ] `BranchDestructionData.getRelativeCutPos(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:233)
- [ ] `BranchDestructionData.getBranchRadius(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:237)
- [ ] `BranchDestructionData.decodeBranchRadius(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:241)
- [ ] `BranchDestructionData.getBranchBlockState(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:245)
- [ ] `BranchDestructionData.getConnections(2)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:258)
- [ ] `BranchDestructionData.convertLeavesToIntArray(2)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:266)
- [ ] `BranchDestructionData.getNumLeaves(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:286)
- [ ] `BranchDestructionData.getLeavesRelPos(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:290)
- [ ] `BranchDestructionData.decodeLeavesRelPos(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:295)
- [ ] `BranchDestructionData.getLeavesHydro(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:299)
- [ ] `BranchDestructionData.decodeLeavesHydro(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:303)
- [ ] `BranchDestructionData.getLeavesProperties(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:307)
- [ ] `BranchDestructionData.getLeavesBlockState(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:311)
- [ ] `BranchDestructionData.getAllLeavesWithPos(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:317)
- [ ] `BranchDestructionData.convertEndPointsToIntArray(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:333)
- [ ] `BranchDestructionData.getNumEndpoints(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:346)
- [ ] `BranchDestructionData.getEndPointRelPos(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:350)
- [ ] `BranchDestructionData.getPositions(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:355)
- [ ] `BranchDestructionData.getPositions(2)`: its body couldn't be read: an anonymous class (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:359)
- [ ] `BranchDestructionData.encodeRelBlockPos(1)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:391)
- [ ] `BranchDestructionData.decodeRelPos(1)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/api/network/BranchDestructionData.java:395)

### `BranchDestructionData.PosType`


### `Connections`

- [x] `Connections.Connections(0)` (com/dtteam/dynamictrees/api/network/Connections.java:8)
- [x] `Connections.Connections(1)` (com/dtteam/dynamictrees/api/network/Connections.java:12)
- [ ] `Connections.setRadius(2)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/api/network/Connections.java:16)
- [x] `Connections.getAllRadii(0)` (com/dtteam/dynamictrees/api/network/Connections.java:20)
- [ ] `Connections.setAllRadii(1)`: `this` used as a value (com/dtteam/dynamictrees/api/network/Connections.java:24)

### `MapSignal`: it uses NodeInspector, which isn't translated (it's an interface)

- [ ] `MapSignal.MapSignal(0)`: its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/api/network/MapSignal.java:23)
- [ ] `MapSignal.MapSignal(1)`: its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/api/network/MapSignal.java:27)
- [ ] `MapSignal.run(4)`: its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/api/network/MapSignal.java:32)
- [ ] `MapSignal.returnRun(4)`: its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/api/network/MapSignal.java:40)
- [ ] `MapSignal.getInspectors(0)`: its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/api/network/MapSignal.java:48)
- [ ] `MapSignal.doTrackingVisited(1)`: its class isn't translated (it uses NodeInspector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/api/network/MapSignal.java:52)

### `NodeInspector`: it's an interface

- [ ] `NodeInspector.run(4)`: it has no body (com/dtteam/dynamictrees/api/network/NodeInspector.java:9)
- [ ] `NodeInspector.returnRun(4)`: it has no body (com/dtteam/dynamictrees/api/network/NodeInspector.java:11)

### `RootConnections`: it extends `Connections`

- [ ] `RootConnections.PLACEHOLDER_CONNECTION_LEVELS`: it uses ConnectionLevel, which isn't translated (its constants carry values) (com/dtteam/dynamictrees/api/network/RootConnections.java:8)
- [ ] `RootConnections.RootConnections(0)`: its class isn't translated (it extends `Connections`) (com/dtteam/dynamictrees/api/network/RootConnections.java:13)
- [ ] `RootConnections.RootConnections(1)`: its class isn't translated (it extends `Connections`) (com/dtteam/dynamictrees/api/network/RootConnections.java:18)
- [ ] `RootConnections.getConnectionLevels(0)`: its class isn't translated (it extends `Connections`) (com/dtteam/dynamictrees/api/network/RootConnections.java:23)
- [ ] `RootConnections.setRadius(2)`: its class isn't translated (it extends `Connections`) (com/dtteam/dynamictrees/api/network/RootConnections.java:27)
- [ ] `RootConnections.setConnectionLevel(2)`: its class isn't translated (it extends `Connections`) (com/dtteam/dynamictrees/api/network/RootConnections.java:32)
- [ ] `RootConnections.setConnectionLevels(1)`: its class isn't translated (it extends `Connections`) (com/dtteam/dynamictrees/api/network/RootConnections.java:36)

### `RootConnections.ConnectionLevel`: its constants carry values

- [ ] `RootConnections.ConnectionLevel.ConnectionLevel(1)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/api/network/RootConnections.java:47)
- [ ] `RootConnections.ConnectionLevel.method_15434(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/api/network/RootConnections.java:51)
- [ ] `RootConnections.ConnectionLevel.getYOffset(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/api/network/RootConnections.java:55)

### `AbstractRegistry`: it's generic

- [ ] `AbstractRegistry.REGISTRY_DUMP`: it uses `Marker` (org.apache.logging.log4j) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:23)
- [ ] `AbstractRegistry.STRING_COMPARATOR`: it uses `Comparator` (java.util) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:24)
- [ ] `AbstractRegistry.AbstractRegistry(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:36)
- [ ] `AbstractRegistry.assertValid(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:44)
- [ ] `AbstractRegistry.getErrorMessage(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:55)
- [ ] `AbstractRegistry.registerAll(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:59)
- [ ] `AbstractRegistry.getOptional(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:67)
- [ ] `AbstractRegistry.get(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:72)
- [ ] `AbstractRegistry.get(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:77)
- [ ] `AbstractRegistry.getAsDataResult(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:82)
- [ ] `AbstractRegistry.has(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:89)
- [ ] `AbstractRegistry.getOptional(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:94)
- [ ] `AbstractRegistry.getRegistryNames(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:99)
- [ ] `AbstractRegistry.getType(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:104)
- [ ] `AbstractRegistry.getName(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:109)
- [ ] `AbstractRegistry.isLocked(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:114)
- [ ] `AbstractRegistry.lock(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:119)
- [ ] `AbstractRegistry.unlock(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:127)
- [ ] `AbstractRegistry.runOnNextLock(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:132)
- [ ] `AbstractRegistry.clear(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:137)
- [ ] `AbstractRegistry.clearAll(0)`: it has no body (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:145)
- [ ] `AbstractRegistry.getGetterCodec(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:147)
- [ ] `AbstractRegistry.generateIfValidRunnable(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:152)
- [ ] `AbstractRegistry.postRegistryEvent(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:161)
- [ ] `AbstractRegistry.getComparator(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:166)
- [ ] `AbstractRegistry.dump(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:171)
- [ ] `AbstractRegistry.getAllFor(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:175)
- [ ] `AbstractRegistry.dataGenerationStream(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:183)
- [ ] `AbstractRegistry.iterator(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/AbstractRegistry.java:188)

### `ConcurrentRegistry`: it's generic

- [ ] `ConcurrentRegistry.ConcurrentRegistry(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java:13)
- [ ] `ConcurrentRegistry.ConcurrentRegistry(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java:17)
- [ ] `ConcurrentRegistry.ConcurrentRegistry(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java:21)
- [ ] `ConcurrentRegistry.ConcurrentRegistry(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java:25)
- [ ] `ConcurrentRegistry.register(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java:30)
- [ ] `ConcurrentRegistry.getAll(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java:37)
- [ ] `ConcurrentRegistry.clearAll(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/ConcurrentRegistry.java:42)

### `Registries`

- [ ] `Registries.REGISTRIES`: it uses Registry, which isn't translated (it's an interface) (com/dtteam/dynamictrees/api/registry/Registries.java:17)

### `Registry`: it's an interface

- [ ] `Registry.register(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:14)
- [ ] `Registry.registerAll(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:16)
- [ ] `Registry.has(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:18)
- [ ] `Registry.getOptional(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:20)
- [ ] `Registry.getOptional(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:22)
- [ ] `Registry.get(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:24)
- [ ] `Registry.get(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:26)
- [ ] `Registry.getAsDataResult(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:28)
- [ ] `Registry.getAll(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:30)
- [ ] `Registry.getRegistryNames(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:32)
- [ ] `Registry.getType(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:34)
- [ ] `Registry.getName(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:36)
- [ ] `Registry.isLocked(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:38)
- [ ] `Registry.lock(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:40)
- [ ] `Registry.unlock(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:42)
- [ ] `Registry.runOnNextLock(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:44)
- [ ] `Registry.clear(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:46)
- [ ] `Registry.getGetterCodec(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:48)
- [ ] `Registry.generateIfValidRunnable(3)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:50)
- [ ] `Registry.postRegistryEvent(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:52)
- [ ] `Registry.getComparator(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:54)
- [ ] `Registry.dump(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:56)
- [ ] `Registry.getAllFor(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:58)
- [ ] `Registry.dataGenerationStream(1)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:60)
- [ ] `Registry.iterator(0)`: it has no body (com/dtteam/dynamictrees/api/registry/Registry.java:62)

### `RegistryEntry`: it's generic

- [ ] `RegistryEntry.RegistryEntry(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:19)
- [ ] `RegistryEntry.RegistryEntry(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:22)
- [ ] `RegistryEntry.nullEntry(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:26)
- [ ] `RegistryEntry.isValid(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:31)
- [ ] `RegistryEntry.ifValid(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:35)
- [ ] `RegistryEntry.ifValidElse(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:44)
- [ ] `RegistryEntry.elseIfInvalid(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:54)
- [ ] `RegistryEntry.ifInvalid(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:58)
- [ ] `RegistryEntry.shouldGenerateData(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:67)
- [ ] `RegistryEntry.setGenerateData(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:71)
- [ ] `RegistryEntry.generateStateData(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:75)
- [ ] `RegistryEntry.generateItemModelData(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:78)
- [ ] `RegistryEntry.generateLangData(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:81)
- [ ] `RegistryEntry.getRegistryName(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:84)
- [ ] `RegistryEntry.getTextComponent(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:88)
- [ ] `RegistryEntry.formatComponent(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:92)
- [ ] `RegistryEntry.setRegistryName(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:103)
- [ ] `RegistryEntry.toString(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:108)
- [ ] `RegistryEntry.toLoadDataString(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:113)
- [ ] `RegistryEntry.toReloadDataString(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:117)
- [ ] `RegistryEntry.getString(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryEntry.java:121)

### `RegistryHandler`: it extends `RegistryEntry`

- [ ] `RegistryHandler.REGISTRY`: it uses ConcurrentRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:13)
- [ ] `RegistryHandler.RegistryHandler(1)`: its class isn't translated (it extends `RegistryEntry`) (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:17)
- [ ] `RegistryHandler.RegistryHandler(0)`: its class isn't translated (it extends `RegistryEntry`) (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:21)
- [ ] `RegistryHandler.get(1)`: it uses RegistryHandler, which isn't translated (it extends `RegistryEntry`) (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:25)
- [ ] `RegistryHandler.getOrCorrected(1)`: it uses RegistryHandler, which isn't translated (it extends `RegistryEntry`) (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:29)
- [ ] `RegistryHandler.correctRegistryName(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:34)
- [ ] `RegistryHandler.addBlock(2)`: it's generic (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:42)
- [ ] `RegistryHandler.addItem(2)`: it's generic (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:47)
- [ ] `RegistryHandler.getBlock(1)`: it has no body (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:52)
- [ ] `RegistryHandler.getItem(1)`: it has no body (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:55)
- [ ] `RegistryHandler.putBlock(2)`: it has no body (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:58)
- [ ] `RegistryHandler.putItem(2)`: it has no body (com/dtteam/dynamictrees/api/registry/RegistryHandler.java:60)

### `SimpleRegistry`: it's generic

- [ ] `SimpleRegistry.SimpleRegistry(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java:10)
- [ ] `SimpleRegistry.SimpleRegistry(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java:14)
- [ ] `SimpleRegistry.SimpleRegistry(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java:18)
- [ ] `SimpleRegistry.SimpleRegistry(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java:22)
- [ ] `SimpleRegistry.register(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java:27)
- [ ] `SimpleRegistry.getAll(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java:33)
- [ ] `SimpleRegistry.clearAll(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/SimpleRegistry.java:38)

### `TypedRegistry`: it's generic

- [ ] `TypedRegistry.RESOURCE_LOCATION`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:26)
- [ ] `TypedRegistry.TypedRegistry(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:30)
- [ ] `TypedRegistry.TypedRegistry(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:35)
- [ ] `TypedRegistry.registerType(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:40)
- [ ] `TypedRegistry.hasType(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:44)
- [ ] `TypedRegistry.getType(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:48)
- [ ] `TypedRegistry.getType(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:53)
- [ ] `TypedRegistry.getDefaultType(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:71)
- [ ] `TypedRegistry.postRegistryEvent(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:75)
- [ ] `TypedRegistry.putJsonRegistryName(2)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:81)
- [ ] `TypedRegistry.createDefaultCodec(1)`: it's generic (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:86)
- [ ] `TypedRegistry.newType(1)`: it's generic (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:93)
- [ ] `TypedRegistry.newType(1)`: it's generic (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:97)

### `TypedRegistry.EntryType`: it's generic

- [ ] `TypedRegistry.EntryType.EntryType(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:106)
- [ ] `TypedRegistry.EntryType.getCodec(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:110)
- [ ] `TypedRegistry.EntryType.setRegistry(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:114)
- [ ] `TypedRegistry.EntryType.decode(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/registry/TypedRegistry.java:119)

### `DTResource`: it's generic

- [ ] `DTResource.DTResource(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/DTResource.java:6)
- [ ] `DTResource.map(1)`: it's generic (com/dtteam/dynamictrees/api/resource/DTResource.java:7)
- [ ] `DTResource.location(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/DTResource.java:6)
- [ ] `DTResource.resource(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/DTResource.java:6)

### `AbstractResourceLoader`: it's generic

- [ ] `AbstractResourceLoader.AbstractResourceLoader(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:12)
- [ ] `AbstractResourceLoader.gatherData(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:16)
- [ ] `AbstractResourceLoader.load(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:22)
- [ ] `AbstractResourceLoader.setup(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:28)
- [ ] `AbstractResourceLoader.prepareReload(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:34)
- [ ] `AbstractResourceLoader.reload(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:39)
- [ ] `AbstractResourceLoader.applyOnLoad(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:44)
- [ ] `AbstractResourceLoader.applyOnGatherData(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:48)
- [ ] `AbstractResourceLoader.applyOnSetup(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:52)
- [ ] `AbstractResourceLoader.applyOnReload(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/AbstractResourceLoader.java:56)

### `ApplicationException`: it extends `Exception`

- [ ] `ApplicationException.ApplicationException(1)`: its class isn't translated (it extends `Exception`) (com/dtteam/dynamictrees/api/resource/loading/ApplicationException.java:4)

### `ApplierResourceLoader`: it's an interface

- [ ] `ApplierResourceLoader.registerAppliers(0)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ApplierResourceLoader.java:4)

### `AbstractResourcePreparer`: it's generic

- [x] `AbstractResourcePreparer.LOGGER` (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:14)
- [ ] `AbstractResourcePreparer.AbstractResourcePreparer(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:20)
- [ ] `AbstractResourcePreparer.prepare(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:27)
- [ ] `AbstractResourcePreparer.collectResources(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:35)
- [ ] `AbstractResourcePreparer.readAndPutResources(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:39)
- [ ] `AbstractResourcePreparer.tryReadAndPutResource(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:46)
- [ ] `AbstractResourcePreparer.readAndPutResource(2)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:54)
- [ ] `AbstractResourcePreparer.logError(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:56)
- [ ] `AbstractResourcePreparer.getResourceName(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/AbstractResourcePreparer.java:60)

### `JsonRegistryResourceLoader`: it's generic

- [x] `JsonRegistryResourceLoader.FRUITS` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:21)
- [x] `JsonRegistryResourceLoader.PODS` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:22)
- [x] `JsonRegistryResourceLoader.SPECIES` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:23)
- [x] `JsonRegistryResourceLoader.FAMILY` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:24)
- [x] `JsonRegistryResourceLoader.LEAVES_PROPERTIES` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:25)
- [x] `JsonRegistryResourceLoader.SOIL_PROPERTIES` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:26)
- [ ] `JsonRegistryResourceLoader.JsonRegistryResourceLoader(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:30)
- [ ] `JsonRegistryResourceLoader.JsonRegistryResourceLoader(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:34)
- [ ] `JsonRegistryResourceLoader.applyOnLoad(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:40)
- [ ] `JsonRegistryResourceLoader.loadResourceOnLoad(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:54)
- [ ] `JsonRegistryResourceLoader.postLoadOnLoad(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:60)
- [ ] `JsonRegistryResourceLoader.applyLoadAppliers(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:65)
- [ ] `JsonRegistryResourceLoader.applyOnGatherData(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:73)
- [ ] `JsonRegistryResourceLoader.postLoadOnGatherData(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:87)
- [ ] `JsonRegistryResourceLoader.applyGatherDataAppliers(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:93)
- [ ] `JsonRegistryResourceLoader.applyOnSetup(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:101)
- [ ] `JsonRegistryResourceLoader.loadResourceOnSetup(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:115)
- [ ] `JsonRegistryResourceLoader.applySetupAppliers(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:126)
- [ ] `JsonRegistryResourceLoader.applyOnReload(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:134)
- [ ] `JsonRegistryResourceLoader.loadResourceOnReload(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:150)
- [ ] `JsonRegistryResourceLoader.postLoadOnReload(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:161)
- [ ] `JsonRegistryResourceLoader.applyReloadAppliers(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:168)
- [ ] `JsonRegistryResourceLoader.prepareJson(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:176)
- [ ] `JsonRegistryResourceLoader.loadResource(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:183)
- [ ] `JsonRegistryResourceLoader.loadNewResource(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:195)
- [ ] `JsonRegistryResourceLoader.postLoad(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:204)
- [ ] `JsonRegistryResourceLoader.logException(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:213)
- [ ] `JsonRegistryResourceLoader.applyCommonAppliers(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:217)
- [ ] `JsonRegistryResourceLoader.logError(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:225)
- [ ] `JsonRegistryResourceLoader.logWarning(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:229)

### `JsonRegistryResourceLoader.LoadData`: it uses `R`

- [ ] `JsonRegistryResourceLoader.LoadData.LoadData(0)`: its class isn't translated (it uses `R`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:237)
- [ ] `JsonRegistryResourceLoader.LoadData.LoadData(1)`: its class isn't translated (it uses `R`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:240)
- [ ] `JsonRegistryResourceLoader.LoadData.getResourceName(0)`: its class isn't translated (it uses `R`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:244)
- [ ] `JsonRegistryResourceLoader.LoadData.getResource(0)`: its class isn't translated (it uses `R`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:248)
- [ ] `JsonRegistryResourceLoader.LoadData.wasAlreadyRegistered(0)`: its class isn't translated (it uses `R`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonRegistryResourceLoader.java:252)

### `JsonResourcePreparer`: it extends `AbstractResourcePreparer`

- [x] `JsonResourcePreparer.JSON_EXTENSION` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonResourcePreparer.java:16)
- [ ] `JsonResourcePreparer.JsonResourcePreparer(1)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonResourcePreparer.java:18)
- [ ] `JsonResourcePreparer.JsonResourcePreparer(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonResourcePreparer.java:22)
- [ ] `JsonResourcePreparer.readAndPutResource(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonResourcePreparer.java:26)
- [ ] `JsonResourcePreparer.readResource(1)`: it uses Minecraft's `class_3298` (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonResourcePreparer.java:32)
- [ ] `JsonResourcePreparer.parseJson(1)`: a catch block that uses its exception (com/dtteam/dynamictrees/api/resource/loading/preparation/JsonResourcePreparer.java:38)

### `MultiJsonResourcePreparer`: it extends `AbstractResourcePreparer`

- [x] `MultiJsonResourcePreparer.JSON_EXTENSION` (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:15)
- [ ] `MultiJsonResourcePreparer.MultiJsonResourcePreparer(1)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:17)
- [ ] `MultiJsonResourcePreparer.MultiJsonResourcePreparer(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:21)
- [ ] `MultiJsonResourcePreparer.readAndPutResources(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:25)
- [ ] `MultiJsonResourcePreparer.tryReadAndPutResource(3)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:33)
- [ ] `MultiJsonResourcePreparer.readAndPutResource(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:41)
- [ ] `MultiJsonResourcePreparer.readAndPutResource(3)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:45)
- [ ] `MultiJsonResourcePreparer.computeResourceListIfAbsent(1)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:49)
- [ ] `MultiJsonResourcePreparer.collectResources(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/MultiJsonResourcePreparer.java:53)

### `PreparationException`: it extends `Exception`

- [ ] `PreparationException.PreparationException(1)`: its class isn't translated (it extends `Exception`) (com/dtteam/dynamictrees/api/resource/loading/preparation/PreparationException.java:4)
- [ ] `PreparationException.PreparationException(1)`: its class isn't translated (it extends `Exception`) (com/dtteam/dynamictrees/api/resource/loading/preparation/PreparationException.java:8)

### `ResourcePreparer`: it's an interface

- [ ] `ResourcePreparer.prepare(1)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/preparation/ResourcePreparer.java:7)

### `TextResourcePreparer`: it extends `AbstractResourcePreparer`

- [x] `TextResourcePreparer.EXTENSION` (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java:13)
- [ ] `TextResourcePreparer.TextResourcePreparer(1)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java:15)
- [ ] `TextResourcePreparer.TextResourcePreparer(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java:19)
- [ ] `TextResourcePreparer.readAndPutResource(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java:23)
- [ ] `TextResourcePreparer.readResource(1)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java:29)
- [ ] `TextResourcePreparer.readLines(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java:33)
- [ ] `TextResourcePreparer.offerLine(2)`: its class isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/api/resource/loading/preparation/TextResourcePreparer.java:42)

### `ResourceLoader`: it's an interface

- [ ] `ResourceLoader.load(1)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:8)
- [ ] `ResourceLoader.gatherData(1)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:10)
- [ ] `ResourceLoader.setup(1)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:12)
- [ ] `ResourceLoader.prepareReload(1)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:14)
- [ ] `ResourceLoader.reload(2)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:16)
- [ ] `ResourceLoader.applyOnLoad(2)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:18)
- [ ] `ResourceLoader.applyOnGatherData(2)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:20)
- [ ] `ResourceLoader.applyOnSetup(2)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:22)
- [ ] `ResourceLoader.applyOnReload(2)`: it has no body (com/dtteam/dynamictrees/api/resource/loading/ResourceLoader.java:24)

### `StagedApplierResourceLoader`: it's generic

- [ ] `StagedApplierResourceLoader.StagedApplierResourceLoader(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/StagedApplierResourceLoader.java:16)
- [ ] `StagedApplierResourceLoader.registerAppliers(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/loading/StagedApplierResourceLoader.java:28)

### `StagedApplierResourceLoader.ApplierStage`


### `ResourceAccessor`: it's an interface

- [ ] `ResourceAccessor.getResource(1)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceAccessor.java:9)
- [ ] `ResourceAccessor.getAllResources(0)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceAccessor.java:11)
- [ ] `ResourceAccessor.getAllResources(1)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceAccessor.java:13)
- [ ] `ResourceAccessor.forEach(1)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceAccessor.java:15)
- [ ] `ResourceAccessor.filtered(1)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceAccessor.java:17)
- [ ] `ResourceAccessor.map(1)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceAccessor.java:19)

### `ResourceCollector`: it's an interface

- [ ] `ResourceCollector.put(1)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceCollector.java:10)
- [ ] `ResourceCollector.computeIfAbsent(2)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceCollector.java:12)
- [ ] `ResourceCollector.createAccessor(0)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceCollector.java:14)
- [ ] `ResourceCollector.clear(0)`: it has no body (com/dtteam/dynamictrees/api/resource/ResourceCollector.java:16)
- [ ] `ResourceCollector.unordered(0)`: it's generic (com/dtteam/dynamictrees/api/resource/ResourceCollector.java:18)
- [ ] `ResourceCollector.ordered(0)`: it's generic (com/dtteam/dynamictrees/api/resource/ResourceCollector.java:22)

### `SimpleResourceAccessor`: it's generic

- [ ] `SimpleResourceAccessor.SimpleResourceAccessor(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:17)
- [ ] `SimpleResourceAccessor.getResource(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:22)
- [ ] `SimpleResourceAccessor.getAllResources(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:27)
- [ ] `SimpleResourceAccessor.getAllResources(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:32)
- [ ] `SimpleResourceAccessor.forEach(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:37)
- [ ] `SimpleResourceAccessor.filtered(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:42)
- [ ] `SimpleResourceAccessor.map(1)`: it's generic (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:49)
- [ ] `SimpleResourceAccessor.cloneResources(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:56)
- [ ] `SimpleResourceAccessor.freshMap(0)`: it's generic (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:62)
- [ ] `SimpleResourceAccessor.filterMap(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceAccessor.java:66)

### `SimpleResourceCollector`: it's generic

- [ ] `SimpleResourceCollector.SimpleResourceCollector(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceCollector.java:11)
- [ ] `SimpleResourceCollector.put(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceCollector.java:16)
- [ ] `SimpleResourceCollector.computeIfAbsent(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceCollector.java:21)
- [ ] `SimpleResourceCollector.createAccessor(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceCollector.java:26)
- [ ] `SimpleResourceCollector.clear(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/api/resource/SimpleResourceCollector.java:31)
- [ ] `SimpleResourceCollector.newMap(0)`: it's generic (com/dtteam/dynamictrees/api/resource/SimpleResourceCollector.java:36)

### `TreeResourceManager`: it's an interface

- [ ] `TreeResourceManager.addLoader(1)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:11)
- [ ] `TreeResourceManager.addLoaders(1)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:13)
- [ ] `TreeResourceManager.addLoaderBefore(2)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:15)
- [ ] `TreeResourceManager.addLoaderAfter(2)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:17)
- [ ] `TreeResourceManager.registerAppliers(0)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:19)
- [ ] `TreeResourceManager.load(0)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:21)
- [ ] `TreeResourceManager.gatherData(0)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:23)
- [ ] `TreeResourceManager.setup(0)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:25)
- [ ] `TreeResourceManager.prepareReload(2)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:27)
- [ ] `TreeResourceManager.reload(1)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:29)
- [ ] `TreeResourceManager.addPack(1)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:31)
- [ ] `TreeResourceManager.method_29213(0)`: it has no body (com/dtteam/dynamictrees/api/resource/TreeResourceManager.java:33)

### `TreeResourcePack`: it's an interface

- [ ] `TreeResourcePack.getResource(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/resource/TreeResourcePack.java:11)
- [ ] `TreeResourcePack.listResources(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/resource/TreeResourcePack.java:15)
- [ ] `TreeResourcePack.hasResource(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/resource/TreeResourcePack.java:19)
- [ ] `TreeResourcePack.getNamespaces(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/resource/TreeResourcePack.java:23)

### `ClimateZoneType`: its constants carry values

- [ ] `ClimateZoneType.ClimateZoneType(2)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/api/season/ClimateZoneType.java:13)

### `SeasonGrowthCalculator`: it's an interface

- [ ] `SeasonGrowthCalculator.calcGrowthRate(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonGrowthCalculator.java:6)
- [ ] `SeasonGrowthCalculator.calcSeedDropRate(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonGrowthCalculator.java:8)
- [ ] `SeasonGrowthCalculator.calcFruitProductionRate(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonGrowthCalculator.java:10)
- [ ] `SeasonGrowthCalculator.getPeakFruitProductionSeasonValue(1)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonGrowthCalculator.java:12)

### `SeasonManager`: it's an interface

- [ ] `SeasonManager.updateTick(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:8)
- [ ] `SeasonManager.flushMappings(0)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:10)
- [ ] `SeasonManager.getGrowthFactor(3)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:12)
- [ ] `SeasonManager.getSeedDropFactor(3)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:14)
- [ ] `SeasonManager.getFruitProductionFactor(3)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:16)
- [ ] `SeasonManager.getSeasonValue(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:18)
- [ ] `SeasonManager.getPeakFruitProductionSeasonValue(3)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:20)
- [ ] `SeasonManager.getClimate(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:22)
- [ ] `SeasonManager.shouldSnowMelt(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:24)
- [ ] `SeasonManager.clearCache(1)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonManager.java:26)

### `SeasonProvider`: it's an interface

- [ ] `SeasonProvider.getSeasonValue(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonProvider.java:7)
- [ ] `SeasonProvider.updateTick(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonProvider.java:9)
- [ ] `SeasonProvider.shouldSnowMelt(2)`: it has no body (com/dtteam/dynamictrees/api/season/SeasonProvider.java:11)

### `SeasonType`


### `Emptiable`: it's an interface

- [ ] `Emptiable.getEmptyContainer(0)`: it has no body (com/dtteam/dynamictrees/api/substance/Emptiable.java:7)

### `SubstanceEffect`: it's an interface

- [ ] `SubstanceEffect.apply(2)`: it has no body (com/dtteam/dynamictrees/api/substance/SubstanceEffect.java:7)
- [ ] `SubstanceEffect.update(4)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/substance/SubstanceEffect.java:9)
- [ ] `SubstanceEffect.getName(0)`: it has no body (com/dtteam/dynamictrees/api/substance/SubstanceEffect.java:13)
- [ ] `SubstanceEffect.isLingering(0)`: it has no body (com/dtteam/dynamictrees/api/substance/SubstanceEffect.java:15)

### `SubstanceEffectProvider`: it's an interface

- [ ] `SubstanceEffectProvider.getSubstanceEffect(1)`: it has no body (com/dtteam/dynamictrees/api/substance/SubstanceEffectProvider.java:7)

### `BiomeSuitabilityDecider`: it's an interface

- [ ] `BiomeSuitabilityDecider.getSuitability(4)`: it has no body (com/dtteam/dynamictrees/api/treedata/BiomeSuitabilityDecider.java:11)

### `BiomeSuitabilityDecider.Decision`

- [x] `BiomeSuitabilityDecider.Decision.Decision(0)` (com/dtteam/dynamictrees/api/treedata/BiomeSuitabilityDecider.java:17)
- [ ] `BiomeSuitabilityDecider.Decision.Decision(1)`: unknown name `class_3532` (com/dtteam/dynamictrees/api/treedata/BiomeSuitabilityDecider.java:21)
- [x] `BiomeSuitabilityDecider.Decision.isHandled(0)` (com/dtteam/dynamictrees/api/treedata/BiomeSuitabilityDecider.java:26)
- [x] `BiomeSuitabilityDecider.Decision.getSuitability(0)` (com/dtteam/dynamictrees/api/treedata/BiomeSuitabilityDecider.java:30)

### `BranchShapeState`

- [x] `BranchShapeState.BLOCK_START` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:4)
- [x] `BranchShapeState.TOTAL_STATES` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:5)
- [x] `BranchShapeState.BranchShapeState(7)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)
- [x] `BranchShapeState.toIndex(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:7)
- [x] `BranchShapeState.fromArray(1)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:18)
- [x] `BranchShapeState.down(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)
- [x] `BranchShapeState.up(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)
- [x] `BranchShapeState.north(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)
- [x] `BranchShapeState.south(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)
- [x] `BranchShapeState.west(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)
- [x] `BranchShapeState.east(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)
- [x] `BranchShapeState.core(0)` (com/dtteam/dynamictrees/api/treedata/BranchShapeState.java:3)

### `SurfaceRootShapeState`

- [x] `SurfaceRootShapeState.BLOCK_START` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:4)
- [x] `SurfaceRootShapeState.TOTAL_STATES` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:5)
- [x] `SurfaceRootShapeState.SurfaceRootShapeState(5)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:3)
- [x] `SurfaceRootShapeState.toIndex(0)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:7)
- [x] `SurfaceRootShapeState.fromArray(1)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:16)
- [x] `SurfaceRootShapeState.south(0)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:3)
- [x] `SurfaceRootShapeState.west(0)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:3)
- [x] `SurfaceRootShapeState.north(0)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:3)
- [x] `SurfaceRootShapeState.east(0)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:3)
- [x] `SurfaceRootShapeState.core(0)` (com/dtteam/dynamictrees/api/treedata/SurfaceRootShapeState.java:3)

### `TreePart`: it's an interface

- [ ] `TreePart.getHydrationCell(5)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:18)
- [ ] `TreePart.growSignal(3)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:20)
- [ ] `TreePart.probabilityForBlock(4)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:22)
- [ ] `TreePart.getRadiusForConnection(6)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:24)
- [ ] `TreePart.getRadius(1)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:26)
- [ ] `TreePart.shouldAnalyse(3)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:28)
- [ ] `TreePart.analyse(5)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:30)
- [ ] `TreePart.getFamily(3)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:32)
- [ ] `TreePart.branchSupport(6)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:34)
- [ ] `TreePart.getTreePartType(0)`: it has no body (com/dtteam/dynamictrees/api/treedata/TreePart.java:36)
- [ ] `TreePart.isRootNode(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/treedata/TreePart.java:38)

### `TreePart.TreePartType`


### `BlockPosBounds`: it implements `Iterable`

- [ ] `BlockPosBounds.INVALID`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:12)
- [ ] `BlockPosBounds.BlockPosBounds(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:25)
- [ ] `BlockPosBounds.BlockPosBounds(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:28)
- [ ] `BlockPosBounds.BlockPosBounds(2)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:34)
- [ ] `BlockPosBounds.BlockPosBounds(6)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:43)
- [ ] `BlockPosBounds.BlockPosBounds(2)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:52)
- [ ] `BlockPosBounds.BlockPosBounds(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:61)
- [ ] `BlockPosBounds.BlockPosBounds(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:70)
- [ ] `BlockPosBounds.union(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:75)
- [ ] `BlockPosBounds.union(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:97)
- [ ] `BlockPosBounds.inBounds(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:102)
- [ ] `BlockPosBounds.getMin(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:111)
- [ ] `BlockPosBounds.getMax(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:115)
- [ ] `BlockPosBounds.expand(2)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:119)
- [ ] `BlockPosBounds.shrink(2)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:143)
- [ ] `BlockPosBounds.move(3)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:147)
- [ ] `BlockPosBounds.move(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:157)
- [ ] `BlockPosBounds.expand(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:161)
- [ ] `BlockPosBounds.shrink(1)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:171)
- [ ] `BlockPosBounds.iterator(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:175)
- [ ] `BlockPosBounds.getXSize(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:180)
- [ ] `BlockPosBounds.getYSize(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:184)
- [ ] `BlockPosBounds.getZSize(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:188)
- [ ] `BlockPosBounds.toAABB(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:192)
- [ ] `BlockPosBounds.toString(0)`: its class isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/api/voxmap/BlockPosBounds.java:196)

### `SimpleVoxmap`: it uses Minecraft's `class_2338`

- [ ] `SimpleVoxmap.SimpleVoxmap(3)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:18)
- [ ] `SimpleVoxmap.SimpleVoxmap(4)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:27)
- [ ] `SimpleVoxmap.SimpleVoxmap(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:41)
- [ ] `SimpleVoxmap.SimpleVoxmap(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:46)
- [ ] `SimpleVoxmap.setMapAndCenter(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:51)
- [ ] `SimpleVoxmap.setMap(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:57)
- [ ] `SimpleVoxmap.setCenter(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:62)
- [ ] `SimpleVoxmap.getCenter(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:67)
- [ ] `SimpleVoxmap.getData(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:71)
- [ ] `SimpleVoxmap.getLenX(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:75)
- [ ] `SimpleVoxmap.getLenY(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:79)
- [ ] `SimpleVoxmap.getLenZ(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:83)
- [ ] `SimpleVoxmap.getBounds(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:87)
- [ ] `SimpleVoxmap.blitOp(3)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:97)
- [ ] `SimpleVoxmap.blitReplace(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:120)
- [ ] `SimpleVoxmap.blitMax(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:124)
- [ ] `SimpleVoxmap.blitClear(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:128)
- [ ] `SimpleVoxmap.filter(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:132)
- [ ] `SimpleVoxmap.crop(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:142)
- [ ] `SimpleVoxmap.filter(3)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:157)
- [ ] `SimpleVoxmap.fill(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:165)
- [ ] `SimpleVoxmap.fill(3)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:169)
- [ ] `SimpleVoxmap.calcPos(3)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:173)
- [ ] `SimpleVoxmap.setVoxel(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:177)
- [ ] `SimpleVoxmap.setVoxelOr(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:181)
- [ ] `SimpleVoxmap.setVoxel(4)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:185)
- [ ] `SimpleVoxmap.setVoxelOr(4)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:198)
- [ ] `SimpleVoxmap.getVoxel(2)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:211)
- [ ] `SimpleVoxmap.getVoxel(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:215)
- [ ] `SimpleVoxmap.getVoxel(3)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:219)
- [ ] `SimpleVoxmap.testBounds(3)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:230)
- [ ] `SimpleVoxmap.isYTouched(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:234)
- [ ] `SimpleVoxmap.setYTouched(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:239)
- [ ] `SimpleVoxmap.getAllNonZeroCells(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:246)
- [ ] `SimpleVoxmap.getAllNonZeroCells(1)`: its body couldn't be read: an anonymous class (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:250)
- [ ] `SimpleVoxmap.getAllNonZero(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:306)
- [ ] `SimpleVoxmap.getAllNonZero(1)`: its body couldn't be read: an anonymous class (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:310)
- [ ] `SimpleVoxmap.getTops(0)`: its body couldn't be read: an anonymous class (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:366)
- [ ] `SimpleVoxmap.print(0)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:423)

### `SimpleVoxmap.BlitOp`: it's an interface

- [ ] `SimpleVoxmap.BlitOp.getOp(2)`: it has no body (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:452)

### `SimpleVoxmap.FilterOp`: it's an interface

- [ ] `SimpleVoxmap.FilterOp.getOp(1)`: it has no body (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:457)

### `SimpleVoxmap.VoxmapCell`: it uses Minecraft's `class_2339`

- [ ] `SimpleVoxmap.VoxmapCell.setValue(1)`: its class isn't translated (it uses Minecraft's `class_2339`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:464)
- [ ] `SimpleVoxmap.VoxmapCell.getValue(0)`: its class isn't translated (it uses Minecraft's `class_2339`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:469)
- [ ] `SimpleVoxmap.VoxmapCell.getPos(0)`: its class isn't translated (it uses Minecraft's `class_2339`) (com/dtteam/dynamictrees/api/voxmap/SimpleVoxmap.java:473)

### `BiomePredicate`: it's an interface


### `BiomePropertySelectors`


### `BiomePropertySelectors.Chance`


### `BiomePropertySelectors.ChanceSelector`: it's an interface

- [ ] `BiomePropertySelectors.ChanceSelector.getChance(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:27)

### `BiomePropertySelectors.DensitySelector`: it's an interface

- [ ] `BiomePropertySelectors.DensitySelector.getDensity(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:32)

### `BiomePropertySelectors.FeatureCancellation`: it's an interface

- [ ] `BiomePropertySelectors.FeatureCancellation.cancelFeatures(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:36)
- [ ] `BiomePropertySelectors.FeatureCancellation.cancelUsing(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:38)
- [ ] `BiomePropertySelectors.FeatureCancellation.cancelWithNamespace(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:40)
- [ ] `BiomePropertySelectors.FeatureCancellation.cancelDuring(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:42)

### `BiomePropertySelectors.NoFeatureCancellation`: it implements `FeatureCancellation`

- [ ] `BiomePropertySelectors.NoFeatureCancellation.INSTANCE`: it uses NoFeatureCancellation, which isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:46)
- [ ] `BiomePropertySelectors.NoFeatureCancellation.NoFeatureCancellation(0)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:48)
- [ ] `BiomePropertySelectors.NoFeatureCancellation.cancelFeatures(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:51)
- [ ] `BiomePropertySelectors.NoFeatureCancellation.cancelUsing(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:55)
- [ ] `BiomePropertySelectors.NoFeatureCancellation.cancelWithNamespace(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:59)
- [ ] `BiomePropertySelectors.NoFeatureCancellation.cancelDuring(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:63)

### `BiomePropertySelectors.NormalFeatureCancellation`: it implements `FeatureCancellation`

- [ ] `BiomePropertySelectors.NormalFeatureCancellation.NormalFeatureCancellation(0)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:73)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.NormalFeatureCancellation(3)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:77)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.cancelFeatures(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:83)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.shouldRemovePlacedFeature(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:91)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.cancelUsing(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:96)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.cancelWithNamespace(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:101)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.cancelDuring(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:106)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.cancelDuringDefaultIfNoneSpecified(0)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:111)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.addFrom(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:117)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.replaceFrom(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:123)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.reset(0)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:130)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.getCancellers(0)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:136)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.getNamespaces(0)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:140)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.getDecorationSteps(0)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:144)
- [ ] `BiomePropertySelectors.NormalFeatureCancellation.shouldCancelNamespace(1)`: its class isn't translated (it implements `FeatureCancellation`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:148)

### `BiomePropertySelectors.RandomSpeciesSelector`: it implements `SpeciesSelector`

- [ ] `BiomePropertySelectors.RandomSpeciesSelector.getSize(0)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:157)
- [ ] `BiomePropertySelectors.RandomSpeciesSelector.add(2)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:161)
- [ ] `BiomePropertySelectors.RandomSpeciesSelector.add(1)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:167)
- [ ] `BiomePropertySelectors.RandomSpeciesSelector.getSpecies(3)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:173)

### `BiomePropertySelectors.RandomSpeciesSelector.Entry`: it uses SpeciesSelection, which isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)))

- [ ] `BiomePropertySelectors.RandomSpeciesSelector.Entry.Entry(2)`: its class isn't translated (it uses SpeciesSelection, which isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)))) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:192)

### `BiomePropertySelectors.SpeciesSelection`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))

- [ ] `BiomePropertySelectors.SpeciesSelection.SpeciesSelection(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:203)
- [ ] `BiomePropertySelectors.SpeciesSelection.SpeciesSelection(1)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:208)
- [ ] `BiomePropertySelectors.SpeciesSelection.isHandled(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:213)
- [ ] `BiomePropertySelectors.SpeciesSelection.getSpecies(0)`: its class isn't translated (it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry))) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:217)

### `BiomePropertySelectors.SpeciesSelector`: it's an interface

- [ ] `BiomePropertySelectors.SpeciesSelector.getSpecies(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:224)

### `BiomePropertySelectors.StaticSpeciesSelector`: it implements `SpeciesSelector`

- [ ] `BiomePropertySelectors.StaticSpeciesSelector.StaticSpeciesSelector(1)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:230)
- [ ] `BiomePropertySelectors.StaticSpeciesSelector.StaticSpeciesSelector(1)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:234)
- [ ] `BiomePropertySelectors.StaticSpeciesSelector.StaticSpeciesSelector(0)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:238)
- [ ] `BiomePropertySelectors.StaticSpeciesSelector.getSpecies(3)`: its class isn't translated (it implements `SpeciesSelector`) (com/dtteam/dynamictrees/api/worldgen/BiomePropertySelectors.java:242)

### `FeatureCanceller`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `FeatureCanceller.NULL_CANCELLER`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/api/worldgen/FeatureCanceller.java:10)
- [ ] `FeatureCanceller.REGISTRY`: it uses SimpleRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/api/worldgen/FeatureCanceller.java:16)
- [ ] `FeatureCanceller.FeatureCanceller(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/api/worldgen/FeatureCanceller.java:18)
- [ ] `FeatureCanceller.shouldCancel(2)`: it has no body (com/dtteam/dynamictrees/api/worldgen/FeatureCanceller.java:22)

### `GroundFinder`: it's an interface

- [ ] `GroundFinder.findGround(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/GroundFinder.java:23)
- [ ] `GroundFinder.registerGroundFinder(2)`: it uses Minecraft's `class_5321` (com/dtteam/dynamictrees/api/worldgen/GroundFinder.java:25)
- [ ] `GroundFinder.getGroundFinder(1)`: it uses GroundFinder, which isn't translated (it's an interface) (com/dtteam/dynamictrees/api/worldgen/GroundFinder.java:29)

### `LevelContext`: it uses Minecraft's `class_5321`

- [ ] `LevelContext.LevelContext(4)`: its class isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:20)
- [ ] `LevelContext.dimensionKey(0)`: its class isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:28)
- [ ] `LevelContext.dimensionName(0)`: its class isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:32)
- [ ] `LevelContext.seed(0)`: its class isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:36)
- [ ] `LevelContext.accessor(0)`: its class isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:41)
- [ ] `LevelContext.level(0)`: its class isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:45)
- [ ] `LevelContext.create(1)`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:49)
- [ ] `LevelContext.getServerLevelOrThrow(1)`: it uses Minecraft's `class_3218` (com/dtteam/dynamictrees/api/worldgen/LevelContext.java:69)

### `PoissonDebug`: it's an interface

- [ ] `PoissonDebug.begin(2)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:82)
- [ ] `PoissonDebug.collectSolved(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:84)
- [ ] `PoissonDebug.doEdgeMasking(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:86)
- [ ] `PoissonDebug.maskSolvedDiscs(1)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:88)
- [ ] `PoissonDebug.createRootDisc(2)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:90)
- [ ] `PoissonDebug.gatherUnsolved(2)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:92)
- [ ] `PoissonDebug.updateCount(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:94)
- [ ] `PoissonDebug.pickMasterDisc(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:96)
- [ ] `PoissonDebug.getRadius(4)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:98)
- [ ] `PoissonDebug.findSecondDisc(4)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:100)
- [ ] `PoissonDebug.maskMasterSlave(4)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:102)
- [ ] `PoissonDebug.intersectingList(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:104)
- [ ] `PoissonDebug.findThirdDiscCandidate(5)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:106)
- [ ] `PoissonDebug.thirdCircleCandidateIntersects(6)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:108)
- [ ] `PoissonDebug.findThirdDiscSolved(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:110)
- [ ] `PoissonDebug.solveDiscs(2)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:112)
- [ ] `PoissonDebug.gatherUnsolved2(2)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:114)
- [ ] `PoissonDebug.unsolvable(5)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/api/worldgen/PoissonDebug.java:116)

### `PoissonDiscProvider`: it's an interface

- [ ] `PoissonDiscProvider.getPoissonDiscs(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDiscProvider.java:7)
- [ ] `PoissonDiscProvider.getChunkPoissonData(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDiscProvider.java:9)
- [ ] `PoissonDiscProvider.setChunkPoissonData(4)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDiscProvider.java:11)
- [ ] `PoissonDiscProvider.unloadChunkPoissonData(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/PoissonDiscProvider.java:13)

### `RadiusCoordinator`: it's an interface

- [ ] `RadiusCoordinator.getRadiusAtCoords(2)`: it has no body (com/dtteam/dynamictrees/api/worldgen/RadiusCoordinator.java:4)
- [ ] `RadiusCoordinator.runPass(3)`: it has no body (com/dtteam/dynamictrees/api/worldgen/RadiusCoordinator.java:6)

### `RandomXOR`: it extends Minecraft's `class_7384`

- [x] `RandomXOR.serialVersionUID` (com/dtteam/dynamictrees/api/worldgen/RandomXOR.java:8)
- [ ] `RandomXOR.RandomXOR(0)`: its class isn't translated (it extends Minecraft's `class_7384`) (com/dtteam/dynamictrees/api/worldgen/RandomXOR.java:11)
- [ ] `RandomXOR.RandomXOR(1)`: its class isn't translated (it extends Minecraft's `class_7384`) (com/dtteam/dynamictrees/api/worldgen/RandomXOR.java:15)
- [ ] `RandomXOR.setXOR(1)`: its class isn't translated (it extends Minecraft's `class_7384`) (com/dtteam/dynamictrees/api/worldgen/RandomXOR.java:19)
- [ ] `RandomXOR.setXOR(1)`: its class isn't translated (it extends Minecraft's `class_7384`) (com/dtteam/dynamictrees/api/worldgen/RandomXOR.java:23)
- [ ] `RandomXOR.method_43156(1)`: its class isn't translated (it extends Minecraft's `class_7384`) (com/dtteam/dynamictrees/api/worldgen/RandomXOR.java:27)

### `Ageable`: it's an interface

- [ ] `Ageable.age(5)`: it has no body (com/dtteam/dynamictrees/block/Ageable.java:10)

### `BlockWithDynamicHardness`: it extends Minecraft's `class_2248`

- [ ] `BlockWithDynamicHardness.BlockWithDynamicHardness(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java:15)
- [ ] `BlockWithDynamicHardness.getHardness(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java:23)
- [ ] `BlockWithDynamicHardness.hasTileEntity(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java:27)

### `BlockWithDynamicHardness.DynamicHardnessBlockState`: it extends Minecraft's `class_2680`

- [ ] `BlockWithDynamicHardness.DynamicHardnessBlockState.DynamicHardnessBlockState(3)`: its class isn't translated (it extends Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java:32)
- [ ] `BlockWithDynamicHardness.DynamicHardnessBlockState.method_26214(2)`: its class isn't translated (it extends Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java:38)
- [ ] `BlockWithDynamicHardness.DynamicHardnessBlockState.method_31709(0)`: its class isn't translated (it extends Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/BlockWithDynamicHardness.java:42)

### `BasicBranchBlock`: it extends `BranchBlock`

- [ ] `BasicBranchBlock.RADIUS`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:45)
- [ ] `BasicBranchBlock.WATERLOGGED`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:46)
- [ ] `BasicBranchBlock.BasicBranchBlock(2)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:52)
- [ ] `BasicBranchBlock.BasicBranchBlock(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:56)
- [ ] `BasicBranchBlock.createBranchStates(2)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:61)
- [ ] `BasicBranchBlock.method_9515(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:73)
- [ ] `BasicBranchBlock.branchSupport(6)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:77)
- [ ] `BasicBranchBlock.canFall(0)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:82)
- [ ] `BasicBranchBlock.checkForRot(8)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:87)
- [ ] `BasicBranchBlock.method_9545(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:118)
- [ ] `BasicBranchBlock.method_9559(6)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:122)
- [ ] `BasicBranchBlock.method_10310(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:130)
- [ ] `BasicBranchBlock.getHardness(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:134)
- [ ] `BasicBranchBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:145)
- [ ] `BasicBranchBlock.getFlammability(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:150)
- [ ] `BasicBranchBlock.setFlammability(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:154)
- [ ] `BasicBranchBlock.setFireSpreadSpeed(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:159)
- [ ] `BasicBranchBlock.getHydrationCell(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:164)
- [ ] `BasicBranchBlock.getRadius(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:177)
- [ ] `BasicBranchBlock.setRadius(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:182)
- [ ] `BasicBranchBlock.getStateForRadius(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:192)
- [ ] `BasicBranchBlock.probabilityForBlock(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:197)
- [ ] `BasicBranchBlock.growIntoAir(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:202)
- [ ] `BasicBranchBlock.growSignal(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:225)
- [ ] `BasicBranchBlock.isLadder(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:279)
- [ ] `BasicBranchBlock.getRadiusForConnection(6)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:286)
- [ ] `BasicBranchBlock.getMaxSignalDepth(0)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:291)
- [ ] `BasicBranchBlock.analyse(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:295)
- [ ] `BasicBranchBlock.getStateForDecay(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicBranchBlock.java:332)

### `BasicRootsBlock`: it extends `BranchBlock`

- [x] `BasicRootsBlock.NAME_SUFFIX` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:84)
- [ ] `BasicRootsBlock.RADIUS`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:85)
- [ ] `BasicRootsBlock.LAYER`: it uses Minecraft's `class_2754` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:86)
- [ ] `BasicRootsBlock.WATERLOGGED`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:87)
- [x] `BasicRootsBlock.log` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:88)
- [ ] `BasicRootsBlock.BasicRootsBlock(2)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:93)
- [ ] `BasicRootsBlock.isFullBlock(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:99)
- [ ] `BasicRootsBlock.getFamily(0)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:103)
- [ ] `BasicRootsBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:107)
- [ ] `BasicRootsBlock.getFlammability(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:112)
- [ ] `BasicRootsBlock.setFlammability(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:116)
- [ ] `BasicRootsBlock.setFireSpreadSpeed(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:121)
- [ ] `BasicRootsBlock.method_9515(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:126)
- [ ] `BasicRootsBlock.getHydrationCell(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:130)
- [ ] `BasicRootsBlock.probabilityForBlock(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:135)
- [ ] `BasicRootsBlock.getRadiusForConnection(6)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:140)
- [ ] `BasicRootsBlock.getRadius(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:145)
- [ ] `BasicRootsBlock.getMaxSignalDepth(0)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:150)
- [ ] `BasicRootsBlock.branchSupport(6)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:154)
- [ ] `BasicRootsBlock.setRadius(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:163)
- [ ] `BasicRootsBlock.getStateForRadius(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:188)
- [ ] `BasicRootsBlock.method_9574(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:193)
- [ ] `BasicRootsBlock.method_9573(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:200)
- [ ] `BasicRootsBlock.method_9545(1)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:205)
- [ ] `BasicRootsBlock.method_9559(6)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:209)
- [ ] `BasicRootsBlock.method_10310(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:217)
- [ ] `BasicRootsBlock.method_10311(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:221)
- [ ] `BasicRootsBlock.getLootTableName(0)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:236)
- [ ] `BasicRootsBlock.getLootTable(2)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:241)
- [ ] `BasicRootsBlock.getPrimitiveLog(0)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:246)
- [ ] `BasicRootsBlock.canPlace(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:251)
- [ ] `BasicRootsBlock.method_55765(7)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:256)
- [ ] `BasicRootsBlock.onDestroyedByPlayer(6)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:291)
- [ ] `BasicRootsBlock.futureBreak(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:321)
- [ ] `BasicRootsBlock.destroyBranchFromNode(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:336)
- [ ] `BasicRootsBlock.getHardness(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:389)
- [ ] `BasicRootsBlock.getStateForDecay(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:404)
- [ ] `BasicRootsBlock.method_9594(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:414)
- [ ] `BasicRootsBlock.checkForRot(8)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:421)
- [ ] `BasicRootsBlock.method_9530(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:456)
- [ ] `BasicRootsBlock.method_9549(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:461)
- [ ] `BasicRootsBlock.method_9571(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:474)
- [ ] `BasicRootsBlock.method_26159(4)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:478)
- [ ] `BasicRootsBlock.method_9522(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:482)
- [ ] `BasicRootsBlock.isTransparent(1)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:488)
- [ ] `BasicRootsBlock.canGrowInto(2)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:492)
- [ ] `BasicRootsBlock.isNextToSoil(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:498)
- [ ] `BasicRootsBlock.growIntoAir(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:508)
- [ ] `BasicRootsBlock.growSignal(3)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:522)
- [ ] `BasicRootsBlock.analyse(5)`: its class isn't translated (it extends `BranchBlock`) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:591)

### `BasicRootsBlock.Layer`: its constants carry values

- [ ] `BasicRootsBlock.Layer.Layer(1)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:635)
- [ ] `BasicRootsBlock.Layer.method_15434(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:639)
- [ ] `BasicRootsBlock.Layer.getPrimitive(1)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/branch/BasicRootsBlock.java:644)

### `BranchBlock`: it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)

- [x] `BranchBlock.MAX_RADIUS` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:84)
- [x] `BranchBlock.NAME_SUFFIX` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:85)
- [ ] `BranchBlock.shapeCache`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:86)
- [x] `BranchBlock.destroyMode` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:87)
- [ ] `BranchBlock.BranchBlock(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:93)
- [ ] `BranchBlock.BranchBlock(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:97)
- [ ] `BranchBlock.setCanBeStripped(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:102)
- [ ] `BranchBlock.setFamily(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:107)
- [ ] `BranchBlock.getFamily(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:111)
- [ ] `BranchBlock.getFamily(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:115)
- [ ] `BranchBlock.isSameTree(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:120)
- [ ] `BranchBlock.isSameTree(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:124)
- [ ] `BranchBlock.isSameTree(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:128)
- [ ] `BranchBlock.getPrimitiveLog(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:132)
- [ ] `BranchBlock.isStrippedBranch(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:136)
- [ ] `BranchBlock.branchSupport(6)`: it has no body (com/dtteam/dynamictrees/block/branch/BranchBlock.java:140)
- [ ] `BranchBlock.checkForRot(8)`: it has no body (com/dtteam/dynamictrees/block/branch/BranchBlock.java:143)
- [x] `BranchBlock.setSupport(2)` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:145)
- [x] `BranchBlock.getBranchSupport(1)` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:149)
- [x] `BranchBlock.getLeavesSupport(1)` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:153)
- [ ] `BranchBlock.isNextToBranch(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:157)
- [ ] `BranchBlock.method_55765(7)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:167)
- [ ] `BranchBlock.canBeStripped(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:177)
- [ ] `BranchBlock.stripBranch(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:182)
- [ ] `BranchBlock.stripBranch(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:188)
- [ ] `BranchBlock.method_9574(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:196)
- [ ] `BranchBlock.method_9516(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:200)
- [ ] `BranchBlock.getStateForDecay(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:204)
- [ ] `BranchBlock.method_9651(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:208)
- [ ] `BranchBlock.method_9650(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:223)
- [ ] `BranchBlock.method_9652(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:227)
- [ ] `BranchBlock.getConnectionData(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:238)
- [ ] `BranchBlock.connectToLeaves(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:256)
- [ ] `BranchBlock.method_9604(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:260)
- [ ] `BranchBlock.method_9530(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:264)
- [ ] `BranchBlock.generateNewShape(1)`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:288)
- [ ] `BranchBlock.makeCube(1)`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/block/branch/BranchBlock.java:305)
- [ ] `BranchBlock.getSideConnectionRadius(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:309)
- [ ] `BranchBlock.getRadius(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:315)
- [ ] `BranchBlock.setRadius(5)`: it has no body (com/dtteam/dynamictrees/block/branch/BranchBlock.java:320)
- [ ] `BranchBlock.setRadius(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:322)
- [ ] `BranchBlock.getStateForRadius(1)`: it has no body (com/dtteam/dynamictrees/block/branch/BranchBlock.java:326)
- [ ] `BranchBlock.getMaxRadius(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:328)
- [ ] `BranchBlock.shouldAnalyse(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:332)
- [ ] `BranchBlock.destroyBranchFromNode(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:337)
- [ ] `BranchBlock.getCachedSoilState(3)`: it uses `Pair` (org.apache.commons.lang3.tuple) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:385)
- [ ] `BranchBlock.rot(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:398)
- [ ] `BranchBlock.destroyLeaves(7)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:402)
- [ ] `BranchBlock.canFall(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:462)
- [ ] `BranchBlock.shouldGenerateBranchDrops(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:466)
- [ ] `BranchBlock.getLootTableName(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:470)
- [ ] `BranchBlock.getLootTable(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:474)
- [ ] `BranchBlock.createBranchDrops(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:478)
- [ ] `BranchBlock.getPrimitiveLogs(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:482)
- [ ] `BranchBlock.setPrimitiveLogDrops(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:496)
- [ ] `BranchBlock.futureBreak(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:501)
- [ ] `BranchBlock.onDestroyedByPlayer(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:518)
- [ ] `BranchBlock.sloppyBreak(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:523)
- [ ] `BranchBlock.damageAxe(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:534)
- [ ] `BranchBlock.method_9536(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:538)
- [ ] `BranchBlock.setBlockStateIgnored(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:583)
- [ ] `BranchBlock.method_9576(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:589)
- [ ] `BranchBlock.breakDeliberate(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:593)
- [ ] `BranchBlock.method_55124(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:605)
- [ ] `BranchBlock.getTreePartType(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:630)

### `BranchBlock.ItemStackPos`: it uses Minecraft's `class_1799`

- [ ] `BranchBlock.ItemStackPos.ItemStackPos(2)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/block/branch/BranchBlock.java:639)

### `Musable`: it's an interface

- [ ] `Musable.isMusable(3)`: it has no body (com/dtteam/dynamictrees/block/branch/Musable.java:9)

### `PalmBranchBlock`: it extends `BasicBranchBlock`

- [ ] `PalmBranchBlock.PalmBranchBlock(2)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/PalmBranchBlock.java:18)
- [ ] `PalmBranchBlock.getHydrationCell(5)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/PalmBranchBlock.java:22)
- [ ] `PalmBranchBlock.growIntoAir(4)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/PalmBranchBlock.java:29)

### `SurfaceRootBlock`: it extends Minecraft's `class_2248`

- [x] `SurfaceRootBlock.MAX_RADIUS` (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:42)
- [ ] `SurfaceRootBlock.RADIUS`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:43)
- [ ] `SurfaceRootBlock.GROUNDED`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:44)
- [ ] `SurfaceRootBlock.WATERLOGGED`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:45)
- [ ] `SurfaceRootBlock.shapeCache`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:47)
- [ ] `SurfaceRootBlock.SurfaceRootBlock(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:49)
- [ ] `SurfaceRootBlock.getFamily(0)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:55)
- [ ] `SurfaceRootBlock.method_9574(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:59)
- [ ] `SurfaceRootBlock.method_9515(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:63)
- [ ] `SurfaceRootBlock.getRadius(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:67)
- [ ] `SurfaceRootBlock.setRadius(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:71)
- [ ] `SurfaceRootBlock.getStateForRadius(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:77)
- [ ] `SurfaceRootBlock.getMaxRadius(0)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:81)
- [ ] `SurfaceRootBlock.getRadialHeight(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:85)
- [ ] `SurfaceRootBlock.method_9545(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:89)
- [ ] `SurfaceRootBlock.method_9559(6)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:93)
- [ ] `SurfaceRootBlock.getConnectionData(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:101)
- [ ] `SurfaceRootBlock.method_9530(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:115)
- [ ] `SurfaceRootBlock.generateNewRootShape(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:140)
- [ ] `SurfaceRootBlock.isAirOrWater(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:166)
- [ ] `SurfaceRootBlock.getSideConnectionRadius(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:170)
- [ ] `SurfaceRootBlock.method_9612(6)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:211)
- [ ] `SurfaceRootBlock.method_9588(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:215)
- [ ] `SurfaceRootBlock.method_9576(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:227)
- [ ] `SurfaceRootBlock.updateDisconnectedRoot(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:238)
- [ ] `SurfaceRootBlock.canBlockStay(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:252)

### `SurfaceRootBlock.RootConnection`: it uses ConnectionLevel, which isn't translated (its constants carry values)

- [ ] `SurfaceRootBlock.RootConnection.RootConnection(2)`: its class isn't translated (it uses ConnectionLevel, which isn't translated (its constants carry values)) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:289)
- [ ] `SurfaceRootBlock.RootConnection.toString(0)`: its class isn't translated (it uses ConnectionLevel, which isn't translated (its constants carry values)) (com/dtteam/dynamictrees/block/branch/SurfaceRootBlock.java:294)

### `ThickBranchBlock`: it extends `BasicBranchBlock`

- [x] `ThickBranchBlock.MAX_RADIUS_THICK` (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:32)
- [ ] `ThickBranchBlock.RADIUS_DOUBLE`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:33)
- [ ] `ThickBranchBlock.trunkShapes`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:34)
- [ ] `ThickBranchBlock.ThickBranchBlock(2)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:36)
- [ ] `ThickBranchBlock.precomputeTrunkShapes(0)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:41)
- [ ] `ThickBranchBlock.getTrunkShell(0)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:49)
- [ ] `ThickBranchBlock.method_9515(1)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:53)
- [ ] `ThickBranchBlock.getRadius(1)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:58)
- [ ] `ThickBranchBlock.setRadius(5)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:67)
- [ ] `ThickBranchBlock.method_9612(6)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:74)
- [ ] `ThickBranchBlock.updateTrunkShells(4)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:79)
- [ ] `ThickBranchBlock.getRadiusForConnection(6)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:121)
- [ ] `ThickBranchBlock.getSideConnectionRadius(4)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:126)
- [ ] `ThickBranchBlock.getReplaceability(3)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:138)
- [ ] `ThickBranchBlock.getMaxRadius(0)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:161)
- [ ] `ThickBranchBlock.method_9530(4)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:166)
- [ ] `ThickBranchBlock.isMusable(3)`: its class isn't translated (it extends `BasicBranchBlock`) (com/dtteam/dynamictrees/block/branch/ThickBranchBlock.java:172)

### `ThickBranchBlock.ReplaceableState`


### `TrunkShellBlock`: it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)

- [ ] `TrunkShellBlock.CORE_DIR`: it uses Minecraft's `class_2754` (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:54)
- [ ] `TrunkShellBlock.WATERLOGGED`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:55)
- [ ] `TrunkShellBlock.TrunkShellBlock(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:57)
- [ ] `TrunkShellBlock.method_9515(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:62)
- [ ] `TrunkShellBlock.method_9588(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:66)
- [ ] `TrunkShellBlock.onDestroyedByPlayer(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:77)
- [ ] `TrunkShellBlock.method_9594(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:85)
- [ ] `TrunkShellBlock.method_9556(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:89)
- [ ] `TrunkShellBlock.method_9576(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:92)
- [ ] `TrunkShellBlock.getHardness(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:96)
- [ ] `TrunkShellBlock.method_9573(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:105)
- [ ] `TrunkShellBlock.method_9616(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:109)
- [ ] `TrunkShellBlock.getMuseDir(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:120)
- [ ] `TrunkShellBlock.museDoesNotExist(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:125)
- [ ] `TrunkShellBlock.getMuseUnchecked(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:135)
- [ ] `TrunkShellBlock.getMuseUnchecked(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:140)
- [ ] `TrunkShellBlock.getMuse(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:176)
- [ ] `TrunkShellBlock.getMuse(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:181)
- [ ] `TrunkShellBlock.isValid(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:191)
- [ ] `TrunkShellBlock.scheduleUpdateTick(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:195)
- [ ] `TrunkShellBlock.method_9612(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:201)
- [ ] `TrunkShellBlock.method_9530(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:205)
- [ ] `TrunkShellBlock.method_9574(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:213)
- [ ] `TrunkShellBlock.method_55124(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:219)
- [ ] `TrunkShellBlock.findDetachedMuse(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:223)
- [ ] `TrunkShellBlock.method_9585(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:235)
- [ ] `TrunkShellBlock.method_55765(7)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:245)
- [ ] `TrunkShellBlock.method_55766(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:254)
- [ ] `TrunkShellBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:261)
- [ ] `TrunkShellBlock.getFlammability(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:265)
- [ ] `TrunkShellBlock.isFullBlockShell(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:269)
- [ ] `TrunkShellBlock.isFullBlockShell(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:273)
- [ ] `TrunkShellBlock.isFullBlockShell(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:277)
- [ ] `TrunkShellBlock.method_9516(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:281)
- [ ] `TrunkShellBlock.method_9545(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:285)
- [ ] `TrunkShellBlock.method_9559(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:289)
- [ ] `TrunkShellBlock.method_10310(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:297)
- [ ] `TrunkShellBlock.isWaterLogged(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:301)
- [ ] `TrunkShellBlock.method_9604(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:305)
- [ ] `TrunkShellBlock.addLandingEffects(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:309)

### `TrunkShellBlock.ShellMuse`: it uses Minecraft's `class_2680`

- [ ] `TrunkShellBlock.ShellMuse.ShellMuse(4)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:313)
- [ ] `TrunkShellBlock.ShellMuse.getRadius(0)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:314)
- [ ] `TrunkShellBlock.ShellMuse.state(0)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:313)
- [ ] `TrunkShellBlock.ShellMuse.pos(0)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:313)
- [ ] `TrunkShellBlock.ShellMuse.dir(0)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:313)
- [ ] `TrunkShellBlock.ShellMuse.museOffset(0)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/block/branch/TrunkShellBlock.java:313)

### `CommonVoxelShapes`

- [ ] `CommonVoxelShapes.SHAPES`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:10)
- [ ] `CommonVoxelShapes.SLAB`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:11)
- [ ] `CommonVoxelShapes.TALL_SLAB`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:12)
- [ ] `CommonVoxelShapes.HALF_SLAB`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:13)
- [ ] `CommonVoxelShapes.SAPLING_TRUNK`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:14)
- [ ] `CommonVoxelShapes.FAT_SAPLING_TRUNK`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:15)
- [ ] `CommonVoxelShapes.SAPLING_LEAVES`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:16)
- [ ] `CommonVoxelShapes.FAT_SAPLING_LEAVES`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:17)
- [ ] `CommonVoxelShapes.SLIM_SAPLING_LEAVES`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:18)
- [ ] `CommonVoxelShapes.MUSHROOM_CAP_FLAT`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:19)
- [ ] `CommonVoxelShapes.FAT_MUSHROOM_CAP_FLAT`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:20)
- [ ] `CommonVoxelShapes.MUSHROOM_CAP_ROUND`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:21)
- [ ] `CommonVoxelShapes.FAT_MUSHROOM_CAP_ROUND`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:22)
- [ ] `CommonVoxelShapes.MUSHROOM_RIM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:23)
- [ ] `CommonVoxelShapes.FAT_MUSHROOM_RIM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:24)
- [ ] `CommonVoxelShapes.SAPLING`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:25)
- [ ] `CommonVoxelShapes.SLIM_SAPLING`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:26)
- [ ] `CommonVoxelShapes.FAT_SAPLING`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:27)
- [ ] `CommonVoxelShapes.FLAT_MUSHROOM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:28)
- [ ] `CommonVoxelShapes.FAT_FLAT_MUSHROOM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:29)
- [ ] `CommonVoxelShapes.ROUND_MUSHROOM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:30)
- [ ] `CommonVoxelShapes.FAT_ROUND_MUSHROOM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:31)
- [ ] `CommonVoxelShapes.ROUND_MUSHROOM_RIM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:32)
- [ ] `CommonVoxelShapes.FAT_ROUND_MUSHROOM_RIM`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/CommonVoxelShapes.java:33)

### `DynamicBlockProperties`

- [ ] `DynamicBlockProperties.AGE_PROPERTIES`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/DynamicBlockProperties.java:11)
- [x] `DynamicBlockProperties.defaultAges` (com/dtteam/dynamictrees/block/DynamicBlockProperties.java:20)
- [ ] `DynamicBlockProperties.OFFSET_PROPERTIES`: it uses `Pair` (oshi.util.tuples) (com/dtteam/dynamictrees/block/DynamicBlockProperties.java:21)
- [x] `DynamicBlockProperties.DynamicBlockProperties(0)` (com/dtteam/dynamictrees/block/DynamicBlockProperties.java:25)
- [ ] `DynamicBlockProperties.getOrCreateAge(1)`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/DynamicBlockProperties.java:28)
- [ ] `DynamicBlockProperties.getOrCreateOffset(2)`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/DynamicBlockProperties.java:32)
- [ ] `DynamicBlockProperties.getOrCreateOffset(1)`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/DynamicBlockProperties.java:36)

### `Fruit`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `Fruit.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/fruit/Fruit.java:40)
- [ ] `Fruit.NULL`: it uses Fruit, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:41)
- [ ] `Fruit.REGISTRY`: it uses TypedRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/fruit/Fruit.java:42)
- [ ] `Fruit.Fruit(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:60)
- [ ] `Fruit.setSeasonalFactorGetter(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:64)
- [ ] `Fruit.setFloweringPeriodPredicate(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:68)
- [ ] `Fruit.getBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:72)
- [ ] `Fruit.createBlock(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:80)
- [ ] `Fruit.createBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:84)
- [ ] `Fruit.getDefaultMapColor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:88)
- [ ] `Fruit.getDefaultBlockProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:92)
- [ ] `Fruit.getDefaultBlockProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:96)
- [ ] `Fruit.getAgeProperty(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:100)
- [ ] `Fruit.getMaxAge(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:104)
- [ ] `Fruit.setMaxAge(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:108)
- [ ] `Fruit.setDropCount(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:113)
- [ ] `Fruit.setMaxDropCount(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:118)
- [ ] `Fruit.setMinDropCount(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:122)
- [ ] `Fruit.getBlockShape(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:126)
- [ ] `Fruit.setBlockShapes(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:130)
- [ ] `Fruit.canBoneMeal(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:138)
- [ ] `Fruit.setCanBoneMeal(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:142)
- [ ] `Fruit.getItemStack(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:146)
- [ ] `Fruit.setItemStack(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:155)
- [ ] `Fruit.getGrowthChance(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:159)
- [ ] `Fruit.setGrowthChance(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:163)
- [ ] `Fruit.getRequiredProductionFactor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:167)
- [ ] `Fruit.setRequiredProductionFactor(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:171)
- [ ] `Fruit.seasonalFruitProductionFactor(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:175)
- [ ] `Fruit.isOutOfSeason(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:179)
- [ ] `Fruit.isInFlowerHoldPeriod(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:183)
- [ ] `Fruit.place(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:187)
- [ ] `Fruit.placeDuringWorldGen(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:192)
- [ ] `Fruit.getStateForAge(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:197)
- [ ] `Fruit.getAgeForWorldGen(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:209)
- [ ] `Fruit.performMatureAction(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:215)
- [ ] `Fruit.getMatureAction(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:219)
- [ ] `Fruit.setMatureAction(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:223)
- [ ] `Fruit.shouldGenerateBlockDrops(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:227)
- [ ] `Fruit.getBlockDropsPath(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:231)
- [ ] `Fruit.createBlockDrops(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:235)
- [ ] `Fruit.reset(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/fruit/Fruit.java:253)

### `FruitBlock`: it extends Minecraft's `class_2248`

- [ ] `FruitBlock.FruitBlock(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:35)
- [ ] `FruitBlock.method_9515(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:44)
- [ ] `FruitBlock.method_9530(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:50)
- [ ] `FruitBlock.method_9514(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:54)
- [ ] `FruitBlock.doTick(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:58)
- [ ] `FruitBlock.outOfSeason(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:83)
- [ ] `FruitBlock.tryGrow(6)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:87)
- [ ] `FruitBlock.getGrowthChance(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:96)
- [ ] `FruitBlock.tickMature(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:100)
- [ ] `FruitBlock.performMatureAction(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:104)
- [ ] `FruitBlock.method_9612(6)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:108)
- [ ] `FruitBlock.method_9558(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:116)
- [ ] `FruitBlock.isSupported(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:120)
- [ ] `FruitBlock.drop(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:125)
- [ ] `FruitBlock.method_9574(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:129)
- [ ] `FruitBlock.harvest(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:133)
- [ ] `FruitBlock.method_55765(7)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:142)
- [ ] `FruitBlock.method_55766(5)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:148)
- [ ] `FruitBlock.method_9651(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:152)
- [ ] `FruitBlock.method_9650(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:156)
- [ ] `FruitBlock.method_9652(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:160)
- [ ] `FruitBlock.getAge(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:168)
- [ ] `FruitBlock.getAgeAsPercentage(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:172)
- [ ] `FruitBlock.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:176)
- [ ] `FruitBlock.setAge(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:180)
- [ ] `FruitBlock.method_9516(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/fruit/FruitBlock.java:184)

### `FutureBreakable`: it's an interface

- [ ] `FutureBreakable.futureBreak(4)`: it has no body (com/dtteam/dynamictrees/block/FutureBreakable.java:10)

### `Growable`: it's an interface

- [ ] `Growable.performMatureAction(3)`: it has no body (com/dtteam/dynamictrees/block/Growable.java:10)
- [ ] `Growable.drop(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/block/Growable.java:12)
- [ ] `Growable.rot(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/block/Growable.java:16)
- [ ] `Growable.isSupported(3)`: it has no body (com/dtteam/dynamictrees/block/Growable.java:20)

### `Growable.Info`: it uses Minecraft's `class_1936`

- [ ] `Growable.Info.Info(3)`: its class isn't translated (it uses Minecraft's `class_1936`) (com/dtteam/dynamictrees/block/Growable.java:27)

### `Growable.MatureAction`: its constants carry values

- [ ] `Growable.MatureAction.MatureAction(1)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/Growable.java:41)
- [ ] `Growable.MatureAction.perform(2)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/Growable.java:45)

### `DynamicLeavesBlock`: it extends Minecraft's `class_2397`

- [ ] `DynamicLeavesBlock.SUPPORT_SHAPE`: it uses Minecraft's `class_265` (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:73)
- [ ] `DynamicLeavesBlock.DynamicLeavesBlock(2)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:77)
- [ ] `DynamicLeavesBlock.DynamicLeavesBlock(1)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:83)
- [ ] `DynamicLeavesBlock.method_9542(1)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:91)
- [ ] `DynamicLeavesBlock.getLeavesProperties(0)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:95)
- [ ] `DynamicLeavesBlock.getFamily(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:99)
- [ ] `DynamicLeavesBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:104)
- [ ] `DynamicLeavesBlock.getFlammability(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:108)
- [ ] `DynamicLeavesBlock.method_9605(1)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:112)
- [ ] `DynamicLeavesBlock.method_9594(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:116)
- [ ] `DynamicLeavesBlock.method_9574(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:120)
- [ ] `DynamicLeavesBlock.age(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:124)
- [ ] `DynamicLeavesBlock.method_9514(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:129)
- [ ] `DynamicLeavesBlock.method_9612(6)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:140)
- [ ] `DynamicLeavesBlock.method_9588(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:149)
- [ ] `DynamicLeavesBlock.updateAllLeaves(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:153)
- [ ] `DynamicLeavesBlock.updateLeaves(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:192)
- [ ] `DynamicLeavesBlock.updateHydro(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:212)
- [ ] `DynamicLeavesBlock.appearanceChangesWithHydro(2)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:230)
- [ ] `DynamicLeavesBlock.getLeavesBlockStateForPlacement(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:234)
- [ ] `DynamicLeavesBlock.growLeavesIfLocationIsSuitable(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:238)
- [ ] `DynamicLeavesBlock.isLocationSuitableForNewLeaves(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:253)
- [ ] `DynamicLeavesBlock.hasAdequateLight(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:288)
- [ ] `DynamicLeavesBlock.removeIfInvalid(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:310)
- [ ] `DynamicLeavesBlock.isBottom(2)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:326)
- [ ] `DynamicLeavesBlock.getHydrationLevelFromNeighbors(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:332)
- [ ] `DynamicLeavesBlock.getHydrationCell(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:345)
- [ ] `DynamicLeavesBlock.growSignal(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:353)
- [ ] `DynamicLeavesBlock.needLeaves(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:362)
- [ ] `DynamicLeavesBlock.branchOut(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:372)
- [ ] `DynamicLeavesBlock.probabilityForBlock(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:405)
- [ ] `DynamicLeavesBlock.addLandingEffects(6)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:410)
- [ ] `DynamicLeavesBlock.method_9530(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:414)
- [ ] `DynamicLeavesBlock.method_9549(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:418)
- [ ] `DynamicLeavesBlock.method_25959(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:426)
- [ ] `DynamicLeavesBlock.isMovementVanilla(0)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:430)
- [ ] `DynamicLeavesBlock.isLeavesPassable(0)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:434)
- [ ] `DynamicLeavesBlock.isEntityPassable(1)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:438)
- [ ] `DynamicLeavesBlock.isEntityPassable(1)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:442)
- [ ] `DynamicLeavesBlock.superFallOn(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:452)
- [ ] `DynamicLeavesBlock.method_9554(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:456)
- [ ] `DynamicLeavesBlock.method_9548(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:502)
- [ ] `DynamicLeavesBlock.method_9560(2)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:522)
- [ ] `DynamicLeavesBlock.createLootParams(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:545)
- [ ] `DynamicLeavesBlock.getExactSpecies(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:552)
- [ ] `DynamicLeavesBlock.getRadiusForConnection(6)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:588)
- [ ] `DynamicLeavesBlock.getRadius(1)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:593)
- [ ] `DynamicLeavesBlock.shouldAnalyse(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:598)
- [ ] `DynamicLeavesBlock.analyse(5)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:603)
- [ ] `DynamicLeavesBlock.branchSupport(6)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:608)
- [ ] `DynamicLeavesBlock.getTreePartType(0)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:613)
- [ ] `DynamicLeavesBlock.method_9575(3)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:618)
- [ ] `DynamicLeavesBlock.method_9496(4)`: its class isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/block/leaves/DynamicLeavesBlock.java:622)

### `LeavesProperties`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `LeavesProperties.blockStateGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:74)
- [ ] `LeavesProperties.itemModelGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:75)
- [ ] `LeavesProperties.languageGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:76)
- [ ] `LeavesProperties.CODEC`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:77)
- [ ] `LeavesProperties.NULL`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:81)
- [ ] `LeavesProperties.REGISTRY`: it uses TypedRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:147)
- [x] `LeavesProperties.maxHydro` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:148)
- [x] `LeavesProperties.LEAVES` (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:175)
- [ ] `LeavesProperties.LeavesProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:182)
- [ ] `LeavesProperties.LeavesProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:187)
- [ ] `LeavesProperties.LeavesProperties(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:191)
- [ ] `LeavesProperties.LeavesProperties(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:195)
- [ ] `LeavesProperties.setSeedDropChances(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:205)
- [ ] `LeavesProperties.setSeedDropChances(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:209)
- [ ] `LeavesProperties.getBlockLootTableName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:218)
- [ ] `LeavesProperties.getBlockLootTable(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:222)
- [ ] `LeavesProperties.shouldGenerateBlockDrops(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:226)
- [ ] `LeavesProperties.createBlockDrops(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:230)
- [ ] `LeavesProperties.getLootTableName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:238)
- [ ] `LeavesProperties.getLootTable(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:242)
- [ ] `LeavesProperties.shouldGenerateDrops(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:246)
- [ ] `LeavesProperties.createDrops(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:250)
- [ ] `LeavesProperties.getDrops(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:254)
- [ ] `LeavesProperties.createLootParams(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:264)
- [ ] `LeavesProperties.generateStateData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:273)
- [ ] `LeavesProperties.generateLangData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:278)
- [ ] `LeavesProperties.getPrimitiveLeaves(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:283)
- [ ] `LeavesProperties.getPrimitiveLeavesBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:287)
- [ ] `LeavesProperties.setPrimitiveLeaves(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:291)
- [ ] `LeavesProperties.isOnlyIfLoaded(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:297)
- [ ] `LeavesProperties.setOnlyIfLoaded(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:301)
- [ ] `LeavesProperties.getPrimitiveLeavesItemStack(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:305)
- [ ] `LeavesProperties.setTextureOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:309)
- [ ] `LeavesProperties.setModelOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:313)
- [ ] `LeavesProperties.setLangOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:317)
- [ ] `LeavesProperties.getTexturePath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:321)
- [ ] `LeavesProperties.getModelPath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:325)
- [ ] `LeavesProperties.getLangOverride(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:329)
- [ ] `LeavesProperties.getBlockRegistryName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:333)
- [ ] `LeavesProperties.setBlockRegistryName(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:337)
- [ ] `LeavesProperties.getBlockRegistryNameSuffix(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:342)
- [ ] `LeavesProperties.getDynamicLeavesBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:346)
- [ ] `LeavesProperties.createDynamicLeaves(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:351)
- [ ] `LeavesProperties.generateDynamicLeaves(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:355)
- [ ] `LeavesProperties.setDynamicLeavesState(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:359)
- [ ] `LeavesProperties.getDynamicLeavesState(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:369)
- [ ] `LeavesProperties.getDynamicLeavesState(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:373)
- [ ] `LeavesProperties.getFamily(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:377)
- [ ] `LeavesProperties.setFamily(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:381)
- [ ] `LeavesProperties.getFlammability(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:391)
- [ ] `LeavesProperties.setFlammability(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:395)
- [ ] `LeavesProperties.getFireSpreadSpeed(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:399)
- [ ] `LeavesProperties.setFireSpreadSpeed(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:403)
- [ ] `LeavesProperties.getSmotherLeavesMax(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:407)
- [ ] `LeavesProperties.setSmotherLeavesMax(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:411)
- [ ] `LeavesProperties.getLightRequirement(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:415)
- [ ] `LeavesProperties.setLightRequirement(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:419)
- [ ] `LeavesProperties.shouldAge(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:423)
- [ ] `LeavesProperties.setAgeingConfiguration(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:427)
- [ ] `LeavesProperties.setCanGrowOnGround(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:431)
- [ ] `LeavesProperties.canGrowOnGround(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:435)
- [ ] `LeavesProperties.getCellKit(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:439)
- [ ] `LeavesProperties.setCellKit(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:443)
- [ ] `LeavesProperties.isConnectAnyRadius(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:447)
- [ ] `LeavesProperties.setConnectAnyRadius(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:451)
- [ ] `LeavesProperties.getDefaultMapColor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:455)
- [ ] `LeavesProperties.getDefaultBlockProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:460)
- [ ] `LeavesProperties.getDefaultBlockProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:465)
- [ ] `LeavesProperties.updateTick(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:481)
- [ ] `LeavesProperties.getRadiusForConnection(6)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:485)
- [ ] `LeavesProperties.doRequireShears(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:497)
- [ ] `LeavesProperties.setRequiresShears(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:501)
- [ ] `LeavesProperties.setHasTickParticles(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:505)
- [ ] `LeavesProperties.setWaterResistant(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:509)
- [ ] `LeavesProperties.defaultLeavesTags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:513)
- [ ] `LeavesProperties.isCompatibleLeaves(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:517)
- [ ] `LeavesProperties.maxLeavesRecursion(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:521)
- [ ] `LeavesProperties.setColorNumber(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:525)
- [ ] `LeavesProperties.setColorString(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:529)
- [ ] `LeavesProperties.treeFallColorMultiplier(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:533)
- [ ] `LeavesProperties.foliageColorMultiplier(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:537)
- [ ] `LeavesProperties.processColor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:541)
- [ ] `LeavesProperties.postInitClient(0)`: it uses LeavesProperties.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:570)
- [ ] `LeavesProperties.toReloadDataString(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:574)
- [ ] `LeavesProperties.addGeneratedBlockTags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:587)

### `LeavesProperties.AgeingConfiguration`: its constants carry values

- [ ] `LeavesProperties.AgeingConfiguration.AgeingConfiguration(2)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:606)
- [ ] `LeavesProperties.AgeingConfiguration.shouldAge(1)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/leaves/LeavesProperties.java:611)

### `PalmLeavesProperties`: it extends `LeavesProperties`

- [ ] `PalmLeavesProperties.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:31)
- [x] `PalmLeavesProperties.FROND` (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:36)
- [x] `PalmLeavesProperties.CORE_TOP` (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:37)
- [x] `PalmLeavesProperties.CORE_BOTTOM` (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:38)
- [ ] `PalmLeavesProperties.PalmLeavesProperties(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:40)
- [ ] `PalmLeavesProperties.createDynamicLeaves(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:44)
- [ ] `PalmLeavesProperties.setFrondLoader(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:49)
- [ ] `PalmLeavesProperties.getFrondLoader(0)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:53)
- [ ] `PalmLeavesProperties.generateStateData(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:57)
- [ ] `PalmLeavesProperties.getFrondsModelName(0)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:62)
- [ ] `PalmLeavesProperties.getCoreTopModelName(0)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:66)
- [ ] `PalmLeavesProperties.getCoreBottomModelName(0)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:70)
- [ ] `PalmLeavesProperties.addFrondTextures(2)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:74)
- [ ] `PalmLeavesProperties.addCoreTextures(2)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:79)
- [ ] `PalmLeavesProperties.getCoreTopSmartModelLocation(0)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:84)
- [ ] `PalmLeavesProperties.getCoreBottomSmartModelLocation(0)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:88)

### `PalmLeavesProperties.DynamicPalmLeavesBlock`: it extends `DynamicLeavesBlock`

- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.DIRECTION`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:95)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.hydroSurroundMap`: it uses Surround, which isn't translated (its constants carry values) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:96)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.method_9514(4)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:107)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.DynamicPalmLeavesBlock(2)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:120)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.method_9515(1)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:125)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.getDirectionState(2)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:130)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.getRadiusForConnection(6)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:138)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.branchSupport(6)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:143)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.appearanceChangesWithHydro(2)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:148)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.getLeavesBlockStateForPlacement(5)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:153)
- [ ] `PalmLeavesProperties.DynamicPalmLeavesBlock.method_9571(3)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/PalmLeavesProperties.java:165)

### `ScruffyLeavesProperties`: it extends `LeavesProperties`

- [ ] `ScruffyLeavesProperties.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/leaves/ScruffyLeavesProperties.java:11)
- [ ] `ScruffyLeavesProperties.ScruffyLeavesProperties(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/ScruffyLeavesProperties.java:15)
- [ ] `ScruffyLeavesProperties.setLeafChance(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/ScruffyLeavesProperties.java:19)
- [ ] `ScruffyLeavesProperties.setMaxHydro(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/ScruffyLeavesProperties.java:23)
- [ ] `ScruffyLeavesProperties.createDynamicLeaves(1)`: its body couldn't be read: an anonymous class (com/dtteam/dynamictrees/block/leaves/ScruffyLeavesProperties.java:27)

### `SolidDynamicLeavesBlock`: it extends `DynamicLeavesBlock`

- [ ] `SolidDynamicLeavesBlock.SolidDynamicLeavesBlock(2)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/SolidDynamicLeavesBlock.java:16)
- [ ] `SolidDynamicLeavesBlock.method_25959(3)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/SolidDynamicLeavesBlock.java:20)
- [ ] `SolidDynamicLeavesBlock.method_9549(4)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/SolidDynamicLeavesBlock.java:25)
- [ ] `SolidDynamicLeavesBlock.addLandingEffects(6)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/SolidDynamicLeavesBlock.java:30)
- [ ] `SolidDynamicLeavesBlock.method_9554(5)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/SolidDynamicLeavesBlock.java:35)
- [ ] `SolidDynamicLeavesBlock.method_9548(4)`: its class isn't translated (it extends `DynamicLeavesBlock`) (com/dtteam/dynamictrees/block/leaves/SolidDynamicLeavesBlock.java:40)

### `SolidLeavesProperties`: it extends `LeavesProperties`

- [ ] `SolidLeavesProperties.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/leaves/SolidLeavesProperties.java:10)
- [ ] `SolidLeavesProperties.SolidLeavesProperties(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/SolidLeavesProperties.java:12)
- [ ] `SolidLeavesProperties.getDefaultBlockProperties(0)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/SolidLeavesProperties.java:17)
- [ ] `SolidLeavesProperties.createDynamicLeaves(1)`: its class isn't translated (it extends `LeavesProperties`) (com/dtteam/dynamictrees/block/leaves/SolidLeavesProperties.java:28)

### `WartProperties`: it extends `SolidLeavesProperties`

- [ ] `WartProperties.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/leaves/WartProperties.java:18)
- [ ] `WartProperties.WartProperties(1)`: its class isn't translated (it extends `SolidLeavesProperties`) (com/dtteam/dynamictrees/block/leaves/WartProperties.java:20)
- [ ] `WartProperties.getBlockRegistryNameSuffix(0)`: its class isn't translated (it extends `SolidLeavesProperties`) (com/dtteam/dynamictrees/block/leaves/WartProperties.java:24)
- [ ] `WartProperties.getDefaultBlockProperties(0)`: its class isn't translated (it extends `SolidLeavesProperties`) (com/dtteam/dynamictrees/block/leaves/WartProperties.java:29)
- [ ] `WartProperties.defaultLeavesTags(0)`: its class isn't translated (it extends `SolidLeavesProperties`) (com/dtteam/dynamictrees/block/leaves/WartProperties.java:34)
- [ ] `WartProperties.createBlockDrops(1)`: its class isn't translated (it extends `SolidLeavesProperties`) (com/dtteam/dynamictrees/block/leaves/WartProperties.java:39)
- [ ] `WartProperties.createDrops(1)`: its class isn't translated (it extends `SolidLeavesProperties`) (com/dtteam/dynamictrees/block/leaves/WartProperties.java:44)

### `NullTreePart`: it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)

- [ ] `NullTreePart.getHydrationCell(5)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:20)
- [ ] `NullTreePart.growSignal(3)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:25)
- [ ] `NullTreePart.getRadiusForConnection(6)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:30)
- [ ] `NullTreePart.probabilityForBlock(4)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:42)
- [ ] `NullTreePart.getRadius(1)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:47)
- [ ] `NullTreePart.shouldAnalyse(3)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:52)
- [ ] `NullTreePart.analyse(5)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:57)
- [ ] `NullTreePart.branchSupport(6)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:63)
- [ ] `NullTreePart.getFamily(3)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:68)
- [ ] `NullTreePart.getTreePartType(0)`: its class isn't translated (it implements `TreePart` (com.dtteam.dynamictrees.api.treedata)) (com/dtteam/dynamictrees/block/NullTreePart.java:73)

### `OffsetablePodBlock`: it extends `PodBlock`

- [ ] `OffsetablePodBlock.OffsetablePodBlock(2)`: its class isn't translated (it extends `PodBlock`) (com/dtteam/dynamictrees/block/pod/OffsetablePodBlock.java:19)
- [ ] `OffsetablePodBlock.method_9515(1)`: its class isn't translated (it extends `PodBlock`) (com/dtteam/dynamictrees/block/pod/OffsetablePodBlock.java:23)
- [ ] `OffsetablePodBlock.method_9605(1)`: its class isn't translated (it extends `PodBlock`) (com/dtteam/dynamictrees/block/pod/OffsetablePodBlock.java:32)
- [ ] `OffsetablePodBlock.method_9612(6)`: its class isn't translated (it extends `PodBlock`) (com/dtteam/dynamictrees/block/pod/OffsetablePodBlock.java:44)
- [ ] `OffsetablePodBlock.method_9530(4)`: its class isn't translated (it extends `PodBlock`) (com/dtteam/dynamictrees/block/pod/OffsetablePodBlock.java:56)

### `Pod`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `Pod.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/pod/Pod.java:47)
- [ ] `Pod.NULL`: it uses Pod, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:48)
- [ ] `Pod.REGISTRY`: it uses TypedRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/pod/Pod.java:49)
- [ ] `Pod.Pod(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:70)
- [ ] `Pod.setSeasonalFactorGetter(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:74)
- [ ] `Pod.setFloweringPeriodPredicate(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:78)
- [ ] `Pod.getBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:82)
- [ ] `Pod.createBlock(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:90)
- [ ] `Pod.createBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:94)
- [ ] `Pod.getDefaultMapColor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:98)
- [ ] `Pod.getDefaultBlockProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:102)
- [ ] `Pod.getDefaultBlockProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:106)
- [ ] `Pod.getAgeProperty(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:116)
- [ ] `Pod.getMaxAge(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:120)
- [ ] `Pod.setMaxAge(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:124)
- [ ] `Pod.setDropCount(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:129)
- [ ] `Pod.setMinDropCount(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:134)
- [ ] `Pod.setMaxDropCount(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:138)
- [ ] `Pod.getBlockShape(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:142)
- [ ] `Pod.setBlockShapes(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:146)
- [ ] `Pod.canBoneMeal(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:150)
- [ ] `Pod.setCanBoneMeal(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:154)
- [ ] `Pod.getItemStack(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:158)
- [ ] `Pod.isItem(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:167)
- [ ] `Pod.setItemStack(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:171)
- [ ] `Pod.getGrowthChance(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:175)
- [ ] `Pod.setGrowthChance(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:179)
- [ ] `Pod.getRequiredProductionFactor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:183)
- [ ] `Pod.setRequiredProductionFactor(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:187)
- [ ] `Pod.seasonalFruitProductionFactor(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:191)
- [ ] `Pod.isOutOfSeason(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:195)
- [ ] `Pod.isInFlowerHoldPeriod(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:199)
- [ ] `Pod.place(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:203)
- [ ] `Pod.placeDuringWorldGen(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:208)
- [ ] `Pod.getStateFor(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:213)
- [ ] `Pod.getAgeForWorldGen(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:233)
- [ ] `Pod.performMatureAction(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:239)
- [ ] `Pod.setMatureAction(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:243)
- [ ] `Pod.shouldGenerateBlockDrops(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:247)
- [ ] `Pod.getBlockDropsPath(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:251)
- [ ] `Pod.createBlockDrops(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:255)
- [ ] `Pod.setMaxRadius(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:273)
- [ ] `Pod.setMinRadius(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:277)
- [ ] `Pod.hasVariableOffset(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:281)
- [ ] `Pod.isValidRadius(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:285)
- [ ] `Pod.getOffsetProperty(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:289)
- [ ] `Pod.reset(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/pod/Pod.java:297)

### `Pod.BlockShapeData`: it uses Minecraft's `class_2350`

- [ ] `Pod.BlockShapeData.getShapeFor(2)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/block/pod/Pod.java:326)
- [ ] `Pod.BlockShapeData.setShapesFor(2)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/block/pod/Pod.java:330)

### `PodBlock`: it extends Minecraft's `class_2383`

- [ ] `PodBlock.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/block/pod/PodBlock.java:37)
- [ ] `PodBlock.PodBlock(1)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:40)
- [ ] `PodBlock.PodBlock(2)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:45)
- [ ] `PodBlock.method_53969(0)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:54)
- [ ] `PodBlock.method_9515(1)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:58)
- [ ] `PodBlock.method_9530(4)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:65)
- [ ] `PodBlock.method_9514(4)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:69)
- [ ] `PodBlock.doTick(4)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:73)
- [ ] `PodBlock.outOfSeason(2)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:98)
- [ ] `PodBlock.tryGrow(6)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:102)
- [ ] `PodBlock.getGrowthChance(2)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:111)
- [ ] `PodBlock.tickMature(3)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:115)
- [ ] `PodBlock.performMatureAction(3)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:119)
- [ ] `PodBlock.method_9612(6)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:123)
- [ ] `PodBlock.method_9558(3)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:129)
- [ ] `PodBlock.isSupported(3)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:133)
- [ ] `PodBlock.method_9574(3)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:139)
- [ ] `PodBlock.harvest(3)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:143)
- [ ] `PodBlock.method_55765(7)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:152)
- [ ] `PodBlock.method_55766(5)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:158)
- [ ] `PodBlock.method_9651(3)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:162)
- [ ] `PodBlock.method_9650(4)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:166)
- [ ] `PodBlock.method_9652(4)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:170)
- [ ] `PodBlock.getFacing(1)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:178)
- [ ] `PodBlock.getAge(1)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:182)
- [ ] `PodBlock.getAgeAsPercentage(1)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:186)
- [ ] `PodBlock.getMaxAge(0)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:190)
- [ ] `PodBlock.setAge(4)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:194)
- [ ] `PodBlock.method_9605(1)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:198)
- [ ] `PodBlock.method_9516(2)`: its class isn't translated (it extends Minecraft's `class_2383`) (com/dtteam/dynamictrees/block/pod/PodBlock.java:215)

### `DynamicSaplingBlock`: it extends Minecraft's `class_2248`

- [ ] `DynamicSaplingBlock.SAPLING_REPLACERS`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:35)
- [ ] `DynamicSaplingBlock.DynamicSaplingBlock(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:38)
- [ ] `DynamicSaplingBlock.getSpecies(0)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:52)
- [ ] `DynamicSaplingBlock.getFireSpreadSpeed(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:56)
- [ ] `DynamicSaplingBlock.getFlammability(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:60)
- [ ] `DynamicSaplingBlock.method_9514(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:64)
- [ ] `DynamicSaplingBlock.canSaplingStay(3)`: it uses Minecraft's `class_4538` (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:70)
- [ ] `DynamicSaplingBlock.method_9558(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:82)
- [ ] `DynamicSaplingBlock.method_9651(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:86)
- [ ] `DynamicSaplingBlock.method_9650(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:90)
- [ ] `DynamicSaplingBlock.method_9652(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:94)
- [ ] `DynamicSaplingBlock.method_9573(1)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:105)
- [ ] `DynamicSaplingBlock.method_9612(6)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:109)
- [ ] `DynamicSaplingBlock.dropBlock(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:115)
- [ ] `DynamicSaplingBlock.method_9574(3)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:128)
- [ ] `DynamicSaplingBlock.method_9560(2)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:132)
- [ ] `DynamicSaplingBlock.method_9530(4)`: its class isn't translated (it extends Minecraft's `class_2248`) (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:141)
- [ ] `DynamicSaplingBlock.registerSaplingReplacer(2)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:145)
- [ ] `DynamicSaplingBlock.shouldReplaceSaplingWhenPlaced(1)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:149)
- [ ] `DynamicSaplingBlock.shouldReplaceSaplingWhenGrown(1)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/block/sapling/DynamicSaplingBlock.java:159)

### `PottedSaplingBlock`: it extends Minecraft's `class_2237`

- [ ] `PottedSaplingBlock.FLOWER_POT_AABB`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:40)
- [ ] `PottedSaplingBlock.PottedSaplingBlock(0)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:42)
- [ ] `PottedSaplingBlock.getSpecies(2)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:46)
- [ ] `PottedSaplingBlock.setSpecies(4)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:50)
- [ ] `PottedSaplingBlock.getPotState(2)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:54)
- [ ] `PottedSaplingBlock.setPotState(3)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:58)
- [ ] `PottedSaplingBlock.getTileEntityPottedSapling(2)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:62)
- [ ] `PottedSaplingBlock.method_10123(2)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:68)
- [ ] `PottedSaplingBlock.method_55765(7)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:73)
- [ ] `PottedSaplingBlock.method_55766(5)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:85)
- [ ] `PottedSaplingBlock.removeSaplingFromPot(5)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:95)
- [ ] `PottedSaplingBlock.method_9574(3)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:113)
- [ ] `PottedSaplingBlock.getCloneItemStack(5)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:118)
- [ ] `PottedSaplingBlock.method_9612(6)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:136)
- [ ] `PottedSaplingBlock.onDestroyedByPlayer(6)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:143)
- [ ] `PottedSaplingBlock.method_9556(6)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:151)
- [ ] `PottedSaplingBlock.spawnDrops(2)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:157)
- [ ] `PottedSaplingBlock.method_9516(2)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:164)
- [ ] `PottedSaplingBlock.method_9530(4)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:168)
- [ ] `PottedSaplingBlock.method_53969(0)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:172)
- [ ] `PottedSaplingBlock.method_9604(1)`: its class isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlock.java:176)

### `PottedSaplingBlockEntity`: it extends Minecraft's `class_2586`

- [x] `PottedSaplingBlockEntity.POT_MIMIC_TAG` (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:21)
- [x] `PottedSaplingBlockEntity.SPECIES_TAG` (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:22)
- [ ] `PottedSaplingBlockEntity.PottedSaplingBlockEntity(2)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:26)
- [ ] `PottedSaplingBlockEntity.getSpecies(0)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:30)
- [ ] `PottedSaplingBlockEntity.setSpecies(1)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:34)
- [ ] `PottedSaplingBlockEntity.getPot(0)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:42)
- [ ] `PottedSaplingBlockEntity.setPot(1)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:46)
- [ ] `PottedSaplingBlockEntity.method_16887(1)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:59)
- [ ] `PottedSaplingBlockEntity.method_38235(0)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:65)
- [ ] `PottedSaplingBlockEntity.method_11014(2)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:70)
- [ ] `PottedSaplingBlockEntity.method_11007(2)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/sapling/PottedSaplingBlockEntity.java:83)

### `AerialRootsSoilProperties`: it extends `SoilProperties`

- [ ] `AerialRootsSoilProperties.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:52)
- [ ] `AerialRootsSoilProperties.AerialRootsSoilProperties(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:55)
- [ ] `AerialRootsSoilProperties.setFamily(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:60)
- [ ] `AerialRootsSoilProperties.getFamily(0)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:64)
- [ ] `AerialRootsSoilProperties.createBlock(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:68)
- [ ] `AerialRootsSoilProperties.getSoilState(3)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:73)
- [ ] `AerialRootsSoilProperties.defaultSoilBlockTags(0)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:83)

### `AerialRootsSoilProperties.RootSoilBlock`: it extends `SoilBlock`

- [x] `AerialRootsSoilProperties.RootSoilBlock.MIN_RADIUS` (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:91)
- [x] `AerialRootsSoilProperties.RootSoilBlock.MAX_RADIUS` (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:92)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.RADIUS`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:93)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.WATERLOGGED`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:94)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.RootSoilBlock(2)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:96)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.GetStateFromIndex(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:102)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.getStateIndex(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:107)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.getSoilProperties(0)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:112)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_9515(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:116)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_9545(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:121)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_9559(6)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:125)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.getHardness(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:135)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_9530(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:146)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_9549(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:152)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_26159(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:157)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_25959(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:162)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.getRadius(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:167)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.isStructurallyUnstable(2)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:172)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.fallWithTree(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:184)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.method_9588(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:194)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.updateTree(5)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:207)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.getDecayBlockStateAir(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:217)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.getDecayBlockState(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:221)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.onDestroyedByPlayer(6)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:234)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.dropWholeTree(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:243)
- [ ] `AerialRootsSoilProperties.RootSoilBlock.updateRadius(5)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/AerialRootsSoilProperties.java:287)

### `SoilBlock`: it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)

- [ ] `SoilBlock.soilBlockDecayer`: it uses SoilBlockDecayer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:61)
- [ ] `SoilBlock.FERTILITY`: it uses Minecraft's `class_2758` (com/dtteam/dynamictrees/block/soil/SoilBlock.java:62)
- [ ] `SoilBlock.IS_VARIANT`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/soil/SoilBlock.java:63)
- [ ] `SoilBlock.SoilBlock(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:66)
- [ ] `SoilBlock.getSoilProperties(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:72)
- [ ] `SoilBlock.getPrimitiveSoilBlock(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:76)
- [ ] `SoilBlock.getPrimitiveSoilState(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:80)
- [ ] `SoilBlock.method_23349(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:84)
- [ ] `SoilBlock.method_23350(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:88)
- [ ] `SoilBlock.method_9530(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:92)
- [ ] `SoilBlock.method_9549(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:98)
- [ ] `SoilBlock.method_26159(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:104)
- [ ] `SoilBlock.method_25959(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:110)
- [ ] `SoilBlock.method_9573(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:116)
- [ ] `SoilBlock.method_9579(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:120)
- [ ] `SoilBlock.method_9499(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:124)
- [ ] `SoilBlock.method_9520(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:128)
- [ ] `SoilBlock.method_9505(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:132)
- [ ] `SoilBlock.method_9560(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:136)
- [ ] `SoilBlock.method_9574(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:140)
- [ ] `SoilBlock.getHardness(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:144)
- [ ] `SoilBlock.method_9515(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:149)
- [ ] `SoilBlock.GetStateFromIndex(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:153)
- [ ] `SoilBlock.getStateIndex(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:157)
- [ ] `SoilBlock.method_10123(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:161)
- [ ] `SoilBlock.hasTileEntity(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:166)
- [ ] `SoilBlock.method_9514(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:171)
- [ ] `SoilBlock.getTrunkDirection(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:182)
- [ ] `SoilBlock.updateTree(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:186)
- [ ] `SoilBlock.getDecayBlockState(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:204)
- [ ] `SoilBlock.doDecay(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:208)
- [ ] `SoilBlock.method_9498(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:212)
- [ ] `SoilBlock.method_9572(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:216)
- [ ] `SoilBlock.method_55765(7)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:220)
- [ ] `SoilBlock.destroyTree(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:229)
- [ ] `SoilBlock.destroyTree(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:233)
- [ ] `SoilBlock.destroyTree(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:238)
- [ ] `SoilBlock.onDestroyedByPlayer(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:247)
- [ ] `SoilBlock.onBlockExploded(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:257)
- [ ] `SoilBlock.method_9576(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:263)
- [ ] `SoilBlock.updateRadius(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:268)
- [ ] `SoilBlock.method_9651(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:272)
- [ ] `SoilBlock.method_9650(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:276)
- [ ] `SoilBlock.method_9652(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:280)
- [ ] `SoilBlock.getFertility(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:287)
- [ ] `SoilBlock.setFertility(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:291)
- [ ] `SoilBlock.fertilize(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:299)
- [ ] `SoilBlock.getHydrationCell(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:309)
- [ ] `SoilBlock.growSignal(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:314)
- [ ] `SoilBlock.getRadius(1)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:319)
- [ ] `SoilBlock.getRadiusForConnection(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:324)
- [ ] `SoilBlock.probabilityForBlock(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:329)
- [ ] `SoilBlock.startAnalysis(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:334)
- [ ] `SoilBlock.shouldAnalyse(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:342)
- [ ] `SoilBlock.analyse(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:347)
- [ ] `SoilBlock.method_9612(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:360)
- [ ] `SoilBlock.method_9588(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:368)
- [ ] `SoilBlock.branchSupport(6)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:378)
- [ ] `SoilBlock.getFamily(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:384)
- [ ] `SoilBlock.getMainTrunk(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:390)
- [ ] `SoilBlock.getTileEntitySpecies(2)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:397)
- [ ] `SoilBlock.getSpecies(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:407)
- [ ] `SoilBlock.setSpecies(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:424)
- [ ] `SoilBlock.getTreePartType(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:431)
- [ ] `SoilBlock.isRootNode(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:436)
- [ ] `SoilBlock.colorMultiplier(5)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:441)
- [ ] `SoilBlock.getColorFromBark(0)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:452)
- [ ] `SoilBlock.rootColor(3)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:456)
- [ ] `SoilBlock.fallWithTree(4)`: its class isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/block/soil/SoilBlock.java:460)

### `SoilBlockDecayer`: it's an interface

- [ ] `SoilBlockDecayer.decay(4)`: it has no body (com/dtteam/dynamictrees/block/soil/SoilBlockDecayer.java:10)

### `SoilHelper`

- [x] `SoilHelper.LOGGER` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:18)
- [x] `SoilHelper.DIRT_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:19)
- [x] `SoilHelper.SAND_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:20)
- [x] `SoilHelper.GRAVEL_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:21)
- [x] `SoilHelper.WATER_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:22)
- [x] `SoilHelper.NETHER_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:23)
- [x] `SoilHelper.NETHER_SOIL_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:24)
- [x] `SoilHelper.END_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:25)
- [x] `SoilHelper.MUD_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:26)
- [x] `SoilHelper.TERRACOTTA_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:27)
- [x] `SoilHelper.SLIME_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:28)
- [x] `SoilHelper.FUNGUS_LIKE` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:29)
- [x] `SoilHelper.adjectiveMap` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:30)
- [ ] `SoilHelper.dirtMap`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:31)
- [x] `SoilHelper.createNewAdjective(1)` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:33)
- [x] `SoilHelper.getFlags(1)` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:37)
- [ ] `SoilHelper.addSoilPropertiesToMap(1)`: it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilHelper.java:41)
- [ ] `SoilHelper.registerSoil(2)`: it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilHelper.java:47)
- [ ] `SoilHelper.registerSoil(3)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/block/soil/SoilHelper.java:52)
- [ ] `SoilHelper.registerSoil(3)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/block/soil/SoilHelper.java:69)
- [ ] `SoilHelper.isSoilAcceptable(2)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:73)
- [ ] `SoilHelper.isSoilRegistered(1)`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:87)
- [ ] `SoilHelper.getProperties(1)`: it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilHelper.java:91)
- [x] `SoilHelper.getSoilFlags(1)` (com/dtteam/dynamictrees/block/soil/SoilHelper.java:95)
- [ ] `SoilHelper.applyIfSoilIsAcceptable(3)`: it's generic (com/dtteam/dynamictrees/block/soil/SoilHelper.java:105)

### `SoilProperties`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `SoilProperties.blockStateGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/block/soil/SoilProperties.java:37)
- [ ] `SoilProperties.itemModelGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/block/soil/SoilProperties.java:38)
- [ ] `SoilProperties.languageGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/block/soil/SoilProperties.java:39)
- [ ] `SoilProperties.CODEC`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/block/soil/SoilProperties.java:40)
- [ ] `SoilProperties.NULL_SOIL_PROPERTIES`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/block/soil/SoilProperties.java:44)
- [ ] `SoilProperties.REGISTRY`: it uses TypedRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:64)
- [x] `SoilProperties.ROOTS` (com/dtteam/dynamictrees/block/soil/SoilProperties.java:80)
- [x] `SoilProperties.SOIL_BLOCK` (com/dtteam/dynamictrees/block/soil/SoilProperties.java:81)
- [ ] `SoilProperties.SoilProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:83)
- [ ] `SoilProperties.SoilProperties(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:86)
- [ ] `SoilProperties.SoilProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:94)
- [ ] `SoilProperties.SoilProperties(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:98)
- [ ] `SoilProperties.getPrimitiveSoilBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:103)
- [ ] `SoilProperties.getPrimitiveSoilBlockOptional(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:107)
- [ ] `SoilProperties.setPrimitiveSoilBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:111)
- [ ] `SoilProperties.isValidState(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:121)
- [ ] `SoilProperties.getSoilState(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:125)
- [ ] `SoilProperties.getPrimitiveSoilState(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:130)
- [ ] `SoilProperties.getBlockRegistryNamePrefix(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:134)
- [ ] `SoilProperties.getBlockRegistryName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:138)
- [ ] `SoilProperties.setBlockRegistryName(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:142)
- [ ] `SoilProperties.setBlockRegistryNameIfNull(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:147)
- [ ] `SoilProperties.getBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:153)
- [ ] `SoilProperties.generateBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:157)
- [ ] `SoilProperties.createBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:162)
- [ ] `SoilProperties.setBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:166)
- [ ] `SoilProperties.shouldGenerateBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:170)
- [ ] `SoilProperties.setGenerateBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:174)
- [ ] `SoilProperties.setSubstitute(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:178)
- [ ] `SoilProperties.setFoliageTintIndex(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:182)
- [ ] `SoilProperties.setRootsTintIndex(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:186)
- [ ] `SoilProperties.getDefaultMapColor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:190)
- [ ] `SoilProperties.getDefaultBlockProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:195)
- [ ] `SoilProperties.getDefaultBlockProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:199)
- [ ] `SoilProperties.getSoilFlags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:204)
- [ ] `SoilProperties.setSoilFlags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:208)
- [ ] `SoilProperties.addSoilFlags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:213)
- [ ] `SoilProperties.generateStateData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:218)
- [ ] `SoilProperties.setTextureOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:223)
- [ ] `SoilProperties.getTexturePath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:227)
- [ ] `SoilProperties.setModelOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:231)
- [ ] `SoilProperties.getModelPath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:235)
- [ ] `SoilProperties.getRootsOverlayModelLocation(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:239)
- [ ] `SoilProperties.defaultSoilBlockTags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:243)
- [ ] `SoilProperties.isOnlyIfLoaded(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:247)
- [ ] `SoilProperties.setOnlyIfLoaded(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:251)
- [ ] `SoilProperties.addGeneratedBlockTags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:255)
- [ ] `SoilProperties.toString(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/block/soil/SoilProperties.java:265)

### `SpeciesBlockEntity`: it extends Minecraft's `class_2586`

- [ ] `SpeciesBlockEntity.SpeciesBlockEntity(2)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java:16)
- [ ] `SpeciesBlockEntity.getSpecies(0)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java:20)
- [ ] `SpeciesBlockEntity.setSpecies(1)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java:24)
- [ ] `SpeciesBlockEntity.method_11014(2)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java:29)
- [ ] `SpeciesBlockEntity.method_11007(2)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java:38)
- [ ] `SpeciesBlockEntity.getUpdatePacket(0)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java:42)
- [ ] `SpeciesBlockEntity.method_16887(1)`: its class isn't translated (it extends Minecraft's `class_2586`) (com/dtteam/dynamictrees/block/soil/SpeciesBlockEntity.java:46)

### `SpreadableSoilProperties`: it extends `SoilProperties`

- [ ] `SpreadableSoilProperties.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:27)
- [ ] `SpreadableSoilProperties.setRequiredLight(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:32)
- [ ] `SpreadableSoilProperties.setSpreadItem(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:36)
- [ ] `SpreadableSoilProperties.SpreadableSoilProperties(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:40)
- [ ] `SpreadableSoilProperties.createBlock(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:44)
- [ ] `SpreadableSoilProperties.addSpreadableSoils(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:49)
- [ ] `SpreadableSoilProperties.addSpreadableSoils(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:58)

### `SpreadableSoilProperties.SpreadableSoilBlock`: it extends `SoilBlock`

- [ ] `SpreadableSoilProperties.SpreadableSoilBlock.SpreadableSoilBlock(2)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:63)
- [ ] `SpreadableSoilProperties.SpreadableSoilBlock.getSoilProperties(0)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:67)
- [ ] `SpreadableSoilProperties.SpreadableSoilBlock.getRootyBlock(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:71)
- [ ] `SpreadableSoilProperties.SpreadableSoilBlock.method_55765(7)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:75)
- [ ] `SpreadableSoilProperties.SpreadableSoilBlock.method_9514(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/SpreadableSoilProperties.java:111)

### `WaterSoilProperties`: it extends `SoilProperties`

- [ ] `WaterSoilProperties.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:34)
- [ ] `WaterSoilProperties.WaterSoilProperties(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:36)
- [ ] `WaterSoilProperties.createBlock(1)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:41)
- [ ] `WaterSoilProperties.getDefaultBlockProperties(0)`: its class isn't translated (it extends `SoilProperties`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:46)

### `WaterSoilProperties.SoilWaterBlock`: it extends `SoilBlock`

- [ ] `WaterSoilProperties.SoilWaterBlock.WATER_ROOTS_AABB`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:52)
- [ ] `WaterSoilProperties.SoilWaterBlock.WATERLOGGED`: it uses Minecraft's `class_2746` (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:53)
- [ ] `WaterSoilProperties.SoilWaterBlock.SoilWaterBlock(2)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:55)
- [ ] `WaterSoilProperties.SoilWaterBlock.method_9515(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:60)
- [ ] `WaterSoilProperties.SoilWaterBlock.getRadiusForConnection(6)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:65)
- [ ] `WaterSoilProperties.SoilWaterBlock.method_9574(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:70)
- [ ] `WaterSoilProperties.SoilWaterBlock.getHardness(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:78)
- [ ] `WaterSoilProperties.SoilWaterBlock.method_9530(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:83)
- [ ] `WaterSoilProperties.SoilWaterBlock.method_25959(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:88)
- [ ] `WaterSoilProperties.SoilWaterBlock.method_9616(2)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:93)
- [ ] `WaterSoilProperties.SoilWaterBlock.method_9545(1)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:97)
- [ ] `WaterSoilProperties.SoilWaterBlock.method_9559(6)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:101)
- [ ] `WaterSoilProperties.SoilWaterBlock.getDecayBlockState(3)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:111)
- [ ] `WaterSoilProperties.SoilWaterBlock.getColorFromBark(0)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:118)
- [ ] `WaterSoilProperties.SoilWaterBlock.fallWithTree(4)`: its class isn't translated (it extends `SoilBlock`) (com/dtteam/dynamictrees/block/soil/WaterSoilProperties.java:123)

### `BlockColorMultipliers`

- [ ] `BlockColorMultipliers.colorBase`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/client/BlockColorMultipliers.java:12)
- [ ] `BlockColorMultipliers.register(2)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/client/BlockColorMultipliers.java:14)
- [ ] `BlockColorMultipliers.register(2)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/client/BlockColorMultipliers.java:18)
- [ ] `BlockColorMultipliers.find(1)`: it uses Minecraft's `class_322` (com/dtteam/dynamictrees/client/BlockColorMultipliers.java:26)
- [ ] `BlockColorMultipliers.find(1)`: it uses Minecraft's `class_322` (com/dtteam/dynamictrees/client/BlockColorMultipliers.java:31)
- [ ] `BlockColorMultipliers.cleanUp(0)`: it uses BlockColorMultipliers.colorBase, which isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/client/BlockColorMultipliers.java:36)

### `ParticleHelper`

- [ ] `ParticleHelper.addDustParticle(11)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/client/ParticleHelper.java:19)
- [ ] `ParticleHelper.spawnParticles(5)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/client/ParticleHelper.java:30)
- [ ] `ParticleHelper.spawnParticles(7)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/client/ParticleHelper.java:34)
- [ ] `ParticleHelper.getFoliageColor(4)`: it uses LeavesProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/client/ParticleHelper.java:45)
- [ ] `ParticleHelper.crushLeavesBlock(4)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/client/ParticleHelper.java:49)

### `RetexturedBakedQuad`: it extends Minecraft's `class_777`

- [ ] `RetexturedBakedQuad.RetexturedBakedQuad(2)`: its class isn't translated (it extends Minecraft's `class_777`) (com/dtteam/dynamictrees/client/RetexturedBakedQuad.java:12)
- [ ] `RetexturedBakedQuad.remapQuad(0)`: its class isn't translated (it extends Minecraft's `class_777`) (com/dtteam/dynamictrees/client/RetexturedBakedQuad.java:24)
- [ ] `RetexturedBakedQuad.method_35788(0)`: its class isn't translated (it extends Minecraft's `class_777`) (com/dtteam/dynamictrees/client/RetexturedBakedQuad.java:37)
- [ ] `RetexturedBakedQuad.getUnInterpolatedU(2)`: it uses Minecraft's `class_1058` (com/dtteam/dynamictrees/client/RetexturedBakedQuad.java:41)
- [ ] `RetexturedBakedQuad.getUnInterpolatedV(2)`: it uses Minecraft's `class_1058` (com/dtteam/dynamictrees/client/RetexturedBakedQuad.java:46)

### `SoundInstanceHandler`

- [ ] `SoundInstanceHandler.instances`: it uses Minecraft's `class_1113` (com/dtteam/dynamictrees/client/SoundInstanceHandler.java:15)
- [ ] `SoundInstanceHandler.playSoundInstance(4)`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/client/SoundInstanceHandler.java:17)
- [ ] `SoundInstanceHandler.playSoundInstance(6)`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/client/SoundInstanceHandler.java:21)
- [ ] `SoundInstanceHandler.stopSoundInstance(1)`: it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/client/SoundInstanceHandler.java:27)

### `TextureHelper`

- [x] `TextureHelper.compose(4)` (com/dtteam/dynamictrees/client/TextureHelper.java:9)
- [x] `TextureHelper.compose(1)` (com/dtteam/dynamictrees/client/TextureHelper.java:16)
- [x] `TextureHelper.decompose(1)` (com/dtteam/dynamictrees/client/TextureHelper.java:20)
- [x] `TextureHelper.alpha(1)` (com/dtteam/dynamictrees/client/TextureHelper.java:24)
- [x] `TextureHelper.blue(1)` (com/dtteam/dynamictrees/client/TextureHelper.java:28)
- [x] `TextureHelper.green(1)` (com/dtteam/dynamictrees/client/TextureHelper.java:32)
- [x] `TextureHelper.red(1)` (com/dtteam/dynamictrees/client/TextureHelper.java:36)
- [x] `TextureHelper.avgColors(1)` (com/dtteam/dynamictrees/client/TextureHelper.java:40)

### `TextureHelper.PixelBuffer`

- [ ] `TextureHelper.PixelBuffer.PixelBuffer(2)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/client/TextureHelper.java:68)
- [ ] `TextureHelper.PixelBuffer.PixelBuffer(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/client/TextureHelper.java:74)
- [ ] `TextureHelper.PixelBuffer.PixelBuffer(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/client/TextureHelper.java:86)
- [ ] `TextureHelper.PixelBuffer.PixelBuffer(2)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/client/TextureHelper.java:98)
- [ ] `TextureHelper.PixelBuffer.PixelBuffer(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/client/TextureHelper.java:102)
- [ ] `TextureHelper.PixelBuffer.toNativeImage(0)`: it uses Minecraft's `class_1011` (com/dtteam/dynamictrees/client/TextureHelper.java:108)
- [x] `TextureHelper.PixelBuffer.calcPos(2)` (com/dtteam/dynamictrees/client/TextureHelper.java:120)
- [x] `TextureHelper.PixelBuffer.getPixel(2)` (com/dtteam/dynamictrees/client/TextureHelper.java:124)
- [x] `TextureHelper.PixelBuffer.setPixel(3)` (com/dtteam/dynamictrees/client/TextureHelper.java:128)
- [x] `TextureHelper.PixelBuffer.blit(3)` (com/dtteam/dynamictrees/client/TextureHelper.java:134)
- [x] `TextureHelper.PixelBuffer.blit(4)` (com/dtteam/dynamictrees/client/TextureHelper.java:138)
- [x] `TextureHelper.PixelBuffer.averageColor(0)` (com/dtteam/dynamictrees/client/TextureHelper.java:177)
- [x] `TextureHelper.PixelBuffer.grayScale(0)` (com/dtteam/dynamictrees/client/TextureHelper.java:181)
- [ ] `TextureHelper.PixelBuffer.fill(1)`: it calls `Arrays` (java.util).fill (com/dtteam/dynamictrees/client/TextureHelper.java:192)

### `ThickBranchRingsSource`: it implements Minecraft's `class_7948`

- [ ] `ThickBranchRingsSource.ID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java:17)
- [ ] `ThickBranchRingsSource.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java:18)
- [ ] `ThickBranchRingsSource.TYPE`: it uses Minecraft's `class_7951` (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java:22)
- [ ] `ThickBranchRingsSource.ThickBranchRingsSource(1)`: its class isn't translated (it implements Minecraft's `class_7948`) (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java:25)
- [ ] `ThickBranchRingsSource.method_47673(2)`: its class isn't translated (it implements Minecraft's `class_7948`) (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java:29)
- [ ] `ThickBranchRingsSource.method_47672(0)`: its class isn't translated (it implements Minecraft's `class_7948`) (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java:43)
- [ ] `ThickBranchRingsSource.setType(1)`: it uses Minecraft's `class_7951` (com/dtteam/dynamictrees/client/ThickBranchRingsSource.java:47)

### `ThickBranchRingsSprite`: it extends Minecraft's `class_7764`

- [x] `ThickBranchRingsSprite.RESOLUTION` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:12)
- [x] `ThickBranchRingsSprite.LAYERS` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:13)
- [x] `ThickBranchRingsSprite.CORNERS_XY` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:14)
- [ ] `ThickBranchRingsSprite.ThickBranchRingsSprite(2)`: its class isn't translated (it extends Minecraft's `class_7764`) (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:16)
- [ ] `ThickBranchRingsSprite.getFrameSize(1)`: it uses Minecraft's `class_7771` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:20)
- [x] `ThickBranchRingsSprite.centerCorner(0)` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:24)
- [ ] `ThickBranchRingsSprite.processImage(1)`: it uses Minecraft's `class_1011` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:28)
- [ ] `ThickBranchRingsSprite.createMajorTexture(1)`: it calls ThickBranchRingsSprite.createBarklessAntecedent, which isn't translated (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:34)
- [ ] `ThickBranchRingsSprite.compileRingCornersAndEdges(4)`: it creates a PixelBuffer with a constructor that isn't translated (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:51)
- [ ] `ThickBranchRingsSprite.fillRings(3)`: unknown name `class_2353` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:62)
- [x] `ThickBranchRingsSprite.fillCorners(3)` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:94)
- [x] `ThickBranchRingsSprite.fillCenter(3)` (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:109)
- [ ] `ThickBranchRingsSprite.compileBarkCornersAndEdges(4)`: it creates a PixelBuffer with a constructor that isn't translated (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:113)
- [ ] `ThickBranchRingsSprite.fillBarkBorder(4)`: `++` inside an expression (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:124)
- [ ] `ThickBranchRingsSprite.createBarklessAntecedent(1)`: it creates a PixelBuffer with a constructor that isn't translated (com/dtteam/dynamictrees/client/ThickBranchRingsSprite.java:145)

### `Tooltips`

- [ ] `Tooltips.applySeasonalTooltips(3)`: it uses Minecraft's `class_2561` (com/dtteam/dynamictrees/client/Tooltips.java:9)

### `CommandConstants`

- [x] `CommandConstants.COMMAND` (com/dtteam/dynamictrees/command/CommandConstants.java:10)
- [x] `CommandConstants.COMMAND_ALIAS` (com/dtteam/dynamictrees/command/CommandConstants.java:11)
- [x] `CommandConstants.GET_TREE` (com/dtteam/dynamictrees/command/CommandConstants.java:12)
- [x] `CommandConstants.GET_ROOTS` (com/dtteam/dynamictrees/command/CommandConstants.java:13)
- [x] `CommandConstants.GROW_PULSE` (com/dtteam/dynamictrees/command/CommandConstants.java:14)
- [x] `CommandConstants.KILL_TREE` (com/dtteam/dynamictrees/command/CommandConstants.java:15)
- [x] `CommandConstants.SPECIES_LIST` (com/dtteam/dynamictrees/command/CommandConstants.java:16)
- [x] `CommandConstants.FERTILITY` (com/dtteam/dynamictrees/command/CommandConstants.java:17)
- [x] `CommandConstants.SET_TREE` (com/dtteam/dynamictrees/command/CommandConstants.java:18)
- [x] `CommandConstants.ROTATE_JO_CODE` (com/dtteam/dynamictrees/command/CommandConstants.java:19)
- [x] `CommandConstants.CREATE_STAFF` (com/dtteam/dynamictrees/command/CommandConstants.java:20)
- [x] `CommandConstants.SET_COORD_XOR` (com/dtteam/dynamictrees/command/CommandConstants.java:21)
- [x] `CommandConstants.CREATE_TRANSFORM_POTION` (com/dtteam/dynamictrees/command/CommandConstants.java:22)
- [x] `CommandConstants.TRANSFORM` (com/dtteam/dynamictrees/command/CommandConstants.java:23)
- [x] `CommandConstants.CLEAR_ORPHANED` (com/dtteam/dynamictrees/command/CommandConstants.java:24)
- [x] `CommandConstants.PURGE_TREES` (com/dtteam/dynamictrees/command/CommandConstants.java:25)
- [x] `CommandConstants.LOCATION` (com/dtteam/dynamictrees/command/CommandConstants.java:26)
- [x] `CommandConstants.JO_CODE` (com/dtteam/dynamictrees/command/CommandConstants.java:27)
- [x] `CommandConstants.ROOTS_JO_CODE` (com/dtteam/dynamictrees/command/CommandConstants.java:28)
- [x] `CommandConstants.TURNS` (com/dtteam/dynamictrees/command/CommandConstants.java:29)
- [x] `CommandConstants.SPECIES` (com/dtteam/dynamictrees/command/CommandConstants.java:30)
- [x] `CommandConstants.RAW` (com/dtteam/dynamictrees/command/CommandConstants.java:31)
- [x] `CommandConstants.DEFAULT_JO_CODE` (com/dtteam/dynamictrees/command/CommandConstants.java:32)
- [x] `CommandConstants.DEFAULT_ROOTS_JO_CODE` (com/dtteam/dynamictrees/command/CommandConstants.java:33)
- [x] `CommandConstants.DEFAULT_TURNS` (com/dtteam/dynamictrees/command/CommandConstants.java:34)
- [ ] `CommandConstants.TURNS_SUGGESTIONS`: it uses Minecraft's `SuggestionProvider` (com/dtteam/dynamictrees/command/CommandConstants.java:35)
- [ ] `CommandConstants.FERTILITY_SUGGESTIONS`: it uses Minecraft's `SuggestionProvider` (com/dtteam/dynamictrees/command/CommandConstants.java:38)

### `CommandHelper`

- [ ] `CommandHelper.posComponent(1)`: it uses Minecraft's `class_2561` (com/dtteam/dynamictrees/command/CommandHelper.java:8)
- [ ] `CommandHelper.posComponent(2)`: it uses Minecraft's `class_2561` (com/dtteam/dynamictrees/command/CommandHelper.java:12)
- [ ] `CommandHelper.colour(2)`: it uses Minecraft's `class_2561` (com/dtteam/dynamictrees/command/CommandHelper.java:16)

### `DTCommand`

- [ ] `DTCommand.DTCommand(0)`: it creates a GetTreeCommand, which isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/DTCommand.java:28)
- [ ] `DTCommand.registerDTCommand(1)`: it uses Minecraft's `CommandDispatcher` (com/dtteam/dynamictrees/command/DTCommand.java:46)

### `HexColorArgument`: it implements Minecraft's `ArgumentType`

- [ ] `HexColorArgument.COLOR_INVALID`: it uses Minecraft's `DynamicCommandExceptionType` (com/dtteam/dynamictrees/command/HexColorArgument.java:17)
- [ ] `HexColorArgument.hex(0)`: it uses HexColorArgument, which isn't translated (it implements Minecraft's `ArgumentType`) (com/dtteam/dynamictrees/command/HexColorArgument.java:21)
- [ ] `HexColorArgument.getHexCode(2)`: it calls Minecraft's FabricClientCommandSource.getArgument (no Horizon helper for it yet) (com/dtteam/dynamictrees/command/HexColorArgument.java:25)
- [ ] `HexColorArgument.parse(1)`: its class isn't translated (it implements Minecraft's `ArgumentType`) (com/dtteam/dynamictrees/command/HexColorArgument.java:29)
- [ ] `HexColorArgument.listSuggestions(2)`: it's generic (com/dtteam/dynamictrees/command/HexColorArgument.java:42)

### `ChunkBasedCommand`: it extends `SubCommand`

- [x] `ChunkBasedCommand.RADIUS` (com/dtteam/dynamictrees/command/subcommand/ChunkBasedCommand.java:13)
- [x] `ChunkBasedCommand.DEFAULT_RADIUS` (com/dtteam/dynamictrees/command/subcommand/ChunkBasedCommand.java:14)
- [ ] `ChunkBasedCommand.register(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/ChunkBasedCommand.java:16)
- [ ] `ChunkBasedCommand.getChunkPos(1)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/ChunkBasedCommand.java:28)
- [ ] `ChunkBasedCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/ChunkBasedCommand.java:32)
- [ ] `ChunkBasedCommand.processChunk(4)`: it has no body (com/dtteam/dynamictrees/command/subcommand/ChunkBasedCommand.java:57)

### `ClearOrphanedCommand`: it extends `ChunkBasedCommand`

- [ ] `ClearOrphanedCommand.getName(0)`: its class isn't translated (it extends `ChunkBasedCommand`) (com/dtteam/dynamictrees/command/subcommand/ClearOrphanedCommand.java:10)
- [ ] `ClearOrphanedCommand.getPermissionLevel(0)`: its class isn't translated (it extends `ChunkBasedCommand`) (com/dtteam/dynamictrees/command/subcommand/ClearOrphanedCommand.java:15)
- [ ] `ClearOrphanedCommand.processChunk(4)`: its class isn't translated (it extends `ChunkBasedCommand`) (com/dtteam/dynamictrees/command/subcommand/ClearOrphanedCommand.java:20)

### `CreateStaffCommand`: it extends `SubCommand`

- [x] `CreateStaffCommand.COLOR` (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:24)
- [x] `CreateStaffCommand.READ_ONLY` (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:25)
- [x] `CreateStaffCommand.MAX_USES` (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:26)
- [x] `CreateStaffCommand.DEFAULT_COLOUR` (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:27)
- [x] `CreateStaffCommand.DEFAULT_READ_ONLY` (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:28)
- [x] `CreateStaffCommand.DEFAULT_MAX_USES` (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:29)
- [ ] `CreateStaffCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:31)
- [ ] `CreateStaffCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:36)
- [ ] `CreateStaffCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:41)
- [ ] `CreateStaffCommand.spawnStaff(7)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/CreateStaffCommand.java:118)

### `FertilityCommand`: it extends `SubCommand`

- [x] `FertilityCommand.FERTILITY` (com/dtteam/dynamictrees/command/subcommand/FertilityCommand.java:18)
- [ ] `FertilityCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/FertilityCommand.java:20)
- [ ] `FertilityCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/FertilityCommand.java:25)
- [ ] `FertilityCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/FertilityCommand.java:30)
- [ ] `FertilityCommand.getFertility(3)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/FertilityCommand.java:54)
- [ ] `FertilityCommand.setFertility(3)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/FertilityCommand.java:70)

### `GetRootsCommand`: it extends `SubCommand`

- [x] `GetRootsCommand.CODE_RAW` (com/dtteam/dynamictrees/command/subcommand/GetRootsCommand.java:16)
- [ ] `GetRootsCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetRootsCommand.java:18)
- [ ] `GetRootsCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetRootsCommand.java:23)
- [ ] `GetRootsCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetRootsCommand.java:28)
- [ ] `GetRootsCommand.getRoots(3)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetRootsCommand.java:37)

### `GetTreeCommand`: it extends `SubCommand`

- [x] `GetTreeCommand.CODE_RAW` (com/dtteam/dynamictrees/command/subcommand/GetTreeCommand.java:16)
- [ ] `GetTreeCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetTreeCommand.java:18)
- [ ] `GetTreeCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetTreeCommand.java:23)
- [ ] `GetTreeCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetTreeCommand.java:28)
- [ ] `GetTreeCommand.getTree(3)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GetTreeCommand.java:37)

### `GrowPulseCommand`: it extends `SubCommand`

- [x] `GrowPulseCommand.NUMBER` (com/dtteam/dynamictrees/command/subcommand/GrowPulseCommand.java:19)
- [x] `GrowPulseCommand.NUMBER_SUGGESTIONS` (com/dtteam/dynamictrees/command/subcommand/GrowPulseCommand.java:20)
- [ ] `GrowPulseCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GrowPulseCommand.java:22)
- [ ] `GrowPulseCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GrowPulseCommand.java:27)
- [ ] `GrowPulseCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GrowPulseCommand.java:32)
- [ ] `GrowPulseCommand.sendGrowPulse(3)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/GrowPulseCommand.java:47)

### `KillTreeCommand`: it extends `SubCommand`

- [ ] `KillTreeCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/KillTreeCommand.java:14)
- [ ] `KillTreeCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/KillTreeCommand.java:19)
- [ ] `KillTreeCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/KillTreeCommand.java:24)
- [ ] `KillTreeCommand.killTree(2)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/KillTreeCommand.java:29)

### `PurgeTreesCommand`: it extends `ChunkBasedCommand`

- [ ] `PurgeTreesCommand.getName(0)`: its class isn't translated (it extends `ChunkBasedCommand`) (com/dtteam/dynamictrees/command/subcommand/PurgeTreesCommand.java:10)
- [ ] `PurgeTreesCommand.getPermissionLevel(0)`: its class isn't translated (it extends `ChunkBasedCommand`) (com/dtteam/dynamictrees/command/subcommand/PurgeTreesCommand.java:15)
- [ ] `PurgeTreesCommand.processChunk(4)`: its class isn't translated (it extends `ChunkBasedCommand`) (com/dtteam/dynamictrees/command/subcommand/PurgeTreesCommand.java:20)

### `RegistryCommand`: it extends `SubCommand`

- [ ] `RegistryCommand.RegistryCommand(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RegistryCommand.java:14)
- [ ] `RegistryCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RegistryCommand.java:18)
- [ ] `RegistryCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RegistryCommand.java:23)
- [ ] `RegistryCommand.registerArguments(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RegistryCommand.java:28)
- [ ] `RegistryCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RegistryCommand.java:33)

### `RegistrySubCommand`: it's generic

- [ ] `RegistrySubCommand.RegistrySubCommand(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/command/subcommand/RegistrySubCommand.java:17)
- [ ] `RegistrySubCommand.getName(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/command/subcommand/RegistrySubCommand.java:21)
- [ ] `RegistrySubCommand.getPermissionLevel(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/command/subcommand/RegistrySubCommand.java:26)
- [ ] `RegistrySubCommand.registerArgument(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/command/subcommand/RegistrySubCommand.java:31)
- [ ] `RegistrySubCommand.listEntries(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/command/subcommand/RegistrySubCommand.java:41)

### `RotateJoCodeCommand`: it extends `SubCommand`

- [ ] `RotateJoCodeCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RotateJoCodeCommand.java:13)
- [ ] `RotateJoCodeCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RotateJoCodeCommand.java:18)
- [ ] `RotateJoCodeCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RotateJoCodeCommand.java:23)
- [ ] `RotateJoCodeCommand.rotateJoCode(3)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/RotateJoCodeCommand.java:34)

### `SetCoordXorCommand`: it extends `SubCommand`

- [x] `SetCoordXorCommand.XOR` (com/dtteam/dynamictrees/command/subcommand/SetCoordXorCommand.java:11)
- [ ] `SetCoordXorCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetCoordXorCommand.java:13)
- [ ] `SetCoordXorCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetCoordXorCommand.java:18)
- [ ] `SetCoordXorCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetCoordXorCommand.java:23)
- [ ] `SetCoordXorCommand.setXor(2)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetCoordXorCommand.java:30)

### `SetTreeCommand`: it extends `SubCommand`

- [x] `SetTreeCommand.DEFAULT_FERTILITY` (com/dtteam/dynamictrees/command/subcommand/SetTreeCommand.java:22)
- [ ] `SetTreeCommand.getName(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetTreeCommand.java:24)
- [ ] `SetTreeCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetTreeCommand.java:29)
- [ ] `SetTreeCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetTreeCommand.java:34)
- [ ] `SetTreeCommand.setTree(6)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetTreeCommand.java:132)
- [ ] `SetTreeCommand.setTreeWithRoots(7)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SetTreeCommand.java:149)

### `SimpleSubCommand`: it extends `SubCommand`

- [ ] `SimpleSubCommand.register(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SimpleSubCommand.java:10)
- [ ] `SimpleSubCommand.execute(1)`: it has no body (com/dtteam/dynamictrees/command/subcommand/SimpleSubCommand.java:16)
- [ ] `SimpleSubCommand.getPermissionLevel(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SimpleSubCommand.java:18)
- [ ] `SimpleSubCommand.registerArgument(0)`: its class isn't translated (it extends `SubCommand`) (com/dtteam/dynamictrees/command/subcommand/SimpleSubCommand.java:23)

### `SubCommand`

- [ ] `SubCommand.NO_TREE_FOUND`: it uses Minecraft's `DynamicCommandExceptionType` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:36)
- [ ] `SubCommand.SPECIES_UNKNOWN`: it uses Minecraft's `DynamicCommandExceptionType` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:47)
- [ ] `SubCommand.getVector3i(1)`: it uses Minecraft's `class_2382` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:51)
- [ ] `SubCommand.getName(0)`: it has no body (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:55)
- [ ] `SubCommand.getPermissionLevel(0)`: it has no body (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:57)
- [ ] `SubCommand.register(0)`: it uses Minecraft's `ArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:59)
- [ ] `SubCommand.registerArguments(0)`: it uses Minecraft's `ArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:66)
- [ ] `SubCommand.registerArgument(0)`: it has no body (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:70)
- [ ] `SubCommand.executesSuccess(1)`: it uses ThrowableRunnable, which isn't translated (it's an interface) (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:72)
- [ ] `SubCommand.executesSuccess(2)`: Minecraft's FabricClientCommandSource kept in a variable or field (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:77)
- [ ] `SubCommand.booleanArgument(1)`: it uses Minecraft's `RequiredArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:82)
- [x] `SubCommand.booleanArgument(2)` in game_side.rs (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:86)
- [ ] `SubCommand.intArgument(1)`: it uses Minecraft's `RequiredArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:90)
- [x] `SubCommand.intArgument(2)` in game_side.rs (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:94)
- [ ] `SubCommand.stringArgument(1)`: it uses Minecraft's `RequiredArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:98)
- [ ] `SubCommand.stringArgument(2)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:102)
- [ ] `SubCommand.stringArgument(2)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:106)
- [ ] `SubCommand.blockPosArgument(0)`: it uses Minecraft's `RequiredArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:110)
- [ ] `SubCommand.blockPosArgument(1)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:114)
- [ ] `SubCommand.rootPosArgument(1)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:118)
- [ ] `SubCommand.speciesArgument(0)`: it uses Minecraft's `RequiredArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:133)
- [ ] `SubCommand.speciesArgument(1)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:137)
- [ ] `SubCommand.resourceLocationArgument(2)`: it uses Minecraft's `RequiredArgumentBuilder` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:147)
- [ ] `SubCommand.aqua(1)`: it uses Minecraft's `class_2561` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:151)
- [ ] `SubCommand.darkRed(1)`: it uses Minecraft's `class_2561` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:155)
- [ ] `SubCommand.sendSuccess(2)`: it uses Minecraft's `class_2168` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:159)
- [ ] `SubCommand.sendSuccessAndLog(2)`: it uses Minecraft's `class_2168` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:163)
- [ ] `SubCommand.sendFailure(2)`: it uses Minecraft's `class_2168` (com/dtteam/dynamictrees/command/subcommand/SubCommand.java:167)

### `ContinuityWrappedModelHandler`: it extends `WrappedModelHandler`

- [ ] `ContinuityWrappedModelHandler.unwrapBranchModel(1)`: its class isn't translated (it extends `WrappedModelHandler`) (com/dtteam/dynamictrees/compat/continuity/ContinuityWrappedModelHandler.java:9)

### `WrappedModelHandler`

- [x] `WrappedModelHandler.INSTANCE` (com/dtteam/dynamictrees/compat/continuity/WrappedModelHandler.java:9)
- [ ] `WrappedModelHandler.getInstance(0)`: its body couldn't be read: an anonymous class (com/dtteam/dynamictrees/compat/continuity/WrappedModelHandler.java:11)
- [ ] `WrappedModelHandler.unwrapBranchModel(1)`: it uses BasicBranchBlockBakedModel, which isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/compat/continuity/WrappedModelHandler.java:23)

### `DTJeiPlugin`: it implements `IModPlugin` (mezz.jei.api)

- [ ] `DTJeiPlugin.getPluginUid(0)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (com/dtteam/dynamictrees/compat/DTJeiPlugin.java:16)
- [ ] `DTJeiPlugin.registerItemSubtypes(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (com/dtteam/dynamictrees/compat/DTJeiPlugin.java:20)
- [ ] `DTJeiPlugin.registerRecipes(1)`: its class isn't translated (it implements `IModPlugin` (mezz.jei.api)) (com/dtteam/dynamictrees/compat/DTJeiPlugin.java:23)
- [ ] `DTJeiPlugin.makeJeiBrewingRecipe(4)`: it uses `IJeiBrewingRecipe` (mezz.jei.api.recipe.vanilla) (com/dtteam/dynamictrees/compat/DTJeiPlugin.java:26)

### `SereneSeasonsSeasonProvider`: it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)

- [ ] `SereneSeasonsSeasonProvider.getSeasonValue(2)`: its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/compat/SereneSeasonsSeasonProvider.java:20)
- [ ] `SereneSeasonsSeasonProvider.updateTick(2)`: its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/compat/SereneSeasonsSeasonProvider.java:25)
- [ ] `SereneSeasonsSeasonProvider.shouldSnowMelt(2)`: its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/compat/SereneSeasonsSeasonProvider.java:30)
- [ ] `SereneSeasonsSeasonProvider.registerSereneSeasonsProvider(0)`: it calls SeasonCompatibilityHandler.registerSeasonManager, which isn't translated (com/dtteam/dynamictrees/compat/SereneSeasonsSeasonProvider.java:35)

### `WailaHelper`

- [x] `WailaHelper.invalid` (com/dtteam/dynamictrees/compat/WailaHelper.java:4)
- [x] `WailaHelper.invalidateWailaPosition(0)` (com/dtteam/dynamictrees/compat/WailaHelper.java:6)

### `WailaBranchHandler`: it implements `IBlockComponentProvider` (snownee.jade.api)

- [ ] `WailaBranchHandler.ID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/compat/waila/WailaBranchHandler.java:35)
- [ ] `WailaBranchHandler.appendTooltip(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaBranchHandler.java:40)
- [ ] `WailaBranchHandler.getTreeVolume(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaBranchHandler.java:126)
- [ ] `WailaBranchHandler.getWailaSpecies(2)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaBranchHandler.java:149)
- [ ] `WailaBranchHandler.getElement(1)`: it uses `IElement` (snownee.jade.api.ui) (com/dtteam/dynamictrees/compat/waila/WailaBranchHandler.java:153)
- [ ] `WailaBranchHandler.getUid(0)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaBranchHandler.java:157)

### `WailaCompat`: it implements `IWailaPlugin` (snownee.jade.api)

- [ ] `WailaCompat.registerClient(1)`: its class isn't translated (it implements `IWailaPlugin` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaCompat.java:15)

### `WailaFruitHandler`: it implements `IBlockComponentProvider` (snownee.jade.api)

- [ ] `WailaFruitHandler.FRUIT_UID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/compat/waila/WailaFruitHandler.java:14)
- [ ] `WailaFruitHandler.appendTooltip(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaFruitHandler.java:16)
- [ ] `WailaFruitHandler.getUid(0)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaFruitHandler.java:32)

### `WailaPodHandler`: it implements `IBlockComponentProvider` (snownee.jade.api)

- [ ] `WailaPodHandler.POD_UID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/compat/waila/WailaPodHandler.java:15)
- [ ] `WailaPodHandler.COCOA`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/compat/waila/WailaPodHandler.java:16)
- [ ] `WailaPodHandler.appendTooltip(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaPodHandler.java:18)
- [ ] `WailaPodHandler.getUid(0)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaPodHandler.java:34)

### `WailaRootyHandler`: it implements `IBlockComponentProvider` (snownee.jade.api)

- [ ] `WailaRootyHandler.ROOTY_UID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/compat/waila/WailaRootyHandler.java:14)
- [ ] `WailaRootyHandler.appendTooltip(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaRootyHandler.java:16)
- [ ] `WailaRootyHandler.getUid(0)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaRootyHandler.java:23)

### `WailaRootyWaterHandler`: it implements `IBlockComponentProvider` (snownee.jade.api)

- [ ] `WailaRootyWaterHandler.ROOTY_WATER_UID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/compat/waila/WailaRootyWaterHandler.java:13)
- [ ] `WailaRootyWaterHandler.appendTooltip(3)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaRootyWaterHandler.java:15)
- [ ] `WailaRootyWaterHandler.getUid(0)`: its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)) (com/dtteam/dynamictrees/compat/waila/WailaRootyWaterHandler.java:19)

### `DTConfigs`: it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)

- [ ] `DTConfigs.SERVER_CONFIG`: it uses `ModConfigSpec` (net.neoforged.neoforge.common) (com/dtteam/dynamictrees/config/DTConfigs.java:17)
- [ ] `DTConfigs.SERVER`: it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/dtteam/dynamictrees/config/DTConfigs.java:18)
- [ ] `DTConfigs.COMMON_CONFIG`: it uses `ModConfigSpec` (net.neoforged.neoforge.common) (com/dtteam/dynamictrees/config/DTConfigs.java:19)
- [ ] `DTConfigs.COMMON`: it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/dtteam/dynamictrees/config/DTConfigs.java:20)
- [ ] `DTConfigs.CLIENT_CONFIG`: it uses `ModConfigSpec` (net.neoforged.neoforge.common) (com/dtteam/dynamictrees/config/DTConfigs.java:21)
- [ ] `DTConfigs.CLIENT`: it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/dtteam/dynamictrees/config/DTConfigs.java:22)
- [ ] `DTConfigs.DTConfigs(0)`: its class isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/dtteam/dynamictrees/config/DTConfigs.java:76)
- [ ] `DTConfigs.buildServerConfig(1)`: it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/dtteam/dynamictrees/config/DTConfigs.java:79)
- [ ] `DTConfigs.buildCommonConfig(1)`: it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/dtteam/dynamictrees/config/DTConfigs.java:189)
- [ ] `DTConfigs.buildClientConfig(1)`: it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec)) (com/dtteam/dynamictrees/config/DTConfigs.java:234)

### `DTDataProvider`: it's an interface

- [ ] `DTDataProvider.block(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/data/DTDataProvider.java:8)
- [ ] `DTDataProvider.item(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/data/DTDataProvider.java:12)

### `DTDataProvider.BlockState`: it's an interface


### `DTDataProvider.ItemModel`: it's an interface


### `DTDataProvider.Language`: it's an interface


### `DTLootTableBuilder`

- [ ] `DTLootTableBuilder.hasSilkTouch(1)`: it uses Minecraft's `class_210` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:49)
- [ ] `DTLootTableBuilder.hasNoShearsOrSilkTouch(1)`: it uses Minecraft's `class_210` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:60)
- [ ] `DTLootTableBuilder.hasShearsOrSilkTouch(1)`: it uses Minecraft's `class_210` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:64)
- [ ] `DTLootTableBuilder.createSelfDropDispatchTable(3)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:69)
- [ ] `DTLootTableBuilder.createLeavesBlockDrops(4)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:78)
- [ ] `DTLootTableBuilder.createPalmLeavesBlockDrops(3)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:104)
- [ ] `DTLootTableBuilder.createWartBlockDrops(2)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:115)
- [ ] `DTLootTableBuilder.createLeavesDrops(3)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:125)
- [ ] `DTLootTableBuilder.createLeavesDrops(4)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:129)
- [ ] `DTLootTableBuilder.createPalmLeavesDrops(3)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:157)
- [ ] `DTLootTableBuilder.createWartDrops(2)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:171)
- [ ] `DTLootTableBuilder.createVoluntaryDrops(2)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:182)
- [ ] `DTLootTableBuilder.createBranchDrops(3)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:192)
- [ ] `DTLootTableBuilder.createFruitPodDrops(6)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:207)
- [ ] `DTLootTableBuilder.createFruitPodDrops(7)`: it uses Minecraft's `class_53` (com/dtteam/dynamictrees/data/DTLootTableBuilder.java:213)

### `Generator`: it's an interface

- [ ] `Generator.generate(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/data/Generator.java:11)
- [ ] `Generator.generate(3)`: it has no body (com/dtteam/dynamictrees/data/Generator.java:18)
- [ ] `Generator.gatherDependencies(1)`: it has no body (com/dtteam/dynamictrees/data/Generator.java:20)
- [ ] `Generator.verifyInput(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/data/Generator.java:22)
- [ ] `Generator.verifyDependencies(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/data/Generator.java:26)

### `Generator.Dependencies`: it uses DependencyKey, which isn't translated (it's generic)

- [ ] `Generator.Dependencies.appendValue(2)`: it's generic (com/dtteam/dynamictrees/data/Generator.java:33)
- [ ] `Generator.Dependencies.append(2)`: it's generic (com/dtteam/dynamictrees/data/Generator.java:37)
- [ ] `Generator.Dependencies.get(1)`: it's generic (com/dtteam/dynamictrees/data/Generator.java:42)
- [ ] `Generator.Dependencies.getOptional(1)`: it's generic (com/dtteam/dynamictrees/data/Generator.java:50)
- [ ] `Generator.Dependencies.allRequiredPresent(0)`: its class isn't translated (it uses DependencyKey, which isn't translated (it's generic)) (com/dtteam/dynamictrees/data/Generator.java:59)

### `Generator.DependencyKey`: it's generic

- [ ] `Generator.DependencyKey.DependencyKey(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/data/Generator.java:68)
- [ ] `Generator.DependencyKey.DependencyKey(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/data/Generator.java:72)
- [ ] `Generator.DependencyKey.getName(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/data/Generator.java:77)
- [ ] `Generator.DependencyKey.isOptional(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/data/Generator.java:81)
- [ ] `Generator.DependencyKey.equals(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/data/Generator.java:85)
- [ ] `Generator.DependencyKey.hashCode(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/data/Generator.java:97)

### `DTBiomeTags`

- [ ] `DTBiomeTags.IS_BEE_NEST_COMMON`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBiomeTags.java:9)
- [ ] `DTBiomeTags.IS_BEE_NEST_UNCOMMON`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBiomeTags.java:10)
- [ ] `DTBiomeTags.IS_BEE_NEST_RARE`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBiomeTags.java:11)
- [ ] `DTBiomeTags.IS_BEE_NEST_GUARANTEED`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBiomeTags.java:12)
- [ ] `DTBiomeTags.bind(1)`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBiomeTags.java:14)

### `DTBlockTags`

- [ ] `DTBlockTags.BRANCHES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:9)
- [ ] `DTBlockTags.STRIPPED_BRANCHES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:10)
- [ ] `DTBlockTags.BRANCHES_THAT_BURN`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:11)
- [ ] `DTBlockTags.STRIPPED_BRANCHES_THAT_BURN`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:12)
- [ ] `DTBlockTags.FOLIAGE`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:13)
- [ ] `DTBlockTags.FUNGUS_BRANCHES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:14)
- [ ] `DTBlockTags.STRIPPED_FUNGUS_BRANCHES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:15)
- [ ] `DTBlockTags.FUNGUS_CAPS`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:16)
- [ ] `DTBlockTags.LEAVES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:17)
- [ ] `DTBlockTags.SAPLINGS`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:18)
- [ ] `DTBlockTags.WART_BLOCKS`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:19)
- [ ] `DTBlockTags.ROOTY_SOIL`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:20)
- [ ] `DTBlockTags.AERIAL_ROOTS_ROOTY_SOIL`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:21)
- [ ] `DTBlockTags.ROOTS`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:22)
- [ ] `DTBlockTags.bind(1)`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTBlockTags.java:24)

### `DTEntityTypeTags`

- [ ] `DTEntityTypeTags.CAN_PASS_THROUGH_LEAVES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTEntityTypeTags.java:9)
- [ ] `DTEntityTypeTags.FALLING_TREE_DAMAGE_IMMUNE`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTEntityTypeTags.java:10)
- [ ] `DTEntityTypeTags.bind(1)`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTEntityTypeTags.java:12)

### `DTItemTags`

- [ ] `DTItemTags.BRANCHES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:9)
- [ ] `DTItemTags.BRANCHES_THAT_BURN`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:10)
- [ ] `DTItemTags.FUNGUS_BRANCHES`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:11)
- [ ] `DTItemTags.SEEDS`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:12)
- [ ] `DTItemTags.FUNGUS_CAPS`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:13)
- [ ] `DTItemTags.FERTILIZER`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:14)
- [ ] `DTItemTags.ENHANCED_FERTILIZER`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:15)
- [ ] `DTItemTags.bind(1)`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/data/tags/DTItemTags.java:17)

### `Applier`: it's an interface

- [ ] `Applier.apply(2)`: it has no body (com/dtteam/dynamictrees/deserialization/applier/Applier.java:5)

### `ArrayIteratorPropertyApplier`: it's generic

- [ ] `ArrayIteratorPropertyApplier.ArrayIteratorPropertyApplier(5)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/ArrayIteratorPropertyApplier.java:18)
- [ ] `ArrayIteratorPropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/ArrayIteratorPropertyApplier.java:26)
- [ ] `ArrayIteratorPropertyApplier.json(4)`: it's generic (com/dtteam/dynamictrees/deserialization/applier/ArrayIteratorPropertyApplier.java:47)

### `ArrayPropertyApplier`: it's generic

- [ ] `ArrayPropertyApplier.ArrayPropertyApplier(5)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/ArrayPropertyApplier.java:20)
- [ ] `ArrayPropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/ArrayPropertyApplier.java:32)
- [ ] `ArrayPropertyApplier.json(4)`: it's generic (com/dtteam/dynamictrees/deserialization/applier/ArrayPropertyApplier.java:53)

### `IfTrueApplier`: it's an interface

- [ ] `IfTrueApplier.apply(1)`: it has no body (com/dtteam/dynamictrees/deserialization/applier/IfTrueApplier.java:5)

### `JsonPropertyApplier`: it's generic

- [ ] `JsonPropertyApplier.JsonPropertyApplier(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/JsonPropertyApplier.java:12)
- [ ] `JsonPropertyApplier.JsonPropertyApplier(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/JsonPropertyApplier.java:16)
- [ ] `JsonPropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/JsonPropertyApplier.java:21)

### `MapPropertyApplier`: it's generic

- [ ] `MapPropertyApplier.MapPropertyApplier(5)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/MapPropertyApplier.java:18)
- [ ] `MapPropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/MapPropertyApplier.java:30)
- [ ] `MapPropertyApplier.json(4)`: it's generic (com/dtteam/dynamictrees/deserialization/applier/MapPropertyApplier.java:56)

### `MultiPropertyApplier`: it's generic

- [ ] `MultiPropertyApplier.MultiPropertyApplier(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/MultiPropertyApplier.java:12)
- [ ] `MultiPropertyApplier.addApplier(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/MultiPropertyApplier.java:18)
- [ ] `MultiPropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/MultiPropertyApplier.java:22)
- [ ] `MultiPropertyApplier.applyNext(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/MultiPropertyApplier.java:35)

### `PropertyApplier`: it's generic

- [ ] `PropertyApplier.PropertyApplier(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/PropertyApplier.java:10)
- [ ] `PropertyApplier.PropertyApplier(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/PropertyApplier.java:14)
- [ ] `PropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/PropertyApplier.java:20)
- [ ] `PropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/PropertyApplier.java:25)
- [ ] `PropertyApplier.getObjectClass(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/applier/PropertyApplier.java:28)

### `PropertyApplierResult`

- [ ] `PropertyApplierResult.PropertyApplierResult(1)`: it calls `Lists` (com.google.common.collect).newLinkedList (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:12)
- [x] `PropertyApplierResult.PropertyApplierResult(2)` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:16)
- [x] `PropertyApplierResult.failed(0)` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:21)
- [x] `PropertyApplierResult.getError(0)` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:25)
- [ ] `PropertyApplierResult.addErrorPrefix(1)`: `this` used as a value (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:29)
- [x] `PropertyApplierResult.getWarnings(0)` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:34)
- [ ] `PropertyApplierResult.addWarnings(1)`: it changes an object through a second name (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:38)
- [ ] `PropertyApplierResult.addWarningsPrefix(1)`: List.replaceAll (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:42)
- [ ] `PropertyApplierResult.success(0)`: it creates a PropertyApplierResult with a constructor that isn't translated (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:47)
- [x] `PropertyApplierResult.success(1)` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:51)
- [ ] `PropertyApplierResult.failure(1)`: it creates a PropertyApplierResult with a constructor that isn't translated (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:55)
- [x] `PropertyApplierResult.failure(2)` (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:59)
- [ ] `PropertyApplierResult.from(1)`: it uses Result, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/applier/PropertyApplierResult.java:63)

### `VoidApplier`: it's an interface

- [ ] `VoidApplier.apply(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/applier/VoidApplier.java:4)
- [ ] `VoidApplier.applySuccessful(2)`: it has no body (com/dtteam/dynamictrees/deserialization/applier/VoidApplier.java:10)

### `DeserializationException`: it extends `Exception`

- [ ] `DeserializationException.DeserializationException(1)`: its class isn't translated (it extends `Exception`) (com/dtteam/dynamictrees/deserialization/DeserializationException.java:4)
- [ ] `DeserializationException.error(1)`: it uses DeserializationException, which isn't translated (it extends `Exception`) (com/dtteam/dynamictrees/deserialization/DeserializationException.java:8)

### `AxisAlignedBBDeserializer`: it implements `JsonDeserializer`

- [ ] `AxisAlignedBBDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/AxisAlignedBBDeserializer.java:10)

### `BiomeListDeserializer`: it implements `JsonDeserializer`

- [ ] `BiomeListDeserializer.DELAYED_BIOME_REGISTRY`: it uses Minecraft's `class_2378` (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:32)
- [ ] `BiomeListDeserializer.TAG_APPLIER`: it uses Applier, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:40)
- [ ] `BiomeListDeserializer.NAME_APPLIER`: it uses VoidApplier, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:61)
- [ ] `BiomeListDeserializer.NAMES_OR_APPLIER`: it uses VoidApplier, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:73)
- [ ] `BiomeListDeserializer.TAGS_OR_APPLIER`: it uses VoidApplier, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:97)
- [x] `BiomeListDeserializer.usingNotOperator(1)` (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:144)
- [ ] `BiomeListDeserializer.BiomeListDeserializer(0)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:148)
- [ ] `BiomeListDeserializer.registerAppliers(0)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:152)
- [ ] `BiomeListDeserializer.applyAllAppliers(2)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:165)
- [ ] `BiomeListDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/BiomeListDeserializer.java:169)

### `BooleanOpDeserializer`: it implements `JsonDeserializer`

- [ ] `BooleanOpDeserializer.VALUES`: it uses Minecraft's `class_247` (com/dtteam/dynamictrees/deserialization/deserializer/BooleanOpDeserializer.java:12)
- [ ] `BooleanOpDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/BooleanOpDeserializer.java:31)

### `BuiltInRegistryEntryDeserializer`: it's generic

- [ ] `BuiltInRegistryEntryDeserializer.BuiltInRegistryEntryDeserializer(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/BuiltInRegistryEntryDeserializer.java:19)
- [ ] `BuiltInRegistryEntryDeserializer.BuiltInRegistryEntryDeserializer(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/BuiltInRegistryEntryDeserializer.java:23)
- [ ] `BuiltInRegistryEntryDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/BuiltInRegistryEntryDeserializer.java:30)

### `ConfiguredDeserializer`: it's generic

- [ ] `ConfiguredDeserializer.ConfiguredDeserializer(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ConfiguredDeserializer.java:28)
- [ ] `ConfiguredDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ConfiguredDeserializer.java:34)
- [ ] `ConfiguredDeserializer.isConfigurationValid(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ConfiguredDeserializer.java:55)
- [ ] `ConfiguredDeserializer.getTemplate(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ConfiguredDeserializer.java:59)
- [ ] `ConfiguredDeserializer.getTemplateName(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ConfiguredDeserializer.java:65)
- [ ] `ConfiguredDeserializer.addProperty(4)`: it's generic (com/dtteam/dynamictrees/deserialization/deserializer/ConfiguredDeserializer.java:70)

### `DeserializableClassDeserializer`: it implements `JsonDeserializer`

- [ ] `DeserializableClassDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/DeserializableClassDeserializer.java:9)

### `Deserializer`: it's an interface

- [ ] `Deserializer.deserialize(1)`: it has no body (com/dtteam/dynamictrees/deserialization/deserializer/Deserializer.java:8)
- [ ] `Deserializer.isValid(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/Deserializer.java:10)
- [ ] `Deserializer.deserializeIfValid(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/Deserializer.java:14)

### `EnumDeserializer`: it's generic

- [ ] `EnumDeserializer.EnumDeserializer(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/EnumDeserializer.java:11)
- [ ] `EnumDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/EnumDeserializer.java:15)

### `JsonDeserializer`: it's an interface

- [ ] `JsonDeserializer.deserializeIfValid(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/JsonDeserializer.java:9)

### `ListDeserializer`: it's generic

- [ ] `ListDeserializer.ListDeserializer(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ListDeserializer.java:14)
- [ ] `ListDeserializer.ListDeserializer(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ListDeserializer.java:18)
- [ ] `ListDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/ListDeserializer.java:23)
- [ ] `ListDeserializer.getListClass(1)`: it's generic (com/dtteam/dynamictrees/deserialization/deserializer/ListDeserializer.java:31)
- [ ] `ListDeserializer.getListClass(2)`: it's generic (com/dtteam/dynamictrees/deserialization/deserializer/ListDeserializer.java:35)

### `MapColorDeserializer`: it implements `JsonDeserializer`

- [ ] `MapColorDeserializer.MATERIAL_COLORS`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/deserialization/deserializer/MapColorDeserializer.java:13)
- [ ] `MapColorDeserializer.registerMapColor(2)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/deserialization/deserializer/MapColorDeserializer.java:78)
- [ ] `MapColorDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/MapColorDeserializer.java:82)

### `MapDeserializer`: it's generic

- [ ] `MapDeserializer.MapDeserializer(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/MapDeserializer.java:17)
- [ ] `MapDeserializer.MapDeserializer(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/MapDeserializer.java:21)
- [ ] `MapDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/MapDeserializer.java:27)
- [ ] `MapDeserializer.getMapClass(2)`: it's generic (com/dtteam/dynamictrees/deserialization/deserializer/MapDeserializer.java:50)
- [ ] `MapDeserializer.getMapClass(3)`: it's generic (com/dtteam/dynamictrees/deserialization/deserializer/MapDeserializer.java:54)

### `PropertyDefinitionDeserializer`: it implements `JsonDeserializer`

- [ ] `PropertyDefinitionDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/PropertyDefinitionDeserializer.java:12)
- [ ] `PropertyDefinitionDeserializer.deserializeDefinition(2)`: it's generic (com/dtteam/dynamictrees/deserialization/deserializer/PropertyDefinitionDeserializer.java:16)

### `RegistryEntryDeserializer`: it's generic

- [ ] `RegistryEntryDeserializer.RegistryEntryDeserializer(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/RegistryEntryDeserializer.java:12)
- [ ] `RegistryEntryDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/RegistryEntryDeserializer.java:16)

### `ResourceLocationDeserializer`: it implements `JsonDeserializer`

- [ ] `ResourceLocationDeserializer.ResourceLocationDeserializer(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/ResourceLocationDeserializer.java:13)
- [ ] `ResourceLocationDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/ResourceLocationDeserializer.java:17)
- [ ] `ResourceLocationDeserializer.isValidResourceLocation(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/deserialization/deserializer/ResourceLocationDeserializer.java:28)
- [ ] `ResourceLocationDeserializer.decode(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/ResourceLocationDeserializer.java:34)
- [ ] `ResourceLocationDeserializer.create(0)`: it uses ResourceLocationDeserializer, which isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/ResourceLocationDeserializer.java:47)
- [ ] `ResourceLocationDeserializer.create(1)`: it uses ResourceLocationDeserializer, which isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/ResourceLocationDeserializer.java:51)

### `SeedSaplingRecipeDeserializer`: it implements `JsonDeserializer`

- [ ] `SeedSaplingRecipeDeserializer.SeedSaplingRecipeDeserializer(0)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/SeedSaplingRecipeDeserializer.java:17)
- [ ] `SeedSaplingRecipeDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/SeedSaplingRecipeDeserializer.java:29)

### `SoundTypeDeserializer`: it implements `JsonDeserializer`

- [ ] `SoundTypeDeserializer.SOUND_TYPES`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/deserialization/deserializer/SoundTypeDeserializer.java:13)
- [ ] `SoundTypeDeserializer.registerSoundType(2)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/deserialization/deserializer/SoundTypeDeserializer.java:91)
- [ ] `SoundTypeDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/SoundTypeDeserializer.java:95)

### `StaticFieldDeserializer`: it's generic

- [ ] `StaticFieldDeserializer.StaticFieldDeserializer(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/StaticFieldDeserializer.java:13)
- [ ] `StaticFieldDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/deserializer/StaticFieldDeserializer.java:17)

### `VoxelShapeDeserializer`: it implements `JsonDeserializer`

- [ ] `VoxelShapeDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/VoxelShapeDeserializer.java:20)
- [ ] `VoxelShapeDeserializer.deserializeArray(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/VoxelShapeDeserializer.java:29)
- [ ] `VoxelShapeDeserializer.deserializeObject(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/VoxelShapeDeserializer.java:39)
- [ ] `VoxelShapeDeserializer.getParametersJson(1)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/VoxelShapeDeserializer.java:52)
- [ ] `VoxelShapeDeserializer.deserializeShapes(2)`: its class isn't translated (it implements `JsonDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/VoxelShapeDeserializer.java:56)

### `ChanceSelectorDeserializer`: it implements `JsonBiomeDatabaseDeserializer`

- [ ] `ChanceSelectorDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/ChanceSelectorDeserializer.java:17)
- [ ] `ChanceSelectorDeserializer.createSimpleChanceSelector(1)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/ChanceSelectorDeserializer.java:25)
- [ ] `ChanceSelectorDeserializer.readJsonChanceSelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/ChanceSelectorDeserializer.java:35)
- [ ] `ChanceSelectorDeserializer.createSimpleChanceSelectorFromJson(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/ChanceSelectorDeserializer.java:45)
- [ ] `ChanceSelectorDeserializer.createMathSelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/ChanceSelectorDeserializer.java:56)
- [ ] `ChanceSelectorDeserializer.readExpressionSelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/ChanceSelectorDeserializer.java:62)

### `DensitySelectorDeserializer`: it implements `JsonBiomeDatabaseDeserializer`

- [ ] `DensitySelectorDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/DensitySelectorDeserializer.java:22)
- [ ] `DensitySelectorDeserializer.createStaticDensitySelector(1)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/DensitySelectorDeserializer.java:31)
- [ ] `DensitySelectorDeserializer.createScaleDensitySelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/DensitySelectorDeserializer.java:35)
- [ ] `DensitySelectorDeserializer.readDensitySelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/DensitySelectorDeserializer.java:47)
- [ ] `DensitySelectorDeserializer.readExpressionSelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/DensitySelectorDeserializer.java:62)

### `JsonBiomeDatabaseDeserializer`: it's an interface

- [ ] `JsonBiomeDatabaseDeserializer.isDefault(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/JsonBiomeDatabaseDeserializer.java:14)

### `SpeciesSelectorDeserializer`: it implements `JsonBiomeDatabaseDeserializer`

- [ ] `SpeciesSelectorDeserializer.deserialize(1)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/SpeciesSelectorDeserializer.java:16)
- [ ] `SpeciesSelectorDeserializer.readStatic(1)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/SpeciesSelectorDeserializer.java:24)
- [ ] `SpeciesSelectorDeserializer.readStatic(1)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/SpeciesSelectorDeserializer.java:28)
- [ ] `SpeciesSelectorDeserializer.readSelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/SpeciesSelectorDeserializer.java:36)
- [ ] `SpeciesSelectorDeserializer.getRandomSpeciesSelector(2)`: its class isn't translated (it implements `JsonBiomeDatabaseDeserializer`) (com/dtteam/dynamictrees/deserialization/deserializer/worldgen/SpeciesSelectorDeserializer.java:53)

### `JsonDeserializers`

- [ ] `JsonDeserializers.DESERIALIZERS`: it uses `Class` (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:72)
- [ ] `JsonDeserializers.NULL`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:73)
- [ ] `JsonDeserializers.JSON_ELEMENT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:74)
- [ ] `JsonDeserializers.JSON_PRIMITIVE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:75)
- [ ] `JsonDeserializers.JSON_OBJECT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:81)
- [ ] `JsonDeserializers.JSON_ARRAY`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:85)
- [ ] `JsonDeserializers.BOOLEAN`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:89)
- [ ] `JsonDeserializers.NUMBER`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:93)
- [ ] `JsonDeserializers.STRING`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:97)
- [ ] `JsonDeserializers.BYTE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:101)
- [ ] `JsonDeserializers.SHORT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:102)
- [ ] `JsonDeserializers.INTEGER`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:103)
- [ ] `JsonDeserializers.LONG`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:104)
- [ ] `JsonDeserializers.FLOAT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:105)
- [ ] `JsonDeserializers.DOUBLE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:106)
- [ ] `JsonDeserializers.RESOURCE_LOCATION`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:107)
- [ ] `JsonDeserializers.DT_RESOURCE_LOCATION`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:108)
- [ ] `JsonDeserializers.BLOCK`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:109)
- [ ] `JsonDeserializers.ITEM`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:110)
- [ ] `JsonDeserializers.ITEM_STACK`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:111)
- [ ] `JsonDeserializers.AABB`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:112)
- [ ] `JsonDeserializers.VOXEL_SHAPE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:113)
- [ ] `JsonDeserializers.CELL_KIT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:114)
- [ ] `JsonDeserializers.LEAVES_PROPERTIES`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:115)
- [ ] `JsonDeserializers.GROWTH_LOGIC_KIT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:118)
- [ ] `JsonDeserializers.GEN_FEATURE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:121)
- [ ] `JsonDeserializers.FAMILY`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:122)
- [ ] `JsonDeserializers.FRUIT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:123)
- [ ] `JsonDeserializers.POD`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:124)
- [ ] `JsonDeserializers.SPECIES`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:125)
- [ ] `JsonDeserializers.SOIL_PROPERTIES`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:126)
- [ ] `JsonDeserializers.SOIL_PROPERTIES_LIST`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:129)
- [ ] `JsonDeserializers.FEATURE_CANCELLER`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:132)
- [ ] `JsonDeserializers.RESOURCE_LOCATION_MAP`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:135)
- [ ] `JsonDeserializers.CONFIGURED_GEN_FEATURE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:138)
- [ ] `JsonDeserializers.CONFIGURED_GROWTH_LOGIC_KIT`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:141)
- [ ] `JsonDeserializers.SEED`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:144)
- [ ] `JsonDeserializers.BRANCH`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:147)
- [ ] `JsonDeserializers.VINE_TYPE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:152)
- [ ] `JsonDeserializers.DECORATION_STAGE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:155)
- [ ] `JsonDeserializers.OPERATION`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:156)
- [ ] `JsonDeserializers.CLIMATE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:159)
- [ ] `JsonDeserializers.BIOME_LIST`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:160)
- [ ] `JsonDeserializers.BIOME_PREDICATE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:161)
- [ ] `JsonDeserializers.SPECIES_SELECTOR`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:171)
- [ ] `JsonDeserializers.DENSITY_SELECTOR`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:174)
- [ ] `JsonDeserializers.CHANCE_SELECTOR`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:177)
- [ ] `JsonDeserializers.SEED_SAPLING_RECIPE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:180)
- [ ] `JsonDeserializers.MAP_COLOR`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:181)
- [ ] `JsonDeserializers.SOUND_TYPE`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:182)
- [ ] `JsonDeserializers.BOOLEAN_FUNCTION`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:183)
- [ ] `JsonDeserializers.DESERIALIZABLE_CLASS`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:184)
- [ ] `JsonDeserializers.VARIABLE_DEFINITION`: it uses JsonDeserializer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:185)
- [ ] `JsonDeserializers.get(1)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:189)
- [ ] `JsonDeserializers.getOrThrow(1)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:193)
- [ ] `JsonDeserializers.getOrThrow(2)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:197)
- [ ] `JsonDeserializers.getDeserializableClasses(0)`: it uses `Class` (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:206)
- [ ] `JsonDeserializers.register(2)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:210)
- [ ] `JsonDeserializers.registerRegistryEntryGetters(0)`: it uses JsonDeserializers.BLOCK, which isn't translated (it uses JsonDeserializer, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:215)
- [ ] `JsonDeserializers.postRegistryEvent(0)`: it uses Services.EVENT, which isn't translated (it uses IEventHelper, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:220)

### `JsonDeserializers.NullDeserializer`: it's generic

- [ ] `JsonDeserializers.NullDeserializer.isValid(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:225)
- [ ] `JsonDeserializers.NullDeserializer.deserializeIfValid(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:230)
- [ ] `JsonDeserializers.NullDeserializer.deserialize(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonDeserializers.java:235)

### `JsonHelper`

- [x] `JsonHelper.LOGGER` (com/dtteam/dynamictrees/deserialization/JsonHelper.java:23)
- [x] `JsonHelper.GSON` (com/dtteam/dynamictrees/deserialization/JsonHelper.java:24)
- [ ] `JsonHelper.getGson(0)`: it stores a second reference to an object (Rust would need it copied or shared) (com/dtteam/dynamictrees/deserialization/JsonHelper.java:26)
- [ ] `JsonHelper.load(1)`: it uses `File` (java.io) (com/dtteam/dynamictrees/deserialization/JsonHelper.java:30)
- [ ] `JsonHelper.isComment(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/deserialization/JsonHelper.java:46)
- [ ] `JsonHelper.isComment(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/deserialization/JsonHelper.java:50)
- [ ] `JsonHelper.getOrDefault(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonHelper.java:54)
- [ ] `JsonHelper.getOrDefault(6)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonHelper.java:59)
- [ ] `JsonHelper.getAsOptional(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonHelper.java:66)
- [ ] `JsonHelper.getBlockProperties(4)`: it uses Minecraft's `class_2251` (com/dtteam/dynamictrees/deserialization/JsonHelper.java:71)
- [ ] `JsonHelper.throwIfNotJsonObject(2)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonHelper.java:79)
- [ ] `JsonHelper.throwIfShouldNotLoad(1)`: a lambda (com/dtteam/dynamictrees/deserialization/JsonHelper.java:85)
- [ ] `JsonHelper.isModLoaded(1)`: it uses JsonDeserializers.STRING, which isn't translated (it uses JsonDeserializer, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/deserialization/JsonHelper.java:106)

### `JsonMapWrapper`: it implements `Map` (java.util)

- [ ] `JsonMapWrapper.JsonMapWrapper(1)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:15)
- [ ] `JsonMapWrapper.size(0)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:19)
- [ ] `JsonMapWrapper.isEmpty(0)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:24)
- [ ] `JsonMapWrapper.containsKey(1)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:29)
- [ ] `JsonMapWrapper.containsValue(1)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:34)
- [ ] `JsonMapWrapper.get(1)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:39)
- [ ] `JsonMapWrapper.put(2)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:44)
- [ ] `JsonMapWrapper.remove(1)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:50)
- [ ] `JsonMapWrapper.putAll(1)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:55)
- [ ] `JsonMapWrapper.clear(0)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:60)
- [ ] `JsonMapWrapper.keySet(0)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:65)
- [ ] `JsonMapWrapper.values(0)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:70)
- [ ] `JsonMapWrapper.entrySet(0)`: its class isn't translated (it implements `Map` (java.util)) (com/dtteam/dynamictrees/deserialization/JsonMapWrapper.java:75)

### `JsonMath`: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)

- [ ] `JsonMath.JsonMath(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:25)
- [ ] `JsonMath.getVariable(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:45)
- [ ] `JsonMath.lazyInitNoise(0)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:57)
- [ ] `JsonMath.processElement(2)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:65)
- [ ] `JsonMath.apply(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:119)

### `JsonMath.Add`: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)

- [ ] `JsonMath.Add.Add(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:129)
- [ ] `JsonMath.Add.apply(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:133)

### `JsonMath.Div`: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)

- [ ] `JsonMath.Div.Div(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:149)
- [ ] `JsonMath.Div.apply(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:153)

### `JsonMath.IfGt`: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)

- [ ] `JsonMath.IfGt.IfGt(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:170)
- [ ] `JsonMath.IfGt.apply(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:174)

### `JsonMath.Mul`: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)

- [ ] `JsonMath.Mul.Mul(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:188)
- [ ] `JsonMath.Mul.apply(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:192)

### `JsonMath.Sub`: it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)

- [ ] `JsonMath.Sub.Sub(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:208)
- [ ] `JsonMath.Sub.apply(1)`: its class isn't translated (it implements `MathOperator` (com.dtteam.dynamictrees.deserialization.math.operator)) (com/dtteam/dynamictrees/deserialization/JsonMath.java:212)

### `JsonPropertyApplierLists`

- [ ] `JsonPropertyApplierLists.PROPERTIES`: it uses JsonPropertyAppliers, which isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonPropertyApplierLists.java:8)

### `JsonPropertyAppliers`: it's generic

- [ ] `JsonPropertyAppliers.JsonPropertyAppliers(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:23)
- [ ] `JsonPropertyAppliers.applyAll(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:27)
- [ ] `JsonPropertyAppliers.apply(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:38)
- [ ] `JsonPropertyAppliers.register(1)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:66)
- [ ] `JsonPropertyAppliers.register(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:71)
- [ ] `JsonPropertyAppliers.register(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:75)
- [ ] `JsonPropertyAppliers.registerArrayApplier(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:79)
- [ ] `JsonPropertyAppliers.registerArrayApplier(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:83)
- [ ] `JsonPropertyAppliers.registerListApplier(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:87)
- [ ] `JsonPropertyAppliers.registerListApplier(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:92)
- [ ] `JsonPropertyAppliers.registerMapApplier(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:97)
- [ ] `JsonPropertyAppliers.registerMapApplier(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:102)
- [ ] `JsonPropertyAppliers.registerIfTrueApplier(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:107)
- [ ] `JsonPropertyAppliers.register(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:111)
- [ ] `JsonPropertyAppliers.register(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:115)
- [ ] `JsonPropertyAppliers.registerIfTrueApplier(3)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:119)
- [ ] `JsonPropertyAppliers.registerArrayApplier(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:127)
- [ ] `JsonPropertyAppliers.registerArrayApplier(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:131)
- [ ] `JsonPropertyAppliers.registerListApplier(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:135)
- [ ] `JsonPropertyAppliers.registerListApplier(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:140)
- [ ] `JsonPropertyAppliers.registerMapApplier(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:147)
- [ ] `JsonPropertyAppliers.registerMapApplier(4)`: it's generic (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:154)
- [ ] `JsonPropertyAppliers.getObjectType(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:161)
- [ ] `JsonPropertyAppliers.getInputType(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/JsonPropertyAppliers.java:166)

### `MathFunction`: its constants carry values

- [ ] `MathFunction.getFunction(1)`: it uses MathFunction, which isn't translated (its constants carry values) (com/dtteam/dynamictrees/deserialization/MathFunction.java:25)

### `ArithmeticType`


### `BooleanType`


### `ComparisonType`


### `ExpressionParser`

- [ ] `ExpressionParser.CACHE`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:32)
- [ ] `ExpressionParser.FUNCTIONS`: it uses MathOperatorBuilder, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:33)
- [ ] `ExpressionParser.VARIABLES`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:34)
- [ ] `ExpressionParser.parse(1)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:38)
- [x] `ExpressionParser.sanitizeInput(1)` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:50)
- [x] `ExpressionParser.ExpressionParser(1)` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:55)
- [ ] `ExpressionParser.parse(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:59)
- [ ] `ExpressionParser.parseBoolean(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:68)
- [ ] `ExpressionParser.parseComparison(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:88)
- [ ] `ExpressionParser.parseAdditive(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:114)
- [ ] `ExpressionParser.parseMultiplicative(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:130)
- [ ] `ExpressionParser.parseUnary(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:150)
- [ ] `ExpressionParser.parsePrimary(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:160)
- [ ] `ExpressionParser.parseNumber(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:175)
- [ ] `ExpressionParser.parseFunction(1)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:200)
- [ ] `ExpressionParser.parseIsSpeciesFunction(0)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:227)
- [ ] `ExpressionParser.parseSpeciesName(0)`: it calls ExpressionParser.parseString, which isn't translated (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:244)
- [ ] `ExpressionParser.parseVariable(1)`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:257)
- [ ] `ExpressionParser.parseIdentifier(0)`: `++` inside an expression (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:265)
- [ ] `ExpressionParser.parseString(0)`: `++` inside an expression (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:279)
- [ ] `ExpressionParser.expect(2)`: it calls ExpressionParser.consume, which isn't translated (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:304)
- [ ] `ExpressionParser.consume(1)`: String.startsWith with 2 values (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:310)
- [x] `ExpressionParser.isNumberStart(0)` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:319)
- [x] `ExpressionParser.isIdentifierStart(1)` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:324)
- [x] `ExpressionParser.isIdentifierPart(1)` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:328)
- [x] `ExpressionParser.peek(0)` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:332)
- [x] `ExpressionParser.isAtEnd(0)` (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:336)
- [ ] `ExpressionParser.error(1)`: it uses `JsonParseException` (com.google.gson) (com/dtteam/dynamictrees/deserialization/math/ExpressionParser.java:340)

### `MathContext`: it uses Minecraft's `class_2382`

- [ ] `MathContext.MathContext(4)`: its class isn't translated (it uses Minecraft's `class_2382`) (com/dtteam/dynamictrees/deserialization/math/MathContext.java:7)
- [ ] `MathContext.MathContext(1)`: its class isn't translated (it uses Minecraft's `class_2382`) (com/dtteam/dynamictrees/deserialization/math/MathContext.java:8)
- [ ] `MathContext.MathContext(2)`: its class isn't translated (it uses Minecraft's `class_2382`) (com/dtteam/dynamictrees/deserialization/math/MathContext.java:12)
- [ ] `MathContext.pos(0)`: its class isn't translated (it uses Minecraft's `class_2382`) (com/dtteam/dynamictrees/deserialization/math/MathContext.java:7)
- [ ] `MathContext.rand(0)`: its class isn't translated (it uses Minecraft's `class_2382`) (com/dtteam/dynamictrees/deserialization/math/MathContext.java:7)
- [ ] `MathContext.species(0)`: its class isn't translated (it uses Minecraft's `class_2382`) (com/dtteam/dynamictrees/deserialization/math/MathContext.java:7)
- [ ] `MathContext.radius(0)`: its class isn't translated (it uses Minecraft's `class_2382`) (com/dtteam/dynamictrees/deserialization/math/MathContext.java:7)

### `NoiseModel`: it's an interface

- [ ] `NoiseModel.sample(3)`: it has no body (com/dtteam/dynamictrees/deserialization/math/noise/NoiseModel.java:4)

### `NoiseType`


### `PerlinNoiseModel`: it implements `NoiseModel`

- [ ] `PerlinNoiseModel.PerlinNoiseModel(2)`: its class isn't translated (it implements `NoiseModel`) (com/dtteam/dynamictrees/deserialization/math/noise/PerlinNoiseModel.java:10)
- [ ] `PerlinNoiseModel.sample(3)`: its class isn't translated (it implements `NoiseModel`) (com/dtteam/dynamictrees/deserialization/math/noise/PerlinNoiseModel.java:14)

### `PerlinSimplexNoiseModel`: it implements `NoiseModel`

- [ ] `PerlinSimplexNoiseModel.PerlinSimplexNoiseModel(2)`: its class isn't translated (it implements `NoiseModel`) (com/dtteam/dynamictrees/deserialization/math/noise/PerlinSimplexNoiseModel.java:10)
- [ ] `PerlinSimplexNoiseModel.sample(3)`: its class isn't translated (it implements `NoiseModel`) (com/dtteam/dynamictrees/deserialization/math/noise/PerlinSimplexNoiseModel.java:14)

### `SimplexNoiseModel`: it implements `NoiseModel`

- [ ] `SimplexNoiseModel.SimplexNoiseModel(1)`: its class isn't translated (it implements `NoiseModel`) (com/dtteam/dynamictrees/deserialization/math/noise/SimplexNoiseModel.java:9)
- [ ] `SimplexNoiseModel.sample(3)`: its class isn't translated (it implements `NoiseModel`) (com/dtteam/dynamictrees/deserialization/math/noise/SimplexNoiseModel.java:13)

### `ArithmeticOperator`: it implements `MathOperator`

- [ ] `ArithmeticOperator.ArithmeticOperator(3)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/ArithmeticOperator.java:11)
- [ ] `ArithmeticOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/ArithmeticOperator.java:17)

### `AverageOperator`: it implements `MathOperator`

- [ ] `AverageOperator.AverageOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/AverageOperator.java:8)
- [ ] `AverageOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/AverageOperator.java:13)

### `BiFunctionOperator`: it implements `MathOperator`

- [ ] `BiFunctionOperator.BiFunctionOperator(2)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/BiFunctionOperator.java:11)
- [ ] `BiFunctionOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/BiFunctionOperator.java:18)

### `BooleanLogicOperator`: it implements `MathOperator`

- [ ] `BooleanLogicOperator.BooleanLogicOperator(3)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/BooleanLogicOperator.java:11)
- [ ] `BooleanLogicOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/BooleanLogicOperator.java:17)
- [x] `BooleanLogicOperator.isTrue(1)` (com/dtteam/dynamictrees/deserialization/math/operator/BooleanLogicOperator.java:48)

### `ClampOperator`: it implements `MathOperator`

- [ ] `ClampOperator.ClampOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/ClampOperator.java:11)
- [ ] `ClampOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/ClampOperator.java:18)

### `ComparisonOperator`: it implements `MathOperator`

- [ ] `ComparisonOperator.ComparisonOperator(3)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/ComparisonOperator.java:11)
- [ ] `ComparisonOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/ComparisonOperator.java:17)

### `Const`: it implements `MathOperator`

- [ ] `Const.Const(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Const.java:8)
- [ ] `Const.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Const.java:12)

### `Debug`: it implements `MathOperator`

- [ ] `Debug.Debug(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Debug.java:9)
- [ ] `Debug.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Debug.java:14)

### `FunctionOperator`: it implements `MathOperator`

- [ ] `FunctionOperator.FunctionOperator(2)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/FunctionOperator.java:10)
- [ ] `FunctionOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/FunctionOperator.java:16)

### `IfOperator`: it implements `MathOperator`

- [ ] `IfOperator.IfOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/IfOperator.java:8)
- [ ] `IfOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/IfOperator.java:13)

### `IfSpecies`: it implements `MathOperator`

- [ ] `IfSpecies.IfSpecies(2)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/IfSpecies.java:11)
- [ ] `IfSpecies.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/IfSpecies.java:16)

### `IsSpecies`: it implements `MathOperator`

- [ ] `IsSpecies.IsSpecies(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/IsSpecies.java:10)
- [ ] `IsSpecies.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/IsSpecies.java:15)

### `LerpOperator`: it implements `MathOperator`

- [ ] `LerpOperator.LerpOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/LerpOperator.java:11)
- [ ] `LerpOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/LerpOperator.java:18)

### `LogOperator`: it implements `MathOperator`

- [x] `LogOperator.LOG2` (com/dtteam/dynamictrees/deserialization/math/operator/LogOperator.java:6)
- [ ] `LogOperator.LogOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/LogOperator.java:10)
- [ ] `LogOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/LogOperator.java:16)

### `MapOperator`: it implements `MathOperator`

- [ ] `MapOperator.MapOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/MapOperator.java:13)
- [ ] `MapOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/MapOperator.java:22)

### `MathOperator`: it's an interface

- [ ] `MathOperator.apply(1)`: it has no body (com/dtteam/dynamictrees/deserialization/math/operator/MathOperator.java:6)
- [ ] `MathOperator.throwIfInvalidParameterLength(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/operator/MathOperator.java:8)
- [ ] `MathOperator.throwIfInvalidParameterLengthRange(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/operator/MathOperator.java:14)
- [ ] `MathOperator.throwIfInvalidParameterLengthMin(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/operator/MathOperator.java:20)

### `MathOperatorBuilder`: it's an interface

- [ ] `MathOperatorBuilder.build(1)`: it has no body (com/dtteam/dynamictrees/deserialization/math/operator/MathOperatorBuilder.java:4)

### `MaxOperator`: it implements `MathOperator`

- [ ] `MaxOperator.MaxOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/MaxOperator.java:8)
- [ ] `MaxOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/MaxOperator.java:13)

### `MinOperator`: it implements `MathOperator`

- [ ] `MinOperator.MinOperator(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/MinOperator.java:8)
- [ ] `MinOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/MinOperator.java:13)

### `Noise`: it implements `MathOperator`

- [x] `Noise.BASE_SEED` (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java:15)
- [x] `Noise.DEFAULT_SCALE` (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java:16)
- [ ] `Noise.Noise(4)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java:22)
- [ ] `Noise.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java:29)
- [x] `Noise.normalize(1)` (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java:37)
- [ ] `Noise.build(1)`: it uses Noise, which isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java:41)
- [ ] `Noise.build(2)`: it uses Noise, which isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Noise.java:45)

### `NullOperator`: it implements `MathOperator`

- [ ] `NullOperator.NULL`: it uses MathOperator, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/math/operator/NullOperator.java:6)
- [ ] `NullOperator.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/NullOperator.java:8)

### `Scale`: it implements `MathOperator`

- [ ] `Scale.Scale(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Scale.java:12)
- [ ] `Scale.apply(1)`: its class isn't translated (it implements `MathOperator`) (com/dtteam/dynamictrees/deserialization/math/operator/Scale.java:17)

### `NoSuchDeserializerException`: it extends `RuntimeException`

- [ ] `NoSuchDeserializerException.NoSuchDeserializerException(1)`: its class isn't translated (it extends `RuntimeException`) (com/dtteam/dynamictrees/deserialization/NoSuchDeserializerException.java:4)

### `PropertyAppliers`: it's an interface

- [ ] `PropertyAppliers.applyAll(2)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:15)
- [ ] `PropertyAppliers.apply(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:17)
- [ ] `PropertyAppliers.register(1)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:19)
- [ ] `PropertyAppliers.register(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:21)
- [ ] `PropertyAppliers.register(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:23)
- [ ] `PropertyAppliers.registerArrayApplier(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:25)
- [ ] `PropertyAppliers.registerArrayApplier(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:27)
- [ ] `PropertyAppliers.registerListApplier(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:29)
- [ ] `PropertyAppliers.registerListApplier(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:31)
- [ ] `PropertyAppliers.registerMapApplier(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:33)
- [ ] `PropertyAppliers.registerMapApplier(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:35)
- [ ] `PropertyAppliers.registerIfTrueApplier(2)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:37)
- [ ] `PropertyAppliers.register(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:39)
- [ ] `PropertyAppliers.register(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:41)
- [ ] `PropertyAppliers.registerIfTrueApplier(3)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:43)
- [ ] `PropertyAppliers.registerArrayApplier(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:45)
- [ ] `PropertyAppliers.registerArrayApplier(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:47)
- [ ] `PropertyAppliers.registerListApplier(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:49)
- [ ] `PropertyAppliers.registerListApplier(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:51)
- [ ] `PropertyAppliers.registerMapApplier(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:53)
- [ ] `PropertyAppliers.registerMapApplier(4)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:55)
- [ ] `PropertyAppliers.getObjectType(0)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:57)
- [ ] `PropertyAppliers.getInputType(0)`: it has no body (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:59)

### `PropertyAppliers.ResultList`: it extends `LinkedList` (java.util)

- [ ] `PropertyAppliers.ResultList.forEachError(1)`: its class isn't translated (it extends `LinkedList` (java.util)) (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:62)
- [ ] `PropertyAppliers.ResultList.forEachWarning(1)`: its class isn't translated (it extends `LinkedList` (java.util)) (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:67)
- [ ] `PropertyAppliers.ResultList.forEachErrorWarning(2)`: its class isn't translated (it extends `LinkedList` (java.util)) (com/dtteam/dynamictrees/deserialization/PropertyAppliers.java:72)

### `AbstractResult`: it's generic

- [ ] `AbstractResult.AbstractResult(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:19)
- [ ] `AbstractResult.AbstractResult(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:23)
- [ ] `AbstractResult.getInput(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:30)
- [ ] `AbstractResult.get(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:35)
- [ ] `AbstractResult.orElse(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:44)
- [ ] `AbstractResult.orElseGet(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:49)
- [ ] `AbstractResult.orElseThrow(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:54)
- [ ] `AbstractResult.getError(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:63)
- [ ] `AbstractResult.forEachWarning(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:69)
- [ ] `AbstractResult.getWarnings(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:75)
- [ ] `AbstractResult.success(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/AbstractResult.java:80)

### `JsonResult`: it's generic

- [ ] `JsonResult.JsonResult(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:16)
- [ ] `JsonResult.JsonResult(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:20)
- [ ] `JsonResult.removeError(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:24)
- [ ] `JsonResult.map(1)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:29)
- [ ] `JsonResult.map(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:38)
- [ ] `JsonResult.mapIfValid(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:54)
- [ ] `JsonResult.mapIfType(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:65)
- [ ] `JsonResult.mapToListOfType(1)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:70)
- [ ] `JsonResult.mapEachIfArray(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:84)
- [ ] `JsonResult.mapIfContains(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:98)
- [ ] `JsonResult.mapIfContains(4)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:110)
- [ ] `JsonResult.forInput(1)`: it uses JsonResult, which isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:118)
- [ ] `JsonResult.success(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:122)
- [ ] `JsonResult.nullSuccess(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:126)
- [ ] `JsonResult.failure(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:130)
- [ ] `JsonResult.failure(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:134)
- [ ] `JsonResult.from(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/JsonResult.java:138)

### `MappedJsonResult`: it's generic

- [ ] `MappedJsonResult.MappedJsonResult(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:9)
- [ ] `MappedJsonResult.MappedJsonResult(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:13)
- [ ] `MappedJsonResult.elseMapIfType(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:17)
- [ ] `MappedJsonResult.elseMapIfContains(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:22)
- [ ] `MappedJsonResult.elseMapIfContains(4)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:27)
- [ ] `MappedJsonResult.elseError(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:32)
- [ ] `MappedJsonResult.mapErrorneous(1)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:37)
- [ ] `MappedJsonResult.map(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:41)
- [ ] `MappedJsonResult.errorneousMap(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedJsonResult.java:45)

### `MappedResult`: it's an interface

- [ ] `MappedResult.elseMapIfType(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:7)
- [ ] `MappedResult.elseMapIfType(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:11)
- [ ] `MappedResult.elseMapIfContains(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:13)
- [ ] `MappedResult.elseMapIfContains(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:17)
- [ ] `MappedResult.elseMapIfContains(4)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:19)
- [ ] `MappedResult.elseMapIfContains(4)`: it's generic (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:23)
- [ ] `MappedResult.elseTypeError(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:25)
- [ ] `MappedResult.elseError(2)`: it has no body (com/dtteam/dynamictrees/deserialization/result/MappedResult.java:29)

### `NullJsonResult`: it's generic

- [ ] `NullJsonResult.NullJsonResult(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/NullJsonResult.java:8)
- [ ] `NullJsonResult.NullJsonResult(4)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/NullJsonResult.java:12)
- [ ] `NullJsonResult.success(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/result/NullJsonResult.java:16)

### `Result`: it's an interface

- [ ] `Result.getInput(0)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:17)
- [ ] `Result.get(0)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:19)
- [ ] `Result.ifSuccess(1)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/Result.java:21)
- [ ] `Result.ifSuccessOrElse(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/Result.java:29)
- [ ] `Result.ifSuccessOrElseThrow(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/Result.java:40)
- [ ] `Result.orElse(1)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:51)
- [ ] `Result.orElse(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/Result.java:53)
- [ ] `Result.orElseApply(2)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/Result.java:62)
- [ ] `Result.orElseApply(3)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/Result.java:70)
- [ ] `Result.orElseGet(1)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:84)
- [ ] `Result.orElseThrow(0)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:86)
- [ ] `Result.getError(0)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:88)
- [ ] `Result.forEachWarning(1)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:91)
- [ ] `Result.getWarnings(0)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:93)
- [ ] `Result.success(0)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:95)
- [ ] `Result.removeError(0)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:97)
- [ ] `Result.map(1)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:99)
- [ ] `Result.map(1)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:103)
- [ ] `Result.map(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:105)
- [ ] `Result.map(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:109)
- [ ] `Result.map(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:113)
- [ ] `Result.map(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:117)
- [ ] `Result.mapIfValid(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:119)
- [ ] `Result.mapIfValid(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:123)
- [ ] `Result.mapIfType(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:125)
- [ ] `Result.mapIfType(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:129)
- [ ] `Result.mapToListOfType(1)`: it has no body (com/dtteam/dynamictrees/deserialization/result/Result.java:131)
- [ ] `Result.mapEachIfArray(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:133)
- [ ] `Result.mapEachIfArray(2)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:137)
- [ ] `Result.mapIfContains(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:139)
- [ ] `Result.mapIfContains(3)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:143)
- [ ] `Result.mapIfContains(4)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:145)
- [ ] `Result.mapIfContains(4)`: it's generic (com/dtteam/dynamictrees/deserialization/result/Result.java:149)

### `Result.Mapper`: it's an interface


### `Result.SimpleMapper`: it's an interface

- [ ] `Result.SimpleMapper.fullMapper(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/result/Result.java:157)

### `ShapeFunctions`

- [ ] `ShapeFunctions.SHAPE_FUNCTIONS`: it uses ShapeFunction, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:19)
- [ ] `ShapeFunctions.registerShapeFunction(2)`: it uses ShapeFunction, which isn't translated (it's an interface) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:26)
- [ ] `ShapeFunctions.calculateShape(2)`: it uses Minecraft's `DataResult` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:30)
- [ ] `ShapeFunctions.calculateShape(2)`: it uses Minecraft's `DataResult` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:37)

### `ShapeFunctions.FruitShapeFunction`: it implements `ShapeFunction`

- [ ] `ShapeFunctions.FruitShapeFunction.getParameters(1)`: its class isn't translated (it implements `ShapeFunction`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:43)
- [ ] `ShapeFunctions.FruitShapeFunction.calculateShape(1)`: its class isn't translated (it implements `ShapeFunction`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:48)
- [ ] `ShapeFunctions.FruitShapeFunction.createFruitShape(4)`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:56)

### `ShapeFunctions.FruitShapeFunction.Parameters`

- [ ] `ShapeFunctions.FruitShapeFunction.Parameters.CODEC`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:68)
- [x] `ShapeFunctions.FruitShapeFunction.Parameters.Parameters(4)` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:67)
- [x] `ShapeFunctions.FruitShapeFunction.Parameters.width(0)` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:67)
- [x] `ShapeFunctions.FruitShapeFunction.Parameters.height(0)` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:67)
- [x] `ShapeFunctions.FruitShapeFunction.Parameters.stemLength(0)` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:67)
- [x] `ShapeFunctions.FruitShapeFunction.Parameters.fraction(0)` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:67)

### `ShapeFunctions.PodShapeFunction`: it implements `ShapeFunction`

- [ ] `ShapeFunctions.PodShapeFunction.getParameters(1)`: its class isn't translated (it implements `ShapeFunction`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:81)
- [ ] `ShapeFunctions.PodShapeFunction.calculateShape(1)`: its class isn't translated (it implements `ShapeFunction`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:86)
- [ ] `ShapeFunctions.PodShapeFunction.offsetBoundingBox(3)`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:98)

### `ShapeFunctions.PodShapeFunction.Parameters`: it uses Minecraft's `class_2350`

- [ ] `ShapeFunctions.PodShapeFunction.Parameters.SIDE_CODEC`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:103)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.CODEC`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:105)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.Parameters(6)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:102)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.width(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:102)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.height(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:102)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.stemLength(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:102)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.fraction(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:102)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.sideOffset(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:102)
- [ ] `ShapeFunctions.PodShapeFunction.Parameters.side(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:102)

### `ShapeFunctions.ShapeFunction`: it's an interface

- [ ] `ShapeFunctions.ShapeFunction.getParameters(1)`: it has no body (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:120)
- [ ] `ShapeFunctions.ShapeFunction.calculateShape(1)`: it has no body (com/dtteam/dynamictrees/deserialization/ShapeFunctions.java:122)

### `TagKeyJsonPropertyApplier`: it's generic

- [ ] `TagKeyJsonPropertyApplier.TagKeyJsonPropertyApplier(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/TagKeyJsonPropertyApplier.java:20)
- [ ] `TagKeyJsonPropertyApplier.TagKeyJsonPropertyApplier(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/TagKeyJsonPropertyApplier.java:27)
- [ ] `TagKeyJsonPropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/TagKeyJsonPropertyApplier.java:35)
- [ ] `TagKeyJsonPropertyApplier.applyIfShould(3)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/deserialization/TagKeyJsonPropertyApplier.java:52)

### `DynamicTrees`

- [x] `DynamicTrees.MOD_ID` (com/dtteam/dynamictrees/DynamicTrees.java:15)
- [x] `DynamicTrees.MOD_NAME` (com/dtteam/dynamictrees/DynamicTrees.java:16)
- [x] `DynamicTrees.LOG` (com/dtteam/dynamictrees/DynamicTrees.java:17)
- [x] `DynamicTrees.MINECRAFT` (com/dtteam/dynamictrees/DynamicTrees.java:18)
- [x] `DynamicTrees.SERENE_SEASONS` (com/dtteam/dynamictrees/DynamicTrees.java:19)
- [x] `DynamicTrees.FAST_LEAF_DECAY` (com/dtteam/dynamictrees/DynamicTrees.java:20)
- [x] `DynamicTrees.PASSABLE_FOLIAGE` (com/dtteam/dynamictrees/DynamicTrees.java:21)
- [x] `DynamicTrees.DYNAMIC_TREES_PLUS` (com/dtteam/dynamictrees/DynamicTrees.java:22)
- [ ] `DynamicTrees.NULL`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:23)
- [ ] `DynamicTrees.OAK`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:24)
- [ ] `DynamicTrees.BIRCH`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:25)
- [ ] `DynamicTrees.SPRUCE`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:26)
- [ ] `DynamicTrees.JUNGLE`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:27)
- [ ] `DynamicTrees.DARK_OAK`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:28)
- [ ] `DynamicTrees.ACACIA`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:29)
- [ ] `DynamicTrees.AZALEA`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:30)
- [ ] `DynamicTrees.CRIMSON`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:31)
- [ ] `DynamicTrees.WARPED`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:32)
- [ ] `DynamicTrees.SETUP_HANDLERS`: a function kept in a variable or field (only parameters can be) (com/dtteam/dynamictrees/DynamicTrees.java:33)
- [ ] `DynamicTrees.init(0)`: it calls SeasonCompatibilityHandler.registerBuiltInSeasonManagers, which isn't translated (com/dtteam/dynamictrees/DynamicTrees.java:35)
- [ ] `DynamicTrees.runOnCommonSetup(1)`: it uses DynamicTrees.SETUP_HANDLERS, which isn't translated (a function kept in a variable or field (only parameters can be)) (com/dtteam/dynamictrees/DynamicTrees.java:39)
- [ ] `DynamicTrees.commonSetup(0)`: it calls DynamicTreeFeature.setup, which isn't translated (com/dtteam/dynamictrees/DynamicTrees.java:43)
- [ ] `DynamicTrees.location(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTrees.java:51)

### `DynamicTrees.AxeDamage`


### `DynamicTrees.DestroyMode`


### `DynamicTreesFabric`: it implements Fabric's `ModInitializer`

- [ ] `DynamicTreesFabric.onInitialize(0)`: its class isn't translated (it implements Fabric's `ModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabric.java:24)
- [ ] `DynamicTreesFabric.getEntryPointContainers(0)`: it uses Fabric's `EntrypointContainer` (com/dtteam/dynamictrees/DynamicTreesFabric.java:43)
- [ ] `DynamicTreesFabric.runOnEntryPoints(2)`: it uses Fabric's `EntrypointContainer` (com/dtteam/dynamictrees/DynamicTreesFabric.java:47)

### `DynamicTreesFabricClient`: it implements Fabric's `ClientModInitializer`

- [x] `DynamicTreesFabricClient.initialized` (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:61)
- [x] `DynamicTreesFabricClient.woodColorsDiscovered` (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:62)
- [ ] `DynamicTreesFabricClient.onInitializeClient(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:64)
- [ ] `DynamicTreesFabricClient.registerClientWorldLoad(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:75)
- [ ] `DynamicTreesFabricClient.registerModelLoaders(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:87)
- [ ] `DynamicTreesFabricClient.registerEntityRenderers(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:91)
- [ ] `DynamicTreesFabricClient.registerColorHandlers(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:96)
- [ ] `DynamicTreesFabricClient.registerBlockColors(0)`: it uses Minecraft's `class_324` (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:103)
- [ ] `DynamicTreesFabricClient.isValidPos(2)`: it uses Minecraft's `class_1922` (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:166)
- [ ] `DynamicTreesFabricClient.registerTooltipCallback(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:170)
- [ ] `DynamicTreesFabricClient.registerClientTick(0)`: its class isn't translated (it implements Fabric's `ClientModInitializer`) (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:190)
- [ ] `DynamicTreesFabricClient.discoverWoodColors(0)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:194)
- [ ] `DynamicTreesFabricClient.getFaceColor(3)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/DynamicTreesFabricClient.java:210)

### `AnimationConstants`

- [x] `AnimationConstants.TREE_GRAVITY` (com/dtteam/dynamictrees/entity/animation/AnimationConstants.java:11)
- [x] `AnimationConstants.TREE_ELASTICITY` (com/dtteam/dynamictrees/entity/animation/AnimationConstants.java:12)
- [ ] `AnimationConstants.TREE_DAMAGE_TYPE`: it uses Minecraft's `class_5321` (com/dtteam/dynamictrees/entity/animation/AnimationConstants.java:13)
- [ ] `AnimationConstants.treeDamage(1)`: it uses Minecraft's `class_1282` (com/dtteam/dynamictrees/entity/animation/AnimationConstants.java:15)

### `AnimationHandler`: it's an interface

- [ ] `AnimationHandler.getName(0)`: it has no body (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java:7)
- [ ] `AnimationHandler.initMotion(1)`: it has no body (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java:9)
- [ ] `AnimationHandler.handleMotion(1)`: it has no body (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java:11)
- [ ] `AnimationHandler.dropPayload(1)`: it has no body (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java:13)
- [ ] `AnimationHandler.shouldDie(1)`: it has no body (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java:15)
- [ ] `AnimationHandler.renderTransform(4)`: it has no body (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java:17)
- [ ] `AnimationHandler.shouldRender(4)`: it has no body (com/dtteam/dynamictrees/entity/animation/AnimationHandler.java:19)

### `AnimationHandlers`

- [ ] `AnimationHandlers.voidAnimationHandler`: it uses AnimationHandler, which isn't translated (it's an interface) (com/dtteam/dynamictrees/entity/animation/AnimationHandlers.java:7)
- [ ] `AnimationHandlers.defaultAnimationHandler`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/entity/animation/AnimationHandlers.java:8)
- [ ] `AnimationHandlers.blastAnimationHandler`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/entity/animation/AnimationHandlers.java:21)
- [ ] `AnimationHandlers.falloverAnimationHandler`: it uses AnimationHandler, which isn't translated (it's an interface) (com/dtteam/dynamictrees/entity/animation/AnimationHandlers.java:37)

### `DataAnimationHandler`


### `FalloverAnimationHandler`: it implements `AnimationHandler`

- [x] `FalloverAnimationHandler.TICKS_BEFORE_CHECKING_COLLISION` (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:35)
- [ ] `FalloverAnimationHandler.getName(0)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:37)
- [ ] `FalloverAnimationHandler.getData(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:42)
- [ ] `FalloverAnimationHandler.playStartSound(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:48)
- [ ] `FalloverAnimationHandler.playEndSound(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:56)
- [ ] `FalloverAnimationHandler.playFallThroughWaterSound(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:69)
- [ ] `FalloverAnimationHandler.rotateAroundAxis(3)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:76)
- [ ] `FalloverAnimationHandler.flingLeavesParticles(2)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:90)
- [ ] `FalloverAnimationHandler.spawnParticlesAtLeaves(7)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:124)
- [ ] `FalloverAnimationHandler.getRelativeLeavesPosition(2)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:145)
- [ ] `FalloverAnimationHandler.initMotion(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:157)
- [ ] `FalloverAnimationHandler.handleMotion(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:168)
- [ ] `FalloverAnimationHandler.testCollision(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:251)
- [ ] `FalloverAnimationHandler.addRotation(2)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:284)
- [ ] `FalloverAnimationHandler.testEntityCollision(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:304)
- [ ] `FalloverAnimationHandler.intersects(3)`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:339)
- [ ] `FalloverAnimationHandler.dropPayload(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:350)
- [ ] `FalloverAnimationHandler.shouldDie(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:357)
- [ ] `FalloverAnimationHandler.renderTransform(4)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:370)
- [ ] `FalloverAnimationHandler.shouldRender(4)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/FalloverAnimationHandler.java:384)

### `FalloverAnimationHandler.HandlerData`: it extends `DataAnimationHandler`


### `PhysicsAnimationHandler`: it implements `AnimationHandler`

- [ ] `PhysicsAnimationHandler.getName(0)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:27)
- [ ] `PhysicsAnimationHandler.getData(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:32)
- [ ] `PhysicsAnimationHandler.playEndSound(2)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:38)
- [ ] `PhysicsAnimationHandler.initMotion(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:52)
- [ ] `PhysicsAnimationHandler.handleMotion(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:70)
- [ ] `PhysicsAnimationHandler.dropPayload(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:136)
- [ ] `PhysicsAnimationHandler.shouldDie(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:143)
- [ ] `PhysicsAnimationHandler.renderTransform(4)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:148)
- [ ] `PhysicsAnimationHandler.shouldRender(4)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/PhysicsAnimationHandler.java:159)

### `PhysicsAnimationHandler.HandlerData`: it extends `DataAnimationHandler`


### `VoidAnimationHandler`: it implements `AnimationHandler`

- [ ] `VoidAnimationHandler.getName(0)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java:7)
- [ ] `VoidAnimationHandler.shouldDie(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java:12)
- [ ] `VoidAnimationHandler.renderTransform(4)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java:17)
- [ ] `VoidAnimationHandler.initMotion(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java:21)
- [ ] `VoidAnimationHandler.handleMotion(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java:27)
- [ ] `VoidAnimationHandler.dropPayload(1)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java:31)
- [ ] `VoidAnimationHandler.shouldRender(4)`: its class isn't translated (it implements `AnimationHandler`) (com/dtteam/dynamictrees/entity/animation/VoidAnimationHandler.java:35)

### `FallingTreeEntity`: it extends Minecraft's `class_1297`

- [ ] `FallingTreeEntity.voxelDataParameter`: it uses Minecraft's `class_2940` (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:45)
- [ ] `FallingTreeEntity.AnimHandlerFall`: it uses AnimationHandler, which isn't translated (it's an interface) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:61)
- [ ] `FallingTreeEntity.AnimHandlerDrop`: it uses AnimationHandler, which isn't translated (it's an interface) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:62)
- [ ] `FallingTreeEntity.AnimHandlerBurn`: it uses AnimationHandler, which isn't translated (it's an interface) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:63)
- [ ] `FallingTreeEntity.AnimHandlerFling`: it uses AnimationHandler, which isn't translated (it's an interface) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:64)
- [ ] `FallingTreeEntity.AnimHandlerBlast`: it uses AnimationHandler, which isn't translated (it's an interface) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:65)
- [ ] `FallingTreeEntity.FallingTreeEntity(2)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:69)
- [ ] `FallingTreeEntity.isClientBuilt(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:73)
- [ ] `FallingTreeEntity.setData(3)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:77)
- [ ] `FallingTreeEntity.buildVoxelData(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:114)
- [ ] `FallingTreeEntity.setupFromNBT(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:130)
- [ ] `FallingTreeEntity.buildClient(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:147)
- [ ] `FallingTreeEntity.cleanupShellBlocks(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:170)
- [ ] `FallingTreeEntity.buildAABBFromDestroyData(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:187)
- [ ] `FallingTreeEntity.method_5830(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:207)
- [ ] `FallingTreeEntity.getDestroyData(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:211)
- [ ] `FallingTreeEntity.getPayload(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:215)
- [ ] `FallingTreeEntity.getGeomCenter(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:219)
- [ ] `FallingTreeEntity.getMassCenter(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:223)
- [ ] `FallingTreeEntity.getVolume(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:227)
- [ ] `FallingTreeEntity.hasLeaves(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:231)
- [ ] `FallingTreeEntity.getSpecies(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:235)
- [ ] `FallingTreeEntity.method_5814(3)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:239)
- [ ] `FallingTreeEntity.method_5773(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:245)
- [ ] `FallingTreeEntity.updateNeighbors(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:270)
- [ ] `FallingTreeEntity.selectAnimationHandler(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:290)
- [ ] `FallingTreeEntity.defaultAnimationHandler(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:294)
- [ ] `FallingTreeEntity.modelCleanup(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:308)
- [ ] `FallingTreeEntity.handleMotion(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:315)
- [ ] `FallingTreeEntity.dropPayLoad(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:324)
- [ ] `FallingTreeEntity.shouldDie(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:330)
- [ ] `FallingTreeEntity.method_5727(3)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:334)
- [ ] `FallingTreeEntity.standardDropLogsPayload(1)`: it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:338)
- [ ] `FallingTreeEntity.standardDropLeavesPayLoad(1)`: it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:346)
- [ ] `FallingTreeEntity.spawnItemAsEntity(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:354)
- [ ] `FallingTreeEntity.method_5693(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:366)
- [ ] `FallingTreeEntity.setVoxelData(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:370)
- [ ] `FallingTreeEntity.getVoxelData(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:376)
- [ ] `FallingTreeEntity.method_5749(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:380)
- [ ] `FallingTreeEntity.method_5652(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:395)
- [ ] `FallingTreeEntity.dropTree(4)`: it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/FallingTreeEntity.java:408)

### `FallingTreeEntity.DestroyType`


### `LingeringEffectorEntity`: it extends Minecraft's `class_1297`

- [ ] `LingeringEffectorEntity.effectorDataParameter`: it uses Minecraft's `class_2940` (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:21)
- [ ] `LingeringEffectorEntity.LingeringEffectorEntity(2)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:27)
- [ ] `LingeringEffectorEntity.setData(3)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:31)
- [ ] `LingeringEffectorEntity.treeHasEffectorForEffect(3)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:46)
- [ ] `LingeringEffectorEntity.setBlockPos(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:56)
- [ ] `LingeringEffectorEntity.getBlockPos(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:61)
- [ ] `LingeringEffectorEntity.getEffect(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:65)
- [ ] `LingeringEffectorEntity.method_5693(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:69)
- [ ] `LingeringEffectorEntity.method_5749(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:73)
- [ ] `LingeringEffectorEntity.method_5652(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:80)
- [ ] `LingeringEffectorEntity.buildEffectorData(2)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:84)
- [ ] `LingeringEffectorEntity.setEffectorData(1)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:93)
- [ ] `LingeringEffectorEntity.getEffectorData(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:99)
- [ ] `LingeringEffectorEntity.method_5773(0)`: its class isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/entity/LingeringEffectorEntity.java:103)

### `FallingTreeRenderer`: it extends Minecraft's `class_897`

- [ ] `FallingTreeRenderer.FallingTreeRenderer(1)`: its class isn't translated (it extends Minecraft's `class_897`) (com/dtteam/dynamictrees/entity/render/FallingTreeRenderer.java:18)
- [ ] `FallingTreeRenderer.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `class_897`) (com/dtteam/dynamictrees/entity/render/FallingTreeRenderer.java:22)
- [ ] `FallingTreeRenderer.render(6)`: its class isn't translated (it extends Minecraft's `class_897`) (com/dtteam/dynamictrees/entity/render/FallingTreeRenderer.java:26)

### `LingeringEffectorRenderer`: it extends Minecraft's `class_897`

- [ ] `LingeringEffectorRenderer.LingeringEffectorRenderer(1)`: its class isn't translated (it extends Minecraft's `class_897`) (com/dtteam/dynamictrees/entity/render/LingeringEffectorRenderer.java:11)
- [ ] `LingeringEffectorRenderer.shouldRender(5)`: its class isn't translated (it extends Minecraft's `class_897`) (com/dtteam/dynamictrees/entity/render/LingeringEffectorRenderer.java:15)
- [ ] `LingeringEffectorRenderer.getTextureLocation(1)`: its class isn't translated (it extends Minecraft's `class_897`) (com/dtteam/dynamictrees/entity/render/LingeringEffectorRenderer.java:19)

### `CommonEventHandler`

- [ ] `CommonEventHandler.RegisterEvents(0)`: it uses Fabric's `ServerTickEvents`.START_WORLD_TICK (com/dtteam/dynamictrees/event/handler/CommonEventHandler.java:37)

### `CommonEventHandler.FabricReloadListener`: it extends `ReloadListener`

- [ ] `CommonEventHandler.FabricReloadListener.FabricReloadListener(0)`: its class isn't translated (it extends `ReloadListener`) (com/dtteam/dynamictrees/event/handler/CommonEventHandler.java:70)
- [ ] `CommonEventHandler.FabricReloadListener.method_25931(6)`: its class isn't translated (it extends `ReloadListener`) (com/dtteam/dynamictrees/event/handler/CommonEventHandler.java:74)
- [ ] `CommonEventHandler.FabricReloadListener.getFabricId(0)`: its class isn't translated (it extends `ReloadListener`) (com/dtteam/dynamictrees/event/handler/CommonEventHandler.java:86)

### `ModEventHandler`

- [ ] `ModEventHandler.RegisterEvents(0)`: it calls ModEventHandler.registerLeavesPropertiesTypes, which isn't translated (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:40)
- [ ] `ModEventHandler.registerLeavesPropertiesTypes(0)`: it uses LeavesProperties.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)) (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:64)
- [ ] `ModEventHandler.registerFamilyTypes(0)`: it uses Family.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)) (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:71)
- [ ] `ModEventHandler.registerSpeciesTypes(0)`: it uses Species.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)) (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:77)
- [ ] `ModEventHandler.registerSoilPropertiesTypes(0)`: it uses SoilProperties.REGISTRY, which isn't translated (it uses TypedRegistry, which isn't translated (it's generic)) (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:84)
- [ ] `ModEventHandler.registerCellKits(0)`: it calls CellKits.register, which isn't translated (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:90)
- [ ] `ModEventHandler.registerGrowthLogicKits(0)`: it calls GrowthLogicKits.register, which isn't translated (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:94)
- [ ] `ModEventHandler.registerGenFeatures(0)`: it calls GenFeatures.register, which isn't translated (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:98)
- [ ] `ModEventHandler.registerFeatureCancellers(0)`: it calls FeatureCancellers.register, which isn't translated (com/dtteam/dynamictrees/event/handler/ModEventHandler.java:102)

### `VanillaSaplingEventHandler`

- [x] `VanillaSaplingEventHandler.isEnabled` (com/dtteam/dynamictrees/event/handler/VanillaSaplingEventHandler.java:18)
- [ ] `VanillaSaplingEventHandler.register(0)`: it uses Fabric's `UseBlockCallback`.EVENT (com/dtteam/dynamictrees/event/handler/VanillaSaplingEventHandler.java:20)
- [ ] `VanillaSaplingEventHandler.updateEnabled(0)`: it uses DTConfigs.COMMON, which isn't translated (it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec))) (com/dtteam/dynamictrees/event/handler/VanillaSaplingEventHandler.java:24)
- [ ] `VanillaSaplingEventHandler.onUseBlock(4)`: it uses Minecraft's `class_1269` (com/dtteam/dynamictrees/event/handler/VanillaSaplingEventHandler.java:28)

### `DendroPotion`: it extends Minecraft's `class_1792`

- [ ] `DendroPotion.DendroPotion(0)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DendroPotion.java:20)
- [ ] `DendroPotion.applyIndexTag(2)`: it uses Minecraft's `class_1799` (com/dtteam/dynamictrees/item/DendroPotion.java:24)
- [ ] `DendroPotion.getPotionType(1)`: it uses DendroPotionType, which isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:29)
- [ ] `DendroPotion.getSubstanceEffect(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DendroPotion.java:34)
- [ ] `DendroPotion.method_7866(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DendroPotion.java:46)
- [ ] `DendroPotion.getColor(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DendroPotion.java:50)
- [ ] `DendroPotion.getEmptyContainer(0)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DendroPotion.java:54)

### `DendroPotion.DendroPotionType`: its constants carry values

- [ ] `DendroPotion.DendroPotionType.DendroPotionType(5)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:73)
- [ ] `DendroPotion.DendroPotionType.DendroPotionType(6)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:77)
- [ ] `DendroPotion.DendroPotionType.getIndex(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:86)
- [ ] `DendroPotion.DendroPotionType.isActive(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:90)
- [ ] `DendroPotion.DendroPotionType.getName(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:94)
- [ ] `DendroPotion.DendroPotionType.getColor(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:98)
- [ ] `DendroPotion.DendroPotionType.getIngredient(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:102)
- [ ] `DendroPotion.DendroPotionType.getDescription(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:106)
- [ ] `DendroPotion.DendroPotionType.getBasePotionType(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/item/DendroPotion.java:110)

### `DirtBucket`: it extends Minecraft's `class_1792`

- [ ] `DirtBucket.DirtBucket(0)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DirtBucket.java:27)
- [ ] `DirtBucket.method_7836(3)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DirtBucket.java:33)
- [ ] `DirtBucket.tryPlaceContainedDirt(3)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/DirtBucket.java:65)

### `Seed`: it extends Minecraft's `class_1792`

- [ ] `Seed.BACKUP_RANDOM`: it uses LazyValue, which isn't translated (it's an interface) (com/dtteam/dynamictrees/item/Seed.java:39)
- [x] `Seed.FORCE_PLANT_KEY` (com/dtteam/dynamictrees/item/Seed.java:40)
- [x] `Seed.LIFESPAN_KEY` (com/dtteam/dynamictrees/item/Seed.java:41)
- [x] `Seed.CODE_KEY` (com/dtteam/dynamictrees/item/Seed.java:42)
- [ ] `Seed.Seed(0)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:45)
- [ ] `Seed.Seed(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:50)
- [ ] `Seed.Seed(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:54)
- [ ] `Seed.getSpecies(0)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:60)
- [ ] `Seed.onEntityItemUpdate(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:65)
- [ ] `Seed.doPlanting(4)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:87)
- [ ] `Seed.shouldPlant(3)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:113)
- [ ] `Seed.hasForcePlant(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:137)
- [ ] `Seed.getTimeToLive(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:141)
- [ ] `Seed.getCode(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:145)
- [ ] `Seed.getJoCodeForRadius(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:149)
- [ ] `Seed.method_7884(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:154)
- [ ] `Seed.method_7836(3)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:161)
- [ ] `Seed.onItemUsePlantSeed(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:177)
- [ ] `Seed.onItemUseFirst(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:201)
- [ ] `Seed.onItemUseFlowerPot(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Seed.java:205)

### `Seed.VoluntaryPlantEventResult`

- [x] `Seed.VoluntaryPlantEventResult.VoluntaryPlantEventResult(2)` (com/dtteam/dynamictrees/item/Seed.java:233)
- [x] `Seed.VoluntaryPlantEventResult.cancelled(0)` (com/dtteam/dynamictrees/item/Seed.java:233)
- [x] `Seed.VoluntaryPlantEventResult.willPlant(0)` (com/dtteam/dynamictrees/item/Seed.java:233)

### `Staff`: it extends Minecraft's `class_1792`

- [x] `Staff.REACH_DISTANCE` (com/dtteam/dynamictrees/item/Staff.java:46)
- [ ] `Staff.Staff(0)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:48)
- [ ] `Staff.method_58404(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:64)
- [ ] `Staff.method_7879(5)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:68)
- [ ] `Staff.method_7836(3)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:80)
- [ ] `Staff.method_7884(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:93)
- [ ] `Staff.method_31567(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:148)
- [ ] `Staff.method_31569(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:152)
- [ ] `Staff.isReadOnly(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:157)
- [ ] `Staff.setReadOnly(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:161)
- [ ] `Staff.setSpecies(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:166)
- [ ] `Staff.setCode(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:171)
- [ ] `Staff.setRootsCode(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:175)
- [ ] `Staff.getSpecies(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:179)
- [ ] `Staff.getDamage(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:189)
- [ ] `Staff.setDamage(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:193)
- [ ] `Staff.getMaxDamage(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:197)
- [ ] `Staff.setMaxDamage(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:201)
- [ ] `Staff.hasMaxDamage(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:205)
- [ ] `Staff.damage(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:209)
- [ ] `Staff.getColor(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:216)
- [ ] `Staff.setColor(2)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:239)
- [ ] `Staff.getCode(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:244)
- [ ] `Staff.getRootsCode(1)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:258)
- [ ] `Staff.method_7851(4)`: its class isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/item/Staff.java:269)

### `SeasonalSeedDropChance`: it implements Minecraft's `class_5341`

- [ ] `SeasonalSeedDropChance.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/condition/SeasonalSeedDropChance.java:16)
- [ ] `SeasonalSeedDropChance.SeasonalSeedDropChance(1)`: its class isn't translated (it implements Minecraft's `class_5341`) (com/dtteam/dynamictrees/loot/condition/SeasonalSeedDropChance.java:20)
- [ ] `SeasonalSeedDropChance.method_29325(0)`: its class isn't translated (it implements Minecraft's `class_5341`) (com/dtteam/dynamictrees/loot/condition/SeasonalSeedDropChance.java:23)
- [ ] `SeasonalSeedDropChance.test(1)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/loot/condition/SeasonalSeedDropChance.java:27)
- [ ] `SeasonalSeedDropChance.seasonalSeedDropChance(0)`: it uses Minecraft's `class_210` (com/dtteam/dynamictrees/loot/condition/SeasonalSeedDropChance.java:35)

### `SpeciesMatches`: it implements Minecraft's `class_5341`

- [ ] `SpeciesMatches.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/condition/SpeciesMatches.java:15)
- [ ] `SpeciesMatches.SpeciesMatches(1)`: its class isn't translated (it implements Minecraft's `class_5341`) (com/dtteam/dynamictrees/loot/condition/SpeciesMatches.java:20)
- [ ] `SpeciesMatches.method_29325(0)`: its class isn't translated (it implements Minecraft's `class_5341`) (com/dtteam/dynamictrees/loot/condition/SpeciesMatches.java:24)
- [ ] `SpeciesMatches.test(1)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/loot/condition/SpeciesMatches.java:28)
- [ ] `SpeciesMatches.speciesMatches(1)`: it uses Minecraft's `class_210` (com/dtteam/dynamictrees/loot/condition/SpeciesMatches.java:34)

### `VoluntarySeedDropChance`: it implements Minecraft's `class_5341`

- [ ] `VoluntarySeedDropChance.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/condition/VoluntarySeedDropChance.java:15)
- [ ] `VoluntarySeedDropChance.VoluntarySeedDropChance(1)`: its class isn't translated (it implements Minecraft's `class_5341`) (com/dtteam/dynamictrees/loot/condition/VoluntarySeedDropChance.java:20)
- [ ] `VoluntarySeedDropChance.method_29325(0)`: its class isn't translated (it implements Minecraft's `class_5341`) (com/dtteam/dynamictrees/loot/condition/VoluntarySeedDropChance.java:24)
- [ ] `VoluntarySeedDropChance.test(1)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/loot/condition/VoluntarySeedDropChance.java:28)
- [ ] `VoluntarySeedDropChance.voluntarySeedDropChance(0)`: it uses Minecraft's `class_210` (com/dtteam/dynamictrees/loot/condition/VoluntarySeedDropChance.java:36)
- [ ] `VoluntarySeedDropChance.voluntarySeedDropChance(1)`: it uses Minecraft's `class_210` (com/dtteam/dynamictrees/loot/condition/VoluntarySeedDropChance.java:40)

### `DTLoot`

- [x] `DTLoot.load(0)` (com/dtteam/dynamictrees/loot/DTLoot.java:4)

### `DTLootContextParams`

- [ ] `DTLootContextParams.SPECIES`: it uses Minecraft's `class_169` (com/dtteam/dynamictrees/loot/DTLootContextParams.java:8)
- [ ] `DTLootContextParams.FERTILITY`: it uses Minecraft's `class_169` (com/dtteam/dynamictrees/loot/DTLootContextParams.java:9)
- [ ] `DTLootContextParams.SEASONAL_SEED_DROP_FACTOR`: it uses Minecraft's `class_169` (com/dtteam/dynamictrees/loot/DTLootContextParams.java:10)
- [ ] `DTLootContextParams.VOLUME`: it uses Minecraft's `class_169` (com/dtteam/dynamictrees/loot/DTLootContextParams.java:11)
- [ ] `DTLootContextParams.create(1)`: it's generic (com/dtteam/dynamictrees/loot/DTLootContextParams.java:13)
- [x] `DTLootContextParams.load(0)` (com/dtteam/dynamictrees/loot/DTLootContextParams.java:17)

### `DTLootParameterSets`

- [ ] `DTLootParameterSets.LEAVES`: it uses Minecraft's `class_176` (com/dtteam/dynamictrees/loot/DTLootParameterSets.java:11)
- [ ] `DTLootParameterSets.LEAVES_BLOCK`: it uses Minecraft's `class_176` (com/dtteam/dynamictrees/loot/DTLootParameterSets.java:19)
- [ ] `DTLootParameterSets.VOLUNTARY`: it uses Minecraft's `class_176` (com/dtteam/dynamictrees/loot/DTLootParameterSets.java:30)
- [ ] `DTLootParameterSets.BRANCHES`: it uses Minecraft's `class_176` (com/dtteam/dynamictrees/loot/DTLootParameterSets.java:34)
- [ ] `DTLootParameterSets.register(2)`: it uses Minecraft's `class_176` (com/dtteam/dynamictrees/loot/DTLootParameterSets.java:42)
- [x] `DTLootParameterSets.load(0)` (com/dtteam/dynamictrees/loot/DTLootParameterSets.java:50)

### `ItemBySpeciesLootPoolEntry`: it extends Minecraft's `class_85`

- [ ] `ItemBySpeciesLootPoolEntry.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/entry/ItemBySpeciesLootPoolEntry.java:25)
- [ ] `ItemBySpeciesLootPoolEntry.ItemBySpeciesLootPoolEntry(5)`: its class isn't translated (it extends Minecraft's `class_85`) (com/dtteam/dynamictrees/loot/entry/ItemBySpeciesLootPoolEntry.java:34)
- [ ] `ItemBySpeciesLootPoolEntry.method_29318(0)`: its class isn't translated (it extends Minecraft's `class_85`) (com/dtteam/dynamictrees/loot/entry/ItemBySpeciesLootPoolEntry.java:41)
- [ ] `ItemBySpeciesLootPoolEntry.method_433(2)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/loot/entry/ItemBySpeciesLootPoolEntry.java:45)

### `SeedItemLootPoolEntry`: it extends Minecraft's `class_85`

- [ ] `SeedItemLootPoolEntry.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/entry/SeedItemLootPoolEntry.java:19)
- [ ] `SeedItemLootPoolEntry.SeedItemLootPoolEntry(4)`: its class isn't translated (it extends Minecraft's `class_85`) (com/dtteam/dynamictrees/loot/entry/SeedItemLootPoolEntry.java:23)
- [ ] `SeedItemLootPoolEntry.method_29318(0)`: its class isn't translated (it extends Minecraft's `class_85`) (com/dtteam/dynamictrees/loot/entry/SeedItemLootPoolEntry.java:27)
- [ ] `SeedItemLootPoolEntry.method_433(2)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/loot/entry/SeedItemLootPoolEntry.java:31)
- [ ] `SeedItemLootPoolEntry.lootTableSeedItem(0)`: it uses Minecraft's `class_86` (com/dtteam/dynamictrees/loot/entry/SeedItemLootPoolEntry.java:37)

### `WeightedItemLootPoolEntry`: it extends Minecraft's `class_85`

- [ ] `WeightedItemLootPoolEntry.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/entry/WeightedItemLootPoolEntry.java:23)
- [ ] `WeightedItemLootPoolEntry.WeightedItemLootPoolEntry(5)`: its class isn't translated (it extends Minecraft's `class_85`) (com/dtteam/dynamictrees/loot/entry/WeightedItemLootPoolEntry.java:32)
- [ ] `WeightedItemLootPoolEntry.method_29318(0)`: its class isn't translated (it extends Minecraft's `class_85`) (com/dtteam/dynamictrees/loot/entry/WeightedItemLootPoolEntry.java:39)
- [ ] `WeightedItemLootPoolEntry.method_433(2)`: its class isn't translated (it extends Minecraft's `class_85`) (com/dtteam/dynamictrees/loot/entry/WeightedItemLootPoolEntry.java:43)

### `MultiplyByLogsCount`: it extends Minecraft's `class_120`

- [ ] `MultiplyByLogsCount.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/function/MultiplyByLogsCount.java:16)
- [ ] `MultiplyByLogsCount.MultiplyByLogsCount(1)`: its class isn't translated (it extends Minecraft's `class_120`) (com/dtteam/dynamictrees/loot/function/MultiplyByLogsCount.java:20)
- [ ] `MultiplyByLogsCount.method_29321(0)`: its class isn't translated (it extends Minecraft's `class_120`) (com/dtteam/dynamictrees/loot/function/MultiplyByLogsCount.java:24)
- [ ] `MultiplyByLogsCount.method_522(2)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/loot/function/MultiplyByLogsCount.java:28)
- [ ] `MultiplyByLogsCount.multiplyByLogsCount(0)`: it uses Minecraft's `class_118` (com/dtteam/dynamictrees/loot/function/MultiplyByLogsCount.java:35)

### `MultiplyBySticksCount`: it extends Minecraft's `class_120`

- [ ] `MultiplyBySticksCount.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/function/MultiplyBySticksCount.java:16)
- [ ] `MultiplyBySticksCount.MultiplyBySticksCount(1)`: its class isn't translated (it extends Minecraft's `class_120`) (com/dtteam/dynamictrees/loot/function/MultiplyBySticksCount.java:20)
- [ ] `MultiplyBySticksCount.method_29321(0)`: its class isn't translated (it extends Minecraft's `class_120`) (com/dtteam/dynamictrees/loot/function/MultiplyBySticksCount.java:24)
- [ ] `MultiplyBySticksCount.method_522(2)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/loot/function/MultiplyBySticksCount.java:28)
- [ ] `MultiplyBySticksCount.multiplyBySticksCount(0)`: it uses Minecraft's `class_118` (com/dtteam/dynamictrees/loot/function/MultiplyBySticksCount.java:35)

### `MultiplyCount`: it extends Minecraft's `class_120`

- [ ] `MultiplyCount.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/loot/function/MultiplyCount.java:16)
- [ ] `MultiplyCount.MultiplyCount(2)`: its class isn't translated (it extends Minecraft's `class_120`) (com/dtteam/dynamictrees/loot/function/MultiplyCount.java:21)
- [ ] `MultiplyCount.method_29321(0)`: its class isn't translated (it extends Minecraft's `class_120`) (com/dtteam/dynamictrees/loot/function/MultiplyCount.java:26)
- [ ] `MultiplyCount.method_522(2)`: its class isn't translated (it extends Minecraft's `class_120`) (com/dtteam/dynamictrees/loot/function/MultiplyCount.java:30)
- [ ] `MultiplyCount.multiplyCount(0)`: it uses Minecraft's `class_118` (com/dtteam/dynamictrees/loot/function/MultiplyCount.java:35)

### `LootTableSupplier`: it uses Minecraft's `class_2960`

- [ ] `LootTableSupplier.LootTableSupplier(2)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/loot/LootTableSupplier.java:16)
- [ ] `LootTableSupplier.get(2)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/loot/LootTableSupplier.java:22)
- [ ] `LootTableSupplier.getName(1)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/loot/LootTableSupplier.java:29)
- [ ] `LootTableSupplier.getName(0)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/loot/LootTableSupplier.java:34)
- [ ] `MixinBrewingStandBlockEntity`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/dtteam/dynamictrees/loot/LootTableSupplier.java:12)
- [ ] `MixinBrewingStandMenu`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/dtteam/dynamictrees/loot/LootTableSupplier.java:12)
- [ ] `MixinBushBlock`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/dtteam/dynamictrees/loot/LootTableSupplier.java:17)
- [ ] `MixinChunkSerializer`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/dtteam/dynamictrees/loot/LootTableSupplier.java:22)
- [ ] `MixinMinecraftServer`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/dtteam/dynamictrees/loot/LootTableSupplier.java:11)
- [ ] `MixinPotionBrewing`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/dtteam/dynamictrees/loot/LootTableSupplier.java:14)
- [ ] `MixinSaplingBlock`: a Mixin class: its handlers are the injections, in the bundle's helpers (com/dtteam/dynamictrees/loot/LootTableSupplier.java:18)

### `BakedModelBlockPottedSapling`: it extends Fabric's `ForwardingBakedModel`

- [ ] `BakedModelBlockPottedSapling.BakedModelBlockPottedSapling(1)`: its class isn't translated (it extends Fabric's `ForwardingBakedModel`) (com/dtteam/dynamictrees/model/BakedModelBlockPottedSapling.java:14)
- [ ] `BakedModelBlockPottedSapling.isVanillaAdapter(0)`: its class isn't translated (it extends Fabric's `ForwardingBakedModel`) (com/dtteam/dynamictrees/model/BakedModelBlockPottedSapling.java:18)
- [ ] `BakedModelBlockPottedSapling.method_4707(3)`: its class isn't translated (it extends Fabric's `ForwardingBakedModel`) (com/dtteam/dynamictrees/model/BakedModelBlockPottedSapling.java:22)

### `BasicBranchBlockBakedModel`: it implements Minecraft's `class_1087`

- [ ] `BasicBranchBlockBakedModel.BasicBranchBlockBakedModel(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:47)
- [ ] `BasicBranchBlockBakedModel.initModels(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:53)
- [ ] `BasicBranchBlockBakedModel.generateSleevePart(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:69)
- [ ] `BasicBranchBlockBakedModel.bakeSleeve(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:111)
- [ ] `BasicBranchBlockBakedModel.generateCorePart(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:124)
- [ ] `BasicBranchBlockBakedModel.bakeCore(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:137)
- [ ] `BasicBranchBlockBakedModel.getFaceAngle(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:150)
- [ ] `BasicBranchBlockBakedModel.isVanillaAdapter(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:167)
- [ ] `BasicBranchBlockBakedModel.emitBlockQuads(5)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:171)
- [ ] `BasicBranchBlockBakedModel.collectQuads(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:182)
- [ ] `BasicBranchBlockBakedModel.maxBranchRadius(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:203)
- [ ] `BasicBranchBlockBakedModel.collectQuads(4)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:207)
- [ ] `BasicBranchBlockBakedModel.getRingQuads(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:243)
- [ ] `BasicBranchBlockBakedModel.getRenderMaterial(0)`: it uses Fabric's `RenderMaterial` (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:247)
- [ ] `BasicBranchBlockBakedModel.emitQuads(4)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:257)
- [ ] `BasicBranchBlockBakedModel.emitItemQuads(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:268)
- [ ] `BasicBranchBlockBakedModel.getSourceDir(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:271)
- [ ] `BasicBranchBlockBakedModel.resolveCoreDir(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:291)
- [ ] `BasicBranchBlockBakedModel.getRadius(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:295)
- [ ] `BasicBranchBlockBakedModel.method_4707(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:299)
- [ ] `BasicBranchBlockBakedModel.method_4708(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:303)
- [ ] `BasicBranchBlockBakedModel.method_4712(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:307)
- [ ] `BasicBranchBlockBakedModel.method_24304(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:311)
- [ ] `BasicBranchBlockBakedModel.method_4713(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:315)
- [ ] `BasicBranchBlockBakedModel.method_4711(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:319)
- [ ] `BasicBranchBlockBakedModel.method_4709(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:323)
- [ ] `BasicBranchBlockBakedModel.method_4710(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/BasicBranchBlockBakedModel.java:327)

### `BasicRootsBlockBakedModel`: it extends `BasicBranchBlockBakedModel`

- [x] `BasicRootsBlockBakedModel.Z_FIGHTING_OFFSET` (com/dtteam/dynamictrees/model/baked/BasicRootsBlockBakedModel.java:22)
- [ ] `BasicRootsBlockBakedModel.BasicRootsBlockBakedModel(2)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/BasicRootsBlockBakedModel.java:25)
- [ ] `BasicRootsBlockBakedModel.initRootsModels(0)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/BasicRootsBlockBakedModel.java:30)
- [ ] `BasicRootsBlockBakedModel.bakeSleeveFace(3)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/BasicRootsBlockBakedModel.java:40)
- [ ] `BasicRootsBlockBakedModel.getRadius(1)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/BasicRootsBlockBakedModel.java:67)
- [ ] `BasicRootsBlockBakedModel.collectQuads(4)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/BasicRootsBlockBakedModel.java:74)

### `SurfaceRootBlockBakedModel`: it implements Minecraft's `class_1087`

- [ ] `SurfaceRootBlockBakedModel.SurfaceRootBlockBakedModel(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:50)
- [ ] `SurfaceRootBlockBakedModel.initModels(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:55)
- [ ] `SurfaceRootBlockBakedModel.getRadialHeight(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:71)
- [ ] `SurfaceRootBlockBakedModel.bakeSleeve(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:75)
- [ ] `SurfaceRootBlockBakedModel.bakeVert(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:130)
- [ ] `SurfaceRootBlockBakedModel.bakeCore(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:157)
- [ ] `SurfaceRootBlockBakedModel.getFaceAngle(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:189)
- [ ] `SurfaceRootBlockBakedModel.getUVs(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:206)
- [ ] `SurfaceRootBlockBakedModel.modUV(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:217)
- [ ] `SurfaceRootBlockBakedModel.AABBLimits(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:225)
- [ ] `SurfaceRootBlockBakedModel.isVanillaAdapter(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:232)
- [ ] `SurfaceRootBlockBakedModel.emitBlockQuads(5)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:236)
- [ ] `SurfaceRootBlockBakedModel.collectQuads(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:247)
- [ ] `SurfaceRootBlockBakedModel.emitQuads(4)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:305)
- [ ] `SurfaceRootBlockBakedModel.getRenderMaterial(0)`: it uses Fabric's `RenderMaterial` (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:316)
- [ ] `SurfaceRootBlockBakedModel.emitItemQuads(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:326)
- [ ] `SurfaceRootBlockBakedModel.getSourceDir(2)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:329)
- [ ] `SurfaceRootBlockBakedModel.resolveCoreDir(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:349)
- [ ] `SurfaceRootBlockBakedModel.getRadius(1)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:353)
- [ ] `SurfaceRootBlockBakedModel.method_4707(3)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:357)
- [ ] `SurfaceRootBlockBakedModel.method_4708(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:361)
- [ ] `SurfaceRootBlockBakedModel.method_4712(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:365)
- [ ] `SurfaceRootBlockBakedModel.method_24304(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:369)
- [ ] `SurfaceRootBlockBakedModel.method_4713(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:373)
- [ ] `SurfaceRootBlockBakedModel.method_4711(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:377)
- [ ] `SurfaceRootBlockBakedModel.method_4709(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:381)
- [ ] `SurfaceRootBlockBakedModel.method_4710(0)`: its class isn't translated (it implements Minecraft's `class_1087`) (com/dtteam/dynamictrees/model/baked/SurfaceRootBlockBakedModel.java:385)

### `ThickBranchBlockBakedModel`: it extends `BasicBranchBlockBakedModel`

- [ ] `ThickBranchBlockBakedModel.ThickBranchBlockBakedModel(3)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:30)
- [ ] `ThickBranchBlockBakedModel.initThickModels(0)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:36)
- [ ] `ThickBranchBlockBakedModel.bakeTrunkBark(3)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:46)
- [ ] `ThickBranchBlockBakedModel.bakeTrunkRings(3)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:87)
- [ ] `ThickBranchBlockBakedModel.getRingsUvs(3)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:120)
- [ ] `ThickBranchBlockBakedModel.getUVs(2)`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:135)
- [x] `ThickBranchBlockBakedModel.modUV(1)` (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:146)
- [ ] `ThickBranchBlockBakedModel.aabbLimits(1)`: it uses Minecraft's `class_238` (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:154)
- [ ] `ThickBranchBlockBakedModel.maxBranchRadius(0)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:161)
- [ ] `ThickBranchBlockBakedModel.collectQuads(4)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:166)
- [ ] `ThickBranchBlockBakedModel.getRingQuads(1)`: its class isn't translated (it extends `BasicBranchBlockBakedModel`) (com/dtteam/dynamictrees/model/baked/ThickBranchBlockBakedModel.java:195)

### `BranchBlockUnbakedModel`: it implements Minecraft's `class_1100`

- [ ] `BranchBlockUnbakedModel.BranchBlockUnbakedModel(4)`: its class isn't translated (it implements Minecraft's `class_1100`) (com/dtteam/dynamictrees/model/BranchBlockUnbakedModel.java:25)
- [ ] `BranchBlockUnbakedModel.method_4755(0)`: its class isn't translated (it implements Minecraft's `class_1100`) (com/dtteam/dynamictrees/model/BranchBlockUnbakedModel.java:32)
- [ ] `BranchBlockUnbakedModel.method_45785(1)`: its class isn't translated (it implements Minecraft's `class_1100`) (com/dtteam/dynamictrees/model/BranchBlockUnbakedModel.java:36)
- [ ] `BranchBlockUnbakedModel.method_4753(3)`: its class isn't translated (it implements Minecraft's `class_1100`) (com/dtteam/dynamictrees/model/BranchBlockUnbakedModel.java:39)

### `DTModelLoadingPlugin`: it implements Fabric's `ModelLoadingPlugin`

- [ ] `DTModelLoadingPlugin.POTTED_SAPLING_MODEL`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:30)
- [ ] `DTModelLoadingPlugin.BRANCH_MODEL_CACHE`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:31)
- [ ] `DTModelLoadingPlugin.ROOT_MODEL_CACHE`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:32)
- [ ] `DTModelLoadingPlugin.UNDERGROUND_ROOTS_MODEL_CACHE`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:33)
- [x] `DTModelLoadingPlugin.modelsInitialized` (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:34)
- [ ] `DTModelLoadingPlugin.onInitializeModelLoader(1)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:36)
- [ ] `DTModelLoadingPlugin.initBranchModels(1)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:44)
- [ ] `DTModelLoadingPlugin.createBranchModel(4)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:134)
- [ ] `DTModelLoadingPlugin.createRootModel(2)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:146)
- [ ] `DTModelLoadingPlugin.createRootsBlockModel(3)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:151)
- [ ] `DTModelLoadingPlugin.createFallbackRootsModel(3)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:157)
- [ ] `DTModelLoadingPlugin.modifyModelAfterBake(2)`: its class isn't translated (it implements Fabric's `ModelLoadingPlugin`) (com/dtteam/dynamictrees/model/DTModelLoadingPlugin.java:175)

### `FallingTreeEntityModel`: it extends Minecraft's `class_583`

- [ ] `FallingTreeEntityModel.FallingTreeEntityModel(1)`: its class isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:20)
- [ ] `FallingTreeEntityModel.getQuads(0)`: its class isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:28)
- [ ] `FallingTreeEntityModel.getEntityId(0)`: its class isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:32)
- [ ] `FallingTreeEntityModel.getBrightness(1)`: it uses FallingTreeEntity, which isn't translated (it extends Minecraft's `class_1297`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:36)
- [ ] `FallingTreeEntityModel.generateTreeQuads(1)`: its class isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:42)
- [ ] `FallingTreeEntityModel.setupAnim(6)`: its class isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:46)
- [ ] `FallingTreeEntityModel.method_2828(5)`: its class isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:49)
- [ ] `FallingTreeEntityModel.toTreeQuadData(2)`: it uses TreeQuadData, which isn't translated (it uses Minecraft's `class_777`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:73)
- [ ] `FallingTreeEntityModel.toTreeQuadData(3)`: it uses TreeQuadData, which isn't translated (it uses Minecraft's `class_777`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:77)

### `FallingTreeEntityModel.TreeQuadData`: it uses Minecraft's `class_777`

- [ ] `FallingTreeEntityModel.TreeQuadData.TreeQuadData(3)`: its class isn't translated (it uses Minecraft's `class_777`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:81)
- [ ] `FallingTreeEntityModel.TreeQuadData.bakedQuad(0)`: its class isn't translated (it uses Minecraft's `class_777`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:81)
- [ ] `FallingTreeEntityModel.TreeQuadData.color(0)`: its class isn't translated (it uses Minecraft's `class_777`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:81)
- [ ] `FallingTreeEntityModel.TreeQuadData.state(0)`: its class isn't translated (it uses Minecraft's `class_777`) (com/dtteam/dynamictrees/model/FallingTreeEntityModel.java:81)

### `FallingTreeEntityModelFabric`: it extends `FallingTreeEntityModel`

- [ ] `FallingTreeEntityModelFabric.FallingTreeEntityModelFabric(1)`: its class isn't translated (it extends `FallingTreeEntityModel`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java:29)
- [ ] `FallingTreeEntityModelFabric.generateTreeQuads(1)`: its class isn't translated (it extends `FallingTreeEntityModel`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java:33)
- [ ] `FallingTreeEntityModelFabric.getBottomRingQuads(4)`: its class isn't translated (it extends `FallingTreeEntityModel`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java:136)
- [ ] `FallingTreeEntityModelFabric.getBranchQuadsWithConnections(7)`: its class isn't translated (it extends `FallingTreeEntityModel`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java:149)
- [ ] `FallingTreeEntityModelFabric.getQuadsWithOffset(4)`: its class isn't translated (it extends `FallingTreeEntityModel`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java:168)
- [ ] `FallingTreeEntityModelFabric.offsetAllQuads(2)`: its class isn't translated (it extends `FallingTreeEntityModel`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java:179)
- [ ] `FallingTreeEntityModelFabric.offsetQuad(2)`: its class isn't translated (it extends `FallingTreeEntityModel`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelFabric.java:193)

### `FallingTreeEntityModelTrackerCache`

- [ ] `FallingTreeEntityModelTrackerCache.models`: it uses FallingTreeEntityModel, which isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelTrackerCache.java:13)
- [ ] `FallingTreeEntityModelTrackerCache.getOrCreateModel(1)`: it uses FallingTreeEntityModel, which isn't translated (it extends Minecraft's `class_583`) (com/dtteam/dynamictrees/model/FallingTreeEntityModelTrackerCache.java:15)
- [ ] `FallingTreeEntityModelTrackerCache.cleanupModels(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/model/FallingTreeEntityModelTrackerCache.java:22)
- [ ] `FallingTreeEntityModelTrackerCache.cleanupModels(1)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/model/FallingTreeEntityModelTrackerCache.java:29)

### `ModelTracker`: it's an interface

- [ ] `ModelTracker.modelCleanup(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/model/ModelTracker.java:4)

### `ClientServices`

- [ ] `ClientServices.CLIENT`: it uses IClientHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/ClientServices.java:8)
- [ ] `ClientServices.load(1)`: it's generic (com/dtteam/dynamictrees/platform/ClientServices.java:10)

### `FabricClientHelper`: it implements `IClientHelper` (com.dtteam.dynamictrees.platform.services)

- [ ] `FabricClientHelper.getPixelRGBA(3)`: its class isn't translated (it implements `IClientHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricClientHelper.java:13)
- [ ] `FabricClientHelper.newFallingTreeEntityModel(1)`: its class isn't translated (it implements `IClientHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricClientHelper.java:25)

### `FabricCompatHelper`: it implements `ICompatHelper` (com.dtteam.dynamictrees.platform.services)

- [ ] `FabricCompatHelper.registerSereneSeasonsSeasonProvider(0)`: its class isn't translated (it implements `ICompatHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricCompatHelper.java:7)

### `FabricConfigHelper`: it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)

- [ ] `FabricConfigHelper.getConfig(2)`: it's generic (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:8)
- [ ] `FabricConfigHelper.getBoolConfig(1)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:12)
- [ ] `FabricConfigHelper.getIntConfig(1)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:17)
- [ ] `FabricConfigHelper.getDoubleConfig(1)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:22)
- [ ] `FabricConfigHelper.getStringConfig(1)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:27)
- [ ] `FabricConfigHelper.getEnumConfig(2)`: it's generic (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:32)
- [ ] `FabricConfigHelper.getStringListConfig(1)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:37)
- [ ] `FabricConfigHelper.isServerConfigLoaded(0)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:42)
- [ ] `FabricConfigHelper.isCommonConfigLoaded(0)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:47)
- [ ] `FabricConfigHelper.isClientConfigLoaded(0)`: its class isn't translated (it implements `IConfigHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricConfigHelper.java:52)

### `FabricEventHelper`: it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)

- [ ] `FabricEventHelper.postRegistryEvent(1)`: it's generic (com/dtteam/dynamictrees/platform/FabricEventHelper.java:25)
- [ ] `FabricEventHelper.postTypedRegistryEvent(1)`: it's generic (com/dtteam/dynamictrees/platform/FabricEventHelper.java:29)
- [ ] `FabricEventHelper.postAddResourceLoadersEventPre(1)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:33)
- [ ] `FabricEventHelper.postAddResourceLoadersEventPost(1)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:40)
- [ ] `FabricEventHelper.postJsonDeserializerRegistryEvent(0)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:44)
- [ ] `FabricEventHelper.postApplierEvent(3)`: it's generic (com/dtteam/dynamictrees/platform/FabricEventHelper.java:48)
- [ ] `FabricEventHelper.postBiomeEntryApplierEvent(2)`: it's generic (com/dtteam/dynamictrees/platform/FabricEventHelper.java:55)
- [ ] `FabricEventHelper.postCancellationApplierEvent(2)`: it's generic (com/dtteam/dynamictrees/platform/FabricEventHelper.java:59)
- [ ] `FabricEventHelper.postSpeciesPostGenerationEvent(1)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:63)
- [ ] `FabricEventHelper.postTransitionSaplingToTreeEvent(3)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:67)
- [ ] `FabricEventHelper.canCropGrow(4)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:72)
- [ ] `FabricEventHelper.cropGrowPost(3)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:77)
- [ ] `FabricEventHelper.postBiomeSuitabilityEvent(4)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:81)
- [ ] `FabricEventHelper.postSeedVoluntaryPlantEvent(4)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:86)
- [ ] `FabricEventHelper.postPoissonDiscProviderCreateEvent(2)`: its class isn't translated (it implements `IEventHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricEventHelper.java:91)

### `FabricInteractionHelper`: it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)

- [ ] `FabricInteractionHelper.canToolAxeStrip(1)`: its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricInteractionHelper.java:20)
- [ ] `FabricInteractionHelper.canToolAxeDig(1)`: its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricInteractionHelper.java:25)
- [ ] `FabricInteractionHelper.setSeedItemEntityLifespan(2)`: its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricInteractionHelper.java:30)
- [ ] `FabricInteractionHelper.blockDestroyByPlayer(6)`: its class isn't translated (it implements `IInteractionHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricInteractionHelper.java:35)

### `FabricMiscHelper`: it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services)

- [ ] `FabricMiscHelper.currentServer`: it uses Minecraft's `MinecraftServer` (com/dtteam/dynamictrees/platform/FabricMiscHelper.java:12)
- [ ] `FabricMiscHelper.debugSpeciesRegistry(0)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/platform/FabricMiscHelper.java:14)
- [ ] `FabricMiscHelper.isLevelRestoringBlockSnapshots(1)`: its class isn't translated (it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricMiscHelper.java:24)
- [ ] `FabricMiscHelper.getCurrentServer(0)`: its class isn't translated (it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricMiscHelper.java:29)
- [ ] `FabricMiscHelper.newDTBiomeHolderSet(0)`: its class isn't translated (it implements `IMiscHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricMiscHelper.java:34)

### `FabricPlatformHelper`: it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)

- [ ] `FabricPlatformHelper.getPlatformName(0)`: its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricPlatformHelper.java:11)
- [ ] `FabricPlatformHelper.isModLoaded(1)`: its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricPlatformHelper.java:16)
- [ ] `FabricPlatformHelper.isDevelopmentEnvironment(0)`: its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricPlatformHelper.java:21)
- [ ] `FabricPlatformHelper.getMods(0)`: its class isn't translated (it implements `IPlatformHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricPlatformHelper.java:26)

### `FabricRegistryHelper`: it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)

- [x] `FabricRegistryHelper.registriesInstance` (com/dtteam/dynamictrees/platform/FabricRegistryHelper.java:14)
- [ ] `FabricRegistryHelper.getRegistryLoader(0)`: its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricRegistryHelper.java:16)
- [ ] `FabricRegistryHelper.newRegistryHandler(0)`: its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricRegistryHelper.java:25)
- [ ] `FabricRegistryHelper.getPottedSaplingBlockEntity(0)`: its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricRegistryHelper.java:30)
- [ ] `FabricRegistryHelper.newPottedSaplingBlockEntity(2)`: its class isn't translated (it implements `IRegistryHelper` (com.dtteam.dynamictrees.platform.services)) (com/dtteam/dynamictrees/platform/FabricRegistryHelper.java:35)

### `Services`

- [ ] `Services.PLATFORM`: it uses IPlatformHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/Services.java:14)
- [ ] `Services.REGISTRY`: it uses IRegistryHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/Services.java:15)
- [ ] `Services.CONFIG`: it uses IConfigHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/Services.java:16)
- [ ] `Services.INTERACTION`: it uses IInteractionHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/Services.java:17)
- [ ] `Services.MISC`: it uses IMiscHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/Services.java:18)
- [ ] `Services.COMPAT`: it uses ICompatHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/Services.java:19)
- [ ] `Services.EVENT`: it uses IEventHelper, which isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/Services.java:20)
- [ ] `Services.load(1)`: it's generic (com/dtteam/dynamictrees/platform/Services.java:22)

### `IClientHelper`: it's an interface

- [ ] `IClientHelper.getPixelRGBA(3)`: it has no body (com/dtteam/dynamictrees/platform/services/IClientHelper.java:8)
- [ ] `IClientHelper.newFallingTreeEntityModel(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IClientHelper.java:10)

### `ICompatHelper`: it's an interface

- [ ] `ICompatHelper.registerSereneSeasonsSeasonProvider(0)`: it has no body (com/dtteam/dynamictrees/platform/services/ICompatHelper.java:4)

### `IConfigHelper`: it's an interface

- [ ] `IConfigHelper.getBoolConfig(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:58)
- [ ] `IConfigHelper.getIntConfig(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:60)
- [ ] `IConfigHelper.getDoubleConfig(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:62)
- [ ] `IConfigHelper.getStringConfig(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:64)
- [ ] `IConfigHelper.getEnumConfig(2)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:66)
- [ ] `IConfigHelper.getStringListConfig(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:68)
- [ ] `IConfigHelper.isServerConfigLoaded(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:70)
- [ ] `IConfigHelper.isCommonConfigLoaded(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:72)
- [ ] `IConfigHelper.isClientConfigLoaded(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IConfigHelper.java:74)

### `IEventHelper`: it's an interface

- [ ] `IEventHelper.postRegistryEvent(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:22)
- [ ] `IEventHelper.postTypedRegistryEvent(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:24)
- [ ] `IEventHelper.postAddResourceLoadersEventPre(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:26)
- [ ] `IEventHelper.postAddResourceLoadersEventPost(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:28)
- [ ] `IEventHelper.postJsonDeserializerRegistryEvent(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:30)
- [ ] `IEventHelper.postApplierEvent(3)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:32)
- [ ] `IEventHelper.postBiomeEntryApplierEvent(2)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:34)
- [ ] `IEventHelper.postCancellationApplierEvent(2)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:36)
- [ ] `IEventHelper.postSpeciesPostGenerationEvent(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:38)
- [ ] `IEventHelper.postTransitionSaplingToTreeEvent(3)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:40)
- [ ] `IEventHelper.canCropGrow(4)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:42)
- [ ] `IEventHelper.cropGrowPost(3)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:44)
- [ ] `IEventHelper.postBiomeSuitabilityEvent(4)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:46)
- [ ] `IEventHelper.postSeedVoluntaryPlantEvent(4)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:48)
- [ ] `IEventHelper.postPoissonDiscProviderCreateEvent(2)`: it has no body (com/dtteam/dynamictrees/platform/services/IEventHelper.java:50)

### `IInteractionHelper`: it's an interface

- [ ] `IInteractionHelper.canToolAxeStrip(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IInteractionHelper.java:13)
- [ ] `IInteractionHelper.canToolAxeDig(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IInteractionHelper.java:15)
- [ ] `IInteractionHelper.setSeedItemEntityLifespan(2)`: it has no body (com/dtteam/dynamictrees/platform/services/IInteractionHelper.java:17)
- [ ] `IInteractionHelper.blockDestroyByPlayer(6)`: it has no body (com/dtteam/dynamictrees/platform/services/IInteractionHelper.java:19)

### `IMiscHelper`: it's an interface

- [ ] `IMiscHelper.isLevelRestoringBlockSnapshots(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IMiscHelper.java:8)
- [ ] `IMiscHelper.getCurrentServer(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IMiscHelper.java:10)
- [ ] `IMiscHelper.newDTBiomeHolderSet(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IMiscHelper.java:12)

### `IPlatformHelper`: it's an interface

- [ ] `IPlatformHelper.getPlatformName(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IPlatformHelper.java:7)
- [ ] `IPlatformHelper.isModLoaded(1)`: it has no body (com/dtteam/dynamictrees/platform/services/IPlatformHelper.java:9)
- [ ] `IPlatformHelper.isDevelopmentEnvironment(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IPlatformHelper.java:11)
- [ ] `IPlatformHelper.getEnvironmentName(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/platform/services/IPlatformHelper.java:13)
- [ ] `IPlatformHelper.getMods(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IPlatformHelper.java:17)

### `IRegistryHelper`: it's an interface

- [ ] `IRegistryHelper.getRegistryLoader(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IRegistryHelper.java:11)
- [ ] `IRegistryHelper.newRegistryHandler(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IRegistryHelper.java:13)
- [ ] `IRegistryHelper.getPottedSaplingBlockEntity(0)`: it has no body (com/dtteam/dynamictrees/platform/services/IRegistryHelper.java:15)
- [ ] `IRegistryHelper.newPottedSaplingBlockEntity(2)`: it has no body (com/dtteam/dynamictrees/platform/services/IRegistryHelper.java:17)

### `DendroBrewingMix`: it uses Minecraft's `class_1799`

- [ ] `DendroBrewingMix.DendroBrewingMix(3)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java:5)
- [ ] `DendroBrewingMix.isInput(1)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java:6)
- [ ] `DendroBrewingMix.isIngredient(1)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java:10)
- [ ] `DendroBrewingMix.getOutput(2)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java:14)
- [ ] `DendroBrewingMix.input(0)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java:5)
- [ ] `DendroBrewingMix.ingredient(0)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java:5)
- [ ] `DendroBrewingMix.output(0)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroBrewingMix.java:5)

### `DendroPotionRecipeHandler`

- [ ] `DendroPotionRecipeHandler.brewingRecipes`: it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:23)
- [ ] `DendroPotionRecipeHandler.ingredients`: it uses Minecraft's `class_1799` (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:24)
- [ ] `DendroPotionRecipeHandler.isIngredient(1)`: it uses Minecraft's `class_1799` (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:26)
- [ ] `DendroPotionRecipeHandler.getAllDendroRecipes(0)`: it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:36)
- [ ] `DendroPotionRecipeHandler.setPotion(2)`: it uses Minecraft's `class_1799` (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:57)
- [ ] `DendroPotionRecipeHandler.getRecipe(2)`: it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:64)
- [ ] `DendroPotionRecipeHandler.getRecipe(3)`: it uses DendroBrewingMix, which isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:68)
- [ ] `DendroPotionRecipeHandler.getPotionStack(1)`: it uses Minecraft's `class_1799` (com/dtteam/dynamictrees/recipe/DendroPotionRecipeHandler.java:72)

### `MegaSeedRecipe`: it extends Minecraft's `class_1852`

- [ ] `MegaSeedRecipe.MegaSeedRecipe(1)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:18)
- [ ] `MegaSeedRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:22)
- [ ] `MegaSeedRecipe.assemble(2)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:34)
- [ ] `MegaSeedRecipe.atLeastHasSeed(1)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:44)
- [ ] `MegaSeedRecipe.recipeMatchCondition(2)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:48)
- [ ] `MegaSeedRecipe.allItemsMatchSeed(2)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:55)
- [ ] `MegaSeedRecipe.nonEmptyStacksStream(1)`: it uses `Stream` (java.util.stream) (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:59)
- [ ] `MegaSeedRecipe.method_8113(2)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:63)
- [ ] `MegaSeedRecipe.method_8119(0)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/MegaSeedRecipe.java:67)

### `SeedConversionRecipe`: it extends Minecraft's `class_1852`

- [ ] `SeedConversionRecipe.SeedConversionRecipe(1)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:20)
- [ ] `SeedConversionRecipe.matches(2)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:24)
- [ ] `SeedConversionRecipe.assemble(2)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:46)
- [ ] `SeedConversionRecipe.seedToSaplingCondition(3)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:62)
- [ ] `SeedConversionRecipe.saplingToSeedCondition(2)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:68)
- [ ] `SeedConversionRecipe.hasExactCount(2)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:74)
- [ ] `SeedConversionRecipe.containsAllIngredients(2)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:78)
- [ ] `SeedConversionRecipe.containsOneOfItem(2)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:82)
- [ ] `SeedConversionRecipe.hasDirtBucket(1)`: it uses Minecraft's `class_9694` (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:86)
- [ ] `SeedConversionRecipe.notEmptyInput(1)`: it uses `Stream` (java.util.stream) (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:90)
- [ ] `SeedConversionRecipe.method_8113(2)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:94)
- [ ] `SeedConversionRecipe.method_8119(0)`: its class isn't translated (it extends Minecraft's `class_1852`) (com/dtteam/dynamictrees/recipe/SeedConversionRecipe.java:98)

### `DTRegistries`

- [ ] `DTRegistries.CREATIVE_TAB_ITEMS`: it uses `LinkedList` (java.util) (com/dtteam/dynamictrees/registry/DTRegistries.java:78)
- [ ] `DTRegistries.POTTED_SAPLING`: it uses PottedSaplingBlock, which isn't translated (it extends Minecraft's `class_2237`) (com/dtteam/dynamictrees/registry/DTRegistries.java:79)
- [ ] `DTRegistries.TRUNK_SHELL`: it uses TrunkShellBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/registry/DTRegistries.java:82)
- [ ] `DTRegistries.DENDRO_POTION`: it uses DendroPotion, which isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/registry/DTRegistries.java:83)
- [ ] `DTRegistries.DIRT_BUCKET`: it uses DirtBucket, which isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/registry/DTRegistries.java:84)
- [ ] `DTRegistries.STAFF`: it uses Staff, which isn't translated (it extends Minecraft's `class_1792`) (com/dtteam/dynamictrees/registry/DTRegistries.java:85)
- [ ] `DTRegistries.DT_CREATIVE_TAB`: it uses Minecraft's `class_1761` (com/dtteam/dynamictrees/registry/DTRegistries.java:86)
- [ ] `DTRegistries.FALLING_TREE`: it uses Minecraft's `class_1299` (com/dtteam/dynamictrees/registry/DTRegistries.java:102)
- [ ] `DTRegistries.LINGERING_EFFECTOR`: it uses Minecraft's `class_1299` (com/dtteam/dynamictrees/registry/DTRegistries.java:105)
- [ ] `DTRegistries.SPECIES_BLOCK_ENTITY`: it uses Minecraft's `class_2591` (com/dtteam/dynamictrees/registry/DTRegistries.java:108)
- [ ] `DTRegistries.POTTED_SAPLING_BLOCK_ENTITY`: it uses Minecraft's `class_2591` (com/dtteam/dynamictrees/registry/DTRegistries.java:111)
- [ ] `DTRegistries.FALLING_TREE_HIT_WATER`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:114)
- [ ] `DTRegistries.FALLING_TREE_BIG_START`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:115)
- [ ] `DTRegistries.FALLING_TREE_BIG_END`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:116)
- [ ] `DTRegistries.FALLING_TREE_MEDIUM_START`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:117)
- [ ] `DTRegistries.FALLING_TREE_MEDIUM_END`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:118)
- [ ] `DTRegistries.FALLING_TREE_SMALL_HIT_WATER`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:119)
- [ ] `DTRegistries.FALLING_TREE_SMALL_END`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:122)
- [ ] `DTRegistries.FALLING_TREE_SMALL_END_BARE`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:123)
- [ ] `DTRegistries.FALLING_TREE_FUNGUS_START`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:126)
- [ ] `DTRegistries.FALLING_TREE_FUNGUS_END`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:127)
- [ ] `DTRegistries.FALLING_TREE_FUNGUS_SMALL_END`: it uses Minecraft's `class_3414` (com/dtteam/dynamictrees/registry/DTRegistries.java:128)
- [ ] `DTRegistries.STAFF_HANDLE_COLOR_DATA_COMPONENT`: it uses Minecraft's `class_9331` (com/dtteam/dynamictrees/registry/DTRegistries.java:131)
- [ ] `DTRegistries.STAFF_CRYSTAL_COLOR_DATA_COMPONENT`: it uses Minecraft's `class_9331` (com/dtteam/dynamictrees/registry/DTRegistries.java:134)
- [ ] `DTRegistries.JOCODE_DATA_COMPONENT`: it uses Minecraft's `class_9331` (com/dtteam/dynamictrees/registry/DTRegistries.java:137)
- [ ] `DTRegistries.ROOTS_JOCODE_DATA_COMPONENT`: it uses Minecraft's `class_9331` (com/dtteam/dynamictrees/registry/DTRegistries.java:140)
- [ ] `DTRegistries.READ_ONLY_DATA_COMPONENT`: it uses Minecraft's `class_9331` (com/dtteam/dynamictrees/registry/DTRegistries.java:143)
- [ ] `DTRegistries.SPECIES_DATA_COMPONENT`: it uses Minecraft's `class_9331` (com/dtteam/dynamictrees/registry/DTRegistries.java:148)
- [ ] `DTRegistries.DENDRO_POTION_INDEX_DATA_COMPONENT`: it uses Minecraft's `class_9331` (com/dtteam/dynamictrees/registry/DTRegistries.java:151)
- [ ] `DTRegistries.HEX_COLOR`: it uses Minecraft's `class_2319` (com/dtteam/dynamictrees/registry/DTRegistries.java:154)
- [ ] `DTRegistries.SPECIES_MATCHES`: it uses Minecraft's `class_5342` (com/dtteam/dynamictrees/registry/DTRegistries.java:157)
- [ ] `DTRegistries.SEASONAL_SEED_DROP_CHANCE`: it uses Minecraft's `class_5342` (com/dtteam/dynamictrees/registry/DTRegistries.java:160)
- [ ] `DTRegistries.VOLUNTARY_SEED_DROP_CHANCE`: it uses Minecraft's `class_5342` (com/dtteam/dynamictrees/registry/DTRegistries.java:163)
- [ ] `DTRegistries.ITEM_BY_SPECIES`: it uses Minecraft's `class_5338` (com/dtteam/dynamictrees/registry/DTRegistries.java:166)
- [ ] `DTRegistries.SEED_ITEM`: it uses Minecraft's `class_5338` (com/dtteam/dynamictrees/registry/DTRegistries.java:169)
- [ ] `DTRegistries.WEIGHTED_ITEM`: it uses Minecraft's `class_5338` (com/dtteam/dynamictrees/registry/DTRegistries.java:172)
- [ ] `DTRegistries.MULTIPLY_COUNT`: it uses Minecraft's `class_5339` (com/dtteam/dynamictrees/registry/DTRegistries.java:175)
- [ ] `DTRegistries.MULTIPLY_LOGS_COUNT`: it uses Minecraft's `class_5339` (com/dtteam/dynamictrees/registry/DTRegistries.java:178)
- [ ] `DTRegistries.MULTIPLY_STICKS_COUNT`: it uses Minecraft's `class_5339` (com/dtteam/dynamictrees/registry/DTRegistries.java:181)
- [ ] `DTRegistries.DYNAMIC_TREE_CONFIGURED_FEATURE`: it uses Minecraft's `class_5321` (com/dtteam/dynamictrees/registry/DTRegistries.java:184)
- [ ] `DTRegistries.CAVE_ROOTED_TREE_CONFIGURED_FEATURE`: it uses Minecraft's `class_5321` (com/dtteam/dynamictrees/registry/DTRegistries.java:187)
- [ ] `DTRegistries.DYNAMIC_TREE_PLACED_FEATURE`: it uses Minecraft's `class_5321` (com/dtteam/dynamictrees/registry/DTRegistries.java:190)
- [ ] `DTRegistries.CAVE_ROOTED_TREE_PLACED_FEATURE`: it uses Minecraft's `class_5321` (com/dtteam/dynamictrees/registry/DTRegistries.java:193)
- [ ] `DTRegistries.CAVE_ROOTED_TREE_PLACEMENT_MODIFIER_TYPE`: it uses Minecraft's `class_6798` (com/dtteam/dynamictrees/registry/DTRegistries.java:196)
- [ ] `DTRegistries.DYNAMIC_TREE_FEATURE`: it uses DynamicTreeFeature, which isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/registry/DTRegistries.java:199)
- [ ] `DTRegistries.CAVE_ROOTED_TREE_FEATURE`: it uses CaveRootedTreeFeature, which isn't translated (it extends `DynamicTreeFeature`) (com/dtteam/dynamictrees/registry/DTRegistries.java:202)
- [ ] `DTRegistries.REPLACE_NYLIUM_FUNGI_BLOCK_STATE_PROVIDER_TYPE`: it uses Minecraft's `class_4652` (com/dtteam/dynamictrees/registry/DTRegistries.java:205)
- [ ] `DTRegistries.CANCEL_VANILLA_VILLAGE_TREE_STRUCTURE_POOL_ELEMENT_TYPE`: it uses Minecraft's `class_3816` (com/dtteam/dynamictrees/registry/DTRegistries.java:208)
- [ ] `DTRegistries.TREE_STRUCTURE_POOL_ELEMENT_TYPE`: it uses Minecraft's `class_3816` (com/dtteam/dynamictrees/registry/DTRegistries.java:211)
- [ ] `DTRegistries.SEED_CONVERSION_RECIPE_TYPE`: it uses Minecraft's `class_1865` (com/dtteam/dynamictrees/registry/DTRegistries.java:214)
- [ ] `DTRegistries.MEGA_SEED_RECIPE_TYPE`: it uses Minecraft's `class_1865` (com/dtteam/dynamictrees/registry/DTRegistries.java:217)
- [ ] `DTRegistries.setup(0)`: it calls DTRegistries.setupConnectables, which isn't translated (com/dtteam/dynamictrees/registry/DTRegistries.java:221)
- [ ] `DTRegistries.setupConnectables(0)`: it calls BranchConnectables.makeBlockConnectable, which isn't translated (com/dtteam/dynamictrees/registry/DTRegistries.java:225)
- [ ] `DTRegistries.getAllRootyBlocks(0)`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/registry/DTRegistries.java:238)

### `FabricRegistryHandler`: it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)

- [ ] `FabricRegistryHandler.FabricRegistryHandler(0)`: its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:14)
- [ ] `FabricRegistryHandler.FabricRegistryHandler(1)`: its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:17)
- [ ] `FabricRegistryHandler.setup(1)`: it uses RegistryHandler, which isn't translated (it extends `RegistryEntry`) (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:22)
- [ ] `FabricRegistryHandler.warnIfInvalid(2)`: its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:26)
- [ ] `FabricRegistryHandler.getBlock(1)`: its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:34)
- [ ] `FabricRegistryHandler.getItem(1)`: its class isn't translated (it extends `RegistryHandler` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:41)
- [ ] `FabricRegistryHandler.putBlock(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:48)
- [ ] `FabricRegistryHandler.putItem(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryHandler.java:58)

### `FabricRegistryHandler.RegisterEventHandler`: it's generic


### `FabricRegistryLoader`: it extends `RegistryLoader`

- [ ] `FabricRegistryLoader.setup(0)`: it calls DTRegistries.setup, which isn't translated (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:49)
- [ ] `FabricRegistryLoader.registerBlock(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:54)
- [ ] `FabricRegistryLoader.registerItem(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:60)
- [ ] `FabricRegistryLoader.registerRecipeType(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:66)
- [ ] `FabricRegistryLoader.registerCreativeTab(4)`: its class isn't translated (it extends `RegistryLoader`) (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:72)
- [ ] `FabricRegistryLoader.registerEntity(3)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:82)
- [ ] `FabricRegistryLoader.registerBlockEntity(3)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:88)
- [ ] `FabricRegistryLoader.registerSoundEvent(1)`: its class isn't translated (it extends `RegistryLoader`) (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:98)
- [ ] `FabricRegistryLoader.registerDataComponentType(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:105)
- [ ] `FabricRegistryLoader.registerCommandArgumentType(3)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:113)
- [ ] `FabricRegistryLoader.registerLootConditionType(2)`: its class isn't translated (it extends `RegistryLoader`) (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:122)
- [ ] `FabricRegistryLoader.registerLootPoolEntryType(2)`: its class isn't translated (it extends `RegistryLoader`) (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:128)
- [ ] `FabricRegistryLoader.registerLootFunctionType(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:134)
- [ ] `FabricRegistryLoader.registerPlacementModifierType(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:140)
- [ ] `FabricRegistryLoader.registerFeature(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:146)
- [ ] `FabricRegistryLoader.registerBlockStateProviderType(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:152)
- [ ] `FabricRegistryLoader.registerStructurePoolElementType(2)`: it's generic (com/dtteam/dynamictrees/registry/FabricRegistryLoader.java:158)

### `RegistryEvent`: it's an interface


### `RegistryLoader`

- [ ] `RegistryLoader.registerCreativeTab(4)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:42)
- [ ] `RegistryLoader.registerBlock(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:44)
- [ ] `RegistryLoader.registerItem(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:46)
- [ ] `RegistryLoader.registerRecipeType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:48)
- [ ] `RegistryLoader.registerEntity(3)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:50)
- [ ] `RegistryLoader.registerBlockEntity(3)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:52)
- [ ] `RegistryLoader.registerSoundEvent(1)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:54)
- [ ] `RegistryLoader.registerDataComponentType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:56)
- [ ] `RegistryLoader.registerCommandArgumentType(3)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:58)
- [ ] `RegistryLoader.registerLootConditionType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:62)
- [ ] `RegistryLoader.registerLootPoolEntryType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:64)
- [ ] `RegistryLoader.registerLootFunctionType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:66)
- [ ] `RegistryLoader.registerPlacementModifierType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:68)
- [ ] `RegistryLoader.registerFeature(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:70)
- [ ] `RegistryLoader.registerBlockStateProviderType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:72)
- [ ] `RegistryLoader.registerStructurePoolElementType(2)`: it has no body (com/dtteam/dynamictrees/registry/RegistryLoader.java:74)

### `BranchConnectables`

- [ ] `BranchConnectables.connectablesMap`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/systems/BranchConnectables.java:14)
- [ ] `BranchConnectables.makeBlockConnectable(2)`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/systems/BranchConnectables.java:16)
- [ ] `BranchConnectables.isBlockConnectable(1)`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/systems/BranchConnectables.java:20)
- [ ] `BranchConnectables.getConnectionRadiusForBlock(4)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/systems/BranchConnectables.java:24)

### `AcaciaLeafCell`: it extends `MatrixCell`

- [x] `AcaciaLeafCell.valMap` (com/dtteam/dynamictrees/systems/cell/AcaciaLeafCell.java:4)
- [ ] `AcaciaLeafCell.AcaciaLeafCell(1)`: its class isn't translated (it extends `MatrixCell`) (com/dtteam/dynamictrees/systems/cell/AcaciaLeafCell.java:8)

### `CellKits`

- [ ] `CellKits.DECIDUOUS`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:13)
- [ ] `CellKits.CONIFER`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:52)
- [ ] `CellKits.ACACIA`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:96)
- [ ] `CellKits.DARK_OAK`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:147)
- [ ] `CellKits.BARE`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:186)
- [ ] `CellKits.PALM`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:214)
- [ ] `CellKits.NETHER_FUNGUS`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:263)
- [ ] `CellKits.AZALEA`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/cell/CellKits.java:307)
- [ ] `CellKits.register(1)`: it uses Registry, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/cell/CellKits.java:347)
- [x] `CellKits.solveCell(2)` (com/dtteam/dynamictrees/systems/cell/CellKits.java:351)

### `CellKits.BasicSolver`: it implements `CellSolver` (com.dtteam.dynamictrees.api.cell)

- [ ] `CellKits.BasicSolver.BasicSolver(1)`: its class isn't translated (it implements `CellSolver` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/CellKits.java:364)
- [ ] `CellKits.BasicSolver.solve(1)`: its class isn't translated (it implements `CellSolver` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/CellKits.java:368)

### `ConiferBranchCell`: it implements `Cell` (com.dtteam.dynamictrees.api.cell)

- [x] `ConiferBranchCell.map` (com/dtteam/dynamictrees/systems/cell/ConiferBranchCell.java:7)
- [ ] `ConiferBranchCell.getValue(0)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/ConiferBranchCell.java:9)
- [ ] `ConiferBranchCell.getValueFromSide(1)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/ConiferBranchCell.java:14)

### `ConiferLeafCell`: it extends `MatrixCell`

- [x] `ConiferLeafCell.valMap` (com/dtteam/dynamictrees/systems/cell/ConiferLeafCell.java:4)
- [ ] `ConiferLeafCell.ConiferLeafCell(1)`: its class isn't translated (it extends `MatrixCell`) (com/dtteam/dynamictrees/systems/cell/ConiferLeafCell.java:8)

### `ConiferTopBranchCell`: it implements `Cell` (com.dtteam.dynamictrees.api.cell)

- [x] `ConiferTopBranchCell.map` (com/dtteam/dynamictrees/systems/cell/ConiferTopBranchCell.java:7)
- [ ] `ConiferTopBranchCell.getValue(0)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/ConiferTopBranchCell.java:9)
- [ ] `ConiferTopBranchCell.getValueFromSide(1)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/ConiferTopBranchCell.java:14)

### `DarkOakLeafCell`: it extends `MatrixCell`

- [x] `DarkOakLeafCell.valMap` (com/dtteam/dynamictrees/systems/cell/DarkOakLeafCell.java:4)
- [ ] `DarkOakLeafCell.DarkOakLeafCell(1)`: its class isn't translated (it extends `MatrixCell`) (com/dtteam/dynamictrees/systems/cell/DarkOakLeafCell.java:8)

### `LeafClusters`

- [ ] `LeafClusters.NULL_MAP`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:7)
- [ ] `LeafClusters.DECIDUOUS`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:8)
- [ ] `LeafClusters.CONIFER`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:116)
- [ ] `LeafClusters.ACACIA`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:174)
- [ ] `LeafClusters.DARK_OAK`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:280)
- [ ] `LeafClusters.BARE`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:533)
- [ ] `LeafClusters.PALM`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:534)
- [ ] `LeafClusters.BUSH`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:538)
- [ ] `LeafClusters.NETHER_FUNGUS`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:596)
- [ ] `LeafClusters.AZALEA`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/cell/LeafClusters.java:658)

### `MatrixCell`: it implements `Cell` (com.dtteam.dynamictrees.api.cell)

- [ ] `MatrixCell.MatrixCell(2)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/MatrixCell.java:10)
- [ ] `MatrixCell.getValue(0)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/MatrixCell.java:15)
- [ ] `MatrixCell.getValueFromSide(1)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/MatrixCell.java:20)

### `MetadataCell`

- [x] `MetadataCell.NONE` (com/dtteam/dynamictrees/systems/cell/MetadataCell.java:4)
- [x] `MetadataCell.TOP_BRANCH` (com/dtteam/dynamictrees/systems/cell/MetadataCell.java:5)
- [x] `MetadataCell.radiusAndMeta(2)` (com/dtteam/dynamictrees/systems/cell/MetadataCell.java:7)
- [x] `MetadataCell.getRadius(1)` (com/dtteam/dynamictrees/systems/cell/MetadataCell.java:11)
- [x] `MetadataCell.getMeta(1)` (com/dtteam/dynamictrees/systems/cell/MetadataCell.java:15)

### `NetherFungusLeafCell`: it extends `MatrixCell`

- [x] `NetherFungusLeafCell.valMap` (com/dtteam/dynamictrees/systems/cell/NetherFungusLeafCell.java:4)
- [ ] `NetherFungusLeafCell.NetherFungusLeafCell(1)`: its class isn't translated (it extends `MatrixCell`) (com/dtteam/dynamictrees/systems/cell/NetherFungusLeafCell.java:8)

### `NormalCell`: it implements `Cell` (com.dtteam.dynamictrees.api.cell)

- [ ] `NormalCell.NormalCell(1)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/NormalCell.java:9)
- [ ] `NormalCell.getValue(0)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/NormalCell.java:13)
- [ ] `NormalCell.getValueFromSide(1)`: its class isn't translated (it implements `Cell` (com.dtteam.dynamictrees.api.cell)) (com/dtteam/dynamictrees/systems/cell/NormalCell.java:18)

### `PalmFrondCell`: it extends `MatrixCell`

- [x] `PalmFrondCell.valMap` (com/dtteam/dynamictrees/systems/cell/PalmFrondCell.java:4)
- [ ] `PalmFrondCell.PalmFrondCell(1)`: its class isn't translated (it extends `MatrixCell`) (com/dtteam/dynamictrees/systems/cell/PalmFrondCell.java:8)

### `FutureBreak`: it uses Minecraft's `class_2680`

- [ ] `FutureBreak.FUTURE_BREAKS`: it uses FutureBreak, which isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/systems/FutureBreak.java:12)
- [ ] `FutureBreak.FutureBreak(5)`: its class isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/systems/FutureBreak.java:19)
- [ ] `FutureBreak.add(1)`: it uses FutureBreak, which isn't translated (it uses Minecraft's `class_2680`) (com/dtteam/dynamictrees/systems/FutureBreak.java:27)
- [ ] `FutureBreak.process(1)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/systems/FutureBreak.java:33)

### `AlternativeLeavesGenFeature`: it extends `GenFeature`

- [ ] `AlternativeLeavesGenFeature.ALT_LEAVES`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:25)
- [ ] `AlternativeLeavesGenFeature.ALT_LEAVES_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:26)
- [ ] `AlternativeLeavesGenFeature.AlternativeLeavesGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:28)
- [ ] `AlternativeLeavesGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:32)
- [ ] `AlternativeLeavesGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:37)
- [ ] `AlternativeLeavesGenFeature.shouldApply(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:46)
- [ ] `AlternativeLeavesGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:55)
- [ ] `AlternativeLeavesGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:61)
- [ ] `AlternativeLeavesGenFeature.getAltLeavesBlock(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:81)
- [ ] `AlternativeLeavesGenFeature.getSwapBlockState(5)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:88)
- [ ] `AlternativeLeavesGenFeature.setAltLeaves(5)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/AlternativeLeavesGenFeature.java:105)

### `BeeNestGenFeature`: it extends `GenFeature`

- [ ] `BeeNestGenFeature.NEST_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:31)
- [ ] `BeeNestGenFeature.WORLD_GEN_CHANCE_FUNCTION`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:32)
- [x] `BeeNestGenFeature.GUARANTEED_CHANCE` (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:35)
- [x] `BeeNestGenFeature.COMMON_CHANCE` (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:36)
- [x] `BeeNestGenFeature.UNCOMMON_CHANCE` (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:37)
- [x] `BeeNestGenFeature.RARE_CHANCE` (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:38)
- [x] `BeeNestGenFeature.GROW_CHANCE` (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:39)
- [ ] `BeeNestGenFeature.BeeNestGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:41)
- [ ] `BeeNestGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:45)
- [ ] `BeeNestGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:50)
- [ ] `BeeNestGenFeature.getWorldGenChanceFunction(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:60)
- [ ] `BeeNestGenFeature.getCanGrowPredicate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:73)
- [ ] `BeeNestGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:87)
- [ ] `BeeNestGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:95)
- [ ] `BeeNestGenFeature.placeBeeNestInValidPlace(5)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:102)
- [ ] `BeeNestGenFeature.placeBeeNestWithBees(6)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:121)
- [ ] `BeeNestGenFeature.storeBee(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:138)
- [ ] `BeeNestGenFeature.nestAlreadyPresent(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:142)
- [ ] `BeeNestGenFeature.getTreeHeight(3)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:156)
- [ ] `BeeNestGenFeature.findBranchPits(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BeeNestGenFeature.java:166)

### `BeeNestGenFeature.WorldGenChanceFunction`: it's an interface


### `BiomePredicateGenFeature`: it extends `GenFeature`

- [ ] `BiomePredicateGenFeature.ONLY_WORLD_GEN`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:13)
- [ ] `BiomePredicateGenFeature.GEN_FEATURE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:14)
- [ ] `BiomePredicateGenFeature.BiomePredicateGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:16)
- [ ] `BiomePredicateGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:20)
- [ ] `BiomePredicateGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:25)
- [ ] `BiomePredicateGenFeature.shouldApply(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:33)
- [ ] `BiomePredicateGenFeature.generate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:38)
- [ ] `BiomePredicateGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:51)
- [ ] `BiomePredicateGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:64)
- [ ] `BiomePredicateGenFeature.postRot(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BiomePredicateGenFeature.java:76)

### `BottomFlareGenFeature`: it extends `GenFeature`

- [ ] `BottomFlareGenFeature.MIN_RADIUS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java:15)
- [ ] `BottomFlareGenFeature.BottomFlareGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java:17)
- [ ] `BottomFlareGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java:21)
- [ ] `BottomFlareGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java:26)
- [ ] `BottomFlareGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java:31)
- [ ] `BottomFlareGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java:41)
- [ ] `BottomFlareGenFeature.flareBottom(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BottomFlareGenFeature.java:47)

### `BushGenFeature`: it extends `GenFeature`

- [ ] `BushGenFeature.LOG`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:24)
- [ ] `BushGenFeature.LEAVES`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:25)
- [ ] `BushGenFeature.SECONDARY_LEAVES`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:26)
- [ ] `BushGenFeature.SECONDARY_LEAVES_CHANCE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:27)
- [ ] `BushGenFeature.BushGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:29)
- [ ] `BushGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:33)
- [ ] `BushGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:38)
- [ ] `BushGenFeature.generate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:48)
- [ ] `BushGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:54)
- [ ] `BushGenFeature.commonGen(7)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:64)
- [ ] `BushGenFeature.placeLeaves(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:95)
- [ ] `BushGenFeature.selectLeavesBlock(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:102)
- [ ] `BushGenFeature.placeLeavesBlock(3)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:106)
- [ ] `BushGenFeature.coordHashCode(1)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/systems/genfeature/BushGenFeature.java:115)

### `ClearVolumeGenFeature`: it extends `GenFeature`

- [ ] `ClearVolumeGenFeature.HEIGHT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/ClearVolumeGenFeature.java:10)
- [ ] `ClearVolumeGenFeature.ClearVolumeGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ClearVolumeGenFeature.java:12)
- [ ] `ClearVolumeGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ClearVolumeGenFeature.java:16)
- [ ] `ClearVolumeGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ClearVolumeGenFeature.java:21)
- [ ] `ClearVolumeGenFeature.preGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ClearVolumeGenFeature.java:26)

### `ConiferTopperGenFeature`: it extends `GenFeature`

- [ ] `ConiferTopperGenFeature.LEAVES_PROPERTIES`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/ConiferTopperGenFeature.java:14)
- [ ] `ConiferTopperGenFeature.ConiferTopperGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ConiferTopperGenFeature.java:16)
- [ ] `ConiferTopperGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ConiferTopperGenFeature.java:20)
- [ ] `ConiferTopperGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ConiferTopperGenFeature.java:25)
- [ ] `ConiferTopperGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ConiferTopperGenFeature.java:30)

### `FullGenerationContext`: it extends `GenFeatureContext`

- [ ] `FullGenerationContext.FullGenerationContext(6)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/FullGenerationContext.java:14)
- [ ] `FullGenerationContext.biome(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/FullGenerationContext.java:21)
- [ ] `FullGenerationContext.radius(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/FullGenerationContext.java:25)
- [ ] `FullGenerationContext.isWorldGen(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/FullGenerationContext.java:29)

### `GenFeatureContext`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)

- [ ] `GenFeatureContext.GenFeatureContext(3)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/systems/genfeature/context/GenFeatureContext.java:14)
- [ ] `GenFeatureContext.levelContext(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/systems/genfeature/context/GenFeatureContext.java:20)
- [ ] `GenFeatureContext.level(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/systems/genfeature/context/GenFeatureContext.java:24)
- [ ] `GenFeatureContext.pos(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/systems/genfeature/context/GenFeatureContext.java:28)
- [ ] `GenFeatureContext.species(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/systems/genfeature/context/GenFeatureContext.java:32)
- [ ] `GenFeatureContext.random(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/systems/genfeature/context/GenFeatureContext.java:36)

### `PostGenerationContext`: it extends `GenFeatureContext`

- [ ] `PostGenerationContext.PostGenerationContext(3)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:20)
- [ ] `PostGenerationContext.originPos(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:31)
- [ ] `PostGenerationContext.biome(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:35)
- [ ] `PostGenerationContext.radius(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:39)
- [ ] `PostGenerationContext.endPoints(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:43)
- [ ] `PostGenerationContext.initialDirtState(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:47)
- [ ] `PostGenerationContext.seasonValue(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:51)
- [ ] `PostGenerationContext.fruitProductionFactor(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:55)
- [ ] `PostGenerationContext.isWorldGen(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGenerationContext.java:59)

### `PostGrowContext`: it extends `GenFeatureContext`

- [ ] `PostGrowContext.PostGrowContext(6)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGrowContext.java:12)
- [ ] `PostGrowContext.treePos(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGrowContext.java:19)
- [ ] `PostGrowContext.fertility(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGrowContext.java:23)
- [ ] `PostGrowContext.natural(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostGrowContext.java:27)

### `PostRotContext`: it extends `GenFeatureContext`

- [ ] `PostRotContext.PostRotContext(7)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostRotContext.java:13)
- [ ] `PostRotContext.radius(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostRotContext.java:21)
- [ ] `PostRotContext.neighbourCount(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostRotContext.java:25)
- [ ] `PostRotContext.fertility(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostRotContext.java:29)
- [ ] `PostRotContext.rapid(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PostRotContext.java:33)

### `PreGenerationContext`: it extends `GenFeatureContext`

- [ ] `PreGenerationContext.PreGenerationContext(7)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PreGenerationContext.java:15)
- [ ] `PreGenerationContext.radius(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PreGenerationContext.java:23)
- [ ] `PreGenerationContext.facing(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PreGenerationContext.java:27)
- [ ] `PreGenerationContext.joCode(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PreGenerationContext.java:31)
- [ ] `PreGenerationContext.isWorldGen(0)`: its class isn't translated (it extends `GenFeatureContext`) (com/dtteam/dynamictrees/systems/genfeature/context/PreGenerationContext.java:35)

### `FruitGenFeature`: it extends `GenFeature`

- [ ] `FruitGenFeature.FRUIT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:21)
- [ ] `FruitGenFeature.FruitGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:23)
- [ ] `FruitGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:27)
- [ ] `FruitGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:32)
- [ ] `FruitGenFeature.shouldApply(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:42)
- [ ] `FruitGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:47)
- [ ] `FruitGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:66)
- [ ] `FruitGenFeature.place(6)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:94)
- [ ] `FruitGenFeature.shouldPlace(3)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:101)
- [ ] `FruitGenFeature.placeDuringWorldGen(7)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:107)
- [ ] `FruitGenFeature.shouldPlaceDuringWorldGen(3)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/FruitGenFeature.java:116)

### `GenFeature`: it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)

- [ ] `GenFeature.VERTICAL_SPREAD`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:22)
- [ ] `GenFeature.QUANTITY`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:23)
- [ ] `GenFeature.RAY_DISTANCE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:24)
- [ ] `GenFeature.MAX_HEIGHT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:25)
- [ ] `GenFeature.CAN_GROW_PREDICATE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:26)
- [ ] `GenFeature.MAX_COUNT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:29)
- [ ] `GenFeature.FRUITING_RADIUS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:30)
- [ ] `GenFeature.PLACE_CHANCE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:31)
- [ ] `GenFeature.BIOME_PREDICATE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:32)
- [ ] `GenFeature.NULL`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:33)
- [ ] `GenFeature.REGISTRY`: it uses ConfigurableRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:42)
- [ ] `GenFeature.GenFeature(1)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:46)
- [ ] `GenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:50)
- [ ] `GenFeature.generate(3)`: it's generic (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:54)
- [ ] `GenFeature.preGenerate(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:58)
- [ ] `GenFeature.postGenerate(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:62)
- [ ] `GenFeature.postGrow(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:66)
- [ ] `GenFeature.postRot(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:70)
- [ ] `GenFeature.generate(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:74)
- [ ] `GenFeature.shouldApply(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:78)

### `GenFeature.CanGrowPredicate`: it's an interface


### `GenFeature.Type`: it's generic

- [ ] `GenFeature.Type.PRE_GENERATION`: it uses Type, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:87)
- [ ] `GenFeature.Type.POST_GENERATION`: it uses Type, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:88)
- [ ] `GenFeature.Type.POST_GROW`: it uses Type, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:89)
- [ ] `GenFeature.Type.POST_ROT`: it uses Type, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:90)
- [ ] `GenFeature.Type.FULL`: it uses Type, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:91)
- [ ] `GenFeature.Type.Type(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:94)
- [ ] `GenFeature.Type.generate(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeature.java:98)

### `GenFeatureConfiguration`: it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)

- [ ] `GenFeatureConfiguration.TEMPLATES`: it uses TemplateRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java:9)
- [ ] `GenFeatureConfiguration.GenFeatureConfiguration(1)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java:11)
- [ ] `GenFeatureConfiguration.getGenFeature(0)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java:15)
- [ ] `GenFeatureConfiguration.copy(0)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java:19)
- [ ] `GenFeatureConfiguration.generate(2)`: it's generic (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java:25)
- [ ] `GenFeatureConfiguration.shouldApply(1)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java:29)
- [ ] `GenFeatureConfiguration.getNull(0)`: it uses GenFeatureConfiguration, which isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatureConfiguration.java:33)

### `GenFeatures`

- [ ] `GenFeatures.FRUIT`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:7)
- [ ] `GenFeatures.POD`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:8)
- [ ] `GenFeatures.VINES`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:9)
- [ ] `GenFeatures.BEE_NEST`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:10)
- [ ] `GenFeatures.SHROOMLIGHT`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:11)
- [ ] `GenFeatures.CONIFER_TOPPER`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:12)
- [ ] `GenFeatures.MOUND`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:13)
- [ ] `GenFeatures.ROOTS`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:14)
- [ ] `GenFeatures.CLEAR_VOLUME`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:15)
- [ ] `GenFeatures.BOTTOM_FLARE`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:16)
- [ ] `GenFeatures.UNDERGROWTH`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:17)
- [ ] `GenFeatures.PODZOL`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:18)
- [ ] `GenFeatures.BUSH`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:19)
- [ ] `GenFeatures.HUGE_MUSHROOM`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:20)
- [ ] `GenFeatures.HUGE_MUSHROOMS`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:21)
- [ ] `GenFeatures.MUSHROOM_ROT`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:22)
- [ ] `GenFeatures.ROT_SOIL`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:23)
- [ ] `GenFeatures.BIOME_PREDICATE`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:24)
- [ ] `GenFeatures.RANDOM_PREDICATE`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:25)
- [ ] `GenFeatures.ALTERNATIVE_LEAVES`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:26)
- [ ] `GenFeatures.ROOT_SYSTEM`: it uses GenFeature, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:27)
- [ ] `GenFeatures.register(1)`: it uses Registry, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/genfeature/GenFeatures.java:29)

### `HugeMushroomGenFeature`: it extends `GenFeature`

- [ ] `HugeMushroomGenFeature.MUSHROOM_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:22)
- [ ] `HugeMushroomGenFeature.STEM_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:23)
- [ ] `HugeMushroomGenFeature.BROWN_CAP`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:25)
- [ ] `HugeMushroomGenFeature.BROWN_CAP_MEDIUM`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:34)
- [ ] `HugeMushroomGenFeature.BROWN_CAP_SMALL`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:36)
- [ ] `HugeMushroomGenFeature.RED_CAP`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:37)
- [ ] `HugeMushroomGenFeature.RED_CAP_SHORT`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:145)
- [ ] `HugeMushroomGenFeature.RED_CAP_SMALL`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:228)
- [ ] `HugeMushroomGenFeature.HugeMushroomGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:233)
- [ ] `HugeMushroomGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:237)
- [ ] `HugeMushroomGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:242)
- [ ] `HugeMushroomGenFeature.shouldApply(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:247)
- [ ] `HugeMushroomGenFeature.setHeight(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:252)
- [ ] `HugeMushroomGenFeature.getCapForHeight(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:257)
- [ ] `HugeMushroomGenFeature.getMushroomHeight(5)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:274)
- [ ] `HugeMushroomGenFeature.generate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:278)
- [ ] `HugeMushroomGenFeature.generateMushrooms(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:283)
- [ ] `HugeMushroomGenFeature.getMushroomStateForValue(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomGenFeature.java:331)

### `HugeMushroomUndergrowthGenFeature`: it extends `HugeMushroomGenFeature`

- [ ] `HugeMushroomUndergrowthGenFeature.MAX_MUSHROOMS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java:18)
- [ ] `HugeMushroomUndergrowthGenFeature.MAX_ATTEMPTS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java:19)
- [ ] `HugeMushroomUndergrowthGenFeature.HugeMushroomUndergrowthGenFeature(1)`: its class isn't translated (it extends `HugeMushroomGenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java:21)
- [ ] `HugeMushroomUndergrowthGenFeature.registerProperties(0)`: its class isn't translated (it extends `HugeMushroomGenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java:25)
- [ ] `HugeMushroomUndergrowthGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `HugeMushroomGenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java:31)
- [ ] `HugeMushroomUndergrowthGenFeature.generate(2)`: its class isn't translated (it extends `HugeMushroomGenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java:36)
- [ ] `HugeMushroomUndergrowthGenFeature.postGenerate(2)`: its class isn't translated (it extends `HugeMushroomGenFeature`) (com/dtteam/dynamictrees/systems/genfeature/HugeMushroomUndergrowthGenFeature.java:41)

### `MoundGenFeature`: it extends `GenFeature`

- [ ] `MoundGenFeature.moundMap`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:15)
- [ ] `MoundGenFeature.MOUND_CUTOFF_RADIUS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:123)
- [ ] `MoundGenFeature.OFFSET`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:124)
- [ ] `MoundGenFeature.MoundGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:126)
- [ ] `MoundGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:130)
- [ ] `MoundGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:135)
- [ ] `MoundGenFeature.preGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:140)
- [ ] `MoundGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MoundGenFeature.java:160)

### `MushroomRotGenFeature`: it extends `GenFeature`

- [ ] `MushroomRotGenFeature.MUSHROOM`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:13)
- [ ] `MushroomRotGenFeature.ALTERNATE_MUSHROOM`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:14)
- [ ] `MushroomRotGenFeature.ALTERNATE_MUSHROOM_CHANCE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:15)
- [ ] `MushroomRotGenFeature.MushroomRotGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:17)
- [ ] `MushroomRotGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:21)
- [ ] `MushroomRotGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:26)
- [ ] `MushroomRotGenFeature.postRot(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:34)
- [ ] `MushroomRotGenFeature.mayMushroomPlaceOn(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/MushroomRotGenFeature.java:49)

### `PodGenFeature`: it extends `GenFeature`

- [ ] `PodGenFeature.POD`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:21)
- [ ] `PodGenFeature.BLOCKS_PER_POD`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:22)
- [ ] `PodGenFeature.LOWEST_TRUNK_HEIGHT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:23)
- [ ] `PodGenFeature.PodGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:25)
- [ ] `PodGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:29)
- [ ] `PodGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:34)
- [ ] `PodGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:44)
- [ ] `PodGenFeature.shouldGrow(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:68)
- [ ] `PodGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:76)
- [ ] `PodGenFeature.shouldGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:95)
- [ ] `PodGenFeature.place(7)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodGenFeature.java:99)

### `PodzolGenFeature`: it extends `GenFeature`

- [ ] `PodzolGenFeature.PODZOL_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:28)
- [ ] `PodzolGenFeature.SPREAD_DISTANCE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:29)
- [ ] `PodzolGenFeature.DARK_THRESHOLD`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:30)
- [ ] `PodzolGenFeature.MAX_HEIGHT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:31)
- [ ] `PodzolGenFeature.KILL_PLANTS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:32)
- [ ] `PodzolGenFeature.PodzolGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:34)
- [ ] `PodzolGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:38)
- [ ] `PodzolGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:43)
- [ ] `PodzolGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:53)
- [ ] `PodzolGenFeature.shouldDestroyPlant(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:104)
- [ ] `PodzolGenFeature.spreadPodzol(3)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/systems/genfeature/PodzolGenFeature.java:109)

### `RandomPredicateGenFeature`: it extends `GenFeature`

- [ ] `RandomPredicateGenFeature.ONLY_WORLD_GEN`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:15)
- [ ] `RandomPredicateGenFeature.GEN_FEATURE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:16)
- [ ] `RandomPredicateGenFeature.RandomPredicateGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:18)
- [ ] `RandomPredicateGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:22)
- [ ] `RandomPredicateGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:27)
- [ ] `RandomPredicateGenFeature.shouldApply(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:32)
- [ ] `RandomPredicateGenFeature.preGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:37)
- [ ] `RandomPredicateGenFeature.generate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:48)
- [ ] `RandomPredicateGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:59)
- [ ] `RandomPredicateGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:70)
- [ ] `RandomPredicateGenFeature.postRot(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RandomPredicateGenFeature.java:80)

### `RootsGenFeature`: it extends `GenFeature`

- [ ] `RootsGenFeature.MIN_TRUNK_RADIUS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:22)
- [ ] `RootsGenFeature.LEVEL_LIMIT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:23)
- [ ] `RootsGenFeature.SCALE_FACTOR`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:24)
- [ ] `RootsGenFeature.RootsGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:31)
- [ ] `RootsGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:35)
- [ ] `RootsGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:40)
- [ ] `RootsGenFeature.createRootMaps(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:45)
- [ ] `RootsGenFeature.getMinTrunkRadius(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:63)
- [ ] `RootsGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:67)
- [ ] `RootsGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:74)
- [ ] `RootsGenFeature.startRoots(5)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:86)
- [ ] `RootsGenFeature.nextRoot(13)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:107)
- [ ] `RootsGenFeature.isReplaceableWithRoots(3)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootsGenFeature.java:168)

### `RootSystemGenFeature`: it extends `GenFeature`

- [ ] `RootSystemGenFeature.MAX_ROOT_COLUMN_HEIGHT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:20)
- [ ] `RootSystemGenFeature.ROOT_RADIUS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:21)
- [ ] `RootSystemGenFeature.ROOT_REPLACEABLE_TAG`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:22)
- [ ] `RootSystemGenFeature.ROOT_PLACEMENT_ATTEMPTS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:23)
- [ ] `RootSystemGenFeature.ROOT_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:24)
- [ ] `RootSystemGenFeature.HANGING_ROOT_RADIUS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:25)
- [ ] `RootSystemGenFeature.HANGING_ROOT_VERTICAL_SPAN`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:26)
- [ ] `RootSystemGenFeature.HANGING_ROOT_PLACEMENT_ATTEMPTS`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:27)
- [ ] `RootSystemGenFeature.HANGING_ROOT_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:28)
- [ ] `RootSystemGenFeature.RootSystemGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:30)
- [ ] `RootSystemGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:34)
- [ ] `RootSystemGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:49)
- [ ] `RootSystemGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:63)
- [ ] `RootSystemGenFeature.placeRootedDirt(6)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:81)
- [ ] `RootSystemGenFeature.placeHangingRoots(5)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/systems/genfeature/RootSystemGenFeature.java:98)

### `RotSoilGenFeature`: it extends `GenFeature`

- [ ] `RotSoilGenFeature.ROTTEN_SOIL`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/RotSoilGenFeature.java:13)
- [ ] `RotSoilGenFeature.RotSoilGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RotSoilGenFeature.java:15)
- [ ] `RotSoilGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RotSoilGenFeature.java:19)
- [ ] `RotSoilGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RotSoilGenFeature.java:24)
- [ ] `RotSoilGenFeature.postRot(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/RotSoilGenFeature.java:29)

### `ShroomlightGenFeature`: it extends `GenFeature`

- [ ] `ShroomlightGenFeature.SHROOMLIGHT_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:21)
- [ ] `ShroomlightGenFeature.HORIZONTALS`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:22)
- [x] `ShroomlightGenFeature.VANILLA_GROW_CHANCE` (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:23)
- [ ] `ShroomlightGenFeature.ShroomlightGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:25)
- [ ] `ShroomlightGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:29)
- [ ] `ShroomlightGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:34)
- [ ] `ShroomlightGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:44)
- [ ] `ShroomlightGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:60)
- [ ] `ShroomlightGenFeature.placeShroomlightsInValidPlace(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:68)
- [ ] `ShroomlightGenFeature.getTreeHeight(3)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:99)
- [ ] `ShroomlightGenFeature.findBranchPits(4)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/ShroomlightGenFeature.java:109)

### `UndergrowthGenFeature`: it extends `GenFeature`

- [ ] `UndergrowthGenFeature.SPECIES_A`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:19)
- [ ] `UndergrowthGenFeature.SPECIES_B`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:20)
- [ ] `UndergrowthGenFeature.PROPORTION`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:21)
- [ ] `UndergrowthGenFeature.UndergrowthGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:25)
- [ ] `UndergrowthGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:29)
- [ ] `UndergrowthGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:34)
- [ ] `UndergrowthGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:39)
- [ ] `UndergrowthGenFeature.tryToPlaceTree(5)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:44)
- [ ] `UndergrowthGenFeature.areTreesAround(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:68)
- [ ] `UndergrowthGenFeature.placeTreeAtLocation(3)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/UndergrowthGenFeature.java:81)

### `VinesGenFeature`: it extends `GenFeature`

- [ ] `VinesGenFeature.MAX_LENGTH`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:33)
- [ ] `VinesGenFeature.BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:34)
- [ ] `VinesGenFeature.TIP_BLOCK`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:35)
- [ ] `VinesGenFeature.VINE_TYPE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:36)
- [ ] `VinesGenFeature.VinesGenFeature(1)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:38)
- [ ] `VinesGenFeature.registerProperties(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:42)
- [ ] `VinesGenFeature.createDefaultConfiguration(0)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:47)
- [ ] `VinesGenFeature.postGenerate(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:60)
- [ ] `VinesGenFeature.postGrow(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:84)
- [ ] `VinesGenFeature.addSideVines(6)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:121)
- [ ] `VinesGenFeature.addVerticalVines(6)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:139)
- [ ] `VinesGenFeature.findGround(2)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:162)
- [ ] `VinesGenFeature.placeVines(7)`: its class isn't translated (it extends `GenFeature`) (com/dtteam/dynamictrees/systems/genfeature/VinesGenFeature.java:175)

### `VinesGenFeature.VineType`


### `GrowSignal`: it uses Minecraft's `class_2350`

- [ ] `GrowSignal.GrowSignal(4)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/systems/GrowSignal.java:23)
- [ ] `GrowSignal.GrowSignal(5)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/systems/GrowSignal.java:27)
- [ ] `GrowSignal.getSpecies(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/systems/GrowSignal.java:43)
- [ ] `GrowSignal.step(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/systems/GrowSignal.java:47)
- [ ] `GrowSignal.doTurn(1)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/systems/GrowSignal.java:57)
- [ ] `GrowSignal.isInTrunk(0)`: its class isn't translated (it uses Minecraft's `class_2350`) (com/dtteam/dynamictrees/systems/GrowSignal.java:67)

### `AzaleaLogic`: it extends `GrowthLogicKit`

- [ ] `AzaleaLogic.HORIZONTALS`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/systems/growthlogic/AzaleaLogic.java:14)
- [ ] `AzaleaLogic.AzaleaLogic(1)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/AzaleaLogic.java:18)
- [ ] `AzaleaLogic.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/AzaleaLogic.java:22)
- [ ] `AzaleaLogic.getDirectionBias(1)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/systems/growthlogic/AzaleaLogic.java:64)

### `ConiferLogic`: it extends `GrowthLogicKit`

- [ ] `ConiferLogic.ENERGY_DIVISOR`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:13)
- [ ] `ConiferLogic.HORIZONTAL_LIMITER`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:14)
- [ ] `ConiferLogic.VARIATE_LOWEST_BRANCH`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:15)
- [ ] `ConiferLogic.ConiferLogic(1)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:17)
- [ ] `ConiferLogic.createDefaultConfiguration(0)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:21)
- [ ] `ConiferLogic.registerProperties(0)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:30)
- [ ] `ConiferLogic.selectNewDirection(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:35)
- [ ] `ConiferLogic.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:50)
- [ ] `ConiferLogic.getEnergy(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:65)
- [ ] `ConiferLogic.getLowestBranchHeight(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:71)
- [ ] `ConiferLogic.getHashVariation(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/ConiferLogic.java:79)

### `DirectionManipulationContext`: it extends `DirectionSelectionContext`

- [ ] `DirectionManipulationContext.DirectionManipulationContext(7)`: its class isn't translated (it extends `DirectionSelectionContext`) (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionManipulationContext.java:13)
- [ ] `DirectionManipulationContext.radius(0)`: its class isn't translated (it extends `DirectionSelectionContext`) (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionManipulationContext.java:19)
- [ ] `DirectionManipulationContext.probMap(0)`: its class isn't translated (it extends `DirectionSelectionContext`) (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionManipulationContext.java:23)
- [ ] `DirectionManipulationContext.probMap(1)`: its class isn't translated (it extends `DirectionSelectionContext`) (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionManipulationContext.java:27)

### `DirectionSelectionContext`: it extends `PositionalSpeciesContext`

- [ ] `DirectionSelectionContext.DirectionSelectionContext(5)`: its class isn't translated (it extends `PositionalSpeciesContext`) (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionSelectionContext.java:13)
- [ ] `DirectionSelectionContext.branch(0)`: its class isn't translated (it extends `PositionalSpeciesContext`) (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionSelectionContext.java:19)
- [ ] `DirectionSelectionContext.signal(0)`: its class isn't translated (it extends `PositionalSpeciesContext`) (com/dtteam/dynamictrees/systems/growthlogic/context/DirectionSelectionContext.java:23)

### `PositionalSpeciesContext`: it uses Minecraft's `class_1937`

- [ ] `PositionalSpeciesContext.PositionalSpeciesContext(3)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/systems/growthlogic/context/PositionalSpeciesContext.java:12)
- [ ] `PositionalSpeciesContext.level(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/systems/growthlogic/context/PositionalSpeciesContext.java:18)
- [ ] `PositionalSpeciesContext.pos(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/systems/growthlogic/context/PositionalSpeciesContext.java:22)
- [ ] `PositionalSpeciesContext.species(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/systems/growthlogic/context/PositionalSpeciesContext.java:26)

### `DarkOakLogic`: it extends `GrowthLogicKit`

- [ ] `DarkOakLogic.DarkOakLogic(1)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/DarkOakLogic.java:10)
- [ ] `DarkOakLogic.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/DarkOakLogic.java:14)
- [ ] `DarkOakLogic.getEnergy(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/DarkOakLogic.java:58)
- [ ] `DarkOakLogic.getLowestBranchHeight(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/DarkOakLogic.java:63)

### `GrowthLogicKit`: it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)

- [ ] `GrowthLogicKit.HEIGHT_VARIATION`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:19)
- [ ] `GrowthLogicKit.DEFAULT`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:20)
- [ ] `GrowthLogicKit.REGISTRY`: it uses ConfigurableRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:25)
- [ ] `GrowthLogicKit.GrowthLogicKit(1)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:29)
- [ ] `GrowthLogicKit.createDefaultConfiguration(0)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:33)
- [ ] `GrowthLogicKit.registerProperties(0)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:37)
- [ ] `GrowthLogicKit.selectNewDirection(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:41)
- [ ] `GrowthLogicKit.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:63)
- [ ] `GrowthLogicKit.getEnergy(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:82)
- [ ] `GrowthLogicKit.getLowestBranchHeight(2)`: its class isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:86)
- [ ] `GrowthLogicKit.findGrowthLogicKit(1)`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:90)
- [ ] `GrowthLogicKit.findGrowthLogicKit(1)`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKit.java:94)

### `GrowthLogicKitConfiguration`: it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)

- [ ] `GrowthLogicKitConfiguration.TEMPLATES`: it uses TemplateRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:11)
- [ ] `GrowthLogicKitConfiguration.GrowthLogicKitConfiguration(1)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:13)
- [ ] `GrowthLogicKitConfiguration.copy(0)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:17)
- [ ] `GrowthLogicKitConfiguration.selectNewDirection(1)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:23)
- [ ] `GrowthLogicKitConfiguration.populateDirectionProbabilityMap(1)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:27)
- [ ] `GrowthLogicKitConfiguration.getEnergy(1)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:31)
- [ ] `GrowthLogicKitConfiguration.getLowestBranchHeight(1)`: its class isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:35)
- [ ] `GrowthLogicKitConfiguration.getDefault(0)`: it uses GrowthLogicKitConfiguration, which isn't translated (it extends `Configuration` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKitConfiguration.java:39)

### `GrowthLogicKits`

- [ ] `GrowthLogicKits.DARK_OAK`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:7)
- [ ] `GrowthLogicKits.CONIFER`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:8)
- [ ] `GrowthLogicKits.JUNGLE`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:9)
- [ ] `GrowthLogicKits.AZALEA`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:10)
- [ ] `GrowthLogicKits.NETHER_FUNGUS`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:11)
- [ ] `GrowthLogicKits.PALM`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:12)
- [ ] `GrowthLogicKits.MANGROVE_ROOTS`: it uses GrowthLogicKit, which isn't translated (it extends `ConfigurableRegistryEntry` (com.dtteam.dynamictrees.api.configuration)) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:13)
- [ ] `GrowthLogicKits.register(1)`: it uses Registry, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/growthlogic/GrowthLogicKits.java:15)

### `JungleLogic`: it extends `GrowthLogicKit`

- [ ] `JungleLogic.CANOPY_HEIGHT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:12)
- [ ] `JungleLogic.BRANCH_OUT_CHANCE`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:13)
- [ ] `JungleLogic.JungleLogic(1)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:15)
- [ ] `JungleLogic.createDefaultConfiguration(0)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:19)
- [ ] `JungleLogic.registerProperties(0)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:24)
- [ ] `JungleLogic.selectNewDirection(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:29)
- [ ] `JungleLogic.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:39)
- [ ] `JungleLogic.getEnergy(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/JungleLogic.java:62)

### `MangroveRootsLogic`: it extends `GrowthLogicKit`

- [ ] `MangroveRootsLogic.MangroveRootsLogic(1)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/MangroveRootsLogic.java:14)
- [ ] `MangroveRootsLogic.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/MangroveRootsLogic.java:18)
- [ ] `MangroveRootsLogic.selectNewDirection(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/MangroveRootsLogic.java:54)
- [ ] `MangroveRootsLogic.getLowestBranchHeight(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/MangroveRootsLogic.java:71)
- [ ] `MangroveRootsLogic.getEnergy(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/MangroveRootsLogic.java:76)

### `NetherFungusLogic`: it extends `GrowthLogicKit`

- [ ] `NetherFungusLogic.MIN_CAP_HEIGHT`: it uses ConfigurationProperty, which isn't translated (it's generic) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:15)
- [ ] `NetherFungusLogic.NetherFungusLogic(1)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:17)
- [ ] `NetherFungusLogic.createDefaultConfiguration(0)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:21)
- [ ] `NetherFungusLogic.registerProperties(0)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:26)
- [ ] `NetherFungusLogic.selectNewDirection(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:31)
- [ ] `NetherFungusLogic.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:41)
- [ ] `NetherFungusLogic.getHashedVariation(3)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:64)
- [ ] `NetherFungusLogic.getEnergy(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:70)
- [ ] `NetherFungusLogic.getLowestBranchHeight(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/NetherFungusLogic.java:80)

### `PalmGrowthLogic`: it extends `GrowthLogicKit`

- [ ] `PalmGrowthLogic.PalmGrowthLogic(1)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/PalmGrowthLogic.java:10)
- [ ] `PalmGrowthLogic.populateDirectionProbabilityMap(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/PalmGrowthLogic.java:14)
- [ ] `PalmGrowthLogic.getEnergy(2)`: its class isn't translated (it extends `GrowthLogicKit`) (com/dtteam/dynamictrees/systems/growthlogic/PalmGrowthLogic.java:25)

### `CoderNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `CoderNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CoderNode.java:15)
- [ ] `CoderNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CoderNode.java:39)
- [ ] `CoderNode.compile(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CoderNode.java:44)
- [ ] `CoderNode.nextLink(3)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CoderNode.java:53)

### `CoderNode.Link`: it uses Minecraft's `class_2338`

- [ ] `CoderNode.Link.Link(1)`: its class isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/systems/nodemapper/CoderNode.java:76)

### `CollectorNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `CollectorNode.CollectorNode(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CollectorNode.java:13)
- [ ] `CollectorNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CollectorNode.java:17)
- [ ] `CollectorNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CollectorNode.java:23)
- [ ] `CollectorNode.contains(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/CollectorNode.java:28)

### `DenuderNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `DenuderNode.DenuderNode(2)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/DenuderNode.java:19)
- [ ] `DenuderNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/DenuderNode.java:24)
- [ ] `DenuderNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/DenuderNode.java:40)
- [ ] `DenuderNode.removeSurroundingLeaves(2)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/DenuderNode.java:45)

### `DestroyerNode`: it extends `FindEndsNode`

- [ ] `DestroyerNode.DestroyerNode(1)`: its class isn't translated (it extends `FindEndsNode`) (com/dtteam/dynamictrees/systems/nodemapper/DestroyerNode.java:22)
- [ ] `DestroyerNode.setPlayer(1)`: its class isn't translated (it extends `FindEndsNode`) (com/dtteam/dynamictrees/systems/nodemapper/DestroyerNode.java:26)
- [ ] `DestroyerNode.run(4)`: its class isn't translated (it extends `FindEndsNode`) (com/dtteam/dynamictrees/systems/nodemapper/DestroyerNode.java:31)
- [ ] `DestroyerNode.returnRun(4)`: its class isn't translated (it extends `FindEndsNode`) (com/dtteam/dynamictrees/systems/nodemapper/DestroyerNode.java:51)

### `DiseaseNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `DiseaseNode.DiseaseNode(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/DiseaseNode.java:15)
- [ ] `DiseaseNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/DiseaseNode.java:19)
- [ ] `DiseaseNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/DiseaseNode.java:29)

### `FindEndsNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `FindEndsNode.FindEndsNode(0)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FindEndsNode.java:15)
- [ ] `FindEndsNode.FindEndsNode(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FindEndsNode.java:19)
- [ ] `FindEndsNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FindEndsNode.java:24)
- [ ] `FindEndsNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FindEndsNode.java:29)
- [ ] `FindEndsNode.getEnds(0)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FindEndsNode.java:40)

### `FreezerNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [x] `FreezerNode.freezeRadius` (com/dtteam/dynamictrees/systems/nodemapper/FreezerNode.java:17)
- [ ] `FreezerNode.FreezerNode(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FreezerNode.java:19)
- [ ] `FreezerNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FreezerNode.java:23)
- [ ] `FreezerNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FreezerNode.java:36)
- [ ] `FreezerNode.freezeSurroundingLeaves(3)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/FreezerNode.java:41)

### `InflatorNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `InflatorNode.InflatorNode(2)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/InflatorNode.java:23)
- [ ] `InflatorNode.InflatorNode(3)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/InflatorNode.java:27)
- [ ] `InflatorNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/InflatorNode.java:35)
- [ ] `InflatorNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/InflatorNode.java:48)

### `NetVolumeNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `NetVolumeNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:17)
- [ ] `NetVolumeNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:28)
- [ ] `NetVolumeNode.getVolume(0)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:33)

### `NetVolumeNode.Volume`

- [x] `NetVolumeNode.Volume.VOXELSPERLOG` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:38)
- [ ] `NetVolumeNode.Volume.Volume(0)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:42)
- [ ] `NetVolumeNode.Volume.Volume(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:46)
- [ ] `NetVolumeNode.Volume.Volume(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:51)
- [ ] `NetVolumeNode.Volume.addVolume(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:61)
- [ ] `NetVolumeNode.Volume.addVolume(2)`: Map.computeIfPresent (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:65)
- [ ] `NetVolumeNode.Volume.addVolume(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:77)
- [ ] `NetVolumeNode.Volume.multiplyVolume(1)`: Map.replaceAll (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:83)
- [x] `NetVolumeNode.Volume.getRawVolumesArray(0)` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:87)
- [x] `NetVolumeNode.Volume.getVolume(0)` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:102)
- [x] `NetVolumeNode.Volume.getVolume(1)` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:106)
- [x] `NetVolumeNode.Volume.getRawVolume(0)` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:110)
- [x] `NetVolumeNode.Volume.getRawVolume(1)` (com/dtteam/dynamictrees/systems/nodemapper/NetVolumeNode.java:120)

### `PodGenerationNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `PodGenerationNode.PodGenerationNode(6)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/PodGenerationNode.java:26)
- [ ] `PodGenerationNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/PodGenerationNode.java:37)
- [ ] `PodGenerationNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/PodGenerationNode.java:69)

### `PodGenerationNode.PodPlacer`: it's an interface

- [ ] `PodGenerationNode.PodPlacer.place(5)`: it has no body (com/dtteam/dynamictrees/systems/nodemapper/PodGenerationNode.java:75)

### `RootIntegrityNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `RootIntegrityNode.RootIntegrityNode(0)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/RootIntegrityNode.java:15)
- [ ] `RootIntegrityNode.RootIntegrityNode(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/RootIntegrityNode.java:19)
- [ ] `RootIntegrityNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/RootIntegrityNode.java:23)
- [ ] `RootIntegrityNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/RootIntegrityNode.java:28)
- [ ] `RootIntegrityNode.getStable(0)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/RootIntegrityNode.java:38)

### `RootsDestroyerNode`: it extends `FindEndsNode`

- [ ] `RootsDestroyerNode.RootsDestroyerNode(1)`: its class isn't translated (it extends `FindEndsNode`) (com/dtteam/dynamictrees/systems/nodemapper/RootsDestroyerNode.java:15)
- [ ] `RootsDestroyerNode.run(4)`: its class isn't translated (it extends `FindEndsNode`) (com/dtteam/dynamictrees/systems/nodemapper/RootsDestroyerNode.java:19)
- [ ] `RootsDestroyerNode.returnRun(4)`: its class isn't translated (it extends `FindEndsNode`) (com/dtteam/dynamictrees/systems/nodemapper/RootsDestroyerNode.java:29)

### `ShrinkerNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `ShrinkerNode.ShrinkerNode(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/ShrinkerNode.java:16)
- [ ] `ShrinkerNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/ShrinkerNode.java:20)
- [ ] `ShrinkerNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/ShrinkerNode.java:33)

### `SpeciesNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `SpeciesNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/SpeciesNode.java:15)
- [ ] `SpeciesNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/SpeciesNode.java:31)
- [ ] `SpeciesNode.getSpecies(0)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/SpeciesNode.java:36)

### `StateNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `StateNode.StateNode(1)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/StateNode.java:19)
- [ ] `StateNode.getBranchConnectionMap(0)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/StateNode.java:23)
- [ ] `StateNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/StateNode.java:27)
- [ ] `StateNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/StateNode.java:38)

### `TransformNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [x] `TransformNode.TEST_LEAVES_RADIUS` (com/dtteam/dynamictrees/systems/nodemapper/TransformNode.java:17)
- [ ] `TransformNode.TransformNode(2)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/TransformNode.java:19)
- [ ] `TransformNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/TransformNode.java:24)
- [ ] `TransformNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/TransformNode.java:45)
- [ ] `TransformNode.transformSurroundingLeaves(2)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/TransformNode.java:50)

### `TwinkleNode`: it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)

- [ ] `TwinkleNode.TwinkleNode(2)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/TwinkleNode.java:16)
- [ ] `TwinkleNode.run(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/TwinkleNode.java:21)
- [ ] `TwinkleNode.returnRun(4)`: its class isn't translated (it implements `NodeInspector` (com.dtteam.dynamictrees.api.network)) (com/dtteam/dynamictrees/systems/nodemapper/TwinkleNode.java:32)

### `LevelPoissonDiscProvider`: it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)

- [ ] `LevelPoissonDiscProvider.LevelPoissonDiscProvider(1)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:28)
- [ ] `LevelPoissonDiscProvider.setSeed(1)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:33)
- [ ] `LevelPoissonDiscProvider.setDebug(1)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:41)
- [ ] `LevelPoissonDiscProvider.getPoissonDiscs(3)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:45)
- [ ] `LevelPoissonDiscProvider.generatePoissonDiscs(3)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:65)
- [ ] `LevelPoissonDiscProvider.doesDiscIntersectWith(2)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:207)
- [ ] `LevelPoissonDiscProvider.getChunkDiscSet(2)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:217)
- [ ] `LevelPoissonDiscProvider.getChunkPoissonData(3)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:230)
- [ ] `LevelPoissonDiscProvider.setChunkPoissonData(4)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:235)
- [ ] `LevelPoissonDiscProvider.unloadChunkPoissonData(3)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:240)
- [ ] `LevelPoissonDiscProvider.getChunkPoissonDiscs(2)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:245)
- [ ] `LevelPoissonDiscProvider.getChunkPoissonDiscs(3)`: its class isn't translated (it implements `PoissonDiscProvider` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/systems/poissondisc/LevelPoissonDiscProvider.java:249)

### `PoissonDisc`: it extends `Vec2i`

- [ ] `PoissonDisc.cbm`: a SimpleBitmap with no value (null in Java) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:10)
- [ ] `PoissonDisc.icbm`: a SimpleBitmap with no value (null in Java) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:11)
- [ ] `PoissonDisc.circleBitmapGen(2)`: `--` inside an expression (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:13)
- [x] `PoissonDisc.bitRun(2)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:27)
- [ ] `PoissonDisc.getCircleBitmap(1)`: it uses PoissonDisc.cbm, which isn't translated (a SimpleBitmap with no value (null in Java)) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:31)
- [ ] `PoissonDisc.getCircleBitmap(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:35)
- [ ] `PoissonDisc.getCircleInteriorBitmap(1)`: it uses PoissonDisc.icbm, which isn't translated (a SimpleBitmap with no value (null in Java)) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:39)
- [ ] `PoissonDisc.getCircleInteriorBitmap(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:43)
- [ ] `PoissonDisc.PoissonDisc(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:47)
- [ ] `PoissonDisc.PoissonDisc(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:51)
- [ ] `PoissonDisc.PoissonDisc(4)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:55)
- [ ] `PoissonDisc.PoissonDisc(3)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:60)
- [ ] `PoissonDisc.PoissonDisc(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:64)
- [ ] `PoissonDisc.PoissonDisc(1)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:68)
- [ ] `PoissonDisc.set(3)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:73)
- [ ] `PoissonDisc.set(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:77)
- [ ] `PoissonDisc.setRadius(1)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:82)
- [ ] `PoissonDisc.isInside(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:87)
- [ ] `PoissonDisc.isInterior(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:91)
- [ ] `PoissonDisc.isEdge(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:95)
- [ ] `PoissonDisc.doCirclesIntersect(1)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:99)
- [ ] `PoissonDisc.doCirclesIntersectPadding(1)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:107)
- [ ] `PoissonDisc.maskArc(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:115)
- [ ] `PoissonDisc.fillArc(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:121)
- [ ] `PoissonDisc.setSolved(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:125)
- [ ] `PoissonDisc.clearArc(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:129)
- [ ] `PoissonDisc.edgeMask(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:133)
- [ ] `PoissonDisc.getFreeBitCW(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:163)
- [ ] `PoissonDisc.getFreeBitCCW(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:178)
- [ ] `PoissonDisc.bitToAngle(1)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:193)
- [ ] `PoissonDisc.getFreeAngleCW(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:197)
- [ ] `PoissonDisc.getFreeAngleCCW(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:201)
- [ ] `PoissonDisc.hasFreeAngles(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:205)
- [ ] `PoissonDisc.isSolved(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:209)
- [ ] `PoissonDisc.discPenetration(1)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:213)
- [ ] `PoissonDisc.isInCenterChunk(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:218)
- [ ] `PoissonDisc.toString(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDisc.java:222)

### `PoissonDiscChunkSet`

- [x] `PoissonDiscChunkSet.PoissonDiscChunkSet(0)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:10)
- [ ] `PoissonDiscChunkSet.PoissonDiscChunkSet(1)`: it calls `Arrays` (java.util).copyOf (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:14)
- [ ] `PoissonDiscChunkSet.getDiscs(3)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:19)
- [x] `PoissonDiscChunkSet.clearDiscs(0)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:36)
- [ ] `PoissonDiscChunkSet.addDiscs(1)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:40)
- [ ] `PoissonDiscChunkSet.unpackDiscData(4)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:50)
- [x] `PoissonDiscChunkSet.getRadiusFromDiscData(1)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:57)
- [ ] `PoissonDiscChunkSet.buildDiscData(1)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:61)
- [ ] `PoissonDiscChunkSet.calcTileNum(1)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:65)
- [ ] `PoissonDiscChunkSet.addDisc(1)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:69)
- [x] `PoissonDiscChunkSet.getDiscData(0)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:97)
- [x] `PoissonDiscChunkSet.setDiscData(1)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscChunkSet.java:101)

### `PoissonDiscHelper`

- [x] `PoissonDiscHelper.singleSearchOrder` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:6)
- [x] `PoissonDiscHelper.pairSearchOrder` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:7)
- [ ] `PoissonDiscHelper.findSecondDisc(4)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:9)
- [ ] `PoissonDiscHelper.findSecondDisc(4)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:13)
- [ ] `PoissonDiscHelper.findThirdDisc(3)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:28)
- [ ] `PoissonDiscHelper.maskDiscs(2)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:87)
- [ ] `PoissonDiscHelper.maskDiscs(3)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:91)
- [ ] `PoissonDiscHelper.solveDiscs(2)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:110)
- [ ] `PoissonDiscHelper.gatherUnsolved(2)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:118)
- [ ] `PoissonDiscHelper.fastRemove(2)`: it uses PoissonDisc, which isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscHelper.java:130)

### `PoissonDiscMathHelper`

- [x] `PoissonDiscMathHelper.wrap(2)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscMathHelper.java:4)
- [x] `PoissonDiscMathHelper.wrapAngle(1)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscMathHelper.java:12)
- [x] `PoissonDiscMathHelper.radiansToTurns(1)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscMathHelper.java:18)
- [x] `PoissonDiscMathHelper.deltaAngle(2)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscMathHelper.java:22)

### `PoissonDiscPairData`

- [x] `PoissonDiscPairData.coordTable` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscPairData.java:6)
- [ ] `PoissonDiscPairData.createPairData(5)`: an assignment inside an expression (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscPairData.java:8)
- [x] `PoissonDiscPairData.uncompressCurve(2)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscPairData.java:25)
- [x] `PoissonDiscPairData.PoissonDiscPairData(2)` (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscPairData.java:35)
- [ ] `PoissonDiscPairData.getCoords(1)`: it creates a Vec2i with a constructor that isn't translated (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscPairData.java:42)
- [ ] `PoissonDiscPairData.getSector(1)`: it calls PoissonDiscPairData.getCoords, which isn't translated (com/dtteam/dynamictrees/systems/poissondisc/PoissonDiscPairData.java:46)

### `SimpleBitmap`

- [ ] `SimpleBitmap.SimpleBitmap(2)`: unknown name `class_3532` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:12)
- [ ] `SimpleBitmap.SimpleBitmap(3)`: it calls a constructor that isn't translated (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:19)
- [ ] `SimpleBitmap.SimpleBitmap(1)`: it calls a constructor that isn't translated (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:31)
- [x] `SimpleBitmap.getBits(0)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:37)
- [x] `SimpleBitmap.getHeight(0)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:41)
- [x] `SimpleBitmap.getWidth(0)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:45)
- [ ] `SimpleBitmap.clear(0)`: `this` used as a value (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:49)
- [ ] `SimpleBitmap.isColliding(3)`: `--` inside an expression (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:55)
- [ ] `SimpleBitmap.blitOr(3)`: `--` inside an expression (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:80)
- [ ] `SimpleBitmap.blitSub(3)`: `--` inside an expression (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:99)
- [ ] `SimpleBitmap.blitAnd(3)`: `--` inside an expression (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:118)
- [ ] `SimpleBitmap.prepBlit(3)`: it calls BlitPreparationResult.failure, which isn't translated (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:137)
- [x] `SimpleBitmap.setPixel(3)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:156)
- [x] `SimpleBitmap.isPixelOn(2)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:164)
- [x] `SimpleBitmap.isRowBlank(1)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:168)
- [x] `SimpleBitmap.print(0)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:172)

### `SimpleBitmap.BlitPreparationResult`

- [x] `SimpleBitmap.BlitPreparationResult.FAILURE` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:185)
- [x] `SimpleBitmap.BlitPreparationResult.BlitPreparationResult(4)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:191)
- [ ] `SimpleBitmap.BlitPreparationResult.failure(0)`: it stores a second reference to an object (Rust would need it copied or shared) (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:198)
- [x] `SimpleBitmap.BlitPreparationResult.success(3)` (com/dtteam/dynamictrees/systems/poissondisc/SimpleBitmap.java:202)

### `UniversalPoissonDiscProvider`: it uses Minecraft's `class_2960`

- [x] `UniversalPoissonDiscProvider.CIRCLE_DATA_ID` (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:14)
- [ ] `UniversalPoissonDiscProvider.createCircleProvider(1)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:17)
- [ ] `UniversalPoissonDiscProvider.getProvider(1)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:22)
- [ ] `UniversalPoissonDiscProvider.getPoissonDiscs(2)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:26)
- [ ] `UniversalPoissonDiscProvider.unloadWorld(1)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:31)
- [ ] `UniversalPoissonDiscProvider.setChunkPoissonData(3)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:35)
- [ ] `UniversalPoissonDiscProvider.getChunkPoissonData(2)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:39)
- [ ] `UniversalPoissonDiscProvider.unloadChunkPoissonData(2)`: its class isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/systems/poissondisc/UniversalPoissonDiscProvider.java:43)

### `Vec2i`

- [x] `Vec2i.Vec2i(0)` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:8)
- [ ] `Vec2i.Vec2i(2)`: it calls a constructor that isn't translated (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:11)
- [ ] `Vec2i.Vec2i(3)`: it calls Vec2i.set, which isn't translated (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:15)
- [ ] `Vec2i.Vec2i(1)`: it calls Vec2i.setTight, which isn't translated (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:20)
- [ ] `Vec2i.set(2)`: `this` used as a value (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:25)
- [ ] `Vec2i.set(1)`: it calls Vec2i.set, which isn't translated (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:31)
- [ ] `Vec2i.setTight(1)`: `this` used as a value (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:37)
- [x] `Vec2i.isTight(0)` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:42)
- [ ] `Vec2i.add(2)`: `this` used as a value (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:46)
- [ ] `Vec2i.sub(2)`: `this` used as a value (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:52)
- [ ] `Vec2i.add(1)`: it calls Vec2i.add, which isn't translated (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:58)
- [ ] `Vec2i.sub(1)`: it calls Vec2i.sub, which isn't translated (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:62)
- [x] `Vec2i.len(0)` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:66)
- [x] `Vec2i.angle(0)` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:70)
- [x] `Vec2i.crossProduct(2)` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:74)
- [ ] `Vec2i.equals(1)`: it uses `Object` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:78)
- [x] `Vec2i.hashCode(0)` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:84)
- [x] `Vec2i.toString(0)` (com/dtteam/dynamictrees/systems/poissondisc/Vec2i.java:89)

### `Vec2iPCA`: it extends `Vec2i`

- [ ] `Vec2iPCA.Vec2iPCA(3)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/Vec2iPCA.java:6)
- [ ] `Vec2iPCA.set(2)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/Vec2iPCA.java:10)
- [ ] `Vec2iPCA.angle(0)`: its class isn't translated (it extends `Vec2i`) (com/dtteam/dynamictrees/systems/poissondisc/Vec2iPCA.java:17)

### `ActiveSeasonGrowthCalculator`: it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)

- [ ] `ActiveSeasonGrowthCalculator.clippedSineWave(4)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/ActiveSeasonGrowthCalculator.java:10)
- [ ] `ActiveSeasonGrowthCalculator.peakClimateOffset(1)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/ActiveSeasonGrowthCalculator.java:14)
- [ ] `ActiveSeasonGrowthCalculator.calcGrowthRate(2)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/ActiveSeasonGrowthCalculator.java:20)
- [ ] `ActiveSeasonGrowthCalculator.calcSeedDropRate(2)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/ActiveSeasonGrowthCalculator.java:35)
- [ ] `ActiveSeasonGrowthCalculator.calcFruitProductionRate(2)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/ActiveSeasonGrowthCalculator.java:50)
- [ ] `ActiveSeasonGrowthCalculator.getPeakFruitProductionSeasonValue(1)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/ActiveSeasonGrowthCalculator.java:65)

### `ClimateHelper`

- [x] `ClimateHelper.CLIMATE_MULTIPLIER_TABLE` (com/dtteam/dynamictrees/systems/season/ClimateHelper.java:9)
- [ ] `ClimateHelper.climateMultiplier(3)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/systems/season/ClimateHelper.java:13)
- [ ] `ClimateHelper.climateMultiplier(3)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/systems/season/ClimateHelper.java:18)
- [ ] `ClimateHelper.getClimate(2)`: it uses ClimateZoneType, which isn't translated (its constants carry values) (com/dtteam/dynamictrees/systems/season/ClimateHelper.java:22)

### `NormalSeasonManager`: it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)

- [ ] `NormalSeasonManager.NULL`: it uses SeasonManager, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:20)
- [x] `NormalSeasonManager.ARID_THRESHHOLD` (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:25)
- [x] `NormalSeasonManager.ARID_DOWNFALL_THRESHHOLD` (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:26)
- [x] `NormalSeasonManager.TROPICAL_THRESHHOLD` (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:27)
- [x] `NormalSeasonManager.COLD_THRESHHOLD` (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:28)
- [ ] `NormalSeasonManager.NormalSeasonManager(0)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:36)
- [ ] `NormalSeasonManager.NormalSeasonManager(1)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:39)
- [ ] `NormalSeasonManager.createProvider(1)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:43)
- [ ] `NormalSeasonManager.getContext(1)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:47)
- [ ] `NormalSeasonManager.flushMappings(0)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:54)
- [ ] `NormalSeasonManager.getBiomeForDist(2)`: it uses Minecraft's `class_1959` (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:59)
- [ ] `NormalSeasonManager.setTropicalPredicate(1)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:65)
- [ ] `NormalSeasonManager.setAridPredicate(1)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:69)
- [ ] `NormalSeasonManager.setColdPredicate(1)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:73)
- [ ] `NormalSeasonManager.getClimate(2)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:77)
- [ ] `NormalSeasonManager.updateTick(2)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:88)
- [ ] `NormalSeasonManager.getGrowthFactor(3)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:93)
- [ ] `NormalSeasonManager.getSeedDropFactor(3)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:100)
- [ ] `NormalSeasonManager.getFruitProductionFactor(3)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:107)
- [ ] `NormalSeasonManager.getSeasonValue(2)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:114)
- [ ] `NormalSeasonManager.getPeakFruitProductionSeasonValue(3)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:119)
- [ ] `NormalSeasonManager.shouldSnowMelt(2)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:126)
- [ ] `NormalSeasonManager.clearCache(1)`: its class isn't translated (it implements `SeasonManager` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NormalSeasonManager.java:131)

### `NullSeasonGrowthCalculator`: it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)

- [ ] `NullSeasonGrowthCalculator.calcGrowthRate(2)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NullSeasonGrowthCalculator.java:7)
- [ ] `NullSeasonGrowthCalculator.calcSeedDropRate(2)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NullSeasonGrowthCalculator.java:12)
- [ ] `NullSeasonGrowthCalculator.calcFruitProductionRate(2)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NullSeasonGrowthCalculator.java:17)
- [ ] `NullSeasonGrowthCalculator.getPeakFruitProductionSeasonValue(1)`: its class isn't translated (it implements `SeasonGrowthCalculator` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NullSeasonGrowthCalculator.java:22)

### `NullSeasonProvider`: it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)

- [ ] `NullSeasonProvider.getSeasonValue(2)`: its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NullSeasonProvider.java:8)
- [ ] `NullSeasonProvider.updateTick(2)`: its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NullSeasonProvider.java:13)
- [ ] `NullSeasonProvider.shouldSnowMelt(2)`: its class isn't translated (it implements `SeasonProvider` (com.dtteam.dynamictrees.api.season)) (com/dtteam/dynamictrees/systems/season/NullSeasonProvider.java:17)

### `SeasonCompatibilityHandler`

- [ ] `SeasonCompatibilityHandler.SEASON_MANAGERS`: it uses SeasonManager, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:14)
- [ ] `SeasonCompatibilityHandler.activeSeasonManager`: it uses SeasonManager, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:15)
- [x] `SeasonCompatibilityHandler.DISABLED` (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:16)
- [x] `SeasonCompatibilityHandler.ANY` (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:17)
- [ ] `SeasonCompatibilityHandler.getSeasonManager(0)`: it uses SeasonManager, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:19)
- [ ] `SeasonCompatibilityHandler.setSeasonManager(1)`: it uses SeasonManager, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:23)
- [ ] `SeasonCompatibilityHandler.registerSeasonManager(2)`: it uses SeasonManager, which isn't translated (it's an interface) (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:27)
- [ ] `SeasonCompatibilityHandler.registerBuiltInSeasonManagers(0)`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:31)
- [ ] `SeasonCompatibilityHandler.reloadSeasonManager(0)`: it uses DTConfigs.COMMON, which isn't translated (it uses DTConfigs, which isn't translated (it uses `DoubleValue` (net.neoforged.neoforge.common.ModConfigSpec))) (com/dtteam/dynamictrees/systems/season/SeasonCompatibilityHandler.java:37)

### `SeasonContext`: it uses SeasonProvider, which isn't translated (it's an interface)

- [x] `SeasonContext.updateRate` (com/dtteam/dynamictrees/systems/season/SeasonContext.java:19)
- [x] `SeasonContext.seasonValueCacheMultiplier` (com/dtteam/dynamictrees/systems/season/SeasonContext.java:20)
- [ ] `SeasonContext.SeasonContext(2)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:22)
- [ ] `SeasonContext.updateTick(2)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:31)
- [ ] `SeasonContext.getSeasonProvider(0)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:40)
- [ ] `SeasonContext.getCalculator(0)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:44)
- [ ] `SeasonContext.getGrowthFactor(2)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:48)
- [ ] `SeasonContext.getSeedDropFactor(2)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:64)
- [ ] `SeasonContext.getFruitProductionFactor(2)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:80)
- [ ] `SeasonContext.getPeakFruitProductionSeasonValue(2)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:96)
- [ ] `SeasonContext.clearCache(0)`: its class isn't translated (it uses SeasonProvider, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/systems/season/SeasonContext.java:100)

### `SeasonHelper`

- [x] `SeasonHelper.SPRING_START` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:9)
- [x] `SeasonHelper.SPRING_MIDDLE` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:10)
- [x] `SeasonHelper.SUMMER_START` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:11)
- [x] `SeasonHelper.SUMMER_MIDDLE` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:12)
- [x] `SeasonHelper.AUTUMN_START` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:13)
- [x] `SeasonHelper.AUTUMN_MIDDLE` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:14)
- [x] `SeasonHelper.WINTER_START` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:15)
- [x] `SeasonHelper.WINTER_MIDDLE` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:16)
- [ ] `SeasonHelper.updateTick(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:18)
- [ ] `SeasonHelper.globalSeasonalGrowthFactor(3)`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:22)
- [ ] `SeasonHelper.globalSeasonalSeedDropFactor(3)`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:28)
- [ ] `SeasonHelper.globalSeasonalFruitProductionFactor(3)`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:34)
- [ ] `SeasonHelper.getPeakFruitProductionSeason(3)`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:40)
- [ ] `SeasonHelper.getSeasonValue(2)`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`) (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:46)
- [x] `SeasonHelper.isSeasonBetween(3)` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:50)
- [ ] `SeasonHelper.shouldSnowMelt(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/systems/season/SeasonHelper.java:57)

### `SeedSaplingRecipe`: it uses Minecraft's `class_1792`

- [ ] `SeedSaplingRecipe.CODEC`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:16)
- [ ] `SeedSaplingRecipe.SeedSaplingRecipe(2)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:41)
- [ ] `SeedSaplingRecipe.SeedSaplingRecipe(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:46)
- [ ] `SeedSaplingRecipe.addExtraIngredientForSaplingToSeed(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:50)
- [ ] `SeedSaplingRecipe.addExtraIngredientForSeedToSapling(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:54)
- [ ] `SeedSaplingRecipe.setCanCraftSaplingToSeed(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:58)
- [ ] `SeedSaplingRecipe.setCanCraftSeedToSapling(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:62)
- [ ] `SeedSaplingRecipe.setReplaceSaplingWhenPlaced(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:66)
- [ ] `SeedSaplingRecipe.setReplaceSaplingWhenGrown(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:70)
- [ ] `SeedSaplingRecipe.getSaplingItem(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:74)
- [ ] `SeedSaplingRecipe.setSaplingItem(1)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:78)
- [ ] `SeedSaplingRecipe.isValid(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:83)
- [ ] `SeedSaplingRecipe.getSaplingBlock(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:87)
- [ ] `SeedSaplingRecipe.getIngredientsForSaplingToSeed(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:92)
- [ ] `SeedSaplingRecipe.getIngredientsForSeedToSapling(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:97)
- [ ] `SeedSaplingRecipe.canCraftSaplingToSeed(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:102)
- [ ] `SeedSaplingRecipe.canCraftSeedToSapling(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:106)
- [ ] `SeedSaplingRecipe.shouldReplaceSaplingWhenPlaced(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:110)
- [ ] `SeedSaplingRecipe.shouldReplaceSaplingWhenGrown(0)`: its class isn't translated (it uses Minecraft's `class_1792`) (com/dtteam/dynamictrees/systems/SeedSaplingRecipe.java:114)

### `DepleteSubstance`: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)

- [ ] `DepleteSubstance.apply(2)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/DepleteSubstance.java:13)
- [ ] `DepleteSubstance.getName(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/DepleteSubstance.java:24)
- [ ] `DepleteSubstance.setAmount(1)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/DepleteSubstance.java:29)
- [ ] `DepleteSubstance.isLingering(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/DepleteSubstance.java:34)

### `FertilizeSubstance`: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)

- [ ] `FertilizeSubstance.apply(2)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:17)
- [ ] `FertilizeSubstance.getName(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:37)
- [ ] `FertilizeSubstance.setAmount(1)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:42)
- [ ] `FertilizeSubstance.setGrow(1)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:47)
- [ ] `FertilizeSubstance.setPulses(1)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:52)
- [ ] `FertilizeSubstance.setDisplayParticles(1)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:56)
- [ ] `FertilizeSubstance.setPulses(1)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:61)
- [ ] `FertilizeSubstance.isLingering(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/FertilizeSubstance.java:66)

### `GrowthSubstance`: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)

- [ ] `GrowthSubstance.GrowthSubstance(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java:18)
- [ ] `GrowthSubstance.GrowthSubstance(2)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java:22)
- [ ] `GrowthSubstance.GrowthSubstance(3)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java:26)
- [ ] `GrowthSubstance.apply(2)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java:32)
- [ ] `GrowthSubstance.update(4)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java:46)
- [ ] `GrowthSubstance.getName(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java:64)
- [ ] `GrowthSubstance.isLingering(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/GrowthSubstance.java:69)

### `HarvestSubstance`: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)

- [ ] `HarvestSubstance.HarvestSubstance(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:34)
- [ ] `HarvestSubstance.HarvestSubstance(5)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:38)
- [ ] `HarvestSubstance.apply(2)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:46)
- [ ] `HarvestSubstance.recalculateFruitPositions(3)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:63)
- [ ] `HarvestSubstance.isCompatibleFruitBlock(2)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:74)
- [ ] `HarvestSubstance.isCompatibleFruitBlock(1)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:79)
- [ ] `HarvestSubstance.update(4)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:83)
- [ ] `HarvestSubstance.getName(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:144)
- [ ] `HarvestSubstance.isLingering(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/HarvestSubstance.java:149)

### `LingeringSubstances`

- [ ] `LingeringSubstances.LINGERING_SUBSTANCES`: it uses `Class` (com/dtteam/dynamictrees/systems/substance/LingeringSubstances.java:9)
- [ ] `LingeringSubstances.registerLingeringSubstance(2)`: it's generic (com/dtteam/dynamictrees/systems/substance/LingeringSubstances.java:11)
- [ ] `LingeringSubstances.indexOf(1)`: it's generic (com/dtteam/dynamictrees/systems/substance/LingeringSubstances.java:15)
- [ ] `LingeringSubstances.fromIndex(1)`: it's generic (com/dtteam/dynamictrees/systems/substance/LingeringSubstances.java:19)

### `MegaSubstance`: it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)

- [ ] `MegaSubstance.apply(2)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/MegaSubstance.java:14)
- [ ] `MegaSubstance.getName(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/MegaSubstance.java:35)
- [ ] `MegaSubstance.isLingering(0)`: its class isn't translated (it implements `SubstanceEffect` (com.dtteam.dynamictrees.api.substance)) (com/dtteam/dynamictrees/systems/substance/MegaSubstance.java:40)

### `FabricModFileContainer`: it extends `ModFileContainer`

- [ ] `FabricModFileContainer.FabricModFileContainer(1)`: its class isn't translated (it extends `ModFileContainer`) (com/dtteam/dynamictrees/treepack/FabricModFileContainer.java:11)
- [ ] `FabricModFileContainer.findResource(1)`: its class isn't translated (it extends `ModFileContainer`) (com/dtteam/dynamictrees/treepack/FabricModFileContainer.java:15)
- [ ] `FabricModFileContainer.getModId(0)`: its class isn't translated (it extends `ModFileContainer`) (com/dtteam/dynamictrees/treepack/FabricModFileContainer.java:27)

### `BiomePopulatorsResourceLoader`: it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)

- [ ] `BiomePopulatorsResourceLoader.RESOURCE_PREPARER`: it uses MultiJsonResourcePreparer, which isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:31)
- [x] `BiomePopulatorsResourceLoader.LOGGER` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:32)
- [x] `BiomePopulatorsResourceLoader.DEFAULT_POPULATOR` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:33)
- [x] `BiomePopulatorsResourceLoader.SELECT` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:34)
- [x] `BiomePopulatorsResourceLoader.METHOD` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:35)
- [x] `BiomePopulatorsResourceLoader.APPLY` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:36)
- [x] `BiomePopulatorsResourceLoader.WHITE` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:37)
- [x] `BiomePopulatorsResourceLoader.CAVE_ROOTED` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:38)
- [x] `BiomePopulatorsResourceLoader.ENTRY_APPLIERS` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:39)
- [ ] `BiomePopulatorsResourceLoader.toLinkedList(1)`: it uses `LinkedList` (java.util) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:43)
- [ ] `BiomePopulatorsResourceLoader.collectBiomes(2)`: it uses IDTBiomeHolderSet, which isn't translated (it's an interface) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:47)
- [x] `BiomePopulatorsResourceLoader.warnNoBiomesSelected(1)` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:54)
- [x] `BiomePopulatorsResourceLoader.noBiomesSelectedWarningNotSuppressed(1)` (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:60)
- [ ] `BiomePopulatorsResourceLoader.BiomePopulatorsResourceLoader(0)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:65)
- [ ] `BiomePopulatorsResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:69)
- [ ] `BiomePopulatorsResourceLoader.applySpecies(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:89)
- [ ] `BiomePopulatorsResourceLoader.applyDensity(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:97)
- [ ] `BiomePopulatorsResourceLoader.applyChance(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:105)
- [ ] `BiomePopulatorsResourceLoader.applyMultipass(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:113)
- [ ] `BiomePopulatorsResourceLoader.applyCaveRootedSpecies(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:119)
- [ ] `BiomePopulatorsResourceLoader.applyCaveRootedChance(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:127)
- [ ] `BiomePopulatorsResourceLoader.getOperationOrWarn(1)`: it calls BiomePopulatorsResourceLoader.getOperation, which isn't translated (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:135)
- [ ] `BiomePopulatorsResourceLoader.getOperation(1)`: it uses Result, which isn't translated (it's an interface) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:139)
- [ ] `BiomePopulatorsResourceLoader.readCaveRootedPopulatorSection(3)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:151)
- [ ] `BiomePopulatorsResourceLoader.applyOnReload(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:158)
- [ ] `BiomePopulatorsResourceLoader.readPopulators(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:167)
- [ ] `BiomePopulatorsResourceLoader.readModPopulators(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:172)
- [ ] `BiomePopulatorsResourceLoader.readTreePackPopulators(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:177)
- [ ] `BiomePopulatorsResourceLoader.readDimensionalPopulators(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:184)
- [ ] `BiomePopulatorsResourceLoader.readDimensionalModPopulators(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:189)
- [ ] `BiomePopulatorsResourceLoader.readDimensionalTreePackPopulators(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:194)
- [ ] `BiomePopulatorsResourceLoader.readDimensionalPopulator(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:199)
- [ ] `BiomePopulatorsResourceLoader.readPopulator(3)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:203)
- [ ] `BiomePopulatorsResourceLoader.readPopulatorSection(3)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:220)
- [ ] `BiomePopulatorsResourceLoader.applyCaveRootedPopulatorSection(3)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:238)
- [ ] `BiomePopulatorsResourceLoader.applyWhite(4)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:247)
- [ ] `BiomePopulatorsResourceLoader.isDefaultPopulator(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/BiomePopulatorsResourceLoader.java:259)

### `FamilyResourceLoader`: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)

- [x] `FamilyResourceLoader.LOGGER` (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:21)
- [ ] `FamilyResourceLoader.FamilyResourceLoader(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:23)
- [ ] `FamilyResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:27)
- [ ] `FamilyResourceLoader.registerMangroveAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:62)
- [ ] `FamilyResourceLoader.setCommonWarn(2)`: it uses Family, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:77)
- [ ] `FamilyResourceLoader.applyLoadAppliers(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:81)
- [ ] `FamilyResourceLoader.setBranchProperties(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:87)
- [ ] `FamilyResourceLoader.postLoadOnLoad(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:98)
- [ ] `FamilyResourceLoader.addAcceptableSoilForRootSystem(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FamilyResourceLoader.java:104)

### `FeatureCancellationResourceLoader`: it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)

- [ ] `FeatureCancellationResourceLoader.RESOURCE_PREPARER`: it uses MultiJsonResourcePreparer, which isn't translated (it extends `AbstractResourcePreparer`) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:32)
- [x] `FeatureCancellationResourceLoader.LOGGER` (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:33)
- [x] `FeatureCancellationResourceLoader.CANCELLATION_FILE` (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:34)
- [x] `FeatureCancellationResourceLoader.CANCELLATION_APPLIERS` (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:35)
- [x] `FeatureCancellationResourceLoader.CANCELLERS` (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:36)
- [ ] `FeatureCancellationResourceLoader.isCancellationFile(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:41)
- [ ] `FeatureCancellationResourceLoader.FeatureCancellationResourceLoader(0)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:45)
- [ ] `FeatureCancellationResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:49)
- [ ] `FeatureCancellationResourceLoader.applyOnReload(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:61)
- [ ] `FeatureCancellationResourceLoader.readCancellers(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:67)
- [ ] `FeatureCancellationResourceLoader.readModCancellers(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:72)
- [ ] `FeatureCancellationResourceLoader.readTreePackCancellers(1)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:77)
- [ ] `FeatureCancellationResourceLoader.readCancellers(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:82)
- [ ] `FeatureCancellationResourceLoader.readCancellersInSection(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:99)
- [ ] `FeatureCancellationResourceLoader.applyCanceller(5)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:117)
- [ ] `FeatureCancellationResourceLoader.applyCancellationAppliers(3)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/FeatureCancellationResourceLoader.java:131)

### `FruitResourceLoader`: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)

- [ ] `FruitResourceLoader.FruitResourceLoader(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java:19)
- [ ] `FruitResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java:23)
- [ ] `FruitResourceLoader.applyLoadAppliers(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java:41)
- [ ] `FruitResourceLoader.getBlockPropertiesJson(1)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java:52)
- [ ] `FruitResourceLoader.createBlock(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java:57)
- [ ] `FruitResourceLoader.createBlock(3)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java:61)
- [ ] `FruitResourceLoader.getBlockRegistryName(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/FruitResourceLoader.java:71)

### `JoCodeResourceLoader`: it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)

- [x] `JoCodeResourceLoader.LOGGER` (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java:19)
- [ ] `JoCodeResourceLoader.JoCodeResourceLoader(0)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java:21)
- [ ] `JoCodeResourceLoader.applyOnReload(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java:25)
- [ ] `JoCodeResourceLoader.registerCodes(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java:31)
- [ ] `JoCodeResourceLoader.registerCodeForLine(2)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java:37)
- [ ] `JoCodeResourceLoader.registerCode(4)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java:48)
- [ ] `JoCodeResourceLoader.collectWorldGenCodes(3)`: its class isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourceLoader.java:59)

### `JoCodeResourcePreparer`: it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)

- [ ] `JoCodeResourcePreparer.JoCodeResourcePreparer(1)`: its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourcePreparer.java:7)
- [ ] `JoCodeResourcePreparer.offerLine(2)`: its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourcePreparer.java:11)
- [ ] `JoCodeResourcePreparer.shouldAddLine(1)`: its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourcePreparer.java:18)
- [ ] `JoCodeResourcePreparer.processLine(1)`: its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourcePreparer.java:22)
- [ ] `JoCodeResourcePreparer.withoutWhitespace(1)`: its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourcePreparer.java:27)
- [ ] `JoCodeResourcePreparer.withoutTrailingComments(1)`: its class isn't translated (it extends `TextResourcePreparer` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/JoCodeResourcePreparer.java:31)

### `LeavesPropertiesResourceLoader`: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)

- [ ] `LeavesPropertiesResourceLoader.LeavesPropertiesResourceLoader(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java:22)
- [ ] `LeavesPropertiesResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java:26)
- [ ] `LeavesPropertiesResourceLoader.readDoesAge(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java:81)
- [ ] `LeavesPropertiesResourceLoader.applyLoadAppliers(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java:102)
- [ ] `LeavesPropertiesResourceLoader.readCustomBlockRegistryName(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java:113)
- [ ] `LeavesPropertiesResourceLoader.shouldGenerateBlocks(1)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java:128)
- [ ] `LeavesPropertiesResourceLoader.generateBlocks(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/LeavesPropertiesResourceLoader.java:132)

### `PodResourceLoader`: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)

- [ ] `PodResourceLoader.PodResourceLoader(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:24)
- [ ] `PodResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:28)
- [ ] `PodResourceLoader.readBlockShapes(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:49)
- [ ] `PodResourceLoader.applyLoadAppliers(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:60)
- [ ] `PodResourceLoader.getBlockPropertiesJson(1)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:71)
- [ ] `PodResourceLoader.createBlock(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:76)
- [ ] `PodResourceLoader.createBlock(3)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:80)
- [ ] `PodResourceLoader.getBlockRegistryName(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/PodResourceLoader.java:90)

### `SoilPropertiesResourceLoader`: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)

- [x] `SoilPropertiesResourceLoader.LOGGER` (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:20)
- [ ] `SoilPropertiesResourceLoader.SoilPropertiesResourceLoader(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:22)
- [ ] `SoilPropertiesResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:26)
- [ ] `SoilPropertiesResourceLoader.setSubstituteWarn(2)`: it uses SoilProperties, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:57)
- [ ] `SoilPropertiesResourceLoader.registerAcceptableSoil(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:61)
- [ ] `SoilPropertiesResourceLoader.registerSpreadableAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:69)
- [ ] `SoilPropertiesResourceLoader.applyLoadAppliers(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:81)
- [ ] `SoilPropertiesResourceLoader.readCustomBlockRegistryName(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:87)
- [ ] `SoilPropertiesResourceLoader.postLoadOnLoad(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:102)
- [ ] `SoilPropertiesResourceLoader.generateSoilBlock(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SoilPropertiesResourceLoader.java:111)

### `SpeciesResourceLoader`: it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)

- [x] `SpeciesResourceLoader.LOGGER` (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:38)
- [ ] `SpeciesResourceLoader.SpeciesResourceLoader(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:42)
- [ ] `SpeciesResourceLoader.registerAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:46)
- [ ] `SpeciesResourceLoader.registerMangroveAppliers(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:142)
- [ ] `SpeciesResourceLoader.setSeed(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:153)
- [ ] `SpeciesResourceLoader.applySeasonalOffsets(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:167)
- [ ] `SpeciesResourceLoader.setMegaSpecies(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:176)
- [ ] `SpeciesResourceLoader.addAcceptableSoil(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:189)
- [ ] `SpeciesResourceLoader.addAcceptableSoilForWorldGen(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:193)
- [ ] `SpeciesResourceLoader.postLoadOnLoad(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:197)
- [ ] `SpeciesResourceLoader.postLoadOnReload(2)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:203)
- [ ] `SpeciesResourceLoader.registerComposterChances(0)`: its class isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/loader/SpeciesResourceLoader.java:218)

### `ModFileContainer`

- [ ] `ModFileContainer.findResource(1)`: it has no body (com/dtteam/dynamictrees/treepack/ModFileContainer.java:7)
- [ ] `ModFileContainer.getModId(0)`: it has no body (com/dtteam/dynamictrees/treepack/ModFileContainer.java:9)

### `Resettable`: it's an interface

- [ ] `Resettable.reset(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/treepack/Resettable.java:4)
- [ ] `Resettable.setPreReloadDefaults(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/treepack/Resettable.java:8)
- [ ] `Resettable.setPostReloadDefaults(0)`: its class isn't translated (it's an interface) (com/dtteam/dynamictrees/treepack/Resettable.java:12)

### `Resources`

- [x] `Resources.TREES` (com/dtteam/dynamictrees/treepack/Resources.java:36)
- [ ] `Resources.MANAGER`: it uses TreeResourceManager, which isn't translated (it's an interface) (com/dtteam/dynamictrees/treepack/Resources.java:37)
- [ ] `Resources.LEAVES_PROPERTIES_LOADER`: it uses LeavesPropertiesResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/Resources.java:38)
- [ ] `Resources.SOIL_PROPERTIES_LOADER`: it uses SoilPropertiesResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/Resources.java:39)
- [ ] `Resources.FAMILY_LOADER`: it uses FamilyResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/Resources.java:40)
- [ ] `Resources.GEN_FEATURE_TEMPLATE_LOADER`: it uses ConfigurationTemplateResourceLoader, which isn't translated (it's generic) (com/dtteam/dynamictrees/treepack/Resources.java:41)
- [ ] `Resources.GROWTH_LOGIC_KIT_TEMPLATE_LOADER`: it uses ConfigurationTemplateResourceLoader, which isn't translated (it's generic) (com/dtteam/dynamictrees/treepack/Resources.java:44)
- [ ] `Resources.FRUIT_LOADER`: it uses FruitResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/Resources.java:47)
- [ ] `Resources.POD_LOADER`: it uses PodResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/Resources.java:48)
- [ ] `Resources.SPECIES_LOADER`: it uses SpeciesResourceLoader, which isn't translated (it extends `JsonRegistryResourceLoader` (com.dtteam.dynamictrees.api.resource.loading.preparation)) (com/dtteam/dynamictrees/treepack/Resources.java:49)
- [ ] `Resources.JO_CODE_LOADER`: it uses JoCodeResourceLoader, which isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/Resources.java:50)
- [ ] `Resources.FEATURE_CANCELLATION_LOADER`: it uses FeatureCancellationResourceLoader, which isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/Resources.java:51)
- [ ] `Resources.BIOME_POPULATORS_LOADER`: it uses BiomePopulatorsResourceLoader, which isn't translated (it extends `AbstractResourceLoader` (com.dtteam.dynamictrees.api.resource.loading)) (com/dtteam/dynamictrees/treepack/Resources.java:52)
- [ ] `Resources.FLAT_TREE_PACK_INFO`: it uses Minecraft's `class_9224` (com/dtteam/dynamictrees/treepack/Resources.java:53)
- [ ] `Resources.setupTreesResourceManager(0)`: it uses Services.EVENT, which isn't translated (it uses IEventHelper, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/treepack/Resources.java:57)
- [ ] `Resources.addDefaultLoaders(0)`: it uses Resources.MANAGER, which isn't translated (it uses TreeResourceManager, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/treepack/Resources.java:67)
- [ ] `Resources.registerModTreePacks(0)`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/treepack/Resources.java:83)
- [ ] `Resources.addModTreePack(1)`: it calls ModFileContainer.findResource, which isn't translated (com/dtteam/dynamictrees/treepack/Resources.java:87)
- [ ] `Resources.registerFlatTreePack(0)`: it uses `File` (java.io) (com/dtteam/dynamictrees/treepack/Resources.java:107)
- [ ] `Resources.getTreeFolder(0)`: it uses `File` (java.io) (com/dtteam/dynamictrees/treepack/Resources.java:112)

### `Resources.ReloadListener`: it implements Minecraft's `class_3302`

- [ ] `Resources.ReloadListener.ReloadListener(1)`: its class isn't translated (it implements Minecraft's `class_3302`) (com/dtteam/dynamictrees/treepack/Resources.java:124)
- [ ] `Resources.ReloadListener.method_25931(6)`: its class isn't translated (it implements Minecraft's `class_3302`) (com/dtteam/dynamictrees/treepack/Resources.java:128)

### `TreePackResources`: it extends Minecraft's `class_3259`

- [x] `TreePackResources.LOGGER` (com/dtteam/dynamictrees/treepack/TreePackResources.java:23)
- [ ] `TreePackResources.TreePackResources(2)`: its class isn't translated (it extends Minecraft's `class_3259`) (com/dtteam/dynamictrees/treepack/TreePackResources.java:26)
- [ ] `TreePackResources.method_14405(2)`: its class isn't translated (it extends Minecraft's `class_3259`) (com/dtteam/dynamictrees/treepack/TreePackResources.java:31)
- [ ] `TreePackResources.getPathFromLocation(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/treepack/TreePackResources.java:35)
- [ ] `TreePackResources.method_14408(4)`: its class isn't translated (it extends Minecraft's `class_3259`) (com/dtteam/dynamictrees/treepack/TreePackResources.java:43)
- [ ] `TreePackResources.method_14406(1)`: its class isn't translated (it extends Minecraft's `class_3259`) (com/dtteam/dynamictrees/treepack/TreePackResources.java:49)

### `TreesResourceManager`: it implements Minecraft's `class_3300`

- [ ] `TreesResourceManager.addLoader(1)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:31)
- [ ] `TreesResourceManager.addLoaders(1)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:36)
- [ ] `TreesResourceManager.addLoaderBefore(2)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:41)
- [ ] `TreesResourceManager.addLoaderAfter(2)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:46)
- [ ] `TreesResourceManager.registerAppliers(0)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:51)
- [ ] `TreesResourceManager.load(0)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:60)
- [ ] `TreesResourceManager.gatherData(0)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:65)
- [ ] `TreesResourceManager.setup(0)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:70)
- [ ] `TreesResourceManager.prepareReload(2)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:75)
- [ ] `TreesResourceManager.reload(1)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:80)
- [ ] `TreesResourceManager.reload(2)`: it's generic (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:87)
- [ ] `TreesResourceManager.addPack(1)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:91)
- [ ] `TreesResourceManager.method_14487(0)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:96)
- [ ] `TreesResourceManager.method_14486(1)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:100)
- [ ] `TreesResourceManager.getResourceOrThrow(1)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:105)
- [ ] `TreesResourceManager.method_14489(1)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:109)
- [ ] `TreesResourceManager.getResource(2)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:117)
- [ ] `TreesResourceManager.method_14488(2)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:121)
- [ ] `TreesResourceManager.method_41265(2)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:137)
- [ ] `TreesResourceManager.method_29213(0)`: its class isn't translated (it implements Minecraft's `class_3300`) (com/dtteam/dynamictrees/treepack/TreesResourceManager.java:153)

### `ChunkTreeHelper`

- [x] `ChunkTreeHelper.CHUNK_WIDTH` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:44)
- [x] `ChunkTreeHelper.NONE` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:45)
- [x] `ChunkTreeHelper.TREE` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:46)
- [x] `ChunkTreeHelper.SURR` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:47)
- [ ] `ChunkTreeHelper.removeOrphanedBranchNodes(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:49)
- [ ] `ChunkTreeHelper.removeAllBranchesFromChunk(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:93)
- [ ] `ChunkTreeHelper.getEffectiveBlockBounds(3)`: it uses BlockPosBounds, which isn't translated (it implements `Iterable`) (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:112)
- [ ] `ChunkTreeHelper.getTopFilledSegment(1)`: it uses Minecraft's `class_2818` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:124)
- [ ] `ChunkTreeHelper.doTreeDestroy(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:128)
- [ ] `ChunkTreeHelper.cleanupNeighbors(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:135)
- [ ] `ChunkTreeHelper.cleanupBlock(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:155)
- [ ] `ChunkTreeHelper.cleanupVines(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:167)
- [ ] `ChunkTreeHelper.canCheckSurroundings(2)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:176)
- [ ] `ChunkTreeHelper.canCheckSurroundings(3)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:180)
- [ ] `ChunkTreeHelper.isSurroundedByLoadedChunks(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:184)
- [ ] `ChunkTreeHelper.canAccessStateSafely(2)`: it uses Minecraft's `class_1922` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:195)
- [ ] `ChunkTreeHelper.getStateSafe(2)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/tree/ChunkTreeHelper.java:203)

### `Family`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `Family.blockStateGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/tree/family/Family.java:74)
- [ ] `Family.itemModelGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/tree/family/Family.java:75)
- [ ] `Family.languageGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/tree/family/Family.java:76)
- [ ] `Family.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/family/Family.java:77)
- [ ] `Family.NULL_FAMILY`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/tree/family/Family.java:78)
- [ ] `Family.REGISTRY`: it uses TypedRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/family/Family.java:108)
- [x] `Family.BRANCH` (com/dtteam/dynamictrees/tree/family/Family.java:153)
- [x] `Family.BRANCH_TOP` (com/dtteam/dynamictrees/tree/family/Family.java:154)
- [x] `Family.STRIPPED_BRANCH` (com/dtteam/dynamictrees/tree/family/Family.java:155)
- [x] `Family.STRIPPED_BRANCH_TOP` (com/dtteam/dynamictrees/tree/family/Family.java:156)
- [x] `Family.ROOTS_SIDE` (com/dtteam/dynamictrees/tree/family/Family.java:157)
- [x] `Family.ROOTS_TOP` (com/dtteam/dynamictrees/tree/family/Family.java:158)
- [x] `Family.COVERED_ROOTS_BLOCK` (com/dtteam/dynamictrees/tree/family/Family.java:159)
- [ ] `Family.Family(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:161)
- [ ] `Family.Family(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:165)
- [ ] `Family.setupBlocks(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:170)
- [ ] `Family.setCommonSpecies(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:182)
- [ ] `Family.getCommonSpecies(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:186)
- [ ] `Family.addSpecies(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:190)
- [ ] `Family.getSpecies(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:195)
- [ ] `Family.getSpeciesForLocation(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:199)
- [ ] `Family.getSpeciesForLocation(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:203)
- [ ] `Family.onTreeActivated(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:213)
- [ ] `Family.canStripBranch(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:221)
- [ ] `Family.stripBranch(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:226)
- [ ] `Family.isWood(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:241)
- [ ] `Family.getBranchName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:245)
- [ ] `Family.getBranchName(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:249)
- [ ] `Family.getBranchNameSuffix(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:253)
- [ ] `Family.createBranchBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:257)
- [ ] `Family.createBranch(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:266)
- [ ] `Family.createBranchItem(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:270)
- [ ] `Family.setBranch(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:276)
- [ ] `Family.setStrippedBranch(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:281)
- [ ] `Family.setupBranch(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:286)
- [ ] `Family.setBranchItem(1)`: it's generic (com/dtteam/dynamictrees/tree/family/Family.java:296)
- [ ] `Family.getBranch(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:301)
- [ ] `Family.getBranchForPlacement(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:305)
- [ ] `Family.getBranchForRootsPlacement(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:309)
- [ ] `Family.getStrippedBranch(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:313)
- [ ] `Family.getBranchItem(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:317)
- [ ] `Family.isThick(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:321)
- [ ] `Family.getMaxBranchRadius(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:325)
- [ ] `Family.setMaxBranchRadius(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:329)
- [ ] `Family.getRootColor(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:333)
- [ ] `Family.setStick(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:337)
- [ ] `Family.getStick(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:342)
- [ ] `Family.setPrimitiveLog(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:346)
- [ ] `Family.setPrimitiveStrippedLog(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:355)
- [ ] `Family.getPrimitiveLog(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:364)
- [ ] `Family.getPrimitiveStrippedLog(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:368)
- [ ] `Family.getLogDropsForBranch(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:372)
- [ ] `Family.isFireProof(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:382)
- [ ] `Family.setIsFireProof(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:386)
- [ ] `Family.getDefaultBranchHarvestTier(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:390)
- [ ] `Family.getDefaultStrippedBranchHarvestTier(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:395)
- [ ] `Family.getDefaultBranchMapColor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:400)
- [ ] `Family.getDefaultBranchSoundType(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:405)
- [ ] `Family.getDefaultBranchProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:410)
- [ ] `Family.getProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:423)
- [ ] `Family.setProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:427)
- [ ] `Family.getLootVolumeMultiplier(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:432)
- [ ] `Family.setLootVolumeMultiplier(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:436)
- [ ] `Family.getRadiusForCellKit(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:440)
- [ ] `Family.setPrimaryThickness(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:450)
- [ ] `Family.setSecondaryThickness(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:454)
- [ ] `Family.getPrimaryThickness(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:458)
- [ ] `Family.getSecondaryThickness(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:462)
- [ ] `Family.hasStrippedBranch(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:466)
- [ ] `Family.setHasStrippedBranch(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:470)
- [ ] `Family.getMinRadiusForStripping(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:474)
- [ ] `Family.setMinRadiusForStripping(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:478)
- [ ] `Family.reduceRadiusWhenStripping(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:482)
- [ ] `Family.setReduceRadiusWhenStripping(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:486)
- [ ] `Family.addValidBranches(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:490)
- [ ] `Family.getBranchBlockIndex(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:494)
- [ ] `Family.getValidBranchBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:504)
- [ ] `Family.isValidBranchBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:515)
- [ ] `Family.getNumberOfValidBranchBlocks(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:519)
- [ ] `Family.setBranchIsLadder(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:523)
- [ ] `Family.branchIsLadder(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:527)
- [ ] `Family.getMaxSignalDepth(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:531)
- [ ] `Family.setMaxSignalDepth(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:535)
- [ ] `Family.hasSurfaceRoot(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:539)
- [ ] `Family.setHasSurfaceRoot(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:543)
- [ ] `Family.createSurfaceRoot(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:547)
- [ ] `Family.getSurfaceRoot(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:553)
- [ ] `Family.setSurfaceRoot(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:557)
- [ ] `Family.isAcceptableSoilForRootSystem(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:562)
- [ ] `Family.hasRootSystem(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:566)
- [ ] `Family.selectAnimationHandler(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:570)
- [ ] `Family.expandLeavesBlockBounds(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:574)
- [ ] `Family.isCompatibleDynamicLeaves(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:578)
- [ ] `Family.isCompatibleGenericLeaves(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:583)
- [ ] `Family.getCommonLeaves(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:587)
- [ ] `Family.setCommonLeaves(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:591)
- [ ] `Family.defaultBranchTags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:596)
- [ ] `Family.defaultBranchItemTags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:600)
- [ ] `Family.defaultStrippedBranchTags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:604)
- [ ] `Family.addGeneratedBlockTags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:608)
- [ ] `Family.tierTag(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:631)
- [ ] `Family.addGeneratedItemTags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:640)
- [ ] `Family.getSurfaceRootLoader(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:650)
- [ ] `Family.getBranchLoader(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:654)
- [ ] `Family.getRootsLoader(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:658)
- [ ] `Family.getBranchItemParentLocation(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:662)
- [ ] `Family.getRootItemParentLocation(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:666)
- [ ] `Family.generateStateData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:670)
- [ ] `Family.generateItemModelData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:677)
- [ ] `Family.generateLangData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:682)
- [ ] `Family.setOnlyIfLoaded(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:687)
- [ ] `Family.isOnlyIfLoaded(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:691)
- [ ] `Family.setTextureOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:695)
- [ ] `Family.getTexturePath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:699)
- [ ] `Family.setModelOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:703)
- [ ] `Family.getModelPath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:707)
- [ ] `Family.setLangOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:711)
- [ ] `Family.getLangOverride(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:715)
- [ ] `Family.addBranchTextures(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:719)
- [ ] `Family.addRootTextures(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:746)
- [ ] `Family.topBranchTextureLocations(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:761)
- [ ] `Family.topBranchTextureLocation(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:774)
- [ ] `Family.toLoadDataString(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:783)
- [ ] `Family.toReloadDataString(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/family/Family.java:793)

### `Family.TreeActivationContext`: it uses Minecraft's `class_1937`

- [ ] `Family.TreeActivationContext.TreeActivationContext(8)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.level(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.rootPos(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.hitPos(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.hitState(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.player(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.hand(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.heldItem(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)
- [ ] `Family.TreeActivationContext.hitResult(0)`: its class isn't translated (it uses Minecraft's `class_1937`) (com/dtteam/dynamictrees/tree/family/Family.java:806)

### `NetherFungusFamily`: it extends `Family`

- [ ] `NetherFungusFamily.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:19)
- [ ] `NetherFungusFamily.NetherFungusFamily(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:21)
- [ ] `NetherFungusFamily.setCommonSpecies(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:25)
- [ ] `NetherFungusFamily.setPreReloadDefaults(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:37)
- [ ] `NetherFungusFamily.isFireProof(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:43)
- [ ] `NetherFungusFamily.expandLeavesBlockBounds(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:48)
- [ ] `NetherFungusFamily.defaultBranchTags(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:53)
- [ ] `NetherFungusFamily.defaultBranchItemTags(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:58)
- [ ] `NetherFungusFamily.defaultStrippedBranchTags(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/NetherFungusFamily.java:63)

### `PalmFamily`: it extends `Family`

- [ ] `PalmFamily.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/family/PalmFamily.java:15)
- [ ] `PalmFamily.PalmFamily(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/PalmFamily.java:17)
- [ ] `PalmFamily.setCommonSpecies(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/PalmFamily.java:21)
- [ ] `PalmFamily.setCommonLeaves(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/PalmFamily.java:30)
- [ ] `PalmFamily.createBranchBlock(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/PalmFamily.java:44)

### `UndergroundRootsFamily`: it extends `Family`

- [ ] `UndergroundRootsFamily.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:41)
- [ ] `UndergroundRootsFamily.UndergroundRootsFamily(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:59)
- [ ] `UndergroundRootsFamily.setCommonSpecies(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:63)
- [ ] `UndergroundRootsFamily.setDefaultSoil(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:77)
- [ ] `UndergroundRootsFamily.getDefaultSoil(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:88)
- [ ] `UndergroundRootsFamily.setupBlocks(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:92)
- [ ] `UndergroundRootsFamily.createRoots(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:99)
- [ ] `UndergroundRootsFamily.createRootsBlock(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:103)
- [ ] `UndergroundRootsFamily.createRootsItem(2)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:112)
- [ ] `UndergroundRootsFamily.getRootsNameSuffix(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:118)
- [ ] `UndergroundRootsFamily.setRoots(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:122)
- [ ] `UndergroundRootsFamily.setRootsItem(1)`: it's generic (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:127)
- [ ] `UndergroundRootsFamily.getRoots(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:132)
- [ ] `UndergroundRootsFamily.getRootsItem(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:136)
- [ ] `UndergroundRootsFamily.getBranchForRootsPlacement(3)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:140)
- [ ] `UndergroundRootsFamily.generateStateData(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:145)
- [ ] `UndergroundRootsFamily.generateItemModelData(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:151)
- [ ] `UndergroundRootsFamily.getBranchItemParentLocation(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:157)
- [ ] `UndergroundRootsFamily.defaultRootsTags(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:162)
- [ ] `UndergroundRootsFamily.getDefaultRootsHarvestTier(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:166)
- [ ] `UndergroundRootsFamily.isAcceptableSoilForRootSystem(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:171)
- [ ] `UndergroundRootsFamily.addAcceptableSoilsForRootSystem(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:177)
- [ ] `UndergroundRootsFamily.hasRootSystem(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:182)
- [ ] `UndergroundRootsFamily.getPrimaryRootThickness(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:187)
- [ ] `UndergroundRootsFamily.getSecondaryRootThickness(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:191)
- [ ] `UndergroundRootsFamily.getSupportedRootThicknessExtra(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:195)
- [ ] `UndergroundRootsFamily.setPrimaryRootThickness(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:199)
- [ ] `UndergroundRootsFamily.setSecondaryRootThickness(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:203)
- [ ] `UndergroundRootsFamily.setSupportedRootThicknessExtra(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:207)
- [ ] `UndergroundRootsFamily.setPrimitiveRoots(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:211)
- [ ] `UndergroundRootsFamily.setPrimitiveRootsFilled(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:218)
- [ ] `UndergroundRootsFamily.setPrimitiveRootsCovered(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:222)
- [ ] `UndergroundRootsFamily.getPrimitiveRoots(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:226)
- [ ] `UndergroundRootsFamily.getPrimitiveFilledRoots(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:230)
- [ ] `UndergroundRootsFamily.getPrimitiveCoveredRoots(0)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:234)
- [ ] `UndergroundRootsFamily.addGeneratedBlockTags(1)`: its class isn't translated (it extends `Family`) (com/dtteam/dynamictrees/tree/family/UndergroundRootsFamily.java:238)

### `NetherFungusSpecies`: it extends `Species`

- [ ] `NetherFungusSpecies.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:28)
- [ ] `NetherFungusSpecies.NetherFungusSpecies(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:30)
- [ ] `NetherFungusSpecies.setDefaultGrowingParameters(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:38)
- [ ] `NetherFungusSpecies.setStandardSoils(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:44)
- [ ] `NetherFungusSpecies.setPreReloadDefaults(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:49)
- [ ] `NetherFungusSpecies.setPostReloadDefaults(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:54)
- [ ] `NetherFungusSpecies.isAcceptableSoilForWorldgen(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:63)
- [ ] `NetherFungusSpecies.defaultSeedComposterChance(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:68)
- [ ] `NetherFungusSpecies.defaultSaplingTags(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:73)
- [ ] `NetherFungusSpecies.defaultSeedTags(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:78)
- [ ] `NetherFungusSpecies.addSaplingTextures(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:83)
- [ ] `NetherFungusSpecies.falloverParticleFlingMultiplier(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:90)
- [ ] `NetherFungusSpecies.getFallingTreeStartSound(2)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:95)
- [ ] `NetherFungusSpecies.getFallingTreeEndSound(2)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:100)
- [ ] `NetherFungusSpecies.getFallingBranchEndSound(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:105)
- [ ] `NetherFungusSpecies.getFallingTreePitch(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/NetherFungusSpecies.java:110)

### `PalmSpecies`: it extends `Species`

- [ ] `PalmSpecies.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/species/PalmSpecies.java:31)
- [ ] `PalmSpecies.PalmSpecies(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/PalmSpecies.java:33)
- [ ] `PalmSpecies.postGrow(5)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/PalmSpecies.java:52)
- [ ] `PalmSpecies.transitionToTree(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/PalmSpecies.java:76)
- [ ] `PalmSpecies.postGeneration(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/PalmSpecies.java:85)
- [ ] `PalmSpecies.getFellingLeavesClusters(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/PalmSpecies.java:109)

### `Species`: it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)

- [ ] `Species.blockStateGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/tree/species/Species.java:155)
- [ ] `Species.itemModelGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/tree/species/Species.java:156)
- [ ] `Species.languageGenerators`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/tree/species/Species.java:157)
- [ ] `Species.NULL_SPECIES`: its value couldn't be read: an anonymous class (com/dtteam/dynamictrees/tree/species/Species.java:158)
- [ ] `Species.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/species/Species.java:206)
- [ ] `Species.CODEC`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/tree/species/Species.java:207)
- [ ] `Species.REGISTRY`: it uses TypedRegistry, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/species/Species.java:208)
- [ ] `Species.upFirst`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/tree/species/Species.java:244)
- [x] `Species.defaultGrowthOffset` (com/dtteam/dynamictrees/tree/species/Species.java:256)
- [x] `Species.defaultFruitingOffset` (com/dtteam/dynamictrees/tree/species/Species.java:257)
- [x] `Species.defaultSeedDropOffset` (com/dtteam/dynamictrees/tree/species/Species.java:258)
- [x] `Species.SAPLING` (com/dtteam/dynamictrees/tree/species/Species.java:278)
- [x] `Species.SEED_PARENT` (com/dtteam/dynamictrees/tree/species/Species.java:279)
- [x] `Species.SEED` (com/dtteam/dynamictrees/tree/species/Species.java:280)
- [ ] `Species.createDefaultType(1)`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/species/Species.java:283)
- [ ] `Species.createDefaultCodec(1)`: it uses Minecraft's `Codec` (com/dtteam/dynamictrees/tree/species/Species.java:287)
- [ ] `Species.read(1)`: it uses Minecraft's `DataResult` (com/dtteam/dynamictrees/tree/species/Species.java:298)
- [ ] `Species.Species(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:303)
- [ ] `Species.Species(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:307)
- [ ] `Species.Species(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:311)
- [ ] `Species.reset(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:319)
- [ ] `Species.setPreReloadDefaults(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:329)
- [ ] `Species.setPostReloadDefaults(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:333)
- [ ] `Species.setDefaultGrowingParameters(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:345)
- [ ] `Species.getFamily(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:349)
- [ ] `Species.setFamily(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:353)
- [ ] `Species.setUnlocalizedName(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:358)
- [ ] `Species.getLocalizedName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:363)
- [ ] `Species.getUnlocalizedName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:367)
- [ ] `Species.getTextComponent(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:371)
- [ ] `Species.setLeavesProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:376)
- [ ] `Species.getLeavesProperties(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:383)
- [ ] `Species.getLeavesBlock(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:387)
- [ ] `Species.getPrimitiveLeaves(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:391)
- [ ] `Species.addValidLeafBlocks(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:395)
- [ ] `Species.getLeafBlockIndex(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:403)
- [ ] `Species.getValidLeavesProperties(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:413)
- [ ] `Species.getValidLeafBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:423)
- [ ] `Species.isValidLeafBlock(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:428)
- [ ] `Species.colorTreeQuads(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:432)
- [ ] `Species.leafColorMultiplier(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:436)
- [ ] `Species.defaultSeedComposterChance(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:440)
- [ ] `Species.getSeedStack(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:444)
- [ ] `Species.hasSeed(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:448)
- [ ] `Species.getSeed(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:452)
- [ ] `Species.shouldGenerateSeed(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:456)
- [ ] `Species.setShouldGenerateSeed(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:460)
- [ ] `Species.setShouldGenerateSeedIfNull(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:464)
- [ ] `Species.getSeedName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:472)
- [ ] `Species.setSeedName(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:478)
- [ ] `Species.generateSeed(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:482)
- [ ] `Species.setSeed(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:486)
- [ ] `Species.getVoluntaryDrops(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:491)
- [ ] `Species.createVoluntaryLootParams(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:502)
- [ ] `Species.getLootTable(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:510)
- [ ] `Species.getBranchesDrops(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:521)
- [ ] `Species.getBranchesDrops(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:525)
- [ ] `Species.getBranchesDrops(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:529)
- [ ] `Species.processVolume(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:549)
- [ ] `Species.getDropsForBranchType(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:554)
- [ ] `Species.createBranchesLootParams(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:561)
- [ ] `Species.cleanDropsList(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:570)
- [ ] `Species.getLogsAndSticks(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:586)
- [ ] `Species.handleVoluntaryDrops(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:602)
- [ ] `Species.addPrimitiveSaplingRecipe(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:636)
- [ ] `Species.shouldReplaceSaplingWhenPlaced(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:641)
- [ ] `Species.shouldReplaceSaplingWhenGrown(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:651)
- [ ] `Species.getPrimitiveSaplingRecipes(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:661)
- [ ] `Species.setOverrideSaplingReplacementWhenCrouching(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:665)
- [ ] `Species.overrideSaplingReplacementWhenCrouching(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:669)
- [ ] `Species.addPrimitiveSaplingItem(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:673)
- [ ] `Species.setSapling(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:678)
- [ ] `Species.shouldGenerateSapling(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:683)
- [ ] `Species.setShouldGenerateSapling(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:687)
- [ ] `Species.setShouldGenerateSaplingIfNull(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:691)
- [ ] `Species.generateSapling(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:699)
- [ ] `Species.getSapling(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:705)
- [ ] `Species.selfOrLocationOverride(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:709)
- [ ] `Species.shouldUseLocationOverride(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:713)
- [ ] `Species.plantSapling(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:717)
- [ ] `Species.addAcceptableBlockForGrowth(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:729)
- [ ] `Species.canSaplingGrow(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:733)
- [ ] `Species.setCanSaplingGrowNaturally(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:738)
- [ ] `Species.canSaplingGrowNaturally(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:743)
- [ ] `Species.canSaplingConsumeBoneMeal(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:747)
- [ ] `Species.canSaplingGrowAfterBoneMeal(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:751)
- [ ] `Species.saplingFireSpread(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:755)
- [ ] `Species.saplingFlammability(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:759)
- [ ] `Species.transitionToTree(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:763)
- [ ] `Species.shouldTransitionToTree(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:769)
- [ ] `Species.transitionToTree(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:773)
- [ ] `Species.getSaplingShape(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:792)
- [ ] `Species.setSaplingShape(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:796)
- [ ] `Species.getSaplingRegName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:801)
- [ ] `Species.getSaplingModelName(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:807)
- [ ] `Species.setSaplingName(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:811)
- [ ] `Species.setTintSapling(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:815)
- [ ] `Species.saplingColorMultiplier(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:819)
- [ ] `Species.getSaplingSound(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:831)
- [ ] `Species.setSaplingSound(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:835)
- [ ] `Species.getPottedSapling(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:840)
- [ ] `Species.placeRootyDirtBlock(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:844)
- [ ] `Species.placeRootyDirtBlock(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:866)
- [ ] `Species.updateRootyDirtBlock(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:874)
- [ ] `Species.getForceSoil(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:885)
- [ ] `Species.setForceSoil(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:889)
- [ ] `Species.setSoilLongevity(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:894)
- [ ] `Species.getSoilLongevity(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:899)
- [ ] `Species.isThick(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:903)
- [ ] `Species.getMaxBranchRadius(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:907)
- [ ] `Species.setMaxBranchRadius(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:911)
- [ ] `Species.addAcceptableSoils(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:915)
- [ ] `Species.addAcceptableSoilsForWorldGen(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:920)
- [ ] `Species.clearAcceptableSoils(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:925)
- [ ] `Species.setStandardSoils(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:931)
- [ ] `Species.hasAcceptableSoil(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:935)
- [ ] `Species.moveGroundPosWorldgen(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:939)
- [ ] `Species.isAcceptableSoil(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:952)
- [ ] `Species.isAcceptableSoil(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:956)
- [ ] `Species.isAcceptableSoil(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:960)
- [ ] `Species.setAllowedWaterHeightForWorldgen(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:964)
- [ ] `Species.getAllowedWaterHeightForWorldgen(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:968)
- [ ] `Species.isAcceptableSoilForWorldgen(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:972)
- [ ] `Species.isAcceptableSoilForWorldgen(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:983)
- [ ] `Species.isWater(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:987)
- [ ] `Species.countWaterBlocksBelow(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:991)
- [ ] `Species.setPlantableOnFluid(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1003)
- [ ] `Species.isPlantableOnFluid(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1007)
- [ ] `Species.soilDestroyAction(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1011)
- [ ] `Species.doesRequireTileEntity(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1015)
- [ ] `Species.hasCommonOverride(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1019)
- [ ] `Species.setCommonOverride(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1023)
- [ ] `Species.shouldOverrideCommon(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1027)
- [ ] `Species.setBasicGrowingParameters(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1036)
- [ ] `Species.setTapering(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1045)
- [ ] `Species.setUpProbability(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1049)
- [ ] `Species.setLowestBranchHeight(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1053)
- [ ] `Species.setSignalEnergy(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1057)
- [ ] `Species.setGrowthRate(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1061)
- [ ] `Species.getSignalEnergy(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1065)
- [ ] `Species.getEnergy(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1069)
- [ ] `Species.getGrowthRate(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1073)
- [ ] `Species.getUpProbability(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1077)
- [ ] `Species.getProbabilityForCurrentDir(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1081)
- [ ] `Species.getLowestBranchHeight(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1085)
- [ ] `Species.getTapering(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1089)
- [ ] `Species.update(8)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1093)
- [ ] `Species.getEnds(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1111)
- [ ] `Species.handleRot(6)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1117)
- [ ] `Species.setDoesRot(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1142)
- [ ] `Species.rot(8)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1146)
- [ ] `Species.postRot(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1181)
- [ ] `Species.rotChance(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1185)
- [ ] `Species.grow(8)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1189)
- [ ] `Species.sendGrowthSignal(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1214)
- [ ] `Species.setGrowthLogicKit(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1219)
- [ ] `Species.setGrowthLogicKit(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1224)
- [ ] `Species.getGrowthLogicKit(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1229)
- [ ] `Species.setCanBoneMealTree(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1233)
- [ ] `Species.canBoneMealTree(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1237)
- [ ] `Species.postGrow(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1241)
- [ ] `Species.handleDisease(5)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1247)
- [ ] `Species.biomeSuitability(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1256)
- [ ] `Species.setClimateTolerance(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1279)
- [ ] `Species.setPreferredClimate(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1283)
- [ ] `Species.getPreferredClimate(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1287)
- [ ] `Species.getClimateTolerance(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1291)
- [ ] `Species.setSeasonalFruitingOffset(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1295)
- [ ] `Species.setSeasonalFruitingOffset(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1299)
- [ ] `Species.setSeasonalGrowthOffset(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1314)
- [ ] `Species.setSeasonalGrowthOffset(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1318)
- [ ] `Species.setSeasonalSeedDropOffset(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1329)
- [ ] `Species.setSeasonalSeedDropOffset(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1333)
- [ ] `Species.setSeasonalGrowthOffset(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1344)
- [ ] `Species.setSeasonalSeedDropOffset(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1349)
- [ ] `Species.setSeasonalFruitingOffset(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1354)
- [ ] `Species.seasonalGrowthFactor(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1359)
- [ ] `Species.seasonalSeedDropFactor(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1365)
- [ ] `Species.seasonalFruitProductionFactor(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1371)
- [ ] `Species.inheritSeasonalFruitingParametersToFruits(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1377)
- [ ] `Species.inheritSeasonalFruitingParametersToPods(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1395)
- [ ] `Species.getSeasonalTooltipFlags(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1413)
- [ ] `Species.showSeasonalTooltip(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1449)
- [ ] `Species.isInFlowerHoldPeriod(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1453)
- [ ] `Species.getSubstanceEffect(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1470)
- [ ] `Species.applySubstance(6)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1481)
- [ ] `Species.onTreeActivated(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1505)
- [ ] `Species.consumePlayerItem(3)`: it uses Minecraft's `class_1657` (com/dtteam/dynamictrees/tree/species/Species.java:1515)
- [ ] `Species.useDefaultWailaBody(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1527)
- [ ] `Species.setAlwaysShowOnWaila(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1531)
- [ ] `Species.showSpeciesOnWaila(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1536)
- [ ] `Species.getMegaSpecies(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1540)
- [ ] `Species.getPreMegaSpecies(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1544)
- [ ] `Species.isMegaSpecies(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1548)
- [ ] `Species.setMegaSpecies(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1552)
- [ ] `Species.setCanCraftMegaSeed(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1557)
- [ ] `Species.canCraftMegaSeed(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1561)
- [ ] `Species.selectAnimationHandler(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1565)
- [ ] `Species.getFellingLeavesClusters(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1569)
- [ ] `Species.canEncodeLeavesBlocks(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1574)
- [ ] `Species.encodeLeavesPos(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1578)
- [ ] `Species.encodeLeavesBlocks(4)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1582)
- [ ] `Species.leavesAreSolid(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1586)
- [ ] `Species.falloverParticleFlingMultiplier(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1590)
- [ ] `Species.getFallingTreeStartSound(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1594)
- [ ] `Species.getFallingTreeEndSound(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1598)
- [ ] `Species.getFallingTreePitch(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1602)
- [ ] `Species.getFallingBranchPitch(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1606)
- [ ] `Species.getFallingTreeHitWaterSound(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1610)
- [ ] `Species.getFallingBranchEndSound(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1614)
- [ ] `Species.setBigTreeSoundThreshold(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1620)
- [ ] `Species.generate(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1624)
- [ ] `Species.shouldGenerate(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1649)
- [ ] `Species.getJoCode(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1663)
- [ ] `Species.getRootsJoCode(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1667)
- [ ] `Species.getJoCodes(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1671)
- [ ] `Species.getRootsJoCodes(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1675)
- [ ] `Species.addGenFeature(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1679)
- [ ] `Species.addGenFeature(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1683)
- [ ] `Species.hasGenFeatures(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1691)
- [ ] `Species.getGenFeatures(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1695)
- [ ] `Species.preGeneration(6)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1699)
- [ ] `Species.postGeneration(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1711)
- [ ] `Species.getWorldGenTaperingFactor(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1715)
- [ ] `Species.getWorldGenLeafMapHeight(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1719)
- [ ] `Species.setWorldGenLeafMapHeight(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1723)
- [ ] `Species.getWorldGenAgeIterations(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1727)
- [ ] `Species.getNodeInflator(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1731)
- [ ] `Species.getNodeInflator(2)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1735)
- [ ] `Species.coordHashCode(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1739)
- [ ] `Species.hasFruit(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1743)
- [ ] `Species.hasFruits(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1747)
- [ ] `Species.addFruits(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1751)
- [ ] `Species.getFruits(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1755)
- [ ] `Species.hasPod(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1759)
- [ ] `Species.hasPods(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1763)
- [ ] `Species.addPods(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1767)
- [ ] `Species.getPods(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1771)
- [ ] `Species.getCommonSpecies(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1775)
- [ ] `Species.isCommonSpecies(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1779)
- [ ] `Species.isSeedCommon(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1783)
- [ ] `Species.generateStateData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1787)
- [ ] `Species.generateItemModelData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1792)
- [ ] `Species.generateLangData(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1797)
- [ ] `Species.setOnlyIfLoaded(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1802)
- [ ] `Species.isOnlyIfLoaded(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1806)
- [ ] `Species.setModelOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1810)
- [ ] `Species.setTextureOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1814)
- [ ] `Species.setLangOverrides(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1818)
- [ ] `Species.getModelPath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1822)
- [ ] `Species.getTexturePath(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1826)
- [ ] `Species.getLangOverride(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1830)
- [ ] `Species.getSaplingSmartModelLocation(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1834)
- [ ] `Species.addSaplingTextures(3)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1838)
- [ ] `Species.getSeedParentModelLocation(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1845)
- [ ] `Species.addGeneratedBlockTags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1849)
- [ ] `Species.addGeneratedItemTags(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1859)
- [ ] `Species.shouldGenerateVoluntaryDrops(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1871)
- [ ] `Species.getVoluntaryDropsPath(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1875)
- [ ] `Species.createVoluntaryDrops(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1879)
- [ ] `Species.setDropSeeds(1)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1883)
- [ ] `Species.shouldDropSeeds(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1887)
- [ ] `Species.defaultSaplingTags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1891)
- [ ] `Species.defaultSeedTags(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1895)
- [ ] `Species.toLoadDataString(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1899)
- [ ] `Species.toReloadDataString(0)`: its class isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1908)
- [ ] `Species.findSpecies(1)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1937)
- [ ] `Species.findSpecies(1)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1941)
- [ ] `Species.findSpeciesSloppy(1)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/species/Species.java:1945)
- [ ] `Species.getSpeciesDirectory(0)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/tree/species/Species.java:1960)

### `Species.BiomeSuitabilityEventResult`

- [x] `Species.BiomeSuitabilityEventResult.BiomeSuitabilityEventResult(2)` (com/dtteam/dynamictrees/tree/species/Species.java:1964)
- [x] `Species.BiomeSuitabilityEventResult.handled(0)` (com/dtteam/dynamictrees/tree/species/Species.java:1964)
- [x] `Species.BiomeSuitabilityEventResult.suitability(0)` (com/dtteam/dynamictrees/tree/species/Species.java:1964)

### `Species.CommonOverride`: it's an interface


### `Species.LogsAndSticks`: it uses Minecraft's `class_1799`

- [ ] `Species.LogsAndSticks.LogsAndSticks(2)`: its class isn't translated (it uses Minecraft's `class_1799`) (com/dtteam/dynamictrees/tree/species/Species.java:1975)

### `SwampSpecies`: it extends `Species`

- [ ] `SwampSpecies.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/species/SwampSpecies.java:12)
- [x] `SwampSpecies.minRadiusForSunkGeneration` (com/dtteam/dynamictrees/tree/species/SwampSpecies.java:13)
- [ ] `SwampSpecies.SwampSpecies(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/SwampSpecies.java:15)
- [ ] `SwampSpecies.generate(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/SwampSpecies.java:19)

### `SwampSpecies.WaterSurfaceGenerationState`


### `UndergroundRootsSpecies`: it extends `Species`

- [ ] `UndergroundRootsSpecies.TYPE`: it uses EntryType, which isn't translated (it's generic) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:35)
- [ ] `UndergroundRootsSpecies.setMinWorldGenHeightOffset(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:44)
- [ ] `UndergroundRootsSpecies.setMaxWorldGenHeightOffset(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:48)
- [ ] `UndergroundRootsSpecies.setRootGrowthMultiplier(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:52)
- [ ] `UndergroundRootsSpecies.setUpdateSoilOnWaterRadius(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:56)
- [ ] `UndergroundRootsSpecies.UndergroundRootsSpecies(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:60)
- [ ] `UndergroundRootsSpecies.getFamily(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:74)
- [ ] `UndergroundRootsSpecies.placeRootyDirtBlock(3)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:78)
- [ ] `UndergroundRootsSpecies.replaceSoilBlock(4)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:95)
- [ ] `UndergroundRootsSpecies.postGrow(5)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:115)
- [ ] `UndergroundRootsSpecies.soilDestroyAction(4)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:135)
- [ ] `UndergroundRootsSpecies.rotChance(4)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:140)
- [ ] `UndergroundRootsSpecies.update(8)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:154)
- [ ] `UndergroundRootsSpecies.generate(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:164)
- [ ] `UndergroundRootsSpecies.getRootSignalEnergy(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:180)
- [ ] `UndergroundRootsSpecies.setRootSignalEnergy(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:184)
- [ ] `UndergroundRootsSpecies.getRootTapering(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:188)
- [ ] `UndergroundRootsSpecies.setRootTapering(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:192)
- [ ] `UndergroundRootsSpecies.sendGrowthSignal(5)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:196)
- [ ] `UndergroundRootsSpecies.getRootEnergy(2)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:216)
- [ ] `UndergroundRootsSpecies.setRootsGrowthLogicKit(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:220)
- [ ] `UndergroundRootsSpecies.setRootsGrowthLogicKit(1)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:225)
- [ ] `UndergroundRootsSpecies.getRootsGrowthLogicKit(0)`: its class isn't translated (it extends `Species`) (com/dtteam/dynamictrees/tree/species/UndergroundRootsSpecies.java:230)

### `TreeHelper`

- [ ] `TreeHelper.NULL_TREE_PART`: it uses TreePart, which isn't translated (it's an interface) (com/dtteam/dynamictrees/tree/TreeHelper.java:35)
- [ ] `TreeHelper.growPulse(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/TreeHelper.java:37)
- [ ] `TreeHelper.ageVolume(4)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/tree/TreeHelper.java:46)
- [ ] `TreeHelper.ageVolume(6)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/tree/TreeHelper.java:82)
- [ ] `TreeHelper.getRootsJoCode(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/TreeHelper.java:98)
- [ ] `TreeHelper.getJoCode(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/TreeHelper.java:102)
- [ ] `TreeHelper.getJoCode(4)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/TreeHelper.java:106)
- [ ] `TreeHelper.dereferenceTrunkShell(2)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/tree/TreeHelper.java:118)
- [ ] `TreeHelper.getCommonSpecies(2)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/TreeHelper.java:130)
- [ ] `TreeHelper.getExactSpecies(2)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/TreeHelper.java:136)
- [ ] `TreeHelper.getBestGuessSpecies(2)`: it uses Species, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/tree/TreeHelper.java:149)
- [ ] `TreeHelper.findRootNode(2)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/tree/TreeHelper.java:154)
- [ ] `TreeHelper.setCustomRootBlockDecay(1)`: it uses SoilBlockDecayer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/tree/TreeHelper.java:173)
- [ ] `TreeHelper.getCustomRootBlockDecay(0)`: it uses SoilBlockDecayer, which isn't translated (it's an interface) (com/dtteam/dynamictrees/tree/TreeHelper.java:177)
- [ ] `TreeHelper.treeParticles(4)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/TreeHelper.java:181)
- [ ] `TreeHelper.rootParticles(5)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/TreeHelper.java:187)
- [ ] `TreeHelper.startAnalysisFromRoot(3)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/tree/TreeHelper.java:196)
- [ ] `TreeHelper.destroyTree(4)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/tree/TreeHelper.java:206)
- [ ] `TreeHelper.isTreePart(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:217)
- [ ] `TreeHelper.isTreePart(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:221)
- [ ] `TreeHelper.isTreePart(2)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/tree/TreeHelper.java:225)
- [ ] `TreeHelper.getTreePart(1)`: it uses TreePart, which isn't translated (it's an interface) (com/dtteam/dynamictrees/tree/TreeHelper.java:229)
- [ ] `TreeHelper.getTreePart(1)`: it uses TreePart, which isn't translated (it's an interface) (com/dtteam/dynamictrees/tree/TreeHelper.java:233)
- [ ] `TreeHelper.isBranch(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:237)
- [ ] `TreeHelper.isBranch(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:241)
- [ ] `TreeHelper.getBranch(1)`: it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:245)
- [ ] `TreeHelper.getBranch(1)`: it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:250)
- [ ] `TreeHelper.getBranch(1)`: it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:255)
- [ ] `TreeHelper.getRadius(2)`: it uses Minecraft's `class_1922` (com/dtteam/dynamictrees/tree/TreeHelper.java:260)
- [ ] `TreeHelper.getBranchOpt(1)`: it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:265)
- [ ] `TreeHelper.getBranchOpt(1)`: it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:269)
- [ ] `TreeHelper.getBranchOpt(1)`: it uses BranchBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:274)
- [ ] `TreeHelper.isLeaves(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:278)
- [ ] `TreeHelper.isLeaves(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:282)
- [ ] `TreeHelper.getLeaves(1)`: it uses DynamicLeavesBlock, which isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/tree/TreeHelper.java:286)
- [ ] `TreeHelper.getLeaves(1)`: it uses DynamicLeavesBlock, which isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/tree/TreeHelper.java:291)
- [ ] `TreeHelper.getLeaves(1)`: it uses DynamicLeavesBlock, which isn't translated (it extends Minecraft's `class_2397`) (com/dtteam/dynamictrees/tree/TreeHelper.java:296)
- [ ] `TreeHelper.isRooty(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:301)
- [ ] `TreeHelper.isRooty(1)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/tree/TreeHelper.java:305)
- [ ] `TreeHelper.getRooty(1)`: it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:309)
- [ ] `TreeHelper.getRooty(1)`: it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:314)
- [ ] `TreeHelper.getRooty(1)`: it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:319)
- [ ] `TreeHelper.getRootyOpt(1)`: it uses SoilBlock, which isn't translated (it extends `BlockWithDynamicHardness` (com.dtteam.dynamictrees.block)) (com/dtteam/dynamictrees/tree/TreeHelper.java:324)

### `CommonCollectors`

- [ ] `CommonCollectors.toLinkedSet(0)`: it's generic (com/dtteam/dynamictrees/utility/CommonCollectors.java:12)
- [ ] `CommonCollectors.toUnmodifiableSet(0)`: it's generic (com/dtteam/dynamictrees/utility/CommonCollectors.java:19)
- [ ] `CommonCollectors.toUnmodifiableLinkedSet(0)`: it's generic (com/dtteam/dynamictrees/utility/CommonCollectors.java:31)
- [ ] `CommonCollectors.toLinkedList(0)`: it's generic (com/dtteam/dynamictrees/utility/CommonCollectors.java:38)

### `CoordUtils`

- [x] `CoordUtils.coordXor` (com/dtteam/dynamictrees/utility/CoordUtils.java:32)
- [ ] `CoordUtils.HORIZONTALS`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/utility/CoordUtils.java:33)
- [x] `CoordUtils.coordHashMap` (com/dtteam/dynamictrees/utility/CoordUtils.java:36)
- [ ] `CoordUtils.getRandomDir(1)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/utility/CoordUtils.java:38)
- [ ] `CoordUtils.getRayTraceFruitPos(5)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/utility/CoordUtils.java:42)
- [ ] `CoordUtils.branchRayTrace(7)`: it uses Minecraft's `class_3965` (com/dtteam/dynamictrees/utility/CoordUtils.java:60)
- [ ] `CoordUtils.rayTraceBlocks(2)`: it uses Minecraft's `class_3965` (com/dtteam/dynamictrees/utility/CoordUtils.java:94)
- [ ] `CoordUtils.getRayTraceVector(3)`: it's generic (com/dtteam/dynamictrees/utility/CoordUtils.java:121)
- [ ] `CoordUtils.findWorldSurface(3)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/utility/CoordUtils.java:186)
- [ ] `CoordUtils.findWorldSurface(3)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/utility/CoordUtils.java:190)
- [ ] `CoordUtils.inRange(3)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/utility/CoordUtils.java:196)
- [ ] `CoordUtils.coordHashCode(4)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/utility/CoordUtils.java:200)
- [ ] `CoordUtils.coordHashCode(2)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/utility/CoordUtils.java:205)
- [ ] `CoordUtils.goHorSides(1)`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/utility/CoordUtils.java:210)
- [ ] `CoordUtils.goHorSides(2)`: its body couldn't be read: an anonymous class (com/dtteam/dynamictrees/utility/CoordUtils.java:214)

### `CoordUtils.CustomRayTraceContext`: it uses Minecraft's `class_243`

- [ ] `CoordUtils.CustomRayTraceContext.CustomRayTraceContext(4)`: its class isn't translated (it uses Minecraft's `class_243`) (com/dtteam/dynamictrees/utility/CoordUtils.java:242)
- [ ] `CoordUtils.CustomRayTraceContext.getEndVector(0)`: its class isn't translated (it uses Minecraft's `class_243`) (com/dtteam/dynamictrees/utility/CoordUtils.java:249)
- [ ] `CoordUtils.CustomRayTraceContext.getStartVector(0)`: its class isn't translated (it uses Minecraft's `class_243`) (com/dtteam/dynamictrees/utility/CoordUtils.java:253)
- [ ] `CoordUtils.CustomRayTraceContext.getBlockShape(3)`: its class isn't translated (it uses Minecraft's `class_243`) (com/dtteam/dynamictrees/utility/CoordUtils.java:257)
- [ ] `CoordUtils.CustomRayTraceContext.getFluidShape(3)`: its class isn't translated (it uses Minecraft's `class_243`) (com/dtteam/dynamictrees/utility/CoordUtils.java:261)

### `CoordUtils.Surround`: its constants carry values

- [ ] `CoordUtils.Surround.Surround(2)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/utility/CoordUtils.java:279)
- [ ] `CoordUtils.Surround.method_15434(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/utility/CoordUtils.java:290)
- [ ] `CoordUtils.Surround.getOffset(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/utility/CoordUtils.java:294)
- [ ] `CoordUtils.Surround.getOffsetPos(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/utility/CoordUtils.java:298)
- [ ] `CoordUtils.Surround.getOpposite(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/utility/CoordUtils.java:302)

### `EntityUtils`

- [ ] `EntityUtils.playerRayTrace(3)`: it uses Minecraft's `class_3965` (com/dtteam/dynamictrees/utility/EntityUtils.java:14)
- [ ] `EntityUtils.getHitDirection(1)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/utility/EntityUtils.java:23)

### `ItemUtils`

- [ ] `ItemUtils.spawnItemStack(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/utility/ItemUtils.java:24)
- [ ] `ItemUtils.spawnItemStack(4)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/utility/ItemUtils.java:28)
- [ ] `ItemUtils.damageAxe(5)`: it uses Minecraft's `class_1309` (com/dtteam/dynamictrees/utility/ItemUtils.java:40)
- [ ] `ItemUtils.getEnchantmentLevel(3)`: it uses Minecraft's `class_5321` (com/dtteam/dynamictrees/utility/ItemUtils.java:57)
- [ ] `ItemUtils.getEnchantment(2)`: it uses Minecraft's `class_6883` (com/dtteam/dynamictrees/utility/ItemUtils.java:61)

### `MathUtils`

- [ ] `MathUtils.selectRandomFromDistribution(2)`: it uses Minecraft's `class_5819` (com/dtteam/dynamictrees/utility/MathUtils.java:6)
- [x] `MathUtils.shortDegreesDist(2)` (com/dtteam/dynamictrees/utility/MathUtils.java:30)
- [x] `MathUtils.angleDegreesInterpolate(3)` (com/dtteam/dynamictrees/utility/MathUtils.java:36)
- [ ] `MathUtils.randomBetween(3)`: it uses Minecraft's `class_5819` (com/dtteam/dynamictrees/utility/MathUtils.java:40)

### `NullUtils`

- [ ] `NullUtils.isNotNull(1)`: it uses `Object` (com/dtteam/dynamictrees/utility/NullUtils.java:8)
- [ ] `NullUtils.consumeIfNonnull(2)`: it's generic (com/dtteam/dynamictrees/utility/NullUtils.java:12)
- [ ] `NullUtils.applyIfNonnull(2)`: it's generic (com/dtteam/dynamictrees/utility/NullUtils.java:21)
- [ ] `NullUtils.applyIfNonnull(3)`: it's generic (com/dtteam/dynamictrees/utility/NullUtils.java:26)

### `Optionals`

- [ ] `Optionals.ofBlock(1)`: it's generic (com/dtteam/dynamictrees/utility/Optionals.java:14)
- [ ] `Optionals.ofBlock(1)`: it's generic (com/dtteam/dynamictrees/utility/Optionals.java:18)
- [ ] `Optionals.ofItem(1)`: it's generic (com/dtteam/dynamictrees/utility/Optionals.java:22)
- [ ] `Optionals.ofItem(1)`: it's generic (com/dtteam/dynamictrees/utility/Optionals.java:26)
- [ ] `Optionals.ifAllPresent(3)`: it's generic (com/dtteam/dynamictrees/utility/Optionals.java:30)
- [ ] `Optionals.ifAllPresent(4)`: it's generic (com/dtteam/dynamictrees/utility/Optionals.java:34)

### `ResourceLocationUtils`

- [ ] `ResourceLocationUtils.parse(2)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java:8)
- [ ] `ResourceLocationUtils.namespace(2)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java:16)
- [ ] `ResourceLocationUtils.prefix(2)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java:20)
- [ ] `ResourceLocationUtils.suffix(2)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java:24)
- [ ] `ResourceLocationUtils.surround(3)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java:28)
- [ ] `ResourceLocationUtils.parseDTLocation(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java:32)
- [ ] `ResourceLocationUtils.parseDTLocation(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/utility/ResourceLocationUtils.java:36)

### `BiomeDatabase`: it uses IDTBiomeHolderSet, which isn't translated (it's an interface)

- [ ] `BiomeDatabase.getJsonEntry(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:25)
- [ ] `BiomeDatabase.getEntry(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:29)
- [ ] `BiomeDatabase.getEntry(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:33)
- [ ] `BiomeDatabase.getEntry(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:49)
- [ ] `BiomeDatabase.getAllEntries(0)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:53)
- [ ] `BiomeDatabase.reset(0)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:57)
- [ ] `BiomeDatabase.isPopulated(0)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:62)
- [ ] `BiomeDatabase.getSpecies(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:66)
- [ ] `BiomeDatabase.getChance(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:70)
- [ ] `BiomeDatabase.getDensitySelector(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:74)
- [ ] `BiomeDatabase.getForestness(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:78)
- [ ] `BiomeDatabase.getHeightmap(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:82)
- [ ] `BiomeDatabase.getMultipass(1)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:86)
- [ ] `BiomeDatabase.setForestness(2)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:90)
- [ ] `BiomeDatabase.setHeightmap(2)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:95)
- [ ] `BiomeDatabase.setMultipass(2)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:100)
- [ ] `BiomeDatabase.copyOf(1)`: it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:105)

### `BiomeDatabase.BaseEntry`: it implements `EntryReader`

- [ ] `BiomeDatabase.BaseEntry.defaultMultipass`: a function kept in a variable or field (only parameters can be) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:123)
- [ ] `BiomeDatabase.BaseEntry.BaseEntry(2)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:126)
- [ ] `BiomeDatabase.BaseEntry.getDatabase(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:131)
- [ ] `BiomeDatabase.BaseEntry.getBiomeKey(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:135)
- [ ] `BiomeDatabase.BaseEntry.getChanceSelector(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:141)
- [ ] `BiomeDatabase.BaseEntry.getDensitySelector(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:146)
- [ ] `BiomeDatabase.BaseEntry.getSpeciesSelector(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:151)
- [ ] `BiomeDatabase.BaseEntry.setChanceSelector(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:156)
- [ ] `BiomeDatabase.BaseEntry.setChanceSelector(2)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:160)
- [ ] `BiomeDatabase.BaseEntry.setDensitySelector(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:180)
- [ ] `BiomeDatabase.BaseEntry.setDensitySelector(2)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:184)
- [ ] `BiomeDatabase.BaseEntry.setSpeciesSelector(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:204)
- [ ] `BiomeDatabase.BaseEntry.setSpeciesSelector(2)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:208)
- [ ] `BiomeDatabase.BaseEntry.getFeatureCancellation(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:228)
- [ ] `BiomeDatabase.BaseEntry.getOrCreateFeatureCancellation(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:233)
- [ ] `BiomeDatabase.BaseEntry.setBlacklisted(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:241)
- [ ] `BiomeDatabase.BaseEntry.isBlacklisted(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:245)
- [ ] `BiomeDatabase.BaseEntry.setForestness(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:250)
- [ ] `BiomeDatabase.BaseEntry.setHeightmap(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:254)
- [ ] `BiomeDatabase.BaseEntry.getForestness(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:258)
- [ ] `BiomeDatabase.BaseEntry.getHeightmap(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:263)
- [ ] `BiomeDatabase.BaseEntry.setMultipass(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:268)
- [ ] `BiomeDatabase.BaseEntry.getMultipass(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:272)
- [ ] `BiomeDatabase.BaseEntry.enableDefaultMultipass(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:277)
- [ ] `BiomeDatabase.BaseEntry.setCustomMultipass(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:288)
- [ ] `BiomeDatabase.BaseEntry.deserialiseCustomMultipass(1)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:293)
- [ ] `BiomeDatabase.BaseEntry.reset(0)`: its class isn't translated (it implements `EntryReader`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:312)

### `BiomeDatabase.CaveRootedData`: it uses SpeciesSelector, which isn't translated (it's an interface)

- [ ] `BiomeDatabase.CaveRootedData.shouldGenerateOnSurface(0)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:329)
- [ ] `BiomeDatabase.CaveRootedData.setGenerateOnSurface(1)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:333)
- [ ] `BiomeDatabase.CaveRootedData.getMaxDistToSurface(0)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:337)
- [ ] `BiomeDatabase.CaveRootedData.setMaxDistToSurface(1)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:341)
- [ ] `BiomeDatabase.CaveRootedData.getCaveRootedSpeciesSelector(0)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:345)
- [ ] `BiomeDatabase.CaveRootedData.getCaveRootedChanceSelector(0)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:349)
- [ ] `BiomeDatabase.CaveRootedData.setCaveRootedSpeciesSelector(2)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:353)
- [ ] `BiomeDatabase.CaveRootedData.setCaveRootedChanceSelector(2)`: its class isn't translated (it uses SpeciesSelector, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:373)

### `BiomeDatabase.Entry`: it extends `BaseEntry`

- [ ] `BiomeDatabase.Entry.Entry(2)`: its class isn't translated (it extends `BaseEntry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:397)
- [ ] `BiomeDatabase.Entry.getCaveRootedData(0)`: its class isn't translated (it extends `BaseEntry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:401)
- [ ] `BiomeDatabase.Entry.hasCaveRootedData(0)`: its class isn't translated (it extends `BaseEntry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:406)
- [ ] `BiomeDatabase.Entry.getOrCreateCaveRootedData(0)`: its class isn't translated (it extends `BaseEntry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:410)
- [ ] `BiomeDatabase.Entry.setCaveRootedData(1)`: its class isn't translated (it extends `BaseEntry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:418)
- [ ] `BiomeDatabase.Entry.reset(0)`: its class isn't translated (it extends `BaseEntry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:422)

### `BiomeDatabase.EntryReader`: it's an interface

- [ ] `BiomeDatabase.EntryReader.read(1)`: it uses Minecraft's `DataResult` (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:430)
- [ ] `BiomeDatabase.EntryReader.getBiomeKey(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:435)
- [ ] `BiomeDatabase.EntryReader.getChanceSelector(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:438)
- [ ] `BiomeDatabase.EntryReader.getDensitySelector(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:440)
- [ ] `BiomeDatabase.EntryReader.getSpeciesSelector(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:442)
- [ ] `BiomeDatabase.EntryReader.getFeatureCancellation(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:444)
- [ ] `BiomeDatabase.EntryReader.isBlacklisted(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:446)
- [ ] `BiomeDatabase.EntryReader.getForestness(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:448)
- [ ] `BiomeDatabase.EntryReader.getHeightmap(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:450)
- [ ] `BiomeDatabase.EntryReader.getMultipass(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:452)

### `BiomeDatabase.JsonEntry`: it extends `Entry`

- [ ] `BiomeDatabase.JsonEntry.JsonEntry(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:468)
- [ ] `BiomeDatabase.JsonEntry.isForce(0)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:472)
- [ ] `BiomeDatabase.JsonEntry.setForce(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:476)
- [ ] `BiomeDatabase.JsonEntry.copyTo(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:480)
- [ ] `BiomeDatabase.JsonEntry.setChanceSelector(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:518)
- [ ] `BiomeDatabase.JsonEntry.setChanceSelector(2)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:524)
- [ ] `BiomeDatabase.JsonEntry.setDensitySelector(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:531)
- [ ] `BiomeDatabase.JsonEntry.setDensitySelector(2)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:537)
- [ ] `BiomeDatabase.JsonEntry.setSpeciesSelector(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:544)
- [ ] `BiomeDatabase.JsonEntry.setSpeciesSelector(2)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:550)
- [ ] `BiomeDatabase.JsonEntry.setBlacklisted(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:557)
- [ ] `BiomeDatabase.JsonEntry.setForestness(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:563)
- [ ] `BiomeDatabase.JsonEntry.setHeightmap(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:569)
- [ ] `BiomeDatabase.JsonEntry.setMultipass(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:575)
- [ ] `BiomeDatabase.JsonEntry.setCustomMultipass(1)`: its class isn't translated (it extends `Entry`) (com/dtteam/dynamictrees/worldgen/BiomeDatabase.java:581)

### `BiomeDatabase.Operation`


### `BiomeDatabases`

- [ ] `BiomeDatabases.DEFAULT_DATABASE`: it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:15)
- [ ] `BiomeDatabases.DIMENSIONAL_DATABASES`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:16)
- [ ] `BiomeDatabases.BLACKLIST`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:17)
- [ ] `BiomeDatabases.getDefault(0)`: it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:19)
- [ ] `BiomeDatabases.getDimensionalOrDefault(1)`: it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:23)
- [ ] `BiomeDatabases.getOrCreateDimensional(1)`: it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:27)
- [ ] `BiomeDatabases.getDimensionalDatabases(0)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:31)
- [ ] `BiomeDatabases.isBlacklisted(1)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:35)
- [ ] `BiomeDatabases.populateBlacklistFromConfig(0)`: it uses DTConfigs.SERVER_CONFIG, which isn't translated (it uses `ModConfigSpec` (net.neoforged.neoforge.common)) (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:39)
- [ ] `BiomeDatabases.tryBlacklist(1)`: a try/catch (class_151) (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:47)
- [ ] `BiomeDatabases.reset(0)`: it uses BiomeDatabases.DEFAULT_DATABASE, which isn't translated (it uses BiomeDatabase, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface))) (com/dtteam/dynamictrees/worldgen/BiomeDatabases.java:55)

### `BiomeGenSettingsBuilderWrapper`

- [ ] `BiomeGenSettingsBuilderWrapper.getPlainBuilder(0)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeGenSettingsBuilderWrapper.java:12)
- [ ] `BiomeGenSettingsBuilderWrapper.getFeatures(1)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeGenSettingsBuilderWrapper.java:14)
- [ ] `BiomeGenSettingsBuilderWrapper.getCarvers(1)`: it has no body (com/dtteam/dynamictrees/worldgen/BiomeGenSettingsBuilderWrapper.java:16)

### `BiomeRadiusCoordinator`: it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)

- [ ] `BiomeRadiusCoordinator.BiomeRadiusCoordinator(2)`: its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/BiomeRadiusCoordinator.java:20)
- [ ] `BiomeRadiusCoordinator.getRadiusAtCoords(2)`: its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/BiomeRadiusCoordinator.java:25)
- [ ] `BiomeRadiusCoordinator.calcDensity(2)`: its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/BiomeRadiusCoordinator.java:36)
- [ ] `BiomeRadiusCoordinator.calcRadius(3)`: its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/BiomeRadiusCoordinator.java:44)
- [ ] `BiomeRadiusCoordinator.runPass(3)`: its class isn't translated (it implements `RadiusCoordinator` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/BiomeRadiusCoordinator.java:52)

### `DynamicTreeGenerationContext`: it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)

- [ ] `DynamicTreeGenerationContext.DynamicTreeGenerationContext(8)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:25)
- [ ] `DynamicTreeGenerationContext.levelContext(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:45)
- [ ] `DynamicTreeGenerationContext.level(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:49)
- [ ] `DynamicTreeGenerationContext.random(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:53)
- [ ] `DynamicTreeGenerationContext.species(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:57)
- [ ] `DynamicTreeGenerationContext.originPos(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:61)
- [ ] `DynamicTreeGenerationContext.rootPos(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:65)
- [ ] `DynamicTreeGenerationContext.biome(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:69)
- [ ] `DynamicTreeGenerationContext.facing(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:73)
- [ ] `DynamicTreeGenerationContext.radius(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:77)
- [ ] `DynamicTreeGenerationContext.isWorldGen(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:81)
- [ ] `DynamicTreeGenerationContext.secondChanceRegen(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:85)
- [ ] `DynamicTreeGenerationContext.secondChance(0)`: its class isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/DynamicTreeGenerationContext.java:89)

### `FabricBiomeModifications`

- [ ] `FabricBiomeModifications.REMOVE_TREES_ID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/FabricBiomeModifications.java:29)
- [ ] `FabricBiomeModifications.ADD_TREES_ID`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/FabricBiomeModifications.java:30)
- [ ] `FabricBiomeModifications.FEATURE_CANCELLER_EXCLUSIONS_KEY`: it uses Minecraft's `class_6862` (com/dtteam/dynamictrees/worldgen/FabricBiomeModifications.java:31)
- [ ] `FabricBiomeModifications.register(0)`: it uses FabricBiomeModifications.REMOVE_TREES_ID, which isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/worldgen/FabricBiomeModifications.java:35)
- [ ] `FabricBiomeModifications.removeVanillaTrees(2)`: it uses Fabric's `BiomeSelectionContext` (com/dtteam/dynamictrees/worldgen/FabricBiomeModifications.java:48)
- [ ] `FabricBiomeModifications.addDynamicTrees(1)`: it uses Fabric's `BiomeModificationContext` (com/dtteam/dynamictrees/worldgen/FabricBiomeModifications.java:94)

### `FeatureCancellationRegistry`

- [ ] `FeatureCancellationRegistry.CANCELLATIONS`: it uses Entry, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java:10)
- [ ] `FeatureCancellationRegistry.addCancellations(3)`: it uses IDTBiomeHolderSet, which isn't translated (it's an interface) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java:12)
- [ ] `FeatureCancellationRegistry.getCancellations(0)`: it uses Entry, which isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java:18)

### `FeatureCancellationRegistry.Entry`: it uses IDTBiomeHolderSet, which isn't translated (it's an interface)

- [ ] `FeatureCancellationRegistry.Entry.Entry(3)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java:22)
- [ ] `FeatureCancellationRegistry.Entry.biomes(0)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java:22)
- [ ] `FeatureCancellationRegistry.Entry.operation(0)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java:22)
- [ ] `FeatureCancellationRegistry.Entry.cancellations(0)`: its class isn't translated (it uses IDTBiomeHolderSet, which isn't translated (it's an interface)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellationRegistry.java:22)

### `FeatureCancellers`

- [ ] `FeatureCancellers.TREE_CANCELLER`: it uses FeatureCanceller, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellers.java:11)
- [ ] `FeatureCancellers.ROOTED_TREE_CANCELLER`: it uses FeatureCanceller, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellers.java:12)
- [ ] `FeatureCancellers.FUNGUS_CANCELLER`: it uses FeatureCanceller, which isn't translated (it extends `RegistryEntry` (com.dtteam.dynamictrees.api.registry)) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellers.java:13)
- [ ] `FeatureCancellers.register(1)`: it uses Registry, which isn't translated (it's an interface) (com/dtteam/dynamictrees/worldgen/featurecancellation/FeatureCancellers.java:15)

### `FungusFeatureCanceller`: it's generic

- [ ] `FungusFeatureCanceller.FungusFeatureCanceller(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/FungusFeatureCanceller.java:13)
- [ ] `FungusFeatureCanceller.shouldCancel(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/FungusFeatureCanceller.java:18)

### `MushroomFeatureCanceller`: it's generic

- [ ] `MushroomFeatureCanceller.MushroomFeatureCanceller(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/MushroomFeatureCanceller.java:15)
- [ ] `MushroomFeatureCanceller.shouldCancel(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/MushroomFeatureCanceller.java:20)
- [ ] `MushroomFeatureCanceller.getConfigs(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/MushroomFeatureCanceller.java:33)

### `TreeFeatureCanceller`: it's generic

- [ ] `TreeFeatureCanceller.TreeFeatureCanceller(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/TreeFeatureCanceller.java:16)
- [ ] `TreeFeatureCanceller.shouldCancel(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/TreeFeatureCanceller.java:21)
- [ ] `TreeFeatureCanceller.doesContainTrees(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/featurecancellation/TreeFeatureCanceller.java:49)

### `CaveRootedTreeFeature`: it extends `DynamicTreeFeature`

- [ ] `CaveRootedTreeFeature.method_13151(1)`: its class isn't translated (it extends `DynamicTreeFeature`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreeFeature.java:22)
- [ ] `CaveRootedTreeFeature.getSpeciesSelector(1)`: its class isn't translated (it extends `DynamicTreeFeature`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreeFeature.java:71)
- [ ] `CaveRootedTreeFeature.getChanceSelector(1)`: its class isn't translated (it extends `DynamicTreeFeature`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreeFeature.java:78)
- [ ] `CaveRootedTreeFeature.getDisc(3)`: its class isn't translated (it extends `DynamicTreeFeature`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreeFeature.java:85)
- [ ] `CaveRootedTreeFeature.getNextGroundPos(2)`: its class isn't translated (it extends `DynamicTreeFeature`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreeFeature.java:92)

### `CaveRootedTreePlacement`: it extends Minecraft's `class_6797`

- [ ] `CaveRootedTreePlacement.INSTANCE`: it uses CaveRootedTreePlacement, which isn't translated (it extends Minecraft's `class_6797`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreePlacement.java:18)
- [ ] `CaveRootedTreePlacement.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreePlacement.java:19)
- [ ] `CaveRootedTreePlacement.CaveRootedTreePlacement(1)`: its class isn't translated (it extends Minecraft's `class_6797`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreePlacement.java:23)
- [ ] `CaveRootedTreePlacement.method_14452(3)`: its class isn't translated (it extends Minecraft's `class_6797`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreePlacement.java:26)
- [ ] `CaveRootedTreePlacement.method_39615(0)`: its class isn't translated (it extends Minecraft's `class_6797`) (com/dtteam/dynamictrees/worldgen/feature/CaveRootedTreePlacement.java:33)

### `DTReplaceNyliumFungiBlockStateProvider`: it extends Minecraft's `class_4651`

- [ ] `DTReplaceNyliumFungiBlockStateProvider.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/worldgen/feature/DTReplaceNyliumFungiBlockStateProvider.java:14)
- [ ] `DTReplaceNyliumFungiBlockStateProvider.DTReplaceNyliumFungiBlockStateProvider(2)`: its class isn't translated (it extends Minecraft's `class_4651`) (com/dtteam/dynamictrees/worldgen/feature/DTReplaceNyliumFungiBlockStateProvider.java:24)
- [ ] `DTReplaceNyliumFungiBlockStateProvider.method_28862(0)`: its class isn't translated (it extends Minecraft's `class_4651`) (com/dtteam/dynamictrees/worldgen/feature/DTReplaceNyliumFungiBlockStateProvider.java:29)
- [ ] `DTReplaceNyliumFungiBlockStateProvider.method_23455(2)`: its class isn't translated (it extends Minecraft's `class_4651`) (com/dtteam/dynamictrees/worldgen/feature/DTReplaceNyliumFungiBlockStateProvider.java:33)

### `DynamicTreeFeature`: it extends Minecraft's `class_3031`

- [ ] `DynamicTreeFeature.DISC_PROVIDER`: it uses UniversalPoissonDiscProvider, which isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:37)
- [ ] `DynamicTreeFeature.RANDOM`: it uses RandomXOR, which isn't translated (it extends Minecraft's `class_7384`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:38)
- [ ] `DynamicTreeFeature.concreteBlocks`: it uses Minecraft's `class_2248` (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:39)
- [ ] `DynamicTreeFeature.setup(0)`: it uses DynamicTreeFeature.concreteBlocks, which isn't translated (it uses Minecraft's `class_2248`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:41)
- [ ] `DynamicTreeFeature.DynamicTreeFeature(0)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:47)
- [ ] `DynamicTreeFeature.method_13151(1)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:51)
- [ ] `DynamicTreeFeature.generateTrees(4)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:63)
- [ ] `DynamicTreeFeature.getNoiseBiome(2)`: it uses Minecraft's `class_6880` (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:74)
- [ ] `DynamicTreeFeature.validTreePos(2)`: it uses Minecraft's `class_3746` (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:80)
- [ ] `DynamicTreeFeature.isFoliage(2)`: it uses Minecraft's `class_3746` (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:84)
- [ ] `DynamicTreeFeature.generateTree(5)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:88)
- [ ] `DynamicTreeFeature.genTree(5)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:108)
- [ ] `DynamicTreeFeature.getSpeciesSelector(1)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:142)
- [ ] `DynamicTreeFeature.getChanceSelector(1)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:146)
- [ ] `DynamicTreeFeature.generateConcreteCircle(4)`: its class isn't translated (it extends Minecraft's `class_3031`) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:150)

### `DynamicTreeFeature.GeneratorResult`: its constants carry values

- [ ] `DynamicTreeFeature.GeneratorResult.GeneratorResult(1)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:178)
- [ ] `DynamicTreeFeature.GeneratorResult.getColor(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:182)
- [ ] `DynamicTreeFeature.GeneratorResult.getColoredBlock(0)`: its class isn't translated (its constants carry values) (com/dtteam/dynamictrees/worldgen/feature/DynamicTreeFeature.java:186)

### `DelayedHolderSet`: it's generic

- [ ] `DelayedHolderSet.DelayedHolderSet(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:21)
- [ ] `DelayedHolderSet.method_40239(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:25)
- [ ] `DelayedHolderSet.method_40247(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:29)
- [ ] `DelayedHolderSet.method_40248(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:33)
- [ ] `DelayedHolderSet.method_40243(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:37)
- [ ] `DelayedHolderSet.method_40240(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:41)
- [ ] `DelayedHolderSet.method_40241(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:45)
- [ ] `DelayedHolderSet.method_46768(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:49)
- [ ] `DelayedHolderSet.method_45925(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:53)
- [ ] `DelayedHolderSet.iterator(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:57)
- [ ] `DelayedHolderSet.forEach(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:62)
- [ ] `DelayedHolderSet.spliterator(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/DelayedHolderSet.java:66)

### `DTBiomeHolderSet`: it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)

- [ ] `DTBiomeHolderSet.getSet(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:27)
- [ ] `DTBiomeHolderSet.containsKey(1)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:52)
- [ ] `DTBiomeHolderSet.method_40239(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:64)
- [ ] `DTBiomeHolderSet.method_40247(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:69)
- [ ] `DTBiomeHolderSet.method_40248(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:73)
- [ ] `DTBiomeHolderSet.method_40243(1)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:77)
- [ ] `DTBiomeHolderSet.method_40240(1)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:93)
- [ ] `DTBiomeHolderSet.method_40241(1)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:103)
- [ ] `DTBiomeHolderSet.method_46768(1)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:107)
- [ ] `DTBiomeHolderSet.method_45925(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:111)
- [ ] `DTBiomeHolderSet.getIncludeComponents(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:115)
- [ ] `DTBiomeHolderSet.getExcludeComponents(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:120)
- [ ] `DTBiomeHolderSet.addHolderSet(2)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:125)
- [ ] `DTBiomeHolderSet.addDelayedHolderSet(2)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:130)
- [ ] `DTBiomeHolderSet.addNameRegexMatch(3)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:135)
- [ ] `DTBiomeHolderSet.addTagsRegexMatch(3)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:141)
- [ ] `DTBiomeHolderSet.addOr(2)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:147)
- [ ] `DTBiomeHolderSet.clear(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:152)
- [ ] `DTBiomeHolderSet.iterator(0)`: its class isn't translated (it implements `IDTBiomeHolderSet` (com.dtteam.dynamictrees.worldgen)) (com/dtteam/dynamictrees/worldgen/holderset/DTBiomeHolderSet.java:158)

### `NameRegexMatchHolderSet`: it's generic

- [ ] `NameRegexMatchHolderSet.NameRegexMatchHolderSet(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/NameRegexMatchHolderSet.java:8)
- [ ] `NameRegexMatchHolderSet.getInput(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/NameRegexMatchHolderSet.java:12)

### `OrHolderSet`: it's generic

- [ ] `OrHolderSet.OrHolderSet(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/OrHolderSet.java:12)
- [ ] `OrHolderSet.method_40239(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/OrHolderSet.java:16)
- [ ] `OrHolderSet.method_46768(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/OrHolderSet.java:20)

### `RegexMatchHolderSet`: it's generic

- [ ] `RegexMatchHolderSet.RegexMatchHolderSet(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java:14)
- [ ] `RegexMatchHolderSet.registryLookup(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java:19)
- [ ] `RegexMatchHolderSet.regex(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java:23)
- [ ] `RegexMatchHolderSet.getPattern(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java:27)
- [ ] `RegexMatchHolderSet.method_40239(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java:35)
- [ ] `RegexMatchHolderSet.method_46768(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java:41)
- [ ] `RegexMatchHolderSet.getInput(1)`: it has no body (com/dtteam/dynamictrees/worldgen/holderset/RegexMatchHolderSet.java:45)

### `StreamBackedHolderSet`: it's generic

- [ ] `StreamBackedHolderSet.contents(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:18)
- [ ] `StreamBackedHolderSet.contentsSet(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:22)
- [ ] `StreamBackedHolderSet.method_40247(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:26)
- [ ] `StreamBackedHolderSet.spliterator(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:30)
- [ ] `StreamBackedHolderSet.iterator(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:34)
- [ ] `StreamBackedHolderSet.method_40243(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:38)
- [ ] `StreamBackedHolderSet.method_40240(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:42)
- [ ] `StreamBackedHolderSet.method_40248(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:46)
- [ ] `StreamBackedHolderSet.method_40241(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:50)
- [ ] `StreamBackedHolderSet.method_45925(0)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/StreamBackedHolderSet.java:54)

### `TagsRegexMatchHolderSet`: it's generic

- [ ] `TagsRegexMatchHolderSet.TagsRegexMatchHolderSet(2)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/TagsRegexMatchHolderSet.java:8)
- [ ] `TagsRegexMatchHolderSet.getInput(1)`: its class isn't translated (it's generic) (com/dtteam/dynamictrees/worldgen/holderset/TagsRegexMatchHolderSet.java:12)

### `IDTBiomeHolderSet`: it's an interface

- [ ] `IDTBiomeHolderSet.containsKey(1)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:13)
- [ ] `IDTBiomeHolderSet.method_40239(0)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:15)
- [ ] `IDTBiomeHolderSet.getIncludeComponents(0)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:17)
- [ ] `IDTBiomeHolderSet.getExcludeComponents(0)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:19)
- [ ] `IDTBiomeHolderSet.addHolderSet(2)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:21)
- [ ] `IDTBiomeHolderSet.addDelayedHolderSet(2)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:23)
- [ ] `IDTBiomeHolderSet.addNameRegexMatch(3)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:25)
- [ ] `IDTBiomeHolderSet.addTagsRegexMatch(3)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:27)
- [ ] `IDTBiomeHolderSet.addOr(2)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:29)
- [ ] `IDTBiomeHolderSet.clear(0)`: it has no body (com/dtteam/dynamictrees/worldgen/IDTBiomeHolderSet.java:31)

### `JoCode`

- [x] `JoCode.LOGGER` (com/dtteam/dynamictrees/worldgen/JoCode.java:47)
- [x] `JoCode.BASE_64` (com/dtteam/dynamictrees/worldgen/JoCode.java:48)
- [x] `JoCode.FORK_CODE` (com/dtteam/dynamictrees/worldgen/JoCode.java:49)
- [x] `JoCode.RETURN_CODE` (com/dtteam/dynamictrees/worldgen/JoCode.java:50)
- [ ] `JoCode.JoCode(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/worldgen/JoCode.java:64)
- [ ] `JoCode.getCodeFromWorld(3)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/worldgen/JoCode.java:68)
- [ ] `JoCode.JoCode(2)`: it uses Minecraft's `class_1937` (com/dtteam/dynamictrees/worldgen/JoCode.java:78)
- [ ] `JoCode.JoCode(1)`: it calls JoCode.decode, which isn't translated (com/dtteam/dynamictrees/worldgen/JoCode.java:82)
- [ ] `JoCode.setCareful(1)`: `this` used as a value (com/dtteam/dynamictrees/worldgen/JoCode.java:86)
- [x] `JoCode.getCode(1)` (com/dtteam/dynamictrees/worldgen/JoCode.java:91)
- [ ] `JoCode.setFacing(1)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/worldgen/JoCode.java:95)
- [ ] `JoCode.rotate(1)`: it uses Minecraft's `class_2350` (com/dtteam/dynamictrees/worldgen/JoCode.java:103)
- [ ] `JoCode.generate(1)`: it uses DynamicTreeGenerationContext, which isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/JoCode.java:113)
- [ ] `JoCode.generateAndAgeLeaves(3)`: it uses DynamicTreeGenerationContext, which isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/JoCode.java:157)
- [ ] `JoCode.tryGenerateAgain(5)`: it uses DynamicTreeGenerationContext, which isn't translated (it uses LevelContext, which isn't translated (it uses Minecraft's `class_5321`)) (com/dtteam/dynamictrees/worldgen/JoCode.java:174)
- [ ] `JoCode.cleanupFrankentree(5)`: its body couldn't be read: an assert (com/dtteam/dynamictrees/worldgen/JoCode.java:193)
- [ ] `JoCode.generateFork(5)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/worldgen/JoCode.java:235)
- [ ] `JoCode.setBlockForGeneration(6)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/worldgen/JoCode.java:258)
- [ ] `JoCode.isFreeToSetBlock(3)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/worldgen/JoCode.java:269)
- [ ] `JoCode.isReplaceable(2)`: it uses Minecraft's `class_2680` (com/dtteam/dynamictrees/worldgen/JoCode.java:278)
- [ ] `JoCode.smother(2)`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/worldgen/JoCode.java:283)
- [ ] `JoCode.isClearOfNearbyBranches(3)`: it uses Minecraft's `class_1936` (com/dtteam/dynamictrees/worldgen/JoCode.java:318)
- [ ] `JoCode.addSnow(4)`: it uses SimpleVoxmap, which isn't translated (it uses Minecraft's `class_2338`) (com/dtteam/dynamictrees/worldgen/JoCode.java:328)
- [ ] `JoCode.encode(1)`: it changes a for-each variable (com/dtteam/dynamictrees/worldgen/JoCode.java:357)
- [ ] `JoCode.decode(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/worldgen/JoCode.java:377)
- [ ] `JoCode.toString(0)`: it calls JoCode.encode, which isn't translated (com/dtteam/dynamictrees/worldgen/JoCode.java:381)
- [ ] `JoCode.getTextComponent(0)`: it uses Minecraft's `class_2561` (com/dtteam/dynamictrees/worldgen/JoCode.java:386)

### `JoCode.CodeCompiler`

- [x] `JoCode.CodeCompiler.CodeCompiler(0)` (com/dtteam/dynamictrees/worldgen/JoCode.java:398)
- [ ] `JoCode.CodeCompiler.CodeCompiler(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/worldgen/JoCode.java:402)
- [ ] `JoCode.CodeCompiler.CodeCompiler(1)`: its constructors are overloaded with the same number of parameters (com/dtteam/dynamictrees/worldgen/JoCode.java:406)
- [x] `JoCode.CodeCompiler.addDirection(1)` (com/dtteam/dynamictrees/worldgen/JoCode.java:418)
- [x] `JoCode.CodeCompiler.addInstruction(1)` (com/dtteam/dynamictrees/worldgen/JoCode.java:424)
- [x] `JoCode.CodeCompiler.addReturn(0)` (com/dtteam/dynamictrees/worldgen/JoCode.java:428)
- [x] `JoCode.CodeCompiler.addFork(0)` (com/dtteam/dynamictrees/worldgen/JoCode.java:432)
- [ ] `JoCode.CodeCompiler.compile(0)`: `++` inside an expression (com/dtteam/dynamictrees/worldgen/JoCode.java:436)

### `JoCodeRegistry`

- [ ] `JoCodeRegistry.CODES`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:13)
- [ ] `JoCodeRegistry.ROOTS_CODES`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:14)
- [x] `JoCodeRegistry.JoCodeRegistry(0)` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:16)
- [ ] `JoCodeRegistry.clear(0)`: it uses JoCodeRegistry.CODES, which isn't translated (it uses Minecraft's `class_2960`) (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:19)
- [ ] `JoCodeRegistry.register(3)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:24)
- [ ] `JoCodeRegistry.registerRoot(3)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:28)
- [ ] `JoCodeRegistry.getCodes(2)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:32)
- [ ] `JoCodeRegistry.getCodes(2)`: it's overloaded with the same number of parameters (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:36)
- [ ] `JoCodeRegistry.getCodes(3)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:40)
- [ ] `JoCodeRegistry.getRandomCode(3)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:44)
- [ ] `JoCodeRegistry.getRandomCode(4)`: it uses Minecraft's `class_2960` (com/dtteam/dynamictrees/worldgen/JoCodeRegistry.java:49)

### `OverworldGroundFinder`: it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)

- [ ] `OverworldGroundFinder.findGround(3)`: its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/OverworldGroundFinder.java:17)

### `RootsJoCode`: it extends `JoCode`

- [ ] `RootsJoCode.RootsJoCode(1)`: its class isn't translated (it extends `JoCode`) (com/dtteam/dynamictrees/worldgen/RootsJoCode.java:22)
- [ ] `RootsJoCode.RootsJoCode(3)`: its class isn't translated (it extends `JoCode`) (com/dtteam/dynamictrees/worldgen/RootsJoCode.java:26)
- [ ] `RootsJoCode.getCodeFromWorld(3)`: its class isn't translated (it extends `JoCode`) (com/dtteam/dynamictrees/worldgen/RootsJoCode.java:30)
- [ ] `RootsJoCode.generate(1)`: its class isn't translated (it extends `JoCode`) (com/dtteam/dynamictrees/worldgen/RootsJoCode.java:41)
- [ ] `RootsJoCode.setBlockForGeneration(6)`: its class isn't translated (it extends `JoCode`) (com/dtteam/dynamictrees/worldgen/RootsJoCode.java:80)
- [ ] `RootsJoCode.isFreeToSetBlock(3)`: its class isn't translated (it extends `JoCode`) (com/dtteam/dynamictrees/worldgen/RootsJoCode.java:93)

### `DTCancelVanillaTreePoolElement`: it extends Minecraft's `class_3784`

- [ ] `DTCancelVanillaTreePoolElement.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:25)
- [ ] `DTCancelVanillaTreePoolElement.DTCancelVanillaTreePoolElement(2)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:35)
- [ ] `DTCancelVanillaTreePoolElement.create(2)`: it uses Minecraft's `class_3786` (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:41)
- [ ] `DTCancelVanillaTreePoolElement.isEnabled(0)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:47)
- [ ] `DTCancelVanillaTreePoolElement.method_16601(2)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:51)
- [ ] `DTCancelVanillaTreePoolElement.method_16627(4)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:55)
- [ ] `DTCancelVanillaTreePoolElement.method_16628(3)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:61)
- [ ] `DTCancelVanillaTreePoolElement.method_16626(11)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:67)
- [ ] `DTCancelVanillaTreePoolElement.method_16624(0)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:87)
- [ ] `DTCancelVanillaTreePoolElement.method_16622(1)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:91)
- [ ] `DTCancelVanillaTreePoolElement.method_16757(0)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/DTCancelVanillaTreePoolElement.java:97)

### `RegularTemplatePoolModifier`: it implements `TemplatePoolModifier`

- [ ] `RegularTemplatePoolModifier.RegularTemplatePoolModifier(2)`: its class isn't translated (it implements `TemplatePoolModifier`) (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java:16)
- [ ] `RegularTemplatePoolModifier.replaceTemplate(2)`: its class isn't translated (it implements `TemplatePoolModifier`) (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java:21)
- [ ] `RegularTemplatePoolModifier.removeTemplate(1)`: its class isn't translated (it implements `TemplatePoolModifier`) (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java:34)
- [ ] `RegularTemplatePoolModifier.removeAllTemplates(0)`: its class isn't translated (it implements `TemplatePoolModifier`) (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java:41)
- [ ] `RegularTemplatePoolModifier.registerPool(1)`: its class isn't translated (it implements `TemplatePoolModifier`) (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java:47)
- [ ] `RegularTemplatePoolModifier.village(3)`: it uses TemplatePoolModifier, which isn't translated (it's an interface) (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java:52)
- [ ] `RegularTemplatePoolModifier.create(2)`: it uses TemplatePoolModifier, which isn't translated (it's an interface) (com/dtteam/dynamictrees/worldgen/structure/RegularTemplatePoolModifier.java:57)

### `TemplatePoolModifier`: it's an interface

- [ ] `TemplatePoolModifier.replaceTemplate(2)`: it has no body (com/dtteam/dynamictrees/worldgen/structure/TemplatePoolModifier.java:8)
- [ ] `TemplatePoolModifier.removeTemplate(1)`: it has no body (com/dtteam/dynamictrees/worldgen/structure/TemplatePoolModifier.java:10)
- [ ] `TemplatePoolModifier.removeAllTemplates(0)`: it has no body (com/dtteam/dynamictrees/worldgen/structure/TemplatePoolModifier.java:12)
- [ ] `TemplatePoolModifier.registerPool(1)`: it has no body (com/dtteam/dynamictrees/worldgen/structure/TemplatePoolModifier.java:14)

### `TreePoolElement`: it extends Minecraft's `class_3784`

- [ ] `TreePoolElement.CODEC`: it uses Minecraft's `MapCodec` (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:36)
- [ ] `TreePoolElement.TreePoolElement(2)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:48)
- [ ] `TreePoolElement.TreePoolElement(3)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:52)
- [ ] `TreePoolElement.create(1)`: it uses Minecraft's `class_3786` (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:59)
- [ ] `TreePoolElement.create(2)`: it uses Minecraft's `class_3786` (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:63)
- [ ] `TreePoolElement.fillDefaultJigsawNBT(0)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:67)
- [ ] `TreePoolElement.method_16627(4)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:77)
- [ ] `TreePoolElement.method_16628(3)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:91)
- [ ] `TreePoolElement.method_16601(2)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:95)
- [ ] `TreePoolElement.method_16626(11)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:99)
- [ ] `TreePoolElement.getOffsetX(1)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:116)
- [ ] `TreePoolElement.getOffsetY(1)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:120)
- [ ] `TreePoolElement.getOffsetZ(1)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:124)
- [ ] `TreePoolElement.method_16757(0)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:128)
- [ ] `TreePoolElement.getSpecies(0)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:132)
- [ ] `TreePoolElement.getOffset(0)`: its class isn't translated (it extends Minecraft's `class_3784`) (com/dtteam/dynamictrees/worldgen/structure/TreePoolElement.java:136)

### `VillageTreeReplacement`

- [x] `VillageTreeReplacement.LOGGER` (com/dtteam/dynamictrees/worldgen/structure/VillageTreeReplacement.java:22)
- [ ] `VillageTreeReplacement.REPLACEMENT_TOWN_CENTER_ID`: it calls DynamicTrees.location, which isn't translated (com/dtteam/dynamictrees/worldgen/structure/VillageTreeReplacement.java:23)
- [ ] `VillageTreeReplacement.replaceTreesFromVanillaVillages(2)`: it uses Minecraft's `class_7874` (com/dtteam/dynamictrees/worldgen/structure/VillageTreeReplacement.java:25)

### `SubterraneanGroundFinder`: it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)

- [ ] `SubterraneanGroundFinder.NO_LAYERS`: it uses Minecraft's `class_2338` (com/dtteam/dynamictrees/worldgen/SubterraneanGroundFinder.java:21)
- [ ] `SubterraneanGroundFinder.isReplaceable(2)`: its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/SubterraneanGroundFinder.java:23)
- [ ] `SubterraneanGroundFinder.getTopY(2)`: its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/SubterraneanGroundFinder.java:27)
- [ ] `SubterraneanGroundFinder.findSubterraneanLayerHeights(2)`: its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/SubterraneanGroundFinder.java:31)
- [ ] `SubterraneanGroundFinder.findGround(3)`: its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/SubterraneanGroundFinder.java:64)

### `SurfaceGroundFinder`: it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)

- [ ] `SurfaceGroundFinder.findGround(3)`: its class isn't translated (it implements `GroundFinder` (com.dtteam.dynamictrees.api.worldgen)) (com/dtteam/dynamictrees/worldgen/SurfaceGroundFinder.java:13)
