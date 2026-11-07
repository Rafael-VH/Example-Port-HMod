// Storage Drawers, ported to Horizon by horizon-port from StorageDrawers-fabric-26.3.0.1.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_storagedrawers). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "storagedrawers";
pub const NAME: &str = "Storage Drawers";
pub const VERSION: &str = "26.3.0.1";
pub const DESCRIPTION: &str = "Interactive compartment storage for your workshops";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[];

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `IDrawerCapability` (com/jaquadro/minecraft/storagedrawers/api/capabilities/IDrawerCapability.java)
    pub mod i_drawer_capability {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerCapability.getCapability(2): not translated, it has no body.
    }

    /// `IDrawerCapabilityProvider` (com/jaquadro/minecraft/storagedrawers/api/capabilities/IDrawerCapabilityProvider.java)
    pub mod i_drawer_capability_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerCapabilityProvider.getCapability(1): not translated, it's generic.
    }

    /// `IItemHandler` (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemHandler.java)
    pub mod i_item_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IItemHandler.getSlots(0): not translated, it has no body.
        // IItemHandler.getStackInSlot(1): not translated, it has no body.
        // IItemHandler.insertItem(3): not translated, it has no body.
        // IItemHandler.extractItem(3): not translated, it has no body.
        // IItemHandler.getSlotLimit(1): not translated, it has no body.
        // IItemHandler.isItemValid(2): not translated, it has no body.
    }

    /// `IItemRepository` (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java)
    pub mod i_item_repository {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IItemRepository.getAllItems(0): not translated, it has no body.
        // IItemRepository.insertItem(3): not translated, it has no body.
        // IItemRepository.insertItem(2): not translated, its class isn't translated (it's an interface).
        // IItemRepository.extractItem(4): not translated, it has no body.
        // IItemRepository.extractItem(3): not translated, its class isn't translated (it's an interface).
        // IItemRepository.getStoredItemCount(2): not translated, its class isn't translated (it's an interface).
        // IItemRepository.getStoredItemCount(1): not translated, its class isn't translated (it's an interface).
        // IItemRepository.getRemainingItemCapacity(2): not translated, its class isn't translated (it's an interface).
        // IItemRepository.getRemainingItemCapacity(1): not translated, its class isn't translated (it's an interface).
        // IItemRepository.getItemCapacity(2): not translated, its class isn't translated (it's an interface).
        // IItemRepository.getItemCapacity(1): not translated, its class isn't translated (it's an interface).
    }

    /// `IItemRepository.DefaultPredicate` (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java)
    pub mod i_item_repository_default_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `IItemRepository.ItemRecord` (com/jaquadro/minecraft/storagedrawers/api/capabilities/IItemRepository.java)
    pub mod i_item_repository_item_record {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // IItemRepository.ItemRecord.ItemRecord(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `IDrawerConfig` (com/jaquadro/minecraft/storagedrawers/api/config/IDrawerConfig.java)
    pub mod i_drawer_config {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerConfig.getUnitsPerSlot(0): not translated, it has no body.
    }

    /// `FrameMaterial` (com/jaquadro/minecraft/storagedrawers/api/framing/FrameMaterial.java)
    pub mod frame_material {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // FrameMaterial.FrameMaterial(2): not translated, its class isn't translated (its constants carry values).
        // FrameMaterial.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // FrameMaterial.getTagKey(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `IFramedBlock` (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedBlock.java)
    pub mod i_framed_block {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFramedBlock.getFramedBlockEntity(2): not translated, it has no body.
        // IFramedBlock.supportsFrameMaterial(1): not translated, it has no body.
        // IFramedBlock.getStateDefinition(0): not translated, it has no body.
    }

    /// `IFramedBlockEntity` (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedBlockEntity.java)
    pub mod i_framed_block_entity {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFramedBlockEntity.material(0): not translated, it has no body.
    }

    /// `IFramedMaterials` (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedMaterials.java)
    pub mod i_framed_materials {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFramedMaterials.getHostBlock(0): not translated, it has no body.
        // IFramedMaterials.setHostBlock(1): not translated, it has no body.
        // IFramedMaterials.getMaterial(1): not translated, it has no body.
        // IFramedMaterials.setMaterial(2): not translated, it has no body.
    }

    /// `IFramedSourceBlock` (com/jaquadro/minecraft/storagedrawers/api/framing/IFramedSourceBlock.java)
    pub mod i_framed_source_block {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFramedSourceBlock.makeFramedItem(4): not translated, it has no body.
    }

    /// `IStorageDrawersApi` (com/jaquadro/minecraft/storagedrawers/api/IStorageDrawersApi.java)
    pub mod i_storage_drawers_api {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `IRenderRegistry` (com/jaquadro/minecraft/storagedrawers/api/registry/IRenderRegistry.java)
    pub mod i_render_registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IRenderRegistry.registerPreLabelRenderHandler(1): not translated, it has no body.
    }

    /// `IWailaRegistry` (com/jaquadro/minecraft/storagedrawers/api/registry/IWailaRegistry.java)
    pub mod i_waila_registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWailaRegistry.registerTooltipHandler(1): not translated, it has no body.
    }

    /// `IWailaTooltipHandler` (com/jaquadro/minecraft/storagedrawers/api/registry/IWailaTooltipHandler.java)
    pub mod i_waila_tooltip_handler {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IWailaTooltipHandler.transformItemName(2): not translated, it has no body.
    }

    /// `IRenderLabel` (com/jaquadro/minecraft/storagedrawers/api/render/IRenderLabel.java)
    pub mod i_render_label {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IRenderLabel.render(5): not translated, it has no body.
    }

    /// `IInteractionProvider` (com/jaquadro/minecraft/storagedrawers/api/security/IInteractionProvider.java)
    pub mod i_interaction_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IInteractionProvider.getProviderID(0): not translated, it has no body.
        // IInteractionProvider.canInteract(3): not translated, it has no body.
    }

    /// `ISecurityProvider` (com/jaquadro/minecraft/storagedrawers/api/security/ISecurityProvider.java)
    pub mod i_security_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ISecurityProvider.getProviderID(0): not translated, it has no body.
        // ISecurityProvider.hasOwnership(2): not translated, it has no body.
        // ISecurityProvider.hasAccess(2): not translated, it has no body.
    }

    /// `StorageDrawersApi` (com/jaquadro/minecraft/storagedrawers/api/StorageDrawersApi.java)
    pub mod storage_drawers_api {
        use super::*;
        // instance: not translated, it uses IStorageDrawersApi, which isn't translated (it's an interface).
        pub const VERSION: &str = "2.2.0";
        #[derive(Clone, Debug)]
        pub struct StorageDrawersApi {
        }
        impl StorageDrawersApi {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StorageDrawersApi.instance(0): not translated, it uses IStorageDrawersApi, which isn't translated (it's an interface).
    }

    /// `IPortable` (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IPortable.java)
    pub mod i_portable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPortable.isHeavy(2): not translated, it has no body.
    }

    /// `IProtectable` (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IProtectable.java)
    pub mod i_protectable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IProtectable.getOwner(0): not translated, it has no body.
        // IProtectable.setOwner(1): not translated, it has no body.
        // IProtectable.getSecurityProvider(0): not translated, it has no body.
        // IProtectable.setSecurityProvider(1): not translated, it has no body.
    }

    /// `ISealable` (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/ISealable.java)
    pub mod i_sealable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ISealable.isSealed(0): not translated, it has no body.
        // ISealable.setIsSealed(1): not translated, it has no body.
    }

    /// `IVoidable` (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/IVoidable.java)
    pub mod i_voidable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IVoidable.isVoid(0): not translated, it has no body.
    }

    /// `LockAttribute` (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/LockAttribute.java)
    pub mod lock_attribute {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum LockAttribute {
            LockPopulated,
            LockEmpty,
        }
        impl LockAttribute {
            pub const VALUES: [LockAttribute; 2] = [LockAttribute::LockPopulated, LockAttribute::LockEmpty, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    LockAttribute::LockPopulated => "LOCK_POPULATED",
                    LockAttribute::LockEmpty => "LOCK_EMPTY",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "LOCK_POPULATED" => LockAttribute::LockPopulated,
                    "LOCK_EMPTY" => LockAttribute::LockEmpty,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl LockAttribute {
            // LockAttribute.getFlagValue(0): not translated, it calls LockAttribute.ordinal, which isn't in the mod's source.
        }
        // LockAttribute.getBitfield(1): not translated, it uses `EnumSet` (java.util).
        // LockAttribute.getEnumSet(1): not translated, it uses `EnumSet` (java.util).
    }

    /// `MagnetDim` (com/jaquadro/minecraft/storagedrawers/api/storage/attribute/MagnetDim.java)
    pub mod magnet_dim {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum MagnetDim {
            Horizontal,
            Up,
            Down,
        }
        impl MagnetDim {
            pub const VALUES: [MagnetDim; 3] = [MagnetDim::Horizontal, MagnetDim::Up, MagnetDim::Down, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    MagnetDim::Horizontal => "HORIZONTAL",
                    MagnetDim::Up => "UP",
                    MagnetDim::Down => "DOWN",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "HORIZONTAL" => MagnetDim::Horizontal,
                    "UP" => MagnetDim::Up,
                    "DOWN" => MagnetDim::Down,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BlockType` (com/jaquadro/minecraft/storagedrawers/api/storage/BlockType.java)
    pub mod block_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum BlockType {
            Drawers,
            Trim,
        }
        impl BlockType {
            pub const VALUES: [BlockType; 2] = [BlockType::Drawers, BlockType::Trim, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    BlockType::Drawers => "Drawers",
                    BlockType::Trim => "Trim",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "Drawers" => BlockType::Drawers,
                    "Trim" => BlockType::Trim,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `Drawers` (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java)
    pub mod drawers {
        use super::*;
        // DISABLED: not translated, it uses IDrawer, which isn't translated (it's an interface).
        // DISABLED_FRACTIONAL: not translated, it uses IFractionalDrawer, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Drawers {
        }
        impl Drawers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `Drawers.DisabledDrawer` (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java)
    pub mod drawers_disabled_drawer {
        use super::*;
        // The class itself isn't translated: it implements `IDrawer`. Its static parts are below.
        // Drawers.DisabledDrawer.getStoredItemPrototype(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.setStoredItem(1): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.getStoredItemCount(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.setStoredItemCount(1): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.getMaxCapacity(1): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.getRemainingCapacity(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.canItemBeStored(2): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.canItemBeExtracted(2): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.isEmpty(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.isEnabled(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.DisabledDrawer.copy(0): not translated, its class isn't translated (it implements `IDrawer`).
    }

    /// `Drawers.DisabledFractionalDrawer` (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java)
    pub mod drawers_disabled_fractional_drawer {
        use super::*;
        // The class itself isn't translated: it extends `DisabledDrawer`. Its static parts are below.
        // Drawers.DisabledFractionalDrawer.getConversionRate(0): not translated, its class isn't translated (it extends `DisabledDrawer`).
        // Drawers.DisabledFractionalDrawer.getStoredItemRemainder(0): not translated, its class isn't translated (it extends `DisabledDrawer`).
        // Drawers.DisabledFractionalDrawer.isSmallestUnit(0): not translated, its class isn't translated (it extends `DisabledDrawer`).
        // Drawers.DisabledFractionalDrawer.copy(0): not translated, its class isn't translated (it extends `DisabledDrawer`).
    }

    /// `Drawers.WrappedDrawer` (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java)
    pub mod drawers_wrapped_drawer {
        use super::*;
        // The class itself isn't translated: it implements `IDrawer`. Its static parts are below.
        // Drawers.WrappedDrawer.WrappedDrawer(1): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.getStoredItemPrototype(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.setStoredItem(1): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.getStoredItemCount(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.setStoredItemCount(1): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.getMaxCapacity(1): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.getRemainingCapacity(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.canItemBeStored(2): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.canItemBeExtracted(2): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.isEmpty(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.isEnabled(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.getAttributes(0): not translated, its class isn't translated (it implements `IDrawer`).
        // Drawers.WrappedDrawer.copy(0): not translated, its class isn't translated (it implements `IDrawer`).
    }

    /// `Drawers.WrappedFractionalDrawer` (com/jaquadro/minecraft/storagedrawers/api/storage/Drawers.java)
    pub mod drawers_wrapped_fractional_drawer {
        use super::*;
        // The class itself isn't translated: it extends `WrappedDrawer`. Its static parts are below.
        // Drawers.WrappedFractionalDrawer.WrappedFractionalDrawer(1): not translated, its class isn't translated (it extends `WrappedDrawer`).
        // Drawers.WrappedFractionalDrawer.getConversionRate(0): not translated, its class isn't translated (it extends `WrappedDrawer`).
        // Drawers.WrappedFractionalDrawer.getStoredItemRemainder(0): not translated, its class isn't translated (it extends `WrappedDrawer`).
        // Drawers.WrappedFractionalDrawer.isSmallestUnit(0): not translated, its class isn't translated (it extends `WrappedDrawer`).
        // Drawers.WrappedFractionalDrawer.copy(0): not translated, its class isn't translated (it extends `WrappedDrawer`).
    }

    /// `EmptyDrawerAttributes` (com/jaquadro/minecraft/storagedrawers/api/storage/EmptyDrawerAttributes.java)
    pub mod empty_drawer_attributes {
        use super::*;
        // The class itself isn't translated: it implements `IDrawerAttributes`. Its static parts are below.
        // EMPTY: not translated, it uses EmptyDrawerAttributes, which isn't translated (it implements `IDrawerAttributes`).
    }

    /// `EnumBasicDrawer` (com/jaquadro/minecraft/storagedrawers/api/storage/EnumBasicDrawer.java)
    pub mod enum_basic_drawer {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // META_LOOKUP: not translated, it uses EnumBasicDrawer, which isn't translated (its constants carry values).
        // EnumBasicDrawer.EnumBasicDrawer(5): not translated, its class isn't translated (its constants carry values).
        // EnumBasicDrawer.getMetadata(0): not translated, its class isn't translated (its constants carry values).
        // EnumBasicDrawer.getDrawerCount(0): not translated, its class isn't translated (its constants carry values).
        // EnumBasicDrawer.isHalfDepth(0): not translated, its class isn't translated (its constants carry values).
        // EnumBasicDrawer.getUnlocalizedName(0): not translated, its class isn't translated (its constants carry values).
        // EnumBasicDrawer.byMetadata(1): not translated, it uses EnumBasicDrawer, which isn't translated (its constants carry values).
        // EnumBasicDrawer.toString(0): not translated, its class isn't translated (its constants carry values).
        // EnumBasicDrawer.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `IControlGroup` (com/jaquadro/minecraft/storagedrawers/api/storage/IControlGroup.java)
    pub mod i_control_group {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IControlGroup.getDrawerGroup(0): not translated, it has no body.
        // IControlGroup.getGroupControllableAttributes(1): not translated, it has no body.
        // IControlGroup.getBoundControlGroup(0): not translated, it has no body.
        // IControlGroup.getBoundRemoteNodes(0): not translated, it has no body.
        // IControlGroup.validateRemoteNode(1): not translated, it has no body.
        // IControlGroup.invalidateRemoteNode(1): not translated, it has no body.
        // IControlGroup.addRemoteNode(1): not translated, it has no body.
        // IControlGroup.isSoftBindingValid(2): not translated, it has no body.
    }

    /// `IDrawer` (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawer.java)
    pub mod i_drawer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawer.getStoredItemPrototype(0): not translated, it has no body.
        // IDrawer.setStoredItem(1): not translated, it has no body.
        // IDrawer.setStoredItem(2): not translated, its class isn't translated (it's an interface).
        // IDrawer.getStoredItemCount(0): not translated, it has no body.
        // IDrawer.setStoredItemCount(1): not translated, it has no body.
        // IDrawer.adjustStoredItemCount(1): not translated, its class isn't translated (it's an interface).
        // IDrawer.getMaxCapacity(0): not translated, its class isn't translated (it's an interface).
        // IDrawer.getMaxCapacity(1): not translated, it has no body.
        // IDrawer.getAcceptingMaxCapacity(1): not translated, its class isn't translated (it's an interface).
        // IDrawer.getRemainingCapacity(0): not translated, it has no body.
        // IDrawer.getAcceptingRemainingCapacity(0): not translated, its class isn't translated (it's an interface).
        // IDrawer.getStoredItemStackSize(0): not translated, its class isn't translated (it's an interface).
        // IDrawer.canItemBeStored(2): not translated, it has no body.
        // IDrawer.canItemBeStored(1): not translated, its class isn't translated (it's an interface).
        // IDrawer.canItemBeStoredManual(2): not translated, its class isn't translated (it's an interface).
        // IDrawer.canItemBeExtracted(2): not translated, it has no body.
        // IDrawer.canItemBeExtracted(1): not translated, its class isn't translated (it's an interface).
        // IDrawer.isEmpty(0): not translated, it has no body.
        // IDrawer.isEnabled(0): not translated, its class isn't translated (it's an interface).
        // IDrawer.isMissing(0): not translated, its class isn't translated (it's an interface).
        // IDrawer.canDetach(0): not translated, its class isn't translated (it's an interface).
        // IDrawer.setDetached(1): not translated, its class isn't translated (it's an interface).
        // IDrawer.getAttributes(0): not translated, its class isn't translated (it's an interface).
        // IDrawer.copy(0): not translated, it has no body.
    }

    /// `IDrawerAttributes` (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributes.java)
    pub mod i_drawer_attributes {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerAttributes.canItemLock(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isItemLocked(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isConcealed(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isSealed(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isShowingQuantity(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.getPriority(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isVoid(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.hasFillLevel(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isUnlimitedStorage(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isUnlimitedVending(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isDictConvertible(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isBalancedFill(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isHopper(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isMagnet(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributes.isSuspended(0): not translated, its class isn't translated (it's an interface).
    }

    /// `IDrawerAttributesGroupControl` (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesGroupControl.java)
    pub mod i_drawer_attributes_group_control {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerAttributesGroupControl.toggleConcealed(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesGroupControl.setIsConcealed(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesGroupControl.toggleItemLocked(2): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesGroupControl.setItemLocked(3): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesGroupControl.toggleIsShowingQuantity(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesGroupControl.setIsShowingQuantity(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesGroupControl.toggleIsSuspended(0): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesGroupControl.setIsSuspended(1): not translated, its class isn't translated (it's an interface).
    }

    /// `IDrawerAttributesModifiable` (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerAttributesModifiable.java)
    pub mod i_drawer_attributes_modifiable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerAttributesModifiable.setIsConcealed(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setItemLocked(2): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsShowingQuantity(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsSealed(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setPriority(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsVoid(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setHasFillLevel(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsUnlimitedStorage(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsUnlimitedVending(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsDictConvertible(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsBalancedFill(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsHopper(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsMagnet(1): not translated, its class isn't translated (it's an interface).
        // IDrawerAttributesModifiable.setIsSuspended(1): not translated, its class isn't translated (it's an interface).
    }

    /// `IDrawerGeometry` (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGeometry.java)
    pub mod i_drawer_geometry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerGeometry.isHalfDepth(0): not translated, it has no body.
        // IDrawerGeometry.getDrawerCount(0): not translated, it has no body.
    }

    /// `IDrawerGroup` (com/jaquadro/minecraft/storagedrawers/api/storage/IDrawerGroup.java)
    pub mod i_drawer_group {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IDrawerGroup.getDrawerCount(0): not translated, it has no body.
        // IDrawerGroup.getDrawer(1): not translated, it has no body.
        // IDrawerGroup.getAccessibleDrawerSlots(0): not translated, it has no body.
        // IDrawerGroup.isGroupValid(0): not translated, its class isn't translated (it's an interface).
        // IDrawerGroup.hasMissingDrawers(0): not translated, its class isn't translated (it's an interface).
    }

    /// `IFractionalDrawer` (com/jaquadro/minecraft/storagedrawers/api/storage/IFractionalDrawer.java)
    pub mod i_fractional_drawer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFractionalDrawer.getConversionRate(0): not translated, it has no body.
        // IFractionalDrawer.getStoredItemRemainder(0): not translated, it has no body.
        // IFractionalDrawer.isSmallestUnit(0): not translated, it has no body.
        // IFractionalDrawer.copy(0): not translated, it has no body.
    }

    /// `INetworked` (com/jaquadro/minecraft/storagedrawers/api/storage/INetworked.java)
    pub mod i_networked {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // INetworked.supportsDirectControllerLink(0): not translated, its class isn't translated (it's an interface).
        // INetworked.getBoundControlGroup(0): not translated, its class isn't translated (it's an interface).
        // INetworked.getSoftBoundControlGroups(0): not translated, its class isn't translated (it's an interface).
        // INetworked.softBindControlGroup(1): not translated, its class isn't translated (it's an interface).
        // INetworked.canRecurseSearch(0): not translated, its class isn't translated (it's an interface).
        // INetworked.unbindControlGroup(0): not translated, its class isn't translated (it's an interface).
        // INetworked.scheduleValidation(0): not translated, its class isn't translated (it's an interface).
    }

    /// `BlockCompDrawers` (com/jaquadro/minecraft/storagedrawers/block/BlockCompDrawers.java)
    pub mod block_comp_drawers {
        use super::*;
        // The class itself isn't translated: it extends `BlockDrawers`. Its static parts are below.
        // SLOTS: not translated, it uses Minecraft's `EnumProperty`.
        // BlockCompDrawers.BlockCompDrawers(4): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.BlockCompDrawers(4): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.BlockCompDrawers(3): not translated, its class isn't translated (it extends `BlockDrawers`).
        pub fn calc_units(drawer_count: i32, half_depth: bool) -> i32 {
            return if half_depth { 16 } else { 32 };
        }
        // BlockCompDrawers.BlockCompDrawers(2): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.BlockCompDrawers(1): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.getFaceSlot(2): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.setPlacedBy(5): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.newBlockEntity(2): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockCompDrawers.makeFramedItem(4): not translated, its class isn't translated (it extends `BlockDrawers`).
    }

    /// `BlockController` (com/jaquadro/minecraft/storagedrawers/block/BlockController.java)
    pub mod block_controller {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HorizontalDirectionalBlock`. Its static parts are below.
        // BlockController.BlockController(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.useItemOn(7): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.useController(6): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.toggle(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.toggle(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.togglePersonal(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.tick(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockController.makeFramedItem(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
    }

    /// `BlockControllerIO` (com/jaquadro/minecraft/storagedrawers/block/BlockControllerIO.java)
    pub mod block_controller_io {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockControllerIO.BlockControllerIO(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockControllerIO.getController(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockControllerIO.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockControllerIO.makeFramedItem(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockDrawers` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java)
    pub mod block_drawers {
        use super::*;
        // The class itself isn't translated: it extends `FaceSlotBlock`. Its static parts are below.
        // AABB_NORTH_FULL: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_SOUTH_FULL: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_WEST_FULL: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_EAST_FULL: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_NORTH_HALF: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_SOUTH_HALF: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_WEST_HALF: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_EAST_HALF: not translated, it uses Minecraft's `VoxelShape`.
        // HOPPER_INSIDE: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_NORTH_HOPPER: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_SOUTH_HOPPER: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_WEST_HOPPER: not translated, it uses Minecraft's `VoxelShape`.
        // AABB_EAST_HOPPER: not translated, it uses Minecraft's `VoxelShape`.
        // lastLeftClick: not translated, it uses `UUID` (java.util).
        // BlockDrawers.BlockDrawers(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.BlockDrawers(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.retrimBlock(3): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.retrimType(0): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.repartitionBlock(3): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getDrawerCount(0): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.isHalfDepth(0): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getStorageUnits(0): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getNameTypeKey(0): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getShape(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.isPathfindable(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getStateForPlacement(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.setPlacedBy(5): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getDrawerSlot(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.hitAny(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.hitTop(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.hitLeft(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.useSlotInvertible(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.useSlot(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.putSlot(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.takeSlot(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.interactPullDrawer(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.interactReturnDrawer(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.pullDrawer(3): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.giveOrDropItemStack(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.openUI(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getMenuProvider(3): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.dropItemStack(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getDrops(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getMainDrop(2): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.isSignalSource(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getSignal(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getDirectSignal(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.affectNeighborsAfterRemoval(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.hasAnalogOutputSignal(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getAnalogOutputSignal(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.useShapeForLightOcclusion(1): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.entityInside(6): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.getInteractionShape(3): not translated, its class isn't translated (it extends `FaceSlotBlock`).
        // BlockDrawers.tick(4): not translated, its class isn't translated (it extends `FaceSlotBlock`).
    }

    /// `BlockDrawers.GeometryType` (com/jaquadro/minecraft/storagedrawers/block/BlockDrawers.java)
    pub mod block_drawers_geometry_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum GeometryType {
            Slot,
            Count,
            Label,
            Indicator,
            Indicatorbase,
        }
        impl GeometryType {
            pub const VALUES: [GeometryType; 5] = [GeometryType::Slot, GeometryType::Count, GeometryType::Label, GeometryType::Indicator, GeometryType::Indicatorbase, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    GeometryType::Slot => "Slot",
                    GeometryType::Count => "Count",
                    GeometryType::Label => "Label",
                    GeometryType::Indicator => "Indicator",
                    GeometryType::Indicatorbase => "IndicatorBase",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "Slot" => GeometryType::Slot,
                    "Count" => GeometryType::Count,
                    "Label" => GeometryType::Label,
                    "Indicator" => GeometryType::Indicator,
                    "IndicatorBase" => GeometryType::Indicatorbase,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `BlockFramingTable` (com/jaquadro/minecraft/storagedrawers/block/BlockFramingTable.java)
    pub mod block_framing_table {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HorizontalDirectionalBlock`. Its static parts are below.
        // PART: not translated, it uses Minecraft's `EnumProperty`.
        // TABLE_TOP: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_BOTTOM_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_BOTTOM_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_BOTTOM_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_BOTTOM_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_SHAPE_NORTH: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_SHAPE_SOUTH: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_SHAPE_WEST: not translated, it uses Minecraft's `VoxelShape`.
        // TABLE_SHAPE_EAST: not translated, it uses Minecraft's `VoxelShape`.
        // BlockFramingTable.BlockFramingTable(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.getTableDirection(2): not translated, it uses Minecraft's `Direction`.
        // BlockFramingTable.getNeighborDirection(2): not translated, it uses Minecraft's `Direction`.
        // BlockFramingTable.getShape(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.playerWillDestroy(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.preventCreativeDropFromLeft(4): not translated, it uses Minecraft's `Level`.
        // BlockFramingTable.getStateForPlacement(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.getConnectedDirection(1): not translated, it uses Minecraft's `Direction`.
        // BlockFramingTable.getBlockType(1): not translated, it uses Minecraft's `BlockState`.
        // BlockFramingTable.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.newBlockEntity(2): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.setPlacedBy(5): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.getMenuProvider(3): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // BlockFramingTable.openUI(3): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
    }

    /// `BlockKeyButton` (com/jaquadro/minecraft/storagedrawers/block/BlockKeyButton.java)
    pub mod block_key_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`. Its static parts are below.
        // POWERED: not translated, it uses Minecraft's `BooleanProperty`.
        // CEILING_AABB_X: not translated, it uses Minecraft's `VoxelShape`.
        // CEILING_AABB_Z: not translated, it uses Minecraft's `VoxelShape`.
        // FLOOR_AABB_X: not translated, it uses Minecraft's `VoxelShape`.
        // FLOOR_AABB_Z: not translated, it uses Minecraft's `VoxelShape`.
        // NORTH_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // SOUTH_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // WEST_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // EAST_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_CEILING_AABB_X: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_CEILING_AABB_Z: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_FLOOR_AABB_X: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_FLOOR_AABB_Z: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_NORTH_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_SOUTH_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_WEST_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // PRESSED_EAST_AABB: not translated, it uses Minecraft's `VoxelShape`.
        // BlockKeyButton.BlockKeyButton(2): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.getShape(4): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.press(3): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.playSound(4): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.getSound(1): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.tick(4): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.checkPressed(3): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.updateNeighbours(3): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
        // BlockKeyButton.createBlockStateDefinition(1): not translated, its class isn't translated (it extends Minecraft's `FaceAttachedHorizontalDirectionalBlock`).
    }

    /// `BlockStandardDrawers` (com/jaquadro/minecraft/storagedrawers/block/BlockStandardDrawers.java)
    pub mod block_standard_drawers {
        use super::*;
        // The class itself isn't translated: it extends `BlockDrawers`. Its static parts are below.
        // BlockStandardDrawers.BlockStandardDrawers(4): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.BlockStandardDrawers(4): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.BlockStandardDrawers(3): not translated, its class isn't translated (it extends `BlockDrawers`).
        pub fn calc_units(drawer_count: i32, half_depth: bool) -> i32 {
            return if half_depth { 4 / drawer_count } else { 8 / drawer_count };
        }
        // BlockStandardDrawers.setMatKey(1): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.setMatKey(1): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.getMatKey(0): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.getNameMatKey(0): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.getFaceSlot(2): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.retrimBlock(3): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.repartitionBlock(3): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.newBlockEntity(2): not translated, its class isn't translated (it extends `BlockDrawers`).
        // BlockStandardDrawers.makeFramedItem(4): not translated, its class isn't translated (it extends `BlockDrawers`).
    }

    /// `BlockTrim` (com/jaquadro/minecraft/storagedrawers/block/BlockTrim.java)
    pub mod block_trim {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockTrim.BlockTrim(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTrim.setMatKey(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTrim.setMatKey(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTrim.getMatKey(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTrim.getNameMatKey(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTrim.getNameTypeKey(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTrim.canUseForRetrim(0): not translated, its class isn't translated (it extends Minecraft's `Block`).
        // BlockTrim.makeFramedItem(4): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `EnumCompDrawer` (com/jaquadro/minecraft/storagedrawers/block/EnumCompDrawer.java)
    pub mod enum_comp_drawer {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // META_LOOKUP: not translated, it uses EnumCompDrawer, which isn't translated (its constants carry values).
        // OPEN_SLOTS_LOOKUP: not translated, it uses EnumCompDrawer, which isn't translated (its constants carry values).
        // EnumCompDrawer.EnumCompDrawer(3): not translated, its class isn't translated (its constants carry values).
        // EnumCompDrawer.isHalfDepth(0): not translated, its class isn't translated (its constants carry values).
        // EnumCompDrawer.getDrawerCount(0): not translated, its class isn't translated (its constants carry values).
        // EnumCompDrawer.getMetadata(0): not translated, its class isn't translated (its constants carry values).
        // EnumCompDrawer.getOpenSlots(0): not translated, its class isn't translated (its constants carry values).
        // EnumCompDrawer.byMetadata(1): not translated, it uses EnumCompDrawer, which isn't translated (its constants carry values).
        // EnumCompDrawer.byOpenSlots(1): not translated, it uses EnumCompDrawer, which isn't translated (its constants carry values).
        // EnumCompDrawer.toString(0): not translated, its class isn't translated (its constants carry values).
        // EnumCompDrawer.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EnumFramingTablePart` (com/jaquadro/minecraft/storagedrawers/block/EnumFramingTablePart.java)
    pub mod enum_framing_table_part {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EnumFramingTablePart.EnumFramingTablePart(1): not translated, its class isn't translated (its constants carry values).
        // EnumFramingTablePart.toString(0): not translated, its class isn't translated (its constants carry values).
        // EnumFramingTablePart.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `FaceSlotBlock` (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java)
    pub mod face_slot_block {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `HorizontalDirectionalBlock`. Its static parts are below.
        // lastLeftActionTick: not translated, it uses `UUID` (java.util).
        // FaceSlotBlock.FaceSlotBlock(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.rightAction(5): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.leftAction(5): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.useWithoutItem(5): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.attack(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.useSlot(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.useSlotInvertible(1): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.putSlot(2): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.takeSlot(2): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.getFaceSlot(2): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.normalizeHitVec(1): not translated, it uses Minecraft's `Vec3`.
        // FaceSlotBlock.getFaceSlot(2): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.hitWithinArea(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.hitWithinY(3): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
        // FaceSlotBlock.hitWithinX(4): not translated, its class isn't translated (it extends Minecraft's `HorizontalDirectionalBlock`).
    }

    /// `FaceSlotBlock.InteractContext` (com/jaquadro/minecraft/storagedrawers/block/FaceSlotBlock.java)
    pub mod face_slot_block_interact_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // FaceSlotBlock.InteractContext.InteractContext(6): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // FaceSlotBlock.InteractContext.getCheckedEntity(1): not translated, it's generic.
        // FaceSlotBlock.InteractContext.getCheckedEntity(2): not translated, it's generic.
    }

    /// `BlockFramedCompDrawers` (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedCompDrawers.java)
    pub mod block_framed_comp_drawers {
        use super::*;
        // The class itself isn't translated: it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block). Its static parts are below.
        // BlockFramedCompDrawers.BlockFramedCompDrawers(4): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.BlockFramedCompDrawers(4): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.BlockFramedCompDrawers(3): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.setPlacedBy(5): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.getMainDrop(2): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.getCloneItemStack(4): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.getFramedBlockEntity(2): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.supportsFrameMaterial(1): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedCompDrawers.getShadeBrightness(3): not translated, its class isn't translated (it extends `BlockCompDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
    }

    /// `BlockFramedController` (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedController.java)
    pub mod block_framed_controller {
        use super::*;
        // The class itself isn't translated: it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block). Its static parts are below.
        // BlockFramedController.BlockFramedController(1): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedController.setPlacedBy(5): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedController.getDrops(2): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedController.getMainDrop(2): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedController.getCloneItemStack(4): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedController.getFramedBlockEntity(2): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedController.supportsFrameMaterial(1): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedController.getShadeBrightness(3): not translated, its class isn't translated (it extends `BlockController` (com.jaquadro.minecraft.storagedrawers.block)).
    }

    /// `BlockFramedControllerIO` (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedControllerIO.java)
    pub mod block_framed_controller_io {
        use super::*;
        // The class itself isn't translated: it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block). Its static parts are below.
        // BlockFramedControllerIO.BlockFramedControllerIO(1): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedControllerIO.setPlacedBy(5): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedControllerIO.getDrops(2): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedControllerIO.getMainDrop(2): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedControllerIO.getCloneItemStack(4): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedControllerIO.getFramedBlockEntity(2): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedControllerIO.supportsFrameMaterial(1): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedControllerIO.getShadeBrightness(3): not translated, its class isn't translated (it extends `BlockControllerIO` (com.jaquadro.minecraft.storagedrawers.block)).
    }

    /// `BlockFramedStandardDrawers` (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedStandardDrawers.java)
    pub mod block_framed_standard_drawers {
        use super::*;
        // The class itself isn't translated: it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block). Its static parts are below.
        // BlockFramedStandardDrawers.BlockFramedStandardDrawers(4): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.BlockFramedStandardDrawers(4): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.BlockFramedStandardDrawers(3): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.retrimType(0): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.setPlacedBy(5): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.getMainDrop(2): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.getCloneItemStack(4): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.getFramedBlockEntity(2): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.supportsFrameMaterial(1): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedStandardDrawers.getShadeBrightness(3): not translated, its class isn't translated (it extends `BlockStandardDrawers` (com.jaquadro.minecraft.storagedrawers.block)).
    }

    /// `BlockFramedTrim` (com/jaquadro/minecraft/storagedrawers/block/framed/BlockFramedTrim.java)
    pub mod block_framed_trim {
        use super::*;
        // The class itself isn't translated: it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block). Its static parts are below.
        // BlockFramedTrim.BlockFramedTrim(1): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.setPlacedBy(5): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.canUseForRetrim(0): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.getDrops(2): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.getMainDrop(2): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.getCloneItemStack(4): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.newBlockEntity(2): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.getFramedBlockEntity(2): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.supportsFrameMaterial(1): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.getShadeBrightness(3): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.propagatesSkylightDown(1): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
        // BlockFramedTrim.getVisualShape(4): not translated, its class isn't translated (it extends `BlockTrim` (com.jaquadro.minecraft.storagedrawers.block)).
    }

    /// `KeyType` (com/jaquadro/minecraft/storagedrawers/block/KeyType.java)
    pub mod key_type {
        use super::*;
        // TYPES: not translated, it creates `Object2ObjectArrayMap` (it.unimi.dsi.fastutil.objects).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // DRAWER: not translated, it calls KeyType.register, which isn't translated.
        // CONCEALMENT: not translated, it calls KeyType.register, which isn't translated.
        // PERSONAL: not translated, it calls KeyType.register, which isn't translated.
        // QUANTIFY: not translated, it calls KeyType.register, which isn't translated.
        // SUSPEND: not translated, it calls KeyType.register, which isn't translated.
        #[derive(Clone, Debug, PartialEq)]
        pub struct KeyType {
            pub name: String,
        }
        impl KeyType {
            pub fn new(name: &str) -> Self {
                let mut s = Self { name: String::new() };
                s.name = name.to_string();
                s
            }
            // KeyType.isEnabled(0): not translated, `this` used as a value.
            pub fn name(&self) -> String {
                return self.name.to_string();
            }
        }
        // KeyType.register(1): not translated, it uses KeyType.TYPES, which isn't translated (it creates `Object2ObjectArrayMap` (it.unimi.dsi.fastutil.objects)).
        // KeyType.values(0): not translated, it uses `Stream` (java.util.stream).
    }

    /// `BlockMeta` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMeta.java)
    pub mod block_meta {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Block`. Its static parts are below.
        // BlockMeta.BlockMeta(1): not translated, its class isn't translated (it extends Minecraft's `Block`).
    }

    /// `BlockMetaFacing` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacing.java)
    pub mod block_meta_facing {
        use super::*;
        // The class itself isn't translated: it extends `BlockMeta`. Its static parts are below.
        // FACING: not translated, it uses Minecraft's `EnumProperty`.
        // BlockMetaFacing.BlockMetaFacing(1): not translated, its class isn't translated (it extends `BlockMeta`).
        // BlockMetaFacing.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMeta`).
        // BlockMetaFacing.rotate(2): not translated, its class isn't translated (it extends `BlockMeta`).
        // BlockMetaFacing.mirror(2): not translated, its class isn't translated (it extends `BlockMeta`).
    }

    /// `BlockMetaFacingSized` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSized.java)
    pub mod block_meta_facing_sized {
        use super::*;
        // The class itself isn't translated: it extends `BlockMetaFacing`. Its static parts are below.
        // HALF: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockMetaFacingSized.BlockMetaFacingSized(1): not translated, its class isn't translated (it extends `BlockMetaFacing`).
        // BlockMetaFacingSized.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMetaFacing`).
    }

    /// `BlockMetaFacingSizedOpen` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java)
    pub mod block_meta_facing_sized_open {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BlockMetaFacingSizedOpen {
        }
        impl BlockMetaFacingSizedOpen {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BlockMetaFacingSizedOpen.Open2` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java)
    pub mod block_meta_facing_sized_open_open2 {
        use super::*;
        // The class itself isn't translated: it extends `BlockMetaFacingSized`. Its static parts are below.
        // SLOTS: not translated, it uses Minecraft's `EnumProperty`.
        // BlockMetaFacingSizedOpen.Open2.Open2(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
        // BlockMetaFacingSizedOpen.Open2.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
    }

    /// `BlockMetaFacingSizedOpen.Open3` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedOpen.java)
    pub mod block_meta_facing_sized_open_open3 {
        use super::*;
        // The class itself isn't translated: it extends `BlockMetaFacingSized`. Its static parts are below.
        // SLOTS: not translated, it uses Minecraft's `EnumProperty`.
        // BlockMetaFacingSizedOpen.Open3.Open3(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
        // BlockMetaFacingSizedOpen.Open3.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
    }

    /// `BlockMetaFacingSizedSlotted` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java)
    pub mod block_meta_facing_sized_slotted {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BlockMetaFacingSizedSlotted {
        }
        impl BlockMetaFacingSizedSlotted {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `BlockMetaFacingSizedSlotted.Label` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java)
    pub mod block_meta_facing_sized_slotted_label {
        use super::*;
        // The class itself isn't translated: it extends `BlockMetaFacingSized`. Its static parts are below.
        // SLOT: not translated, it uses Minecraft's `IntegerProperty`.
        // BlockMetaFacingSizedSlotted.Label.Label(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
        // BlockMetaFacingSizedSlotted.Label.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
    }

    /// `BlockMetaFacingSizedSlotted.Slots124` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java)
    pub mod block_meta_facing_sized_slotted_slots124 {
        use super::*;
        // The class itself isn't translated: it extends `BlockMetaFacingSized`. Its static parts are below.
        // SLOTS: not translated, it uses IntegerSetProperty, which isn't translated (it extends Minecraft's `Property`).
        // BlockMetaFacingSizedSlotted.Slots124.Slots124(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
        // BlockMetaFacingSizedSlotted.Slots124.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
    }

    /// `BlockMetaFacingSizedSlotted.Slots23` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaFacingSizedSlotted.java)
    pub mod block_meta_facing_sized_slotted_slots23 {
        use super::*;
        // The class itself isn't translated: it extends `BlockMetaFacingSized`. Its static parts are below.
        // SLOTS: not translated, it uses IntegerSetProperty, which isn't translated (it extends Minecraft's `Property`).
        // BlockMetaFacingSizedSlotted.Slots23.Slots23(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
        // BlockMetaFacingSizedSlotted.Slots23.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMetaFacingSized`).
    }

    /// `BlockMetaTrans` (com/jaquadro/minecraft/storagedrawers/block/meta/BlockMetaTrans.java)
    pub mod block_meta_trans {
        use super::*;
        // The class itself isn't translated: it extends `BlockMeta`. Its static parts are below.
        // TRANS: not translated, it uses Minecraft's `BooleanProperty`.
        // BlockMetaTrans.BlockMetaTrans(1): not translated, its class isn't translated (it extends `BlockMeta`).
        // BlockMetaTrans.createBlockStateDefinition(1): not translated, its class isn't translated (it extends `BlockMeta`).
    }

    /// `IntegerSetProperty` (com/jaquadro/minecraft/storagedrawers/block/state/IntegerSetProperty.java)
    pub mod integer_set_property {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Property`. Its static parts are below.
        // IntegerSetProperty.IntegerSetProperty(2): not translated, its class isn't translated (it extends Minecraft's `Property`).
        // IntegerSetProperty.getPossibleValues(0): not translated, its class isn't translated (it extends Minecraft's `Property`).
        // IntegerSetProperty.equals(1): not translated, its class isn't translated (it extends Minecraft's `Property`).
        // IntegerSetProperty.generateHashCode(0): not translated, its class isn't translated (it extends Minecraft's `Property`).
        // IntegerSetProperty.create(2): not translated, it uses IntegerSetProperty, which isn't translated (it extends Minecraft's `Property`).
        // IntegerSetProperty.getValue(1): not translated, its class isn't translated (it extends Minecraft's `Property`).
        // IntegerSetProperty.getName(1): not translated, its class isn't translated (it extends Minecraft's `Property`).
        // IntegerSetProperty.getInternalIndex(1): not translated, its class isn't translated (it extends Minecraft's `Property`).
    }

    /// `BaseBlockEntity` (com/jaquadro/minecraft/storagedrawers/block/tile/BaseBlockEntity.java)
    pub mod base_block_entity {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntity`. Its static parts are below.
        // BaseBlockEntity.BaseBlockEntity(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.hasDataPacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.dataPacketRequiresRenderUpdate(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.injectData(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.injectPortableData(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.loadAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.onLoadFinished(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.read(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.saveAdditional(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.readPortable(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.writePortable(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.readFixed(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.writeFixed(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.trapLoadFailure(2): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.restoreLoadFailure(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.loadDidFail(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.getUpdateTag(1): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.getUpdatePacket(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.onDataPacket(3): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.markBlockForUpdate(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.markBlockForUpdateClient(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
        // BaseBlockEntity.markBlockForRenderUpdate(0): not translated, its class isn't translated (it extends Minecraft's `BlockEntity`).
    }

    /// `BlockEntityController` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java)
    pub mod block_entity_controller {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockEntity`. Its static parts are below.
        pub const PRI_LOCKED: i32 = 0;
        pub const PRI_NORMAL: i32 = 1;
        pub const PRI_LOCKED_VOID: i32 = 2;
        pub const PRI_VOID: i32 = 3;
        pub const PRI_EMPTY: i32 = 4;
        pub const PRI_LOCKED_EMPTY: i32 = 5;
        pub const PRI_DISABLED: i32 = 6;
        // BlockEntityController.material(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getRenderData(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getAttributes(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getSlotPriorityGroup(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getSlotPriority(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.BlockEntityController(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.BlockEntityController(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getBlock(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.printDebugInfo(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getDrawerGroup(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getGroupControllableAttributes(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getBoundControlGroup(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getBoundRemoteNodes(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.validateRemoteNode(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.invalidateRemoteNode(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.addRemoteNode(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.setRemoved(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.isSoftBindingValid(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.onEntityLoad(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.isGroupValid(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.interactPutItemsIntoInventory(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.insertItems(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.insertItems(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.toggleProtection(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.clearProtection(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getSlotsValidAttributes(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.resetCache(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.isValidIO(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.updateCache(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.indexSlotRecords(1): not translated, its body couldn't be read: a class declared inside a method.
        // BlockEntityController.sortSlotRecords(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.rebuildPrimaryLookup(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.containsNullEntries(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.flattenLists(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.clearRecordInfo(2): not translated, its body couldn't be read: a class declared inside a method.
        // BlockEntityController.updateRecordInfo(3): not translated, its body couldn't be read: a class declared inside a method.
        // BlockEntityController.populateNodes(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.populateRoot(3): not translated, its body couldn't be read: a class declared inside a method.
        // BlockEntityController.getGroupForDrawerSlot(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getGroupForSlotRecord(1): not translated, its body couldn't be read: a class declared inside a method.
        // BlockEntityController.getLocalDrawerSlot(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.readFixed(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.dataPacketRequiresRenderUpdate(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getDrawerCount(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getDrawer(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getAccessibleDrawerSlots(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getItemRepository(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityController.getCapability(1): not translated, it's generic.
        // BlockEntityController.getBalanceDrawers(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
    }

    /// `BlockEntityController.GroupAttributeController` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java)
    pub mod block_entity_controller_group_attribute_controller {
        use super::*;
        // The class itself isn't translated: it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage). Its static parts are below.
        // BlockEntityController.GroupAttributeController.GroupAttributeController(0): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.GroupAttributeController(1): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.setIsConcealed(1): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.setIsShowingQuantity(1): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.setIsSuspended(1): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.setItemLocked(3): not translated, its body couldn't be read: a class declared inside a method.
        // BlockEntityController.GroupAttributeController.toggleConcealed(0): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.toggleIsShowingQuantity(0): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.toggleIsSuspended(0): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BlockEntityController.GroupAttributeController.toggleItemLocked(2): not translated, its class isn't translated (it implements `IDrawerAttributesGroupControl` (com.jaquadro.minecraft.storagedrawers.api.storage)).
    }

    /// `BlockEntityController.ItemRepository` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java)
    pub mod block_entity_controller_item_repository {
        use super::*;
        // The class itself isn't translated: it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities). Its static parts are below.
        // BlockEntityController.ItemRepository.ItemRepository(1): not translated, its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)).
        // BlockEntityController.ItemRepository.insertItem(3): not translated, its body couldn't be read: a class declared inside a method.
        // BlockEntityController.ItemRepository.getValidDrawer(3): not translated, its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)).
        // BlockEntityController.ItemRepository.isValidDrawer(4): not translated, its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)).
        // BlockEntityController.ItemRepository.extractItem(4): not translated, its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)).
        // BlockEntityController.ItemRepository.hasAccess(2): not translated, its class isn't translated (it extends `DrawerItemRepository` (com.jaquadro.minecraft.storagedrawers.capabilities)).
    }

    /// `BlockEntityController.ProtectedItemRepository` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java)
    pub mod block_entity_controller_protected_item_repository {
        use super::*;
        // The class itself isn't translated: it extends `ItemRepository`. Its static parts are below.
        // BlockEntityController.ProtectedItemRepository.ProtectedItemRepository(2): not translated, its class isn't translated (it extends `ItemRepository`).
        // BlockEntityController.ProtectedItemRepository.ProtectedItemRepository(3): not translated, its class isn't translated (it extends `ItemRepository`).
        // BlockEntityController.ProtectedItemRepository.hasAccess(2): not translated, its class isn't translated (it extends `ItemRepository`).
    }

    /// `BlockEntityController.SlotRecord` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java)
    pub mod block_entity_controller_slot_record {
        use super::*;
        // The class itself isn't translated: it implements `Comparable`. Its static parts are below.
        // BlockEntityController.SlotRecord.SlotRecord(3): not translated, its class isn't translated (it implements `Comparable`).
        // BlockEntityController.SlotRecord.compareTo(1): not translated, its class isn't translated (it implements `Comparable`).
    }

    /// `BlockEntityController.StorageRecord` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityController.java)
    pub mod block_entity_controller_storage_record {
        use super::*;
        // The class itself isn't translated: it uses IDrawerGroup, which isn't translated (it's an interface). Its static parts are below.
        // BlockEntityController.StorageRecord.clear(0): not translated, its class isn't translated (it uses IDrawerGroup, which isn't translated (it's an interface)).
    }

    /// `BlockEntityControllerIO` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java)
    pub mod block_entity_controller_io {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockEntity`. Its static parts are below.
        pub static DRAWER_SLOTS: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0]);
        // BlockEntityControllerIO.BlockEntityControllerIO(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.BlockEntityControllerIO(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getDrawerGroup(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getGroupControllableAttributes(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getBoundControlGroup(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.isSoftBindingValid(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getBoundRemoteNodes(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.validateRemoteNode(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.invalidateRemoteNode(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.addRemoteNode(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.isGroupValid(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.bindController(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getControllerPos(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getController(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getItemRepository(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getAccessibleDrawerSlots(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getDrawerCount(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getDrawer(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.setChanged(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getCapability(1): not translated, it's generic.
        // BlockEntityControllerIO.material(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityControllerIO.getRenderData(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
    }

    /// `BlockEntityControllerIO.ItemRepositoryProxy` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityControllerIO.java)
    pub mod block_entity_controller_io_item_repository_proxy {
        use super::*;
        // The class itself isn't translated: it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities). Its static parts are below.
        // BlockEntityControllerIO.ItemRepositoryProxy.getAllItems(0): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // BlockEntityControllerIO.ItemRepositoryProxy.insertItem(3): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // BlockEntityControllerIO.ItemRepositoryProxy.extractItem(4): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // BlockEntityControllerIO.ItemRepositoryProxy.getStoredItemCount(2): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // BlockEntityControllerIO.ItemRepositoryProxy.getRemainingItemCapacity(2): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // BlockEntityControllerIO.ItemRepositoryProxy.getItemCapacity(2): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
    }

    /// `BlockEntityDrawers` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java)
    pub mod block_entity_drawers {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockEntity`. Its static parts are below.
        // BlockEntityDrawers.BlockEntityDrawers(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.checkBoundController(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.validateBoundController(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.onEntityLoad(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.scheduleConditionalTick(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.supportsDirectControllerLink(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getBoundControlGroup(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getSoftBoundControlGroups(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.softBindControlGroup(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.canRecurseSearch(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.unbindControlGroup(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.scheduleValidation(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getGroup(0): not translated, it has no body.
        // BlockEntityDrawers.getGroup(1): not translated, it uses IDrawerGroup, which isn't translated (it's an interface).
        // BlockEntityDrawers.getDrawerAttributes(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getDrawerAttributes(1): not translated, it uses IDrawerAttributes, which isn't translated (it's an interface).
        // BlockEntityDrawers.upgrades(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.material(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.isGroupValid(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getDrawerCapacity(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getEffectiveDrawerCapacity(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.emptySlotCanBeCleared(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.preRemoveSideEffects(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getOwner(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.setOwner(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getSecurityProvider(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.setSecurityProvider(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.onAttributeChanged(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.refreshMagnetBound(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.isRedstone(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getRedstoneLevel(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getCombinedRedstoneLevel(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getMinRedstoneLevel(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getMaxRedstoneLevel(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.takeItemsFromSlot(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.takeItemsFromSlot(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.putItemsIntoSlot(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.putItemsIntoSlot(4): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.interactPutCurrentItemIntoSlot(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.interactPutCurrentInventoryIntoSlot(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.interactPutItemsIntoSlot(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.interactReplaceDrawer(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.interactReplaceDrawer(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.readPortable(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.writePortable(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.setChanged(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.syncClientCount(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.clientUpdateCount(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.clientUpdateCountAsync(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.dataPacketRequiresRenderUpdate(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getDrawerCount(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getDrawer(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getAccessibleDrawerSlots(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getName(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getCustomName(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.applyImplicitComponents(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.collectImplicitComponents(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.removeComponentsFromTag(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getCapability(1): not translated, it's generic.
        // BlockEntityDrawers.getRenderData(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.pushItemsTick(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.entityInside(4): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.suckInItems(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.getItemEntitiesInRange(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityDrawers.addItemEntity(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
    }

    /// `BlockEntityDrawers.ContentProvider` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java)
    pub mod block_entity_drawers_content_provider {
        use super::*;
        // The class itself isn't translated: it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory). Its static parts are below.
        // BlockEntityDrawers.ContentProvider.ContentProvider(1): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
        // BlockEntityDrawers.ContentProvider.createContent(1): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
        // BlockEntityDrawers.ContentProvider.getDisplayName(0): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
        // BlockEntityDrawers.ContentProvider.createMenu(3): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
    }

    /// `BlockEntityDrawers.DrawerAttributes` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java)
    pub mod block_entity_drawers_drawer_attributes {
        use super::*;
        // The class itself isn't translated: it extends `BasicDrawerAttributes` (com.jaquadro.minecraft.storagedrawers.capabilities). Its static parts are below.
        // BlockEntityDrawers.DrawerAttributes.onAttributeChanged(0): not translated, its class isn't translated (it extends `BasicDrawerAttributes` (com.jaquadro.minecraft.storagedrawers.capabilities)).
    }

    /// `BlockEntityDrawers.DrawerUpgradeData` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawers.java)
    pub mod block_entity_drawers_drawer_upgrade_data {
        use super::*;
        // The class itself isn't translated: it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata). Its static parts are below.
        // BlockEntityDrawers.DrawerUpgradeData.DrawerUpgradeData(0): not translated, its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawers.DrawerUpgradeData.canAddUpgrade(1): not translated, its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawers.DrawerUpgradeData.canRemoveUpgrade(1): not translated, its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawers.DrawerUpgradeData.canSwapUpgrade(2): not translated, its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawers.DrawerUpgradeData.onUpgradeChanged(2): not translated, its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawers.DrawerUpgradeData.stackCapacityCheck(1): not translated, its class isn't translated (it extends `UpgradeData` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
    }

    /// `BlockEntityDrawersComp` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java)
    pub mod block_entity_drawers_comp {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawers`. Its static parts are below.
        // BlockEntityDrawersComp.BlockEntityDrawersComp(3): not translated, its class isn't translated (it extends `BlockEntityDrawers`).
        // BlockEntityDrawersComp.emptySlotCanBeCleared(1): not translated, its class isn't translated (it extends `BlockEntityDrawers`).
        // BlockEntityDrawersComp.dataPacketRequiresRenderUpdate(0): not translated, its class isn't translated (it extends `BlockEntityDrawers`).
        // BlockEntityDrawersComp.clientUpdateCount(2): not translated, its class isn't translated (it extends `BlockEntityDrawers`).
        // BlockEntityDrawersComp.clientUpdateCountAsync(1): not translated, its class isn't translated (it extends `BlockEntityDrawers`).
    }

    /// `BlockEntityDrawersComp.GroupData` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java)
    pub mod block_entity_drawers_comp_group_data {
        use super::*;
        // The class itself isn't translated: it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata). Its static parts are below.
        // BlockEntityDrawersComp.GroupData.GroupData(1): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.getWorld(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.isGroupValid(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.log(1): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.getStackCapacity(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.onItemChanged(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.onAmountChanged(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.getCapability(1): not translated, it's generic.
        // BlockEntityDrawersComp.GroupData.getOwner(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.setOwner(1): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.getSecurityProvider(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.setSecurityProvider(1): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.getBoundControlGroup(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersComp.GroupData.getSoftBoundControlGroups(0): not translated, its class isn't translated (it extends `FractionalDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
    }

    /// `BlockEntityDrawersComp.Slot2` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java)
    pub mod block_entity_drawers_comp_slot2 {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawersComp`. Its static parts are below.
        // BlockEntityDrawersComp.Slot2.Slot2(2): not translated, its class isn't translated (it extends `BlockEntityDrawersComp`).
        // BlockEntityDrawersComp.Slot2.getGroup(0): not translated, its class isn't translated (it extends `BlockEntityDrawersComp`).
        // BlockEntityDrawersComp.Slot2.onAttributeChanged(0): not translated, its class isn't translated (it extends `BlockEntityDrawersComp`).
    }

    /// `BlockEntityDrawersComp.Slot3` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersComp.java)
    pub mod block_entity_drawers_comp_slot3 {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawersComp`. Its static parts are below.
        // BlockEntityDrawersComp.Slot3.Slot3(2): not translated, its class isn't translated (it extends `BlockEntityDrawersComp`).
        // BlockEntityDrawersComp.Slot3.getGroup(0): not translated, its class isn't translated (it extends `BlockEntityDrawersComp`).
        // BlockEntityDrawersComp.Slot3.onAttributeChanged(0): not translated, its class isn't translated (it extends `BlockEntityDrawersComp`).
    }

    /// `BlockEntityDrawersStandard` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java)
    pub mod block_entity_drawers_standard {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawers`. Its static parts are below.
        // BlockEntityDrawersStandard.BlockEntityDrawersStandard(3): not translated, its class isn't translated (it extends `BlockEntityDrawers`).
    }

    /// `BlockEntityDrawersStandard.GroupData` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java)
    pub mod block_entity_drawers_standard_group_data {
        use super::*;
        // The class itself isn't translated: it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata). Its static parts are below.
        // BlockEntityDrawersStandard.GroupData.GroupData(1): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersStandard.GroupData.createDrawer(1): not translated, its body couldn't be read: an inner class creation.
        // BlockEntityDrawersStandard.GroupData.isGroupValid(0): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersStandard.GroupData.getCapability(1): not translated, it's generic.
        // BlockEntityDrawersStandard.GroupData.getOwner(0): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersStandard.GroupData.setOwner(1): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersStandard.GroupData.getSecurityProvider(0): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersStandard.GroupData.setSecurityProvider(1): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersStandard.GroupData.getBoundControlGroup(0): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
        // BlockEntityDrawersStandard.GroupData.getSoftBoundControlGroups(0): not translated, its class isn't translated (it extends `StandardDrawerGroup` (com.jaquadro.minecraft.storagedrawers.block.tile.tiledata)).
    }

    /// `BlockEntityDrawersStandard.Slot1` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java)
    pub mod block_entity_drawers_standard_slot1 {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawersStandard`. Its static parts are below.
        // BlockEntityDrawersStandard.Slot1.Slot1(2): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
        // BlockEntityDrawersStandard.Slot1.getGroup(0): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
        // BlockEntityDrawersStandard.Slot1.onAttributeChanged(0): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
    }

    /// `BlockEntityDrawersStandard.Slot2` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java)
    pub mod block_entity_drawers_standard_slot2 {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawersStandard`. Its static parts are below.
        // BlockEntityDrawersStandard.Slot2.Slot2(2): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
        // BlockEntityDrawersStandard.Slot2.getGroup(0): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
        // BlockEntityDrawersStandard.Slot2.onAttributeChanged(0): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
    }

    /// `BlockEntityDrawersStandard.Slot4` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java)
    pub mod block_entity_drawers_standard_slot4 {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawersStandard`. Its static parts are below.
        // BlockEntityDrawersStandard.Slot4.Slot4(2): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
        // BlockEntityDrawersStandard.Slot4.getGroup(0): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
        // BlockEntityDrawersStandard.Slot4.onAttributeChanged(0): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
    }

    /// `BlockEntityDrawersStandard.StandardDrawerData` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityDrawersStandard.java)
    pub mod block_entity_drawers_standard_standard_drawer_data {
        use super::*;
        // The class itself isn't translated: it extends `DrawerData`. Its static parts are below.
        // BlockEntityDrawersStandard.StandardDrawerData.StandardDrawerData(2): not translated, its class isn't translated (it extends `DrawerData`).
        // BlockEntityDrawersStandard.StandardDrawerData.StandardDrawerData(1): not translated, its class isn't translated (it extends `DrawerData`).
        // BlockEntityDrawersStandard.StandardDrawerData.getStackCapacity(0): not translated, its class isn't translated (it extends `DrawerData`).
        // BlockEntityDrawersStandard.StandardDrawerData.onItemChanged(0): not translated, its class isn't translated (it extends `DrawerData`).
        // BlockEntityDrawersStandard.StandardDrawerData.onAmountChanged(0): not translated, its class isn't translated (it extends `DrawerData`).
        // BlockEntityDrawersStandard.StandardDrawerData.copy(0): not translated, its body couldn't be read: an inner class creation.
    }

    /// `BlockEntityFramingTable` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java)
    pub mod block_entity_framing_table {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockEntity`. Its static parts are below.
        pub const SLOT_INPUT: i32 = 0;
        pub const SLOT_SIDE: i32 = 1;
        pub const SLOT_TRIM: i32 = 2;
        pub const SLOT_FRONT: i32 = 3;
        pub const SLOT_RESULT: i32 = 4;
        // BlockEntityFramingTable.BlockEntityFramingTable(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.BlockEntityFramingTable(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        pub fn is_material_slot(slot_index: i32) -> bool {
            return ((slot_index == 1) || (slot_index == 2)) || (slot_index == 3);
        }
        // BlockEntityFramingTable.material(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.inventory(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.readFixed(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.writeFixed(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.isItemValidTarget(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.isItemValidMaterial(1): not translated, it uses Minecraft's `ItemStack`.
        // BlockEntityFramingTable.getName(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.getCustomName(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.applyImplicitComponents(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.collectImplicitComponents(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.removeComponentsFromTag(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.readPortable(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.writePortable(1): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.preRemoveSideEffects(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityFramingTable.getRenderBoundingBox(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
    }

    /// `BlockEntityFramingTable.BlockInventory` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java)
    pub mod block_entity_framing_table_block_inventory {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Container`. Its static parts are below.
        // BlockEntityFramingTable.BlockInventory.BlockInventory(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.getContainerSize(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.isEmpty(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.canPlaceItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.getItem(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.removeItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.removeItemNoUpdate(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.setItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.setChanged(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.setInputItem(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.rebuildResult(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.stillValid(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // BlockEntityFramingTable.BlockInventory.clearContent(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
    }

    /// `BlockEntityFramingTable.ContentProvider` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityFramingTable.java)
    pub mod block_entity_framing_table_content_provider {
        use super::*;
        // The class itself isn't translated: it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory). Its static parts are below.
        // BlockEntityFramingTable.ContentProvider.ContentProvider(1): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
        // BlockEntityFramingTable.ContentProvider.createContent(1): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
        // BlockEntityFramingTable.ContentProvider.getDisplayName(0): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
        // BlockEntityFramingTable.ContentProvider.createMenu(3): not translated, its class isn't translated (it implements `ContentMenuProvider` (com.texelsaurus.minecraft.chameleon.inventory)).
    }

    /// `BlockEntityTrim` (com/jaquadro/minecraft/storagedrawers/block/tile/BlockEntityTrim.java)
    pub mod block_entity_trim {
        use super::*;
        // The class itself isn't translated: it extends `BaseBlockEntity`. Its static parts are below.
        // BlockEntityTrim.BlockEntityTrim(3): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityTrim.BlockEntityTrim(2): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityTrim.material(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityTrim.dataPacketRequiresRenderUpdate(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
        // BlockEntityTrim.getRenderData(0): not translated, its class isn't translated (it extends `BaseBlockEntity`).
    }

    /// `DrawerModelProperties` (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/DrawerModelProperties.java)
    pub mod drawer_model_properties {
        use super::*;
        // The class itself isn't translated: it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model). Its static parts are below.
        // INSTANCE: not translated, it uses DrawerModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // DrawerModelProperties.DrawerModelProperties(0): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // DrawerModelProperties.DrawerModelProperties(1): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // DrawerModelProperties.getModelData(1): not translated, it uses DrawerModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // DrawerModelProperties.makeContext(3): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // DrawerModelProperties.makeContext(1): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
    }

    /// `FramedModelProperties` (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/FramedModelProperties.java)
    pub mod framed_model_properties {
        use super::*;
        // The class itself isn't translated: it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model). Its static parts are below.
        // INSTANCE: not translated, it uses FramedModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // FramedModelProperties.FramedModelProperties(0): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // FramedModelProperties.FramedModelProperties(1): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // FramedModelProperties.getModelData(1): not translated, it uses FramedModelProperties, which isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // FramedModelProperties.makeContext(3): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
        // FramedModelProperties.makeContext(1): not translated, its class isn't translated (it implements `ModelContextSupplier` (com.jaquadro.minecraft.storagedrawers.client.model)).
    }

    /// `RenderDataProvider` (com/jaquadro/minecraft/storagedrawers/block/tile/modelprops/RenderDataProvider.java)
    pub mod render_data_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RenderDataProvider.getRenderData(0): not translated, it has no body.
    }

    /// `PlatformBlockEntityController` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityController.java)
    pub mod platform_block_entity_controller {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityController`. Its static parts are below.
        // PlatformBlockEntityController.PlatformBlockEntityController(2): not translated, its class isn't translated (it extends `BlockEntityController`).
        // PlatformBlockEntityController.getRenderData(0): not translated, its class isn't translated (it extends `BlockEntityController`).
    }

    /// `PlatformBlockEntityControllerIO` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityControllerIO.java)
    pub mod platform_block_entity_controller_io {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityControllerIO`. Its static parts are below.
        // PlatformBlockEntityControllerIO.PlatformBlockEntityControllerIO(2): not translated, its class isn't translated (it extends `BlockEntityControllerIO`).
        // PlatformBlockEntityControllerIO.getRenderData(0): not translated, its class isn't translated (it extends `BlockEntityControllerIO`).
    }

    /// `PlatformBlockEntityDrawersComp` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java)
    pub mod platform_block_entity_drawers_comp {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawersComp`. Its static parts are below.
        // PlatformBlockEntityDrawersComp.PlatformBlockEntityDrawersComp(3): not translated, its class isn't translated (it extends `BlockEntityDrawersComp`).
    }

    /// `PlatformBlockEntityDrawersComp.Slot2` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java)
    pub mod platform_block_entity_drawers_comp_slot2 {
        use super::*;
        // The class itself isn't translated: it extends `Slot2`. Its static parts are below.
        // PlatformBlockEntityDrawersComp.Slot2.Slot2(2): not translated, its class isn't translated (it extends `Slot2`).
        // PlatformBlockEntityDrawersComp.Slot2.getRenderData(0): not translated, its class isn't translated (it extends `Slot2`).
    }

    /// `PlatformBlockEntityDrawersComp.Slot3` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersComp.java)
    pub mod platform_block_entity_drawers_comp_slot3 {
        use super::*;
        // The class itself isn't translated: it extends `Slot3`. Its static parts are below.
        // PlatformBlockEntityDrawersComp.Slot3.Slot3(2): not translated, its class isn't translated (it extends `Slot3`).
        // PlatformBlockEntityDrawersComp.Slot3.getRenderData(0): not translated, its class isn't translated (it extends `Slot3`).
    }

    /// `PlatformBlockEntityDrawersStandard` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java)
    pub mod platform_block_entity_drawers_standard {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDrawersStandard`. Its static parts are below.
        // PlatformBlockEntityDrawersStandard.PlatformBlockEntityDrawersStandard(3): not translated, its class isn't translated (it extends `BlockEntityDrawersStandard`).
    }

    /// `PlatformBlockEntityDrawersStandard.Slot1` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java)
    pub mod platform_block_entity_drawers_standard_slot1 {
        use super::*;
        // The class itself isn't translated: it extends `Slot1`. Its static parts are below.
        // PlatformBlockEntityDrawersStandard.Slot1.Slot1(2): not translated, its class isn't translated (it extends `Slot1`).
        // PlatformBlockEntityDrawersStandard.Slot1.getRenderData(0): not translated, its class isn't translated (it extends `Slot1`).
    }

    /// `PlatformBlockEntityDrawersStandard.Slot2` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java)
    pub mod platform_block_entity_drawers_standard_slot2 {
        use super::*;
        // The class itself isn't translated: it extends `Slot2`. Its static parts are below.
        // PlatformBlockEntityDrawersStandard.Slot2.Slot2(2): not translated, its class isn't translated (it extends `Slot2`).
        // PlatformBlockEntityDrawersStandard.Slot2.getRenderData(0): not translated, its class isn't translated (it extends `Slot2`).
    }

    /// `PlatformBlockEntityDrawersStandard.Slot4` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityDrawersStandard.java)
    pub mod platform_block_entity_drawers_standard_slot4 {
        use super::*;
        // The class itself isn't translated: it extends `Slot4`. Its static parts are below.
        // PlatformBlockEntityDrawersStandard.Slot4.Slot4(2): not translated, its class isn't translated (it extends `Slot4`).
        // PlatformBlockEntityDrawersStandard.Slot4.getRenderData(0): not translated, its class isn't translated (it extends `Slot4`).
    }

    /// `PlatformBlockEntityTrim` (com/jaquadro/minecraft/storagedrawers/block/tile/PlatformBlockEntityTrim.java)
    pub mod platform_block_entity_trim {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityTrim`. Its static parts are below.
        // PlatformBlockEntityTrim.PlatformBlockEntityTrim(2): not translated, its class isn't translated (it extends `BlockEntityTrim`).
        // PlatformBlockEntityTrim.getRenderData(0): not translated, its class isn't translated (it extends `BlockEntityTrim`).
    }

    /// `BlockEntityDataShim` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/BlockEntityDataShim.java)
    pub mod block_entity_data_shim {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BlockEntityDataShim {
        }
        impl BlockEntityDataShim {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // BlockEntityDataShim.read(1): not translated, it has no body.
            // BlockEntityDataShim.write(1): not translated, it has no body.
        }
    }

    /// `ControllerData` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerData.java)
    pub mod controller_data {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDataShim`. Its static parts are below.
        // ControllerData.read(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerData.write(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerData.getCoord(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerData.getController(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerData.bind(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerData.bindCoord(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerData.needsValidation(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerData.setNeedsValidation(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
    }

    /// `ControllerHostData` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/ControllerHostData.java)
    pub mod controller_host_data {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDataShim`. Its static parts are below.
        // ControllerHostData.read(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerHostData.write(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerHostData.validateRemoteNodes(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerHostData.validateRemoteNode(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerHostData.addRemoteNode(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerHostData.removeRemoteNode(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // ControllerHostData.getRemoteNodes(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
    }

    /// `DetachedDrawerData` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/DetachedDrawerData.java)
    pub mod detached_drawer_data {
        use super::*;
        // The class itself isn't translated: it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage). Its static parts are below.
        // DetachedDrawerData.DetachedDrawerData(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.DetachedDrawerData(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.DetachedDrawerData(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.DetachedDrawerData(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.DetachedDrawerData(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.getStorageMultiplier(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.setStorageMultiplier(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.isHeavy(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.setIsHeavy(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.getStoredItemPrototype(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.setStoredItem(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.setStoredItemRaw(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.getStoredItemCount(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.setStoredItemCount(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.setStoredItemCountRaw(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.getMaxCapacity(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.getRemainingCapacity(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.canItemBeStored(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.canItemBeExtracted(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.isEmpty(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.copy(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.serializeNBT(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // DetachedDrawerData.deserializeNBT(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
    }

    /// `FractionalDrawerGroup` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java)
    pub mod fractional_drawer_group {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDataShim`. Its static parts are below.
        // FractionalDrawerGroup.FractionalDrawerGroup(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.getDrawerCount(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.getDrawer(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.getAccessibleDrawerSlots(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.getPooledCount(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.setPooledCount(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.read(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.write(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.syncAttributes(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.getWorld(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.log(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.getStackCapacity(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.onItemChanged(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // FractionalDrawerGroup.onAmountChanged(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
    }

    /// `FractionalDrawerGroup.FractionalDrawer` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java)
    pub mod fractional_drawer_group_fractional_drawer {
        use super::*;
        // The class itself isn't translated: it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage). Its static parts are below.
        // FractionalDrawerGroup.FractionalDrawer.FractionalDrawer(2): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.FractionalDrawer(1): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getStoredItemPrototype(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.setStoredItem(1): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getStoredItemCount(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.setStoredItemCount(1): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.adjustStoredItemCount(1): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getMaxCapacity(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getMaxCapacity(1): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getAcceptingMaxCapacity(1): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getRemainingCapacity(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getAcceptingRemainingCapacity(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.canItemBeStored(2): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.canItemBeStoredManual(2): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.canItemBeExtracted(2): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.isEmpty(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.isEnabled(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getConversionRate(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getStoredItemRemainder(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.isSmallestUnit(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.getAttributes(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // FractionalDrawerGroup.FractionalDrawer.copy(0): not translated, its class isn't translated (it implements `IFractionalDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
    }

    /// `FractionalDrawerGroup.FractionalStorage` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/FractionalDrawerGroup.java)
    pub mod fractional_drawer_group_fractional_storage {
        use super::*;
        // The class itself isn't translated: it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`). Its static parts are below.
        // FractionalDrawerGroup.FractionalStorage.FractionalStorage(2): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getAttributes(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getPooledCount(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.setPooledCount(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getStack(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.baseStack(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.baseRate(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.setStoredItem(2): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getStoredCount(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.setStoredItemCount(2): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.adjustStoredItemCount(2): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getMaxCapacity(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getMaxCapacity(2): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getAcceptingMaxCapacity(2): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getRemainingCapacity(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getAcceptingRemainingCapacity(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.isEmpty(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.isEnabled(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.canItemBeStored(4): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.canItemBeExtracted(3): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getConversionRate(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.getStoredItemRemainder(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.isSmallestUnit(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.reset(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.populateSlotsFromCache(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.populateSlots(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.populateRawSlot(3): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.normalizeGroup(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.serializeNBT(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.deserializeNBT(1): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
        // FractionalDrawerGroup.FractionalStorage.syncAttributes(0): not translated, its class isn't translated (it uses FractionalDrawerGroup, which isn't translated (it extends `BlockEntityDataShim`)).
    }

    /// `MaterialData` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/MaterialData.java)
    pub mod material_data {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDataShim`. Its static parts are below.
        // EMPTY: not translated, it uses MaterialData, which isn't translated (it extends `BlockEntityDataShim`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // MaterialData.MaterialData(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.MaterialData(4): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.MaterialData(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getFrameBase(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getSide(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getFront(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getTrim(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getEffectiveSide(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getEffectiveFront(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getEffectiveTrim(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.isMatOpaque(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.allMatOpaque(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.setFrameBase(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.setSide(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.setFront(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.setTrim(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.clear(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.isEmpty(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.read(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.read(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.write(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getHostBlock(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.setHostBlock(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.getMaterial(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // MaterialData.setMaterial(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
    }

    /// `StandardDrawerGroup` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java)
    pub mod standard_drawer_group {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDataShim`. Its static parts are below.
        // StandardDrawerGroup.StandardDrawerGroup(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // StandardDrawerGroup.getDrawerCount(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // StandardDrawerGroup.getDrawer(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // StandardDrawerGroup.getAccessibleDrawerSlots(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // StandardDrawerGroup.read(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // StandardDrawerGroup.write(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // StandardDrawerGroup.createDrawer(1): not translated, it has no body.
        // StandardDrawerGroup.syncAttributes(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // StandardDrawerGroup.syncSlots(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
    }

    /// `StandardDrawerGroup.DrawerData` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/StandardDrawerGroup.java)
    pub mod standard_drawer_group_drawer_data {
        use super::*;
        // The class itself isn't translated: it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage). Its static parts are below.
        // StandardDrawerGroup.DrawerData.DrawerData(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.DrawerData(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getAttributes(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getStoredItemPrototype(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.setStoredItem(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.setStoredItem(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.setStoredItemRaw(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getStoredItemCount(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.setStoredItemCount(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.setStoredItemCount(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.setStoredItemCountRaw(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.adjustStoredItemCount(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.adjustStoredItemCount(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getMaxCapacity(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getStackSize(1): not translated, it uses Minecraft's `ItemStack`.
        // StandardDrawerGroup.DrawerData.getAcceptingMaxCapacity(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getRemainingCapacity(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getAcceptingRemainingCapacity(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.canItemBeStored(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.canItemBeStoredManual(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.canItemBeStored(3): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.canItemBeExtracted(2): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.isEmpty(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.reset(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.serializeNBT(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.deserializeNBT(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.syncAttributes(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.getStackCapacity(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.onItemChanged(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.onAmountChanged(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.canDetach(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.isMissing(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.setDetached(1): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // StandardDrawerGroup.DrawerData.copy(0): not translated, its class isn't translated (it implements `IDrawer` (com.jaquadro.minecraft.storagedrawers.api.storage)).
    }

    /// `UpgradeData` (com/jaquadro/minecraft/storagedrawers/block/tile/tiledata/UpgradeData.java)
    pub mod upgrade_data {
        use super::*;
        // The class itself isn't translated: it extends `BlockEntityDataShim`. Its static parts are below.
        // UpgradeData.UpgradeData(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.setDrawerAttributes(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getSlotCount(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getUpgrade(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasEmptySlot(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.addUpgrade(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.setUpgrade(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.canAddUpgrade(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.canRemoveUpgrade(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.canSwapUpgrade(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getStorageMultiplier(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getMagnetRange(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getMagnetActiveRate(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getMagnetIdleRate(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getRedstoneType(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasOneStackUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasUnlimitedUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasVendingUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasConversionUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasIlluminationUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasbalancedFillUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasHopperUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasMagnetUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasRemoteUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.hasPortabilityUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getNextUpgradeSlot(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.getRemoteUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.unbindRemoteUpgrade(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.updateRemoteUpgradeBinding(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.syncUpgrades(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.syncStorageMultiplier(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.syncMagnetRange(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.syncRedstoneLevel(0): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.read(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.write(1): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
        // UpgradeData.onUpgradeChanged(2): not translated, its class isn't translated (it extends `BlockEntityDataShim`).
    }

    /// `FrameHelper` (com/jaquadro/minecraft/storagedrawers/block/tile/util/FrameHelper.java)
    pub mod frame_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FrameHelper {
        }
        impl FrameHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FrameHelper.makeFramedItem(5): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `BasicDrawerAttributes` (com/jaquadro/minecraft/storagedrawers/capabilities/BasicDrawerAttributes.java)
    pub mod basic_drawer_attributes {
        use super::*;
        // The class itself isn't translated: it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage). Its static parts are below.
        // BasicDrawerAttributes.canItemLock(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isItemLocked(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setItemLocked(2): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isConcealed(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsConcealed(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isVoid(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsVoid(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.hasFillLevel(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setHasFillLevel(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isShowingQuantity(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsShowingQuantity(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.getPriority(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setPriority(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isUnlimitedStorage(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsUnlimitedStorage(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isUnlimitedVending(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsUnlimitedVending(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isDictConvertible(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsDictConvertible(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isBalancedFill(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsBalancedFill(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isHopper(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsHopper(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isMagnet(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsMagnet(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.isSuspended(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.setIsSuspended(1): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
        // BasicDrawerAttributes.onAttributeChanged(0): not translated, its class isn't translated (it implements `IDrawerAttributesModifiable` (com.jaquadro.minecraft.storagedrawers.api.storage)).
    }

    /// `Capabilities` (com/jaquadro/minecraft/storagedrawers/capabilities/Capabilities.java)
    pub mod capabilities {
        use super::*;
        // DRAWER_ATTRIBUTES: not translated, it uses ChameleonCapability, which isn't translated (it's an interface).
        // DRAWER_GROUP: not translated, it uses ChameleonCapability, which isn't translated (it's an interface).
        // ITEM_REPOSITORY: not translated, it uses ChameleonCapability, which isn't translated (it's an interface).
        // ITEM_HANDLER: not translated, it uses ChameleonCapability, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Capabilities {
        }
        impl Capabilities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `DrawerItemHandler` (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemHandler.java)
    pub mod drawer_item_handler {
        use super::*;
        // The class itself isn't translated: it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities). Its static parts are below.
        // DrawerItemHandler.DrawerItemHandler(1): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.getSlots(0): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.getStackInSlot(1): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.insertItem(3): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.insertItemFullScan(2): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.insertItemInternal(3): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.extractItem(3): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.getSlotLimit(1): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.isItemValid(2): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.isSuspended(0): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.slotIsVirtual(1): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemHandler.stackResult(2): not translated, its class isn't translated (it implements `IItemHandler` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
    }

    /// `DrawerItemRepository` (com/jaquadro/minecraft/storagedrawers/capabilities/DrawerItemRepository.java)
    pub mod drawer_item_repository {
        use super::*;
        // The class itself isn't translated: it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities). Its static parts are below.
        // DrawerItemRepository.DrawerItemRepository(1): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.getAllItems(0): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.insertItem(3): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.extractItem(4): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.getStoredItemCount(2): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.getRemainingItemCapacity(2): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.getItemCapacity(2): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.testPredicateInsert(3): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.testPredicateExtract(3): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.stackResult(2): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
        // DrawerItemRepository.isSuspended(0): not translated, its class isn't translated (it implements `IItemRepository` (com.jaquadro.minecraft.storagedrawers.api.capabilities)).
    }

    /// `PlatformCapabilities` (com/jaquadro/minecraft/storagedrawers/capabilities/PlatformCapabilities.java)
    pub mod platform_capabilities {
        use super::*;
        // DRAWER_ATTRIBUTES: not translated, it uses FabricCapability, which isn't translated (it's generic).
        // DRAWER_GROUP: not translated, it uses FabricCapability, which isn't translated (it's generic).
        // ITEM_REPOSITORY: not translated, it uses FabricCapability, which isn't translated (it's generic).
        // ITEM_HANDLER: not translated, it uses FabricCapability, which isn't translated (it's generic).
        #[derive(Clone, Debug)]
        pub struct PlatformCapabilities {
        }
        impl PlatformCapabilities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PlatformCapabilities.cast(1): not translated, it's generic.
        // PlatformCapabilities.initHandlers(0): not translated, it uses FabricCapability, which isn't translated (it's generic).
    }

    /// `ClientUtil` (com/jaquadro/minecraft/storagedrawers/client/ClientUtil.java)
    pub mod client_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ClientUtil {
        }
        impl ClientUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ClientUtil.getLocalPlayer(0): needs the game, so it's in game_side.rs.
        // ClientUtil.getLocalGameProfile(0): not translated, it uses Minecraft's `GameProfile`.
    }

    /// `ClientDetachedDrawerTooltip` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientDetachedDrawerTooltip.java)
    pub mod client_detached_drawer_tooltip {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ClientTooltipComponent`. Its static parts are below.
        pub static BACKGROUND_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("textures/gui/detached_tooltip.png"));
        // ClientDetachedDrawerTooltip.ClientDetachedDrawerTooltip(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientDetachedDrawerTooltip.getHeight(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientDetachedDrawerTooltip.getWidth(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientDetachedDrawerTooltip.backgroundWidth(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientDetachedDrawerTooltip.backgroundHeight(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientDetachedDrawerTooltip.extractImage(6): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientDetachedDrawerTooltip.renderSlot(4): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
    }

    /// `ClientKeyringTooltip` (com/jaquadro/minecraft/storagedrawers/client/gui/ClientKeyringTooltip.java)
    pub mod client_keyring_tooltip {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ClientTooltipComponent`. Its static parts are below.
        pub static SLOT_HIGHLIGHT_BACK_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/bundle/slot_highlight_back"));
        pub static SLOT_HIGHLIGHT_FRONT_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/bundle/slot_highlight_front"));
        pub static SLOT_BACKGROUND_SPRITE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "container/bundle/slot_background"));
        pub const SLOT_MARGIN: i32 = 4;
        pub const SLOT_SIZE: i32 = 24;
        pub const GRID_WIDTH: i32 = 96;
        // ClientKeyringTooltip.ClientKeyringTooltip(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.getHeight(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.getWidth(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.backgroundWidth(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.backgroundHeight(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.getContentXOffset(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.itemGridHeight(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.gridSizeY(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.slotCount(0): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.extractImage(6): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.renderEmptyBundleTooltip(6): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.renderBundleWithItemsTooltip(6): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.getShownItems(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        pub fn should_render_surplus_text(overflow: bool, ix: i32, iy: i32) -> bool {
            return overflow && ((ix * iy) == 1);
        }
        // ClientKeyringTooltip.getAmountOfHiddenItems(1): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.shouldRenderItemSlot(2): needs the game, so it's in game_side.rs.
        // ClientKeyringTooltip.renderCount(5): needs the game, so it's in game_side.rs.
        // ClientKeyringTooltip.renderSlot(7): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
        // ClientKeyringTooltip.drawSelectedItemTooltip(5): not translated, its class isn't translated (it implements Minecraft's `ClientTooltipComponent`).
    }

    /// `BakedModelProvider` (com/jaquadro/minecraft/storagedrawers/client/model/BakedModelProvider.java)
    pub mod baked_model_provider {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BakedModelProvider {
        }
        impl BakedModelProvider {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BakedModelProvider.makeStandardDrawerModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedDrawerModel(2): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedStandardDrawerModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedCompDrawerModel(2): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedComp2DrawerModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedComp3DrawerModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedTrimModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedControllerModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // BakedModelProvider.makeFramedControllerIOModel(1): not translated, it uses Minecraft's `BlockStateModel`.
    }

    /// `DrawerModelContext` (com/jaquadro/minecraft/storagedrawers/client/model/context/DrawerModelContext.java)
    pub mod drawer_model_context {
        use super::*;
        // The class itself isn't translated: it extends `FramedModelContext`. Its static parts are below.
        // DrawerModelContext.DrawerModelContext(1): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.DrawerModelContext(2): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.attr(0): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.group(0): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.protectable(0): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.protectable(1): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.group(1): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.attr(1): not translated, its class isn't translated (it extends `FramedModelContext`).
        // DrawerModelContext.materialData(1): not translated, its class isn't translated (it extends `FramedModelContext`).
    }

    /// `FramedModelContext` (com/jaquadro/minecraft/storagedrawers/client/model/context/FramedModelContext.java)
    pub mod framed_model_context {
        use super::*;
        // The class itself isn't translated: it extends `ModelContext`. Its static parts are below.
        // FramedModelContext.FramedModelContext(2): not translated, its class isn't translated (it extends `ModelContext`).
        // FramedModelContext.FramedModelContext(1): not translated, its class isn't translated (it extends `ModelContext`).
        // FramedModelContext.materialData(0): not translated, its class isn't translated (it extends `ModelContext`).
        // FramedModelContext.materialData(1): not translated, its class isn't translated (it extends `ModelContext`).
    }

    /// `ModelContext` (com/jaquadro/minecraft/storagedrawers/client/model/context/ModelContext.java)
    pub mod model_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // ModelContext.ModelContext(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // ModelContext.ModelContext(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // ModelContext.ModelContext(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // ModelContext.state(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // ModelContext.randomSource(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `CombinedModelDecorator` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/CombinedModelDecorator.java)
    pub mod combined_model_decorator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CombinedModelDecorator.add(1): not translated, its class isn't translated (it's generic).
        // CombinedModelDecorator.shouldRenderItem(0): not translated, its class isn't translated (it's generic).
        // CombinedModelDecorator.shouldRenderBase(1): not translated, its class isn't translated (it's generic).
        // CombinedModelDecorator.shouldRenderBase(2): not translated, its class isn't translated (it's generic).
        // CombinedModelDecorator.getRenderTypes(1): not translated, its class isn't translated (it's generic).
        // CombinedModelDecorator.emitQuads(3): not translated, its class isn't translated (it's generic).
        // CombinedModelDecorator.emitItemQuads(4): not translated, its class isn't translated (it's generic).
    }

    /// `DecoratorRenderType` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DecoratorRenderType.java)
    pub mod decorator_render_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DecoratorRenderType {
            Solid,
            Cutout,
            Translucent,
        }
        impl DecoratorRenderType {
            pub const VALUES: [DecoratorRenderType; 3] = [DecoratorRenderType::Solid, DecoratorRenderType::Cutout, DecoratorRenderType::Translucent, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DecoratorRenderType::Solid => "SOLID",
                    DecoratorRenderType::Cutout => "CUTOUT",
                    DecoratorRenderType::Translucent => "TRANSLUCENT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "SOLID" => DecoratorRenderType::Solid,
                    "CUTOUT" => DecoratorRenderType::Cutout,
                    "TRANSLUCENT" => DecoratorRenderType::Translucent,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        // DecoratorRenderType.fromItemType(1): not translated, it's overloaded with the same number of parameters.
        // DecoratorRenderType.fromItemType(1): not translated, it's overloaded with the same number of parameters.
        // DecoratorRenderType.toChunkType(1): not translated, it uses Minecraft's `ChunkSectionLayer`.
        // DecoratorRenderType.toItemType(1): not translated, it uses Minecraft's `RenderType`.
    }

    /// `DrawerModelDecorator` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/DrawerModelDecorator.java)
    pub mod drawer_model_decorator {
        use super::*;
        // The class itself isn't translated: it extends `ModelDecorator`. Its static parts are below.
        pub static DEFAULT_RENDER_TYPES: LazyLock<Vec<decorator_render_type::DecoratorRenderType>> = LazyLock::new(|| vec![decorator_render_type::DecoratorRenderType::Cutout]);
        // DrawerModelDecorator.DrawerModelDecorator(1): not translated, its class isn't translated (it extends `ModelDecorator`).
        // DrawerModelDecorator.getRenderTypes(1): not translated, its class isn't translated (it extends `ModelDecorator`).
        // DrawerModelDecorator.emitQuads(3): not translated, its class isn't translated (it extends `ModelDecorator`).
        // DrawerModelDecorator.emitDecoratedQuads(3): not translated, its class isn't translated (it extends `ModelDecorator`).
    }

    /// `MaterialModelDecorator` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java)
    pub mod material_model_decorator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // replacementCache: not translated, it uses Minecraft's `BlockStateModel`.
        pub static DEFAULT_RENDER_LIST: LazyLock<Vec<decorator_render_type::DecoratorRenderType>> = LazyLock::new(|| vec![decorator_render_type::DecoratorRenderType::Solid]);
        pub static DEFAULT_SHADED_RENDER_LIST: LazyLock<Vec<decorator_render_type::DecoratorRenderType>> = LazyLock::new(|| vec![decorator_render_type::DecoratorRenderType::Solid, decorator_render_type::DecoratorRenderType::Translucent]);
        // MaterialModelDecorator.MaterialModelDecorator(2): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.shouldRenderItem(0): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.shouldRenderBase(1): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.shouldRenderBase(2): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.getRenderTypes(1): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.emitQuads(3): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.emitItemQuads(4): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.getReplacementModel(3): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.emitFramedQuads(3): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.emitFramedOverlayQuads(3): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.getStoreModel(2): not translated, it has no body.
        // MaterialModelDecorator.getStoreModel(3): not translated, its class isn't translated (it's generic).
    }

    /// `MaterialModelDecorator.Facing` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java)
    pub mod material_model_decorator_facing {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MaterialModelDecorator.Facing.Facing(2): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.Facing.getStoreModel(2): not translated, its class isn't translated (it's generic).
    }

    /// `MaterialModelDecorator.FacingSized` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java)
    pub mod material_model_decorator_facing_sized {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MaterialModelDecorator.FacingSized.FacingSized(2): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.FacingSized.getStoreModel(2): not translated, its class isn't translated (it's generic).
    }

    /// `MaterialModelDecorator.FacingSizedOpen` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java)
    pub mod material_model_decorator_facing_sized_open {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MaterialModelDecorator.FacingSizedOpen.FacingSizedOpen(2): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.FacingSizedOpen.getStoreModel(2): not translated, its class isn't translated (it's generic).
    }

    /// `MaterialModelDecorator.FacingSizedSlotted` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java)
    pub mod material_model_decorator_facing_sized_slotted {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MaterialModelDecorator.FacingSizedSlotted.FacingSizedSlotted(2): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.FacingSizedSlotted.getStoreModel(2): not translated, its class isn't translated (it's generic).
    }

    /// `MaterialModelDecorator.Single` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/MaterialModelDecorator.java)
    pub mod material_model_decorator_single {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // MaterialModelDecorator.Single.Single(2): not translated, its class isn't translated (it's generic).
        // MaterialModelDecorator.Single.getStoreModel(2): not translated, its class isn't translated (it's generic).
    }

    /// `ModelDecorator` (com/jaquadro/minecraft/storagedrawers/client/model/decorator/ModelDecorator.java)
    pub mod model_decorator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ModelDecorator.shouldRenderItem(0): not translated, its class isn't translated (it's generic).
        // ModelDecorator.shouldRenderBase(1): not translated, its class isn't translated (it's generic).
        // ModelDecorator.shouldRenderBase(2): not translated, its class isn't translated (it's generic).
        // ModelDecorator.getRenderTypes(1): not translated, its class isn't translated (it's generic).
        // ModelDecorator.emitQuads(2): not translated, its class isn't translated (it's generic).
        // ModelDecorator.emitQuads(3): not translated, its class isn't translated (it's generic).
        // ModelDecorator.emitItemQuads(3): not translated, its class isn't translated (it's generic).
        // ModelDecorator.emitItemQuads(4): not translated, its class isn't translated (it's generic).
    }

    /// `DrawerModelGeometry` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelGeometry.java)
    pub mod drawer_model_geometry {
        use super::*;
        pub static GEOMETRY_DATA_LOADED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct DrawerModelGeometry {
        }
        impl DrawerModelGeometry {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DrawerModelGeometry.loadGeometryData(0): not translated, it uses BlockDrawers, which isn't translated (it extends `FaceSlotBlock`).
        // DrawerModelGeometry.populateGeometryData(5): not translated, it uses BlockDrawers, which isn't translated (it extends `FaceSlotBlock`).
        // DrawerModelGeometry.populateGeometryData(3): not translated, it uses BlockDrawers, which isn't translated (it extends `FaceSlotBlock`).
        // DrawerModelGeometry.getBlockModel(1): not translated, it uses Minecraft's `CuboidModel`.
        // DrawerModelGeometry.getElements(1): not translated, it uses Minecraft's `CuboidModelElement`.
    }

    /// `DrawerModelStore` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java)
    pub mod drawer_model_store {
        use super::*;
        // FramedStandardDrawerMaterials: not translated, it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // FramedComp2DrawerMaterials: not translated, it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // FramedComp3DrawerMaterials: not translated, it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // FramedControllerMaterials: not translated, it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // FramedControllerIOMaterials: not translated, it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // FramedTrimMaterials: not translated, it uses FrameMatSet, which isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // modelStore: not translated, it uses Minecraft's `BlockState`.
        // replacementStore: not translated, it uses Minecraft's `BlockState`.
        // locationStore: not translated, it uses Minecraft's `BlockState`.
        // INSTANCE: not translated, it uses DecorationSet, which isn't translated (it uses Minecraft's `BlockState`).
        // missingSlots1: not translated, it uses DynamicPart, which isn't translated (its constants carry values).
        // missingSlots2: not translated, it uses DynamicPart, which isn't translated (its constants carry values).
        // missingSlots4: not translated, it uses DynamicPart, which isn't translated (its constants carry values).
        // missingSlots: not translated, it uses DynamicPart, which isn't translated (its constants carry values).
        #[derive(Clone, Debug)]
        pub struct DrawerModelStore {
        }
        impl DrawerModelStore {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DrawerModelStore.addLocation(1): not translated, it uses Minecraft's `BlockState`.
        pub fn get_variant_0() -> String {
            return "".to_string();
        }
        // DrawerModelStore.getVariant(1): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(1): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(2): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(3): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(3): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getSlotVariant(3): not translated, it uses Minecraft's `Direction`.
        // DrawerModelStore.getVariant(1): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(2): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(2): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(3): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(4): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getVariant(4): not translated, it's overloaded with the same number of parameters.
        // DrawerModelStore.getSlotVariant(4): not translated, it uses DynamicPart, which isn't translated (its constants carry values).
        // DrawerModelStore.getModelLocations(0): not translated, it uses `Stream` (java.util.stream).
        // DrawerModelStore.tryAddModel(2): not translated, it uses Minecraft's `BlockState`.
        // DrawerModelStore.getModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getModel(1): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getModel(2): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getModel(2): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getModel(3): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getModel(4): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getModel(4): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getReplacementModel(2): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getReplacementModel(2): not translated, it uses Minecraft's `BlockStateModel`.
        // DrawerModelStore.getReplacementModel(5): not translated, it uses Minecraft's `BlockStateModel`.
    }

    /// `DrawerModelStore.DecorationSet` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java)
    pub mod drawer_model_store_decoration_set {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // DrawerModelStore.DecorationSet.DecorationSet(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // DrawerModelStore.DecorationSet.add(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // DrawerModelStore.DecorationSet.add(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // DrawerModelStore.DecorationSet.addOverlay(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // DrawerModelStore.DecorationSet.isTargetedModel(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `DrawerModelStore.DynamicPart` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java)
    pub mod drawer_model_store_dynamic_part {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // DrawerModelStore.DynamicPart.DynamicPart(1): not translated, its class isn't translated (its constants carry values).
        // DrawerModelStore.DynamicPart.getName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `DrawerModelStore.FrameMatSet` (com/jaquadro/minecraft/storagedrawers/client/model/DrawerModelStore.java)
    pub mod drawer_model_store_frame_mat_set {
        use super::*;
        // The class itself isn't translated: it uses DynamicPart, which isn't translated (its constants carry values). Its static parts are below.
        // DrawerModelStore.FrameMatSet.sidePart(0): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.trimPart(0): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.frontPart(0): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.shadeFrontPart(0): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.shadeSidePart(0): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.sidePart(1): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.trimPart(1): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.frontPart(1): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.shadeFrontPart(1): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
        // DrawerModelStore.FrameMatSet.shadeSidePart(1): not translated, its class isn't translated (it uses DynamicPart, which isn't translated (its constants carry values)).
    }

    /// `ItemModelStore` (com/jaquadro/minecraft/storagedrawers/client/model/ItemModelStore.java)
    pub mod item_model_store {
        use super::*;
        // models: not translated, it uses Minecraft's `BlockState`.
        #[derive(Clone, Debug)]
        pub struct ItemModelStore {
        }
        impl ItemModelStore {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ModelContextSupplier` (com/jaquadro/minecraft/storagedrawers/client/model/ModelContextSupplier.java)
    pub mod model_context_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ModelContextSupplier.makeContext(3): not translated, it has no body.
        // ModelContextSupplier.makeContext(1): not translated, it has no body.
    }

    /// `ModelLoadPlugin` (com/jaquadro/minecraft/storagedrawers/client/model/ModelLoadPlugin.java)
    pub mod model_load_plugin {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModelLoadingPlugin`. Its static parts are below.
        // ModelLoadPlugin.initialize(1): not translated, its class isn't translated (it implements Fabric's `ModelLoadingPlugin`).
    }

    /// `ModelLoadPlugin.UnbakedProxyModel` (com/jaquadro/minecraft/storagedrawers/client/model/ModelLoadPlugin.java)
    pub mod model_load_plugin_unbaked_proxy_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `UnbakedRoot`. Its static parts are below.
        // ModelLoadPlugin.UnbakedProxyModel.UnbakedProxyModel(2): not translated, its class isn't translated (it implements Minecraft's `UnbakedRoot`).
        // ModelLoadPlugin.UnbakedProxyModel.bake(2): not translated, its class isn't translated (it implements Minecraft's `UnbakedRoot`).
        // ModelLoadPlugin.UnbakedProxyModel.resolveDependencies(1): not translated, its class isn't translated (it implements Minecraft's `UnbakedRoot`).
        // ModelLoadPlugin.UnbakedProxyModel.visualEqualityGroup(1): not translated, its class isn't translated (it implements Minecraft's `UnbakedRoot`).
    }

    /// `ParentModel` (com/jaquadro/minecraft/storagedrawers/client/model/ParentModel.java)
    pub mod parent_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockStateModel`. Its static parts are below.
        // ParentModel.ParentModel(1): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // ParentModel.collectParts(2): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // ParentModel.particleMaterial(0): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
        // ParentModel.materialFlags(0): not translated, its class isn't translated (it implements Minecraft's `BlockStateModel`).
    }

    /// `PlatformDecoratedModel` (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java)
    pub mod platform_decorated_model {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // meshCache: not translated, it uses Minecraft's `BlockStateModel`.
        pub static DECORATOR_RENDER_TYPES: LazyLock<Vec<decorator_render_type::DecoratorRenderType>> = LazyLock::new(|| vec![decorator_render_type::DecoratorRenderType::Solid, decorator_render_type::DecoratorRenderType::Cutout, decorator_render_type::DecoratorRenderType::Translucent]);
        // PlatformDecoratedModel.PlatformDecoratedModel(3): not translated, its class isn't translated (it's generic).
        // PlatformDecoratedModel.PlatformDecoratedModel(2): not translated, its class isn't translated (it's generic).
        // PlatformDecoratedModel.emitQuads(6): not translated, its class isn't translated (it's generic).
        // PlatformDecoratedModel.getMesh(4): not translated, its class isn't translated (it's generic).
        // PlatformDecoratedModel.buildMesh(4): not translated, its class isn't translated (it's generic).
    }

    /// `PlatformDecoratedModel.PlatformDecoratedItemModel` (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java)
    pub mod platform_decorated_model_platform_decorated_item_model {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ItemModel`. Its static parts are below.
        // PlatformDecoratedModel.PlatformDecoratedItemModel.PlatformDecoratedItemModel(3): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
        // PlatformDecoratedModel.PlatformDecoratedItemModel.update(7): not translated, its class isn't translated (it implements Minecraft's `ItemModel`).
        // PlatformDecoratedModel.PlatformDecoratedItemModel.setProperty(3): not translated, it's generic.
    }

    /// `PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked` (com/jaquadro/minecraft/storagedrawers/client/model/PlatformDecoratedModel.java)
    pub mod platform_decorated_model_platform_decorated_item_model_unbaked {
        use super::*;
        // The class itself isn't translated: it implements `Unbaked`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.Unbaked(2): not translated, its class isn't translated (it implements `Unbaked`).
        // PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.type(0): not translated, its class isn't translated (it implements `Unbaked`).
        // PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.bake(2): not translated, its class isn't translated (it implements `Unbaked`).
        // PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.resolveDependencies(1): not translated, its class isn't translated (it implements `Unbaked`).
        // PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.model(0): not translated, its class isn't translated (it implements `Unbaked`).
        // PlatformDecoratedModel.PlatformDecoratedItemModel.Unbaked.variant(0): not translated, its class isn't translated (it implements `Unbaked`).
    }

    /// `SpriteReplacementModel` (com/jaquadro/minecraft/storagedrawers/client/model/SpriteReplacementModel.java)
    pub mod sprite_replacement_model {
        use super::*;
        // The class itself isn't translated: it extends `ParentModel`. Its static parts are below.
        // SpriteReplacementModel.SpriteReplacementModel(2): not translated, its class isn't translated (it extends `ParentModel`).
        // SpriteReplacementModel.SpriteReplacementModel(3): not translated, its class isn't translated (it extends `ParentModel`).
        // SpriteReplacementModel.SpriteReplacementModel(3): not translated, its class isn't translated (it extends `ParentModel`).
        // SpriteReplacementModel.SpriteReplacementModel(2): not translated, its class isn't translated (it extends `ParentModel`).
        // SpriteReplacementModel.collectParts(2): not translated, its class isn't translated (it extends `ParentModel`).
        // SpriteReplacementModel.particleMaterial(0): not translated, its class isn't translated (it extends `ParentModel`).
    }

    /// `BlockEntityDrawersRenderer` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java)
    pub mod block_entity_drawers_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        // TEXT_COLOR_TRANSPARENT: not translated, it calls Minecraft's `ARGB`.color.
        // ITEM_LIGHT_ROTATION_3D: not translated, it creates `Matrix3f` (org.joml).
        pub static SIDE_ROTATION_Y2_D: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0f32, 0.0f32, 2.0f32, 0.0f32, 3.0f32, 1.0f32]);
        // BlockEntityDrawersRenderer.BlockEntityDrawersRenderer(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.playerBehindBlock(2): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.renderFastItemSet(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.renderText(7): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.renderFastItem(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.alignRendering(2): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.getRotationYForSide2D(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityDrawersRenderer.renderIndicator(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
    }

    /// `BlockEntityDrawersRenderer.QuadBuilder` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityDrawersRenderer.java)
    pub mod block_entity_drawers_renderer_quad_builder {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomGeometryRenderer`. Its static parts are below.
        pub static TEXTURE_IND_1: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("block/indicator/indicator_1_on"));
        pub static TEXTURE_IND_2: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("block/indicator/indicator_2_on"));
        pub static TEXTURE_IND_4: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("block/indicator/indicator_4_on"));
        pub static TEXTURE_IND_COMP_3: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("block/indicator/indicator_comp_on"));
        pub static TEXTURE_IND_COMP_2: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("block/indicator/indicator_comp2_on"));
        // MAT_IND_1: not translated, it uses Minecraft's `SpriteId`.
        // MAT_IND_2: not translated, it uses Minecraft's `SpriteId`.
        // MAT_IND_4: not translated, it uses Minecraft's `SpriteId`.
        // MAT_IND_COMP_3: not translated, it uses Minecraft's `SpriteId`.
        // MAT_IND_COMP_2: not translated, it uses Minecraft's `SpriteId`.
        // BlockEntityDrawersRenderer.QuadBuilder.QuadBuilder(1): not translated, its class isn't translated (it implements Minecraft's `CustomGeometryRenderer`).
        // BlockEntityDrawersRenderer.QuadBuilder.render(2): not translated, its class isn't translated (it implements Minecraft's `CustomGeometryRenderer`).
        pub fn get_ind_end(stored_count: i32, max_capacity: i32, slot: i32, x: f32, w: f32, step: i32) -> f32 {
            if (max_capacity != 0) && (stored_count != 0) {
                let fill_amt: f32 = (((step * stored_count) / max_capacity) as f32) / (step as f32);
                return x + (w * fill_amt);
            } else {
                return x;
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // BlockEntityDrawersRenderer.QuadBuilder.addQuad(13): not translated, it uses Minecraft's `Pose`.
        // BlockEntityDrawersRenderer.QuadBuilder.addVertex(9): not translated, it uses Minecraft's `Pose`.
    }

    /// `BlockEntityFramingRenderer` (com/jaquadro/minecraft/storagedrawers/client/renderer/BlockEntityFramingRenderer.java)
    pub mod block_entity_framing_renderer {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockEntityRenderer`. Its static parts are below.
        // BlockEntityFramingRenderer.BlockEntityFramingRenderer(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityFramingRenderer.createRenderState(0): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityFramingRenderer.extractRenderState(5): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityFramingRenderer.getRenderState(3): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityFramingRenderer.submit(4): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityFramingRenderer.renderSlot(9): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
        // BlockEntityFramingRenderer.getRenderBoundingBox(1): not translated, its class isn't translated (it implements Minecraft's `BlockEntityRenderer`).
    }

    /// `DrawersRenderState` (com/jaquadro/minecraft/storagedrawers/client/renderer/state/DrawersRenderState.java)
    pub mod drawers_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `DrawersRenderState.SlotState` (com/jaquadro/minecraft/storagedrawers/client/renderer/state/DrawersRenderState.java)
    pub mod drawers_render_state_slot_state {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStackRenderState`. Its static parts are below.
        // DrawersRenderState.SlotState.SlotState(3): not translated, its class isn't translated (it uses Minecraft's `ItemStackRenderState`).
        // DrawersRenderState.SlotState.itemState(0): not translated, its class isn't translated (it uses Minecraft's `ItemStackRenderState`).
        // DrawersRenderState.SlotState.count(0): not translated, its class isn't translated (it uses Minecraft's `ItemStackRenderState`).
        // DrawersRenderState.SlotState.limit(0): not translated, its class isn't translated (it uses Minecraft's `ItemStackRenderState`).
    }

    /// `FramingRenderState` (com/jaquadro/minecraft/storagedrawers/client/renderer/state/FramingRenderState.java)
    pub mod framing_render_state {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockEntityRenderState`. Its static parts are below.
    }

    /// `ControllerBinding` (com/jaquadro/minecraft/storagedrawers/components/item/ControllerBinding.java)
    pub mod controller_binding {
        use super::*;
        pub static EMPTY: LazyLock<controller_binding::ControllerBinding> = LazyLock::new(|| controller_binding::ControllerBinding::new_4(false, 0, 0, 0));
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct ControllerBinding {
            pub valid: bool,
            pub x: i32,
            pub y: i32,
            pub z: i32,
        }
        impl ControllerBinding {
            pub fn new_4(valid: bool, x: i32, y: i32, z: i32) -> Self {
                let mut s = Self { valid: valid, x: x, y: y, z: z };
                s.valid = valid;
                s.x = x;
                s.y = y;
                s.z = z;
                s
            }
            // ControllerBinding.ControllerBinding(1): not translated, it uses Minecraft's `BlockPos`.
            // ControllerBinding.blockPos(0): not translated, it uses Minecraft's `BlockPos`.
            // ControllerBinding.equals(1): not translated, it uses `Object`.
            // ControllerBinding.hashCode(0): not translated, it calls `Objects` (java.util).hash.
            pub fn to_string(&self) -> String {
                return format!("ControllerBinding [valid={}, x={}, y={}, z={}]", self.valid, self.x, self.y, self.z);
            }
            pub fn valid(&self) -> bool {
                return self.valid;
            }
            pub fn x(&self) -> i32 {
                return self.x;
            }
            pub fn y(&self) -> i32 {
                return self.y;
            }
            pub fn z(&self) -> i32 {
                return self.z;
            }
        }
    }

    /// `DetachedDrawerContents` (com/jaquadro/minecraft/storagedrawers/components/item/DetachedDrawerContents.java)
    pub mod detached_drawer_contents {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `TooltipComponent`. Its static parts are below.
        // EMPTY: not translated, it uses DetachedDrawerContents, which isn't translated (it implements Minecraft's `TooltipComponent`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // DetachedDrawerContents.DetachedDrawerContents(4): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.DetachedDrawerContents(3): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.getItem(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.getItemPrototype(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.getItemCount(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.getStackLimit(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.isHeavy(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.equals(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.hashCode(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerContents.toString(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
    }

    /// `DrawerCountData` (com/jaquadro/minecraft/storagedrawers/components/item/DrawerCountData.java)
    pub mod drawer_count_data {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        pub static EMPTY: LazyLock<drawer_count_data::DrawerCountData> = LazyLock::new(|| drawer_count_data::DrawerCountData::new(0));
        #[derive(Clone, Debug, PartialEq)]
        pub struct DrawerCountData {
            pub count: i32,
        }
        impl DrawerCountData {
            pub fn new(count: i32) -> Self {
                let mut s = Self { count: count };
                s.count = count;
                s
            }
            // DrawerCountData.equals(1): not translated, it uses `Object`.
            // DrawerCountData.hashCode(0): not translated, it calls `Objects` (java.util).hashCode.
            pub fn to_string(&self) -> String {
                return format!("DrawerCountData [count={}]", self.count);
            }
            pub fn count(&self) -> i32 {
                return self.count;
            }
        }
    }

    /// `FrameData` (com/jaquadro/minecraft/storagedrawers/components/item/FrameData.java)
    pub mod frame_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // EMPTY: not translated, it uses FrameData, which isn't translated (it uses Minecraft's `ItemStack`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // FrameData.FrameData(4): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.FrameData(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.FrameData(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.asMaterialData(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.equals(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.toString(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.base(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.side(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.trim(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // FrameData.front(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `KeyringContents` (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java)
    pub mod keyring_contents {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `TooltipComponent`. Its static parts are below.
        // EMPTY: not translated, it uses KeyringContents, which isn't translated (it implements Minecraft's `TooltipComponent`).
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        pub const NO_STACK_INDEX: i32 = -1;
        pub const NO_SELECTED_ITEM_INDEX: i32 = -1;
        // KeyringContents.KeyringContents(2): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.KeyringContents(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.getItemUnsafe(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.itemCopyStream(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.items(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.itemsCopy(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.size(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.isEmpty(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.getSelectedItem(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.hasSelectedItem(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.getMaxShowSize(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.getShowRowSize(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.getNumberOfItemsToShow(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.equals(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.hashCode(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringContents.toString(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
    }

    /// `KeyringContents.Mutable` (com/jaquadro/minecraft/storagedrawers/components/item/KeyringContents.java)
    pub mod keyring_contents_mutable {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // KeyringContents.Mutable.Mutable(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.size(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.clearItems(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.findStackIndex(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.getMaxAmountToAdd(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.tryInsert(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.tryTransfer(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.toggleSelectedItem(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.indexIsOutsideAllowedBounds(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.removeOne(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // KeyringContents.Mutable.toImmutable(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `CompTierRegistry` (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java)
    pub mod comp_tier_registry {
        use super::*;
        // The class itself isn't translated: it uses Record, which isn't translated (it uses Minecraft's `ItemStack`). Its static parts are below.
        // INSTANCE: not translated, it uses CompTierRegistry, which isn't translated (it uses Record, which isn't translated (it uses Minecraft's `ItemStack`)).
        // CompTierRegistry.initialize(0): not translated, it uses ModCommonConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
        // CompTierRegistry.register(3): not translated, it uses Minecraft's `ItemStack`.
        // CompTierRegistry.validateRuleSyntax(1): not translated, it calls `Integer`.parseInt.
        // CompTierRegistry.register(1): not translated, it's overloaded with the same number of parameters.
        // CompTierRegistry.register(1): not translated, it's overloaded with the same number of parameters.
        // CompTierRegistry.unregisterUpperTarget(1): not translated, it uses Minecraft's `ItemStack`.
        // CompTierRegistry.unregisterLowerTarget(1): not translated, it uses Minecraft's `ItemStack`.
        // CompTierRegistry.findHigherTier(1): not translated, it uses Minecraft's `ItemStack`.
        // CompTierRegistry.findLowerTier(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `CompTierRegistry.Record` (com/jaquadro/minecraft/storagedrawers/config/CompTierRegistry.java)
    pub mod comp_tier_registry_record {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // CompTierRegistry.Record.Record(3): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `ConfigItemList` (com/jaquadro/minecraft/storagedrawers/config/ConfigItemList.java)
    pub mod config_item_list {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Item`. Its static parts are below.
        // ConfigItemList.initialize(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.innerInitialize(0): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.isListed(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.registerNamespace(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.logRegisterNamespace(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.registerItem(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.logRegisterItem(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.register(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.register(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.unregisterNamespace(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
        // ConfigItemList.unregisterItem(1): not translated, its class isn't translated (it uses Minecraft's `Item`).
    }

    /// `ConversionRegistry` (com/jaquadro/minecraft/storagedrawers/config/ConversionRegistry.java)
    pub mod conversion_registry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `TagKey`. Its static parts are below.
        // INSTANCE: not translated, it uses ConversionRegistry, which isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.initialize(0): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addBlacklist(1): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addBlacklist(2): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addBlacklist(1): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addWhitelist(1): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addWhitelist(2): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addWhitelist(3): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addWhitelist(2): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.addEquivGroup(1): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.itemsShareEquivGroup(2): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.getEquivItems(1): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
        // ConversionRegistry.isEntryValid(1): not translated, its class isn't translated (it uses Minecraft's `TagKey`).
    }

    /// `MaterialBlacklist` (com/jaquadro/minecraft/storagedrawers/config/MaterialBlacklist.java)
    pub mod material_blacklist {
        use super::*;
        // The class itself isn't translated: it extends `ConfigItemList`. Its static parts are below.
        // INSTANCE: not translated, it uses MaterialBlacklist, which isn't translated (it extends `ConfigItemList`).
        // MaterialBlacklist.innerInitialize(0): not translated, its class isn't translated (it extends `ConfigItemList`).
        // MaterialBlacklist.isBlacklisted(1): not translated, its class isn't translated (it extends `ConfigItemList`).
        // MaterialBlacklist.logRegisterNamespace(1): not translated, its class isn't translated (it extends `ConfigItemList`).
        // MaterialBlacklist.logRegisterItem(1): not translated, its class isn't translated (it extends `ConfigItemList`).
    }

    /// `ModClientConfig` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java)
    pub mod mod_client_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config). Its static parts are below.
        // INSTANCE: not translated, it uses ModClientConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
        // ModClientConfig.ModClientConfig(0): not translated, its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
        // ModClientConfig.context(0): not translated, its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
        // ModClientConfig.init(0): not translated, its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
    }

    /// `ModClientConfig.ConfigSection` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java)
    pub mod mod_client_config_config_section {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigSection {
            pub name: String,
            pub comment: Option<Vec<String>>,
        }
        impl ConfigSection {
            pub fn new(name: &str, comment: &[String]) -> Self {
                let mut s = Self { name: String::new(), comment: Some(comment.to_vec()) };
                s.name = name.to_string();
                s.comment = Some(comment.to_vec());
                s
            }
            // ModClientConfig.ConfigSection.build(0): not translated, unknown static ModClientConfig.this.
            pub fn build_entries(&self) {
            }
        }
    }

    /// `ModClientConfig.General` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java)
    pub mod mod_client_config_general {
        use super::*;
        // The class itself isn't translated: it uses ConfigEntry, which isn't translated (it's generic). Its static parts are below.
        // ModClientConfig.General.General(0): not translated, its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)).
    }

    /// `ModClientConfig.Integration` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java)
    pub mod mod_client_config_integration {
        use super::*;
        // The class itself isn't translated: it uses ConfigEntry, which isn't translated (it's generic). Its static parts are below.
        // ModClientConfig.Integration.Integration(0): not translated, its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)).
    }

    /// `ModClientConfig.Render` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java)
    pub mod mod_client_config_render {
        use super::*;
        // The class itself isn't translated: it uses ConfigEntry, which isn't translated (it's generic). Its static parts are below.
        // ModClientConfig.Render.Render(0): not translated, its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)).
    }

    /// `ModClientConfig.Render.FramedDrawers` (com/jaquadro/minecraft/storagedrawers/config/ModClientConfig.java)
    pub mod mod_client_config_render_framed_drawers {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModClientConfig.Render.FramedDrawers.FramedDrawers(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModClientConfig.Render.FramedDrawers.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModClientConfig.Render.FramedDrawers.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config). Its static parts are below.
        // INSTANCE: not translated, it uses ModCommonConfig, which isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
        // ModCommonConfig.ModCommonConfig(0): not translated, its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
        // ModCommonConfig.context(0): not translated, its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
        // ModCommonConfig.init(0): not translated, its class isn't translated (it extends `ConfigSpec` (com.texelsaurus.minecraft.chameleon.config)).
    }

    /// `ModCommonConfig.ConfigSection` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_config_section {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ConfigSection {
            pub name: String,
            pub comment: Option<Vec<String>>,
        }
        impl ConfigSection {
            pub fn new(name: &str, comment: &[String]) -> Self {
                let mut s = Self { name: String::new(), comment: Some(comment.to_vec()) };
                s.name = name.to_string();
                s.comment = Some(comment.to_vec());
                s
            }
            // ModCommonConfig.ConfigSection.build(0): not translated, unknown static ModCommonConfig.this.
            pub fn build_entries(&self) {
            }
        }
    }

    /// `ModCommonConfig.Controller` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_controller {
        use super::*;
        // The class itself isn't translated: it uses ConfigEntry, which isn't translated (it's generic). Its static parts are below.
        // ModCommonConfig.Controller.Controller(0): not translated, its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)).
    }

    /// `ModCommonConfig.Drawers` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drawers {
        use super::*;
        // The class itself isn't translated: it uses DrawerConfig, which isn't translated (it extends `ConfigSection`). Its static parts are below.
        // ModCommonConfig.Drawers.Drawers(0): not translated, its class isn't translated (it uses DrawerConfig, which isn't translated (it extends `ConfigSection`)).
        // ModCommonConfig.Drawers.anyHeavyDrawers(0): not translated, its class isn't translated (it uses DrawerConfig, which isn't translated (it extends `ConfigSection`)).
        // ModCommonConfig.Drawers.getBaseStackStorage(0): not translated, its class isn't translated (it uses DrawerConfig, which isn't translated (it extends `ConfigSection`)).
    }

    /// `ModCommonConfig.Drawers.Compacting` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drawers_compacting {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Drawers.Compacting.Compacting(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Compacting.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Compacting.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Drawers.Detached` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drawers_detached {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Drawers.Detached.Detached(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Detached.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Detached.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Drawers.DrawerConfig` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drawers_drawer_config {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Drawers.DrawerConfig.DrawerConfig(3): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.DrawerConfig.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.DrawerConfig.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.DrawerConfig.getUnitsPerSlot(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Drawers.Filled` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drawers_filled {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Drawers.Filled.Filled(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Filled.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Filled.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Drawers.Framed` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drawers_framed {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Drawers.Framed.Framed(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Framed.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Framed.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Drawers.Storage` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drawers_storage {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Drawers.Storage.Storage(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Storage.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Drawers.Storage.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.DropMode` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_drop_mode {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DropMode {
            Keep,
            Drop,
            Void,
        }
        impl DropMode {
            pub const VALUES: [DropMode; 3] = [DropMode::Keep, DropMode::Drop, DropMode::Void, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DropMode::Keep => "KEEP",
                    DropMode::Drop => "DROP",
                    DropMode::Void => "VOID",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "KEEP" => DropMode::Keep,
                    "DROP" => DropMode::Drop,
                    "VOID" => DropMode::Void,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        pub fn from_value_ignore_case(value: &str) -> mod_common_config_drop_mode::DropMode {
            if (j_compare(&value.to_lowercase(), &"KEEP".to_lowercase())) == 0 {
                return mod_common_config_drop_mode::DropMode::Keep;
            } else if (j_compare(&value.to_lowercase(), &"DROP".to_lowercase())) == 0 {
                return mod_common_config_drop_mode::DropMode::Drop;
            } else {
                return if (j_compare(&value.to_lowercase(), &"VOID".to_lowercase())) == 0 { mod_common_config_drop_mode::DropMode::Void } else { mod_common_config_drop_mode::DropMode::Keep };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
    }

    /// `ModCommonConfig.General` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_general {
        use super::*;
        // The class itself isn't translated: it uses ConfigEntry, which isn't translated (it's generic). Its static parts are below.
        // ModCommonConfig.General.General(0): not translated, its class isn't translated (it uses ConfigEntry, which isn't translated (it's generic)).
    }

    /// `ModCommonConfig.Integration` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_integration {
        use super::*;
        // The class itself isn't translated: it uses Waila, which isn't translated (it extends `ConfigSection`). Its static parts are below.
        // ModCommonConfig.Integration.Integration(0): not translated, its class isn't translated (it uses Waila, which isn't translated (it extends `ConfigSection`)).
    }

    /// `ModCommonConfig.Integration.FTBChunks` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_integration_ftb_chunks {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Integration.FTBChunks.FTBChunks(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Integration.FTBChunks.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Integration.FTBChunks.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Integration.FTBTeams` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_integration_ftb_teams {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Integration.FTBTeams.FTBTeams(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Integration.FTBTeams.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Integration.FTBTeams.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Integration.Waila` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_integration_waila {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Integration.Waila.Waila(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Integration.Waila.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Integration.Waila.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Mode` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_mode {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Mode {
            None,
            List,
            All,
        }
        impl Mode {
            pub const VALUES: [Mode; 3] = [Mode::None, Mode::List, Mode::All, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Mode::None => "NONE",
                    Mode::List => "LIST",
                    Mode::All => "ALL",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Mode::None,
                    "LIST" => Mode::List,
                    "ALL" => Mode::All,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        pub fn from_value_ignore_case(value: &str) -> mod_common_config_mode::Mode {
            if (j_compare(&value.to_lowercase(), &"NONE".to_lowercase())) == 0 {
                return mod_common_config_mode::Mode::None;
            } else if (j_compare(&value.to_lowercase(), &"LIST".to_lowercase())) == 0 {
                return mod_common_config_mode::Mode::List;
            } else {
                return if (j_compare(&value.to_lowercase(), &"ALL".to_lowercase())) == 0 { mod_common_config_mode::Mode::All } else { mod_common_config_mode::Mode::List };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
    }

    /// `ModCommonConfig.Tools` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_tools {
        use super::*;
        // The class itself isn't translated: it uses Key, which isn't translated (it extends `ConfigSection`). Its static parts are below.
        // ModCommonConfig.Tools.Tools(0): not translated, its class isn't translated (it uses Key, which isn't translated (it extends `ConfigSection`)).
    }

    /// `ModCommonConfig.Tools.Key` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_tools_key {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Tools.Key.Key(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Tools.Key.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Tools.Key.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Tools.QuantifyKey` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_tools_quantify_key {
        use super::*;
        // The class itself isn't translated: it extends `Key`. Its static parts are below.
        // ModCommonConfig.Tools.QuantifyKey.QuantifyKey(2): not translated, its class isn't translated (it extends `Key`).
        // ModCommonConfig.Tools.QuantifyKey.buildEntries(0): not translated, its class isn't translated (it extends `Key`).
        // ModCommonConfig.Tools.QuantifyKey.build(0): not translated, its class isn't translated (it extends `Key`).
    }

    /// `ModCommonConfig.Upgrades` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades {
        use super::*;
        // The class itself isn't translated: it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`). Its static parts are below.
        // ModCommonConfig.Upgrades.Upgrades(0): not translated, its class isn't translated (it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`)).
        // ModCommonConfig.Upgrades.getStorageUpgrade(1): not translated, its class isn't translated (it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`)).
        // ModCommonConfig.Upgrades.getLevelMult(1): not translated, its class isn't translated (it uses StorageTierUpgrade, which isn't translated (it extends `Upgrade`)).
    }

    /// `ModCommonConfig.Upgrades.ConversionUpgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_conversion_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `Upgrade`. Its static parts are below.
        // ModCommonConfig.Upgrades.ConversionUpgrade.ConversionUpgrade(2): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.ConversionUpgrade.buildEntries(0): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.ConversionUpgrade.build(0): not translated, its class isn't translated (it extends `Upgrade`).
    }

    /// `ModCommonConfig.Upgrades.IlluminationUpgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_illumination_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `Upgrade`. Its static parts are below.
        // ModCommonConfig.Upgrades.IlluminationUpgrade.IlluminationUpgrade(2): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.IlluminationUpgrade.buildEntries(0): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.IlluminationUpgrade.build(0): not translated, its class isn't translated (it extends `Upgrade`).
    }

    /// `ModCommonConfig.Upgrades.MagnetTierUpgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_magnet_tier_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `Upgrade`. Its static parts are below.
        // ModCommonConfig.Upgrades.MagnetTierUpgrade.MagnetTierUpgrade(4): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.MagnetTierUpgrade.buildEntries(0): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.MagnetTierUpgrade.build(0): not translated, its class isn't translated (it extends `Upgrade`).
    }

    /// `ModCommonConfig.Upgrades.MagnetUpgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_magnet_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Upgrades.MagnetUpgrade.MagnetUpgrade(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Upgrades.MagnetUpgrade.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Upgrades.MagnetUpgrade.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `ModCommonConfig.Upgrades.RedstoneUpgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_redstone_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `Upgrade`. Its static parts are below.
        // ModCommonConfig.Upgrades.RedstoneUpgrade.RedstoneUpgrade(2): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.RedstoneUpgrade.buildEntries(0): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.RedstoneUpgrade.build(0): not translated, its class isn't translated (it extends `Upgrade`).
    }

    /// `ModCommonConfig.Upgrades.RemoteUpgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_remote_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `Upgrade`. Its static parts are below.
        // ModCommonConfig.Upgrades.RemoteUpgrade.RemoteUpgrade(2): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.RemoteUpgrade.buildEntries(0): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.RemoteUpgrade.build(0): not translated, its class isn't translated (it extends `Upgrade`).
    }

    /// `ModCommonConfig.Upgrades.StorageTierUpgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_storage_tier_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `Upgrade`. Its static parts are below.
        // ModCommonConfig.Upgrades.StorageTierUpgrade.StorageTierUpgrade(3): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.StorageTierUpgrade.buildEntries(0): not translated, its class isn't translated (it extends `Upgrade`).
        // ModCommonConfig.Upgrades.StorageTierUpgrade.build(0): not translated, its class isn't translated (it extends `Upgrade`).
    }

    /// `ModCommonConfig.Upgrades.Upgrade` (com/jaquadro/minecraft/storagedrawers/config/ModCommonConfig.java)
    pub mod mod_common_config_upgrades_upgrade {
        use super::*;
        // The class itself isn't translated: it extends `ConfigSection`. Its static parts are below.
        // ModCommonConfig.Upgrades.Upgrade.Upgrade(2): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Upgrades.Upgrade.buildEntries(0): not translated, its class isn't translated (it extends `ConfigSection`).
        // ModCommonConfig.Upgrades.Upgrade.build(0): not translated, its class isn't translated (it extends `ConfigSection`).
    }

    /// `PlayerConfig` (com/jaquadro/minecraft/storagedrawers/config/PlayerConfig.java)
    pub mod player_config {
        use super::*;
        // serverPlayerConfigSettings: not translated, it uses `UUID` (java.util).
        #[derive(Clone, Debug)]
        pub struct PlayerConfig {
        }
        impl PlayerConfig {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PlayerConfig.getBooleanValue(2): not translated, it uses Minecraft's `Player`.
        // PlayerConfig.getInvertShift(1): not translated, it uses Minecraft's `Player`.
        // PlayerConfig.getInvertClick(1): not translated, it uses Minecraft's `Player`.
    }

    /// `PlayerConfigSetting` (com/jaquadro/minecraft/storagedrawers/config/PlayerConfigSetting.java)
    pub mod player_config_setting {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // PlayerConfigSetting.PlayerConfigSetting(3): not translated, its class isn't translated (it's generic).
        // PlayerConfigSetting.getValue(0): not translated, its class isn't translated (it's generic).
    }

    /// `StorageBlacklist` (com/jaquadro/minecraft/storagedrawers/config/StorageBlacklist.java)
    pub mod storage_blacklist {
        use super::*;
        // The class itself isn't translated: it extends `ConfigItemList`. Its static parts are below.
        // INSTANCE: not translated, it uses StorageBlacklist, which isn't translated (it extends `ConfigItemList`).
        // StorageBlacklist.innerInitialize(0): not translated, its class isn't translated (it extends `ConfigItemList`).
        // StorageBlacklist.isBlacklisted(1): not translated, its class isn't translated (it extends `ConfigItemList`).
        // StorageBlacklist.logRegisterNamespace(1): not translated, its class isn't translated (it extends `ConfigItemList`).
        // StorageBlacklist.logRegisterItem(1): not translated, its class isn't translated (it extends `ConfigItemList`).
    }

    /// `Api` (com/jaquadro/minecraft/storagedrawers/core/Api.java)
    pub mod api {
        use super::*;
        // The class itself isn't translated: it implements `IStorageDrawersApi` (com.jaquadro.minecraft.storagedrawers.api). Its static parts are below.
        // instance: not translated, it uses IStorageDrawersApi, which isn't translated (it's an interface).
        // Api.Api(0): not translated, its class isn't translated (it implements `IStorageDrawersApi` (com.jaquadro.minecraft.storagedrawers.api)).
    }

    /// `CommonEvents` (com/jaquadro/minecraft/storagedrawers/core/CommonEvents.java)
    pub mod common_events {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CommonEvents {
        }
        impl CommonEvents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CommonEvents.init(0): not translated, it uses Fabric's `AttackBlockCallback`.EVENT.
    }

    /// `ModBlockEntities` (com/jaquadro/minecraft/storagedrawers/core/ModBlockEntities.java)
    pub mod mod_block_entities {
        use super::*;
        // BLOCK_ENTITIES: not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        // DRAWER_TYPES: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMING_TABLE_TYPES: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // STANDARD_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // STANDARD_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // STANDARD_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRACTIONAL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRACTIONAL_DRAWERS_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CONTROLLER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CONTROLLER_IO: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMING_TABLE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ModBlockEntities {
        }
        impl ModBlockEntities {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlockEntities.registerDrawerBlockEntityType(4): not translated, it's generic.
        // ModBlockEntities.registerBlockEntityType(4): not translated, it's generic.
        // ModBlockEntities.registerControllerBlockEntityType(2): not translated, it's generic.
        // ModBlockEntities.registerControllerIOBlockEntityType(2): not translated, it's generic.
        // ModBlockEntities.init(1): not translated, it uses ModBlockEntities.BLOCK_ENTITIES, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)).
        // ModBlockEntities.getDrawerTypes(0): not translated, it uses `Stream` (java.util.stream).
    }

    /// `ModBlocks` (com/jaquadro/minecraft/storagedrawers/core/ModBlocks.java)
    pub mod mod_blocks {
        use super::*;
        // BLOCKS: not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        pub static EXCLUDE_ITEMS: LazyLock<Vec<String>> = LazyLock::new(|| Vec::new());
        // OAK_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // OAK_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // OAK_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // OAK_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // OAK_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // OAK_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SPRUCE_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SPRUCE_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SPRUCE_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SPRUCE_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SPRUCE_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SPRUCE_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BIRCH_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BIRCH_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BIRCH_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BIRCH_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BIRCH_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BIRCH_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // JUNGLE_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // JUNGLE_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // JUNGLE_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // JUNGLE_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // JUNGLE_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // JUNGLE_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ACACIA_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ACACIA_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ACACIA_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ACACIA_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ACACIA_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ACACIA_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DARK_OAK_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DARK_OAK_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DARK_OAK_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DARK_OAK_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DARK_OAK_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DARK_OAK_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MANGROVE_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MANGROVE_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MANGROVE_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MANGROVE_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MANGROVE_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MANGROVE_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CHERRY_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CHERRY_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CHERRY_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CHERRY_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CHERRY_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CHERRY_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BAMBOO_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BAMBOO_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BAMBOO_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BAMBOO_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BAMBOO_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BAMBOO_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CRIMSON_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CRIMSON_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CRIMSON_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CRIMSON_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CRIMSON_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CRIMSON_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // WARPED_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // WARPED_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // WARPED_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // WARPED_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // WARPED_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // WARPED_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PALE_OAK_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PALE_OAK_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PALE_OAK_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PALE_OAK_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PALE_OAK_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PALE_OAK_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_FULL_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_FULL_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_FULL_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_HALF_DRAWERS_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_HALF_DRAWERS_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // COMPACTING_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // COMPACTING_DRAWERS_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // COMPACTING_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // COMPACTING_HALF_DRAWERS_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_COMPACTING_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_COMPACTING_DRAWERS_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_COMPACTING_HALF_DRAWERS_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_COMPACTING_HALF_DRAWERS_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // OAK_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SPRUCE_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BIRCH_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // JUNGLE_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ACACIA_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DARK_OAK_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MANGROVE_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CHERRY_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BAMBOO_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CRIMSON_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // WARPED_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PALE_OAK_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CONTROLLER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CONTROLLER_IO: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_CONTROLLER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMED_CONTROLLER_IO: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMING_TABLE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_LOCKED: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_CLAIMED: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_LOCKED_CLAIMED: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_VOID_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_SHROUD_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_SUSPEND_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MAGNET_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_PRIORITY_P1_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_PRIORITY_P2_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_PRIORITY_N1_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_PRIORITY_N2_ICON: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_INDICATOR: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_COMP_INDICATOR: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_RIGHT_LABEL: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_HOPPER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MISSING_1_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MISSING_2_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MISSING_2_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MISSING_4_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MISSING_4_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MISSING_4_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_MISSING_4_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_DRAWERS_SIDE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_DRAWERS_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_DRAWERS_FRONT: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_DRAWERS_SHADING: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_TRIM_SIDE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_TRIM_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_CONTROLLER_SIDE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_CONTROLLER_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_CONTROLLER_FRONT: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_CONTROLLER_SHADING: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_CONTROLLER_IO_SIDE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_CONTROLLER_IO_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_CONTROLLER_IO_SHADING: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_2_SIDE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_2_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_2_FRONT: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_2_SHADING: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_3_SIDE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_3_TRIM: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_3_FRONT: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // META_FRAMED_COMPDRAWERS_3_SHADING: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYBUTTON_DRAWER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYBUTTON_QUANTIFY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYBUTTON_CONCEALMENT: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ModBlocks {
        }
        impl ModBlocks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn mod_loc(name: &str) -> JIdentifier {
            return JIdentifier::new("storagedrawers", &name);
        }
        // ModBlocks.modKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // ModBlocks.getStandardConfig(2): not translated, it uses IDrawerConfig, which isn't translated (it's an interface).
        // ModBlocks.getCompConfig(1): not translated, it uses IDrawerConfig, which isn't translated (it's an interface).
        // ModBlocks.registerWoodenDrawerBlock(3): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerWoodenDrawerBlock(4): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerWoodenDrawerBlock(4): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerFramedDrawerBlock(3): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerCompactingDrawerBlock(3): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerFramedCompactingDrawerBlock(3): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerTrimBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerTrimBlock(2): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerTrimBlock(2): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerFramedTrimBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerControllerBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerControllerIOBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerFramingTableBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaTransBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaFacingBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaFacingSizedBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaFacingSizedSlotted23Block(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaFacingSizedSlotted124Block(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaFacingSizedLabelBlock(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaFacingSizedOpen2Block(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.registerMetaFacingSizedOpen3Block(1): not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ModBlocks.getWoodenBlockProperties(0): not translated, it calls Minecraft's `Properties`.of.
        // ModBlocks.getWoodenDrawerBlockProperties(0): not translated, it calls ModBlocks.getWoodenBlockProperties, which isn't translated.
        // ModBlocks.getStoneBlockProperties(0): not translated, it calls Minecraft's `Properties`.of.
        // ModBlocks.getStoneDrawerBlockProperties(0): not translated, it calls ModBlocks.getStoneBlockProperties, which isn't translated.
        // ModBlocks.init(1): not translated, it uses ModBlocks.BLOCKS, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)).
        // ModBlocks.getBlocksOfType(1): not translated, it's generic.
        // ModBlocks.getDrawers(0): not translated, it uses `Stream` (java.util.stream).
        // ModBlocks.getFramedDrawers(0): not translated, it uses `Stream` (java.util.stream).
        // ModBlocks.getControllers(0): not translated, it uses `Stream` (java.util.stream).
        // ModBlocks.getControllerIOs(0): not translated, it uses `Stream` (java.util.stream).
        // ModBlocks.getDrawersOfType(1): not translated, it's generic.
        // ModBlocks.getDrawersOfTypeAndSize(2): not translated, it's generic.
        // ModBlocks.getDrawersOfTypeAndSizeAndDepth(3): not translated, it's generic.
        // ModBlocks.getDrawersofTypeAndDepth(2): not translated, it's generic.
        // ModBlocks.getFramedTrim(0): not translated, it uses `Stream` (java.util.stream).
        // ModBlocks.getFramedBlocks(0): not translated, it uses `Stream` (java.util.stream).
        // ModBlocks.predFalse(3): not translated, it uses Minecraft's `BlockState`.
    }

    /// `ModBlockVariants` (com/jaquadro/minecraft/storagedrawers/core/ModBlockVariants.java)
    pub mod mod_block_variants {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModBlockVariants {
        }
        impl ModBlockVariants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModBlockVariants.registerVariant(2): not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        // ModBlockVariants.registerVariantItem(2): not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
    }

    /// `ModBlockVariants.VariantData` (com/jaquadro/minecraft/storagedrawers/core/ModBlockVariants.java)
    pub mod mod_block_variants_variant_data {
        use super::*;
        // The class itself isn't translated: it uses RegistryEntry, which isn't translated (it's an interface). Its static parts are below.
        // ModBlockVariants.VariantData.VariantData(1): not translated, its class isn't translated (it uses RegistryEntry, which isn't translated (it's an interface)).
    }

    /// `ModContainers` (com/jaquadro/minecraft/storagedrawers/core/ModContainers.java)
    pub mod mod_containers {
        use super::*;
        // CONTAINERS: not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        // DRAWER_CONTAINER_1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DRAWER_CONTAINER_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DRAWER_CONTAINER_4: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DRAWER_CONTAINER_COMP_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DRAWER_CONTAINER_COMP_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAMING_TABLE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ModContainers {
        }
        impl ModContainers {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModContainers.init(1): not translated, it uses ModContainers.CONTAINERS, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)).
    }

    /// `ModCreativeTabs` (com/jaquadro/minecraft/storagedrawers/core/ModCreativeTabs.java)
    pub mod mod_creative_tabs {
        use super::*;
        // MAIN: not translated, it uses Minecraft's `ResourceKey`.
        // MAIN_TAB: not translated, it uses Minecraft's `CreativeModeTab`.
        #[derive(Clone, Debug)]
        pub struct ModCreativeTabs {
        }
        impl ModCreativeTabs {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModCreativeTabs.init(1): not translated, it uses Minecraft's `BuiltInRegistries`.CREATIVE_MODE_TAB.
    }

    /// `ModDataComponents` (com/jaquadro/minecraft/storagedrawers/core/ModDataComponents.java)
    pub mod mod_data_components {
        use super::*;
        // COMPONENTS: not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        // CONTROLLER_BINDING: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DRAWER_COUNT: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_CONTENTS: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FRAME_DATA: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DETACHED_DRAWER_CONTENTS: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ModDataComponents {
        }
        impl ModDataComponents {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModDataComponents.init(1): not translated, it uses ModDataComponents.COMPONENTS, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)).
    }

    /// `ModItems` (com/jaquadro/minecraft/storagedrawers/core/ModItems.java)
    pub mod mod_items {
        use super::*;
        // ITEMS: not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        // EXCLUDE_ITEMS_CREATIVE_TAB: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // OBSIDIAN_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // COPPER_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // IRON_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // GOLD_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // EMERALD_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DIAMOND_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // NETHERITE_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ONE_STACK_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // VOID_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CREATIVE_STORAGE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CREATIVE_VENDING_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // CONVERSION_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // REDSTONE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MIN_REDSTONE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MAX_REDSTONE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // ILLUMINATION_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // FILL_LEVEL_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // BALANCE_FILL_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PORTABILITY_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // HOPPER_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MAGNET_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MAGNET_UPGRADE_2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // MAGNET_UPGRADE_3: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // REMOTE_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // REMOTE_UPGRADE_BOUND: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // REMOTE_GROUP_UPGRADE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // REMOTE_GROUP_UPGRADE_BOUND: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // UPGRADE_TEMPLATE: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DETACHED_DRAWER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DETACHED_DRAWER_FULL: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DRAWER_KEY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // QUANTIFY_KEY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SHROUD_KEY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PERSONAL_KEY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PERSONAL_KEY_COFH: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PERSONAL_KEY_FTB: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PERSONAL_KEY_UNLOCK: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // SUSPEND_KEY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PRIORITY_KEY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PRIORITY_KEY_P1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PRIORITY_KEY_P2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PRIORITY_KEY_N1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PRIORITY_KEY_N2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DRAWER_PULLER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_DRAWER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_QUANTIFY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_SHROUD: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PERSONAL: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PERSONAL_COFH: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PERSONAL_FTB: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PERSONAL_UNLOCK: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_SUSPEND: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PRIORITY: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PRIORITY_P1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PRIORITY_P2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PRIORITY_N1: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PRIORITY_N2: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_PULLER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ModItems {
        }
        impl ModItems {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModItems.register(3): not translated, it's generic.
        // ModItems.init(1): not translated, it uses ModItems.EXCLUDE_ITEMS_CREATIVE_TAB, which isn't translated (it uses RegistryEntry, which isn't translated (it's an interface)).
        pub fn mod_loc(name: &str) -> JIdentifier {
            return JIdentifier::new("storagedrawers", &name);
        }
        // ModItems.modKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // ModItems.modKey(1): not translated, it uses Minecraft's `ResourceKey`.
        // ModItems.registerBlock(2): not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        // ModItems.getItemsOfType(1): not translated, it's generic.
        // ModItems.getKeys(0): not translated, it uses `Stream` (java.util.stream).
        // ModItems.getKeyrings(0): not translated, it uses `Stream` (java.util.stream).
    }

    /// `ModNetworking` (com/jaquadro/minecraft/storagedrawers/core/ModNetworking.java)
    pub mod mod_networking {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonInit` (com.texelsaurus.minecraft.chameleon.api). Its static parts are below.
        // INSTANCE: not translated, it uses ModNetworking, which isn't translated (it implements `ChameleonInit` (com.texelsaurus.minecraft.chameleon.api)).
        // ModNetworking.init(1): not translated, its class isn't translated (it implements `ChameleonInit` (com.texelsaurus.minecraft.chameleon.api)).
    }

    /// `ModRecipes` (com/jaquadro/minecraft/storagedrawers/core/ModRecipes.java)
    pub mod mod_recipes {
        use super::*;
        // RECIPES: not translated, it uses ChameleonRegistry, which isn't translated (it's an interface).
        // UPGRADE_RECIPE_SERIALIZER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // KEYRING_RECIPE_SERIALIZER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // REMOTE_GROUP_UPGRADE_SERIALIZER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // DETACHED_UPGRADE_RECIPE_SERIALIZER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        // PERSONAL_KEY_RECIPE_SERIALIZER: not translated, it uses RegistryEntry, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ModRecipes {
        }
        impl ModRecipes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModRecipes.init(1): not translated, it uses ModRecipes.RECIPES, which isn't translated (it uses ChameleonRegistry, which isn't translated (it's an interface)).
    }

    /// `ModSecurity` (com/jaquadro/minecraft/storagedrawers/core/ModSecurity.java)
    pub mod mod_security {
        use super::*;
        // registry: not translated, it uses SecurityRegistry, which isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)).
        #[derive(Clone, Debug)]
        pub struct ModSecurity {
        }
        impl ModSecurity {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `PlayerEventListener` (com/jaquadro/minecraft/storagedrawers/core/PlayerEventListener.java)
    pub mod player_event_listener {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PlayerEventListener {
        }
        impl PlayerEventListener {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PlayerEventListener.applyDebuff(1): not translated, it uses Minecraft's `Player`.
        // PlayerEventListener.onPlayerTick(1): not translated, it uses Minecraft's `Player`.
        // PlayerEventListener.checkItemDebuf(2): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `AddUpgradeRecipe` (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java)
    pub mod add_upgrade_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CustomRecipe`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // AddUpgradeRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // AddUpgradeRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // AddUpgradeRecipe.findContext(2): not translated, its body couldn't be read: an anonymous class.
        // AddUpgradeRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
    }

    /// `AddUpgradeRecipe.Context` (com/jaquadro/minecraft/storagedrawers/core/recipe/AddUpgradeRecipe.java)
    pub mod add_upgrade_recipe_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
    }

    /// `KeyringRecipe` (com/jaquadro/minecraft/storagedrawers/core/recipe/KeyringRecipe.java)
    pub mod keyring_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CustomRecipe`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // PATTERN: not translated, it uses Minecraft's `ShapedRecipePattern`.
        // KeyringRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // KeyringRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // KeyringRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
    }

    /// `PersonalKeyRecipe` (com/jaquadro/minecraft/storagedrawers/core/recipe/PersonalKeyRecipe.java)
    pub mod personal_key_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CustomRecipe`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // PersonalKeyRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // PersonalKeyRecipe.findPersonalKey(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // PersonalKeyRecipe.checkPersonalKey(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // PersonalKeyRecipe.ftbEnabled(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // PersonalKeyRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // PersonalKeyRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
    }

    /// `RemoteGroupUpgradeRecipe` (com/jaquadro/minecraft/storagedrawers/core/recipe/RemoteGroupUpgradeRecipe.java)
    pub mod remote_group_upgrade_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CustomRecipe`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // PATTERN: not translated, it uses Minecraft's `ShapedRecipePattern`.
        // RemoteGroupUpgradeRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // RemoteGroupUpgradeRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // RemoteGroupUpgradeRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
    }

    /// `UpgradeDetachedDrawerRecipe` (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java)
    pub mod upgrade_detached_drawer_recipe {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `CustomRecipe`. Its static parts are below.
        // MAP_CODEC: not translated, it uses Minecraft's `MapCodec`.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // UpgradeDetachedDrawerRecipe.matches(2): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // UpgradeDetachedDrawerRecipe.assemble(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // UpgradeDetachedDrawerRecipe.findContext(1): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
        // UpgradeDetachedDrawerRecipe.getSerializer(0): not translated, its class isn't translated (it extends Minecraft's `CustomRecipe`).
    }

    /// `UpgradeDetachedDrawerRecipe.Context` (com/jaquadro/minecraft/storagedrawers/core/recipe/UpgradeDetachedDrawerRecipe.java)
    pub mod upgrade_detached_drawer_recipe_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
    }

    /// `DrawerOverlay` (com/jaquadro/minecraft/storagedrawers/integration/DrawerOverlay.java)
    pub mod drawer_overlay {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DrawerOverlay {
            pub show_content: bool,
            pub show_stack_limit: bool,
            pub show_status: bool,
            pub show_stack_remainder: bool,
            pub respect_quantify_key: bool,
        }
        impl DrawerOverlay {
            // DrawerOverlay.getOverlay(1): not translated, it uses Minecraft's `Component`.
            // DrawerOverlay.addContent(3): not translated, it uses Minecraft's `Component`.
            // DrawerOverlay.addStackLimit(3): not translated, it uses Minecraft's `Component`.
            // DrawerOverlay.addStatus(3): not translated, it uses Minecraft's `Component`.
        }
    }

    /// `IntegrationModule` (com/jaquadro/minecraft/storagedrawers/integration/IntegrationModule.java)
    pub mod integration_module {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct IntegrationModule {
        }
        impl IntegrationModule {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // IntegrationModule.getModID(0): not translated, it has no body.
            pub fn version_check(&self) -> bool {
                // pattern: always null in Horizon (versionPattern only returns null there).
                return true;
            }
            pub fn version_pattern(&self) -> Option<String> {
                return None;
            }
            // IntegrationModule.init(0): not translated, it has no body.
            // IntegrationModule.postInit(0): not translated, it has no body.
        }
    }

    /// `IntegrationRegistry` (com/jaquadro/minecraft/storagedrawers/integration/IntegrationRegistry.java)
    pub mod integration_registry {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct IntegrationRegistry {
            pub mod_id: String,
            pub registry: Vec<integration_module::IntegrationModule>,
        }
        impl IntegrationRegistry {
            pub fn new(mod_id: &str) -> Self {
                let mut s = Self { mod_id: String::new(), registry: Vec::new() };
                s.mod_id = mod_id.to_string();
                s.registry = Vec::new();
                s
            }
            pub fn add(&mut self, module: &integration_module::IntegrationModule) {
                if module.version_check() {
                    self.registry.push(module.clone());
                }
            }
            // IntegrationRegistry.init(0): not translated, it calls IntegrationModule.getModID, which isn't translated.
            // IntegrationRegistry.postInit(0): not translated, it calls IntegrationModule.postInit, which isn't translated.
            // IntegrationRegistry.isModLoaded(1): not translated, it calls IntegrationModule.getModID, which isn't translated.
        }
    }

    /// `LocalIntegrationRegistry` (com/jaquadro/minecraft/storagedrawers/integration/LocalIntegrationRegistry.java)
    pub mod local_integration_registry {
        use super::*;
        pub static INSTANCE: LazyLock<JCell<Option<local_integration_registry::LocalIntegrationRegistry>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct LocalIntegrationRegistry {
            pub registry: integration_registry::IntegrationRegistry,
        }
        impl LocalIntegrationRegistry {
            pub fn new() -> Self {
                let mut s = Self { registry: integration_registry::IntegrationRegistry::new("storagedrawers") };
                s
            }
        }
        // LocalIntegrationRegistry.initialize(0): not translated, it calls LocalIntegrationRegistry.instance, which isn't translated.
        // LocalIntegrationRegistry.isModLoaded(1): not translated, a nullable object kept in a changing static.
        // LocalIntegrationRegistry.instance(0): not translated, a nullable object kept in a changing static.
    }

    /// `Waila` (com/jaquadro/minecraft/storagedrawers/integration/Waila.java)
    pub mod waila {
        use super::*;
        // The class itself isn't translated: it implements `IWailaPlugin` (snownee.jade.api). Its static parts are below.
        // Waila.registerClient(1): not translated, its class isn't translated (it implements `IWailaPlugin` (snownee.jade.api)).
    }

    /// `Waila.WailaDrawer` (com/jaquadro/minecraft/storagedrawers/integration/Waila.java)
    pub mod waila_waila_drawer {
        use super::*;
        // The class itself isn't translated: it implements `IBlockComponentProvider` (snownee.jade.api). Its static parts are below.
        // Waila.WailaDrawer.getIcon(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // Waila.WailaDrawer.appendTooltip(3): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
        // Waila.WailaDrawer.getUid(0): not translated, its class isn't translated (it implements `IBlockComponentProvider` (snownee.jade.api)).
    }

    /// `ContainerDrawers` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers.java)
    pub mod container_drawers {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerMenu`. Its static parts are below.
        pub const INVENTORY_X: i32 = 8;
        pub const INVENTORY_Y: i32 = 117;
        pub const HOTBAR_Y: i32 = 175;
        pub const UPGRADE_X: i32 = 26;
        pub const UPGRADE_Y: i32 = 86;
        // ContainerDrawers.ContainerDrawers(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.getBlockEntity(2): not translated, it uses BlockEntityDrawers, which isn't translated (it extends `BaseBlockEntity`).
        // ContainerDrawers.ContainerDrawers(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.getStorageSlotX(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.getStorageSlotY(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.getStorageSlots(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.getUpgradeSlots(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.getStackCapacity(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerDrawers.quickMoveStack(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
    }

    /// `ContainerDrawers1` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers1.java)
    pub mod container_drawers1 {
        use super::*;
        // The class itself isn't translated: it extends `ContainerDrawers`. Its static parts are below.
        pub static SLOT_COORDINATES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![80, 36]]);
        // ContainerDrawers1.ContainerDrawers1(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers1.ContainerDrawers1(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers1.getStorageSlotX(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers1.getStorageSlotY(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
    }

    /// `ContainerDrawers2` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers2.java)
    pub mod container_drawers2 {
        use super::*;
        // The class itself isn't translated: it extends `ContainerDrawers`. Its static parts are below.
        pub static SLOT_COORDINATES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![80, 23], vec![80, 49]]);
        // ContainerDrawers2.ContainerDrawers2(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers2.ContainerDrawers2(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers2.getStorageSlotX(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers2.getStorageSlotY(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
    }

    /// `ContainerDrawers4` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawers4.java)
    pub mod container_drawers4 {
        use super::*;
        // The class itself isn't translated: it extends `ContainerDrawers`. Its static parts are below.
        pub static SLOT_COORDINATES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![67, 23], vec![93, 23], vec![67, 49], vec![93, 49]]);
        // ContainerDrawers4.ContainerDrawers4(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers4.ContainerDrawers4(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers4.getStorageSlotX(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawers4.getStorageSlotY(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
    }

    /// `ContainerDrawersComp2` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp2.java)
    pub mod container_drawers_comp2 {
        use super::*;
        // The class itself isn't translated: it extends `ContainerDrawers`. Its static parts are below.
        pub static SLOT_COORDINATES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![80, 23], vec![67, 49]]);
        // ContainerDrawersComp2.ContainerDrawersComp2(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawersComp2.ContainerDrawersComp2(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawersComp2.getStorageSlotX(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawersComp2.getStorageSlotY(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
    }

    /// `ContainerDrawersComp3` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerDrawersComp3.java)
    pub mod container_drawers_comp3 {
        use super::*;
        // The class itself isn't translated: it extends `ContainerDrawers`. Its static parts are below.
        pub static SLOT_COORDINATES: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![80, 23], vec![67, 49], vec![93, 49]]);
        // ContainerDrawersComp3.ContainerDrawersComp3(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawersComp3.ContainerDrawersComp3(3): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawersComp3.getStorageSlotX(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
        // ContainerDrawersComp3.getStorageSlotY(1): not translated, its class isn't translated (it extends `ContainerDrawers`).
    }

    /// `ContainerFramingTable` (com/jaquadro/minecraft/storagedrawers/inventory/ContainerFramingTable.java)
    pub mod container_framing_table {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerMenu`. Its static parts are below.
        pub const INVENTORY_X: i32 = 8;
        pub const INVENTORY_Y: i32 = 84;
        pub const HOTBAR_Y: i32 = 142;
        pub const INPUT_X: i32 = 23;
        pub const INPUT_Y: i32 = 35;
        pub const MATERIAL_SIDE_X: i32 = 50;
        pub const MATERIAL_SIDE_Y: i32 = 17;
        pub const MATERIAL_TRIM_X: i32 = 102;
        pub const MATERIAL_TRIM_Y: i32 = 17;
        pub const MATERIAL_FRONT_X: i32 = 50;
        pub const MATERIAL_FRONT_Y: i32 = 53;
        pub const OUTPUT_X: i32 = 133;
        pub const OUTPUT_Y: i32 = 35;
        // ContainerFramingTable.ContainerFramingTable(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerFramingTable.ContainerFramingTable(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerFramingTable.quickMoveStack(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
        // ContainerFramingTable.stillValid(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerMenu`).
    }

    /// `CraftResultSlot` (com/jaquadro/minecraft/storagedrawers/inventory/CraftResultSlot.java)
    pub mod craft_result_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // CraftResultSlot.CraftResultSlot(7): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // CraftResultSlot.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // CraftResultSlot.remove(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // CraftResultSlot.onTake(2): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // CraftResultSlot.onQuickCraft(2): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `DrawerInventoryHelper` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerInventoryHelper.java)
    pub mod drawer_inventory_helper {
        use super::*;
        // RANDOM: not translated, it creates `Random` (java.util).
        #[derive(Clone, Debug)]
        pub struct DrawerInventoryHelper {
        }
        impl DrawerInventoryHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DrawerInventoryHelper.dropUpgradeItems(3): not translated, it uses Minecraft's `Level`.
        // DrawerInventoryHelper.dropInventoryItems(3): not translated, it uses Minecraft's `Level`.
        // DrawerInventoryHelper.spawnItemStack(5): not translated, it uses Minecraft's `Level`.
    }

    /// `DrawerScreen` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java)
    pub mod drawer_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerScreen`. Its static parts are below.
        pub static GUI_TEXTURES1: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("textures/gui/drawers_1.png"));
        pub static GUI_TEXTURES2: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("textures/gui/drawers_2.png"));
        pub static GUI_TEXTURES4: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("textures/gui/drawers_4.png"));
        pub static GUI_TEXTURES_COMP2: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("textures/gui/drawers_comp_2.png"));
        pub static GUI_TEXTURES_COMP3: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("textures/gui/drawers_comp.png"));
        pub const SM_DISABLED_X: i32 = 176;
        pub const SM_DISABLED_Y: i32 = 0;
        pub const SM_MISSING_Y: i32 = 16;
        // DrawerScreen.DrawerScreen(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.extractLabels(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.extractSlot(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.renderDrawerBar(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.renderDrawerCooldown(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.renderDrawerCount(5): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // DrawerScreen.isHovering(6): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
    }

    /// `DrawerScreen.Compacting2` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java)
    pub mod drawer_screen_compacting2 {
        use super::*;
        // The class itself isn't translated: it extends `DrawerScreen`. Its static parts are below.
        // DrawerScreen.Compacting2.Compacting2(3): not translated, its class isn't translated (it extends `DrawerScreen`).
    }

    /// `DrawerScreen.Compacting3` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java)
    pub mod drawer_screen_compacting3 {
        use super::*;
        // The class itself isn't translated: it extends `DrawerScreen`. Its static parts are below.
        // DrawerScreen.Compacting3.Compacting3(3): not translated, its class isn't translated (it extends `DrawerScreen`).
    }

    /// `DrawerScreen.Slot1` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java)
    pub mod drawer_screen_slot1 {
        use super::*;
        // The class itself isn't translated: it extends `DrawerScreen`. Its static parts are below.
        // DrawerScreen.Slot1.Slot1(3): not translated, its class isn't translated (it extends `DrawerScreen`).
    }

    /// `DrawerScreen.Slot2` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java)
    pub mod drawer_screen_slot2 {
        use super::*;
        // The class itself isn't translated: it extends `DrawerScreen`. Its static parts are below.
        // DrawerScreen.Slot2.Slot2(3): not translated, its class isn't translated (it extends `DrawerScreen`).
    }

    /// `DrawerScreen.Slot4` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerScreen.java)
    pub mod drawer_screen_slot4 {
        use super::*;
        // The class itself isn't translated: it extends `DrawerScreen`. Its static parts are below.
        // DrawerScreen.Slot4.Slot4(3): not translated, its class isn't translated (it extends `DrawerScreen`).
    }

    /// `DrawerStackStorage` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStackStorage.java)
    pub mod drawer_stack_storage {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `SingleStackStorage`. Its static parts are below.
        // DrawerStackStorage.DrawerStackStorage(2): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.updateSlot(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.getStack(0): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.setStack(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.getCapacity(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.isAccessible(0): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.getDrawerAttributes(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.checkControllerVoid(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.insert(3): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.extract(3): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.releaseSnapshot(1): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.onFinalCommit(0): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
        // DrawerStackStorage.rebalanceBalancedFill(0): not translated, its class isn't translated (it extends Fabric's `SingleStackStorage`).
    }

    /// `DrawerStorageImpl` (com/jaquadro/minecraft/storagedrawers/inventory/DrawerStorageImpl.java)
    pub mod drawer_storage_impl {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `CombinedStorage`. Its static parts are below.
        // WRAPPERS: not translated, it uses IDrawerGroup, which isn't translated (it's an interface).
        // DrawerStorageImpl.of(1): not translated, it uses DrawerStorageImpl, which isn't translated (it extends Fabric's `CombinedStorage`).
        // DrawerStorageImpl.DrawerStorageImpl(1): not translated, its class isn't translated (it extends Fabric's `CombinedStorage`).
        // DrawerStorageImpl.getSlots(0): not translated, its class isn't translated (it extends Fabric's `CombinedStorage`).
        // DrawerStorageImpl.resizeSlotList(0): not translated, its class isn't translated (it extends Fabric's `CombinedStorage`).
        // DrawerStorageImpl.getSlotCount(0): not translated, its class isn't translated (it extends Fabric's `CombinedStorage`).
        // DrawerStorageImpl.getSlot(1): not translated, its class isn't translated (it extends Fabric's `CombinedStorage`).
        // DrawerStorageImpl.getDrawer(1): not translated, its class isn't translated (it extends Fabric's `CombinedStorage`).
    }

    /// `FramingTableScreen` (com/jaquadro/minecraft/storagedrawers/inventory/FramingTableScreen.java)
    pub mod framing_table_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerScreen`. Its static parts are below.
        pub static GUI_TEXTIRES: LazyLock<JIdentifier> = LazyLock::new(|| mod_constants::loc("textures/gui/framing.png"));
        // FramingTableScreen.FramingTableScreen(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // FramingTableScreen.extractLabels(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
        // FramingTableScreen.extractBackground(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerScreen`).
    }

    /// `InventoryUpgrade` (com/jaquadro/minecraft/storagedrawers/inventory/InventoryUpgrade.java)
    pub mod inventory_upgrade {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Container`. Its static parts are below.
        pub const UPGRADE_CAPACITY: i32 = 7;
        // InventoryUpgrade.InventoryUpgrade(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.getContainerSize(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.isEmpty(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.getItem(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.removeItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.removeItemNoUpdate(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.setItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.getMaxStackSize(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.setChanged(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.stillValid(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.startOpen(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.stopOpen(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.canPlaceItem(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.clearContent(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.getStorageMultiplier(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.getStackCapacity(0): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.slotIsLocked(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.canAddUpgrade(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.canRemoveUpgrade(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.canRemoveStorageUpgrade(1): not translated, its class isn't translated (it implements Minecraft's `Container`).
        // InventoryUpgrade.canSwapUpgrade(2): not translated, its class isn't translated (it implements Minecraft's `Container`).
    }

    /// `ItemStackHelper` (com/jaquadro/minecraft/storagedrawers/inventory/ItemStackHelper.java)
    pub mod item_stack_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ItemStackHelper {
        }
        impl ItemStackHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ItemStackHelper.getTrueItem(1): needs the game, so it's in game_side.rs.
        // ItemStackHelper.getItemPrototype(1): needs the game, so it's in game_side.rs.
        // ItemStackHelper.getMaxStackSize(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackHelper.encodeItemStack(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackHelper.encodeItemStack(2): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackHelper.decodeItemStack(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackHelper.decodeItemStackPrototype(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackHelper.decodedCount(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackHelper.stripDecoding(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemStackHelper.isStackEncoded(1): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `RestrictedSlot` (com/jaquadro/minecraft/storagedrawers/inventory/RestrictedSlot.java)
    pub mod restricted_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // RestrictedSlot.RestrictedSlot(4): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // RestrictedSlot.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `SlotDrawer` (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java)
    pub mod slot_drawer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // emptyInventory: not translated, it uses Minecraft's `Container`.
        // SlotDrawer.SlotDrawer(5): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.getItem(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.set(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.onQuickCraft(2): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.getMaxStackSize(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.mayPickup(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.remove(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.getDrawerGroup(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.getDrawer(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotDrawer.isSameInventory(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `SlotDrawer.EmptyInventory` (com/jaquadro/minecraft/storagedrawers/inventory/SlotDrawer.java)
    pub mod slot_drawer_empty_inventory {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `SimpleContainer`. Its static parts are below.
        // SlotDrawer.EmptyInventory.EmptyInventory(0): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // SlotDrawer.EmptyInventory.getSlotsForFace(1): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // SlotDrawer.EmptyInventory.canPlaceItemThroughFace(3): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
        // SlotDrawer.EmptyInventory.canTakeItemThroughFace(3): not translated, its class isn't translated (it extends Minecraft's `SimpleContainer`).
    }

    /// `SlotStorage` (com/jaquadro/minecraft/storagedrawers/inventory/SlotStorage.java)
    pub mod slot_storage {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // SlotStorage.SlotStorage(4): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotStorage.SlotStorage(5): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotStorage.mayPickup(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotStorage.isMissing(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `SlotUpgrade` (com/jaquadro/minecraft/storagedrawers/inventory/SlotUpgrade.java)
    pub mod slot_upgrade {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Slot`. Its static parts are below.
        // SlotUpgrade.SlotUpgrade(4): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotUpgrade.mayPlace(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotUpgrade.mayPickup(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotUpgrade.canTakeStack(0): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotUpgrade.canTakeStack(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
        // SlotUpgrade.canSwapStack(1): not translated, its class isn't translated (it extends Minecraft's `Slot`).
    }

    /// `DetachedDrawerTooltip` (com/jaquadro/minecraft/storagedrawers/inventory/tooltip/DetachedDrawerTooltip.java)
    pub mod detached_drawer_tooltip {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `TooltipComponent`. Its static parts are below.
        // DetachedDrawerTooltip.DetachedDrawerTooltip(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // DetachedDrawerTooltip.contents(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
    }

    /// `KeyringTooltip` (com/jaquadro/minecraft/storagedrawers/inventory/tooltip/KeyringTooltip.java)
    pub mod keyring_tooltip {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `TooltipComponent`. Its static parts are below.
        // KeyringTooltip.KeyringTooltip(1): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
        // KeyringTooltip.contents(0): not translated, its class isn't translated (it implements Minecraft's `TooltipComponent`).
    }

    /// `EnumUpgradeCreative` (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeCreative.java)
    pub mod enum_upgrade_creative {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // META_LOOKUP: not translated, it uses EnumUpgradeCreative, which isn't translated (its constants carry values).
        // EnumUpgradeCreative.EnumUpgradeCreative(3): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeCreative.getMetadata(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeCreative.getUnlocalizedName(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeCreative.byMetadata(1): not translated, it uses EnumUpgradeCreative, which isn't translated (its constants carry values).
        // EnumUpgradeCreative.toString(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeCreative.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EnumUpgradeMagnet` (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeMagnet.java)
    pub mod enum_upgrade_magnet {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // META_LOOKUP: not translated, it uses EnumUpgradeMagnet, which isn't translated (its constants carry values).
        // EnumUpgradeMagnet.EnumUpgradeMagnet(4): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeMagnet.getMetadata(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeMagnet.getLevel(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeMagnet.getUnlocalizedName(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeMagnet.byMetadata(1): not translated, it uses EnumUpgradeMagnet, which isn't translated (its constants carry values).
        // EnumUpgradeMagnet.toString(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeMagnet.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EnumUpgradeRedstone` (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeRedstone.java)
    pub mod enum_upgrade_redstone {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // META_LOOKUP: not translated, it uses EnumUpgradeRedstone, which isn't translated (its constants carry values).
        // EnumUpgradeRedstone.EnumUpgradeRedstone(3): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeRedstone.getMetadata(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeRedstone.getUnlocalizedName(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeRedstone.byMetadata(1): not translated, it uses EnumUpgradeRedstone, which isn't translated (its constants carry values).
        // EnumUpgradeRedstone.toString(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeRedstone.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EnumUpgradeStorage` (com/jaquadro/minecraft/storagedrawers/item/EnumUpgradeStorage.java)
    pub mod enum_upgrade_storage {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // META_LOOKUP: not translated, it uses EnumUpgradeStorage, which isn't translated (its constants carry values).
        // LEVEL_LOOKUP: not translated, it uses EnumUpgradeStorage, which isn't translated (its constants carry values).
        // EnumUpgradeStorage.EnumUpgradeStorage(4): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeStorage.getMetadata(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeStorage.getLevel(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeStorage.getUnlocalizedName(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeStorage.byMetadata(1): not translated, it uses EnumUpgradeStorage, which isn't translated (its constants carry values).
        // EnumUpgradeStorage.byLevel(1): not translated, it uses EnumUpgradeStorage, which isn't translated (its constants carry values).
        // EnumUpgradeStorage.toString(0): not translated, its class isn't translated (its constants carry values).
        // EnumUpgradeStorage.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ItemDetachedDrawer` (com/jaquadro/minecraft/storagedrawers/item/ItemDetachedDrawer.java)
    pub mod item_detached_drawer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // ItemDetachedDrawer.ItemDetachedDrawer(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemDetachedDrawer.getDefaultInstance(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemDetachedDrawer.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemDetachedDrawer.getDescription(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemDetachedDrawer.getTooltipImage(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemDetachedDrawer.isHeavy(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemDetachedDrawer.canFitInsideContainerItems(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `ItemDrawerKey` (com/jaquadro/minecraft/storagedrawers/item/ItemDrawerKey.java)
    pub mod item_drawer_key {
        use super::*;
        // The class itself isn't translated: it extends `ItemKey`. Its static parts are below.
        // ItemDrawerKey.ItemDrawerKey(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemDrawerKey.handleDrawerAttributes(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemDrawerKey.isEnabled(0): not translated, its class isn't translated (it extends `ItemKey`).
    }

    /// `ItemDrawerPuller` (com/jaquadro/minecraft/storagedrawers/item/ItemDrawerPuller.java)
    pub mod item_drawer_puller {
        use super::*;
        // The class itself isn't translated: it extends `ItemKey`. Its static parts are below.
        // ItemDrawerPuller.ItemDrawerPuller(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemDrawerPuller.isEnabled(0): not translated, its class isn't translated (it extends `ItemKey`).
    }

    /// `ItemDrawers` (com/jaquadro/minecraft/storagedrawers/item/ItemDrawers.java)
    pub mod item_drawers {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // ItemDrawers.ItemDrawers(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemDrawers.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemDrawers.getName(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemDrawers.getDescription(0): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemDrawers.isHeavy(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemDrawers.getCapacityForBlock(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemDrawers.canFitInsideContainerItems(0): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemDrawers.doesSneakBypassUse(4): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `ItemFramedDrawers` (com/jaquadro/minecraft/storagedrawers/item/ItemFramedDrawers.java)
    pub mod item_framed_drawers {
        use super::*;
        // The class itself isn't translated: it extends `ItemDrawers`. Its static parts are below.
        // ItemFramedDrawers.ItemFramedDrawers(2): not translated, its class isn't translated (it extends `ItemDrawers`).
        // ItemFramedDrawers.placeBlock(2): not translated, its class isn't translated (it extends `ItemDrawers`).
    }

    /// `ItemFramedTrim` (com/jaquadro/minecraft/storagedrawers/item/ItemFramedTrim.java)
    pub mod item_framed_trim {
        use super::*;
        // The class itself isn't translated: it extends `ItemTrim`. Its static parts are below.
        // ItemFramedTrim.ItemFramedTrim(2): not translated, its class isn't translated (it extends `ItemTrim`).
        // ItemFramedTrim.placeBlock(2): not translated, its class isn't translated (it extends `ItemTrim`).
    }

    /// `ItemKey` (com/jaquadro/minecraft/storagedrawers/item/ItemKey.java)
    pub mod item_key {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        // ItemKey.ItemKey(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKey.createAttributes(0): not translated, it uses Minecraft's `ItemAttributeModifiers`.
        // ItemKey.canDestroyBlock(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKey.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKey.getDescription(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKey.useOn(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKey.handleDrawerAttributes(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKey.isEnabled(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `ItemKeyring` (com/jaquadro/minecraft/storagedrawers/item/ItemKeyring.java)
    pub mod item_keyring {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        pub const TAG_ITEMS: &str = "Items";
        // ItemKeyring.ItemKeyring(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.getKey(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.overrideOtherStackedOnMe(6): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.use(3): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.useOn(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.getKeyring(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemKeyring.getTooltipImage(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.getName(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.getDescription(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.onDestroyed(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemKeyring.getActiveKey(2): not translated, it uses Minecraft's `ItemStack`.
        // ItemKeyring.getContents(1): not translated, it uses `Stream` (java.util.stream).
        // ItemKeyring.rotateKeyring(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `ItemPersonalKey` (com/jaquadro/minecraft/storagedrawers/item/ItemPersonalKey.java)
    pub mod item_personal_key {
        use super::*;
        // The class itself isn't translated: it extends `ItemKey`. Its static parts are below.
        // ItemPersonalKey.ItemPersonalKey(2): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemPersonalKey.handleDrawerAttributes(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemPersonalKey.getSecurityProviderKey(0): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemPersonalKey.isEnabled(0): not translated, its class isn't translated (it extends `ItemKey`).
    }

    /// `ItemPriorityKey` (com/jaquadro/minecraft/storagedrawers/item/ItemPriorityKey.java)
    pub mod item_priority_key {
        use super::*;
        // The class itself isn't translated: it extends `ItemKey`. Its static parts are below.
        // priorityLookup: not translated, it uses ItemPriorityKey, which isn't translated (it extends `ItemKey`).
        // ItemPriorityKey.ItemPriorityKey(3): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemPriorityKey.getNextKey(0): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemPriorityKey.handleDrawerAttributes(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemPriorityKey.use(3): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemPriorityKey.isEnabled(0): not translated, its class isn't translated (it extends `ItemKey`).
    }

    /// `ItemQuantifyKey` (com/jaquadro/minecraft/storagedrawers/item/ItemQuantifyKey.java)
    pub mod item_quantify_key {
        use super::*;
        // The class itself isn't translated: it extends `ItemKey`. Its static parts are below.
        // ItemQuantifyKey.ItemQuantifyKey(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemQuantifyKey.handleDrawerAttributes(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemQuantifyKey.isEnabled(0): not translated, its class isn't translated (it extends `ItemKey`).
    }

    /// `ItemShroudKey` (com/jaquadro/minecraft/storagedrawers/item/ItemShroudKey.java)
    pub mod item_shroud_key {
        use super::*;
        // The class itself isn't translated: it extends `ItemKey`. Its static parts are below.
        // ItemShroudKey.ItemShroudKey(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemShroudKey.handleDrawerAttributes(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemShroudKey.isEnabled(0): not translated, its class isn't translated (it extends `ItemKey`).
    }

    /// `ItemSuspendKey` (com/jaquadro/minecraft/storagedrawers/item/ItemSuspendKey.java)
    pub mod item_suspend_key {
        use super::*;
        // The class itself isn't translated: it extends `ItemKey`. Its static parts are below.
        // ItemSuspendKey.ItemSuspendKey(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemSuspendKey.handleDrawerAttributes(1): not translated, its class isn't translated (it extends `ItemKey`).
        // ItemSuspendKey.isEnabled(0): not translated, its class isn't translated (it extends `ItemKey`).
    }

    /// `ItemTrim` (com/jaquadro/minecraft/storagedrawers/item/ItemTrim.java)
    pub mod item_trim {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BlockItem`. Its static parts are below.
        // ItemTrim.ItemTrim(2): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemTrim.getName(1): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
        // ItemTrim.doesSneakBypassUse(4): not translated, its class isn't translated (it extends Minecraft's `BlockItem`).
    }

    /// `ItemUpgrade` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgrade.java)
    pub mod item_upgrade {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Item`. Its static parts are below.
        pub static NEXT_GROUP_ID: JCell<i32> = JCell::new(0);
        // ItemUpgrade.ItemUpgrade(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.ItemUpgrade(2): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.getNextGroupId(0): not translated, `++` inside an expression.
        // ItemUpgrade.doesSneakBypassUse(4): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.getUpgradeGroup(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.appendHoverText(5): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.getDescription(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.isEnabled(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.setAllowMultiple(1): not translated, its class isn't translated (it extends Minecraft's `Item`).
        // ItemUpgrade.getAllowMultiple(0): not translated, its class isn't translated (it extends Minecraft's `Item`).
    }

    /// `ItemUpgradeBalance` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeBalance.java)
    pub mod item_upgrade_balance {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradeBalance.ItemUpgradeBalance(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeBalance.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeBalance.getDescription(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeConversion` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeConversion.java)
    pub mod item_upgrade_conversion {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradeConversion.ItemUpgradeConversion(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeConversion.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeCreative` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeCreative.java)
    pub mod item_upgrade_creative {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // creativeGroupId: not translated, it calls ItemUpgrade.getNextGroupId, which isn't translated.
        // ItemUpgradeCreative.ItemUpgradeCreative(2): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeCreative.ItemUpgradeCreative(3): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeCreative.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeFillLevel` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeFillLevel.java)
    pub mod item_upgrade_fill_level {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradeFillLevel.ItemUpgradeFillLevel(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeFillLevel.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeHopper` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeHopper.java)
    pub mod item_upgrade_hopper {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradeHopper.ItemUpgradeHopper(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeHopper.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeIllumination` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeIllumination.java)
    pub mod item_upgrade_illumination {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradeIllumination.ItemUpgradeIllumination(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeIllumination.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeMagnet` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeMagnet.java)
    pub mod item_upgrade_magnet {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // magnetGroupId: not translated, it calls ItemUpgrade.getNextGroupId, which isn't translated.
        // ItemUpgradeMagnet.ItemUpgradeMagnet(2): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.ItemUpgradeMagnet(3): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getDescription(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.appendHoverText(5): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.buildRangeString(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getActiveSpeed(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getIdleSpeed(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getHorzRange(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getUpRange(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getDownRange(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getIndexedRange(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeMagnet.getConfig(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeOneStack` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeOneStack.java)
    pub mod item_upgrade_one_stack {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradeOneStack.ItemUpgradeOneStack(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeOneStack.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradePortability` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradePortability.java)
    pub mod item_upgrade_portability {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradePortability.ItemUpgradePortability(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradePortability.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeRedstone` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRedstone.java)
    pub mod item_upgrade_redstone {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // redstoneGroupId: not translated, it calls ItemUpgrade.getNextGroupId, which isn't translated.
        // ItemUpgradeRedstone.ItemUpgradeRedstone(2): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeRedstone.ItemUpgradeRedstone(3): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeRedstone.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeRemote` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeRemote.java)
    pub mod item_upgrade_remote {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // remoteGroupId: not translated, it calls ItemUpgrade.getNextGroupId, which isn't translated.
        // ItemUpgradeRemote.ItemUpgradeRemote(3): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeRemote.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeRemote.appendHoverText(5): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeRemote.getBoundPosition(1): not translated, it uses Minecraft's `BlockPos`.
        // ItemUpgradeRemote.setBoundController(2): not translated, it uses Minecraft's `ItemStack`.
        // ItemUpgradeRemote.setUnbound(1): not translated, it uses Minecraft's `ItemStack`.
        // ItemUpgradeRemote.copyControllerBinding(2): not translated, it uses Minecraft's `ItemStack`.
        // ItemUpgradeRemote.getBoundController(2): not translated, it uses BlockEntityController, which isn't translated (it extends `BaseBlockEntity`).
        // ItemUpgradeRemote.validateInventory(2): not translated, it uses Minecraft's `Inventory`.
        // ItemUpgradeRemote.isGroupUpgrade(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeRemote.isBound(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeStorage` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeStorage.java)
    pub mod item_upgrade_storage {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // storageGroupId: not translated, it calls ItemUpgrade.getNextGroupId, which isn't translated.
        // ItemUpgradeStorage.ItemUpgradeStorage(2): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeStorage.ItemUpgradeStorage(3): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeStorage.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeStorage.getDescription(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ItemUpgradeVoid` (com/jaquadro/minecraft/storagedrawers/item/ItemUpgradeVoid.java)
    pub mod item_upgrade_void {
        use super::*;
        // The class itself isn't translated: it extends `ItemUpgrade`. Its static parts are below.
        // ItemUpgradeVoid.ItemUpgradeVoid(1): not translated, its class isn't translated (it extends `ItemUpgrade`).
        // ItemUpgradeVoid.isEnabled(0): not translated, its class isn't translated (it extends `ItemUpgrade`).
    }

    /// `ModConstants` (com/jaquadro/minecraft/storagedrawers/ModConstants.java)
    pub mod mod_constants {
        use super::*;
        pub const MOD_ID: &str = "storagedrawers";
        #[derive(Clone, Debug)]
        pub struct ModConstants {
        }
        impl ModConstants {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn loc(path: &str) -> JIdentifier {
            return JIdentifier::new("storagedrawers", &path);
        }
    }

    /// `ModServices` (com/jaquadro/minecraft/storagedrawers/ModServices.java)
    pub mod mod_services {
        use super::*;
        // log: the logger. Log calls become comments.
        // RESOURCE_FACTORY: not translated, it uses ResourceFactory, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ModServices {
        }
        impl ModServices {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModServices.load(1): not translated, it's generic.
    }

    /// `CountUpdateMessage` (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessage.java)
    pub mod count_update_message {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network). Its static parts are below.
        // TYPE: not translated, no constructor of Type takes 1 values.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // CountUpdateMessage.CountUpdateMessage(5): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.CountUpdateMessage(3): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.type(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.handleMessage(2): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.x(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.y(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.z(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.slot(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // CountUpdateMessage.count(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
    }

    /// `CountUpdateMessageHandler` (com/jaquadro/minecraft/storagedrawers/network/CountUpdateMessageHandler.java)
    pub mod count_update_message_handler {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CountUpdateMessageHandler {
        }
        impl CountUpdateMessageHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // CountUpdateMessageHandler.handle(1): not translated, it uses CountUpdateMessage, which isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        }
    }

    /// `PlayerBoolConfigMessage` (com/jaquadro/minecraft/storagedrawers/network/PlayerBoolConfigMessage.java)
    pub mod player_bool_config_message {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network). Its static parts are below.
        // TYPE: not translated, no constructor of Type takes 1 values.
        // STREAM_CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // PlayerBoolConfigMessage.PlayerBoolConfigMessage(3): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // PlayerBoolConfigMessage.type(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // PlayerBoolConfigMessage.handleMessage(2): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // PlayerBoolConfigMessage.uuid(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // PlayerBoolConfigMessage.key(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
        // PlayerBoolConfigMessage.value(0): not translated, its class isn't translated (it implements `ChameleonPacket` (com.texelsaurus.minecraft.chameleon.network)).
    }

    /// `DefaultInteractionProvider` (com/jaquadro/minecraft/storagedrawers/security/DefaultInteractionProvider.java)
    pub mod default_interaction_provider {
        use super::*;
        // The class itself isn't translated: it implements `IInteractionProvider` (com.jaquadro.minecraft.storagedrawers.api.security). Its static parts are below.
        // DefaultInteractionProvider.getProviderID(0): not translated, its class isn't translated (it implements `IInteractionProvider` (com.jaquadro.minecraft.storagedrawers.api.security)).
        // DefaultInteractionProvider.canInteract(3): not translated, its class isn't translated (it implements `IInteractionProvider` (com.jaquadro.minecraft.storagedrawers.api.security)).
    }

    /// `DefaultSecurityProvider` (com/jaquadro/minecraft/storagedrawers/security/DefaultSecurityProvider.java)
    pub mod default_security_provider {
        use super::*;
        // The class itself isn't translated: it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security). Its static parts are below.
        // DefaultSecurityProvider.getProviderID(0): not translated, its class isn't translated (it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security)).
        // DefaultSecurityProvider.hasOwnership(2): not translated, its class isn't translated (it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security)).
        // DefaultSecurityProvider.hasAccess(2): not translated, its class isn't translated (it implements `ISecurityProvider` (com.jaquadro.minecraft.storagedrawers.api.security)).
    }

    /// `SecurityManager` (com/jaquadro/minecraft/storagedrawers/security/SecurityManager.java)
    pub mod security_manager {
        use super::*;
        // defaultProvider: not translated, it uses ISecurityProvider, which isn't translated (it's an interface).
        // defaultInteractProvider: not translated, it uses IInteractionProvider, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct SecurityManager {
        }
        impl SecurityManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SecurityManager.hasOwnership(2): not translated, it uses Minecraft's `GameProfile`.
        // SecurityManager.hasAccess(2): not translated, it uses Minecraft's `Player`.
        // SecurityManager.canInteract(3): not translated, it uses Minecraft's `Player`.
        // SecurityManager.clientHasAccess(1): not translated, it uses IProtectable, which isn't translated (it's an interface).
    }

    /// `SecurityRegistry` (com/jaquadro/minecraft/storagedrawers/security/SecurityRegistry.java)
    pub mod security_registry {
        use super::*;
        // The class itself isn't translated: it uses ISecurityProvider, which isn't translated (it's an interface). Its static parts are below.
        // SecurityRegistry.registerProvider(1): not translated, its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)).
        // SecurityRegistry.registerProvider(1): not translated, its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)).
        // SecurityRegistry.getProvider(1): not translated, its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)).
        // SecurityRegistry.getInteractionProvider(1): not translated, its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)).
        // SecurityRegistry.getAllInteractionProviders(0): not translated, its class isn't translated (it uses ISecurityProvider, which isn't translated (it's an interface)).
    }

    /// `PlatformResourceFactory` (com/jaquadro/minecraft/storagedrawers/service/PlatformResourceFactory.java)
    pub mod platform_resource_factory {
        use super::*;
        // The class itself isn't translated: it implements `ResourceFactory`. Its static parts are below.
        // PlatformResourceFactory.createBlockEntityDrawersStandard(1): not translated, its class isn't translated (it implements `ResourceFactory`).
        // PlatformResourceFactory.createBlockEntityDrawersComp(1): not translated, its class isn't translated (it implements `ResourceFactory`).
        // PlatformResourceFactory.createBlockEntityController(0): not translated, its class isn't translated (it implements `ResourceFactory`).
        // PlatformResourceFactory.createBlockEntityControllerIO(0): not translated, its class isn't translated (it implements `ResourceFactory`).
        // PlatformResourceFactory.createBlockEntityFramingTable(0): not translated, its class isn't translated (it implements `ResourceFactory`).
        // PlatformResourceFactory.createBlockEntityTrim(0): not translated, its class isn't translated (it implements `ResourceFactory`).
    }

    /// `ResourceFactory` (com/jaquadro/minecraft/storagedrawers/service/ResourceFactory.java)
    pub mod resource_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ResourceFactory.createBlockEntityDrawersStandard(1): not translated, it has no body.
        // ResourceFactory.createBlockEntityDrawersComp(1): not translated, it has no body.
        // ResourceFactory.createBlockEntityController(0): not translated, it has no body.
        // ResourceFactory.createBlockEntityControllerIO(0): not translated, it has no body.
        // ResourceFactory.createBlockEntityFramingTable(0): not translated, it has no body.
        // ResourceFactory.createBlockEntityTrim(0): not translated, it has no body.
    }

    /// `StorageDrawers` (com/jaquadro/minecraft/storagedrawers/StorageDrawers.java)
    pub mod storage_drawers {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // api: not translated, it uses Api, which isn't translated (it implements `IStorageDrawersApi` (com.jaquadro.minecraft.storagedrawers.api)).
        pub static GAMEPLAY_REGISTRIES_INITIALIZED: JCell<bool> = JCell::new(false);
        // StorageDrawers.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
        pub fn rl(path: &str) -> JIdentifier {
            return JIdentifier::new("storagedrawers", &path);
        }
    }

    /// `StorageDrawersClient` (com/jaquadro/minecraft/storagedrawers/StorageDrawersClient.java)
    pub mod storage_drawers_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // StorageDrawersClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `StorageUtil` (com/jaquadro/minecraft/storagedrawers/storage/StorageUtil.java)
    pub mod storage_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct StorageUtil {
        }
        impl StorageUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StorageUtil.rebalanceDrawers(2): not translated, it's overloaded with the same number of parameters.
        // StorageUtil.rebalanceDrawers(3): not translated, it's overloaded with the same number of parameters.
        // StorageUtil.rebalanceDrawers(2): not translated, it's overloaded with the same number of parameters.
        // StorageUtil.rebalanceDrawers(3): not translated, it's overloaded with the same number of parameters.
        // StorageUtil.onNetwork(1): not translated, it uses IDrawerGroup, which isn't translated (it's an interface).
        // StorageUtil.getRebalanceDrawers(3): not translated, it uses `Stream` (java.util.stream).
        // StorageUtil.rebalanceDrawers(2): not translated, it's overloaded with the same number of parameters.
        // StorageUtil.rebalanceDrawers(1): not translated, it uses `Stream` (java.util.stream).
    }

    /// `CompactingHelper` (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java)
    pub mod compacting_helper {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Level`. Its static parts are below.
        // lookup1: not translated, it uses InventoryLookup, which isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // lookup2: not translated, it uses InventoryLookup, which isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // lookup3: not translated, it uses InventoryLookup, which isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.CompactingHelper(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // CompactingHelper.findHigherTier(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // CompactingHelper.findLowerTier(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // CompactingHelper.findAllMatchingRecipes(1): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // CompactingHelper.findMatchingModCandidate(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // CompactingHelper.tryMatch(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
        // CompactingHelper.setupLookup(2): not translated, its class isn't translated (it uses Minecraft's `Level`).
    }

    /// `CompactingHelper.InventoryLookup` (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java)
    pub mod compacting_helper_inventory_lookup {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `TransientCraftingContainer`. Its static parts are below.
        // CompactingHelper.InventoryLookup.InventoryLookup(2): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.getContainerSize(0): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.isEmpty(0): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.getItem(1): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.setItem(2): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.removeItemNoUpdate(1): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.removeItem(2): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.clearContent(0): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
        // CompactingHelper.InventoryLookup.getItems(0): not translated, its class isn't translated (it extends Minecraft's `TransientCraftingContainer`).
    }

    /// `CompactingHelper.Result` (com/jaquadro/minecraft/storagedrawers/util/CompactingHelper.java)
    pub mod compacting_helper_result {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // CompactingHelper.Result.Result(2): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // CompactingHelper.Result.getStack(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // CompactingHelper.Result.getSize(0): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
    }

    /// `ComponentUtil` (com/jaquadro/minecraft/storagedrawers/util/ComponentUtil.java)
    pub mod component_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ComponentUtil {
        }
        impl ComponentUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ComponentUtil.getSplitDescription(1): not translated, it uses Minecraft's `MutableComponent`.
        // ComponentUtil.getSplitDescription(1): not translated, it uses Minecraft's `MutableComponent`.
        // ComponentUtil.appendSplitDescription(2): not translated, it's overloaded with the same number of parameters.
        // ComponentUtil.appendSplitDescription(2): not translated, it's overloaded with the same number of parameters.
        // ComponentUtil.appendSplitDescription(3): not translated, it's overloaded with the same number of parameters.
        // ComponentUtil.appendSplitDescription(3): not translated, it's overloaded with the same number of parameters.
    }

    /// `CountFormatter` (com/jaquadro/minecraft/storagedrawers/util/CountFormatter.java)
    pub mod count_formatter {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CountFormatter {
        }
        impl CountFormatter {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CountFormatter.format(2): not translated, it's overloaded with the same number of parameters.
        // CountFormatter.format(2): not translated, it's overloaded with the same number of parameters.
        // CountFormatter.formatStackNotation(1): not translated, it uses IDrawer, which isn't translated (it's an interface).
        pub fn from_stack_notation(item_count: i32, stack_size: i32) -> String {
            if (item_count != 0) && (stack_size != 0) {
                let stacks: i32 = item_count / stack_size;
                let remainder: i32 = item_count - (stacks * stack_size);
                let mut text: String;
                if (stacks > 0) && (remainder > 0) {
                    text = format!("{}x{}+{}", stacks, stack_size, remainder);
                } else if stacks > 0 {
                    text = format!("{}x{}", stacks, stack_size);
                } else {
                    text = format!("{}", remainder);
                }
                return text.to_string();
            } else {
                return "".to_string();
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // CountFormatter.formatExact(1): not translated, it's overloaded with the same number of parameters.
        // CountFormatter.formatExact(1): not translated, it's overloaded with the same number of parameters.
        // CountFormatter.formatApprox(2): not translated, it's overloaded with the same number of parameters.
        // CountFormatter.formatApprox(2): not translated, it's overloaded with the same number of parameters.
    }

    /// `ItemCollectionRegistry` (com/jaquadro/minecraft/storagedrawers/util/ItemCollectionRegistry.java)
    pub mod item_collection_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ItemCollectionRegistry.register(2): not translated, its class isn't translated (it's generic).
        // ItemCollectionRegistry.getEntries(1): not translated, its class isn't translated (it's generic).
        // ItemCollectionRegistry.remove(1): not translated, its class isn't translated (it's generic).
        // ItemCollectionRegistry.clear(1): not translated, its class isn't translated (it's generic).
        // ItemCollectionRegistry.clear(0): not translated, its class isn't translated (it's generic).
        // ItemCollectionRegistry.entrySet(0): not translated, its class isn't translated (it's generic).
    }

    /// `ItemStackMatcher` (com/jaquadro/minecraft/storagedrawers/util/ItemStackMatcher.java)
    pub mod item_stack_matcher {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemStack`. Its static parts are below.
        // EMPTY: not translated, it uses ItemStackMatcher, which isn't translated (it uses Minecraft's `ItemStack`).
        // ItemStackMatcher.ItemStackMatcher(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // ItemStackMatcher.matches(1): not translated, its class isn't translated (it uses Minecraft's `ItemStack`).
        // ItemStackMatcher.areItemsEqual(2): not translated, it uses Minecraft's `ItemStack`.
    }

    /// `ItemStackTagMatcher` (com/jaquadro/minecraft/storagedrawers/util/ItemStackTagMatcher.java)
    pub mod item_stack_tag_matcher {
        use super::*;
        // The class itself isn't translated: it extends `ItemStackMatcher`. Its static parts are below.
        // ItemStackTagMatcher.ItemStackTagMatcher(1): not translated, its class isn't translated (it extends `ItemStackMatcher`).
        // ItemStackTagMatcher.matches(1): not translated, its class isn't translated (it extends `ItemStackMatcher`).
        // ItemStackTagMatcher.refreshTagMatches(0): not translated, its class isn't translated (it extends `ItemStackMatcher`).
    }

    /// `WorldUtils` (com/jaquadro/minecraft/storagedrawers/util/WorldUtils.java)
    pub mod world_utils {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // WorldUtils.WorldUtils(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // WorldUtils.rayTraceEyes(3): not translated, it uses Minecraft's `BlockHitResult`.
        // WorldUtils.getBlockEntity(3): not translated, it's generic.
    }

    /// `ChameleonInit` (com/texelsaurus/minecraft/chameleon/api/ChameleonInit.java)
    pub mod chameleon_init {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonInit.init(1): not translated, it has no body.
    }

    /// `ChameleonInit.InitContext` (com/texelsaurus/minecraft/chameleon/api/ChameleonInit.java)
    pub mod chameleon_init_init_context {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct InitContext {
        }
        impl InitContext {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `ChameleonCapability` (com/texelsaurus/minecraft/chameleon/capabilities/ChameleonCapability.java)
    pub mod chameleon_capability {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonCapability.id(0): not translated, it has no body.
        // ChameleonCapability.getCapability(2): not translated, it has no body.
    }

    /// `FabricCapability` (com/texelsaurus/minecraft/chameleon/capabilities/FabricCapability.java)
    pub mod fabric_capability {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FabricCapability.FabricCapability(1): not translated, its class isn't translated (it's generic).
        // FabricCapability.id(0): not translated, its class isn't translated (it's generic).
        // FabricCapability.getCapability(2): not translated, its class isn't translated (it's generic).
        // FabricCapability.getCapability(1): not translated, it's generic.
        // FabricCapability.register(2): not translated, it's generic.
    }

    /// `IFabricCapability` (com/texelsaurus/minecraft/chameleon/capabilities/IFabricCapability.java)
    pub mod i_fabric_capability {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IFabricCapability.getCapability(1): not translated, it has no body.
        // IFabricCapability.register(2): not translated, it has no body.
    }

    /// `ChameleonServices` (com/texelsaurus/minecraft/chameleon/ChameleonServices.java)
    pub mod chameleon_services {
        use super::*;
        // REGISTRY: not translated, it uses ChameleonRegistries, which isn't translated (it's an interface).
        // NETWORK: not translated, it uses ChameleonNetworking, which isn't translated (it's an interface).
        // CONFIG: not translated, it uses ChameleonConfig, which isn't translated (it's an interface).
        // CAPABILITY: not translated, it uses ChameleonCapabilities, which isn't translated (it's an interface).
        // CONTAINER: not translated, it uses ChameleonContainer, which isn't translated (it's an interface).
        // PLATFORM: not translated, it uses ChameleonPlatform, which isn't translated (it's an interface).
        // RENDER: not translated, it uses ChameleonRender, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ChameleonServices {
        }
        impl ChameleonServices {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ChameleonServices.load(1): not translated, it's generic.
        // ChameleonServices.clientLoad(1): not translated, it's generic.
    }

    /// `FabricClient` (com/texelsaurus/minecraft/chameleon/client/FabricClient.java)
    pub mod fabric_client {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct FabricClient {
        }
        impl FabricClient {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // FabricClient.registerPacket(1): not translated, it's generic.
        // FabricClient.sendToServer(1): not translated, it's generic.
    }

    /// `ConfigSpec` (com/texelsaurus/minecraft/chameleon/config/ConfigSpec.java)
    pub mod config_spec {
        use super::*;
        // The class itself isn't translated: a function kept in a variable or field (only parameters can be). Its static parts are below.
        // ConfigSpec.init(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigSpec.setLoaded(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigSpec.isLoaded(0): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
        // ConfigSpec.onLoad(1): not translated, its class isn't translated (a function kept in a variable or field (only parameters can be)).
    }

    /// `ForgeApiConfig` (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java)
    pub mod forge_api_config {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service). Its static parts are below.
        // ForgeApiConfig.ForgeApiConfig(0): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // ForgeApiConfig.ForgeApiConfig(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // ForgeApiConfig.init(2): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // ForgeApiConfig.create(1): not translated, it's generic.
        // ForgeApiConfig.define(2): not translated, it's generic.
        // ForgeApiConfig.defineInRange(5): not translated, it's generic.
        // ForgeApiConfig.defineEnum(2): not translated, it's generic.
        // ForgeApiConfig.defineList(3): not translated, it's generic.
        // ForgeApiConfig.pushGroup(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // ForgeApiConfig.popGroup(0): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // ForgeApiConfig.comment(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // ForgeApiConfig.comment(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
    }

    /// `ForgeApiConfig.ForgeConfigEntry` (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java)
    pub mod forge_api_config_forge_config_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ForgeApiConfig.ForgeConfigEntry.ForgeConfigEntry(1): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntry.build(0): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntry.get(0): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntry.set(1): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntry.define(0): not translated, its class isn't translated (it's generic).
    }

    /// `ForgeApiConfig.ForgeConfigEntryEnum` (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java)
    pub mod forge_api_config_forge_config_entry_enum {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ForgeApiConfig.ForgeConfigEntryEnum.ForgeConfigEntryEnum(1): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntryEnum.define(0): not translated, its class isn't translated (it's generic).
    }

    /// `ForgeApiConfig.ForgeConfigEntryList` (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java)
    pub mod forge_api_config_forge_config_entry_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ForgeApiConfig.ForgeConfigEntryList.ForgeConfigEntryList(1): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntryList.validator(1): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntryList.define(0): not translated, its class isn't translated (it's generic).
    }

    /// `ForgeApiConfig.ForgeConfigEntryRange` (com/texelsaurus/minecraft/chameleon/config/ForgeApiConfig.java)
    pub mod forge_api_config_forge_config_entry_range {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ForgeApiConfig.ForgeConfigEntryRange.ForgeConfigEntryRange(2): not translated, its class isn't translated (it's generic).
        // ForgeApiConfig.ForgeConfigEntryRange.define(0): not translated, its class isn't translated (it's generic).
    }

    /// `StaticConfig` (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java)
    pub mod static_config {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service). Its static parts are below.
        // StaticConfig.StaticConfig(0): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // StaticConfig.StaticConfig(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // StaticConfig.create(1): not translated, it's generic.
        // StaticConfig.init(2): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // StaticConfig.define(2): not translated, it's generic.
        // StaticConfig.defineInRange(5): not translated, it's generic.
        // StaticConfig.defineEnum(2): not translated, it's generic.
        // StaticConfig.defineList(3): not translated, it's generic.
        // StaticConfig.pushGroup(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // StaticConfig.popGroup(0): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // StaticConfig.comment(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
        // StaticConfig.comment(1): not translated, its class isn't translated (it implements `ChameleonConfig` (com.texelsaurus.minecraft.chameleon.service)).
    }

    /// `StaticConfig.FabricConfigEntry` (com/texelsaurus/minecraft/chameleon/config/StaticConfig.java)
    pub mod static_config_fabric_config_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // StaticConfig.FabricConfigEntry.build(0): not translated, its class isn't translated (it's generic).
        // StaticConfig.FabricConfigEntry.set(1): not translated, its class isn't translated (it's generic).
        // StaticConfig.FabricConfigEntry.get(0): not translated, its class isn't translated (it's generic).
    }

    /// `ContainerContent` (com/texelsaurus/minecraft/chameleon/inventory/ContainerContent.java)
    pub mod container_content {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ContainerContent.serializer(0): not translated, it has no body.
    }

    /// `ContainerContentSerializer` (com/texelsaurus/minecraft/chameleon/inventory/ContainerContentSerializer.java)
    pub mod container_content_serializer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ContainerContentSerializer.from(1): not translated, it has no body.
        // ContainerContentSerializer.to(2): not translated, it has no body.
    }

    /// `ContentMenuProvider` (com/texelsaurus/minecraft/chameleon/inventory/ContentMenuProvider.java)
    pub mod content_menu_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ContentMenuProvider.createContent(1): not translated, it has no body.
        // ContentMenuProvider.openMenu(1): not translated, its class isn't translated (it's an interface).
    }

    /// `PositionContent` (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java)
    pub mod position_content {
        use super::*;
        // The class itself isn't translated: it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory). Its static parts are below.
        // SERIALIZER: not translated, it uses ContainerContentSerializer, which isn't translated (it's an interface).
        // PositionContent.PositionContent(1): not translated, its class isn't translated (it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory)).
        // PositionContent.serializer(0): not translated, its class isn't translated (it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory)).
        // PositionContent.getBlockEntity(2): not translated, it's generic.
        // PositionContent.getOrNull(3): not translated, it's generic.
        // PositionContent.pos(0): not translated, its class isn't translated (it implements `ContainerContent` (com.texelsaurus.minecraft.chameleon.inventory)).
    }

    /// `PositionContent.Serializer` (com/texelsaurus/minecraft/chameleon/inventory/content/PositionContent.java)
    pub mod position_content_serializer {
        use super::*;
        // The class itself isn't translated: it implements `ContainerContentSerializer` (com.texelsaurus.minecraft.chameleon.inventory). Its static parts are below.
        // PositionContent.Serializer.from(1): not translated, its class isn't translated (it implements `ContainerContentSerializer` (com.texelsaurus.minecraft.chameleon.inventory)).
        // PositionContent.Serializer.to(2): not translated, its class isn't translated (it implements `ContainerContentSerializer` (com.texelsaurus.minecraft.chameleon.inventory)).
    }

    /// `ChameleonPacket` (com/texelsaurus/minecraft/chameleon/network/ChameleonPacket.java)
    pub mod chameleon_packet {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonPacket.handleMessage(2): not translated, it has no body.
    }

    /// `ChameleonRegistry` (com/texelsaurus/minecraft/chameleon/registry/ChameleonRegistry.java)
    pub mod chameleon_registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonRegistry.register(2): not translated, it has no body.
        // ChameleonRegistry.getEntries(0): not translated, it has no body.
        // ChameleonRegistry.stream(0): not translated, its class isn't translated (it's an interface).
    }

    /// `FabricRegistry` (com/texelsaurus/minecraft/chameleon/registry/FabricRegistry.java)
    pub mod fabric_registry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FabricRegistry.FabricRegistry(2): not translated, its class isn't translated (it's generic).
        // FabricRegistry.register(2): not translated, it's generic.
        // FabricRegistry.getEntries(0): not translated, its class isn't translated (it's generic).
        // FabricRegistry.init(1): not translated, its class isn't translated (it's generic).
    }

    /// `FabricRegistryEntry` (com/texelsaurus/minecraft/chameleon/registry/FabricRegistryEntry.java)
    pub mod fabric_registry_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FabricRegistryEntry.FabricRegistryEntry(2): not translated, its class isn't translated (it's generic).
        // FabricRegistryEntry.getId(0): not translated, its class isn't translated (it's generic).
        // FabricRegistryEntry.get(0): not translated, its class isn't translated (it's generic).
    }

    /// `RegistryEntry` (com/texelsaurus/minecraft/chameleon/registry/RegistryEntry.java)
    pub mod registry_entry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RegistryEntry.getId(0): not translated, it has no body.
    }

    /// `RegistryEntryList` (com/texelsaurus/minecraft/chameleon/registry/RegistryEntryList.java)
    pub mod registry_entry_list {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RegistryEntryList.add(1): not translated, it's generic.
        // RegistryEntryList.getEntries(0): not translated, its class isn't translated (it's generic).
    }

    /// `ChameleonBlockModelPart` (com/texelsaurus/minecraft/chameleon/render/ChameleonBlockModelPart.java)
    pub mod chameleon_block_model_part {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonBlockModelPart.setRenderType(1): not translated, its class isn't translated (it's an interface).
        // ChameleonBlockModelPart.getRenderType(0): not translated, it has no body.
    }

    /// `FabricReplacementBlockPart` (com/texelsaurus/minecraft/chameleon/render/FabricReplacementBlockPart.java)
    pub mod fabric_replacement_block_part {
        use super::*;
        // The class itself isn't translated: it extends `ReplacementBlockPart`. Its static parts are below.
        // FabricReplacementBlockPart.FabricReplacementBlockPart(2): not translated, its class isn't translated (it extends `ReplacementBlockPart`).
        // FabricReplacementBlockPart.setRenderType(1): not translated, its class isn't translated (it extends `ReplacementBlockPart`).
        // FabricReplacementBlockPart.getRenderType(0): not translated, its class isn't translated (it extends `ReplacementBlockPart`).
    }

    /// `ReplacementBlockPart` (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPart.java)
    pub mod replacement_block_part {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonBlockModelPart`. Its static parts are below.
        // ReplacementBlockPart.ReplacementBlockPart(2): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.ReplacementBlockPart(2): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.getQuads(1): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.useAmbientOcclusion(0): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.particleMaterial(0): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.materialFlags(0): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.remapQuad(2): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.remapPackedUV(2): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.getUnInterpolatedU(2): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
        // ReplacementBlockPart.getUnInterpolatedV(2): not translated, its class isn't translated (it implements `ChameleonBlockModelPart`).
    }

    /// `ReplacementBlockPartDef` (com/texelsaurus/minecraft/chameleon/render/ReplacementBlockPartDef.java)
    pub mod replacement_block_part_def {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockStateModelPart`. Its static parts are below.
        // ReplacementBlockPartDef.ReplacementBlockPartDef(2): not translated, its class isn't translated (it uses Minecraft's `BlockStateModelPart`).
        // ReplacementBlockPartDef.getPart(0): not translated, its class isn't translated (it uses Minecraft's `BlockStateModelPart`).
        // ReplacementBlockPartDef.getSprite(0): not translated, its class isn't translated (it uses Minecraft's `BlockStateModelPart`).
    }

    /// `ChameleonCapabilities` (com/texelsaurus/minecraft/chameleon/service/ChameleonCapabilities.java)
    pub mod chameleon_capabilities {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonCapabilities.create(3): not translated, it has no body.
    }

    /// `ChameleonConfig` (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java)
    pub mod chameleon_config {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonConfig.create(1): not translated, it has no body.
        // ChameleonConfig.init(2): not translated, it has no body.
        // ChameleonConfig.define(2): not translated, it has no body.
        // ChameleonConfig.defineInRange(5): not translated, it has no body.
        // ChameleonConfig.defineInRange(4): not translated, its class isn't translated (it's an interface).
        // ChameleonConfig.defineEnum(2): not translated, it has no body.
        // ChameleonConfig.defineList(3): not translated, it has no body.
        // ChameleonConfig.comment(1): not translated, it has no body.
        // ChameleonConfig.comment(1): not translated, it has no body.
        // ChameleonConfig.pushGroup(1): not translated, it has no body.
        // ChameleonConfig.popGroup(0): not translated, it has no body.
    }

    /// `ChameleonConfig.ConfigEntry` (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java)
    pub mod chameleon_config_config_entry {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ChameleonConfig.ConfigEntry.name(1): not translated, its class isn't translated (it's generic).
        // ChameleonConfig.ConfigEntry.defaultValue(1): not translated, its class isn't translated (it's generic).
        // ChameleonConfig.ConfigEntry.comment(1): not translated, its class isn't translated (it's generic).
        // ChameleonConfig.ConfigEntry.range(2): not translated, its class isn't translated (it's generic).
        // ChameleonConfig.ConfigEntry.build(0): not translated, it has no body.
        // ChameleonConfig.ConfigEntry.accept(1): not translated, its class isn't translated (it's generic).
        // ChameleonConfig.ConfigEntry.set(1): not translated, it has no body.
    }

    /// `ChameleonConfig.Type` (com/texelsaurus/minecraft/chameleon/service/ChameleonConfig.java)
    pub mod chameleon_config_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Type {
            Common,
            Client,
            Server,
        }
        impl Type {
            pub const VALUES: [Type; 3] = [Type::Common, Type::Client, Type::Server, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Type::Common => "COMMON",
                    Type::Client => "CLIENT",
                    Type::Server => "SERVER",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "COMMON" => Type::Common,
                    "CLIENT" => Type::Client,
                    "SERVER" => Type::Server,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ChameleonContainer` (com/texelsaurus/minecraft/chameleon/service/ChameleonContainer.java)
    pub mod chameleon_container {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonContainer.getContainerSupplier(2): not translated, it has no body.
        // ChameleonContainer.openMenu(2): not translated, it has no body.
    }

    /// `ChameleonContainer.ChameleonContainerFactory` (com/texelsaurus/minecraft/chameleon/service/ChameleonContainer.java)
    pub mod chameleon_container_chameleon_container_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonContainer.ChameleonContainerFactory.create(3): not translated, it has no body.
        // ChameleonContainer.ChameleonContainerFactory.create(2): not translated, its class isn't translated (it's an interface).
    }

    /// `ChameleonNetworking` (com/texelsaurus/minecraft/chameleon/service/ChameleonNetworking.java)
    pub mod chameleon_networking {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonNetworking.registerPacket(3): not translated, it's generic.
        // ChameleonNetworking.registerPacketInternal(3): not translated, it has no body.
        // ChameleonNetworking.sendToPlayer(2): not translated, it has no body.
        // ChameleonNetworking.sendToPlayersNear(6): not translated, it has no body.
        // ChameleonNetworking.sendToServer(1): not translated, it has no body.
    }

    /// `ChameleonPlatform` (com/texelsaurus/minecraft/chameleon/service/ChameleonPlatform.java)
    pub mod chameleon_platform {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonPlatform.isPhysicalClient(0): not translated, it has no body.
    }

    /// `ChameleonRegistries` (com/texelsaurus/minecraft/chameleon/service/ChameleonRegistries.java)
    pub mod chameleon_registries {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonRegistries.create(2): not translated, it has no body.
    }

    /// `ChameleonRender` (com/texelsaurus/minecraft/chameleon/service/ChameleonRender.java)
    pub mod chameleon_render {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ChameleonRender.createReplacementPart(2): not translated, it has no body.
    }

    /// `FabricCapabilities` (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java)
    pub mod fabric_capabilities {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonCapabilities`. Its static parts are below.
        // capabilties: not translated, it uses IFabricCapability, which isn't translated (it's an interface).
        // FabricCapabilities.create(3): not translated, it's generic.
        // FabricCapabilities.reigsterCapability(1): not translated, it's generic.
    }

    /// `FabricCapabilities.DeferredCapability` (com/texelsaurus/minecraft/chameleon/service/FabricCapabilities.java)
    pub mod fabric_capabilities_deferred_capability {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FabricCapabilities.DeferredCapability.DeferredCapability(1): not translated, its class isn't translated (it's generic).
        // FabricCapabilities.DeferredCapability.id(0): not translated, its class isn't translated (it's generic).
        // FabricCapabilities.DeferredCapability.getCapability(2): not translated, its class isn't translated (it's generic).
        // FabricCapabilities.DeferredCapability.getCapability(1): not translated, it's generic.
        // FabricCapabilities.DeferredCapability.register(2): not translated, it's generic.
    }

    /// `FabricConfig` (com/texelsaurus/minecraft/chameleon/service/FabricConfig.java)
    pub mod fabric_config {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonConfig`. Its static parts are below.
        // FabricConfig.FabricConfig(0): not translated, its class isn't translated (it implements `ChameleonConfig`).
        // FabricConfig.FabricConfig(1): not translated, its class isn't translated (it implements `ChameleonConfig`).
        // FabricConfig.init(2): not translated, its class isn't translated (it implements `ChameleonConfig`).
        // FabricConfig.create(1): not translated, it's generic.
        // FabricConfig.define(2): not translated, it's generic.
        // FabricConfig.defineInRange(5): not translated, it's generic.
        // FabricConfig.defineEnum(2): not translated, it's generic.
        // FabricConfig.defineList(3): not translated, it's generic.
        // FabricConfig.pushGroup(1): not translated, its class isn't translated (it implements `ChameleonConfig`).
        // FabricConfig.popGroup(0): not translated, its class isn't translated (it implements `ChameleonConfig`).
        // FabricConfig.comment(1): not translated, its class isn't translated (it implements `ChameleonConfig`).
        // FabricConfig.comment(1): not translated, its class isn't translated (it implements `ChameleonConfig`).
    }

    /// `FabricContainer` (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java)
    pub mod fabric_container {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonContainer`. Its static parts are below.
        // FabricContainer.getContainerSupplier(2): not translated, it's generic.
        // FabricContainer.openMenu(2): not translated, it's generic.
    }

    /// `FabricContainer.PlatformContainerFactory` (com/texelsaurus/minecraft/chameleon/service/FabricContainer.java)
    pub mod fabric_container_platform_container_factory {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // FabricContainer.PlatformContainerFactory.PlatformContainerFactory(1): not translated, its class isn't translated (it's generic).
        // FabricContainer.PlatformContainerFactory.getDisplayName(0): not translated, its class isn't translated (it's generic).
        // FabricContainer.PlatformContainerFactory.createMenu(3): not translated, its class isn't translated (it's generic).
        // FabricContainer.PlatformContainerFactory.shouldCloseCurrentScreen(0): not translated, its class isn't translated (it's generic).
        // FabricContainer.PlatformContainerFactory.getScreenOpeningData(1): not translated, its class isn't translated (it's generic).
        // FabricContainer.PlatformContainerFactory.provider(0): not translated, its class isn't translated (it's generic).
    }

    /// `FabricNetworking` (com/texelsaurus/minecraft/chameleon/service/FabricNetworking.java)
    pub mod fabric_networking {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonNetworking`. Its static parts are below.
        // FabricNetworking.registerPacketInternal(3): not translated, it's generic.
        // FabricNetworking.sendToPlayer(2): not translated, its class isn't translated (it implements `ChameleonNetworking`).
        // FabricNetworking.sendToPlayersNear(6): not translated, its class isn't translated (it implements `ChameleonNetworking`).
        // FabricNetworking.sendToServer(1): not translated, its class isn't translated (it implements `ChameleonNetworking`).
    }

    /// `FabricPlatform` (com/texelsaurus/minecraft/chameleon/service/FabricPlatform.java)
    pub mod fabric_platform {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonPlatform`. Its static parts are below.
        // FabricPlatform.isPhysicalClient(0): not translated, its class isn't translated (it implements `ChameleonPlatform`).
    }

    /// `FabricRegistries` (com/texelsaurus/minecraft/chameleon/service/FabricRegistries.java)
    pub mod fabric_registries {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonRegistries`. Its static parts are below.
        // FabricRegistries.create(2): not translated, it's generic.
    }

    /// `FabricRender` (com/texelsaurus/minecraft/chameleon/service/FabricRender.java)
    pub mod fabric_render {
        use super::*;
        // The class itself isn't translated: it implements `ChameleonRender`. Its static parts are below.
        // FabricRender.createReplacementPart(2): not translated, its class isn't translated (it implements `ChameleonRender`).
    }

    /// `WorldUtils` (com/texelsaurus/minecraft/chameleon/util/WorldUtils.java)
    pub mod world_utils_2 {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // WorldUtils.WorldUtils(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // WorldUtils.rayTraceEyes(3): not translated, it uses Minecraft's `BlockHitResult`.
        // WorldUtils.getBlockEntity(3): not translated, it's generic.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "storagedrawers");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
