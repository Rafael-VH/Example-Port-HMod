// Lithostitched, ported to Horizon by horizon-port from lithostitched-2.0.6-fabric-26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_lithostitched). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "lithostitched";
pub const NAME: &str = "Lithostitched";
pub const VERSION: &str = "2.0.6";
pub const DESCRIPTION: &str = "An expansive library mod with new configurability and compatibility enhancements for worldgen";

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

    /// `AddBiomeInjectorsEvent` (dev/worldgen/lithostitched/api/event/AddBiomeInjectorsEvent.java)
    pub mod add_biome_injectors_event {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AddBiomeInjectorsEvent.addInjectors(2): not translated, it has no body.
    }

    /// `AddRegionsEvent` (dev/worldgen/lithostitched/api/event/AddRegionsEvent.java)
    pub mod add_regions_event {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AddRegionsEvent.addRegions(2): not translated, it has no body.
    }

    /// `AddRegionsEvent.RegionConsumer` (dev/worldgen/lithostitched/api/event/AddRegionsEvent.java)
    pub mod add_regions_event_region_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AddRegionsEvent.RegionConsumer.accept(4): not translated, it has no body.
    }

    /// `AddWorldgenModifiersEvent` (dev/worldgen/lithostitched/api/event/AddWorldgenModifiersEvent.java)
    pub mod add_worldgen_modifiers_event {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AddWorldgenModifiersEvent.addModifiers(2): not translated, it has no body.
    }

    /// `LoadPredicate` (dev/worldgen/lithostitched/api/predicate/LoadPredicate.java)
    pub mod load_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LoadPredicate.test(0): not translated, it has no body.
        // LoadPredicate.codec(0): not translated, it has no body.
        // LoadPredicate.allOf(1): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.anyOf(1): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.isFabric(0): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.isNeoforge(0): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.modLoaded(1): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.not(1): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.packFormat(1): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.alwaysFalse(0): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // LoadPredicate.alwaysTrue(0): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
    }

    /// `LithostitchedBuiltInRegistries` (dev/worldgen/lithostitched/api/registry/LithostitchedBuiltInRegistries.java)
    pub mod lithostitched_built_in_registries {
        use super::*;
        // MODIFIER_TYPE: not translated, it uses Minecraft's `Registry`.
        // PLACEMENT_CONDITION_TYPE: not translated, it uses Minecraft's `Registry`.
        // PROCESSOR_CONDITION_TYPE: not translated, it uses Minecraft's `Registry`.
        // BANDLANDS_BAND_TYPE: not translated, it uses Minecraft's `Registry`.
        // BIOME_INJECTOR_TYPE: not translated, it uses Minecraft's `Registry`.
        // FAST_NOISE_CONFIG_TYPE: not translated, it uses Minecraft's `Registry`.
        // LOAD_PREDICATE_TYPE: not translated, it uses Minecraft's `Registry`.
        // HOLDER_TARGET_TYPE: not translated, it uses Minecraft's `Registry`.
        #[derive(Clone, Debug)]
        pub struct LithostitchedBuiltInRegistries {
        }
        impl LithostitchedBuiltInRegistries {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LithostitchedBuiltInRegistries.create(1): not translated, it's generic.
        // LithostitchedBuiltInRegistries.init(0): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `LithostitchedRegistries` (dev/worldgen/lithostitched/api/registry/LithostitchedRegistries.java)
    pub mod lithostitched_registries {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedRegistries.create(1): not translated, it's generic.
    }

    /// `LithostitchedBiomeSourceTags` (dev/worldgen/lithostitched/api/tag/LithostitchedBiomeSourceTags.java)
    pub mod lithostitched_biome_source_tags {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedBiomeSourceTags.create(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `LithostitchedBiomeTags` (dev/worldgen/lithostitched/api/tag/LithostitchedBiomeTags.java)
    pub mod lithostitched_biome_tags {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedBiomeTags.create(1): not translated, it uses Minecraft's `TagKey`.
        // LithostitchedBiomeTags.createVillagerTypeTag(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `LithostitchedBlockTags` (dev/worldgen/lithostitched/api/tag/LithostitchedBlockTags.java)
    pub mod lithostitched_block_tags {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedBlockTags.create(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `LithostitchedProcessorListTags` (dev/worldgen/lithostitched/api/tag/LithostitchedProcessorListTags.java)
    pub mod lithostitched_processor_list_tags {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedProcessorListTags.create(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `LithostitchedTemplatePoolTags` (dev/worldgen/lithostitched/api/tag/LithostitchedTemplatePoolTags.java)
    pub mod lithostitched_template_pool_tags {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedTemplatePoolTags.create(1): not translated, it uses Minecraft's `TagKey`.
    }

    /// `InjectionType` (dev/worldgen/lithostitched/api/util/InjectionType.java)
    pub mod injection_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // InjectionType.InjectionType(1): not translated, its class isn't translated (its constants carry values).
        // InjectionType.apply(3): not translated, it's generic.
        // InjectionType.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `WeightedHolderSet` (dev/worldgen/lithostitched/api/util/WeightedHolderSet.java)
    pub mod weighted_holder_set {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // WeightedHolderSet.create(1): not translated, it's generic.
        // WeightedHolderSet.create(1): not translated, it's generic.
        // WeightedHolderSet.WeightedHolderSet(1): not translated, its class isn't translated (it's generic).
        // WeightedHolderSet.getRandom(1): not translated, its class isn't translated (it's generic).
        // WeightedHolderSet.codec(2): not translated, it's generic.
    }

    /// `LithostitchedEnvironmentAttributes` (dev/worldgen/lithostitched/api/worldgen/attribute/LithostitchedEnvironmentAttributes.java)
    pub mod lithostitched_environment_attributes {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `Band` (dev/worldgen/lithostitched/api/worldgen/bandlands/Band.java)
    pub mod band {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Band.fill(2): not translated, it has no body.
        // Band.codec(0): not translated, it has no body.
    }

    /// `BandlandsBuilder` (dev/worldgen/lithostitched/api/worldgen/bandlands/BandlandsBuilder.java)
    pub mod bandlands_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BandlandsBuilder.create(1): not translated, it uses BandlandsBuilder, which isn't translated (it's an interface).
        // BandlandsBuilder.baseBand(1): not translated, its class isn't translated (it's an interface).
        // BandlandsBuilder.baseBand(3): not translated, it has no body.
        // BandlandsBuilder.repeatBand(3): not translated, it has no body.
        // BandlandsBuilder.wrappedBand(5): not translated, it has no body.
        // BandlandsBuilder.build(0): not translated, it has no body.
    }

    /// `BiomeInjector` (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java)
    pub mod biome_injector {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomeInjector.predicate(0): not translated, it has no body.
        // BiomeInjector.dimension(0): not translated, it has no body.
        // BiomeInjector.priority(0): not translated, it has no body.
        // BiomeInjector.possibleBiomes(0): not translated, it has no body.
        // BiomeInjector.codec(0): not translated, it has no body.
        // BiomeInjector.builder(1): not translated, it uses Minecraft's `ResourceKey`.
        // BiomeInjector.builder(2): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `BiomeInjector.ClimateParameter` (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java)
    pub mod biome_injector_climate_parameter {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // BiomeInjector.ClimateParameter.ClimateParameter(2): not translated, its class isn't translated (its constants carry values).
        // BiomeInjector.ClimateParameter.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BiomeInjector.InjectorBuilder` (dev/worldgen/lithostitched/api/worldgen/biomeinjector/BiomeInjector.java)
    pub mod biome_injector_injector_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // BiomeInjector.InjectorBuilder.InjectorBuilder(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.priority(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.addPoints(1): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.dispatchAlternateLayout(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.forcePlacement(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.replaceFully(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.replaceFully(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.replacePartially(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // BiomeInjector.InjectorBuilder.replacePartially(3): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `ParameterBuilder` (dev/worldgen/lithostitched/api/worldgen/biomeinjector/ParameterBuilder.java)
    pub mod parameter_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ParameterBuilder.create(0): not translated, it uses ParameterBuilder, which isn't translated (it's an interface).
        // ParameterBuilder.densityFunctionExactly(2): not translated, it has no body.
        // ParameterBuilder.densityFunctionMin(2): not translated, it has no body.
        // ParameterBuilder.densityFunctionMax(2): not translated, it has no body.
        // ParameterBuilder.densityFunctionRange(3): not translated, it has no body.
        // ParameterBuilder.climateExactly(2): not translated, it has no body.
        // ParameterBuilder.climateMin(2): not translated, it has no body.
        // ParameterBuilder.climateMax(2): not translated, it has no body.
        // ParameterBuilder.climateRange(3): not translated, it has no body.
        // ParameterBuilder.region(1): not translated, it has no body.
        // ParameterBuilder.build(0): not translated, it has no body.
    }

    /// `LithostitchedBlockEntityModifiers` (dev/worldgen/lithostitched/api/worldgen/blockentitymodifier/LithostitchedBlockEntityModifiers.java)
    pub mod lithostitched_block_entity_modifiers {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedBlockEntityModifiers.applyAll(1): not translated, it uses Minecraft's `RuleBlockEntityModifier`.
        // LithostitchedBlockEntityModifiers.applyRandom(1): not translated, it uses Minecraft's `RuleBlockEntityModifier`.
    }

    /// `LithostitchedBlockPredicates` (dev/worldgen/lithostitched/api/worldgen/blockpredicate/LithostitchedBlockPredicates.java)
    pub mod lithostitched_block_predicates {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedBlockPredicates.blockState(1): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.blockState(2): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.grid(4): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.grid(5): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.inStructure(1): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.inStructure(2): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.inStructure(2): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.inStructure(3): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.multipleOf(2): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.offset(2): not translated, it uses Minecraft's `BlockPredicate`.
        // LithostitchedBlockPredicates.randomChance(1): not translated, it uses Minecraft's `BlockPredicate`.
    }

    /// `ReturnType` (dev/worldgen/lithostitched/api/worldgen/densityfunction/cellular/ReturnType.java)
    pub mod return_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ReturnType.ReturnType(1): not translated, its class isn't translated (its constants carry values).
        // ReturnType.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `FastNoiseConfig` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FastNoiseConfig.java)
    pub mod fast_noise_config {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Noise`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // FastNoiseConfig.getCodec(0): not translated, it has no body.
        // FastNoiseConfig.FastNoiseConfig(2): not translated, its class isn't translated (it implements Minecraft's `Noise`).
        // FastNoiseConfig.frequency(0): not translated, its class isn't translated (it implements Minecraft's `Noise`).
        // FastNoiseConfig.salt(0): not translated, its class isn't translated (it implements Minecraft's `Noise`).
        // FastNoiseConfig.bind(1): not translated, its class isn't translated (it implements Minecraft's `Noise`).
        // FastNoiseConfig.range(0): not translated, its class isn't translated (it implements Minecraft's `Noise`).
        // FastNoiseConfig.get(2): not translated, its class isn't translated (it implements Minecraft's `Noise`).
        // FastNoiseConfig.get(3): not translated, its class isn't translated (it implements Minecraft's `Noise`).
    }

    /// `FNL` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl {
        use super::*;
        // The class itself isn't translated: it uses FractalType, which isn't translated (its constants carry values). Its static parts are below.
        pub static GRADIENTS2_D: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.13052619f32, 0.9914449f32, 0.38268343f32, 0.9238795f32, 0.6087614f32, 0.7933533f32, 0.7933533f32, 0.6087614f32, 0.9238795f32, 0.38268343f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, -0.13052619f32, 0.9238795f32, -0.38268343f32, 0.7933533f32, -0.6087614f32, 0.6087614f32, -0.7933533f32, 0.38268343f32, -0.9238795f32, 0.13052619f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, -0.38268343f32, -0.9238795f32, -0.6087614f32, -0.7933533f32, -0.7933533f32, -0.6087614f32, -0.9238795f32, -0.38268343f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, 0.13052619f32, -0.9238795f32, 0.38268343f32, -0.7933533f32, 0.6087614f32, -0.6087614f32, 0.7933533f32, -0.38268343f32, 0.9238795f32, -0.13052619f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, 0.38268343f32, 0.9238795f32, 0.6087614f32, 0.7933533f32, 0.7933533f32, 0.6087614f32, 0.9238795f32, 0.38268343f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, -0.13052619f32, 0.9238795f32, -0.38268343f32, 0.7933533f32, -0.6087614f32, 0.6087614f32, -0.7933533f32, 0.38268343f32, -0.9238795f32, 0.13052619f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, -0.38268343f32, -0.9238795f32, -0.6087614f32, -0.7933533f32, -0.7933533f32, -0.6087614f32, -0.9238795f32, -0.38268343f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, 0.13052619f32, -0.9238795f32, 0.38268343f32, -0.7933533f32, 0.6087614f32, -0.6087614f32, 0.7933533f32, -0.38268343f32, 0.9238795f32, -0.13052619f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, 0.38268343f32, 0.9238795f32, 0.6087614f32, 0.7933533f32, 0.7933533f32, 0.6087614f32, 0.9238795f32, 0.38268343f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, -0.13052619f32, 0.9238795f32, -0.38268343f32, 0.7933533f32, -0.6087614f32, 0.6087614f32, -0.7933533f32, 0.38268343f32, -0.9238795f32, 0.13052619f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, -0.38268343f32, -0.9238795f32, -0.6087614f32, -0.7933533f32, -0.7933533f32, -0.6087614f32, -0.9238795f32, -0.38268343f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, 0.13052619f32, -0.9238795f32, 0.38268343f32, -0.7933533f32, 0.6087614f32, -0.6087614f32, 0.7933533f32, -0.38268343f32, 0.9238795f32, -0.13052619f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, 0.38268343f32, 0.9238795f32, 0.6087614f32, 0.7933533f32, 0.7933533f32, 0.6087614f32, 0.9238795f32, 0.38268343f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, -0.13052619f32, 0.9238795f32, -0.38268343f32, 0.7933533f32, -0.6087614f32, 0.6087614f32, -0.7933533f32, 0.38268343f32, -0.9238795f32, 0.13052619f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, -0.38268343f32, -0.9238795f32, -0.6087614f32, -0.7933533f32, -0.7933533f32, -0.6087614f32, -0.9238795f32, -0.38268343f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, 0.13052619f32, -0.9238795f32, 0.38268343f32, -0.7933533f32, 0.6087614f32, -0.6087614f32, 0.7933533f32, -0.38268343f32, 0.9238795f32, -0.13052619f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, 0.38268343f32, 0.9238795f32, 0.6087614f32, 0.7933533f32, 0.7933533f32, 0.6087614f32, 0.9238795f32, 0.38268343f32, 0.9914449f32, 0.13052619f32, 0.9914449f32, -0.13052619f32, 0.9238795f32, -0.38268343f32, 0.7933533f32, -0.6087614f32, 0.6087614f32, -0.7933533f32, 0.38268343f32, -0.9238795f32, 0.13052619f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, -0.38268343f32, -0.9238795f32, -0.6087614f32, -0.7933533f32, -0.7933533f32, -0.6087614f32, -0.9238795f32, -0.38268343f32, -0.9914449f32, -0.13052619f32, -0.9914449f32, 0.13052619f32, -0.9238795f32, 0.38268343f32, -0.7933533f32, 0.6087614f32, -0.6087614f32, 0.7933533f32, -0.38268343f32, 0.9238795f32, -0.13052619f32, 0.9914449f32, 0.38268343f32, 0.9238795f32, 0.9238795f32, 0.38268343f32, 0.9238795f32, -0.38268343f32, 0.38268343f32, -0.9238795f32, -0.38268343f32, -0.9238795f32, -0.9238795f32, -0.38268343f32, -0.9238795f32, 0.38268343f32, -0.38268343f32, 0.9238795f32]);
        pub static RAND_VECS2_D: LazyLock<Vec<f32>> = LazyLock::new(|| vec![-0.2700222f32, -0.9628541f32, 0.38630927f32, -0.9223693f32, 0.04444859f32, -0.9990117f32, -0.59925234f32, -0.80056024f32, -0.781928f32, 0.62336874f32, 0.9464672f32, 0.32279992f32, -0.6514147f32, -0.7587219f32, 0.93784726f32, 0.34704837f32, -0.8497876f32, -0.52712524f32, -0.87904257f32, 0.47674325f32, -0.8923003f32, -0.45144236f32, -0.37984443f32, -0.9250504f32, -0.9951651f32, 0.09821638f32, 0.7724398f32, -0.635088f32, 0.75732833f32, -0.6530343f32, -0.9928005f32, -0.119780056f32, -0.05326657f32, 0.99858034f32, 0.97542536f32, -0.22033007f32, -0.76650184f32, 0.64224213f32, 0.9916367f32, 0.12906061f32, -0.99469686f32, 0.10285038f32, -0.53792053f32, -0.8429955f32, 0.50228155f32, -0.86470413f32, 0.45598215f32, -0.8899889f32, -0.8659131f32, -0.50019443f32, 0.08794584f32, -0.9961253f32, -0.5051685f32, 0.8630207f32, 0.7753185f32, -0.6315704f32, -0.69219446f32, 0.72171104f32, -0.51916593f32, -0.85467345f32, 0.8978623f32, -0.4402764f32, -0.17067741f32, 0.98532695f32, -0.935343f32, -0.35374206f32, -0.99924046f32, 0.038967468f32, -0.2882064f32, -0.9575683f32, -0.96638113f32, 0.2571138f32, -0.87597144f32, -0.48236302f32, -0.8303123f32, -0.55729836f32, 0.051101338f32, -0.99869347f32, -0.85583735f32, -0.51724505f32, 0.098870255f32, 0.9951003f32, 0.9189016f32, 0.39448678f32, -0.24393758f32, -0.96979094f32, -0.81214094f32, -0.5834613f32, -0.99104315f32, 0.13354214f32, 0.8492424f32, -0.52800316f32, -0.9717839f32, -0.23587295f32, 0.9949457f32, 0.10041421f32, 0.6241065f32, -0.7813392f32, 0.6629103f32, 0.74869883f32, -0.7197418f32, 0.6942418f32, -0.8143371f32, -0.58039224f32, 0.10452105f32, -0.9945227f32, -0.10659261f32, -0.99430275f32, 0.44579968f32, -0.8951328f32, 0.105547406f32, 0.99441427f32, -0.9927903f32, 0.11986445f32, -0.83343667f32, 0.55261505f32, 0.9115562f32, -0.4111756f32, 0.8285545f32, -0.55990845f32, 0.7217098f32, -0.6921958f32, 0.49404928f32, -0.8694339f32, -0.36523214f32, -0.9309165f32, -0.9696607f32, 0.24445485f32, 0.089255095f32, -0.9960088f32, 0.5354071f32, -0.8445941f32, -0.10535762f32, 0.9944344f32, -0.98902845f32, 0.1477251f32, 0.004856105f32, 0.9999882f32, 0.98855984f32, 0.15082914f32, 0.92861295f32, -0.37104982f32, -0.5832394f32, -0.8123003f32, 0.30152076f32, 0.9534596f32, -0.95751107f32, 0.28839657f32, 0.9715802f32, -0.23671055f32, 0.2299818f32, 0.97319496f32, 0.9557638f32, -0.2941352f32, 0.7409561f32, 0.67155343f32, -0.9971514f32, -0.07542631f32, 0.69057107f32, -0.7232645f32, -0.2907137f32, -0.9568101f32, 0.5912778f32, -0.80646795f32, -0.94545925f32, -0.3257405f32, 0.66644555f32, 0.7455537f32, 0.6236135f32, 0.78173286f32, 0.9126994f32, -0.40863165f32, -0.8191762f32, 0.57354194f32, -0.8812746f32, -0.4726046f32, 0.99533135f32, 0.09651673f32, 0.98556507f32, -0.16929697f32, -0.8495981f32, 0.52743065f32, 0.6174854f32, -0.78658235f32, 0.85081565f32, 0.5254643f32, 0.99850327f32, -0.0546925f32, 0.19713716f32, -0.98037595f32, 0.66078556f32, -0.7505747f32, -0.030974941f32, 0.9995202f32, -0.6731661f32, 0.73949134f32, -0.71950185f32, -0.69449055f32, 0.97275114f32, 0.2318516f32, 0.9997059f32, -0.02425069f32, 0.44217876f32, -0.89692694f32, 0.9981351f32, -0.061043672f32, -0.9173661f32, -0.39804456f32, -0.81500566f32, -0.579453f32, -0.87893313f32, 0.476945f32, 0.015860584f32, 0.99987423f32, -0.8095465f32, 0.5870558f32, -0.9165899f32, -0.39982867f32, -0.8023543f32, 0.5968481f32, -0.5176738f32, 0.85557806f32, -0.8154407f32, -0.57884055f32, 0.40220103f32, -0.91555136f32, -0.9052557f32, -0.4248672f32, 0.7317446f32, 0.681579f32, -0.56476325f32, -0.825253f32, -0.8403276f32, -0.54207885f32, -0.93142813f32, 0.36392525f32, 0.52381986f32, 0.85182905f32, 0.7432804f32, -0.66898f32, -0.9853716f32, -0.17041974f32, 0.46014687f32, 0.88784283f32, 0.8258554f32, 0.56388193f32, 0.6182366f32, 0.785992f32, 0.83315027f32, -0.55304664f32, 0.15003075f32, 0.9886813f32, -0.6623304f32, -0.7492119f32, -0.66859865f32, 0.74362344f32, 0.7025606f32, 0.7116239f32, -0.54193896f32, -0.84041786f32, -0.33886164f32, 0.9408362f32, 0.833153f32, 0.55304253f32, -0.29897207f32, -0.95426184f32, 0.2638523f32, 0.9645631f32, 0.12410874f32, -0.9922686f32, -0.7282649f32, -0.6852957f32, 0.69625f32, 0.71779937f32, -0.91835356f32, 0.395761f32, -0.6326102f32, -0.7744703f32, -0.9331892f32, -0.35938552f32, -0.11537793f32, -0.99332166f32, 0.9514975f32, -0.30765656f32, -0.08987977f32, -0.9959526f32, 0.6678497f32, 0.7442962f32, 0.79524004f32, -0.6062947f32, -0.6462007f32, -0.7631675f32, -0.27335986f32, 0.96191186f32, 0.966959f32, -0.25493184f32, -0.9792895f32, 0.20246519f32, -0.5369503f32, -0.84361386f32, -0.27003646f32, -0.9628501f32, -0.6400277f32, 0.76835185f32, -0.78545374f32, -0.6189204f32, 0.060059056f32, -0.9981948f32, -0.024557704f32, 0.9996984f32, -0.65983623f32, 0.7514095f32, -0.62538946f32, -0.7803128f32, -0.6210409f32, -0.7837782f32, 0.8348889f32, 0.55041856f32, -0.15922752f32, 0.9872419f32, 0.83676225f32, 0.54756635f32, -0.8675754f32, -0.4973057f32, -0.20226626f32, -0.97933054f32, 0.939919f32, 0.34139755f32, 0.98774046f32, -0.1561049f32, -0.90344554f32, 0.42870283f32, 0.12698042f32, -0.9919052f32, -0.3819601f32, 0.92417884f32, 0.9754626f32, 0.22016525f32, -0.32040158f32, -0.94728184f32, -0.9874761f32, 0.15776874f32, 0.025353484f32, -0.99967855f32, 0.4835131f32, -0.8753371f32, -0.28508f32, -0.9585037f32, -0.06805516f32, -0.99768156f32, -0.7885244f32, -0.61500347f32, 0.3185392f32, -0.9479097f32, 0.8880043f32, 0.45983514f32, 0.64769214f32, -0.76190215f32, 0.98202413f32, 0.18875542f32, 0.93572754f32, -0.35272372f32, -0.88948953f32, 0.45695552f32, 0.7922791f32, 0.6101588f32, 0.74838185f32, 0.66326815f32, -0.728893f32, -0.68462765f32, 0.8729033f32, -0.48789328f32, 0.8288346f32, 0.5594937f32, 0.08074567f32, 0.99673474f32, 0.97991484f32, -0.1994165f32, -0.5807307f32, -0.81409574f32, -0.47000498f32, -0.8826638f32, 0.2409493f32, 0.9705377f32, 0.9437817f32, -0.33056942f32, -0.89279985f32, -0.45045355f32, -0.80696225f32, 0.59060305f32, 0.062589735f32, 0.99803936f32, -0.93125975f32, 0.36435598f32, 0.57774496f32, 0.81621736f32, -0.3360096f32, -0.9418586f32, 0.69793206f32, -0.71616393f32, -0.0020081573f32, -0.999998f32, -0.18272944f32, -0.98316324f32, -0.6523912f32, 0.7578824f32, -0.43026268f32, -0.9027037f32, -0.9985126f32, -0.054520912f32, -0.010281022f32, -0.99994713f32, -0.49460712f32, 0.86911666f32, -0.299935f32, 0.95395964f32, 0.8165472f32, 0.5772787f32, 0.26974604f32, 0.9629315f32, -0.7306287f32, -0.68277496f32, -0.7590952f32, -0.65097964f32, -0.9070538f32, 0.4210146f32, -0.5104861f32, -0.859886f32, 0.86133504f32, 0.5080373f32, 0.50078815f32, -0.8655699f32, -0.6541582f32, 0.7563578f32, -0.83827555f32, -0.54524684f32, 0.6940071f32, 0.7199682f32, 0.06950936f32, 0.9975813f32, 0.17029423f32, -0.9853933f32, 0.26959732f32, 0.9629731f32, 0.55196124f32, -0.83386976f32, 0.2256575f32, -0.9742067f32, 0.42152628f32, -0.9068162f32, 0.48818734f32, -0.87273884f32, -0.3683855f32, -0.92967314f32, -0.98253906f32, 0.18605645f32, 0.81256473f32, 0.582871f32, 0.3196461f32, -0.947537f32, 0.9570914f32, 0.28978625f32, -0.6876655f32, -0.7260276f32, -0.9988771f32, -0.04737673f32, -0.1250179f32, 0.9921545f32, -0.82801336f32, 0.56070834f32, 0.93248636f32, -0.36120513f32, 0.63946533f32, 0.7688199f32, -0.016238471f32, -0.99986815f32, -0.99550146f32, -0.094746135f32, -0.8145332f32, 0.580117f32, 0.4037328f32, -0.91487694f32, 0.9944263f32, 0.10543368f32, -0.16247116f32, 0.9867133f32, -0.9949488f32, -0.10038388f32, -0.69953024f32, 0.714603f32, 0.5263415f32, -0.85027325f32, -0.5395222f32, 0.8419714f32, 0.65793705f32, 0.7530729f32, 0.014267588f32, -0.9998982f32, -0.6734384f32, 0.7392433f32, 0.6394121f32, -0.7688642f32, 0.9211571f32, 0.38919085f32, -0.14663722f32, -0.98919034f32, -0.7823181f32, 0.6228791f32, -0.5039611f32, -0.8637264f32, -0.774312f32, -0.632804f32]);
        pub static GRADIENTS3_D: LazyLock<Vec<f32>> = LazyLock::new(|| vec![0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 0.0f32, 1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, -1.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 1.0f32, -1.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32, 0.0f32, 1.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, -1.0f32, 1.0f32, 0.0f32, 0.0f32, 0.0f32, -1.0f32, -1.0f32, 0.0f32]);
        pub static RAND_VECS3_D: LazyLock<Vec<f32>> = LazyLock::new(|| vec![-0.7292737f32, -0.66184396f32, 0.17355819f32, 0.0f32, 0.7902921f32, -0.5480887f32, -0.2739291f32, 0.0f32, 0.7217579f32, 0.62262124f32, -0.3023381f32, 0.0f32, 0.5656831f32, -0.8208298f32, -0.079000026f32, 0.0f32, 0.76004905f32, -0.55559796f32, -0.33709997f32, 0.0f32, 0.37139457f32, 0.50112647f32, 0.78162545f32, 0.0f32, -0.12770624f32, -0.4254439f32, -0.8959289f32, 0.0f32, -0.2881561f32, -0.5815839f32, 0.7607406f32, 0.0f32, 0.5849561f32, -0.6628202f32, -0.4674352f32, 0.0f32, 0.33071712f32, 0.039165374f32, 0.94291687f32, 0.0f32, 0.8712122f32, -0.41133744f32, -0.26793817f32, 0.0f32, 0.580981f32, 0.7021916f32, 0.41156778f32, 0.0f32, 0.5037569f32, 0.6330057f32, -0.5878204f32, 0.0f32, 0.44937122f32, 0.6013902f32, 0.6606023f32, 0.0f32, -0.6878404f32, 0.090188906f32, -0.7202372f32, 0.0f32, -0.59589565f32, -0.64693505f32, 0.47579765f32, 0.0f32, -0.5127052f32, 0.1946922f32, -0.83619875f32, 0.0f32, -0.99115074f32, -0.054102764f32, -0.12121531f32, 0.0f32, -0.21497211f32, 0.9720882f32, -0.09397608f32, 0.0f32, -0.7518651f32, -0.54280573f32, 0.37424695f32, 0.0f32, 0.5237069f32, 0.8516377f32, -0.021078179f32, 0.0f32, 0.6333505f32, 0.19261672f32, -0.74951047f32, 0.0f32, -0.06788242f32, 0.39983058f32, 0.9140719f32, 0.0f32, -0.55386287f32, -0.47298968f32, -0.6852129f32, 0.0f32, -0.72614557f32, -0.5911991f32, 0.35099334f32, 0.0f32, -0.9229275f32, -0.17828088f32, 0.34120494f32, 0.0f32, -0.6968815f32, 0.65112746f32, 0.30064803f32, 0.0f32, 0.96080446f32, -0.20983632f32, -0.18117249f32, 0.0f32, 0.068171464f32, -0.9743405f32, 0.21450691f32, 0.0f32, -0.3577285f32, -0.6697087f32, -0.65078455f32, 0.0f32, -0.18686211f32, 0.7648617f32, -0.61649746f32, 0.0f32, -0.65416974f32, 0.3967915f32, 0.64390874f32, 0.0f32, 0.699334f32, -0.6164538f32, 0.36182392f32, 0.0f32, -0.15466657f32, 0.6291284f32, 0.7617583f32, 0.0f32, -0.6841613f32, -0.2580482f32, -0.68215424f32, 0.0f32, 0.5383981f32, 0.4258655f32, 0.727163f32, 0.0f32, -0.5026988f32, -0.7939833f32, -0.3418837f32, 0.0f32, 0.32029718f32, 0.28344154f32, 0.9039196f32, 0.0f32, 0.86832273f32, -3.7626564e-4f32, -0.49599952f32, 0.0f32, 0.79112005f32, -0.085110456f32, 0.60571057f32, 0.0f32, -0.04011016f32, -0.43972486f32, 0.8972364f32, 0.0f32, 0.914512f32, 0.35793462f32, -0.18854876f32, 0.0f32, -0.96120393f32, -0.27564842f32, 0.010246669f32, 0.0f32, 0.65103614f32, -0.28777993f32, -0.70237786f32, 0.0f32, -0.20417863f32, 0.73652375f32, 0.6448596f32, 0.0f32, -0.7718264f32, 0.37906268f32, 0.5104856f32, 0.0f32, -0.30600828f32, -0.7692988f32, 0.56083715f32, 0.0f32, 0.45400733f32, -0.5024843f32, 0.73578995f32, 0.0f32, 0.48167956f32, 0.6021208f32, -0.636738f32, 0.0f32, 0.69619805f32, -0.32221973f32, 0.6414692f32, 0.0f32, -0.65321606f32, -0.6781149f32, 0.33685157f32, 0.0f32, 0.50893015f32, -0.61546624f32, -0.60182345f32, 0.0f32, -0.16359198f32, -0.9133605f32, -0.37284088f32, 0.0f32, 0.5240802f32, -0.8437664f32, 0.11575059f32, 0.0f32, 0.5902587f32, 0.4983818f32, -0.63498837f32, 0.0f32, 0.5863228f32, 0.49476475f32, 0.6414308f32, 0.0f32, 0.6779335f32, 0.23413453f32, 0.6968409f32, 0.0f32, 0.7177054f32, -0.68589795f32, 0.12017863f32, 0.0f32, -0.532882f32, -0.5205125f32, 0.6671608f32, 0.0f32, -0.8654874f32, -0.07007271f32, -0.4960054f32, 0.0f32, -0.286181f32, 0.79520893f32, 0.53454953f32, 0.0f32, -0.048495296f32, 0.98108363f32, -0.18741156f32, 0.0f32, -0.63585216f32, 0.60583484f32, 0.47818002f32, 0.0f32, 0.62547946f32, -0.28616196f32, 0.72586966f32, 0.0f32, -0.258526f32, 0.50619495f32, -0.8227582f32, 0.0f32, 0.021363068f32, 0.50640166f32, -0.862033f32, 0.0f32, 0.20011178f32, 0.85992634f32, 0.46955505f32, 0.0f32, 0.47435614f32, 0.6014985f32, -0.6427953f32, 0.0f32, 0.6622994f32, -0.52024746f32, -0.539168f32, 0.0f32, 0.08084973f32, -0.65327203f32, 0.7527941f32, 0.0f32, -0.6893687f32, 0.059286036f32, 0.7219805f32, 0.0f32, -0.11218871f32, -0.96731853f32, 0.22739525f32, 0.0f32, 0.7344116f32, 0.59796685f32, -0.3210533f32, 0.0f32, 0.5789393f32, -0.24888498f32, 0.776457f32, 0.0f32, 0.69881827f32, 0.35571697f32, -0.6205791f32, 0.0f32, -0.86368454f32, -0.27487713f32, -0.4224826f32, 0.0f32, -0.4247028f32, -0.46408808f32, 0.77733505f32, 0.0f32, 0.5257723f32, -0.84270173f32, 0.11583299f32, 0.0f32, 0.93438303f32, 0.31630248f32, -0.16395439f32, 0.0f32, -0.10168364f32, -0.8057303f32, -0.58348876f32, 0.0f32, -0.6529239f32, 0.50602126f32, -0.5635893f32, 0.0f32, -0.24652861f32, -0.9668206f32, -0.06694497f32, 0.0f32, -0.9776897f32, -0.20992506f32, -0.0073688254f32, 0.0f32, 0.7736893f32, 0.57342446f32, 0.2694238f32, 0.0f32, -0.6095088f32, 0.4995679f32, 0.6155737f32, 0.0f32, 0.5794535f32, 0.7434547f32, 0.33392924f32, 0.0f32, -0.8226211f32, 0.081425816f32, 0.56272936f32, 0.0f32, -0.51038545f32, 0.47036678f32, 0.719904f32, 0.0f32, -0.5764972f32, -0.072316565f32, -0.81389266f32, 0.0f32, 0.7250629f32, 0.39499715f32, -0.56414634f32, 0.0f32, -0.1525424f32, 0.48608407f32, -0.8604958f32, 0.0f32, -0.55509764f32, -0.49578208f32, 0.6678823f32, 0.0f32, -0.18836144f32, 0.91458696f32, 0.35784173f32, 0.0f32, 0.76255566f32, -0.54144084f32, -0.35404897f32, 0.0f32, -0.5870232f32, -0.3226498f32, -0.7424964f32, 0.0f32, 0.30511242f32, 0.2262544f32, -0.9250488f32, 0.0f32, 0.63795763f32, 0.57724243f32, -0.50970703f32, 0.0f32, -0.5966776f32, 0.14548524f32, -0.7891831f32, 0.0f32, -0.65833056f32, 0.65554875f32, -0.36994147f32, 0.0f32, 0.74348927f32, 0.23510846f32, 0.6260573f32, 0.0f32, 0.5562114f32, 0.82643604f32, -0.08736329f32, 0.0f32, -0.302894f32, -0.8251527f32, 0.47684193f32, 0.0f32, 0.11293438f32, -0.9858884f32, -0.123571075f32, 0.0f32, 0.5937653f32, -0.5896814f32, 0.5474657f32, 0.0f32, 0.6757964f32, -0.58357584f32, -0.45026484f32, 0.0f32, 0.7242303f32, -0.11527198f32, 0.67985505f32, 0.0f32, -0.9511914f32, 0.0753624f32, -0.29925808f32, 0.0f32, 0.2539471f32, -0.18863393f32, 0.9486454f32, 0.0f32, 0.5714336f32, -0.16794509f32, -0.8032796f32, 0.0f32, -0.06778235f32, 0.39782694f32, 0.9149532f32, 0.0f32, 0.6074973f32, 0.73306f32, -0.30589226f32, 0.0f32, -0.54354787f32, 0.16758224f32, 0.8224791f32, 0.0f32, -0.5876678f32, -0.3380045f32, -0.7351187f32, 0.0f32, -0.79675627f32, 0.040978227f32, -0.60290986f32, 0.0f32, -0.19963509f32, 0.8706295f32, 0.4496111f32, 0.0f32, -0.027876602f32, -0.91062325f32, -0.4122962f32, 0.0f32, -0.7797626f32, -0.6257635f32, 0.019757755f32, 0.0f32, -0.5211233f32, 0.74016446f32, -0.42495546f32, 0.0f32, 0.8575425f32, 0.4053273f32, -0.31675017f32, 0.0f32, 0.10452233f32, 0.8390196f32, -0.53396744f32, 0.0f32, 0.3501823f32, 0.9242524f32, -0.15208502f32, 0.0f32, 0.19878499f32, 0.076476134f32, 0.9770547f32, 0.0f32, 0.78459966f32, 0.6066257f32, -0.12809642f32, 0.0f32, 0.09006737f32, -0.97509897f32, -0.20265691f32, 0.0f32, -0.82743436f32, -0.54229957f32, 0.14582036f32, 0.0f32, -0.34857976f32, -0.41580227f32, 0.8400004f32, 0.0f32, -0.2471779f32, -0.730482f32, -0.6366311f32, 0.0f32, -0.3700155f32, 0.8577948f32, 0.35675845f32, 0.0f32, 0.59133947f32, -0.54831195f32, -0.59133035f32, 0.0f32, 0.120487355f32, -0.7626472f32, -0.6354935f32, 0.0f32, 0.6169593f32, 0.03079648f32, 0.7863923f32, 0.0f32, 0.12581569f32, -0.664083f32, -0.73699677f32, 0.0f32, -0.6477565f32, -0.17401473f32, -0.74170774f32, 0.0f32, 0.6217889f32, -0.7804431f32, -0.06547655f32, 0.0f32, 0.6589943f32, -0.6096988f32, 0.44044736f32, 0.0f32, -0.26898375f32, -0.6732403f32, -0.68876356f32, 0.0f32, -0.38497752f32, 0.56765425f32, 0.7277094f32, 0.0f32, 0.57544446f32, 0.81104714f32, -0.10519635f32, 0.0f32, 0.91415936f32, 0.3832948f32, 0.13190056f32, 0.0f32, -0.10792532f32, 0.9245494f32, 0.36545935f32, 0.0f32, 0.3779771f32, 0.30431488f32, 0.87437165f32, 0.0f32, -0.21428852f32, -0.8259286f32, 0.5214617f32, 0.0f32, 0.58025444f32, 0.41480985f32, -0.7008834f32, 0.0f32, -0.19826609f32, 0.85671616f32, -0.47615966f32, 0.0f32, -0.033815537f32, 0.37731808f32, -0.9254661f32, 0.0f32, -0.68679225f32, -0.6656598f32, 0.29191336f32, 0.0f32, 0.7731743f32, -0.28757936f32, -0.565243f32, 0.0f32, -0.09655942f32, 0.91937083f32, -0.3813575f32, 0.0f32, 0.27157024f32, -0.957791f32, -0.09426606f32, 0.0f32, 0.24510157f32, -0.6917999f32, -0.6792188f32, 0.0f32, 0.97770077f32, -0.17538553f32, 0.115503654f32, 0.0f32, -0.522474f32, 0.8521607f32, 0.029036159f32, 0.0f32, -0.77348804f32, -0.52612925f32, 0.35341796f32, 0.0f32, -0.71344924f32, -0.26954725f32, 0.6467878f32, 0.0f32, 0.16440372f32, 0.5105846f32, -0.84396374f32, 0.0f32, 0.6494636f32, 0.055856112f32, 0.7583384f32, 0.0f32, -0.4711971f32, 0.50172806f32, -0.7254256f32, 0.0f32, -0.63357645f32, -0.23816863f32, -0.7361091f32, 0.0f32, -0.9021533f32, -0.2709478f32, -0.33571818f32, 0.0f32, -0.3793711f32, 0.8722581f32, 0.3086152f32, 0.0f32, -0.68555987f32, -0.32501432f32, 0.6514394f32, 0.0f32, 0.29009423f32, -0.7799058f32, -0.5546101f32, 0.0f32, -0.20983194f32, 0.8503707f32, 0.48253515f32, 0.0f32, -0.45926037f32, 0.6598504f32, -0.5947077f32, 0.0f32, 0.87159455f32, 0.09616365f32, -0.48070312f32, 0.0f32, -0.6776666f32, 0.71185046f32, -0.1844907f32, 0.0f32, 0.7044378f32, 0.3124276f32, 0.637304f32, 0.0f32, -0.7052319f32, -0.24010932f32, -0.6670798f32, 0.0f32, 0.081921004f32, -0.72073364f32, -0.68835455f32, 0.0f32, -0.6993681f32, -0.5875763f32, -0.4069869f32, 0.0f32, -0.12814544f32, 0.6419896f32, 0.75592864f32, 0.0f32, -0.6337388f32, -0.67854714f32, -0.3714147f32, 0.0f32, 0.5565052f32, -0.21688876f32, -0.8020357f32, 0.0f32, -0.57915545f32, 0.7244372f32, -0.3738579f32, 0.0f32, 0.11757791f32, -0.7096451f32, 0.69467926f32, 0.0f32, -0.613462f32, 0.13236311f32, 0.7785528f32, 0.0f32, 0.69846356f32, -0.029805163f32, -0.7150247f32, 0.0f32, 0.83180827f32, -0.3930172f32, 0.39195976f32, 0.0f32, 0.14695764f32, 0.055416517f32, -0.98758924f32, 0.0f32, 0.70886856f32, -0.2690504f32, 0.65201014f32, 0.0f32, 0.27260533f32, 0.67369765f32, -0.68688995f32, 0.0f32, -0.65912956f32, 0.30354586f32, -0.68804663f32, 0.0f32, 0.48151314f32, -0.752827f32, 0.4487723f32, 0.0f32, 0.943001f32, 0.16756473f32, -0.28752613f32, 0.0f32, 0.43480295f32, 0.7695305f32, -0.46772778f32, 0.0f32, 0.39319962f32, 0.5944736f32, 0.70142365f32, 0.0f32, 0.72543365f32, -0.60392565f32, 0.33018148f32, 0.0f32, 0.75902355f32, -0.6506083f32, 0.024333132f32, 0.0f32, -0.8552769f32, -0.3430043f32, 0.38839358f32, 0.0f32, -0.6139747f32, 0.6981725f32, 0.36822575f32, 0.0f32, -0.74659055f32, -0.575201f32, 0.33428493f32, 0.0f32, 0.5730066f32, 0.8105555f32, -0.12109168f32, 0.0f32, -0.92258775f32, -0.3475211f32, -0.16751404f32, 0.0f32, -0.71058166f32, -0.47196922f32, -0.5218417f32, 0.0f32, -0.0856461f32, 0.35830015f32, 0.9296697f32, 0.0f32, -0.8279698f32, -0.2043157f32, 0.5222271f32, 0.0f32, 0.42794403f32, 0.278166f32, 0.8599346f32, 0.0f32, 0.539908f32, -0.78571206f32, -0.3019204f32, 0.0f32, 0.5678404f32, -0.5495414f32, -0.61283076f32, 0.0f32, -0.9896071f32, 0.13656391f32, -0.045034185f32, 0.0f32, -0.6154343f32, -0.64408755f32, 0.45430374f32, 0.0f32, 0.10742044f32, -0.79463404f32, 0.59750944f32, 0.0f32, -0.359545f32, -0.888553f32, 0.28495783f32, 0.0f32, -0.21804053f32, 0.1529889f32, 0.9638738f32, 0.0f32, -0.7277432f32, -0.61640507f32, -0.30072346f32, 0.0f32, 0.7249729f32, -0.0066971947f32, 0.68874484f32, 0.0f32, -0.5553659f32, -0.5336586f32, 0.6377908f32, 0.0f32, 0.5137558f32, 0.79762083f32, -0.316f32, 0.0f32, -0.3794025f32, 0.92456084f32, -0.035227515f32, 0.0f32, 0.82292485f32, 0.27453658f32, -0.49741766f32, 0.0f32, -0.5404114f32, 0.60911417f32, 0.5804614f32, 0.0f32, 0.8036582f32, -0.27030295f32, 0.5301602f32, 0.0f32, 0.60443187f32, 0.68329686f32, 0.40959433f32, 0.0f32, 0.06389989f32, 0.96582085f32, -0.2512108f32, 0.0f32, 0.10871133f32, 0.74024713f32, -0.6634878f32, 0.0f32, -0.7134277f32, -0.6926784f32, 0.10591285f32, 0.0f32, 0.64588976f32, -0.57245487f32, -0.50509584f32, 0.0f32, -0.6553931f32, 0.73814714f32, 0.15999562f32, 0.0f32, 0.39109614f32, 0.91888714f32, -0.05186756f32, 0.0f32, -0.48790225f32, -0.5904377f32, 0.64291114f32, 0.0f32, 0.601479f32, 0.77074414f32, -0.21018201f32, 0.0f32, -0.5677173f32, 0.7511361f32, 0.33688518f32, 0.0f32, 0.7858574f32, 0.22667466f32, 0.5753667f32, 0.0f32, -0.45203456f32, -0.6042227f32, -0.65618575f32, 0.0f32, 0.0022721163f32, 0.4132844f32, -0.9105992f32, 0.0f32, -0.58157516f32, -0.5162926f32, 0.6286591f32, 0.0f32, -0.03703705f32, 0.8273786f32, 0.5604221f32, 0.0f32, -0.51196927f32, 0.79535437f32, -0.324498f32, 0.0f32, -0.26824173f32, -0.957229f32, -0.10843876f32, 0.0f32, -0.23224828f32, -0.9679131f32, -0.09594243f32, 0.0f32, 0.3554329f32, -0.8881506f32, 0.29130062f32, 0.0f32, 0.73465204f32, -0.4371373f32, 0.5188423f32, 0.0f32, 0.998512f32, 0.046590112f32, -0.028339446f32, 0.0f32, -0.37276876f32, -0.9082481f32, 0.19007573f32, 0.0f32, 0.9173738f32, -0.3483642f32, 0.19252984f32, 0.0f32, 0.2714911f32, 0.41475296f32, -0.86848867f32, 0.0f32, 0.5131763f32, -0.71163344f32, 0.4798207f32, 0.0f32, -0.87373537f32, 0.18886992f32, -0.44823506f32, 0.0f32, 0.84600437f32, -0.3725218f32, 0.38145f32, 0.0f32, 0.89787275f32, -0.17802091f32, -0.40265754f32, 0.0f32, 0.21780656f32, -0.9698323f32, -0.10947895f32, 0.0f32, -0.15180314f32, -0.7788918f32, -0.6085091f32, 0.0f32, -0.2600385f32, -0.4755398f32, -0.840382f32, 0.0f32, 0.5723135f32, -0.7474341f32, -0.33734185f32, 0.0f32, -0.7174141f32, 0.16990171f32, -0.67561114f32, 0.0f32, -0.6841808f32, 0.021457076f32, -0.72899675f32, 0.0f32, -0.2007448f32, 0.06555606f32, -0.9774477f32, 0.0f32, -0.11488037f32, -0.8044887f32, 0.5827524f32, 0.0f32, -0.787035f32, 0.03447489f32, 0.6159443f32, 0.0f32, -0.20155965f32, 0.68598723f32, 0.69913894f32, 0.0f32, -0.085810825f32, -0.10920836f32, -0.99030805f32, 0.0f32, 0.5532693f32, 0.73252505f32, -0.39661077f32, 0.0f32, -0.18424894f32, -0.9777375f32, -0.100407675f32, 0.0f32, 0.07754738f32, -0.9111506f32, 0.40471104f32, 0.0f32, 0.13998385f32, 0.7601631f32, -0.63447344f32, 0.0f32, 0.44844192f32, -0.84528923f32, 0.29049253f32, 0.0f32]);
        pub const PRIME_X: i32 = 501125321;
        pub const PRIME_Y: i32 = 1136930381;
        pub const PRIME_Z: i32 = 1720413743;
        // FNL.FNL(0): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.FNL(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetSeed(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetFrequency(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetNoiseType(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetRotationType3D(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetFractalType(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetFractalOctaves(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetFractalLacunarity(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetFractalGain(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetFractalWeightedStrength(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetFractalPingPongStrength(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetCellularDistanceFunction(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetCellularReturnType(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetCellularJitter(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetDomainWarpType(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SetDomainWarpAmp(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GetNoise(2): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GetNoise(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarp(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarp(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        pub fn fast_min(a: f32, b: f32) -> f32 {
            return if a < b { a } else { b };
        }
        pub fn fast_max(a: f32, b: f32) -> f32 {
            return if a > b { a } else { b };
        }
        pub fn fast_abs(f: f32) -> f32 {
            return if f < 0.0f32 { -f } else { f };
        }
        pub fn fast_sqrt(f: f32) -> f32 {
            return (((f as f64).sqrt()) as f32);
        }
        pub fn fast_floor(f: f64) -> i32 {
            return if f >= 0.0f64 { (f as i32) } else { (f as i32) - 1 };
        }
        pub fn fast_round(f: f64) -> i32 {
            return if f >= 0.0f64 { ((f + 0.5f64) as i32) } else { ((f - 0.5f64) as i32) };
        }
        pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
            return a + (t * (b - a));
        }
        pub fn interp_hermite(t: f32) -> f32 {
            return (t * t) * (3.0f32 - (2.0f32 * t));
        }
        pub fn interp_quintic(t: f32) -> f32 {
            return ((t * t) * t) * ((t * ((t * 6.0f32) - 15.0f32)) + 10.0f32);
        }
        pub fn cubic_lerp(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
            let p: f32 = (d - c) - (a - b);
            return (((((t * t) * t) * p) + ((t * t) * ((a - b) - p))) + (t * (c - a))) + b;
        }
        pub fn ping_pong(mut t: f32) -> f32 {
            t -= ((((t * 0.5f32) as i32) * 2) as f32);
            return if t < 1.0f32 { t } else { 2.0f32 - t };
        }
        // FNL.CalculateFractalBounding(0): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        pub fn hash_3(seed: i32, x_primed: i32, y_primed: i32) -> i32 {
            let hash: i32 = (seed ^ x_primed) ^ y_primed;
            return hash * 668265261;
        }
        pub fn hash_4(seed: i32, x_primed: i32, y_primed: i32, z_primed: i32) -> i32 {
            let hash: i32 = ((seed ^ x_primed) ^ y_primed) ^ z_primed;
            return hash * 668265261;
        }
        pub fn val_coord_3(seed: i32, x_primed: i32, y_primed: i32) -> f32 {
            let mut hash: i32 = fnl::hash_3(seed, x_primed, y_primed);
            hash *= hash;
            hash ^= hash << 19;
            return (hash as f32) * (4.656613e-10f32);
        }
        pub fn val_coord_4(seed: i32, x_primed: i32, y_primed: i32, z_primed: i32) -> f32 {
            let mut hash: i32 = fnl::hash_4(seed, x_primed, y_primed, z_primed);
            hash *= hash;
            hash ^= hash << 19;
            return (hash as f32) * (4.656613e-10f32);
        }
        pub fn grad_coord_5(seed: i32, x_primed: i32, y_primed: i32, xd: f32, yd: f32) -> f32 {
            let mut hash: i32 = fnl::hash_3(seed, x_primed, y_primed);
            hash ^= hash >> 15;
            hash &= 254;
            let xg: f32 = (*fnl::GRADIENTS2_D)[hash as usize];
            let yg: f32 = (*fnl::GRADIENTS2_D)[(hash | 1) as usize];
            return (xd * xg) + (yd * yg);
        }
        pub fn grad_coord_7(seed: i32, x_primed: i32, y_primed: i32, z_primed: i32, xd: f32, yd: f32, zd: f32) -> f32 {
            let mut hash: i32 = fnl::hash_4(seed, x_primed, y_primed, z_primed);
            hash ^= hash >> 15;
            hash &= 252;
            let xg: f32 = (*fnl::GRADIENTS3_D)[hash as usize];
            let yg: f32 = (*fnl::GRADIENTS3_D)[(hash | 1) as usize];
            let zg: f32 = (*fnl::GRADIENTS3_D)[(hash | 2) as usize];
            return ((xd * xg) + (yd * yg)) + (zd * zg);
        }
        // FNL.GenNoiseSingle(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GenNoiseSingle(4): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.UpdateTransformType3D(0): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.UpdateWarpTransformType3D(0): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GenFractalFBm(2): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GenFractalFBm(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GenFractalRidged(2): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GenFractalRidged(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GenFractalPingPong(2): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.GenFractalPingPong(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleSimplex(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleOpenSimplex2(4): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleOpenSimplex2S(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleOpenSimplex2S(4): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleCellular(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleCellular(4): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SinglePerlin(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SinglePerlin(4): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleValueCubic(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleValueCubic(4): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleValue(3): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleValue(4): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DoSingleDomainWarp(6): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DoSingleDomainWarp(7): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarpSingle(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarpSingle(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarpFractalProgressive(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarpFractalProgressive(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarpFractalIndependent(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.DomainWarpFractalIndependent(1): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleDomainWarpBasicGrid(6): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleDomainWarpBasicGrid(7): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleDomainWarpSimplexGradient(7): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
        // FNL.SingleDomainWarpOpenSimplex2Gradient(8): not translated, its class isn't translated (it uses FractalType, which isn't translated (its constants carry values)).
    }

    /// `FNL.CellularDistanceFunction` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_cellular_distance_function {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum CellularDistanceFunction {
            Euclidean,
            Euclideansq,
            Manhattan,
            Hybrid,
        }
        impl CellularDistanceFunction {
            pub const VALUES: [CellularDistanceFunction; 4] = [CellularDistanceFunction::Euclidean, CellularDistanceFunction::Euclideansq, CellularDistanceFunction::Manhattan, CellularDistanceFunction::Hybrid, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    CellularDistanceFunction::Euclidean => "Euclidean",
                    CellularDistanceFunction::Euclideansq => "EuclideanSq",
                    CellularDistanceFunction::Manhattan => "Manhattan",
                    CellularDistanceFunction::Hybrid => "Hybrid",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "Euclidean" => CellularDistanceFunction::Euclidean,
                    "EuclideanSq" => CellularDistanceFunction::Euclideansq,
                    "Manhattan" => CellularDistanceFunction::Manhattan,
                    "Hybrid" => CellularDistanceFunction::Hybrid,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FNL.CellularReturnType` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_cellular_return_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum CellularReturnType {
            Cellvalue,
            Distance,
            Distance2,
            Distance2add,
            Distance2sub,
            Distance2mul,
            Distance2div,
        }
        impl CellularReturnType {
            pub const VALUES: [CellularReturnType; 7] = [CellularReturnType::Cellvalue, CellularReturnType::Distance, CellularReturnType::Distance2, CellularReturnType::Distance2add, CellularReturnType::Distance2sub, CellularReturnType::Distance2mul, CellularReturnType::Distance2div, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    CellularReturnType::Cellvalue => "CellValue",
                    CellularReturnType::Distance => "Distance",
                    CellularReturnType::Distance2 => "Distance2",
                    CellularReturnType::Distance2add => "Distance2Add",
                    CellularReturnType::Distance2sub => "Distance2Sub",
                    CellularReturnType::Distance2mul => "Distance2Mul",
                    CellularReturnType::Distance2div => "Distance2Div",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "CellValue" => CellularReturnType::Cellvalue,
                    "Distance" => CellularReturnType::Distance,
                    "Distance2" => CellularReturnType::Distance2,
                    "Distance2Add" => CellularReturnType::Distance2add,
                    "Distance2Sub" => CellularReturnType::Distance2sub,
                    "Distance2Mul" => CellularReturnType::Distance2mul,
                    "Distance2Div" => CellularReturnType::Distance2div,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FNL.DomainWarpType` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_domain_warp_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum DomainWarpType {
            Opensimplex2,
            Opensimplex2reduced,
            Basicgrid,
        }
        impl DomainWarpType {
            pub const VALUES: [DomainWarpType; 3] = [DomainWarpType::Opensimplex2, DomainWarpType::Opensimplex2reduced, DomainWarpType::Basicgrid, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    DomainWarpType::Opensimplex2 => "OpenSimplex2",
                    DomainWarpType::Opensimplex2reduced => "OpenSimplex2Reduced",
                    DomainWarpType::Basicgrid => "BasicGrid",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "OpenSimplex2" => DomainWarpType::Opensimplex2,
                    "OpenSimplex2Reduced" => DomainWarpType::Opensimplex2reduced,
                    "BasicGrid" => DomainWarpType::Basicgrid,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FNL.FractalType` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_fractal_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // FNL.FractalType.FractalType(1): not translated, its class isn't translated (its constants carry values).
        // FNL.FractalType.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `FNL.NoiseType` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_noise_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum NoiseType {
            Opensimplex2,
            Opensimplex2s,
            Cellular,
            Perlin,
            Valuecubic,
            Value,
        }
        impl NoiseType {
            pub const VALUES: [NoiseType; 6] = [NoiseType::Opensimplex2, NoiseType::Opensimplex2s, NoiseType::Cellular, NoiseType::Perlin, NoiseType::Valuecubic, NoiseType::Value, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    NoiseType::Opensimplex2 => "OpenSimplex2",
                    NoiseType::Opensimplex2s => "OpenSimplex2S",
                    NoiseType::Cellular => "Cellular",
                    NoiseType::Perlin => "Perlin",
                    NoiseType::Valuecubic => "ValueCubic",
                    NoiseType::Value => "Value",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "OpenSimplex2" => NoiseType::Opensimplex2,
                    "OpenSimplex2S" => NoiseType::Opensimplex2s,
                    "Cellular" => NoiseType::Cellular,
                    "Perlin" => NoiseType::Perlin,
                    "ValueCubic" => NoiseType::Valuecubic,
                    "Value" => NoiseType::Value,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FNL.RotationType3D` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_rotation_type3_d {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum RotationType3D {
            None,
            Improvexyplanes,
            Improvexzplanes,
        }
        impl RotationType3D {
            pub const VALUES: [RotationType3D; 3] = [RotationType3D::None, RotationType3D::Improvexyplanes, RotationType3D::Improvexzplanes, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    RotationType3D::None => "None",
                    RotationType3D::Improvexyplanes => "ImproveXYPlanes",
                    RotationType3D::Improvexzplanes => "ImproveXZPlanes",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "None" => RotationType3D::None,
                    "ImproveXYPlanes" => RotationType3D::Improvexyplanes,
                    "ImproveXZPlanes" => RotationType3D::Improvexzplanes,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FNL.TransformType3D` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_transform_type3_d {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum TransformType3D {
            None,
            Improvexyplanes,
            Improvexzplanes,
            Defaultopensimplex2,
        }
        impl TransformType3D {
            pub const VALUES: [TransformType3D; 4] = [TransformType3D::None, TransformType3D::Improvexyplanes, TransformType3D::Improvexzplanes, TransformType3D::Defaultopensimplex2, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    TransformType3D::None => "None",
                    TransformType3D::Improvexyplanes => "ImproveXYPlanes",
                    TransformType3D::Improvexzplanes => "ImproveXZPlanes",
                    TransformType3D::Defaultopensimplex2 => "DefaultOpenSimplex2",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "None" => TransformType3D::None,
                    "ImproveXYPlanes" => TransformType3D::Improvexyplanes,
                    "ImproveXZPlanes" => TransformType3D::Improvexzplanes,
                    "DefaultOpenSimplex2" => TransformType3D::Defaultopensimplex2,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `FNL.Vector2` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_vector2 {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vector2 {
            pub x: f64,
            pub y: f64,
        }
        impl Vector2 {
            pub fn new(x: f64, y: f64) -> Self {
                let mut s = Self { x: x, y: y };
                s.x = x;
                s.y = y;
                s
            }
        }
    }

    /// `FNL.Vector3` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/FNL.java)
    pub mod fnl_vector3 {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vector3 {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }
        impl Vector3 {
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                let mut s = Self { x: x, y: y, z: z };
                s.x = x;
                s.y = y;
                s.z = z;
                s
            }
        }
    }

    /// `LithostitchedFastNoiseConfigs` (dev/worldgen/lithostitched/api/worldgen/densityfunction/fastnoise/LithostitchedFastNoiseConfigs.java)
    pub mod lithostitched_fast_noise_configs {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedFastNoiseConfigs.cellular(5): not translated, it uses FastNoiseConfig, which isn't translated (it implements Minecraft's `Noise`).
        // LithostitchedFastNoiseConfigs.perlin(2): not translated, it uses FastNoiseConfig, which isn't translated (it implements Minecraft's `Noise`).
        // LithostitchedFastNoiseConfigs.simplex(6): not translated, it uses FastNoiseConfig, which isn't translated (it implements Minecraft's `Noise`).
    }

    /// `LithostitchedDensityFunctions` (dev/worldgen/lithostitched/api/worldgen/densityfunction/LithostitchedDensityFunctions.java)
    pub mod lithostitched_density_functions {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedDensityFunctions.cos(1): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.shift(4): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.sin(1): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.cellular2d(4): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.cellular2d(8): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.cellular3d(5): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.cellular3d(9): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.fastNoise(3): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.fastNoise(6): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.wrappedMarker(0): not translated, it uses Minecraft's `DensityFunction`.
        // LithostitchedDensityFunctions.originalMarker(0): not translated, it uses Minecraft's `DensityFunction`.
    }

    /// `LithostitchedFeatures` (dev/worldgen/lithostitched/api/worldgen/feature/LithostitchedFeatures.java)
    pub mod lithostitched_features {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedFeatures.placeAll(1): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.placeUntilFailure(1): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.placeUntilSuccess(1): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.dungeon(9): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.largeDripstone(11): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.ore(2): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.placed(1): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.select(1): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.structureTemplate(2): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.structureTemplate(5): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.structureTemplate(2): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.structureTemplate(5): not translated, it uses Minecraft's `Feature`.
        // LithostitchedFeatures.vines(3): not translated, it uses Minecraft's `Feature`.
    }

    /// `LithostitchedMaterialConditions` (dev/worldgen/lithostitched/api/worldgen/material/LithostitchedMaterialConditions.java)
    pub mod lithostitched_material_conditions {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedMaterialConditions.allOf(1): not translated, it uses Minecraft's `MaterialCondition`.
        // LithostitchedMaterialConditions.anyOf(1): not translated, it uses Minecraft's `MaterialCondition`.
        // LithostitchedMaterialConditions.slope(2): not translated, it uses Minecraft's `MaterialCondition`.
        // LithostitchedMaterialConditions.sampleDensity(4): not translated, it uses Minecraft's `MaterialCondition`.
    }

    /// `LithostitchedMaterialRules` (dev/worldgen/lithostitched/api/worldgen/material/LithostitchedMaterialRules.java)
    pub mod lithostitched_material_rules {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedMaterialRules.bandlands(1): not translated, it uses Minecraft's `MaterialRule`.
    }

    /// `BiomeEffectsBuilder` (dev/worldgen/lithostitched/api/worldgen/modifier/BiomeEffectsBuilder.java)
    pub mod biome_effects_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomeEffectsBuilder.create(0): not translated, it uses BiomeEffectsBuilder, which isn't translated (it's an interface).
        // BiomeEffectsBuilder.waterColor(1): not translated, it has no body.
        // BiomeEffectsBuilder.foliageColor(1): not translated, it has no body.
        // BiomeEffectsBuilder.dryFoliageColor(1): not translated, it has no body.
        // BiomeEffectsBuilder.grassColor(1): not translated, it has no body.
        // BiomeEffectsBuilder.grassColorModifier(1): not translated, it has no body.
        // BiomeEffectsBuilder.build(0): not translated, it has no body.
    }

    /// `HolderTarget` (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java)
    pub mod holder_target {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HolderTarget.getHolders(1): not translated, it has no body.
        // HolderTarget.getType(0): not translated, it has no body.
        // HolderTarget.getHoldersAsSet(1): not translated, its class isn't translated (it's an interface).
        // HolderTarget.of(1): not translated, it's generic.
        // HolderTarget.of(1): not translated, it's generic.
        // HolderTarget.createCodec(1): not translated, it's generic.
    }

    /// `HolderTarget.SimpleType` (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java)
    pub mod holder_target_simple_type {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HolderTarget.SimpleType.apply(1): not translated, it's generic.
        // HolderTarget.SimpleType.applySimple(1): not translated, it has no body.
    }

    /// `HolderTarget.Type` (dev/worldgen/lithostitched/api/worldgen/modifier/target/HolderTarget.java)
    pub mod holder_target_type {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // HolderTarget.Type.apply(1): not translated, it has no body.
    }

    /// `WorldgenModifier` (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java)
    pub mod worldgen_modifier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // WorldgenModifier.predicate(0): not translated, it has no body.
        // WorldgenModifier.priority(0): not translated, it has no body.
        // WorldgenModifier.apply(1): not translated, it has no body.
        // WorldgenModifier.codec(0): not translated, it has no body.
        // WorldgenModifier.shouldRecompileSortedFeatures(0): not translated, its class isn't translated (it's an interface).
        // WorldgenModifier.resetRegistrationInfo(2): not translated, it's generic.
        // WorldgenModifier.builder(0): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
        // WorldgenModifier.builder(1): not translated, it uses LoadPredicate, which isn't translated (it's an interface).
    }

    /// `WorldgenModifier.ModifierBuilder` (dev/worldgen/lithostitched/api/worldgen/modifier/WorldgenModifier.java)
    pub mod worldgen_modifier_modifier_builder {
        use super::*;
        // The class itself isn't translated: it uses LoadPredicate, which isn't translated (it's an interface). Its static parts are below.
        // WorldgenModifier.ModifierBuilder.ModifierBuilder(1): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.priority(1): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addProcessorListProcessors(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addProcessorListProcessors(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addProcessorListProcessors(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addSpawnCosts(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addSpawnCosts(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addSpawnCosts(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addStructureSetEntries(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addStructureSetEntries(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addStructureSetEntries(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addTemplatePoolElements(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addTemplatePoolElements(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.addTemplatePoolElements(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.noop(0): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeBiomeSpawns(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeStructureSetEntries(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeStructureSetEntries(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.removeStructureSetEntries(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeAttributes(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeAttributes(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeAttributes(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeAttributes(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeClimate(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeClimate(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeClimate(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeEffects(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeEffects(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setBiomeEffects(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setMaterialRule(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setMaterialRule(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setMaterialRule(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setMaterialRule(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolAliases(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolAliases(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolAliases(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolElementProcessors(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolElementProcessors(4): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolElementProcessors(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolElementProcessors(4): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolElementProcessors(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setPoolElementProcessors(4): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setStructureSpawnCondition(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setStructureSpawnCondition(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.setStructureSpawnCondition(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.stackFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.stackFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.stackFeatures(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.wrapAquifers(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.wrapDensityFunction(2): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
        // WorldgenModifier.ModifierBuilder.wrapNoiseRouter(3): not translated, its class isn't translated (it uses LoadPredicate, which isn't translated (it's an interface)).
    }

    /// `LithostitchedPlacementConditions` (dev/worldgen/lithostitched/api/worldgen/placementcondition/LithostitchedPlacementConditions.java)
    pub mod lithostitched_placement_conditions {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedPlacementConditions.allOf(1): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.anyOf(1): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.grid(4): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.absoluteHeightFilter(2): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.heightmapRelativeFilter(2): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.inBiome(1): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.inBiome(1): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.multipleOf(2): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.not(1): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.offset(2): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.sampleDensity(2): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.sampleNoiseRouter(2): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
        // LithostitchedPlacementConditions.alwaysTrue(0): not translated, it uses PlacementCondition, which isn't translated (it's an interface).
    }

    /// `PlacementCondition` (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java)
    pub mod placement_condition {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PlacementCondition.test(2): not translated, it has no body.
        // PlacementCondition.test(2): not translated, it has no body.
        // PlacementCondition.test(2): not translated, it has no body.
        // PlacementCondition.codec(0): not translated, it has no body.
    }

    /// `PlacementCondition.Context` (dev/worldgen/lithostitched/api/worldgen/placementcondition/PlacementCondition.java)
    pub mod placement_condition_context {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `RegistryAccess`. Its static parts are below.
        // PlacementCondition.Context.Context(6): not translated, its class isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.create(1): not translated, it uses Context, which isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.create(1): not translated, it uses Context, which isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.registries(0): not translated, its class isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.generator(0): not translated, its class isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.heightAccessor(0): not translated, its class isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.randomState(0): not translated, its class isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.biomeResolver(0): not translated, its class isn't translated (it uses Minecraft's `RegistryAccess`).
        // PlacementCondition.Context.seed(0): not translated, its class isn't translated (it uses Minecraft's `RegistryAccess`).
    }

    /// `LithostitchedPlacementModifiers` (dev/worldgen/lithostitched/api/worldgen/placementmodifier/LithostitchedPlacementModifiers.java)
    pub mod lithostitched_placement_modifiers {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedPlacementModifiers.condition(1): not translated, it uses Minecraft's `PlacementModifier`.
        // LithostitchedPlacementModifiers.noiseSlope(5): not translated, it uses Minecraft's `PlacementModifier`.
    }

    /// `DelegatingElementBuilder` (dev/worldgen/lithostitched/api/worldgen/poolelement/DelegatingElementBuilder.java)
    pub mod delegating_element_builder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // DelegatingElementBuilder.create(1): not translated, it uses DelegatingElementBuilder, which isn't translated (it's an interface).
        // DelegatingElementBuilder.forcedCount(1): not translated, it has no body.
        // DelegatingElementBuilder.limitedCount(1): not translated, it has no body.
        // DelegatingElementBuilder.allowedDepth(1): not translated, it has no body.
        // DelegatingElementBuilder.terrainAdaptation(1): not translated, it has no body.
        // DelegatingElementBuilder.condition(1): not translated, it has no body.
        // DelegatingElementBuilder.named(1): not translated, it has no body.
        // DelegatingElementBuilder.terrainMatchingHeightmap(1): not translated, it has no body.
        // DelegatingElementBuilder.allowBoundingBoxCollisions(0): not translated, it has no body.
        // DelegatingElementBuilder.otherPiecesCanIntersect(0): not translated, it has no body.
        // DelegatingElementBuilder.build(0): not translated, it has no body.
    }

    /// `LithostitchedPoolElements` (dev/worldgen/lithostitched/api/worldgen/poolelement/LithostitchedPoolElements.java)
    pub mod lithostitched_pool_elements {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedPoolElements.guaranteed(2): not translated, it uses Minecraft's `StructurePoolElement`.
        // LithostitchedPoolElements.limited(2): not translated, it uses Minecraft's `StructurePoolElement`.
        // LithostitchedPoolElements.delegating(2): not translated, it uses Minecraft's `StructurePoolElement`.
    }

    /// `LithostitchedProcessorConditions` (dev/worldgen/lithostitched/api/worldgen/processorcondition/LithostitchedProcessorConditions.java)
    pub mod lithostitched_processor_conditions {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedProcessorConditions.allOf(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.anyOf(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.matchingBiomes(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.matchingBiomes(2): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.matchingBiomes(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.matchingBiomes(2): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.matchingBlocks(3): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.matchingBlocks(3): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.matchingBlocks(3): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.not(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.piecePosition(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.structureStartPosition(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.randomChance(1): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
        // LithostitchedProcessorConditions.alwaysTrue(0): not translated, it uses ProcessorCondition, which isn't translated (it's an interface).
    }

    /// `ProcessorCondition` (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java)
    pub mod processor_condition {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ProcessorCondition.test(4): not translated, it has no body.
        // ProcessorCondition.codec(0): not translated, it has no body.
    }

    /// `ProcessorCondition.Data` (dev/worldgen/lithostitched/api/worldgen/processorcondition/ProcessorCondition.java)
    pub mod processor_condition_data {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPos`. Its static parts are below.
        // ProcessorCondition.Data.Data(4): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ProcessorCondition.Data.pos(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ProcessorCondition.Data.pivot(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ProcessorCondition.Data.relative(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
        // ProcessorCondition.Data.absolute(0): not translated, its class isn't translated (it uses Minecraft's `BlockPos`).
    }

    /// `BlockType` (dev/worldgen/lithostitched/api/worldgen/processor/enums/BlockType.java)
    pub mod block_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // BlockType.BlockType(2): not translated, its class isn't translated (its constants carry values).
        // BlockType.state(1): not translated, its class isn't translated (its constants carry values).
        // BlockType.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ProcessorPosition` (dev/worldgen/lithostitched/api/worldgen/processor/enums/ProcessorPosition.java)
    pub mod processor_position {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // ProcessorPosition.ProcessorPosition(1): not translated, its class isn't translated (its constants carry values).
        // ProcessorPosition.select(1): not translated, its class isn't translated (its constants carry values).
        // ProcessorPosition.select(3): not translated, its class isn't translated (its constants carry values).
        // ProcessorPosition.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `RandomMode` (dev/worldgen/lithostitched/api/worldgen/processor/enums/RandomMode.java)
    pub mod random_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // RandomMode.RandomMode(1): not translated, its class isn't translated (its constants carry values).
        // RandomMode.select(3): not translated, its class isn't translated (its constants carry values).
        // RandomMode.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `LithostitchedProcessorLists` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessorLists.java)
    pub mod lithostitched_processor_lists {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedProcessorLists.key(1): not translated, it uses Minecraft's `ResourceKey`.
        // LithostitchedProcessorLists.pick(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `LithostitchedProcessors` (dev/worldgen/lithostitched/api/worldgen/processor/LithostitchedProcessors.java)
    pub mod lithostitched_processors {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedProcessors.applyRandom(2): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.applyRandom(2): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.blockSwap(1): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.condition(3): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.condition(4): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.discardInput(0): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.reference(1): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.reference(1): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.scheduleTick(0): not translated, it uses Minecraft's `StructureProcessor`.
        // LithostitchedProcessors.setBlock(4): not translated, it uses Minecraft's `StructureProcessor`.
    }

    /// `RandomSettings` (dev/worldgen/lithostitched/api/worldgen/processor/RandomSettings.java)
    pub mod random_settings {
        use super::*;
        // The class itself isn't translated: it uses RandomMode, which isn't translated (its constants carry values). Its static parts are below.
        // FULL_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // RandomSettings.RandomSettings(2): not translated, its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)).
        // RandomSettings.RandomSettings(1): not translated, its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)).
        // RandomSettings.create(4): not translated, its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)).
        // RandomSettings.mode(0): not translated, its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)).
        // RandomSettings.name(0): not translated, its class isn't translated (it uses RandomMode, which isn't translated (its constants carry values)).
    }

    /// `LithostitchedStateProviders` (dev/worldgen/lithostitched/api/worldgen/stateprovider/LithostitchedStateProviders.java)
    pub mod lithostitched_state_providers {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedStateProviders.weighted(1): not translated, it uses Minecraft's `BlockStateProvider`.
    }

    /// `LithostitchedStructures` (dev/worldgen/lithostitched/api/worldgen/structure/LithostitchedStructures.java)
    pub mod lithostitched_structures {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedStructures.jigsaw(6): not translated, it uses Minecraft's `Structure`.
        // LithostitchedStructures.jigsaw(12): not translated, it uses Minecraft's `Structure`.
        // LithostitchedStructures.delegating(2): not translated, it uses Minecraft's `Structure`.
    }

    /// `MaxDistanceFromCenter` (dev/worldgen/lithostitched/api/worldgen/structure/MaxDistanceFromCenter.java)
    pub mod max_distance_from_center {
        use super::*;
        // BASE_CODEC: not translated, it uses Minecraft's `Codec`.
        // FULL_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct MaxDistanceFromCenter {
            pub horizontal: i32,
            pub vertical: i32,
        }
        impl MaxDistanceFromCenter {
            pub fn new_2(horizontal: i32, vertical: i32) -> Self {
                let mut s = Self { horizontal: horizontal, vertical: vertical };
                s.horizontal = horizontal;
                s.vertical = vertical;
                s
            }
            pub fn new_1(value: i32) -> Self {
                let mut s = Self::new_2(value, value);
                s
            }
            pub fn horizontal(&self) -> i32 {
                return self.horizontal;
            }
            pub fn vertical(&self) -> i32 {
                return self.vertical;
            }
        }
        pub fn of_1(value: i32) -> max_distance_from_center::MaxDistanceFromCenter {
            return max_distance_from_center::MaxDistanceFromCenter::new_1(value);
        }
        pub fn of_2(horizontal: i32, vertical: i32) -> max_distance_from_center::MaxDistanceFromCenter {
            return max_distance_from_center::MaxDistanceFromCenter::new_2(horizontal, vertical);
        }
    }

    /// `SurfaceSnap` (dev/worldgen/lithostitched/api/worldgen/structure/SurfaceSnap.java)
    pub mod surface_snap {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // SurfaceSnap.SurfaceSnap(2): not translated, its class isn't translated (its constants carry values).
        // SurfaceSnap.findY(4): not translated, its class isn't translated (its constants carry values).
        // SurfaceSnap.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `AquiferTarget` (dev/worldgen/lithostitched/api/worldgen/util/AquiferTarget.java)
    pub mod aquifer_target {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // AquiferTarget.AquiferTarget(2): not translated, its class isn't translated (its constants carry values).
        // AquiferTarget.getDensityFunction(1): not translated, its class isn't translated (its constants carry values).
        // AquiferTarget.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `BiomeClimate` (dev/worldgen/lithostitched/api/worldgen/util/BiomeClimate.java)
    pub mod biome_climate {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `TemperatureModifier`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BiomeClimate.BiomeClimate(4): not translated, its class isn't translated (it uses Minecraft's `TemperatureModifier`).
        // BiomeClimate.hasPrecipitation(0): not translated, its class isn't translated (it uses Minecraft's `TemperatureModifier`).
        // BiomeClimate.temperature(0): not translated, its class isn't translated (it uses Minecraft's `TemperatureModifier`).
        // BiomeClimate.temperatureModifier(0): not translated, its class isn't translated (it uses Minecraft's `TemperatureModifier`).
        // BiomeClimate.downfall(0): not translated, its class isn't translated (it uses Minecraft's `TemperatureModifier`).
    }

    /// `BiomeEffects` (dev/worldgen/lithostitched/api/worldgen/util/BiomeEffects.java)
    pub mod biome_effects {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GrassColorModifier`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // BiomeEffects.BiomeEffects(5): not translated, its class isn't translated (it uses Minecraft's `GrassColorModifier`).
        // BiomeEffects.waterColor(0): not translated, its class isn't translated (it uses Minecraft's `GrassColorModifier`).
        // BiomeEffects.foliageColor(0): not translated, its class isn't translated (it uses Minecraft's `GrassColorModifier`).
        // BiomeEffects.dryFoliageColor(0): not translated, its class isn't translated (it uses Minecraft's `GrassColorModifier`).
        // BiomeEffects.grassColor(0): not translated, its class isn't translated (it uses Minecraft's `GrassColorModifier`).
        // BiomeEffects.grassColorModifier(0): not translated, its class isn't translated (it uses Minecraft's `GrassColorModifier`).
    }

    /// `DensityFunctionWrapper` (dev/worldgen/lithostitched/api/worldgen/util/DensityFunctionWrapper.java)
    pub mod density_function_wrapper {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CompileContext`. Its static parts are below.
        // DensityFunctionWrapper.DensityFunctionWrapper(2): not translated, its class isn't translated (it implements Minecraft's `CompileContext`).
        // DensityFunctionWrapper.DensityFunctionWrapper(4): not translated, its class isn't translated (it implements Minecraft's `CompileContext`).
        // DensityFunctionWrapper.newLegacyInstance(1): not translated, its class isn't translated (it implements Minecraft's `CompileContext`).
        // DensityFunctionWrapper.createNoiseSampler(1): not translated, its class isn't translated (it implements Minecraft's `CompileContext`).
        // DensityFunctionWrapper.createRandom(1): not translated, its class isn't translated (it implements Minecraft's `CompileContext`).
        // DensityFunctionWrapper.createEndIslandRandom(0): not translated, its class isn't translated (it implements Minecraft's `CompileContext`).
    }

    /// `NoiseRouterTarget` (dev/worldgen/lithostitched/api/worldgen/util/NoiseRouterTarget.java)
    pub mod noise_router_target {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // NoiseRouterTarget.NoiseRouterTarget(2): not translated, its class isn't translated (its constants carry values).
        // NoiseRouterTarget.getDensityFunction(1): not translated, its class isn't translated (its constants carry values).
        // NoiseRouterTarget.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `ConfigState` (dev/worldgen/lithostitched/impl/config/ConfigState.java)
    pub mod config_state {
        use super::*;
        // The class itself isn't translated: it implements `ApollibCopyable` (dev.worldgen.apollib.config). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // DEFAULT: not translated, it uses ConfigState, which isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.ConfigState(3): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
        // ConfigState.copy(0): not translated, its class isn't translated (it implements `ApollibCopyable` (dev.worldgen.apollib.config)).
    }

    /// `LithostitchedDebugging` (dev/worldgen/lithostitched/impl/debug/LithostitchedDebugging.java)
    pub mod lithostitched_debugging {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LithostitchedDebugging {
        }
        impl LithostitchedDebugging {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LithostitchedDebugging.init(0): not translated, unknown static AddWorldgenModifiersEvent.EVENT.
        pub fn id(name: &str) -> JIdentifier {
            return JIdentifier::new("test", &name);
        }
    }

    /// `AttributeLayerBuilderDuck` (dev/worldgen/lithostitched/impl/duck/AttributeLayerBuilderDuck.java)
    pub mod attribute_layer_builder_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AttributeLayerBuilderDuck.addPositionalTimeBasedLayer(2): not translated, it has no body.
    }

    /// `AttributeMapDuck` (dev/worldgen/lithostitched/impl/duck/AttributeMapDuck.java)
    pub mod attribute_map_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // AttributeMapDuck.clear(0): not translated, it has no body.
    }

    /// `BiomeTimelineDuck` (dev/worldgen/lithostitched/impl/duck/BiomeTimelineDuck.java)
    pub mod biome_timeline_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomeTimelineDuck.lithostitched$addTimeline(1): not translated, it has no body.
        // BiomeTimelineDuck.lithostitched$setTimelines(1): not translated, it has no body.
        // BiomeTimelineDuck.lithostitched$getTimelines(0): not translated, it has no body.
        // BiomeTimelineDuck.cast(1): not translated, it uses BiomeTimelineDuck, which isn't translated (it's an interface).
    }

    /// `BiomeWeightingDuck` (dev/worldgen/lithostitched/impl/duck/BiomeWeightingDuck.java)
    pub mod biome_weighting_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // BiomeWeightingDuck.lithostitched$clear(0): not translated, it has no body.
        // BiomeWeightingDuck.lithostitched$accumulate(2): not translated, it has no body.
        // BiomeWeightingDuck.lithostitched$getWeights(0): not translated, it has no body.
    }

    /// `MansionRoomDuck` (dev/worldgen/lithostitched/impl/duck/MansionRoomDuck.java)
    pub mod mansion_room_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MansionRoomDuck.lithostitched$floorNumber(0): not translated, it has no body.
        // MansionRoomDuck.lithostitched$getRandom(2): not translated, its class isn't translated (it's an interface).
    }

    /// `MaterialSystemAccessor` (dev/worldgen/lithostitched/impl/duck/MaterialSystemAccessor.java)
    pub mod material_system_accessor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MaterialSystemAccessor.getSystem(0): not translated, it has no body.
    }

    /// `MNBSDuck` (dev/worldgen/lithostitched/impl/duck/mnbs/MNBSDuck.java)
    pub mod mnbs_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MNBSDuck.lithostitched$getEntries(0): not translated, it has no body.
        // MNBSDuck.lithostitched$setEntries(1): not translated, it has no body.
    }

    /// `MNBSPLDuck` (dev/worldgen/lithostitched/impl/duck/mnbs/MNBSPLDuck.java)
    pub mod mnbspl_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MNBSPLDuck.lithostitched$setParameters(1): not translated, it has no body.
        // MNBSPLDuck.lithostitched$setMigrationBiome(1): not translated, it has no body.
        // MNBSPLDuck.lithostitched$getMigrationBiome(0): not translated, it has no body.
    }

    /// `ReferencePosDuck` (dev/worldgen/lithostitched/impl/duck/ReferencePosDuck.java)
    pub mod reference_pos_duck {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ReferencePosDuck.lithostitched$setReferencePos(1): not translated, it has no body.
        // ReferencePosDuck.lithostitched$getReferencePos(0): not translated, it has no body.
    }

    /// `RegistryHolder` (dev/worldgen/lithostitched/impl/duck/RegistryHolder.java)
    pub mod registry_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RegistryHolder.getRegistries(0): not translated, it has no body.
        // RegistryHolder.setRegistries(1): not translated, it has no body.
    }

    /// `SeedAccessor` (dev/worldgen/lithostitched/impl/duck/SeedAccessor.java)
    pub mod seed_accessor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SeedAccessor.getSeed(0): not translated, it has no body.
    }

    /// `StructureAttributesHolder` (dev/worldgen/lithostitched/impl/duck/StructureAttributesHolder.java)
    pub mod structure_attributes_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // StructureAttributesHolder.getPreviousStructureAttributes(0): not translated, it has no body.
        // StructureAttributesHolder.getStructureAttributes(0): not translated, it has no body.
        // StructureAttributesHolder.updateStructureAttributes(1): not translated, it has no body.
        // StructureAttributesHolder.getTicksSinceUpdated(0): not translated, it has no body.
        // StructureAttributesHolder.incrementTicksSinceUpdated(0): not translated, it has no body.
        // StructureAttributesHolder.shouldFadeMusic(0): not translated, its class isn't translated (it's an interface).
        // StructureAttributesHolder.from(1): not translated, it uses StructureAttributesHolder, which isn't translated (it's an interface).
    }

    /// `StructurePoolAccess` (dev/worldgen/lithostitched/impl/duck/StructurePoolAccess.java)
    pub mod structure_pool_access {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // StructurePoolAccess.getLithostitchedTemplates(0): not translated, it has no body.
        // StructurePoolAccess.compileRawTemplates(0): not translated, it has no body.
    }

    /// `SurfaceSystemAccessor` (dev/worldgen/lithostitched/impl/duck/SurfaceSystemAccessor.java)
    pub mod surface_system_accessor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SurfaceSystemAccessor.getBandOffsetNoise(0): not translated, it has no body.
        // SurfaceSystemAccessor.getNoiseRandom(0): not translated, it has no body.
    }

    /// `LithostitchedEvent` (dev/worldgen/lithostitched/impl/event/LithostitchedEvent.java)
    pub mod lithostitched_event {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // LithostitchedEvent.LithostitchedEvent(1): not translated, its class isn't translated (it's generic).
        // LithostitchedEvent.register(1): not translated, its class isn't translated (it's generic).
        // LithostitchedEvent.invoker(0): not translated, its class isn't translated (it's generic).
    }

    /// `Lithostitched` (dev/worldgen/lithostitched/impl/Lithostitched.java)
    pub mod lithostitched {
        use super::*;
        pub const MOD_ID: &str = "lithostitched";
        // LOGGER: the logger. Log calls become comments.
        // CONFIG: not translated, it uses `ApollibConfigHolder` (dev.worldgen.apollib.config).
        // REGISTRAR: not translated, it uses `ApollibRegistrar` (dev.worldgen.apollib.registry).
        #[derive(Clone, Debug)]
        pub struct Lithostitched {
        }
        impl Lithostitched {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Lithostitched.init(0): not translated, it uses Lithostitched.CONFIG, which isn't translated (it uses `ApollibConfigHolder` (dev.worldgen.apollib.config)).
        // Lithostitched.key(2): not translated, it's generic.
        pub fn id(name: &str) -> JIdentifier {
            return JIdentifier::new("lithostitched", &name);
        }
        pub fn vanilla_to_lithostitched(id: &JIdentifier) -> JIdentifier {
            return if j_eq(&id.namespace.clone(), "minecraft") { lithostitched::id(&id.path.clone()) } else { id.clone() };
        }
        // Lithostitched.registry(2): not translated, it's generic.
        // Lithostitched.breaksSeedParity(0): not translated, it uses Lithostitched.CONFIG, which isn't translated (it uses `ApollibConfigHolder` (dev.worldgen.apollib.config)).
        // Lithostitched.debug(2): not translated, it uses `Object`.
    }

    /// `LithostitchedCodecs` (dev/worldgen/lithostitched/impl/LithostitchedCodecs.java)
    pub mod lithostitched_codecs {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // LithostitchedCodecs.holderTarget(2): not translated, it's generic.
        // LithostitchedCodecs.compactWeightedList(2): not translated, it's generic.
    }

    /// `LithostitchedInternalHooks` (dev/worldgen/lithostitched/impl/LithostitchedInternalHooks.java)
    pub mod lithostitched_internal_hooks {
        use super::*;
        pub static WORLD_SEED: JCell<i64> = JCell::new(0i64);
        #[derive(Clone, Debug)]
        pub struct LithostitchedInternalHooks {
        }
        impl LithostitchedInternalHooks {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LithostitchedInternalHooks.onServerAboutToStart(1): not translated, it uses Minecraft's `MinecraftServer`.
        // LithostitchedInternalHooks.applyModifiersAndInjections(4): not translated, it uses Minecraft's `MinecraftServer`.
    }

    /// `ApplyStructureAttributesPacket` (dev/worldgen/lithostitched/impl/network/ApplyStructureAttributesPacket.java)
    pub mod apply_structure_attributes_packet {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `CustomPacketPayload`. Its static parts are below.
        // TYPE: not translated, it uses Type, which isn't translated (it's an interface).
        // CODEC: not translated, it uses Minecraft's `StreamCodec`.
        // ApplyStructureAttributesPacket.ApplyStructureAttributesPacket(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ApplyStructureAttributesPacket.createEmpty(0): not translated, it uses ApplyStructureAttributesPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ApplyStructureAttributesPacket.create(2): not translated, it uses ApplyStructureAttributesPacket, which isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ApplyStructureAttributesPacket.getAttributes(1): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ApplyStructureAttributesPacket.type(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
        // ApplyStructureAttributesPacket.attributes(0): not translated, its class isn't translated (it implements Minecraft's `CustomPacketPayload`).
    }

    /// `LithostitchedFabric` (dev/worldgen/lithostitched/impl/platform/fabric/LithostitchedFabric.java)
    pub mod lithostitched_fabric {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ModInitializer`. Its static parts are below.
        // LithostitchedFabric.onInitialize(0): not translated, its class isn't translated (it implements Fabric's `ModInitializer`).
    }

    /// `LithostitchedFabricClient` (dev/worldgen/lithostitched/impl/platform/fabric/LithostitchedFabricClient.java)
    pub mod lithostitched_fabric_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // LithostitchedFabricClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `LithostitchedPlatform` (dev/worldgen/lithostitched/impl/platform/LithostitchedPlatform.java)
    pub mod lithostitched_platform {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LithostitchedPlatform {
        }
        impl LithostitchedPlatform {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn is_mod_loaded(id: &str) -> bool {
            return false;
        }
        // LithostitchedPlatform.rebuildSettings(2): not translated, it uses Minecraft's `Biome`.
        // LithostitchedPlatform.memoize(1): not translated, it's generic.
    }

    /// `AllOfPredicate` (dev/worldgen/lithostitched/impl/predicate/AllOfPredicate.java)
    pub mod all_of_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AllOfPredicate.AllOfPredicate(1): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // AllOfPredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // AllOfPredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // AllOfPredicate.predicates(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `AnyOfPredicate` (dev/worldgen/lithostitched/impl/predicate/AnyOfPredicate.java)
    pub mod any_of_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AnyOfPredicate.AnyOfPredicate(1): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // AnyOfPredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // AnyOfPredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // AnyOfPredicate.predicates(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `LoaderPredicate` (dev/worldgen/lithostitched/impl/predicate/LoaderPredicate.java)
    pub mod loader_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        pub const CURRENT_LOADER: &str = "fabric";
        // LoaderPredicate.LoaderPredicate(1): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // LoaderPredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // LoaderPredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // LoaderPredicate.loader(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `ModLoadedPredicate` (dev/worldgen/lithostitched/impl/predicate/ModLoadedPredicate.java)
    pub mod mod_loaded_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ModLoadedPredicate.ModLoadedPredicate(1): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // ModLoadedPredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // ModLoadedPredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // ModLoadedPredicate.modId(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `NotPredicate` (dev/worldgen/lithostitched/impl/predicate/NotPredicate.java)
    pub mod not_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NotPredicate.NotPredicate(1): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // NotPredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // NotPredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // NotPredicate.predicate(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `PackFormatPredicate` (dev/worldgen/lithostitched/impl/predicate/PackFormatPredicate.java)
    pub mod pack_format_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // PackFormatPredicate.PackFormatPredicate(1): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // PackFormatPredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // PackFormatPredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // PackFormatPredicate.supportedFormats(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `StubException` (dev/worldgen/lithostitched/impl/predicate/StubException.java)
    pub mod stub_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
    }

    /// `TruePredicate` (dev/worldgen/lithostitched/impl/predicate/TruePredicate.java)
    pub mod true_predicate {
        use super::*;
        // The class itself isn't translated: it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TruePredicate.TruePredicate(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // TruePredicate.test(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
        // TruePredicate.codec(0): not translated, its class isn't translated (it implements `LoadPredicate` (dev.worldgen.lithostitched.api.predicate)).
    }

    /// `LithostitchedRegistrar` (dev/worldgen/lithostitched/impl/registry/LithostitchedRegistrar.java)
    pub mod lithostitched_registrar {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct LithostitchedRegistrar {
        }
        impl LithostitchedRegistrar {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // LithostitchedRegistrar.registerRegistry(2): not translated, it's generic.
        // LithostitchedRegistrar.register(2): not translated, it's generic.
    }

    /// `CodecExtender` (dev/worldgen/lithostitched/impl/util/CodecExtender.java)
    pub mod codec_extender {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct CodecExtender {
        }
        impl CodecExtender {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // CodecExtender.extend(2): not translated, it's generic.
        // CodecExtender.extend(2): not translated, it's generic.
    }

    /// `PositionalTimeBasedLayer` (dev/worldgen/lithostitched/impl/worldgen/attribute/PositionalTimeBasedLayer.java)
    pub mod positional_time_based_layer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // PositionalTimeBasedLayer.applyPositionalTimeBased(4): not translated, it has no body.
        // PositionalTimeBasedLayer.applyPositional(3): not translated, its class isn't translated (it's an interface).
    }

    /// `Bandlands` (dev/worldgen/lithostitched/impl/worldgen/bandlands/Bandlands.java)
    pub mod bandlands {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // Bandlands.Bandlands(2): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Bandlands.base(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Bandlands.bands(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Bandlands.getBand(4): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // Bandlands.fillBands(1): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `BandlandsBuilderImpl` (dev/worldgen/lithostitched/impl/worldgen/bandlands/BandlandsBuilderImpl.java)
    pub mod bandlands_builder_impl {
        use super::*;
        // The class itself isn't translated: it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands). Its static parts are below.
        // BandlandsBuilderImpl.BandlandsBuilderImpl(1): not translated, its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BandlandsBuilderImpl.baseBand(3): not translated, its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BandlandsBuilderImpl.repeatBand(3): not translated, its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BandlandsBuilderImpl.wrappedBand(5): not translated, its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BandlandsBuilderImpl.build(0): not translated, its class isn't translated (it implements `BandlandsBuilder` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
    }

    /// `BaseBand` (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/BaseBand.java)
    pub mod base_band {
        use super::*;
        // The class itself isn't translated: it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BaseBand.BaseBand(3): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BaseBand.fill(2): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BaseBand.codec(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BaseBand.count(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BaseBand.size(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // BaseBand.state(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
    }

    /// `RepeatingBand` (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/RepeatingBand.java)
    pub mod repeating_band {
        use super::*;
        // The class itself isn't translated: it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RepeatingBand.RepeatingBand(3): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // RepeatingBand.fill(2): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // RepeatingBand.codec(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // RepeatingBand.interval(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // RepeatingBand.size(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // RepeatingBand.state(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
    }

    /// `WrappedBand` (dev/worldgen/lithostitched/impl/worldgen/bandlands/band/WrappedBand.java)
    pub mod wrapped_band {
        use super::*;
        // The class itself isn't translated: it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WrappedBand.WrappedBand(5): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // WrappedBand.fill(2): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // WrappedBand.codec(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // WrappedBand.interval(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // WrappedBand.maxCount(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // WrappedBand.wrapperChance(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // WrappedBand.wrapperState(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
        // WrappedBand.wrappedState(0): not translated, its class isn't translated (it implements `Band` (dev.worldgen.lithostitched.api.worldgen.bandlands)).
    }

    /// `AddPoints` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/AddPoints.java)
    pub mod add_points {
        use super::*;
        // The class itself isn't translated: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddPoints.AddPoints(4): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // AddPoints.apply(2): not translated, it uses Minecraft's `Pair`.
        // AddPoints.possibleBiomes(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // AddPoints.codec(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // AddPoints.predicate(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // AddPoints.dimension(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // AddPoints.priority(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // AddPoints.points(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
    }

    /// `DispatchAlternateLayout` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/DispatchAlternateLayout.java)
    pub mod dispatch_alternate_layout {
        use super::*;
        // The class itself isn't translated: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DispatchAlternateLayout.DispatchAlternateLayout(5): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.matches(6): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.possibleBiomes(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.codec(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.predicate(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.dimension(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.priority(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.parameters(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // DispatchAlternateLayout.points(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
    }

    /// `ForcePlacement` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ForcePlacement.java)
    pub mod force_placement {
        use super::*;
        // The class itself isn't translated: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ForcePlacement.ForcePlacement(5): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.matches(6): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.possibleBiomes(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.codec(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.predicate(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.dimension(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.priority(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.biome(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ForcePlacement.parameters(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
    }

    /// `BiomeInjectorManager` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/BiomeInjectorManager.java)
    pub mod biome_injector_manager {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BiomeInjectorManager {
        }
        impl BiomeInjectorManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BiomeInjectorManager.applyBiomeInjectors(4): not translated, it uses Minecraft's `MinecraftServer`.
        // BiomeInjectorManager.createRegionId(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `InjectorBiomeSource` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/InjectorBiomeSource.java)
    pub mod injector_biome_source {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `BiomeSource`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // InjectorBiomeSource.InjectorBiomeSource(1): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.rootDelegate(0): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.directDelegate(0): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.applyInjectors(4): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.updateParameters(2): not translated, it uses Minecraft's `ParameterList`.
        // InjectorBiomeSource.codec(0): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.collectPossibleBiomes(0): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.createResolver(1): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.getNoiseBiome(6): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.addDebugInfo(3): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.getRegionLine(3): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
        // InjectorBiomeSource.getRootSource(1): not translated, it uses Minecraft's `BiomeSource`.
        // InjectorBiomeSource.instanceOfBlueprintSource(1): not translated, it uses Minecraft's `BiomeSource`.
        // InjectorBiomeSource.clone(0): not translated, its class isn't translated (it extends Minecraft's `BiomeSource`).
    }

    /// `ParameterBuilderImpl` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterBuilderImpl.java)
    pub mod parameter_builder_impl {
        use super::*;
        // The class itself isn't translated: it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector). Its static parts are below.
        // ParameterBuilderImpl.densityFunctionExactly(2): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.densityFunctionMin(2): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.densityFunctionMax(2): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.densityFunctionRange(3): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.climateExactly(2): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.climateMin(2): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.climateMax(2): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.climateRange(3): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.region(1): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ParameterBuilderImpl.build(0): not translated, its class isn't translated (it implements `ParameterBuilder` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
    }

    /// `ParameterMap` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/internal/ParameterMap.java)
    pub mod parameter_map {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Either`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ParameterMap.ParameterMap(2): not translated, its class isn't translated (it uses Minecraft's `Either`).
        // ParameterMap.matches(6): not translated, its class isn't translated (it uses Minecraft's `Either`).
    }

    /// `Region` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/Region.java)
    pub mod region {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // KEY_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // Region.Region(4): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Region.create(3): not translated, it uses Region, which isn't translated (it uses Minecraft's `ResourceKey`).
        // Region.create(4): not translated, it uses Region, which isn't translated (it uses Minecraft's `ResourceKey`).
        // Region.name(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Region.dimension(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Region.biomes(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // Region.weight(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `RegionManager` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java)
    pub mod region_manager {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DensityFunction`. Its static parts are below.
        // NO_REGIONS: not translated, it uses Minecraft's `ResourceKey`.
        // NO_REGIONS_IN_RANGE: not translated, it uses Minecraft's `ResourceKey`.
        // RegionManager.RegionManager(3): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // RegionManager.getRegion(5): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // RegionManager.getRegionValue(5): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // RegionManager.error(1): not translated, it uses Minecraft's `ResourceKey`.
    }

    /// `RegionManager.BiomeRegions` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/region/RegionManager.java)
    pub mod region_manager_biome_regions {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceKey`. Its static parts are below.
        // RegionManager.BiomeRegions.BiomeRegions(2): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // RegionManager.BiomeRegions.getRegionValue(5): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // RegionManager.BiomeRegions.getRegion(5): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // RegionManager.BiomeRegions.regionsByOutputs(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
        // RegionManager.BiomeRegions.totalWeight(0): not translated, its class isn't translated (it uses Minecraft's `ResourceKey`).
    }

    /// `ReplaceFully` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplaceFully.java)
    pub mod replace_fully {
        use super::*;
        // The class itself isn't translated: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ReplaceFully.ReplaceFully(5): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplaceFully.possibleBiomes(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplaceFully.codec(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplaceFully.predicate(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplaceFully.dimension(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplaceFully.priority(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplaceFully.targets(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplaceFully.replacement(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
    }

    /// `ReplacePartially` (dev/worldgen/lithostitched/impl/worldgen/biomeinjector/ReplacePartially.java)
    pub mod replace_partially {
        use super::*;
        // The class itself isn't translated: it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ReplacePartially.ReplacePartially(6): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.matches(7): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.possibleBiomes(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.codec(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.predicate(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.dimension(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.priority(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.targets(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.replacement(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
        // ReplacePartially.parameters(0): not translated, its class isn't translated (it implements `BiomeInjector` (dev.worldgen.lithostitched.api.worldgen.biomeinjector)).
    }

    /// `ApplyAll` (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyAll.java)
    pub mod apply_all {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RuleBlockEntityModifier`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `RuleBlockEntityModifierType`.
        // ApplyAll.ApplyAll(1): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
        // ApplyAll.apply(2): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
        // ApplyAll.getType(0): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
        // ApplyAll.modifiers(0): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
    }

    /// `ApplyRandom` (dev/worldgen/lithostitched/impl/worldgen/blockentitymodifier/ApplyRandom.java)
    pub mod apply_random {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `RuleBlockEntityModifier`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `RuleBlockEntityModifierType`.
        // ApplyRandom.ApplyRandom(1): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
        // ApplyRandom.apply(2): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
        // ApplyRandom.getType(0): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
        // ApplyRandom.modifiers(0): not translated, its class isn't translated (it implements Minecraft's `RuleBlockEntityModifier`).
    }

    /// `BlockStatePredicate` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/BlockStatePredicate.java)
    pub mod block_state_predicate {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StateTestingPredicate`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `BlockPredicateType`.
        // BlockStatePredicate.BlockStatePredicate(2): not translated, its class isn't translated (it extends Minecraft's `StateTestingPredicate`).
        // BlockStatePredicate.properties(0): not translated, its class isn't translated (it extends Minecraft's `StateTestingPredicate`).
        // BlockStatePredicate.test(1): not translated, its class isn't translated (it extends Minecraft's `StateTestingPredicate`).
        // BlockStatePredicate.type(0): not translated, its class isn't translated (it extends Minecraft's `StateTestingPredicate`).
    }

    /// `GridPredicate` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/GridPredicate.java)
    pub mod grid_predicate {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockPredicate`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `BlockPredicateType`.
        // GridPredicate.GridPredicate(4): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // GridPredicate.test(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // GridPredicate.type(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // GridPredicate.radius(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // GridPredicate.distBetweenPoints(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // GridPredicate.predicate(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // GridPredicate.allowedCount(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
    }

    /// `HolderHolderPredicate` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/HolderHolderPredicate.java)
    pub mod holder_holder_predicate {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockPredicate`. Its static parts are below.
        // HolderHolderPredicate.HolderHolderPredicate(1): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // HolderHolderPredicate.type(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // HolderHolderPredicate.test(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // HolderHolderPredicate.predicate(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
    }

    /// `InStructurePredicate` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java)
    pub mod in_structure_predicate {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockPredicate`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `BlockPredicateType`.
        // InStructurePredicate.InStructurePredicate(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // InStructurePredicate.type(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // InStructurePredicate.test(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // InStructurePredicate.structure(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // InStructurePredicate.searchRange(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
    }

    /// `InStructurePredicate.SearchRange` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/InStructurePredicate.java)
    pub mod in_structure_predicate_search_range {
        use super::*;
        // BASE_CODEC: not translated, it uses Minecraft's `Codec`.
        // FULL_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct SearchRange {
            pub horizontal: i32,
            pub vertical: i32,
        }
        impl SearchRange {
            pub fn new_2(horizontal: i32, vertical: i32) -> Self {
                let mut s = Self { horizontal: horizontal, vertical: vertical };
                s.horizontal = horizontal;
                s.vertical = vertical;
                s
            }
            pub fn new_1(value: i32) -> Self {
                let mut s = Self::new_2(value, value);
                s
            }
            // InStructurePredicate.SearchRange.box(0): not translated, it uses Minecraft's `BoundingBox`.
            pub fn horizontal(&self) -> i32 {
                return self.horizontal;
            }
            pub fn vertical(&self) -> i32 {
                return self.vertical;
            }
        }
    }

    /// `MultipleOfPredicate` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/MultipleOfPredicate.java)
    pub mod multiple_of_predicate {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockPredicate`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `BlockPredicateType`.
        // MultipleOfPredicate.MultipleOfPredicate(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // MultipleOfPredicate.test(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // MultipleOfPredicate.type(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // MultipleOfPredicate.predicates(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // MultipleOfPredicate.allowedCount(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
    }

    /// `OffsetPredicate` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/OffsetPredicate.java)
    pub mod offset_predicate {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockPredicate`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `BlockPredicateType`.
        // OffsetPredicate.OffsetPredicate(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // OffsetPredicate.test(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // OffsetPredicate.type(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // OffsetPredicate.predicate(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // OffsetPredicate.offset(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
    }

    /// `RandomChancePredicate` (dev/worldgen/lithostitched/impl/worldgen/blockpredicate/RandomChancePredicate.java)
    pub mod random_chance_predicate {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockPredicate`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `BlockPredicateType`.
        // RandomChancePredicate.RandomChancePredicate(1): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // RandomChancePredicate.test(2): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // RandomChancePredicate.type(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
        // RandomChancePredicate.chance(0): not translated, its class isn't translated (it implements Minecraft's `BlockPredicate`).
    }

    /// `CellularDensityFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java)
    pub mod cellular_density_function {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CellularDensityFunction.CellularDensityFunction(5): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.isCellPositionInvalid(3): not translated, it uses Minecraft's `SamplerContext`.
        // CellularDensityFunction.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.condition(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.gridDimensions(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.jitter(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.returnType(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CellularDensityFunction.salt(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `CellularDensityFunction.CellCondition` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java)
    pub mod cellular_density_function_cell_condition {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // CellularDensityFunction.CellCondition.CellCondition(3): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // CellularDensityFunction.CellCondition.rewriteChildren(1): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // CellularDensityFunction.CellCondition.compileSampler(1): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // CellularDensityFunction.CellCondition.function(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // CellularDensityFunction.CellCondition.range(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
        // CellularDensityFunction.CellCondition.fallbackValue(0): not translated, its class isn't translated (it uses Minecraft's `DensityFunction`).
    }

    /// `CellularDensityFunction.CellCondition.Sampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java)
    pub mod cellular_density_function_cell_condition_sampler {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `DensitySampler`. Its static parts are below.
        // CellularDensityFunction.CellCondition.Sampler.Sampler(3): not translated, its class isn't translated (it uses Minecraft's `DensitySampler`).
        // CellularDensityFunction.CellCondition.Sampler.function(0): not translated, its class isn't translated (it uses Minecraft's `DensitySampler`).
        // CellularDensityFunction.CellCondition.Sampler.range(0): not translated, its class isn't translated (it uses Minecraft's `DensitySampler`).
        // CellularDensityFunction.CellCondition.Sampler.fallbackValue(0): not translated, its class isn't translated (it uses Minecraft's `DensitySampler`).
    }

    /// `CellularDensityFunction.GridDimensions` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java)
    pub mod cellular_density_function_grid_dimensions {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct GridDimensions {
            pub horizontal: i32,
            pub vertical: i32,
        }
        impl GridDimensions {
            pub fn new(horizontal: i32, vertical: i32) -> Self {
                let mut s = Self { horizontal: horizontal, vertical: vertical };
                s.horizontal = horizontal;
                s.vertical = vertical;
                s
            }
            pub fn is2d(&self) -> bool {
                return self.vertical == 0;
            }
            pub fn horizontal(&self) -> i32 {
                return self.horizontal;
            }
            pub fn vertical(&self) -> i32 {
                return self.vertical;
            }
        }
    }

    /// `CellularDensityFunction.XYZSampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java)
    pub mod cellular_density_function_xyz_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // CellularDensityFunction.XYZSampler.XYZSampler(5): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XYZSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XYZSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XYZSampler.condition(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XYZSampler.gridSize(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XYZSampler.jitter(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XYZSampler.returnType(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XYZSampler.salt(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `CellularDensityFunction.XZSampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CellularDensityFunction.java)
    pub mod cellular_density_function_xz_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // CellularDensityFunction.XZSampler.XZSampler(5): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.toGridPos(1): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.condition(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.gridWidth(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.jitter(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.returnType(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CellularDensityFunction.XZSampler.salt(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `CosFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java)
    pub mod cos_function {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CosFunction.CosFunction(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CosFunction.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CosFunction.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CosFunction.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CosFunction.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CosFunction.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // CosFunction.input(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `CosFunction.CosSampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/CosFunction.java)
    pub mod cos_function_cos_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // CosFunction.CosSampler.CosSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CosFunction.CosSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CosFunction.CosSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // CosFunction.CosSampler.input(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `FastNoiseFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java)
    pub mod fast_noise_function {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // FastNoiseFunction.FastNoiseFunction(6): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.config(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.xzScale(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.yScale(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.shiftY(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // FastNoiseFunction.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `FastNoiseFunction.Sampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java)
    pub mod fast_noise_function_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // FastNoiseFunction.Sampler.Sampler(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.Sampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.Sampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.Sampler.noise(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.Sampler.xzScale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.Sampler.yScale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `FastNoiseFunction.ShiftedXyzSampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java)
    pub mod fast_noise_function_shifted_xyz_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // FastNoiseFunction.ShiftedXyzSampler.ShiftedXyzSampler(6): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.shiftY(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.noise(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.xzScale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXyzSampler.yScale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `FastNoiseFunction.ShiftedXzSampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/FastNoiseFunction.java)
    pub mod fast_noise_function_shifted_xz_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // FastNoiseFunction.ShiftedXzSampler.ShiftedXzSampler(5): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXzSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXzSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXzSampler.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXzSampler.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXzSampler.noise(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXzSampler.xzScale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // FastNoiseFunction.ShiftedXzSampler.yScale(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `MarkerFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MarkerFunction.java)
    pub mod marker_function {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MarkerFunction.compileSampler(1): not translated, its class isn't translated (it's an interface).
        // MarkerFunction.rewriteChildren(1): not translated, its class isn't translated (it's an interface).
        // MarkerFunction.range(0): not translated, its class isn't translated (it's an interface).
        // MarkerFunction.domainAxes(0): not translated, its class isn't translated (it's an interface).
    }

    /// `MergedFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/MergedFunction.java)
    pub mod merged_function {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MergedFunction.MergedFunction(3): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.unwrappedOriginal(1): not translated, it uses Minecraft's `DensityFunction`.
        // MergedFunction.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.original(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.wrapped(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // MergedFunction.full(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `OriginalMarkerFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/OriginalMarkerFunction.java)
    pub mod original_marker_function {
        use super::*;
        // The class itself isn't translated: it implements `MarkerFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // OriginalMarkerFunction.codec(0): not translated, its class isn't translated (it implements `MarkerFunction`).
    }

    /// `WrappedMarkerFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/marker/WrappedMarkerFunction.java)
    pub mod wrapped_marker_function {
        use super::*;
        // The class itself isn't translated: it implements `MarkerFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WrappedMarkerFunction.codec(0): not translated, its class isn't translated (it implements `MarkerFunction`).
    }

    /// `ShiftFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java)
    pub mod shift_function {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ShiftFunction.ShiftFunction(4): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.input(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.shiftY(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // ShiftFunction.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `ShiftFunction.Sampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/ShiftFunction.java)
    pub mod shift_function_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // ShiftFunction.Sampler.Sampler(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ShiftFunction.Sampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ShiftFunction.Sampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ShiftFunction.Sampler.input(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ShiftFunction.Sampler.shiftX(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ShiftFunction.Sampler.shiftY(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // ShiftFunction.Sampler.shiftZ(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `SinFunction` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java)
    pub mod sin_function {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensityFunction`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SinFunction.SinFunction(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // SinFunction.compileSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // SinFunction.rewriteChildren(1): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // SinFunction.range(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // SinFunction.domainAxes(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // SinFunction.codec(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
        // SinFunction.input(0): not translated, its class isn't translated (it implements Minecraft's `DensityFunction`).
    }

    /// `SinFunction.SinSampler` (dev/worldgen/lithostitched/impl/worldgen/densityfunction/SinFunction.java)
    pub mod sin_function_sin_sampler {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DensitySampler`. Its static parts are below.
        // SinFunction.SinSampler.SinSampler(1): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // SinFunction.SinSampler.sampleVolume(3): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // SinFunction.SinSampler.sampleValue(4): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
        // SinFunction.SinSampler.input(0): not translated, its class isn't translated (it implements Minecraft's `DensitySampler`).
    }

    /// `CellularNoiseType` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java)
    pub mod cellular_noise_type {
        use super::*;
        // The class itself isn't translated: it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CellularNoiseType.CellularNoiseType(5): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
        // CellularNoiseType.distanceFunction(0): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
        // CellularNoiseType.returnType(0): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
        // CellularNoiseType.jitter(0): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
        // CellularNoiseType.getCodec(0): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
    }

    /// `CellularNoiseType.DistanceFunction` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java)
    pub mod cellular_noise_type_distance_function {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CellularNoiseType.DistanceFunction.DistanceFunction(2): not translated, its class isn't translated (its constants carry values).
        // CellularNoiseType.DistanceFunction.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `CellularNoiseType.ReturnType` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/CellularNoiseType.java)
    pub mod cellular_noise_type_return_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CellularNoiseType.ReturnType.ReturnType(2): not translated, its class isn't translated (its constants carry values).
        // CellularNoiseType.ReturnType.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `PerlinNoiseType` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/PerlinNoiseType.java)
    pub mod perlin_noise_type {
        use super::*;
        // The class itself isn't translated: it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // PerlinNoiseType.PerlinNoiseType(2): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
        // PerlinNoiseType.getCodec(0): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
    }

    /// `SimplexNoiseType` (dev/worldgen/lithostitched/impl/worldgen/fastnoise/SimplexNoiseType.java)
    pub mod simplex_noise_type {
        use super::*;
        // The class itself isn't translated: it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SimplexNoiseType.SimplexNoiseType(6): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
        // SimplexNoiseType.getCodec(0): not translated, its class isn't translated (it extends `FastNoiseConfig` (dev.worldgen.lithostitched.api.worldgen.densityfunction.fastnoise)).
    }

    /// `CompositeFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java)
    pub mod composite_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CompositeFeature.CompositeFeature(2): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CompositeFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CompositeFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CompositeFeature.features(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // CompositeFeature.placementType(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `CompositeFeature.Type` (dev/worldgen/lithostitched/impl/worldgen/feature/CompositeFeature.java)
    pub mod composite_feature_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // CompositeFeature.Type.Type(2): not translated, its class isn't translated (its constants carry values).
        // CompositeFeature.Type.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
        // CompositeFeature.Type.shouldContinue(1): not translated, its class isn't translated (its constants carry values).
    }

    /// `DungeonFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/DungeonFeature.java)
    pub mod dungeon_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // DEFAULT_MOBS: not translated, it uses Minecraft's `WeightedList`.
        // DEFAULT_FLOOR: not translated, it uses Minecraft's `WeightedList`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DungeonFeature.DungeonFeature(9): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.minOpenings(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.maxOpenings(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.radius(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.maxChests(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.spawnerMobs(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.floorProvider(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.wallProvider(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.dungeonInvalidBlocks(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // DungeonFeature.lootTable(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `LargeDripstoneFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java)
    pub mod large_dripstone_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // LargeDripstoneFeature.LargeDripstoneFeature(11): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.makeDripstone(7): not translated, it uses Minecraft's `BlockStateProvider`.
        // LargeDripstoneFeature.stateProvider(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.replaceableBlocks(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.floorToCeilingSearchRange(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.columnRadius(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.heightScale(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.maxColumnRadiusToCaveHeightRatio(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.stalactiteBluntness(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.stalagmiteBluntness(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.windSpeed(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.minRadiusForWind(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // LargeDripstoneFeature.minBluntnessForWind(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `LargeDripstoneFeature.LargeDripstone` (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java)
    pub mod large_dripstone_feature_large_dripstone {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockStateProvider`. Its static parts are below.
        // LargeDripstoneFeature.LargeDripstone.LargeDripstone(6): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // LargeDripstoneFeature.LargeDripstone.getHeight(0): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // LargeDripstoneFeature.LargeDripstone.moveBackUntilBaseIsInsideStoneAndShrinkRadiusIfNecessary(2): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // LargeDripstoneFeature.LargeDripstone.getHeightAtRadius(1): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // LargeDripstoneFeature.LargeDripstone.placeBlocks(3): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
        // LargeDripstoneFeature.LargeDripstone.isSuitableForWind(1): not translated, its class isn't translated (it uses Minecraft's `BlockStateProvider`).
    }

    /// `LargeDripstoneFeature.WindOffsetter` (dev/worldgen/lithostitched/impl/worldgen/feature/LargeDripstoneFeature.java)
    pub mod large_dripstone_feature_wind_offsetter {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Vec3`. Its static parts are below.
        // LargeDripstoneFeature.WindOffsetter.WindOffsetter(3): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // LargeDripstoneFeature.WindOffsetter.WindOffsetter(0): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
        // LargeDripstoneFeature.WindOffsetter.noWind(0): not translated, it uses WindOffsetter, which isn't translated (it uses Minecraft's `Vec3`).
        // LargeDripstoneFeature.WindOffsetter.offset(1): not translated, its class isn't translated (it uses Minecraft's `Vec3`).
    }

    /// `OreFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java)
    pub mod ore_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // OreFeature.OreFeature(2): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // OreFeature.create(2): not translated, it uses OreFeature, which isn't translated (it implements Minecraft's `Feature`).
        // OreFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // OreFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // OreFeature.doPlace(13): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // OreFeature.size(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // OreFeature.targets(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `OreFeature.Target` (dev/worldgen/lithostitched/impl/worldgen/feature/OreFeature.java)
    pub mod ore_feature_target {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockPredicate`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // OreFeature.Target.Target(2): not translated, its class isn't translated (it uses Minecraft's `BlockPredicate`).
        // OreFeature.Target.predicate(0): not translated, its class isn't translated (it uses Minecraft's `BlockPredicate`).
        // OreFeature.Target.stateProvider(0): not translated, its class isn't translated (it uses Minecraft's `BlockPredicate`).
    }

    /// `SelectFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/SelectFeature.java)
    pub mod select_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // PAIR_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SelectFeature.SelectFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SelectFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SelectFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SelectFeature.features(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `SimplePlacedFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/SimplePlacedFeature.java)
    pub mod simple_placed_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SimplePlacedFeature.SimplePlacedFeature(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SimplePlacedFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SimplePlacedFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // SimplePlacedFeature.feature(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `StructureTemplateFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/StructureTemplateFeature.java)
    pub mod structure_template_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // StructureTemplateFeature.StructureTemplateFeature(5): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.getRotatedOffset(3): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.template(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.processors(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.rotation(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.liquidSettings(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // StructureTemplateFeature.placeOnCenter(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `DripstoneUtils` (dev/worldgen/lithostitched/impl/worldgen/feature/util/DripstoneUtils.java)
    pub mod dripstone_utils {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DripstoneUtils {
        }
        impl DripstoneUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn get_dripstone_height(mut __0: f64, __1: f64, __2: f64, __3: f64) -> f64 {
            if __0 < __3 {
                __0 = __3;
            }
            let __5: f64 = (__0 / __1) * 0.384f64;
            let __6: f64 = 0.75f64 * (__5.powf(1.3333333333333333f64));
            let __7: f64 = __5.powf(0.6666666666666666f64);
            let __8: f64 = 0.3333333333333333f64 * __5.ln();
            let mut __9: f64 = __2 * ((__6 - __7) - __8);
            __9 = __9.max(0.0f64);
            return (__9 / 0.384f64) * __1;
        }
        // DripstoneUtils.isCircleMostlyEmbeddedInStone(3): not translated, it uses Minecraft's `WorldGenLevel`.
        // DripstoneUtils.isEmptyOrWater(2): not translated, it uses Minecraft's `LevelAccessor`.
        // DripstoneUtils.isEmptyOrWaterOrLava(2): not translated, it uses Minecraft's `LevelAccessor`.
        // DripstoneUtils.isReplaceableOrLava(2): not translated, it uses Minecraft's `BlockState`.
        // DripstoneUtils.isReplaceable(2): not translated, it uses Minecraft's `BlockState`.
        // DripstoneUtils.isEmptyOrWater(1): not translated, it uses Minecraft's `BlockState`.
        // DripstoneUtils.isEmptyOrWaterOrLava(1): not translated, it uses Minecraft's `BlockState`.
    }

    /// `VinesFeature` (dev/worldgen/lithostitched/impl/worldgen/feature/VinesFeature.java)
    pub mod vines_feature {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `Feature`. Its static parts are below.
        // DEFAULT_BLOCK: not translated, it uses Minecraft's `WeightedList`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // VinesFeature.VinesFeature(3): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // VinesFeature.canPlaceOn(1): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // VinesFeature.place(4): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // VinesFeature.codec(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // VinesFeature.blocks(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // VinesFeature.canPlaceOn(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
        // VinesFeature.maxLength(0): not translated, its class isn't translated (it implements Minecraft's `Feature`).
    }

    /// `AllOfCondition` (dev/worldgen/lithostitched/impl/worldgen/material/condition/AllOfCondition.java)
    pub mod all_of_condition {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `MaterialCondition`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AllOfCondition.AllOfCondition(1): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // AllOfCondition.codec(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // AllOfCondition.compile(1): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // AllOfCondition.conditions(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
    }

    /// `AnyOfCondition` (dev/worldgen/lithostitched/impl/worldgen/material/condition/AnyOfCondition.java)
    pub mod any_of_condition {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `MaterialCondition`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AnyOfCondition.AnyOfCondition(1): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // AnyOfCondition.codec(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // AnyOfCondition.compile(1): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // AnyOfCondition.conditions(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
    }

    /// `SampleDensityCondition` (dev/worldgen/lithostitched/impl/worldgen/material/condition/SampleDensityCondition.java)
    pub mod sample_density_condition {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `MaterialCondition`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SampleDensityCondition.SampleDensityCondition(3): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // SampleDensityCondition.compile(1): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // SampleDensityCondition.codec(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // SampleDensityCondition.function(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // SampleDensityCondition.range(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // SampleDensityCondition.prefill(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
    }

    /// `SlopeCondition` (dev/worldgen/lithostitched/impl/worldgen/material/condition/SlopeCondition.java)
    pub mod slope_condition {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `MaterialCondition`. Its static parts are below.
        // BASE_DIFFERENCE: not translated, it uses Minecraft's `InclusiveRange`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SlopeCondition.SlopeCondition(1): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // SlopeCondition.compile(1): not translated, its body couldn't be read: an anonymous class.
        // SlopeCondition.codec(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
        // SlopeCondition.threshold(0): not translated, its class isn't translated (it implements Minecraft's `MaterialCondition`).
    }

    /// `BandlandsRule` (dev/worldgen/lithostitched/impl/worldgen/material/rule/BandlandsRule.java)
    pub mod bandlands_rule {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `MaterialRule`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BandlandsRule.BandlandsRule(1): not translated, its class isn't translated (it implements Minecraft's `MaterialRule`).
        // BandlandsRule.compile(1): not translated, its class isn't translated (it implements Minecraft's `MaterialRule`).
        // BandlandsRule.codec(0): not translated, its class isn't translated (it implements Minecraft's `MaterialRule`).
        // BandlandsRule.options(0): not translated, its class isn't translated (it implements Minecraft's `MaterialRule`).
    }

    /// `AddBiomeSpawnsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddBiomeSpawnsModifier.java)
    pub mod add_biome_spawns_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddBiomeSpawnsModifier.AddBiomeSpawnsModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddBiomeSpawnsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddBiomeSpawnsModifier.applyModifier(2): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddBiomeSpawnsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddBiomeSpawnsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddBiomeSpawnsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddBiomeSpawnsModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddBiomeSpawnsModifier.biomeSpawns(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AddFeaturesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddFeaturesModifier.java)
    pub mod add_features_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddFeaturesModifier.AddFeaturesModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.shouldRecompileSortedFeatures(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.features(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddFeaturesModifier.step(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AddProcessorListProcessorsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddProcessorListProcessorsModifier.java)
    pub mod add_processor_list_processors_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddProcessorListProcessorsModifier.AddProcessorListProcessorsModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddProcessorListProcessorsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddProcessorListProcessorsModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddProcessorListProcessorsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddProcessorListProcessorsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddProcessorListProcessorsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddProcessorListProcessorsModifier.processorLists(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddProcessorListProcessorsModifier.processors(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AddSpawnCostsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddSpawnCostsModifier.java)
    pub mod add_spawn_costs_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // SPAWN_COST_CODEC: not translated, it uses Minecraft's `SimpleMapCodec`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddSpawnCostsModifier.AddSpawnCostsModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddSpawnCostsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddSpawnCostsModifier.applyModifier(2): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddSpawnCostsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddSpawnCostsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddSpawnCostsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddSpawnCostsModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddSpawnCostsModifier.spawnCosts(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AddStructureSetEntriesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureSetEntriesModifier.java)
    pub mod add_structure_set_entries_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddStructureSetEntriesModifier.AddStructureSetEntriesModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureSetEntriesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureSetEntriesModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureSetEntriesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureSetEntriesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureSetEntriesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureSetEntriesModifier.structureSets(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureSetEntriesModifier.entries(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AddStructureTemplatesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddStructureTemplatesModifier.java)
    pub mod add_structure_templates_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddStructureTemplatesModifier.AddStructureTemplatesModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureTemplatesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureTemplatesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureTemplatesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureTemplatesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureTemplatesModifier.targets(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddStructureTemplatesModifier.templates(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AddTemplatePoolElementsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/AddTemplatePoolElementsModifier.java)
    pub mod add_template_pool_elements_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AddTemplatePoolElementsModifier.AddTemplatePoolElementsModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddTemplatePoolElementsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddTemplatePoolElementsModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddTemplatePoolElementsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddTemplatePoolElementsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddTemplatePoolElementsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddTemplatePoolElementsModifier.templatePools(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // AddTemplatePoolElementsModifier.elements(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `BiomeEffectsBuilderImpl` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/BiomeEffectsBuilderImpl.java)
    pub mod biome_effects_builder_impl {
        use super::*;
        // The class itself isn't translated: it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // BiomeEffectsBuilderImpl.waterColor(1): not translated, its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // BiomeEffectsBuilderImpl.foliageColor(1): not translated, its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // BiomeEffectsBuilderImpl.dryFoliageColor(1): not translated, its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // BiomeEffectsBuilderImpl.grassColor(1): not translated, its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // BiomeEffectsBuilderImpl.grassColorModifier(1): not translated, its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // BiomeEffectsBuilderImpl.build(0): not translated, its class isn't translated (it implements `BiomeEffectsBuilder` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `CompileRawTemplatesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/CompileRawTemplatesModifier.java)
    pub mod compile_raw_templates_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // CompileRawTemplatesModifier.CompileRawTemplatesModifier(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // CompileRawTemplatesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // CompileRawTemplatesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // CompileRawTemplatesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // CompileRawTemplatesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `DensityFunctionInjectorHelper` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java)
    pub mod density_function_injector_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct DensityFunctionInjectorHelper {
        }
        impl DensityFunctionInjectorHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DensityFunctionInjectorHelper.wrap(2): not translated, it uses Minecraft's `DensityFunction`.
        // DensityFunctionInjectorHelper.isMarker(1): not translated, it uses Minecraft's `DensityFunction`.
    }

    /// `DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java)
    pub mod density_function_injector_helper_apply_wrapped_and_original_rule {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DfRewriteRule`. Its static parts are below.
        // DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.ApplyWrappedAndOriginalRule(2): not translated, its class isn't translated (it implements Minecraft's `DfRewriteRule`).
        // DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.rewrite(1): not translated, its class isn't translated (it implements Minecraft's `DfRewriteRule`).
        // DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.original(0): not translated, its class isn't translated (it implements Minecraft's `DfRewriteRule`).
        // DensityFunctionInjectorHelper.ApplyWrappedAndOriginalRule.wrapped(0): not translated, its class isn't translated (it implements Minecraft's `DfRewriteRule`).
    }

    /// `DensityFunctionInjectorHelper.ApplyWrappedRule` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/DensityFunctionInjectorHelper.java)
    pub mod density_function_injector_helper_apply_wrapped_rule {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `DfRewriteRule`. Its static parts are below.
        // DensityFunctionInjectorHelper.ApplyWrappedRule.ApplyWrappedRule(1): not translated, its class isn't translated (it implements Minecraft's `DfRewriteRule`).
        // DensityFunctionInjectorHelper.ApplyWrappedRule.rewrite(1): not translated, its class isn't translated (it implements Minecraft's `DfRewriteRule`).
        // DensityFunctionInjectorHelper.ApplyWrappedRule.wrapped(0): not translated, its class isn't translated (it implements Minecraft's `DfRewriteRule`).
    }

    /// `ModifierManager` (dev/worldgen/lithostitched/impl/worldgen/modifier/internal/ModifierManager.java)
    pub mod modifier_manager {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ModifierManager {
        }
        impl ModifierManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModifierManager.applyModifiers(2): not translated, it uses Minecraft's `RegistryAccess`.
        // ModifierManager.sortByPriority(1): not translated, it uses WorldgenModifier, which isn't translated (it's an interface).
        // ModifierManager.getAllModifiers(1): not translated, it uses WorldgenModifier, which isn't translated (it's an interface).
        // ModifierManager.getModifiersOfType(2): not translated, it's generic.
    }

    /// `NoOpModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/NoOpModifier.java)
    pub mod no_op_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NoOpModifier.NoOpModifier(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // NoOpModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // NoOpModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // NoOpModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // NoOpModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `RemoveBiomeSpawnsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveBiomeSpawnsModifier.java)
    pub mod remove_biome_spawns_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RemoveBiomeSpawnsModifier.RemoveBiomeSpawnsModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveBiomeSpawnsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveBiomeSpawnsModifier.applyModifier(2): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveBiomeSpawnsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveBiomeSpawnsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveBiomeSpawnsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveBiomeSpawnsModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveBiomeSpawnsModifier.mobs(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `RemoveFeaturesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveFeaturesModifier.java)
    pub mod remove_features_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RemoveFeaturesModifier.RemoveFeaturesModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.shouldRecompileSortedFeatures(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.features(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveFeaturesModifier.step(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `RemoveStructureSetEntriesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/RemoveStructureSetEntriesModifier.java)
    pub mod remove_structure_set_entries_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RemoveStructureSetEntriesModifier.RemoveStructureSetEntriesModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveStructureSetEntriesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveStructureSetEntriesModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveStructureSetEntriesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveStructureSetEntriesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveStructureSetEntriesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveStructureSetEntriesModifier.structureSets(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // RemoveStructureSetEntriesModifier.entries(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetBiomeAttributesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeAttributesModifier.java)
    pub mod set_biome_attributes_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetBiomeAttributesModifier.SetBiomeAttributesModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.applyModifier(2): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.attributes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeAttributesModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetBiomeClimateModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeClimateModifier.java)
    pub mod set_biome_climate_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetBiomeClimateModifier.SetBiomeClimateModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeClimateModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeClimateModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeClimateModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeClimateModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeClimateModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeClimateModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeClimateModifier.climateSettings(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetBiomeEffectsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeEffectsModifier.java)
    pub mod set_biome_effects_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetBiomeEffectsModifier.SetBiomeEffectsModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeEffectsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeEffectsModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeEffectsModifier.applyRequiredEffect(3): not translated, it's generic.
        // SetBiomeEffectsModifier.applyOptionalEffect(3): not translated, it's generic.
        // SetBiomeEffectsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeEffectsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeEffectsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeEffectsModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeEffectsModifier.effects(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetBiomeTimelineModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetBiomeTimelineModifier.java)
    pub mod set_biome_timeline_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetBiomeTimelineModifier.SetBiomeTimelineModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeTimelineModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeTimelineModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeTimelineModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeTimelineModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeTimelineModifier.biomes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeTimelineModifier.timeline(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetBiomeTimelineModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetDimensionAttributesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetDimensionAttributesModifier.java)
    pub mod set_dimension_attributes_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetDimensionAttributesModifier.SetDimensionAttributesModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetDimensionAttributesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetDimensionAttributesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetDimensionAttributesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetDimensionAttributesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetDimensionAttributesModifier.dimensionTypes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetDimensionAttributesModifier.attributes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetDimensionAttributesModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetMaterialRuleModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetMaterialRuleModifier.java)
    pub mod set_material_rule_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetMaterialRuleModifier.SetMaterialRuleModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.wrap(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.targetRules(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.materialRule(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetMaterialRuleModifier.injectionType(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetPoolAliasesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolAliasesModifier.java)
    pub mod set_pool_aliases_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetPoolAliasesModifier.SetPoolAliasesModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.validate(1): not translated, it uses Minecraft's `DataResult`.
        // SetPoolAliasesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.structures(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.poolAliases(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolAliasesModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetPoolElementProcessorsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetPoolElementProcessorsModifier.java)
    pub mod set_pool_element_processors_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetPoolElementProcessorsModifier.SetPoolElementProcessorsModifier(6): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.addProcessor(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.templatePools(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.locations(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.processorList(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetPoolElementProcessorsModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetStructureAttributesModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureAttributesModifier.java)
    pub mod set_structure_attributes_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetStructureAttributesModifier.SetStructureAttributesModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureAttributesModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureAttributesModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureAttributesModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureAttributesModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureAttributesModifier.structures(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureAttributesModifier.attributes(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureAttributesModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetStructureSpawnConditionModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetStructureSpawnConditionModifier.java)
    pub mod set_structure_spawn_condition_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetStructureSpawnConditionModifier.SetStructureSpawnConditionModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.applyModifier(2): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.structures(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.spawnCondition(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetStructureSpawnConditionModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetTimelineTracksModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTimelineTracksModifier.java)
    pub mod set_timeline_tracks_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // TRACKS_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetTimelineTracksModifier.SetTimelineTracksModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTimelineTracksModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTimelineTracksModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTimelineTracksModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTimelineTracksModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTimelineTracksModifier.timelines(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTimelineTracksModifier.tracks(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTimelineTracksModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `SetTreeDecoratorsModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/SetTreeDecoratorsModifier.java)
    pub mod set_tree_decorators_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetTreeDecoratorsModifier.SetTreeDecoratorsModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.features(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.decorators(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // SetTreeDecoratorsModifier.append(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `StackFeatureModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/StackFeatureModifier.java)
    pub mod stack_feature_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // StackFeatureModifier.StackFeatureModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.applyModifier(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.baseFeatures(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.stackedFeature(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // StackFeatureModifier.placementType(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AnyTarget` (dev/worldgen/lithostitched/impl/worldgen/modifier/target/AnyTarget.java)
    pub mod any_target {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TYPE: not translated, its value couldn't be read: an anonymous class.
        // AnyTarget.AnyTarget(1): not translated, its class isn't translated (it's generic).
        // AnyTarget.getHolders(1): not translated, its class isn't translated (it's generic).
        // AnyTarget.getType(0): not translated, its class isn't translated (it's generic).
        // AnyTarget.key(0): not translated, its class isn't translated (it's generic).
    }

    /// `ClimateTarget` (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/ClimateTarget.java)
    pub mod climate_target {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TYPE: not translated, its value couldn't be read: an anonymous class.
        // ClimateTarget.ClimateTarget(5): not translated, its class isn't translated (it's generic).
        // ClimateTarget.getHolders(1): not translated, its class isn't translated (it's generic).
        // ClimateTarget.test(1): not translated, its class isn't translated (it's generic).
        // ClimateTarget.getType(0): not translated, its class isn't translated (it's generic).
        // ClimateTarget.key(0): not translated, its class isn't translated (it's generic).
        // ClimateTarget.temperature(0): not translated, its class isn't translated (it's generic).
        // ClimateTarget.downfall(0): not translated, its class isn't translated (it's generic).
        // ClimateTarget.hasPrecipitation(0): not translated, its class isn't translated (it's generic).
        // ClimateTarget.temperatureModifier(0): not translated, its class isn't translated (it's generic).
    }

    /// `HasFeatureTarget` (dev/worldgen/lithostitched/impl/worldgen/modifier/target/biome/HasFeatureTarget.java)
    pub mod has_feature_target {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TYPE: not translated, its value couldn't be read: an anonymous class.
        // HasFeatureTarget.HasFeatureTarget(2): not translated, its class isn't translated (it's generic).
        // HasFeatureTarget.getHolders(1): not translated, its class isn't translated (it's generic).
        // HasFeatureTarget.test(1): not translated, its class isn't translated (it's generic).
        // HasFeatureTarget.getType(0): not translated, its class isn't translated (it's generic).
        // HasFeatureTarget.key(0): not translated, its class isn't translated (it's generic).
        // HasFeatureTarget.feature(0): not translated, its class isn't translated (it's generic).
    }

    /// `HolderSetTarget` (dev/worldgen/lithostitched/impl/worldgen/modifier/target/HolderSetTarget.java)
    pub mod holder_set_target {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TYPE: not translated, its value couldn't be read: an anonymous class.
        // HolderSetTarget.HolderSetTarget(2): not translated, its class isn't translated (it's generic).
        // HolderSetTarget.getHolders(1): not translated, its class isn't translated (it's generic).
        // HolderSetTarget.getType(0): not translated, its class isn't translated (it's generic).
        // HolderSetTarget.set(0): not translated, its class isn't translated (it's generic).
        // HolderSetTarget.key(0): not translated, its class isn't translated (it's generic).
    }

    /// `RegexTarget` (dev/worldgen/lithostitched/impl/worldgen/modifier/target/RegexTarget.java)
    pub mod regex_target {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TYPE: not translated, its value couldn't be read: an anonymous class.
        // RegexTarget.RegexTarget(2): not translated, its class isn't translated (it's generic).
        // RegexTarget.getHolders(1): not translated, its class isn't translated (it's generic).
        // RegexTarget.getType(0): not translated, its class isn't translated (it's generic).
        // RegexTarget.key(0): not translated, its class isn't translated (it's generic).
        // RegexTarget.regex(0): not translated, its class isn't translated (it's generic).
    }

    /// `TemplateList` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateList.java)
    pub mod template_list {
        use super::*;
        // CODEC: not translated, it uses Minecraft's `Codec`.
        #[derive(Clone, Debug, PartialEq)]
        pub struct TemplateList {
            pub templates: Vec<JIdentifier>,
        }
        impl TemplateList {
            pub fn new(templates: &[JIdentifier]) -> Self {
                let mut s = Self { templates: Vec::new() };
                s.templates = templates.to_vec();
                s
            }
            pub fn get_random(&self, random_source: JRandom) -> JIdentifier {
                return self.templates[(random_source.next_int((self.templates.len() as i32))) as usize].clone();
            }
            pub fn add_all(&mut self, templates: &[JIdentifier]) {
                self.templates.extend(templates.iter().cloned().collect::<Vec<_>>());
            }
            pub fn templates(&self) -> Vec<JIdentifier> {
                return self.templates.to_vec();
            }
        }
    }

    /// `TemplateLists` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateLists.java)
    pub mod template_lists {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TemplateLists.key(1): not translated, it uses Minecraft's `ResourceKey`.
        // TemplateLists.mansion(2): not translated, it uses Minecraft's `ResourceKey`.
        // TemplateLists.getRandom(3): not translated, it uses Minecraft's `RegistryAccess`.
    }

    /// `TemplateLists.Mansion` (dev/worldgen/lithostitched/impl/worldgen/modifier/template/TemplateLists.java)
    pub mod template_lists_mansion {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
    }

    /// `WrapAquifersModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapAquifersModifier.java)
    pub mod wrap_aquifers_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WrapAquifersModifier.WrapAquifersModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapAquifersModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapAquifersModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapAquifersModifier.modifyAquiferFunction(3): not translated, it uses Minecraft's `DensityFunction`.
        // WrapAquifersModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapAquifersModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapAquifersModifier.dimension(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapAquifersModifier.target(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapAquifersModifier.wrapperFunction(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `WrapDensityFunctionModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapDensityFunctionModifier.java)
    pub mod wrap_density_function_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WrapDensityFunctionModifier.WrapDensityFunctionModifier(4): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapDensityFunctionModifier.create(4): not translated, it uses WorldgenModifier, which isn't translated (it's an interface).
        // WrapDensityFunctionModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapDensityFunctionModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapDensityFunctionModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapDensityFunctionModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapDensityFunctionModifier.targetFunction(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapDensityFunctionModifier.wrapperFunction(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `WrapNoiseRouterModifier` (dev/worldgen/lithostitched/impl/worldgen/modifier/WrapNoiseRouterModifier.java)
    pub mod wrap_noise_router_modifier {
        use super::*;
        // The class itself isn't translated: it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WrapNoiseRouterModifier.WrapNoiseRouterModifier(5): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapNoiseRouterModifier.apply(1): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapNoiseRouterModifier.codec(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapNoiseRouterModifier.modifyRouterFunction(3): not translated, it uses Minecraft's `DensityFunction`.
        // WrapNoiseRouterModifier.predicate(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapNoiseRouterModifier.priority(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapNoiseRouterModifier.dimension(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapNoiseRouterModifier.target(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
        // WrapNoiseRouterModifier.wrapperFunction(0): not translated, its class isn't translated (it implements `WorldgenModifier` (dev.worldgen.lithostitched.api.worldgen.modifier)).
    }

    /// `AllOfPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AllOfPlacementCondition.java)
    pub mod all_of_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AllOfPlacementCondition.AllOfPlacementCondition(1): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // AllOfPlacementCondition.appendCondition(1): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // AllOfPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // AllOfPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // AllOfPlacementCondition.conditions(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `AnyOfPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/AnyOfPlacementCondition.java)
    pub mod any_of_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AnyOfPlacementCondition.AnyOfPlacementCondition(1): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // AnyOfPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // AnyOfPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // AnyOfPlacementCondition.conditions(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `GridPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/GridPlacementCondition.java)
    pub mod grid_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // GridPlacementCondition.GridPlacementCondition(4): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // GridPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // GridPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // GridPlacementCondition.radius(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // GridPlacementCondition.distBetweenPoints(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // GridPlacementCondition.condition(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // GridPlacementCondition.allowedCount(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `HeightFilterPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java)
    pub mod height_filter_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // HeightFilterPlacementCondition.HeightFilterPlacementCondition(3): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // HeightFilterPlacementCondition.validate(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // HeightFilterPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // HeightFilterPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // HeightFilterPlacementCondition.rangeType(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // HeightFilterPlacementCondition.heightmap(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // HeightFilterPlacementCondition.permittedRange(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `HeightFilterPlacementCondition.RangeType` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/HeightFilterPlacementCondition.java)
    pub mod height_filter_placement_condition_range_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // HeightFilterPlacementCondition.RangeType.RangeType(1): not translated, its class isn't translated (its constants carry values).
        // HeightFilterPlacementCondition.RangeType.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `InBiomePlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/InBiomePlacementCondition.java)
    pub mod in_biome_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // InBiomePlacementCondition.InBiomePlacementCondition(1): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // InBiomePlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // InBiomePlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // InBiomePlacementCondition.biomes(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `MultipleOfPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/MultipleOfPlacementCondition.java)
    pub mod multiple_of_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MultipleOfPlacementCondition.MultipleOfPlacementCondition(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // MultipleOfPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // MultipleOfPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // MultipleOfPlacementCondition.conditions(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // MultipleOfPlacementCondition.allowedCount(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `NotPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/NotPlacementCondition.java)
    pub mod not_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NotPlacementCondition.NotPlacementCondition(1): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // NotPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // NotPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // NotPlacementCondition.condition(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `OffsetPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/OffsetPlacementCondition.java)
    pub mod offset_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // OffsetPlacementCondition.OffsetPlacementCondition(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // OffsetPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // OffsetPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // OffsetPlacementCondition.condition(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // OffsetPlacementCondition.offset(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `SampleAquifersPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleAquifersPlacementCondition.java)
    pub mod sample_aquifers_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SampleAquifersPlacementCondition.SampleAquifersPlacementCondition(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleAquifersPlacementCondition.SampleAquifersPlacementCondition(3): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleAquifersPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleAquifersPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleAquifersPlacementCondition.target(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleAquifersPlacementCondition.range(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `SampleDensityPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleDensityPlacementCondition.java)
    pub mod sample_density_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SampleDensityPlacementCondition.SampleDensityPlacementCondition(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleDensityPlacementCondition.SampleDensityPlacementCondition(3): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleDensityPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleDensityPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleDensityPlacementCondition.densityFunction(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleDensityPlacementCondition.range(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `SampleNoiseRouterPlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/SampleNoiseRouterPlacementCondition.java)
    pub mod sample_noise_router_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SampleNoiseRouterPlacementCondition.SampleNoiseRouterPlacementCondition(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleNoiseRouterPlacementCondition.SampleNoiseRouterPlacementCondition(3): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleNoiseRouterPlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleNoiseRouterPlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleNoiseRouterPlacementCondition.target(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // SampleNoiseRouterPlacementCondition.range(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `TruePlacementCondition` (dev/worldgen/lithostitched/impl/worldgen/placementcondition/TruePlacementCondition.java)
    pub mod true_placement_condition {
        use super::*;
        // The class itself isn't translated: it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition). Its static parts are below.
        // INSTANCE: not translated, it uses TruePlacementCondition, which isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TruePlacementCondition.test(2): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
        // TruePlacementCondition.codec(0): not translated, its class isn't translated (it implements `PlacementCondition` (dev.worldgen.lithostitched.api.worldgen.placementcondition)).
    }

    /// `ConditionPlacement` (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/ConditionPlacement.java)
    pub mod condition_placement {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PlacementFilter`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ConditionPlacement.ConditionPlacement(1): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // ConditionPlacement.shouldPlace(3): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // ConditionPlacement.codec(0): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
        // ConditionPlacement.condition(0): not translated, its class isn't translated (it implements Minecraft's `PlacementFilter`).
    }

    /// `NoiseSlopePlacement` (dev/worldgen/lithostitched/impl/worldgen/placementmodifier/NoiseSlopePlacement.java)
    pub mod noise_slope_placement {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PlacementModifier`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // NoiseSlopePlacement.NoiseSlopePlacement(5): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.modify(4): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.count(2): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.codec(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.noise(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.slope(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.offset(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.xzScale(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
        // NoiseSlopePlacement.yScale(0): not translated, its class isn't translated (it implements Minecraft's `PlacementModifier`).
    }

    /// `RandomEntries` (dev/worldgen/lithostitched/impl/worldgen/poolalias/RandomEntries.java)
    pub mod random_entries {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PoolAliasBinding`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RandomEntries.RandomEntries(2): not translated, its class isn't translated (it implements Minecraft's `PoolAliasBinding`).
        // RandomEntries.validate(1): not translated, it uses Minecraft's `DataResult`.
        // RandomEntries.forEachResolved(2): not translated, its class isn't translated (it implements Minecraft's `PoolAliasBinding`).
        // RandomEntries.allTargets(0): not translated, its class isn't translated (it implements Minecraft's `PoolAliasBinding`).
        // RandomEntries.codec(0): not translated, its class isn't translated (it implements Minecraft's `PoolAliasBinding`).
        // RandomEntries.aliases(0): not translated, its class isn't translated (it implements Minecraft's `PoolAliasBinding`).
        // RandomEntries.pools(0): not translated, its class isn't translated (it implements Minecraft's `PoolAliasBinding`).
    }

    /// `DelegatingConfig` (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingConfig.java)
    pub mod delegating_config {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DelegatingConfig.DelegatingConfig(10): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.DelegatingConfig(1): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.validate(1): not translated, it uses Minecraft's `DataResult`.
        // DelegatingConfig.getName(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.shouldCancelPlacement(4): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.delegate(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.name(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.placementCondition(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.allowedDepth(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.forcedCount(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.maxCount(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.allowBoundingBoxCollisions(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.otherPiecesCanIntersect(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.terrainMatchingHeightmap(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.overrideTerrainAdaptation(0): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `DelegatingElementBuilderImpl` (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingElementBuilderImpl.java)
    pub mod delegating_element_builder_impl {
        use super::*;
        // The class itself isn't translated: it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement). Its static parts are below.
        // DelegatingElementBuilderImpl.DelegatingElementBuilderImpl(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.forcedCount(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.limitedCount(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.allowedDepth(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.terrainAdaptation(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.condition(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.named(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.terrainMatchingHeightmap(1): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.allowBoundingBoxCollisions(0): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.otherPiecesCanIntersect(0): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
        // DelegatingElementBuilderImpl.build(0): not translated, its class isn't translated (it implements `DelegatingElementBuilder` (dev.worldgen.lithostitched.api.worldgen.poolelement)).
    }

    /// `DelegatingPoolElement` (dev/worldgen/lithostitched/impl/worldgen/poolelement/DelegatingPoolElement.java)
    pub mod delegating_pool_element {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `StructurePoolElement`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `StructurePoolElementType`.
        // DelegatingPoolElement.DelegatingPoolElement(1): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.DelegatingPoolElement(4): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.config(0): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.delegate(0): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.minDepth(0): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.prioritized(0): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.getSize(2): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.getShuffledJigsawBlocks(4): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.getBoundingBox(3): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.place(11): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.setProjection(1): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
        // DelegatingPoolElement.getType(0): not translated, its class isn't translated (it extends Minecraft's `StructurePoolElement`).
    }

    /// `LithostitchedFeatureConfig` (dev/worldgen/lithostitched/impl/worldgen/poolelement/LithostitchedFeatureConfig.java)
    pub mod lithostitched_feature_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Holder`. Its static parts are below.
        pub static DEFAULT_JIGSAW_NAME: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "bottom"));
        pub static DEFAULT_TARGET_NAME: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "empty"));
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // LithostitchedFeatureConfig.LithostitchedFeatureConfig(3): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // LithostitchedFeatureConfig.feature(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // LithostitchedFeatureConfig.jigsawName(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // LithostitchedFeatureConfig.targetName(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
    }

    /// `ApplyRandomStructureProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/ApplyRandomStructureProcessor.java)
    pub mod apply_random_structure_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // SET_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ApplyRandomStructureProcessor.ApplyRandomStructureProcessor(2): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ApplyRandomStructureProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ApplyRandomStructureProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ApplyRandomStructureProcessor.processorLists(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ApplyRandomStructureProcessor.randomSettings(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `BlockSwapStructureProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/BlockSwapStructureProcessor.java)
    pub mod block_swap_structure_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // BlockSwapStructureProcessor.BlockSwapStructureProcessor(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // BlockSwapStructureProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // BlockSwapStructureProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // BlockSwapStructureProcessor.blockSwapMap(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `ConditionProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/ConditionProcessor.java)
    pub mod condition_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // PROCESSOR_CODEC: not translated, it uses Minecraft's `Codec`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ConditionProcessor.ConditionProcessor(4): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ConditionProcessor.randomSettings(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ConditionProcessor.condition(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ConditionProcessor.thenRun(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ConditionProcessor.elseRun(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ConditionProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ConditionProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `AllOf` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AllOf.java)
    pub mod all_of {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AllOf.AllOf(1): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // AllOf.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // AllOf.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // AllOf.conditions(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `AnyOf` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/AnyOf.java)
    pub mod any_of {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AnyOf.AnyOf(1): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // AnyOf.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // AnyOf.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // AnyOf.conditions(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `MatchingBiomes` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBiomes.java)
    pub mod matching_biomes {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MatchingBiomes.MatchingBiomes(2): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBiomes.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBiomes.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBiomes.biomes(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBiomes.position(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `MatchingBlocks` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/MatchingBlocks.java)
    pub mod matching_blocks {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // DEFAULT_PREDICATE: not translated, it uses Minecraft's `StatePropertiesPredicate`.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // MatchingBlocks.MatchingBlocks(3): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBlocks.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBlocks.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBlocks.blocks(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBlocks.properties(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // MatchingBlocks.matchType(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `Not` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Not.java)
    pub mod not {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Not.Not(1): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // Not.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // Not.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // Not.condition(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `Position` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java)
    pub mod position {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // Position.Position(2): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // Position.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // Position.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // Position.predicate(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // Position.anchor(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `Position.PosAnchor` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/Position.java)
    pub mod position_pos_anchor {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `Codec`.
        // Position.PosAnchor.PosAnchor(1): not translated, its class isn't translated (its constants carry values).
        // Position.PosAnchor.get(1): not translated, its class isn't translated (its constants carry values).
        // Position.PosAnchor.getSerializedName(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `RandomChance` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/RandomChance.java)
    pub mod random_chance {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // RandomChance.RandomChance(1): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // RandomChance.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // RandomChance.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // RandomChance.chance(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `True` (dev/worldgen/lithostitched/impl/worldgen/processor/condition/True.java)
    pub mod true_ {
        use super::*;
        // The class itself isn't translated: it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition). Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // True.True(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // True.test(4): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
        // True.codec(0): not translated, its class isn't translated (it implements `ProcessorCondition` (dev.worldgen.lithostitched.api.worldgen.processorcondition)).
    }

    /// `DiscardInputProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/DiscardInputProcessor.java)
    pub mod discard_input_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // INSTANCE: not translated, it uses DiscardInputProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DiscardInputProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // DiscardInputProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `NonTemplateProcessorHelper` (dev/worldgen/lithostitched/impl/worldgen/processor/internal/NonTemplateProcessorHelper.java)
    pub mod non_template_processor_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct NonTemplateProcessorHelper {
        }
        impl NonTemplateProcessorHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // NonTemplateProcessorHelper.getProcessor(2): not translated, it uses Minecraft's `StructureProcessorList`.
        // NonTemplateProcessorHelper.getProcessor(2): not translated, it uses Minecraft's `StructureProcessorList`.
        // NonTemplateProcessorHelper.processBlock(6): not translated, it uses Minecraft's `BlockState`.
        // NonTemplateProcessorHelper.processBlock(7): not translated, it uses Minecraft's `BlockState`.
        // NonTemplateProcessorHelper.processBlock(5): not translated, it uses Minecraft's `BlockState`.
        // NonTemplateProcessorHelper.getProcessorAndApply(7): not translated, it uses ReferencePosDuck, which isn't translated (it's an interface).
        // NonTemplateProcessorHelper.getProcessorAndApply(10): not translated, it uses ReferencePosDuck, which isn't translated (it's an interface).
    }

    /// `ReferenceStructureProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/ReferenceStructureProcessor.java)
    pub mod reference_structure_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ReferenceStructureProcessor.ReferenceStructureProcessor(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ReferenceStructureProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ReferenceStructureProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ReferenceStructureProcessor.processorLists(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `ScheduleTickProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/ScheduleTickProcessor.java)
    pub mod schedule_tick_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // INSTANCE: not translated, it uses ScheduleTickProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`).
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // ScheduleTickProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // ScheduleTickProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `SetBlockProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/SetBlockProcessor.java)
    pub mod set_block_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // SetBlockProcessor.SetBlockProcessor(4): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SetBlockProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SetBlockProcessor.withState(3): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SetBlockProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SetBlockProcessor.stateProvider(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SetBlockProcessor.preserveState(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SetBlockProcessor.randomMode(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // SetBlockProcessor.modifier(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `UnboundReferenceProcessor` (dev/worldgen/lithostitched/impl/worldgen/processor/UnboundReferenceProcessor.java)
    pub mod unbound_reference_processor {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `StructureProcessor`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // UnboundReferenceProcessor.UnboundReferenceProcessor(1): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // UnboundReferenceProcessor.of(1): not translated, it uses UnboundReferenceProcessor, which isn't translated (it implements Minecraft's `StructureProcessor`).
        // UnboundReferenceProcessor.key(1): not translated, it uses Minecraft's `ResourceKey`.
        // UnboundReferenceProcessor.name(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // UnboundReferenceProcessor.processBlock(6): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
        // UnboundReferenceProcessor.codec(0): not translated, its class isn't translated (it implements Minecraft's `StructureProcessor`).
    }

    /// `WeightedProvider` (dev/worldgen/lithostitched/impl/worldgen/stateprovider/WeightedProvider.java)
    pub mod weighted_provider {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `BlockStateProvider`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // WeightedProvider.WeightedProvider(1): not translated, its class isn't translated (it implements Minecraft's `BlockStateProvider`).
        // WeightedProvider.getState(3): not translated, its class isn't translated (it implements Minecraft's `BlockStateProvider`).
        // WeightedProvider.getOptionalState(3): not translated, its class isn't translated (it implements Minecraft's `BlockStateProvider`).
        // WeightedProvider.codec(0): not translated, its class isn't translated (it implements Minecraft's `BlockStateProvider`).
        // WeightedProvider.providers(0): not translated, its class isn't translated (it implements Minecraft's `BlockStateProvider`).
    }

    /// `AlternateJigsawConfig` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawConfig.java)
    pub mod alternate_jigsaw_config {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Holder`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // AlternateJigsawConfig.AlternateJigsawConfig(11): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.create(12): not translated, it uses AlternateJigsawConfig, which isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.setPoolAliases(2): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.projectStartToHeightmap(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.startPool(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.startJigsawName(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.size(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.fixedRotation(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.startHeight(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.useExpansionHack(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.startProjection(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.maxDistanceFromCenter(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.poolAliases(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.dimensionPadding(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
        // AlternateJigsawConfig.liquidSettings(0): not translated, its class isn't translated (it uses Minecraft's `Holder`).
    }

    /// `AlternateJigsawGenerator` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java)
    pub mod alternate_jigsaw_generator {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct AlternateJigsawGenerator {
        }
        impl AlternateJigsawGenerator {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // AlternateJigsawGenerator.generate(6): not translated, it uses Minecraft's `GenerationStub`.
        // AlternateJigsawGenerator.pieceWithinPaddingBounds(3): not translated, it uses Minecraft's `LevelHeightAccessor`.
        // AlternateJigsawGenerator.findNamedJigsaw(6): not translated, it uses Minecraft's `BlockPos`.
        // AlternateJigsawGenerator.generatePieces(14): not translated, it uses Minecraft's `GenerationContext`.
        // AlternateJigsawGenerator.getConfig(1): not translated, it uses `DelegatingConfig`.
    }

    /// `AlternateJigsawGenerator.PieceState` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java)
    pub mod alternate_jigsaw_generator_piece_state {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `PoolElementStructurePiece`. Its static parts are below.
        // AlternateJigsawGenerator.PieceState.PieceState(3): not translated, its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`).
        // AlternateJigsawGenerator.PieceState.piece(0): not translated, its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`).
        // AlternateJigsawGenerator.PieceState.octree(0): not translated, its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`).
        // AlternateJigsawGenerator.PieceState.currentSize(0): not translated, its class isn't translated (it uses Minecraft's `PoolElementStructurePiece`).
    }

    /// `AlternateJigsawGenerator.StructurePoolGenerator` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawGenerator.java)
    pub mod alternate_jigsaw_generator_structure_pool_generator {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `GenerationContext`. Its static parts are below.
        // AlternateJigsawGenerator.StructurePoolGenerator.StructurePoolGenerator(8): not translated, its class isn't translated (it uses Minecraft's `GenerationContext`).
        // AlternateJigsawGenerator.StructurePoolGenerator.generatePiece(7): not translated, its class isn't translated (it uses Minecraft's `GenerationContext`).
        // AlternateJigsawGenerator.StructurePoolGenerator.findAndTestChildCandidates(12): not translated, its class isn't translated (it uses Minecraft's `GenerationContext`).
        // AlternateJigsawGenerator.StructurePoolGenerator.getPoolElements(4): not translated, its class isn't translated (it uses Minecraft's `GenerationContext`).
        // AlternateJigsawGenerator.StructurePoolGenerator.findValidChildPiece(10): not translated, its class isn't translated (it uses Minecraft's `GenerationContext`).
        // AlternateJigsawGenerator.StructurePoolGenerator.getTemplatePoolHolder(2): not translated, its class isn't translated (it uses Minecraft's `GenerationContext`).
        // AlternateJigsawGenerator.StructurePoolGenerator.adjustJigsawPos(1): not translated, it uses Minecraft's `BlockPos`.
    }

    /// `AlternateJigsawStructure` (dev/worldgen/lithostitched/impl/worldgen/structure/AlternateJigsawStructure.java)
    pub mod alternate_jigsaw_structure {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Structure`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `StructureType`.
        // AlternateJigsawStructure.validate(1): not translated, it uses Minecraft's `DataResult`.
        // AlternateJigsawStructure.AlternateJigsawStructure(2): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // AlternateJigsawStructure.setPoolAliases(2): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // AlternateJigsawStructure.config(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // AlternateJigsawStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // AlternateJigsawStructure.generate(6): not translated, it uses Minecraft's `GenerationStub`.
        // AlternateJigsawStructure.type(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
    }

    /// `BoxOctree` (dev/worldgen/lithostitched/impl/worldgen/structure/BoxOctree.java)
    pub mod box_octree {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `AABB`. Its static parts are below.
        pub const SUBDIVIDE_THRESHOLD: i32 = 10;
        pub const MAXIMUM_DEPTH: i32 = 3;
        // BoxOctree.BoxOctree(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.BoxOctree(2): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.roundAwayFromZero(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.subdivide(0): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.addBox(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.boundaryEntirelyContains(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.boundaryIntersects(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.withinBoundsButNotIntersectingChildren(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.intersectsAnyBox(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.boundaryContains(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
        // BoxOctree.withinAnyBox(1): not translated, its class isn't translated (it uses Minecraft's `AABB`).
    }

    /// `DelegatingConfig` (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingConfig.java)
    pub mod delegating_config_2 {
        use super::*;
        // The class itself isn't translated: another class has the same name, in a different package. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // DelegatingConfig.DelegatingConfig(2): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.DelegatingConfig(3): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.delegate(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.spawnCondition(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.attributes(0): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.setSpawnCondition(2): not translated, its class isn't translated (another class has the same name, in a different package).
        // DelegatingConfig.setAttributes(2): not translated, its class isn't translated (another class has the same name, in a different package).
    }

    /// `DelegatingStructure` (dev/worldgen/lithostitched/impl/worldgen/structure/DelegatingStructure.java)
    pub mod delegating_structure {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Structure`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `StructureType`.
        // DelegatingStructure.DelegatingStructure(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.config(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.delegate(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.findValidGenerationPoint(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.isValid(2): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.findGenerationPoint(1): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.afterPlace(7): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.type(0): not translated, its class isn't translated (it extends Minecraft's `Structure`).
        // DelegatingStructure.createSettings(1): not translated, it uses Minecraft's `StructureSettings`.
    }

    /// `LithostitchedTemplates` (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java)
    pub mod lithostitched_templates {
        use super::*;
        // The class itself isn't translated: it implements `Iterable`. Its static parts are below.
        // LithostitchedTemplates.add(2): not translated, its class isn't translated (it implements `Iterable`).
        // LithostitchedTemplates.shuffle(1): not translated, its class isn't translated (it implements `Iterable`).
        // LithostitchedTemplates.stream(0): not translated, its class isn't translated (it implements `Iterable`).
        // LithostitchedTemplates.iterator(0): not translated, its class isn't translated (it implements `Iterable`).
    }

    /// `LithostitchedTemplates.WeightedEntry` (dev/worldgen/lithostitched/impl/worldgen/structure/LithostitchedTemplates.java)
    pub mod lithostitched_templates_weighted_entry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `StructurePoolElement`. Its static parts are below.
        // LithostitchedTemplates.WeightedEntry.WeightedEntry(3): not translated, its class isn't translated (it uses Minecraft's `StructurePoolElement`).
        // LithostitchedTemplates.WeightedEntry.copy(0): not translated, its class isn't translated (it uses Minecraft's `StructurePoolElement`).
        // LithostitchedTemplates.WeightedEntry.getRandWeight(0): not translated, its class isn't translated (it uses Minecraft's `StructurePoolElement`).
        // LithostitchedTemplates.WeightedEntry.setRandom(1): not translated, its class isn't translated (it uses Minecraft's `StructurePoolElement`).
        // LithostitchedTemplates.WeightedEntry.getElement(0): not translated, its class isn't translated (it uses Minecraft's `StructurePoolElement`).
        // LithostitchedTemplates.WeightedEntry.getIndex(0): not translated, its class isn't translated (it uses Minecraft's `StructurePoolElement`).
        // LithostitchedTemplates.WeightedEntry.toString(0): not translated, its class isn't translated (it uses Minecraft's `StructurePoolElement`).
    }

    /// `StructureAttributeHandler` (dev/worldgen/lithostitched/impl/worldgen/structure/StructureAttributeHandler.java)
    pub mod structure_attribute_handler {
        use super::*;
        pub const DELAY_BETWEEN_STRUCTURE_CHECKS: i32 = 5;
        pub const STRUCTURE_ATTRIBUTE_LERP: i32 = 40;
        // playersToStructures: not translated, it uses Minecraft's `ServerPlayer`.
        pub static DELAY: JCell<i32> = JCell::new(5);
        #[derive(Clone, Debug)]
        pub struct StructureAttributeHandler {
        }
        impl StructureAttributeHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // StructureAttributeHandler.tick(1): not translated, it uses Minecraft's `ServerLevel`.
        // StructureAttributeHandler.disconnect(1): not translated, it uses Minecraft's `ServerPlayer`.
        // StructureAttributeHandler.hasStructureNotChanged(2): not translated, it uses Minecraft's `ServerPlayer`.
        // StructureAttributeHandler.updateAttributes(3): not translated, it uses Minecraft's `ServerPlayer`.
        // StructureAttributeHandler.sendPacket(2): not translated, it uses Minecraft's `ServerPlayer`.
    }

    /// `BranchedMegaJungleFoliagePlacer` (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaJungleFoliagePlacer.java)
    pub mod branched_mega_jungle_foliage_placer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MegaJungleFoliagePlacer`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `FoliagePlacerType`.
        // BranchedMegaJungleFoliagePlacer.BranchedMegaJungleFoliagePlacer(3): not translated, its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`).
        // BranchedMegaJungleFoliagePlacer.type(0): not translated, its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`).
        // BranchedMegaJungleFoliagePlacer.createFoliage(9): not translated, its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`).
        // BranchedMegaJungleFoliagePlacer.shouldSkipLocation(6): not translated, its class isn't translated (it extends Minecraft's `MegaJungleFoliagePlacer`).
    }

    /// `BranchedMegaPineFoliagePlacer` (dev/worldgen/lithostitched/impl/worldgen/tree/foliageplacer/BranchedMegaPineFoliagePlacer.java)
    pub mod branched_mega_pine_foliage_placer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MegaPineFoliagePlacer`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `FoliagePlacerType`.
        // BranchedMegaPineFoliagePlacer.BranchedMegaPineFoliagePlacer(3): not translated, its class isn't translated (it extends Minecraft's `MegaPineFoliagePlacer`).
        // BranchedMegaPineFoliagePlacer.type(0): not translated, its class isn't translated (it extends Minecraft's `MegaPineFoliagePlacer`).
        // BranchedMegaPineFoliagePlacer.createFoliage(9): not translated, its class isn't translated (it extends Minecraft's `MegaPineFoliagePlacer`).
    }

    /// `LargeMangroveRootPlacer` (dev/worldgen/lithostitched/impl/worldgen/tree/rootplacer/LargeMangroveRootPlacer.java)
    pub mod large_mangrove_root_placer {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `MangroveRootPlacer`. Its static parts are below.
        // CODEC: not translated, it uses Minecraft's `MapCodec`.
        // TYPE: not translated, it uses Minecraft's `RootPlacerType`.
        // LargeMangroveRootPlacer.LargeMangroveRootPlacer(4): not translated, its class isn't translated (it extends Minecraft's `MangroveRootPlacer`).
        // LargeMangroveRootPlacer.type(0): not translated, its class isn't translated (it extends Minecraft's `MangroveRootPlacer`).
        // LargeMangroveRootPlacer.placeRoots(6): not translated, its class isn't translated (it extends Minecraft's `MangroveRootPlacer`).
        // LargeMangroveRootPlacer.getTrunkOrigin(2): not translated, its class isn't translated (it extends Minecraft's `MangroveRootPlacer`).
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "lithostitched");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
