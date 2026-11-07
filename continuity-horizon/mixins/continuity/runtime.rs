// Continuity, ported to Horizon by horizon-port from continuity-3.0.1+26.3.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_continuity). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "continuity";
pub const NAME: &str = "Continuity";
pub const VERSION: &str = "3.0.1+26.3";
pub const DESCRIPTION: &str = "Continuity is a Minecraft mod that allows resource packs that use the OptiFine connected textures format or OptiFine emissive textures format (only for blocks and item models) to work without OptiFine.";

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

    /// `CachingPredicates` (me/pepperbell/continuity/api/client/CachingPredicates.java)
    pub mod caching_predicates {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CachingPredicates.affectsSprites(0): not translated, it has no body.
        // CachingPredicates.affectsSprite(1): not translated, it has no body.
        // CachingPredicates.affectsBlockStates(0): not translated, it has no body.
        // CachingPredicates.affectsBlockState(1): not translated, it has no body.
        // CachingPredicates.isValidForMultipass(0): not translated, it has no body.
    }

    /// `CachingPredicates.Factory` (me/pepperbell/continuity/api/client/CachingPredicates.java)
    pub mod caching_predicates_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CachingPredicates.Factory.createPredicates(2): not translated, it has no body.
    }

    /// `ContinuityFeatureStates` (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java)
    pub mod continuity_feature_states {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ContinuityFeatureStates.get(0): not translated, it uses ContinuityFeatureStates, which isn't translated (it's an interface).
        // ContinuityFeatureStates.getConnectedTexturesState(0): not translated, it has no body.
        // ContinuityFeatureStates.getEmissiveTexturesState(0): not translated, it has no body.
    }

    /// `ContinuityFeatureStates.FeatureState` (me/pepperbell/continuity/api/client/ContinuityFeatureStates.java)
    pub mod continuity_feature_states_feature_state {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ContinuityFeatureStates.FeatureState.isEnabled(0): not translated, it has no body.
        // ContinuityFeatureStates.FeatureState.enable(0): not translated, it has no body.
        // ContinuityFeatureStates.FeatureState.disable(0): not translated, it has no body.
    }

    /// `CtmLoader` (me/pepperbell/continuity/api/client/CtmLoader.java)
    pub mod ctm_loader {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CtmLoader.getPropertiesFactory(0): not translated, it has no body.
        // CtmLoader.getProcessorFactory(0): not translated, it has no body.
        // CtmLoader.getPredicatesFactory(0): not translated, it has no body.
    }

    /// `CtmLoaderRegistry` (me/pepperbell/continuity/api/client/CtmLoaderRegistry.java)
    pub mod ctm_loader_registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CtmLoaderRegistry.get(0): not translated, it uses CtmLoaderRegistry, which isn't translated (it's an interface).
        // CtmLoaderRegistry.registerLoader(2): not translated, it has no body.
        // CtmLoaderRegistry.getLoader(1): not translated, it has no body.
    }

    /// `CtmProperties` (me/pepperbell/continuity/api/client/CtmProperties.java)
    pub mod ctm_properties {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CtmProperties.getSpriteDependencies(0): not translated, it has no body.
    }

    /// `CtmProperties.Factory` (me/pepperbell/continuity/api/client/CtmProperties.java)
    pub mod ctm_properties_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CtmProperties.Factory.createProperties(6): not translated, it has no body.
    }

    /// `EmissiveSpriteApi` (me/pepperbell/continuity/api/client/EmissiveSpriteApi.java)
    pub mod emissive_sprite_api {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EmissiveSpriteApi.get(0): not translated, it uses EmissiveSpriteApi, which isn't translated (it's an interface).
        // EmissiveSpriteApi.getEmissiveSprite(1): not translated, it has no body.
    }

    /// `ProcessingDataKey` (me/pepperbell/continuity/api/client/ProcessingDataKey.java)
    pub mod processing_data_key {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ProcessingDataKey.getId(0): not translated, it has no body.
        // ProcessingDataKey.getRawId(0): not translated, it has no body.
        // ProcessingDataKey.getValueSupplier(0): not translated, it has no body.
        // ProcessingDataKey.getValueResetAction(0): not translated, it has no body.
    }

    /// `ProcessingDataKeyRegistry` (me/pepperbell/continuity/api/client/ProcessingDataKeyRegistry.java)
    pub mod processing_data_key_registry {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ProcessingDataKeyRegistry.get(0): not translated, it uses ProcessingDataKeyRegistry, which isn't translated (it's an interface).
        // ProcessingDataKeyRegistry.registerKey(2): not translated, it's generic.
        // ProcessingDataKeyRegistry.registerKey(3): not translated, it has no body.
        // ProcessingDataKeyRegistry.getKey(1): not translated, it has no body.
        // ProcessingDataKeyRegistry.getRegisteredAmount(0): not translated, it has no body.
    }

    /// `ProcessingDataProvider` (me/pepperbell/continuity/api/client/ProcessingDataProvider.java)
    pub mod processing_data_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ProcessingDataProvider.getData(1): not translated, it has no body.
    }

    /// `QuadProcessor` (me/pepperbell/continuity/api/client/QuadProcessor.java)
    pub mod quad_processor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // QuadProcessor.processQuad(9): not translated, it has no body.
    }

    /// `QuadProcessor.Factory` (me/pepperbell/continuity/api/client/QuadProcessor.java)
    pub mod quad_processor_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // QuadProcessor.Factory.createProcessor(2): not translated, it has no body.
    }

    /// `QuadProcessor.ProcessingContext` (me/pepperbell/continuity/api/client/QuadProcessor.java)
    pub mod quad_processor_processing_context {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // QuadProcessor.ProcessingContext.getExtraQuadEmitter(0): not translated, it has no body.
    }

    /// `QuadProcessor.ProcessingResult` (me/pepperbell/continuity/api/client/QuadProcessor.java)
    pub mod quad_processor_processing_result {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum ProcessingResult {
            NextProcessor,
            NextPass,
            Stop,
            Discard,
        }
        impl ProcessingResult {
            pub const VALUES: [ProcessingResult; 4] = [ProcessingResult::NextProcessor, ProcessingResult::NextPass, ProcessingResult::Stop, ProcessingResult::Discard, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    ProcessingResult::NextProcessor => "NEXT_PROCESSOR",
                    ProcessingResult::NextPass => "NEXT_PASS",
                    ProcessingResult::Stop => "STOP",
                    ProcessingResult::Discard => "DISCARD",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NEXT_PROCESSOR" => ProcessingResult::NextProcessor,
                    "NEXT_PASS" => ProcessingResult::NextPass,
                    "STOP" => ProcessingResult::Stop,
                    "DISCARD" => ProcessingResult::Discard,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `ContinuityConfig` (me/pepperbell/continuity/client/config/ContinuityConfig.java)
    pub mod continuity_config {
        use super::*;
        // The class itself isn't translated: it uses `File` (java.io). Its static parts are below.
        // LOGGER: the logger. Log calls become comments.
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson { pretty: true, ..JGson::default() });
        // INSTANCE: not translated, it uses ContinuityConfig, which isn't translated (it uses `File` (java.io)).
        // ContinuityConfig.ContinuityConfig(1): not translated, its class isn't translated (it uses `File` (java.io)).
        // ContinuityConfig.load(0): not translated, its class isn't translated (it uses `File` (java.io)).
        // ContinuityConfig.save(0): not translated, its class isn't translated (it uses `File` (java.io)).
        // ContinuityConfig.fromJson(1): not translated, its class isn't translated (it uses `File` (java.io)).
        // ContinuityConfig.toJson(0): not translated, its class isn't translated (it uses `File` (java.io)).
        // ContinuityConfig.addOption(1): not translated, it's generic.
        // ContinuityConfig.getOption(1): not translated, its class isn't translated (it uses `File` (java.io)).
        // ContinuityConfig.getOptionMapView(0): not translated, its class isn't translated (it uses `File` (java.io)).
    }

    /// `ContinuityConfigScreen` (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java)
    pub mod continuity_config_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // ContinuityConfigScreen.ContinuityConfigScreen(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ContinuityConfigScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ContinuityConfigScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ContinuityConfigScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ContinuityConfigScreen.saveValues(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        pub fn get_translation_key(option_key: &str) -> String {
            return format!("options.continuity.{}", option_key);
        }
        pub fn get_tooltip_key(translation_key: &str) -> String {
            return format!("{}.tooltip", translation_key);
        }
        // ContinuityConfigScreen.startBooleanValueButton(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `ContinuityConfigScreen.Value` (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java)
    pub mod continuity_config_screen_value {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ContinuityConfigScreen.Value.Value(2): not translated, its class isn't translated (it's generic).
        // ContinuityConfigScreen.Value.of(2): not translated, it's generic.
        // ContinuityConfigScreen.Value.getOption(0): not translated, its class isn't translated (it's generic).
        // ContinuityConfigScreen.Value.getFlags(0): not translated, its class isn't translated (it's generic).
        // ContinuityConfigScreen.Value.get(0): not translated, its class isn't translated (it's generic).
        // ContinuityConfigScreen.Value.set(1): not translated, its class isn't translated (it's generic).
        // ContinuityConfigScreen.Value.isChanged(0): not translated, its class isn't translated (it's generic).
        // ContinuityConfigScreen.Value.saveToOption(0): not translated, its class isn't translated (it's generic).
    }

    /// `ContinuityConfigScreen.Value.Flag` (me/pepperbell/continuity/client/config/ContinuityConfigScreen.java)
    pub mod continuity_config_screen_value_flag {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // ContinuityConfigScreen.Value.Flag.onSave(0): not translated, it has no body.
    }

    /// `ModMenuApiImpl` (me/pepperbell/continuity/client/config/ModMenuApiImpl.java)
    pub mod mod_menu_api_impl {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ModMenuApiImpl.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `Option` (me/pepperbell/continuity/client/config/Option.java)
    pub mod option {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // Option.getKey(0): not translated, it has no body.
        // Option.getDefault(0): not translated, it has no body.
        // Option.get(0): not translated, it has no body.
        // Option.set(1): not translated, it has no body.
        // Option.toJson(0): not translated, it has no body.
        // Option.fromJson(1): not translated, it has no body.
    }

    /// `Option.BaseOption` (me/pepperbell/continuity/client/config/Option.java)
    pub mod option_base_option {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // Option.BaseOption.BaseOption(2): not translated, its class isn't translated (it's generic).
        // Option.BaseOption.getKey(0): not translated, its class isn't translated (it's generic).
        // Option.BaseOption.getDefault(0): not translated, its class isn't translated (it's generic).
        // Option.BaseOption.get(0): not translated, its class isn't translated (it's generic).
        // Option.BaseOption.set(1): not translated, its class isn't translated (it's generic).
    }

    /// `Option.BooleanOption` (me/pepperbell/continuity/client/config/Option.java)
    pub mod option_boolean_option {
        use super::*;
        // The class itself isn't translated: it extends `BaseOption`. Its static parts are below.
        // Option.BooleanOption.BooleanOption(2): not translated, its class isn't translated (it extends `BaseOption`).
        // Option.BooleanOption.toJson(0): not translated, its class isn't translated (it extends `BaseOption`).
        // Option.BooleanOption.fromJson(1): not translated, its class isn't translated (it extends `BaseOption`).
    }

    /// `SodiumConfigImpl` (me/pepperbell/continuity/client/config/SodiumConfigImpl.java)
    pub mod sodium_config_impl {
        use super::*;
        // The class itself isn't translated: it implements `ConfigEntryPoint` (net.caffeinemc.mods.sodium.api.config). Its static parts are below.
        // SodiumConfigImpl.registerConfigLate(1): not translated, its class isn't translated (it implements `ConfigEntryPoint` (net.caffeinemc.mods.sodium.api.config)).
        // SodiumConfigImpl.asSodiumOption(3): not translated, it uses `BooleanOptionBuilder` (net.caffeinemc.mods.sodium.api.config.structure).
    }

    /// `SodiumConfigImpl.OptionBindingImpl` (me/pepperbell/continuity/client/config/SodiumConfigImpl.java)
    pub mod sodium_config_impl_option_binding_impl {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SodiumConfigImpl.OptionBindingImpl.OptionBindingImpl(1): not translated, its class isn't translated (it's generic).
        // SodiumConfigImpl.OptionBindingImpl.save(1): not translated, its class isn't translated (it's generic).
        // SodiumConfigImpl.OptionBindingImpl.load(0): not translated, its class isn't translated (it's generic).
    }

    /// `ContinuityClient` (me/pepperbell/continuity/client/ContinuityClient.java)
    pub mod continuity_client {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        pub const ID: &str = "continuity";
        pub const NAME: &str = "Continuity";
        // LOGGER: the logger. Log calls become comments.
        // ContinuityClient.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
        // ContinuityClient.createLoader(3): not translated, its body couldn't be read: an anonymous class.
        // ContinuityClient.createLoader(4): not translated, it's generic.
        // ContinuityClient.createLoader(3): not translated, its body couldn't be read: an anonymous class.
        // ContinuityClient.createLoader(3): not translated, its body couldn't be read: an anonymous class.
        // ContinuityClient.createLoader(2): not translated, it's generic.
        // ContinuityClient.createCustomLoader(4): not translated, it's generic.
        // ContinuityClient.createCustomLoader(3): not translated, it's generic.
        // ContinuityClient.wrapWithOptifineOnlyCheck(1): not translated, it's generic.
        pub fn as_id(path: &str) -> JIdentifier {
            return JIdentifier::new("continuity", &path);
        }
    }

    /// `TextureAtlasSpriteExtension` (me/pepperbell/continuity/client/mixinterface/TextureAtlasSpriteExtension.java)
    pub mod texture_atlas_sprite_extension {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TextureAtlasSpriteExtension.continuity$getEmissiveSprite(0): not translated, it has no body.
        // TextureAtlasSpriteExtension.continuity$setEmissiveSprite(1): not translated, it has no body.
    }

    /// `CtmBlockStateModel` (me/pepperbell/continuity/client/model/CtmBlockStateModel.java)
    pub mod ctm_block_state_model {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `WrapperBlockStateModel`. Its static parts are below.
        pub const PASSES: i32 = 4;
        // CtmBlockStateModel.CtmBlockStateModel(2): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // CtmBlockStateModel.emitQuads(6): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // CtmBlockStateModel.createGeometryKey(4): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // CtmBlockStateModel.getSliceFunc(1): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
    }

    /// `CtmBlockStateModel.CtmQuadTransform` (me/pepperbell/continuity/client/model/CtmBlockStateModel.java)
    pub mod ctm_block_state_model_ctm_quad_transform {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `QuadTransform`. Its static parts are below.
        // CtmBlockStateModel.CtmQuadTransform.transform(1): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // CtmBlockStateModel.CtmQuadTransform.transformOnce(2): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // CtmBlockStateModel.CtmQuadTransform.isActive(0): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // CtmBlockStateModel.CtmQuadTransform.prepare(7): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // CtmBlockStateModel.CtmQuadTransform.reset(0): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
    }

    /// `EmissiveBlockStateModel` (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java)
    pub mod emissive_block_state_model {
        use super::*;
        // The class itself isn't translated: it extends Fabric's `WrapperBlockStateModel`. Its static parts are below.
        // EmissiveBlockStateModel.EmissiveBlockStateModel(1): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // EmissiveBlockStateModel.emitQuads(6): not translated, its class isn't translated (it extends Fabric's `WrapperBlockStateModel`).
        // EmissiveBlockStateModel.createGeometryKey(4): not translated, its body couldn't be read: a class declared inside a method.
    }

    /// `EmissiveBlockStateModel.EmissiveQuadTransform` (me/pepperbell/continuity/client/model/EmissiveBlockStateModel.java)
    pub mod emissive_block_state_model_emissive_quad_transform {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `QuadTransform`. Its static parts are below.
        // EmissiveBlockStateModel.EmissiveQuadTransform.transform(1): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // EmissiveBlockStateModel.EmissiveQuadTransform.isActive(0): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // EmissiveBlockStateModel.EmissiveQuadTransform.prepare(3): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // EmissiveBlockStateModel.EmissiveQuadTransform.reset(0): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
    }

    /// `EmissiveItemGeometry` (me/pepperbell/continuity/client/model/EmissiveItemGeometry.java)
    pub mod emissive_item_geometry {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ItemQuads`. Its static parts are below.
        // EMISSIVE_GEOMETRY_MARKER: not translated, it uses `Object`.
        // EmissiveItemGeometry.EmissiveItemGeometry(3): not translated, its class isn't translated (it uses Minecraft's `ItemQuads`).
        // EmissiveItemGeometry.of(3): not translated, it uses EmissiveItemGeometry, which isn't translated (it uses Minecraft's `ItemQuads`).
        // EmissiveItemGeometry.of(1): not translated, it uses EmissiveItemGeometry, which isn't translated (it uses Minecraft's `ItemQuads`).
        // EmissiveItemGeometry.combine(2): not translated, it uses Minecraft's `BakedQuad`.
        // EmissiveItemGeometry.of(2): not translated, its body couldn't be read: `var`.
        // EmissiveItemGeometry.baseAndEmissiveItemQuads(0): not translated, its class isn't translated (it uses Minecraft's `ItemQuads`).
        // EmissiveItemGeometry.emissiveMesh(0): not translated, its class isn't translated (it uses Minecraft's `ItemQuads`).
        // EmissiveItemGeometry.emissiveAnimated(0): not translated, its class isn't translated (it uses Minecraft's `ItemQuads`).
    }

    /// `ModelObjectsContainer` (me/pepperbell/continuity/client/model/ModelObjectsContainer.java)
    pub mod model_objects_container {
        use super::*;
        // The class itself isn't translated: it uses CtmQuadTransform, which isn't translated (it implements Fabric's `QuadTransform`). Its static parts are below.
        // THREAD_LOCAL: not translated, it uses `ThreadLocal`.
        // ModelObjectsContainer.get(0): not translated, it uses ModelObjectsContainer, which isn't translated (it uses CtmQuadTransform, which isn't translated (it implements Fabric's `QuadTransform`)).
    }

    /// `QuadProcessors` (me/pepperbell/continuity/client/model/QuadProcessors.java)
    pub mod quad_processors {
        use super::*;
        // processorHolders: not translated, it uses ProcessorHolder, which isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)).
        // CACHE: not translated, it uses BlockStateKeyCache, which isn't translated (it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects)).
        #[derive(Clone, Debug)]
        pub struct QuadProcessors {
        }
        impl QuadProcessors {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // QuadProcessors.getCache(1): not translated, it uses Minecraft's `TextureAtlasSprite`.
        // QuadProcessors.computeSlice(2): not translated, it uses Minecraft's `BlockState`.
        // QuadProcessors.reload(1): not translated, it uses QuadProcessors.processorHolders, which isn't translated (a QuadProcessors.ProcessorHolder with no value (null in Java)).
    }

    /// `QuadProcessors.BlockStateKeyCache` (me/pepperbell/continuity/client/model/QuadProcessors.java)
    pub mod quad_processors_block_state_key_cache {
        use super::*;
        // The class itself isn't translated: it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects). Its static parts are below.
        // QuadProcessors.BlockStateKeyCache.apply(1): not translated, its class isn't translated (it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects)).
        // QuadProcessors.BlockStateKeyCache.clear(0): not translated, its class isn't translated (it uses `Reference2ReferenceOpenHashMap` (it.unimi.dsi.fastutil.objects)).
    }

    /// `QuadProcessors.ProcessorHolder` (me/pepperbell/continuity/client/model/QuadProcessors.java)
    pub mod quad_processors_processor_holder {
        use super::*;
        // The class itself isn't translated: it uses QuadProcessor, which isn't translated (it's an interface). Its static parts are below.
        // QuadProcessors.ProcessorHolder.ProcessorHolder(2): not translated, its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)).
        // QuadProcessors.ProcessorHolder.processor(0): not translated, its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)).
        // QuadProcessors.ProcessorHolder.predicates(0): not translated, its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)).
    }

    /// `QuadProcessors.Slice` (me/pepperbell/continuity/client/model/QuadProcessors.java)
    pub mod quad_processors_slice {
        use super::*;
        // The class itself isn't translated: it uses QuadProcessor, which isn't translated (it's an interface). Its static parts are below.
        // QuadProcessors.Slice.Slice(2): not translated, its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)).
        // QuadProcessors.Slice.processors(0): not translated, its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)).
        // QuadProcessors.Slice.multipassProcessors(0): not translated, its class isn't translated (it uses QuadProcessor, which isn't translated (it's an interface)).
    }

    /// `QuadProcessors.SpriteKeyCache` (me/pepperbell/continuity/client/model/QuadProcessors.java)
    pub mod quad_processors_sprite_key_cache {
        use super::*;
        // The class itself isn't translated: it implements `Function` (java.util.function). Its static parts are below.
        // QuadProcessors.SpriteKeyCache.SpriteKeyCache(1): not translated, its class isn't translated (it implements `Function` (java.util.function)).
        // QuadProcessors.SpriteKeyCache.apply(1): not translated, its class isn't translated (it implements `Function` (java.util.function)).
        // QuadProcessors.SpriteKeyCache.clear(0): not translated, its class isn't translated (it implements `Function` (java.util.function)).
    }

    /// `AbstractQuadProcessor` (me/pepperbell/continuity/client/processor/AbstractQuadProcessor.java)
    pub mod abstract_quad_processor {
        use super::*;
        // The class itself isn't translated: it implements `QuadProcessor` (me.pepperbell.continuity.api.client). Its static parts are below.
        // AbstractQuadProcessor.AbstractQuadProcessor(2): not translated, its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)).
        // AbstractQuadProcessor.processQuad(9): not translated, its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)).
        // AbstractQuadProcessor.processQuadInner(9): not translated, it has no body.
    }

    /// `AbstractQuadProcessorFactory` (me/pepperbell/continuity/client/processor/AbstractQuadProcessorFactory.java)
    pub mod abstract_quad_processor_factory {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // AbstractQuadProcessorFactory.createProcessor(2): not translated, its class isn't translated (it's generic).
        // AbstractQuadProcessorFactory.createProcessor(2): not translated, its class isn't translated (it's generic).
        // AbstractQuadProcessorFactory.getSpriteAmount(1): not translated, it has no body.
        // AbstractQuadProcessorFactory.supportsNullSprites(1): not translated, its class isn't translated (it's generic).
    }

    /// `BaseCachingPredicates` (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java)
    pub mod base_caching_predicates {
        use super::*;
        // The class itself isn't translated: it implements `CachingPredicates` (me.pepperbell.continuity.api.client). Its static parts are below.
        // BaseCachingPredicates.BaseCachingPredicates(3): not translated, its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)).
        // BaseCachingPredicates.affectsSprites(0): not translated, its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)).
        // BaseCachingPredicates.affectsSprite(1): not translated, its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)).
        // BaseCachingPredicates.affectsBlockStates(0): not translated, its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)).
        // BaseCachingPredicates.affectsBlockState(1): not translated, its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)).
        // BaseCachingPredicates.isValidForMultipass(0): not translated, its class isn't translated (it implements `CachingPredicates` (me.pepperbell.continuity.api.client)).
    }

    /// `BaseCachingPredicates.Factory` (me/pepperbell/continuity/client/processor/BaseCachingPredicates.java)
    pub mod base_caching_predicates_factory {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // BaseCachingPredicates.Factory.Factory(1): not translated, its class isn't translated (it's generic).
        // BaseCachingPredicates.Factory.createPredicates(2): not translated, its class isn't translated (it's generic).
    }

    /// `BaseProcessingPredicate` (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java)
    pub mod base_processing_predicate {
        use super::*;
        // The class itself isn't translated: it implements `ProcessingPredicate`. Its static parts are below.
        // BaseProcessingPredicate.BaseProcessingPredicate(4): not translated, its class isn't translated (it implements `ProcessingPredicate`).
        // BaseProcessingPredicate.shouldProcessQuad(7): not translated, its class isn't translated (it implements `ProcessingPredicate`).
        // BaseProcessingPredicate.fromProperties(1): not translated, it uses BaseProcessingPredicate, which isn't translated (it implements `ProcessingPredicate`).
    }

    /// `BaseProcessingPredicate.BiomeCache` (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java)
    pub mod base_processing_predicate_biome_cache {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Biome`. Its static parts are below.
        // BaseProcessingPredicate.BiomeCache.get(2): not translated, its class isn't translated (it uses Minecraft's `Biome`).
        // BaseProcessingPredicate.BiomeCache.reset(0): not translated, its class isn't translated (it uses Minecraft's `Biome`).
    }

    /// `BaseProcessingPredicate.BlockEntityNameCache` (me/pepperbell/continuity/client/processor/BaseProcessingPredicate.java)
    pub mod base_processing_predicate_block_entity_name_cache {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BlockEntityNameCache {
            pub block_entity_name: Option<String>,
            pub invalid: bool,
        }
        impl BlockEntityNameCache {
            pub fn new() -> Self {
                let mut s = Self { block_entity_name: None, invalid: true };
                s
            }
            // BaseProcessingPredicate.BlockEntityNameCache.get(2): not translated, it uses Minecraft's `BlockAndTintGetter`.
            pub fn reset(&mut self) {
                self.invalid = true;
            }
        }
    }

    /// `CompactCtmQuadProcessor` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java)
    pub mod compact_ctm_quad_processor {
        use super::*;
        // The class itself isn't translated: it extends `AbstractQuadProcessor`. Its static parts are below.
        pub static QUADRANT_INDEX_MAPS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![Vec::new(); 8]);
        // CompactCtmQuadProcessor.CompactCtmQuadProcessor(6): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
        // CompactCtmQuadProcessor.processQuadInner(9): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
        pub fn should_split_uv(signum_a: i32, signum_b: i32) -> bool {
            return (signum_a ^ signum_b) == (-2);
        }
        // CompactCtmQuadProcessor.getSpriteIndex(2): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
        // CompactCtmQuadProcessor.tryInterpolate(3): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
        // CompactCtmQuadProcessor.splitHalf(6): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
        // CompactCtmQuadProcessor.splitQuadrant(6): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
    }

    /// `CompactCtmQuadProcessor.Factory` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java)
    pub mod compact_ctm_quad_processor_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // CompactCtmQuadProcessor.Factory.createProcessor(2): not translated, its class isn't translated (it implements `Factory`).
        // CompactCtmQuadProcessor.Factory.createProcessor(3): not translated, its class isn't translated (it implements `Factory`).
        // CompactCtmQuadProcessor.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
        // CompactCtmQuadProcessor.Factory.getReplacementSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
        // CompactCtmQuadProcessor.Factory.supportsNullSprites(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `CompactCtmQuadProcessor.Vertex` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java)
    pub mod compact_ctm_quad_processor_vertex {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct Vertex {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub color: i32,
            pub light: i32,
            pub u: f32,
            pub v: f32,
            pub has_normal: bool,
            pub normal_x: f32,
            pub normal_y: f32,
            pub normal_z: f32,
        }
        impl Vertex {
            pub fn new() -> Self {
                let mut s = Self { x: 0.0f32, y: 0.0f32, z: 0.0f32, color: 0i32, light: 0i32, u: 0.0f32, v: 0.0f32, has_normal: false, normal_x: 0.0f32, normal_y: 0.0f32, normal_z: 0.0f32 };
                s
            }
            // CompactCtmQuadProcessor.Vertex.readFromQuad(2): not translated, it uses Fabric's `QuadView`.
            // CompactCtmQuadProcessor.Vertex.writeToQuad(2): not translated, it uses Fabric's `MutableQuadView`.
            pub fn set(&mut self, other: &compact_ctm_quad_processor_vertex::Vertex) {
                self.x = other.x;
                self.y = other.y;
                self.z = other.z;
                self.color = other.color;
                self.light = other.light;
                self.u = other.u;
                self.v = other.v;
                self.has_normal = other.has_normal;
                if self.has_normal {
                    self.normal_x = other.normal_x;
                    self.normal_y = other.normal_y;
                    self.normal_z = other.normal_z;
                }
            }
            pub fn set_lerped(&mut self, delta: f32, vertex_a: &compact_ctm_quad_processor_vertex::Vertex, vertex_b: &compact_ctm_quad_processor_vertex::Vertex) {
                self.x = j_lerp(delta, vertex_a.x, vertex_b.x);
                self.y = j_lerp(delta, vertex_a.y, vertex_b.y);
                self.z = j_lerp(delta, vertex_a.z, vertex_b.z);
                self.color = math_util::lerp_color(delta, vertex_a.color, vertex_b.color);
                self.light = math_util::lerp_light(delta, vertex_a.light, vertex_b.light);
                self.u = j_lerp(delta, vertex_a.u, vertex_b.u);
                self.v = j_lerp(delta, vertex_a.v, vertex_b.v);
                if vertex_a.has_normal && vertex_b.has_normal {
                    self.normal_x = j_lerp(delta, vertex_a.normal_x, vertex_b.normal_x);
                    self.normal_y = j_lerp(delta, vertex_a.normal_y, vertex_b.normal_y);
                    self.normal_z = j_lerp(delta, vertex_a.normal_z, vertex_b.normal_z);
                    let sq_length: f32 = ((self.normal_x * self.normal_x) + (self.normal_y * self.normal_y)) + (self.normal_z * self.normal_z);
                    if sq_length != 0.0f32 {
                        let scale: f32 = 1.0f32 / (((sq_length as f64).sqrt()) as f32);
                        self.normal_x *= scale;
                        self.normal_y *= scale;
                        self.normal_z *= scale;
                    }
                }
            }
        }
    }

    /// `CompactCtmQuadProcessor.VertexContainer` (me/pepperbell/continuity/client/processor/CompactCtmQuadProcessor.java)
    pub mod compact_ctm_quad_processor_vertex_container {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct VertexContainer {
            pub vertex0: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex1: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex2: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex3: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex01: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex12: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex23: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex30: compact_ctm_quad_processor_vertex::Vertex,
            pub vertex4: compact_ctm_quad_processor_vertex::Vertex,
            pub lerped_vertices: Vec<compact_ctm_quad_processor_vertex::Vertex>,
        }
        impl VertexContainer {
            // CompactCtmQuadProcessor.VertexContainer.fillBaseVertices(1): not translated, it uses Fabric's `QuadView`.
        }
    }

    /// `ConnectionPredicate` (me/pepperbell/continuity/client/processor/ConnectionPredicate.java)
    pub mod connection_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ConnectionPredicate.shouldConnect(9): not translated, it has no body.
        // ConnectionPredicate.shouldConnect(7): not translated, its class isn't translated (it's an interface).
        // ConnectionPredicate.shouldConnect(8): not translated, its class isn't translated (it's an interface).
    }

    /// `DirectionMaps` (me/pepperbell/continuity/client/processor/DirectionMaps.java)
    pub mod direction_maps {
        use super::*;
        // DIRECTION_MAPS: not translated, it uses Minecraft's `Direction`.
        #[derive(Clone, Debug)]
        pub struct DirectionMaps {
        }
        impl DirectionMaps {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // DirectionMaps.getMap(1): not translated, it uses Minecraft's `Direction`.
        // DirectionMaps.getDirections(3): not translated, it uses Minecraft's `Direction`.
    }

    /// `OrientationMode` (me/pepperbell/continuity/client/processor/OrientationMode.java)
    pub mod orientation_mode {
        use super::*;
        pub static AXIS_ORIENTATIONS: LazyLock<Vec<Vec<i32>>> = LazyLock::new(|| vec![vec![3, 3, 1, 3, 0, 2], vec![0, 0, 0, 0, 0, 0], vec![2, 0, 2, 0, 1, 3]]);
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum OrientationMode {
            None,
            StateAxis,
            Texture,
        }
        impl OrientationMode {
            pub const VALUES: [OrientationMode; 3] = [OrientationMode::None, OrientationMode::StateAxis, OrientationMode::Texture, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    OrientationMode::None => "NONE",
                    OrientationMode::StateAxis => "STATE_AXIS",
                    OrientationMode::Texture => "TEXTURE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => OrientationMode::None,
                    "STATE_AXIS" => OrientationMode::StateAxis,
                    "TEXTURE" => OrientationMode::Texture,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl OrientationMode {
            // OrientationMode.getOrientation(2): not translated, it uses Fabric's `QuadView`.
        }
    }

    /// `OverlayProcessingPredicate` (me/pepperbell/continuity/client/processor/overlay/OverlayProcessingPredicate.java)
    pub mod overlay_processing_predicate {
        use super::*;
        // The class itself isn't translated: it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor). Its static parts are below.
        // OverlayProcessingPredicate.OverlayProcessingPredicate(4): not translated, its class isn't translated (it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor)).
        // OverlayProcessingPredicate.shouldProcessQuad(7): not translated, its class isn't translated (it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor)).
        // OverlayProcessingPredicate.fromProperties(1): not translated, it uses OverlayProcessingPredicate, which isn't translated (it extends `BaseProcessingPredicate` (me.pepperbell.continuity.client.processor)).
    }

    /// `SimpleOverlayQuadProcessor` (me/pepperbell/continuity/client/processor/overlay/SimpleOverlayQuadProcessor.java)
    pub mod simple_overlay_quad_processor {
        use super::*;
        // The class itself isn't translated: it extends `SimpleQuadProcessor` (me.pepperbell.continuity.client.processor.simple). Its static parts are below.
        // SimpleOverlayQuadProcessor.SimpleOverlayQuadProcessor(5): not translated, its class isn't translated (it extends `SimpleQuadProcessor` (me.pepperbell.continuity.client.processor.simple)).
        // SimpleOverlayQuadProcessor.processQuad(9): not translated, its class isn't translated (it extends `SimpleQuadProcessor` (me.pepperbell.continuity.client.processor.simple)).
    }

    /// `SimpleOverlayQuadProcessor.Factory` (me/pepperbell/continuity/client/processor/overlay/SimpleOverlayQuadProcessor.java)
    pub mod simple_overlay_quad_processor_factory {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SimpleOverlayQuadProcessor.Factory.Factory(1): not translated, its class isn't translated (it's generic).
        // SimpleOverlayQuadProcessor.Factory.createProcessor(2): not translated, its class isn't translated (it's generic).
        // SimpleOverlayQuadProcessor.Factory.supportsNullSprites(1): not translated, its class isn't translated (it's generic).
    }

    /// `StandardOverlayQuadProcessor` (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java)
    pub mod standard_overlay_quad_processor {
        use super::*;
        // The class itself isn't translated: it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor). Its static parts are below.
        // StandardOverlayQuadProcessor.StandardOverlayQuadProcessor(10): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.processQuadInner(9): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.matchesAny(2): not translated, it uses Minecraft's `TextureAtlasSprite`.
        // StandardOverlayQuadProcessor.appliesOverlay(9): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.hasSameOverlay(2): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.appliesOverlayCorner(9): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.fromTwoSidesAdj(14): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.fromOneSide(17): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.getCollector(1): not translated, it uses ProcessingDataProvider, which isn't translated (it's an interface).
        // StandardOverlayQuadProcessor.prepareCollector(2): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.prepareCollector(3): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.getSprites(8): not translated, its class isn't translated (it extends `AbstractQuadProcessor` (me.pepperbell.continuity.client.processor)).
    }

    /// `StandardOverlayQuadProcessor.Factory` (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java)
    pub mod standard_overlay_quad_processor_factory {
        use super::*;
        // The class itself isn't translated: it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor). Its static parts are below.
        // StandardOverlayQuadProcessor.Factory.createProcessor(2): not translated, its class isn't translated (it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.Factory.getSpriteAmount(1): not translated, its class isn't translated (it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor)).
        // StandardOverlayQuadProcessor.Factory.supportsNullSprites(1): not translated, its class isn't translated (it extends `AbstractQuadProcessorFactory` (me.pepperbell.continuity.client.processor)).
    }

    /// `StandardOverlayQuadProcessor.SpriteCollector` (me/pepperbell/continuity/client/processor/overlay/StandardOverlayQuadProcessor.java)
    pub mod standard_overlay_quad_processor_sprite_collector {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `TextureAtlasSprite`. Its static parts are below.
        // EMPTY_SPRITES: not translated, it uses Minecraft's `TextureAtlasSprite`.
        // StandardOverlayQuadProcessor.SpriteCollector.add(1): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
        // StandardOverlayQuadProcessor.SpriteCollector.clear(0): not translated, its class isn't translated (it uses Minecraft's `TextureAtlasSprite`).
    }

    /// `ProcessingDataKeys` (me/pepperbell/continuity/client/processor/ProcessingDataKeys.java)
    pub mod processing_data_keys {
        use super::*;
        // MUTABLE_POS: not translated, it uses ProcessingDataKey, which isn't translated (it's an interface).
        // BIOME_CACHE: not translated, it uses ProcessingDataKey, which isn't translated (it's an interface).
        // BLOCK_ENTITY_NAME_CACHE: not translated, it uses ProcessingDataKey, which isn't translated (it's an interface).
        // VERTEX_CONTAINER: not translated, it uses ProcessingDataKey, which isn't translated (it's an interface).
        // SPRITE_COLLECTOR: not translated, it uses ProcessingDataKey, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct ProcessingDataKeys {
        }
        impl ProcessingDataKeys {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ProcessingDataKeys.create(2): not translated, it's generic.
        // ProcessingDataKeys.create(3): not translated, it's generic.
        pub fn init() {
        }
    }

    /// `ProcessingPredicate` (me/pepperbell/continuity/client/processor/ProcessingPredicate.java)
    pub mod processing_predicate {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ProcessingPredicate.shouldProcessQuad(7): not translated, it has no body.
    }

    /// `CtmSpriteProvider` (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java)
    pub mod ctm_sprite_provider {
        use super::*;
        // The class itself isn't translated: it implements `SpriteProvider`. Its static parts are below.
        pub static SPRITE_INDEX_MAP: LazyLock<Vec<i32>> = LazyLock::new(|| vec![0, 3, 0, 3, 12, 5, 12, 15, 0, 3, 0, 3, 12, 5, 12, 15, 1, 2, 1, 2, 4, 7, 4, 29, 1, 2, 1, 2, 13, 31, 13, 14, 0, 3, 0, 3, 12, 5, 12, 15, 0, 3, 0, 3, 12, 5, 12, 15, 1, 2, 1, 2, 4, 7, 4, 29, 1, 2, 1, 2, 13, 31, 13, 14, 36, 17, 36, 17, 24, 19, 24, 43, 36, 17, 36, 17, 24, 19, 24, 43, 16, 18, 16, 18, 6, 46, 6, 21, 16, 18, 16, 18, 28, 9, 28, 22, 36, 17, 36, 17, 24, 19, 24, 43, 36, 17, 36, 17, 24, 19, 24, 43, 37, 40, 37, 40, 30, 8, 30, 34, 37, 40, 37, 40, 25, 23, 25, 45, 0, 3, 0, 3, 12, 5, 12, 15, 0, 3, 0, 3, 12, 5, 12, 15, 1, 2, 1, 2, 4, 7, 4, 29, 1, 2, 1, 2, 13, 31, 13, 14, 0, 3, 0, 3, 12, 5, 12, 15, 0, 3, 0, 3, 12, 5, 12, 15, 1, 2, 1, 2, 4, 7, 4, 29, 1, 2, 1, 2, 13, 31, 13, 14, 36, 39, 36, 39, 24, 41, 24, 27, 36, 39, 36, 39, 24, 41, 24, 27, 16, 42, 16, 42, 6, 20, 6, 10, 16, 42, 16, 42, 28, 35, 28, 44, 36, 39, 36, 39, 24, 41, 24, 27, 36, 39, 36, 39, 24, 41, 24, 27, 37, 38, 37, 38, 30, 11, 30, 32, 37, 38, 37, 38, 25, 33, 25, 26]);
        // CtmSpriteProvider.CtmSpriteProvider(4): not translated, its class isn't translated (it implements `SpriteProvider`).
        // CtmSpriteProvider.getSprite(8): not translated, its class isn't translated (it implements `SpriteProvider`).
        // CtmSpriteProvider.getConnections(10): not translated, it uses Minecraft's `Direction`.
    }

    /// `CtmSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/CtmSpriteProvider.java)
    pub mod ctm_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // CtmSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // CtmSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `FixedSpriteProvider` (me/pepperbell/continuity/client/processor/simple/FixedSpriteProvider.java)
    pub mod fixed_sprite_provider {
        use super::*;
        // The class itself isn't translated: it implements `SpriteProvider`. Its static parts are below.
        // FixedSpriteProvider.FixedSpriteProvider(1): not translated, its class isn't translated (it implements `SpriteProvider`).
        // FixedSpriteProvider.getSprite(8): not translated, its class isn't translated (it implements `SpriteProvider`).
    }

    /// `FixedSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/FixedSpriteProvider.java)
    pub mod fixed_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // FixedSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // FixedSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `HorizontalSpriteProvider` (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java)
    pub mod horizontal_sprite_provider {
        use super::*;
        // The class itself isn't translated: it implements `SpriteProvider`. Its static parts are below.
        pub static SPRITE_INDEX_MAP: LazyLock<Vec<i32>> = LazyLock::new(|| vec![3, 2, 0, 1]);
        // HorizontalSpriteProvider.HorizontalSpriteProvider(4): not translated, its class isn't translated (it implements `SpriteProvider`).
        // HorizontalSpriteProvider.getSprite(8): not translated, its class isn't translated (it implements `SpriteProvider`).
        // HorizontalSpriteProvider.getConnections(8): not translated, its class isn't translated (it implements `SpriteProvider`).
    }

    /// `HorizontalSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/HorizontalSpriteProvider.java)
    pub mod horizontal_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // HorizontalSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // HorizontalSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `HorizontalVerticalSpriteProvider` (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java)
    pub mod horizontal_vertical_sprite_provider {
        use super::*;
        // The class itself isn't translated: it extends `HorizontalSpriteProvider`. Its static parts are below.
        pub static SECONDARY_SPRITE_INDEX_MAP: LazyLock<Vec<i32>> = LazyLock::new(|| vec![3, 3, 6, 3, 3, 3, 3, 3, 3, 3, 6, 3, 3, 3, 3, 3, 4, 4, 5, 4, 4, 4, 4, 4, 3, 3, 6, 3, 3, 3, 3, 3, 3, 3, 6, 3, 3, 3, 3, 3, 3, 3, 6, 3, 3, 3, 3, 3, 3, 3, 6, 3, 3, 3, 3, 3, 3, 3, 6, 3, 3, 3, 3, 3]);
        // HorizontalVerticalSpriteProvider.HorizontalVerticalSpriteProvider(4): not translated, its class isn't translated (it extends `HorizontalSpriteProvider`).
        // HorizontalVerticalSpriteProvider.getSprite(8): not translated, its class isn't translated (it extends `HorizontalSpriteProvider`).
        // HorizontalVerticalSpriteProvider.getSecondaryConnections(8): not translated, its class isn't translated (it extends `HorizontalSpriteProvider`).
    }

    /// `HorizontalVerticalSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/HorizontalVerticalSpriteProvider.java)
    pub mod horizontal_vertical_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // HorizontalVerticalSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // HorizontalVerticalSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `RandomSpriteProvider` (me/pepperbell/continuity/client/processor/simple/RandomSpriteProvider.java)
    pub mod random_sprite_provider {
        use super::*;
        // The class itself isn't translated: it implements `SpriteProvider`. Its static parts are below.
        // RandomSpriteProvider.RandomSpriteProvider(5): not translated, its class isn't translated (it implements `SpriteProvider`).
        // RandomSpriteProvider.getSprite(8): not translated, its class isn't translated (it implements `SpriteProvider`).
    }

    /// `RandomSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/RandomSpriteProvider.java)
    pub mod random_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // RandomSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // RandomSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `RepeatSpriteProvider` (me/pepperbell/continuity/client/processor/simple/RepeatSpriteProvider.java)
    pub mod repeat_sprite_provider {
        use super::*;
        // The class itself isn't translated: it implements `SpriteProvider`. Its static parts are below.
        // RepeatSpriteProvider.RepeatSpriteProvider(5): not translated, its class isn't translated (it implements `SpriteProvider`).
        // RepeatSpriteProvider.getSprite(8): not translated, its class isn't translated (it implements `SpriteProvider`).
    }

    /// `RepeatSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/RepeatSpriteProvider.java)
    pub mod repeat_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // RepeatSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // RepeatSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `SimpleQuadProcessor` (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java)
    pub mod simple_quad_processor {
        use super::*;
        // The class itself isn't translated: it implements `QuadProcessor` (me.pepperbell.continuity.api.client). Its static parts are below.
        // SimpleQuadProcessor.SimpleQuadProcessor(2): not translated, its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)).
        // SimpleQuadProcessor.processQuad(9): not translated, its class isn't translated (it implements `QuadProcessor` (me.pepperbell.continuity.api.client)).
        // SimpleQuadProcessor.process(3): not translated, it uses Fabric's `MutableQuadView`.
    }

    /// `SimpleQuadProcessor.Factory` (me/pepperbell/continuity/client/processor/simple/SimpleQuadProcessor.java)
    pub mod simple_quad_processor_factory {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // SimpleQuadProcessor.Factory.Factory(1): not translated, its class isn't translated (it's generic).
        // SimpleQuadProcessor.Factory.createProcessor(2): not translated, its class isn't translated (it's generic).
        // SimpleQuadProcessor.Factory.getSpriteAmount(1): not translated, its class isn't translated (it's generic).
    }

    /// `SpriteProvider` (me/pepperbell/continuity/client/processor/simple/SpriteProvider.java)
    pub mod sprite_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpriteProvider.getSprite(8): not translated, it has no body.
    }

    /// `SpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/SpriteProvider.java)
    pub mod sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpriteProvider.Factory.createSpriteProvider(2): not translated, it has no body.
        // SpriteProvider.Factory.getSpriteAmount(1): not translated, it has no body.
    }

    /// `VerticalHorizontalSpriteProvider` (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java)
    pub mod vertical_horizontal_sprite_provider {
        use super::*;
        // The class itself isn't translated: it extends `VerticalSpriteProvider`. Its static parts are below.
        pub static SECONDARY_SPRITE_INDEX_MAP: LazyLock<Vec<i32>> = LazyLock::new(|| vec![3, 6, 3, 3, 3, 6, 3, 3, 4, 5, 4, 4, 3, 6, 3, 3, 3, 6, 3, 3, 3, 6, 3, 3, 3, 6, 3, 3, 3, 6, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 4, 4, 4, 4, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]);
        // VerticalHorizontalSpriteProvider.VerticalHorizontalSpriteProvider(4): not translated, its class isn't translated (it extends `VerticalSpriteProvider`).
        // VerticalHorizontalSpriteProvider.getSprite(8): not translated, its class isn't translated (it extends `VerticalSpriteProvider`).
        // VerticalHorizontalSpriteProvider.getSecondaryConnections(8): not translated, its class isn't translated (it extends `VerticalSpriteProvider`).
    }

    /// `VerticalHorizontalSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/VerticalHorizontalSpriteProvider.java)
    pub mod vertical_horizontal_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // VerticalHorizontalSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // VerticalHorizontalSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `VerticalSpriteProvider` (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java)
    pub mod vertical_sprite_provider {
        use super::*;
        // The class itself isn't translated: it implements `SpriteProvider`. Its static parts are below.
        pub static SPRITE_INDEX_MAP: LazyLock<Vec<i32>> = LazyLock::new(|| vec![3, 2, 0, 1]);
        // VerticalSpriteProvider.VerticalSpriteProvider(4): not translated, its class isn't translated (it implements `SpriteProvider`).
        // VerticalSpriteProvider.getSprite(8): not translated, its class isn't translated (it implements `SpriteProvider`).
        // VerticalSpriteProvider.getConnections(8): not translated, its class isn't translated (it implements `SpriteProvider`).
    }

    /// `VerticalSpriteProvider.Factory` (me/pepperbell/continuity/client/processor/simple/VerticalSpriteProvider.java)
    pub mod vertical_sprite_provider_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // VerticalSpriteProvider.Factory.createSpriteProvider(2): not translated, its class isn't translated (it implements `Factory`).
        // VerticalSpriteProvider.Factory.getSpriteAmount(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `Symmetry` (me/pepperbell/continuity/client/processor/Symmetry.java)
    pub mod symmetry {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum Symmetry {
            None,
            Opposite,
            All,
        }
        impl Symmetry {
            pub const VALUES: [Symmetry; 3] = [Symmetry::None, Symmetry::Opposite, Symmetry::All, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    Symmetry::None => "NONE",
                    Symmetry::Opposite => "OPPOSITE",
                    Symmetry::All => "ALL",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => Symmetry::None,
                    "OPPOSITE" => Symmetry::Opposite,
                    "ALL" => Symmetry::All,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
        impl Symmetry {
            // Symmetry.apply(1): not translated, it uses Minecraft's `Direction`.
        }
    }

    /// `TopQuadProcessor` (me/pepperbell/continuity/client/processor/TopQuadProcessor.java)
    pub mod top_quad_processor {
        use super::*;
        // The class itself isn't translated: it extends `AbstractQuadProcessor`. Its static parts are below.
        // TopQuadProcessor.TopQuadProcessor(4): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
        // TopQuadProcessor.processQuadInner(9): not translated, its class isn't translated (it extends `AbstractQuadProcessor`).
    }

    /// `TopQuadProcessor.Factory` (me/pepperbell/continuity/client/processor/TopQuadProcessor.java)
    pub mod top_quad_processor_factory {
        use super::*;
        // The class itself isn't translated: it extends `AbstractQuadProcessorFactory`. Its static parts are below.
        // TopQuadProcessor.Factory.createProcessor(2): not translated, its class isn't translated (it extends `AbstractQuadProcessorFactory`).
        // TopQuadProcessor.Factory.getSpriteAmount(1): not translated, its class isn't translated (it extends `AbstractQuadProcessorFactory`).
    }

    /// `BaseCtmProperties` (me/pepperbell/continuity/client/properties/BaseCtmProperties.java)
    pub mod base_ctm_properties {
        use super::*;
        // The class itself isn't translated: it implements `CtmProperties` (me.pepperbell.continuity.api.client). Its static parts are below.
        pub static SPECIAL_SKIP_ID: LazyLock<JIdentifier> = LazyLock::new(|| continuity_client::as_id("special/skip"));
        pub static SPECIAL_DEFAULT_ID: LazyLock<JIdentifier> = LazyLock::new(|| continuity_client::as_id("special/default"));
        // DIRECTION_AMOUNT: not translated, it calls Minecraft's `Direction`.values.
        // BaseCtmProperties.BaseCtmProperties(6): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getSpriteDependencies(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.compareTo(1): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.init(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseMatchTiles(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseMatchBlocks(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.detectMatches(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.validateMatches(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseTiles(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseFaces(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseBiomes(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseHeights(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseLegacyHeights(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseName(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parsePrioritize(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.parseResourceCondition(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.isValid(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getProperties(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getResourceId(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getPackId(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getPackPriority(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getMethod(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getMatchTilesSet(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getMatchBlocksPredicate(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getSpriteIds(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getFaces(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getBiomePredicate(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getHeightPredicate(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.getBlockEntityNamePredicate(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.isPrioritized(0): not translated, its class isn't translated (it implements `CtmProperties` (me.pepperbell.continuity.api.client)).
        // BaseCtmProperties.wrapFactory(1): not translated, it's generic.
    }

    /// `BasicConnectingCtmProperties` (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java)
    pub mod basic_connecting_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `BaseCtmProperties`. Its static parts are below.
        // BasicConnectingCtmProperties.BasicConnectingCtmProperties(6): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // BasicConnectingCtmProperties.init(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // BasicConnectingCtmProperties.parseConnect(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // BasicConnectingCtmProperties.detectConnect(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // BasicConnectingCtmProperties.validateConnect(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // BasicConnectingCtmProperties.getConnectionPredicate(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
    }

    /// `BasicConnectingCtmProperties.ConnectionType` (me/pepperbell/continuity/client/properties/BasicConnectingCtmProperties.java)
    pub mod basic_connecting_ctm_properties_connection_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
    }

    /// `CompactConnectingCtmProperties` (me/pepperbell/continuity/client/properties/CompactConnectingCtmProperties.java)
    pub mod compact_connecting_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `OrientedConnectingCtmProperties`. Its static parts are below.
        // CompactConnectingCtmProperties.CompactConnectingCtmProperties(7): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties`).
        // CompactConnectingCtmProperties.CompactConnectingCtmProperties(6): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties`).
        // CompactConnectingCtmProperties.init(0): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties`).
        // CompactConnectingCtmProperties.parseTileReplacements(0): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties`).
        // CompactConnectingCtmProperties.getTileReplacementMap(0): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties`).
    }

    /// `ConnectingCtmProperties` (me/pepperbell/continuity/client/properties/ConnectingCtmProperties.java)
    pub mod connecting_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `BasicConnectingCtmProperties`. Its static parts are below.
        // ConnectingCtmProperties.ConnectingCtmProperties(6): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties`).
        // ConnectingCtmProperties.init(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties`).
        // ConnectingCtmProperties.parseInnerSeams(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties`).
        // ConnectingCtmProperties.getInnerSeams(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties`).
    }

    /// `OrientedConnectingCtmProperties` (me/pepperbell/continuity/client/properties/OrientedConnectingCtmProperties.java)
    pub mod oriented_connecting_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `ConnectingCtmProperties`. Its static parts are below.
        // OrientedConnectingCtmProperties.OrientedConnectingCtmProperties(7): not translated, its class isn't translated (it extends `ConnectingCtmProperties`).
        // OrientedConnectingCtmProperties.OrientedConnectingCtmProperties(6): not translated, its class isn't translated (it extends `ConnectingCtmProperties`).
        // OrientedConnectingCtmProperties.init(0): not translated, its class isn't translated (it extends `ConnectingCtmProperties`).
        // OrientedConnectingCtmProperties.parseOrient(0): not translated, its class isn't translated (it extends `ConnectingCtmProperties`).
        // OrientedConnectingCtmProperties.getOrientationMode(0): not translated, its class isn't translated (it extends `ConnectingCtmProperties`).
    }

    /// `BaseOverlayCtmProperties` (me/pepperbell/continuity/client/properties/overlay/BaseOverlayCtmProperties.java)
    pub mod base_overlay_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties). Its static parts are below.
        // BaseOverlayCtmProperties.BaseOverlayCtmProperties(6): not translated, its class isn't translated (it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties)).
        // BaseOverlayCtmProperties.init(0): not translated, its class isn't translated (it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties)).
        // BaseOverlayCtmProperties.getOverlayPropertiesSection(0): not translated, its class isn't translated (it extends `BaseCtmProperties` (me.pepperbell.continuity.client.properties)).
    }

    /// `OrientedConnectingOverlayCtmProperties` (me/pepperbell/continuity/client/properties/overlay/OrientedConnectingOverlayCtmProperties.java)
    pub mod oriented_connecting_overlay_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties). Its static parts are below.
        // OrientedConnectingOverlayCtmProperties.OrientedConnectingOverlayCtmProperties(7): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // OrientedConnectingOverlayCtmProperties.OrientedConnectingOverlayCtmProperties(6): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // OrientedConnectingOverlayCtmProperties.init(0): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // OrientedConnectingOverlayCtmProperties.getOverlayPropertiesSection(0): not translated, its class isn't translated (it extends `OrientedConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
    }

    /// `OverlayPropertiesSection` (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java)
    pub mod overlay_properties_section {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `BlockState`. Its static parts are below.
        // OverlayPropertiesSection.OverlayPropertiesSection(3): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayPropertiesSection.init(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayPropertiesSection.parseTintIndex(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayPropertiesSection.parseTintBlock(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayPropertiesSection.parseLayer(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayPropertiesSection.getTintIndex(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayPropertiesSection.getTintBlock(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
        // OverlayPropertiesSection.getLayer(0): not translated, its class isn't translated (it uses Minecraft's `BlockState`).
    }

    /// `OverlayPropertiesSection.Provider` (me/pepperbell/continuity/client/properties/overlay/OverlayPropertiesSection.java)
    pub mod overlay_properties_section_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // OverlayPropertiesSection.Provider.getOverlayPropertiesSection(0): not translated, it has no body.
    }

    /// `RandomOverlayCtmProperties` (me/pepperbell/continuity/client/properties/overlay/RandomOverlayCtmProperties.java)
    pub mod random_overlay_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties). Its static parts are below.
        // RandomOverlayCtmProperties.RandomOverlayCtmProperties(6): not translated, its class isn't translated (it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties)).
        // RandomOverlayCtmProperties.init(0): not translated, its class isn't translated (it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties)).
        // RandomOverlayCtmProperties.getOverlayPropertiesSection(0): not translated, its class isn't translated (it extends `RandomCtmProperties` (me.pepperbell.continuity.client.properties)).
    }

    /// `RepeatOverlayCtmProperties` (me/pepperbell/continuity/client/properties/overlay/RepeatOverlayCtmProperties.java)
    pub mod repeat_overlay_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties). Its static parts are below.
        // RepeatOverlayCtmProperties.RepeatOverlayCtmProperties(6): not translated, its class isn't translated (it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties)).
        // RepeatOverlayCtmProperties.init(0): not translated, its class isn't translated (it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties)).
        // RepeatOverlayCtmProperties.getOverlayPropertiesSection(0): not translated, its class isn't translated (it extends `RepeatCtmProperties` (me.pepperbell.continuity.client.properties)).
    }

    /// `StandardOverlayCtmProperties` (me/pepperbell/continuity/client/properties/overlay/StandardOverlayCtmProperties.java)
    pub mod standard_overlay_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties). Its static parts are below.
        // StandardOverlayCtmProperties.StandardOverlayCtmProperties(6): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // StandardOverlayCtmProperties.init(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // StandardOverlayCtmProperties.getOverlayPropertiesSection(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // StandardOverlayCtmProperties.parseConnectTiles(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // StandardOverlayCtmProperties.parseConnectBlocks(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // StandardOverlayCtmProperties.getConnectTilesSet(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
        // StandardOverlayCtmProperties.getConnectBlocksPredicate(0): not translated, its class isn't translated (it extends `BasicConnectingCtmProperties` (me.pepperbell.continuity.client.properties)).
    }

    /// `PropertiesParsingHelper` (me/pepperbell/continuity/client/properties/PropertiesParsingHelper.java)
    pub mod properties_parsing_helper {
        use super::*;
        // EMPTY_BLOCK_STATE_PREDICATE: not translated, it uses Minecraft's `BlockState`.
        #[derive(Clone, Debug)]
        pub struct PropertiesParsingHelper {
        }
        impl PropertiesParsingHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PropertiesParsingHelper.parseMatchTiles(4): not translated, it calls `FilenameUtils` (org.apache.commons.io).getPath.
        // PropertiesParsingHelper.parseBlockStates(4): not translated, its body couldn't be read: a labelled statement.
        pub fn parse_symmetry(properties: JProps, property_key: &str, file_location: &JIdentifier, pack_id: &str) -> Option<symmetry::Symmetry> {
            let symmetry_str: Option<String> = properties.get_property(&property_key);
            if symmetry_str.is_none() {
                return None;
            }
            let result_0 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> std::io::Result<Option<Option<symmetry::Symmetry>>> {
                return Ok(Some(Some(symmetry::Symmetry::value_of(&(((symmetry_str.as_deref().expect("null")).trim()).to_uppercase())))));
                Ok(None)
            })).unwrap_or_else(|_| Err(std::io::Error::other("a runtime exception")));
            match result_0 {
                Ok(Some(v)) => return v,
                Ok(None) => {}
                Err(_) => {
                    // log:
                    return None;
                }
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn parse_orientation_mode(properties: JProps, property_key: &str, file_location: &JIdentifier, pack_id: &str) -> Option<orientation_mode::OrientationMode> {
            let orientation_mode_str: Option<String> = properties.get_property(&property_key);
            if orientation_mode_str.is_none() {
                return None;
            }
            let result_0 = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| -> std::io::Result<Option<Option<orientation_mode::OrientationMode>>> {
                return Ok(Some(Some(orientation_mode::OrientationMode::value_of(&(((orientation_mode_str.as_deref().expect("null")).trim()).to_uppercase())))));
                Ok(None)
            })).unwrap_or_else(|_| Err(std::io::Error::other("a runtime exception")));
            match result_0 {
                Ok(Some(v)) => return v,
                Ok(None) => {}
                Err(_) => {
                    // log:
                    return None;
                }
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn parse_optifine_only(properties: JProps, file_location: &JIdentifier) -> bool {
            if !(j_eq(&file_location.namespace.clone(), "minecraft")) {
                return false;
            }
            let optifine_only_str: Option<String> = properties.get_property("optifineOnly");
            return if optifine_only_str.is_none() { false } else { j_parse_bool(&((optifine_only_str.as_deref().expect("null")).trim())) };
        }
    }

    /// `RandomCtmProperties` (me/pepperbell/continuity/client/properties/RandomCtmProperties.java)
    pub mod random_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `BaseCtmProperties`. Its static parts are below.
        // RandomCtmProperties.RandomCtmProperties(6): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.init(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.parseWeights(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.parseRandomLoops(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.parseSymmetry(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.parseLinked(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.getIndexProviderFactory(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.getRandomLoops(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.getSymmetry(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RandomCtmProperties.getLinked(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
    }

    /// `RepeatCtmProperties` (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java)
    pub mod repeat_ctm_properties {
        use super::*;
        // The class itself isn't translated: it extends `BaseCtmProperties`. Its static parts are below.
        // RepeatCtmProperties.RepeatCtmProperties(6): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.init(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.parseWidth(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.parseHeight(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.parseSymmetry(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.parseOrient(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.getWidth(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.getHeight(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.getSymmetry(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
        // RepeatCtmProperties.getOrientationMode(0): not translated, its class isn't translated (it extends `BaseCtmProperties`).
    }

    /// `RepeatCtmProperties.Validator` (me/pepperbell/continuity/client/properties/RepeatCtmProperties.java)
    pub mod repeat_ctm_properties_validator {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // RepeatCtmProperties.Validator.validateTileAmount(2): not translated, its class isn't translated (it's generic).
    }

    /// `TileAmountValidator` (me/pepperbell/continuity/client/properties/TileAmountValidator.java)
    pub mod tile_amount_validator {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // TileAmountValidator.validateTileAmount(2): not translated, it has no body.
        // TileAmountValidator.wrapFactory(2): not translated, it's generic.
    }

    /// `TileAmountValidator.AtLeast` (me/pepperbell/continuity/client/properties/TileAmountValidator.java)
    pub mod tile_amount_validator_at_least {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TileAmountValidator.AtLeast.AtLeast(1): not translated, its class isn't translated (it's generic).
        // TileAmountValidator.AtLeast.validateTileAmount(2): not translated, its class isn't translated (it's generic).
    }

    /// `TileAmountValidator.Exactly` (me/pepperbell/continuity/client/properties/TileAmountValidator.java)
    pub mod tile_amount_validator_exactly {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // TileAmountValidator.Exactly.Exactly(1): not translated, its class isn't translated (it's generic).
        // TileAmountValidator.Exactly.validateTileAmount(2): not translated, its class isn't translated (it's generic).
    }

    /// `CtmPropertiesLoader` (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java)
    pub mod ctm_properties_loader {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ResourceManager`. Its static parts are below.
        // CtmPropertiesLoader.CtmPropertiesLoader(1): not translated, its class isn't translated (it uses Minecraft's `ResourceManager`).
        // CtmPropertiesLoader.loadAllWithState(1): not translated, it uses Minecraft's `ResourceManager`.
        // CtmPropertiesLoader.loadAll(1): not translated, it uses Minecraft's `ResourceManager`.
        // CtmPropertiesLoader.loadAll(0): not translated, its class isn't translated (it uses Minecraft's `ResourceManager`).
        // CtmPropertiesLoader.loadAll(2): not translated, its class isn't translated (it uses Minecraft's `ResourceManager`).
        // CtmPropertiesLoader.load(4): not translated, its class isn't translated (it uses Minecraft's `ResourceManager`).
        // CtmPropertiesLoader.load(6): not translated, it's generic.
    }

    /// `CtmPropertiesLoader.LoadingContainer` (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java)
    pub mod ctm_properties_loader_loading_container {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // CtmPropertiesLoader.LoadingContainer.LoadingContainer(2): not translated, its class isn't translated (it's generic).
        // CtmPropertiesLoader.LoadingContainer.toProcessorHolder(1): not translated, its class isn't translated (it's generic).
        // CtmPropertiesLoader.LoadingContainer.compareTo(1): not translated, its class isn't translated (it's generic).
        // CtmPropertiesLoader.LoadingContainer.loader(0): not translated, its class isn't translated (it's generic).
        // CtmPropertiesLoader.LoadingContainer.properties(0): not translated, its class isn't translated (it's generic).
    }

    /// `CtmPropertiesLoader.LoadingResult` (me/pepperbell/continuity/client/resource/CtmPropertiesLoader.java)
    pub mod ctm_properties_loader_loading_result {
        use super::*;
        // The class itself isn't translated: it uses LoadingContainer, which isn't translated (it's generic). Its static parts are below.
        // CtmPropertiesLoader.LoadingResult.LoadingResult(2): not translated, its class isn't translated (it uses LoadingContainer, which isn't translated (it's generic)).
        // CtmPropertiesLoader.LoadingResult.createProcessorHolders(1): not translated, its class isn't translated (it uses LoadingContainer, which isn't translated (it's generic)).
        // CtmPropertiesLoader.LoadingResult.getBlockAtlasSpriteDependencies(0): not translated, its class isn't translated (it uses LoadingContainer, which isn't translated (it's generic)).
    }

    /// `CtmResourceReloader` (me/pepperbell/continuity/client/resource/CtmResourceReloader.java)
    pub mod ctm_resource_reloader {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `PreparableReloadListener`. Its static parts are below.
        // BLOCK_ATLAS_EXTRA_IDS_FUTURE_KEY: not translated, it uses Minecraft's `StateKey`.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| continuity_client::as_id("ctm"));
        // INSTANCE: not translated, it uses CtmResourceReloader, which isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // CtmResourceReloader.prepareSharedState(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // CtmResourceReloader.reload(4): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
        // CtmResourceReloader.apply(1): not translated, its class isn't translated (it implements Minecraft's `PreparableReloadListener`).
    }

    /// `CuboidItemModelWrapperInitContext` (me/pepperbell/continuity/client/resource/CuboidItemModelWrapperInitContext.java)
    pub mod cuboid_item_model_wrapper_init_context {
        use super::*;
        // SPRITE_FINDER_GETTER: not translated, it uses `ThreadLocal`.
        #[derive(Clone, Debug)]
        pub struct CuboidItemModelWrapperInitContext {
        }
        impl CuboidItemModelWrapperInitContext {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EmissiveSuffixLoader` (me/pepperbell/continuity/client/resource/EmissiveSuffixLoader.java)
    pub mod emissive_suffix_loader {
        use super::*;
        pub static LOCATION: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "optifine/emissive.properties"));
        pub static EMISSIVE_SUFFIX: LazyLock<JCell<Option<String>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct EmissiveSuffixLoader {
        }
        impl EmissiveSuffixLoader {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn get_emissive_suffix() -> Option<String> {
            return emissive_suffix_loader::EMISSIVE_SUFFIX.get();
        }
        // EmissiveSuffixLoader.load(1): not translated, it uses Minecraft's `ResourceManager`.
    }

    /// `ModelWrappingHandler` (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java)
    pub mod model_wrapping_handler {
        use super::*;
        // WRAP_CTM_FUTURE_KEY: not translated, it uses Minecraft's `StateKey`.
        // WRAP_EMISSIVE_FUTURE_KEY: not translated, it uses Minecraft's `StateKey`.
        #[derive(Clone, Debug)]
        pub struct ModelWrappingHandler {
        }
        impl ModelWrappingHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ModelWrappingHandler.init(0): not translated, a lambda.
    }

    /// `ModelWrappingHandler.Data` (me/pepperbell/continuity/client/resource/ModelWrappingHandler.java)
    pub mod model_wrapping_handler_data {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct Data {
            pub wrap_ctm: bool,
            pub wrap_emissive: bool,
        }
        impl Data {
            pub fn new(wrap_ctm: bool, wrap_emissive: bool) -> Self {
                let mut s = Self { wrap_ctm: wrap_ctm, wrap_emissive: wrap_emissive };
                s.wrap_ctm = wrap_ctm;
                s.wrap_emissive = wrap_emissive;
                s
            }
            pub fn wrap_ctm(&self) -> bool {
                return self.wrap_ctm;
            }
            pub fn wrap_emissive(&self) -> bool {
                return self.wrap_emissive;
            }
        }
    }

    /// `ResourceRedirectHandler` (me/pepperbell/continuity/client/resource/ResourceRedirectHandler.java)
    pub mod resource_redirect_handler {
        use super::*;
        pub const SPRITE_PATH_START: &str = "continuity_reserved/";
        pub const PATH_START: &str = "textures/continuity_reserved/";
        pub static PATH_START_LENGTH: LazyLock<i32> = LazyLock::new(|| j_len("textures/continuity_reserved/"));
        #[derive(Clone, Debug)]
        pub struct ResourceRedirectHandler {
        }
        impl ResourceRedirectHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn redirect(id: &JIdentifier) -> JIdentifier {
            let path: String = id.path.clone();
            return if !(path.starts_with("textures/continuity_reserved/")) { id.clone() } else { JIdentifier::new(&id.namespace, &(format!("optifine/{}", j_substring(&path, *resource_redirect_handler::PATH_START_LENGTH, None)))) };
        }
    }

    /// `SpriteLoaderLoadContext` (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContext.java)
    pub mod sprite_loader_load_context {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpriteLoaderLoadContext.getExtraIdsFuture(1): not translated, it has no body.
        // SpriteLoaderLoadContext.getEmissiveControl(1): not translated, it has no body.
    }

    /// `SpriteLoaderLoadContext.EmissiveControl` (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContext.java)
    pub mod sprite_loader_load_context_emissive_control {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpriteLoaderLoadContext.EmissiveControl.getEmissiveIdMap(0): not translated, it has no body.
        // SpriteLoaderLoadContext.EmissiveControl.setEmissiveIdMap(1): not translated, it has no body.
        // SpriteLoaderLoadContext.EmissiveControl.setHasEmissives(1): not translated, it has no body.
    }

    /// `SpriteLoaderLoadContextImpl` (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java)
    pub mod sprite_loader_load_context_impl {
        use super::*;
        // The class itself isn't translated: it implements `SpriteLoaderLoadContext`. Its static parts are below.
        // SpriteLoaderLoadContextImpl.SpriteLoaderLoadContextImpl(2): not translated, its class isn't translated (it implements `SpriteLoaderLoadContext`).
        // SpriteLoaderLoadContextImpl.getExtraIdsFuture(1): not translated, its class isn't translated (it implements `SpriteLoaderLoadContext`).
        // SpriteLoaderLoadContextImpl.getEmissiveControl(1): not translated, its class isn't translated (it implements `SpriteLoaderLoadContext`).
    }

    /// `SpriteLoaderLoadContextImpl.EmissiveControlImpl` (me/pepperbell/continuity/client/resource/SpriteLoaderLoadContextImpl.java)
    pub mod sprite_loader_load_context_impl_emissive_control_impl {
        use super::*;
        // The class itself isn't translated: it implements `EmissiveControl`. Its static parts are below.
        // SpriteLoaderLoadContextImpl.EmissiveControlImpl.EmissiveControlImpl(1): not translated, its class isn't translated (it implements `EmissiveControl`).
        // SpriteLoaderLoadContextImpl.EmissiveControlImpl.EmissiveControlImpl(0): not translated, its class isn't translated (it implements `EmissiveControl`).
        // SpriteLoaderLoadContextImpl.EmissiveControlImpl.getEmissiveIdMap(0): not translated, its class isn't translated (it implements `EmissiveControl`).
        // SpriteLoaderLoadContextImpl.EmissiveControlImpl.setEmissiveIdMap(1): not translated, its class isn't translated (it implements `EmissiveControl`).
        // SpriteLoaderLoadContextImpl.EmissiveControlImpl.setHasEmissives(1): not translated, its class isn't translated (it implements `EmissiveControl`).
    }

    /// `SpriteLoaderStitchContext` (me/pepperbell/continuity/client/resource/SpriteLoaderStitchContext.java)
    pub mod sprite_loader_stitch_context {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpriteLoaderStitchContext.getEmissiveIdMap(0): not translated, it has no body.
        // SpriteLoaderStitchContext.setHasEmissives(1): not translated, it has no body.
    }

    /// `SpriteSourceListInitContext` (me/pepperbell/continuity/client/resource/SpriteSourceListInitContext.java)
    pub mod sprite_source_list_init_context {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpriteSourceListInitContext.getExtraIds(0): not translated, it has no body.
    }

    /// `SpriteSourceListListContext` (me/pepperbell/continuity/client/resource/SpriteSourceListListContext.java)
    pub mod sprite_source_list_list_context {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // SpriteSourceListListContext.setEmissiveIdMap(1): not translated, it has no body.
    }

    /// `BiomeHolder` (me/pepperbell/continuity/client/util/biome/BiomeHolder.java)
    pub mod biome_holder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Biome`. Its static parts are below.
        // BiomeHolder.BiomeHolder(1): not translated, its class isn't translated (it uses Minecraft's `Biome`).
        // BiomeHolder.getId(0): not translated, its class isn't translated (it uses Minecraft's `Biome`).
        // BiomeHolder.getBiome(0): not translated, its class isn't translated (it uses Minecraft's `Biome`).
        // BiomeHolder.refresh(2): not translated, its class isn't translated (it uses Minecraft's `Biome`).
        // BiomeHolder.equals(1): not translated, its class isn't translated (it uses Minecraft's `Biome`).
        // BiomeHolder.hashCode(0): not translated, its class isn't translated (it uses Minecraft's `Biome`).
    }

    /// `BiomeHolderManager` (me/pepperbell/continuity/client/util/biome/BiomeHolderManager.java)
    pub mod biome_holder_manager {
        use super::*;
        // HOLDER_CACHE: not translated, it uses BiomeHolder, which isn't translated (it uses Minecraft's `Biome`).
        // REFRESH_CALLBACKS: not translated, a Map or Set keyed by a fn#0.
        // registryManager: not translated, it uses Minecraft's `RegistryAccess`.
        #[derive(Clone, Debug)]
        pub struct BiomeHolderManager {
        }
        impl BiomeHolderManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // BiomeHolderManager.getOrCreateHolder(1): not translated, it uses BiomeHolder, which isn't translated (it uses Minecraft's `Biome`).
        // BiomeHolderManager.addRefreshCallback(1): not translated, it uses BiomeHolderManager.REFRESH_CALLBACKS, which isn't translated (a Map or Set keyed by a fn#0).
        // BiomeHolderManager.init(0): not translated, it uses Fabric's `ClientPlayConnectionEvents`.JOIN.
        // BiomeHolderManager.refreshHolders(0): not translated, it uses BiomeHolderManager.registryManager, which isn't translated (it uses Minecraft's `RegistryAccess`).
        // BiomeHolderManager.clearCache(0): not translated, it uses BiomeHolderManager.HOLDER_CACHE, which isn't translated (it uses BiomeHolder, which isn't translated (it uses Minecraft's `Biome`)).
    }

    /// `BiomeSetPredicate` (me/pepperbell/continuity/client/util/biome/BiomeSetPredicate.java)
    pub mod biome_set_predicate {
        use super::*;
        // The class itself isn't translated: it implements `Predicate` (java.util.function). Its static parts are below.
        // BiomeSetPredicate.BiomeSetPredicate(1): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // BiomeSetPredicate.test(1): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
        // BiomeSetPredicate.refresh(0): not translated, its class isn't translated (it implements `Predicate` (java.util.function)).
    }

    /// `BooleanState` (me/pepperbell/continuity/client/util/BooleanState.java)
    pub mod boolean_state {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct BooleanState {
            pub times_enabled: i32,
        }
        impl BooleanState {
            pub fn new() -> Self {
                let mut s = Self { times_enabled: 0 };
                s
            }
            pub fn is_enabled(&self) -> bool {
                return self.times_enabled > 0;
            }
            pub fn enable(&mut self) {
                self.times_enabled += 1;
            }
            pub fn disable(&mut self) {
                self.times_enabled -= 1;
            }
        }
    }

    /// `ListPairView` (me/pepperbell/continuity/client/util/ListPairView.java)
    pub mod list_pair_view {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ListPairView.ListPairView(2): not translated, its class isn't translated (it's generic).
        // ListPairView.get(1): not translated, its class isn't translated (it's generic).
        // ListPairView.size(0): not translated, its class isn't translated (it's generic).
    }

    /// `MathUtil` (me/pepperbell/continuity/client/util/MathUtil.java)
    pub mod math_util {
        use super::*;
        pub const GOLDEN_GAMMA: i64 = -7046029254386353131i64;
        #[derive(Clone, Debug)]
        pub struct MathUtil {
        }
        impl MathUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn signum(value: i32) -> i32 {
            if value > 0 {
                return 1;
            } else {
                return if value < 0 { -1 } else { 0 };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn lerp(delta: f32, start: i32, end: i32) -> i32 {
            return start + ((delta * ((end - start) as f32)) as i32);
        }
        pub fn lerp_color(delta: f32, color_a: i32, color_b: i32) -> i32 {
            return ((((math_util::lerp(delta, (((color_a as u32) >> 24) as i32) & 0xFF, (((color_b as u32) >> 24) as i32) & 0xFF)) << 24) | ((math_util::lerp(delta, (((color_a as u32) >> 16) as i32) & 0xFF, (((color_b as u32) >> 16) as i32) & 0xFF)) << 16)) | ((math_util::lerp(delta, (((color_a as u32) >> 8) as i32) & 0xFF, (((color_b as u32) >> 8) as i32) & 0xFF)) << 8)) | math_util::lerp(delta, color_a & 0xFF, color_b & 0xFF);
        }
        pub fn lerp_light(delta: f32, light_a: i32, light_b: i32) -> i32 {
            return ((math_util::lerp(delta, (((light_a as u32) >> 20) as i32) & 15, (((light_b as u32) >> 20) as i32) & 15)) << 20) | ((math_util::lerp(delta, (((light_a as u32) >> 4) as i32) & 15, (((light_b as u32) >> 4) as i32) & 15)) << 4);
        }
        pub fn mix64(mut z: i64) -> i64 {
            z = (z ^ (((z as u64) >> 30) as i64)) * (-4658895280553007687i64);
            z = (z ^ (((z as u64) >> 27) as i64)) * (-7723592293110705685i64);
            return z ^ (((z as u64) >> 31) as i64);
        }
        pub fn mix32(mut z: i64) -> i32 {
            z = (z ^ (((z as u64) >> 33) as i64)) * 7109453100751455733i64;
            return ((((((z ^ (((z as u64) >> 28) as i64)) * (-3808689974395783757i64)) as u64) >> 32) as i64) as i32);
        }
        // MathUtil.mix(5): not translated, it calls Minecraft's `Mth`.getSeed.
        pub fn remove_sign_bit(value: i32) -> i32 {
            return value & 2147483647;
        }
    }

    /// `QuadUtil` (me/pepperbell/continuity/client/util/QuadUtil.java)
    pub mod quad_util {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct QuadUtil {
        }
        impl QuadUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // QuadUtil.interpolate(3): not translated, it uses Fabric's `MutableQuadView`.
        // QuadUtil.interpolate(4): not translated, it uses Minecraft's `BakedQuad`.
        // QuadUtil.emitOverlayQuad(6): not translated, it uses Fabric's `QuadEmitter`.
        // QuadUtil.isQuadUnitSquare(1): not translated, it uses Fabric's `QuadView`.
        // QuadUtil.getTextureOrientation(1): not translated, it uses Fabric's `QuadView`.
    }

    /// `QuadUtil.PackedUvContainer` (me/pepperbell/continuity/client/util/QuadUtil.java)
    pub mod quad_util_packed_uv_container {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PackedUvContainer {
            pub packed_uv0: i64,
            pub packed_uv1: i64,
            pub packed_uv2: i64,
            pub packed_uv3: i64,
        }
        impl PackedUvContainer {
            pub fn new() -> Self {
                let mut s = Self { packed_uv0: 0i64, packed_uv1: 0i64, packed_uv2: 0i64, packed_uv3: 0i64 };
                s
            }
            pub fn packed_uv(&mut self, index: i32, packed_uv: i64) {
                match index {
                    0 => {
                        self.packed_uv0 = packed_uv;
                    }
                    1 => {
                        self.packed_uv1 = packed_uv;
                    }
                    2 => {
                        self.packed_uv2 = packed_uv;
                    }
                    3 => {
                        self.packed_uv3 = packed_uv;
                    }
                    _ => {
                        panic!("IndexOutOfBoundsException");
                    }
                }
            }
        }
    }

    /// `RandomIndexProvider` (me/pepperbell/continuity/client/util/RandomIndexProvider.java)
    pub mod random_index_provider {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RandomIndexProvider.getRandomIndex(1): not translated, it has no body.
    }

    /// `RandomIndexProvider.Factory` (me/pepperbell/continuity/client/util/RandomIndexProvider.java)
    pub mod random_index_provider_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // RandomIndexProvider.Factory.createIndexProvider(1): not translated, it has no body.
    }

    /// `RandomIndexProvider.Unweighted` (me/pepperbell/continuity/client/util/RandomIndexProvider.java)
    pub mod random_index_provider_unweighted {
        use super::*;
        // The class itself isn't translated: it implements `RandomIndexProvider`. Its static parts are below.
        // RandomIndexProvider.Unweighted.Unweighted(1): not translated, its class isn't translated (it implements `RandomIndexProvider`).
        // RandomIndexProvider.Unweighted.getRandomIndex(1): not translated, its class isn't translated (it implements `RandomIndexProvider`).
    }

    /// `RandomIndexProvider.UnweightedFactory` (me/pepperbell/continuity/client/util/RandomIndexProvider.java)
    pub mod random_index_provider_unweighted_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // INSTANCE: not translated, it uses UnweightedFactory, which isn't translated (it implements `Factory`).
        // RandomIndexProvider.UnweightedFactory.createIndexProvider(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `RandomIndexProvider.Weighted` (me/pepperbell/continuity/client/util/RandomIndexProvider.java)
    pub mod random_index_provider_weighted {
        use super::*;
        // The class itself isn't translated: it implements `RandomIndexProvider`. Its static parts are below.
        // RandomIndexProvider.Weighted.Weighted(2): not translated, its class isn't translated (it implements `RandomIndexProvider`).
        // RandomIndexProvider.Weighted.getRandomIndex(1): not translated, its class isn't translated (it implements `RandomIndexProvider`).
    }

    /// `RandomIndexProvider.WeightedFactory` (me/pepperbell/continuity/client/util/RandomIndexProvider.java)
    pub mod random_index_provider_weighted_factory {
        use super::*;
        // The class itself isn't translated: it implements `Factory`. Its static parts are below.
        // RandomIndexProvider.WeightedFactory.WeightedFactory(1): not translated, its class isn't translated (it implements `Factory`).
        // RandomIndexProvider.WeightedFactory.createIndexProvider(1): not translated, its class isn't translated (it implements `Factory`).
    }

    /// `RenderUtil` (me/pepperbell/continuity/client/util/RenderUtil.java)
    pub mod render_util {
        use super::*;
        // BLOCK_COLORS: not translated, it uses Minecraft's `BlockColors`.
        // blockAtlasSpriteFinder: not translated, it uses Fabric's `SpriteFinder`.
        #[derive(Clone, Debug)]
        pub struct RenderUtil {
        }
        impl RenderUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // RenderUtil.getTintColor(4): not translated, it uses Minecraft's `BlockState`.
        // RenderUtil.aoFromTintBlock(1): not translated, it uses Fabric's `TriState`.
        // RenderUtil.canHaveAO(1): not translated, it uses Minecraft's `BlockState`.
        // RenderUtil.isMissingSprite(1): not translated, it uses Minecraft's `TextureAtlasSprite`.
        // RenderUtil.getSpriteFinder(0): not translated, it uses Fabric's `SpriteFinder`.
    }

    /// `RenderUtil.ReloadListener` (me/pepperbell/continuity/client/util/RenderUtil.java)
    pub mod render_util_reload_listener {
        use super::*;
        // The class itself isn't translated: it implements Minecraft's `ResourceManagerReloadListener`. Its static parts are below.
        pub static ID: LazyLock<JIdentifier> = LazyLock::new(|| continuity_client::as_id("render_util"));
        // INSTANCE: not translated, it uses ReloadListener, which isn't translated (it implements Minecraft's `ResourceManagerReloadListener`).
        // RenderUtil.ReloadListener.onResourceManagerReload(1): not translated, its class isn't translated (it implements Minecraft's `ResourceManagerReloadListener`).
    }

    /// `SpriteCalculator` (me/pepperbell/continuity/client/util/SpriteCalculator.java)
    pub mod sprite_calculator {
        use super::*;
        // MODEL_MANAGER: not translated, it uses Minecraft's `ModelManager`.
        // SPRITE_CACHES: not translated, it uses `EnumMap` (java.util).
        #[derive(Clone, Debug)]
        pub struct SpriteCalculator {
        }
        impl SpriteCalculator {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // SpriteCalculator.getSprites(2): not translated, it uses Minecraft's `TextureAtlasSprite`.
        // SpriteCalculator.clearCache(0): not translated, it uses SpriteCalculator.SPRITE_CACHES, which isn't translated (it uses `EnumMap` (java.util)).
    }

    /// `SpriteCalculator.SpriteCache` (me/pepperbell/continuity/client/util/SpriteCalculator.java)
    pub mod sprite_calculator_sprite_cache {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Direction`. Its static parts are below.
        // SpriteCalculator.SpriteCache.SpriteCache(1): not translated, its class isn't translated (it uses Minecraft's `Direction`).
        // SpriteCalculator.SpriteCache.getSprites(1): not translated, its class isn't translated (it uses Minecraft's `Direction`).
        // SpriteCalculator.SpriteCache.calculateSprites(1): not translated, its class isn't translated (it uses Minecraft's `Direction`).
        // SpriteCalculator.SpriteCache.clear(0): not translated, its class isn't translated (it uses Minecraft's `Direction`).
    }

    /// `SpriteCalculator.SpriteCache.CollectingQuadTransform` (me/pepperbell/continuity/client/util/SpriteCalculator.java)
    pub mod sprite_calculator_sprite_cache_collecting_quad_transform {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `QuadTransform`. Its static parts are below.
        // SpriteCalculator.SpriteCache.CollectingQuadTransform.CollectingQuadTransform(1): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // SpriteCalculator.SpriteCache.CollectingQuadTransform.transform(1): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // SpriteCalculator.SpriteCache.CollectingQuadTransform.clear(0): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
        // SpriteCalculator.SpriteCache.CollectingQuadTransform.result(0): not translated, its class isn't translated (it implements Fabric's `QuadTransform`).
    }

    /// `ContinuityFeatureStatesImpl` (me/pepperbell/continuity/impl/client/ContinuityFeatureStatesImpl.java)
    pub mod continuity_feature_states_impl {
        use super::*;
        // The class itself isn't translated: it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client). Its static parts are below.
        // ContinuityFeatureStatesImpl.ContinuityFeatureStatesImpl(0): not translated, its class isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)).
        // ContinuityFeatureStatesImpl.get(0): not translated, it uses ContinuityFeatureStatesImpl, which isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)).
        // ContinuityFeatureStatesImpl.getConnectedTexturesState(0): not translated, its class isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)).
        // ContinuityFeatureStatesImpl.getEmissiveTexturesState(0): not translated, its class isn't translated (it implements `ContinuityFeatureStates` (me.pepperbell.continuity.api.client)).
    }

    /// `ContinuityFeatureStatesImpl.FeatureStateImpl` (me/pepperbell/continuity/impl/client/ContinuityFeatureStatesImpl.java)
    pub mod continuity_feature_states_impl_feature_state_impl {
        use super::*;
        // The class itself isn't translated: it extends `BooleanState` (me.pepperbell.continuity.client.util). Its static parts are below.
    }

    /// `CtmLoaderRegistryImpl` (me/pepperbell/continuity/impl/client/CtmLoaderRegistryImpl.java)
    pub mod ctm_loader_registry_impl {
        use super::*;
        // The class itself isn't translated: it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client). Its static parts are below.
        // INSTANCE: not translated, it uses CtmLoaderRegistryImpl, which isn't translated (it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client)).
        // CtmLoaderRegistryImpl.registerLoader(2): not translated, its class isn't translated (it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client)).
        // CtmLoaderRegistryImpl.getLoader(1): not translated, its class isn't translated (it implements `CtmLoaderRegistry` (me.pepperbell.continuity.api.client)).
    }

    /// `EmissiveSpriteApiImpl` (me/pepperbell/continuity/impl/client/EmissiveSpriteApiImpl.java)
    pub mod emissive_sprite_api_impl {
        use super::*;
        // The class itself isn't translated: it implements `EmissiveSpriteApi` (me.pepperbell.continuity.api.client). Its static parts are below.
        // INSTANCE: not translated, it uses EmissiveSpriteApiImpl, which isn't translated (it implements `EmissiveSpriteApi` (me.pepperbell.continuity.api.client)).
        // EmissiveSpriteApiImpl.getEmissiveSprite(1): not translated, its class isn't translated (it implements `EmissiveSpriteApi` (me.pepperbell.continuity.api.client)).
    }

    /// `ProcessingContextImpl` (me/pepperbell/continuity/impl/client/ProcessingContextImpl.java)
    pub mod processing_context_impl {
        use super::*;
        // The class itself isn't translated: it implements `ProcessingContext`. Its static parts are below.
        // ProcessingContextImpl.getExtraQuadEmitter(0): not translated, its class isn't translated (it implements `ProcessingContext`).
        // ProcessingContextImpl.getData(1): not translated, it's generic.
        // ProcessingContextImpl.getDataOrNull(1): not translated, it's generic.
        // ProcessingContextImpl.outputTo(1): not translated, its class isn't translated (it implements `ProcessingContext`).
        // ProcessingContextImpl.reset(0): not translated, its class isn't translated (it implements `ProcessingContext`).
        // ProcessingContextImpl.resetData(0): not translated, its class isn't translated (it implements `ProcessingContext`).
        // ProcessingContextImpl.resetData(1): not translated, it's generic.
    }

    /// `ProcessingDataKeyImpl` (me/pepperbell/continuity/impl/client/ProcessingDataKeyImpl.java)
    pub mod processing_data_key_impl {
        use super::*;
        // The class itself isn't translated: it's generic. Its static parts are below.
        // ProcessingDataKeyImpl.ProcessingDataKeyImpl(4): not translated, its class isn't translated (it's generic).
        // ProcessingDataKeyImpl.getId(0): not translated, its class isn't translated (it's generic).
        // ProcessingDataKeyImpl.getRawId(0): not translated, its class isn't translated (it's generic).
        // ProcessingDataKeyImpl.getValueSupplier(0): not translated, its class isn't translated (it's generic).
        // ProcessingDataKeyImpl.getValueResetAction(0): not translated, its class isn't translated (it's generic).
    }

    /// `ProcessingDataKeyRegistryImpl` (me/pepperbell/continuity/impl/client/ProcessingDataKeyRegistryImpl.java)
    pub mod processing_data_key_registry_impl {
        use super::*;
        // The class itself isn't translated: it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client). Its static parts are below.
        // INSTANCE: not translated, it uses ProcessingDataKeyRegistryImpl, which isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)).
        // ProcessingDataKeyRegistryImpl.registerKey(3): not translated, it's generic.
        // ProcessingDataKeyRegistryImpl.getKey(1): not translated, its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)).
        // ProcessingDataKeyRegistryImpl.getRegisteredAmount(0): not translated, its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)).
        // ProcessingDataKeyRegistryImpl.init(0): not translated, its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)).
        // ProcessingDataKeyRegistryImpl.getAllResettable(0): not translated, its class isn't translated (it implements `ProcessingDataKeyRegistry` (me.pepperbell.continuity.api.client)).
    }

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }

    
    // ---- java.util.Properties: `key=value` text files ----
    
    #[derive(Clone, Debug, Default)]
    pub struct JProps(pub std::collections::BTreeMap<String, String>);
    
    fn j_props_unescape(s: &str) -> String {
        let mut out = String::new();
        let mut it = s.chars().peekable();
        while let Some(c) = it.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            match it.next() {
                Some('t') => out.push('\t'),
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('f') => out.push('\u{c}'),
                Some('u') => {
                    let hex: String = (0..4).filter_map(|_| it.next()).collect();
                    out.push(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32).unwrap_or('\u{FFFD}'));
                }
                Some(o) => out.push(o),
                None => {}
            }
        }
        out
    }
    
    fn j_props_escape(s: &str, key: bool) -> String {
        let mut out = String::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '\\' => out.push_str("\\\\"),
                '\t' => out.push_str("\\t"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\u{c}' => out.push_str("\\f"),
                '=' | ':' | '#' | '!' => {
                    out.push('\\');
                    out.push(c);
                }
                ' ' if key || i == 0 => out.push_str("\\ "),
                c if (c as u32) < 0x20 || (c as u32) > 0x7e => out.push_str(&format!("\\u{:04X}", c as u32)),
                c => out.push(c),
            }
        }
        out
    }
    
    impl JProps {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn get_property(&self, k: &str) -> Option<String> {
            self.0.get(k).cloned()
        }
        pub fn get_property_or(&self, k: &str, default: &str) -> String {
            self.0.get(k).cloned().unwrap_or_else(|| default.to_string())
        }
        pub fn set_property(&mut self, k: &str, v: &str) -> Option<String> {
            self.0.insert(k.to_string(), v.to_string())
        }
        pub fn contains_key(&self, k: &str) -> bool {
            self.0.contains_key(k)
        }
        pub fn remove(&mut self, k: &str) -> Option<String> {
            self.0.remove(k)
        }
        pub fn names(&self) -> JSet<String> {
            JSet::from_iter(self.0.keys().cloned())
        }
        /// Properties.load: `key=value`, `key: value` or `key value`, `#`/`!` comments, `\` continuations.
        pub fn load(&mut self, text: &str) {
            let mut logical = String::new();
            for raw in text.lines() {
                let line = raw.trim_start();
                if logical.is_empty() && (line.is_empty() || line.starts_with('#') || line.starts_with('!')) {
                    continue;
                }
                let trailing = line.chars().rev().take_while(|&c| c == '\\').count();
                if trailing % 2 == 1 {
                    logical.push_str(&line[..line.len() - 1]);
                    continue;
                }
                logical.push_str(line);
                let chars: Vec<char> = logical.chars().collect();
                let mut i = 0;
                while i < chars.len() && !(matches!(chars[i], '=' | ':') || chars[i].is_whitespace()) {
                    i += if chars[i] == '\\' { 2 } else { 1 };
                }
                let key: String = chars[..i.min(chars.len())].iter().collect();
                let mut j = i;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                if j < chars.len() && matches!(chars[j], '=' | ':') {
                    j += 1;
                }
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                let value: String = chars[j.min(chars.len())..].iter().collect();
                self.0.insert(j_props_unescape(&key), j_props_unescape(&value));
                logical.clear();
            }
        }
        /// Properties.store: the comment, then `key=value` lines (Java also writes the date, left out here).
        pub fn store_text(&self, comment: Option<&str>) -> String {
            let mut out = String::new();
            if let Some(c) = comment {
                for l in c.lines() {
                    out.push('#');
                    out.push_str(l);
                    out.push('\n');
                }
            }
            for (k, v) in &self.0 {
                out.push_str(&j_props_escape(k, true));
                out.push('=');
                out.push_str(&j_props_escape(v, false));
                out.push('\n');
            }
            out
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "continuity");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
