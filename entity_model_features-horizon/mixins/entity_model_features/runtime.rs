// Entity Model Features, ported to Horizon by horizon-port from entity_model_features-3.3.12-26.3-fabric.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_entity_model_features). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "entity_model_features";
pub const NAME: &str = "Entity Model Features";
pub const VERSION: &str = "3.3.12";
pub const DESCRIPTION: &str = "This is an expansion of the ETF mod, it adds support for OptiFine format Custom Entity Model (CEM) resource packs. While still allowing you to disable this to use a different model mod :)";

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

    /// `EMFConfig` (traben/entity_model_features/config/EMFConfig.java)
    pub mod emf_config {
        use super::*;
        // The class itself isn't translated: it extends `TConfig` (traben.tconfig). Its static parts are below.
        // EMFConfig.getRenderModeFor(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getRenderModeFor(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getPhysicsModModeFor(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getVanillaHologramModeFor(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getTypeString(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFConfig.isModelDisabled(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getGUIOptions(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.reprintLoadingErrors(1): not translated, it uses `Exception`.
        // EMFConfig.getMathInfo(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getModelSettings(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getExport(2): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getmappings(1): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getEntitySettings(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.addEntityConfigs(2): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
        // EMFConfig.getModIcon(0): not translated, its class isn't translated (it extends `TConfig` (traben.tconfig)).
    }

    /// `EMFConfig.CustomEntityModelSupportMode` (traben/entity_model_features/config/EMFConfig.java)
    pub mod emf_config_custom_entity_model_support_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EMFConfig.CustomEntityModelSupportMode.CustomEntityModelSupportMode(1): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.CustomEntityModelSupportMode.toString(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.CustomEntityModelSupportMode.allowsKnown(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.CustomEntityModelSupportMode.allowsUnknowns(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.CustomEntityModelSupportMode.isOn(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.CustomEntityModelSupportMode.allowsAll(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EMFConfig.ModelPrintMode` (traben/entity_model_features/config/EMFConfig.java)
    pub mod emf_config_model_print_mode {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EMFConfig.ModelPrintMode.ModelPrintMode(1): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.ModelPrintMode.doesJems(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.ModelPrintMode.doesAll(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.ModelPrintMode.doesLog(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.ModelPrintMode.toString(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EMFConfig.ModelRootRenderer` (traben/entity_model_features/config/EMFConfig.java)
    pub mod emf_config_model_root_renderer {
        use super::*;
        // The class itself isn't translated: it implements `Renderable` (traben.tconfig.gui.TConfigScreenList). Its static parts are below.
        // EMFConfig.ModelRootRenderer.ModelRootRenderer(1): not translated, its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)).
        // EMFConfig.ModelRootRenderer.canRender(0): not translated, its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)).
        // EMFConfig.ModelRootRenderer.render(3): not translated, its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)).
        // EMFConfig.ModelRootRenderer.renderBoxes(3): not translated, its class isn't translated (it implements `Renderable` (traben.tconfig.gui.TConfigScreenList)).
    }

    /// `EMFConfig.PhysicsModCompatChoice` (traben/entity_model_features/config/EMFConfig.java)
    pub mod emf_config_physics_mod_compat_choice {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum PhysicsModCompatChoice {
            Off,
            Vanilla,
            Custom,
        }
        impl PhysicsModCompatChoice {
            pub const VALUES: [PhysicsModCompatChoice; 3] = [PhysicsModCompatChoice::Off, PhysicsModCompatChoice::Vanilla, PhysicsModCompatChoice::Custom, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    PhysicsModCompatChoice::Off => "OFF",
                    PhysicsModCompatChoice::Vanilla => "VANILLA",
                    PhysicsModCompatChoice::Custom => "CUSTOM",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "OFF" => PhysicsModCompatChoice::Off,
                    "VANILLA" => PhysicsModCompatChoice::Vanilla,
                    "CUSTOM" => PhysicsModCompatChoice::Custom,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `EMFConfig.RenderModeChoice` (traben/entity_model_features/config/EMFConfig.java)
    pub mod emf_config_render_mode_choice {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EMFConfig.RenderModeChoice.RenderModeChoice(1): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.RenderModeChoice.toString(0): not translated, its class isn't translated (its constants carry values).
        // EMFConfig.RenderModeChoice.allowsRegularDraw(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EMFConfig.VanillaModelRenderMode` (traben/entity_model_features/config/EMFConfig.java)
    pub mod emf_config_vanilla_model_render_mode {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum VanillaModelRenderMode {
            Off,
            Normal,
            Offset,
        }
        impl VanillaModelRenderMode {
            pub const VALUES: [VanillaModelRenderMode; 3] = [VanillaModelRenderMode::Off, VanillaModelRenderMode::Normal, VanillaModelRenderMode::Offset, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    VanillaModelRenderMode::Off => "OFF",
                    VanillaModelRenderMode::Normal => "NORMAL",
                    VanillaModelRenderMode::Offset => "OFFSET",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "OFF" => VanillaModelRenderMode::Off,
                    "NORMAL" => VanillaModelRenderMode::Normal,
                    "OFFSET" => VanillaModelRenderMode::Offset,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `EMFModMenu` (traben/entity_model_features/config/EMFModMenu.java)
    pub mod emf_mod_menu {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // EMFModMenu.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `EMF` (traben/entity_model_features/EMF.java)
    pub mod emf {
        use super::*;
        pub const MOD_ID: &str = "entity_model_features";
        pub static FORGE_HAD_LOADING_ERROR: JCell<bool> = JCell::new(false);
        // testedForge: not translated, it calls `ETF` (traben.entity_texture_features).isForge.
        pub static TEMP_DISABLE_MODEL_MODIFICATIONS: JCell<bool> = JCell::new(false);
        pub static IS_LOADING_PHASE: JCell<bool> = JCell::new(true);
        // configHandler: not translated, it uses `TConfigHandler` (traben.tconfig).
        #[derive(Clone, Debug)]
        pub struct EMF {
        }
        impl EMF {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EMF.config(0): not translated, it uses `TConfigHandler` (traben.tconfig).
        // EMF.init(0): not translated, its body couldn't be read: an anonymous class.
        // EMF.getConfigScreen(1): not translated, it uses Minecraft's `Screen`.
        // EMF.getConfigScreen(2): not translated, it uses Minecraft's `Screen`.
        // EMF.testForForgeLoadingError(0): not translated, it uses EMF.testedForge, which isn't translated (it calls `ETF` (traben.entity_texture_features).isForge).
        // EMF.randomQuip(0): not translated, it creates `Random` (java.util).
    }

    /// `EMFAnimationApi` (traben/entity_model_features/EMFAnimationApi.java)
    pub mod emf_animation_api {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        pub fn get_api_version() -> i32 {
            return 11;
        }
        // EMFAnimationApi.getCurrentEntity(0): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.registerSingletonAnimationVariable(4): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.registerSingletonAnimationVariable(4): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.registerSingletonAnimationVariable(4): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.registerUniqueAnimationVariableFactory(3): not translated, it calls VariableRegistry.getInstance, which isn't translated.
        // EMFAnimationApi.registerCustomFunctionFromStaticMethod(5): not translated, it uses `Method` (java.lang.reflect).
        // EMFAnimationApi.registerCustomFunctionFactory(4): not translated, it uses MethodFactory, which isn't translated (it's an interface).
        // EMFAnimationApi.emfEntityOf(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.emfEntityOf(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        pub fn register_animation_hook(hook: Option<emf_animation_api_emf_animation_hook::EMFAnimationHook>) -> std::io::Result<bool> {
            if hook.is_none() {
                panic!("exception");
            }
            { let a_0 = (hook.as_ref().expect("null")).clone(); emf_state::ANIMATION_HOOKS.with(|o| o.push(a_0)) };
            return Ok(true);
        }
        // EMFAnimationApi.animateModelForEntity(3): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.animateModelForEntity(3): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.animateModelForState(3): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.animateModelForState(3): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.registerPauseCondition(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.pauseAllCustomAnimationsForEntity(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.resumeAllCustomAnimationsForEntity(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.pauseCustomAnimationsForThesePartsOfEntity(2): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.registerVanillaModelCondition(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.lockEntityToVanillaModel(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.unlockEntityToVanillaModel(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationApi.getCurrentEMFVariantOfModel(1): not translated, it uses Minecraft's `EntityModel`.
        // EMFAnimationApi.isModelAnimatedByEMF(1): not translated, it uses Minecraft's `EntityModel`.
        // EMFAnimationApi.isModelCustomizedByEMF(1): not translated, it uses Minecraft's `EntityModel`.
        // EMFAnimationApi.isModelPartCustomToEMF(1): not translated, it uses Minecraft's `ModelPart`.
        // EMFAnimationApi.isModelPartAnimatedByEMF(1): not translated, it uses Minecraft's `ModelPart`.
        pub fn register_animation_function_4(source_mod_id: &str, method_name: &str, method_explanation_translation_key_or_text: &str, function: &dyn Fn(&f32) -> f32) -> std::io::Result<()> {
            panic!("exception");
            Ok(())
        }
        pub fn register_animation_bi_function_4(source_mod_id: &str, method_name: &str, method_explanation_translation_key_or_text: &str, bi_function: &dyn Fn(&f32, &f32) -> f32) -> std::io::Result<()> {
            panic!("exception");
            Ok(())
        }
        // EMFAnimationApi.registerAnimationTriFunction(4): not translated, it uses `TriFunction` (org.apache.commons.lang3.function).
        pub fn register_animation_multi_function_4(source_mod_id: &str, method_name: &str, method_explanation_translation_key_or_text: &str, multi_function: &dyn Fn(&Vec<f32>) -> f32) -> std::io::Result<()> {
            panic!("exception");
            Ok(())
        }
        // EMFAnimationApi.registerSingletonAnimationVariable(3): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.registerSingletonAnimationVariable(3): not translated, it's overloaded with the same number of parameters.
        // EMFAnimationApi.registerSingletonAnimationVariable(3): not translated, it's overloaded with the same number of parameters.
        pub fn register_animation_function_3(source_mod_id: &str, method_name: &str, function: &dyn Fn(&f32) -> f32) -> std::io::Result<()> {
            panic!("exception");
            Ok(())
        }
        pub fn register_animation_bi_function_3(source_mod_id: &str, method_name: &str, bi_function: &dyn Fn(&f32, &f32) -> f32) -> std::io::Result<()> {
            panic!("exception");
            Ok(())
        }
        // EMFAnimationApi.registerAnimationTriFunction(3): not translated, it uses `TriFunction` (org.apache.commons.lang3.function).
        pub fn register_animation_multi_function_3(source_mod_id: &str, method_name: &str, multi_function: &dyn Fn(&Vec<f32>) -> f32) -> std::io::Result<()> {
            panic!("exception");
            Ok(())
        }
        // EMFAnimationApi.registerCustomFunctionFactory(3): not translated, it uses MethodFactory, which isn't translated (it's an interface).
        // EMFAnimationApi.dependencyException(0): not translated, it uses `Exception`.
        // EMFAnimationApi.paramFail(1): not translated, it uses `Exception`.
    }

    /// `EMFAnimationApi.EMFAnimationHook` (traben/entity_model_features/EMFAnimationApi.java)
    pub mod emf_animation_api_emf_animation_hook {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EMFAnimationHook {
        }
        impl EMFAnimationHook {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // EMFAnimationApi.EMFAnimationHook.onAnimationStart(2): not translated, it uses AnimationContext, which isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
            // EMFAnimationApi.EMFAnimationHook.onAnimationEnd(2): not translated, it uses AnimationContext, which isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
            // EMFAnimationApi.EMFAnimationHook.onBipedPoseCopyStart(3): not translated, it uses Minecraft's `HumanoidModel`.
            // EMFAnimationApi.EMFAnimationHook.onBipedPoseCopyEnd(3): not translated, it uses Minecraft's `HumanoidModel`.
        }
    }

    /// `EMFAnimationApi.EMFAnimationHook.AnimationContext` (traben/entity_model_features/EMFAnimationApi.java)
    pub mod emf_animation_api_emf_animation_hook_animation_context {
        use super::*;
        // The class itself isn't translated: it uses EMFEntityRenderState, which isn't translated (it's an interface). Its static parts are below.
        // EMFAnimationApi.EMFAnimationHook.AnimationContext.AnimationContext(5): not translated, its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
        // EMFAnimationApi.EMFAnimationHook.AnimationContext.activeState(0): not translated, its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
        // EMFAnimationApi.EMFAnimationHook.AnimationContext.animatingModelRoot(0): not translated, its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
        // EMFAnimationApi.EMFAnimationHook.AnimationContext.error(0): not translated, its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
        // EMFAnimationApi.EMFAnimationHook.AnimationContext.partsRequestedToPauseThisAnimation(0): not translated, its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
        // EMFAnimationApi.EMFAnimationHook.AnimationContext.animationHandler(0): not translated, its class isn't translated (it uses EMFEntityRenderState, which isn't translated (it's an interface)).
    }

    /// `EMFException` (traben/entity_model_features/EMFException.java)
    pub mod emf_exception {
        use super::*;
        // The class itself isn't translated: it extends `Exception`. Its static parts are below.
        // EMFException.EMFException(1): not translated, its class isn't translated (it extends `Exception`).
        // EMFException.record(0): not translated, its class isn't translated (it extends `Exception`).
        // EMFException.recordException(1): not translated, it's generic.
        // EMFException.toString(0): not translated, its class isn't translated (it extends `Exception`).
    }

    /// `EMFInit` (traben/entity_model_features/EMFInit.java)
    pub mod emf_init {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // EMFInit.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `EMFManager` (traben/entity_model_features/EMFManager.java)
    pub mod emf_manager {
        use super::*;
        // The class itself isn't translated: it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache). Its static parts are below.
        // EBETypes: not translated, it uses Minecraft's `BlockEntityType`.
        // lastCreatedRootModelPart: not translated, it uses EMFModelPartRoot, which isn't translated (it extends `EMFModelPartVanilla`).
        // self: not translated, it uses EMFManager, which isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        pub static MANAGER_INSTANCE_HASH: JCell<i32> = JCell::new(0);
        // EMFManager.receiveException(1): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.EMFManager(0): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        pub fn get_manager_instance_hash() -> i32 {
            return emf_manager::MANAGER_INSTANCE_HASH.get();
        }
        // EMFManager.getInstance(0): not translated, it uses EMFManager, which isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.resetInstance(0): not translated, it calls EMF.config, which isn't translated.
        // EMFManager.getJemDataWithDirectory(2): not translated, it uses EMFModel_ID, which isn't translated (it implements `Comparable`).
        // EMFManager.getModelFromHierarchicalId(2): not translated, it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`).
        // EMFManager.handleBoats(2): not translated, it uses EMFModel_ID, which isn't translated (it implements `Comparable`).
        // EMFManager.wasEBEModified(0): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.getResourcePackList(0): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.modifyEBEIfRequired(0): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.injectIntoModelRootGetter(2): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.reloadEnd(0): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.getArmorParts(0): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.getJemAndContext(3): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.getDoubleChest(4): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
        // EMFManager.setupAnimationsFromJemToModel(3): not translated, its class isn't translated (it uses `UUIDInteger` (traben.entity_texture_features.utils.ETFLruCache)).
    }

    /// `Plugin` (traben/entity_model_features/mixin/Plugin.java)
    pub mod plugin {
        use super::*;
        // The class itself isn't translated: it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility). Its static parts are below.
        // Plugin.onLoad(1): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.getRefMapperConfig(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.shouldApplyMixin(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.acceptTargets(2): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.getMixins(0): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.preApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
        // Plugin.postApply(4): not translated, its class isn't translated (it implements `IMixinConfigPlugin` (org.spongepowered.asm.mixin.extensibility)).
    }

    /// `AnimSetupContext` (traben/entity_model_features/models/animation/AnimSetupContext.java)
    pub mod anim_setup_context {
        use super::*;
        // The class itself isn't translated: it implements `AutoCloseable`. Its static parts are below.
        // AnimSetupContext.AnimSetupContext(3): not translated, its class isn't translated (it implements `AutoCloseable`).
        // AnimSetupContext.getModelFromHierarchicalId(1): not translated, its class isn't translated (it implements `AutoCloseable`).
        // AnimSetupContext.close(0): not translated, its class isn't translated (it implements `AutoCloseable`).
    }

    /// `EMFAnimation` (traben/entity_model_features/models/animation/EMFAnimation.java)
    pub mod emf_animation {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EMFAnimation {
        }
        impl EMFAnimation {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    /// `EMFAnimationHandler` (traben/entity_model_features/models/animation/EMFAnimationHandler.java)
    pub mod emf_animation_handler {
        use super::*;
        // The class itself isn't translated: it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)). Its static parts are below.
        // EMFAnimationHandler.EMFAnimationHandler(2): not translated, its class isn't translated (it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`))).
        // EMFAnimationHandler.addAnimLineData(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // EMFAnimationHandler.lines(0): not translated, its class isn't translated (it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`))).
        // EMFAnimationHandler.getLine(1): not translated, it stores a second reference to an object (Rust would need it copied or shared).
        // EMFAnimationHandler.animate(1): not translated, it uses Minecraft's `ModelPart`.
        // EMFAnimationHandler.animateInner(1): not translated, it has no body.
        // EMFAnimationHandler.finishAndValidate(0): not translated, it has no body.
        // EMFAnimationHandler.toString(0): not translated, it calls EMFAnimationHandler.getClass, which isn't in the mod's source.
    }

    /// `EMFAnimationHandler.AnimLineData` (traben/entity_model_features/models/animation/EMFAnimationHandler.java)
    pub mod emf_animation_handler_anim_line_data {
        use super::*;
        // The class itself isn't translated: it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`). Its static parts are below.
        // EMFAnimationHandler.AnimLineData.AnimLineData(4): not translated, its class isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)).
        // EMFAnimationHandler.AnimLineData.hashCode(0): not translated, its class isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)).
        // EMFAnimationHandler.AnimLineData.toString(0): not translated, its class isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)).
    }

    /// `EMFAttachment` (traben/entity_model_features/models/animation/EMFAttachment.java)
    pub mod emf_attachment {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Pose`. Its static parts are below.
        // EMFAttachment.EMFAttachment(4): not translated, its class isn't translated (it uses Minecraft's `Pose`).
        // EMFAttachment.translate(1): not translated, its class isn't translated (it uses Minecraft's `Pose`).
    }

    /// `EMFAttachment.Type` (traben/entity_model_features/models/animation/EMFAttachment.java)
    pub mod emf_attachment_type {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // NON_HANDS: not translated, it uses Type, which isn't translated (its constants carry values).
        // EMFAttachment.Type.Type(1): not translated, its class isn't translated (its constants carry values).
        // EMFAttachment.Type.translateForType(2): not translated, its class isn't translated (its constants carry values).
        // EMFAttachment.Type.isHumanoidHand(0): not translated, its class isn't translated (its constants carry values).
        // EMFAttachment.Type.of(1): not translated, it uses Type, which isn't translated (its constants carry values).
    }

    /// `ASMAnimationHandler` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java)
    pub mod asm_animation_handler {
        use super::*;
        // The class itself isn't translated: it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation). Its static parts are below.
        // ASMAnimationHandler.ASMAnimationHandler(3): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.complete(2): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.animateInner(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.canLODPause(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.animateInnerLOD(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.animateInnerNoLOD(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.finishAndValidate(0): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.buildFloatConsumers(0): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.buildBoolConsumers(0): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.buildVarSupplier(2): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMAnimationHandler.asmLog(3): not translated, it uses ASMVariableHandler, which isn't translated (it uses `Stack` (java.util)).
    }

    /// `ASMAnimationHandler.AnimVars` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java)
    pub mod asm_animation_handler_anim_vars {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct AnimVars {
            pub floats: Vec<f32>,
            pub bools: Vec<bool>,
        }
        impl AnimVars {
            pub fn new(floats: &[f32], bools: &[bool]) -> Self {
                let mut s = Self { floats: Vec::new(), bools: Vec::new() };
                s.floats = floats.to_vec();
                s.bools = bools.to_vec();
                s
            }
            pub fn floats(&self) -> Vec<f32> {
                return self.floats.to_vec();
            }
            pub fn bools(&self) -> Vec<bool> {
                return self.bools.to_vec();
            }
        }
    }

    /// `ASMAnimationHandler.BoolConsumerAsm` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java)
    pub mod asm_animation_handler_bool_consumer_asm {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ASMAnimationHandler.BoolConsumerAsm.accept(2): not translated, it has no body.
    }

    /// `ASMAnimationHandler.FloatConsumerAsm` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java)
    pub mod asm_animation_handler_float_consumer_asm {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ASMAnimationHandler.FloatConsumerAsm.accept(2): not translated, it has no body.
    }

    /// `ASMAnimationHandler.VarConsumer` (traben/entity_model_features/models/animation/math/asm/ASMAnimationHandler.java)
    pub mod asm_animation_handler_var_consumer {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ASMAnimationHandler.VarConsumer.accept(2): not translated, it has no body.
    }

    /// `ASMHelper` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java)
    pub mod asm_helper {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct ASMHelper {
        }
        impl ASMHelper {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ASMHelper.visitHelperFunctionASM(2): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        // ASMHelper.getHelperMethod(1): not translated, it uses `Method` (java.lang.reflect).
        // ASMHelper.getHelperMethodCompiler(1): not translated, it uses ASMVisitable, which isn't translated (it's an interface).
        // ASMHelper.getStaticMethod(2): not translated, it uses `Method` (java.lang.reflect).
        // ASMHelper.visitStaticFunctionASM(3): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        // ASMHelper.visitStaticFunctionASM(2): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        // ASMHelper.asmIfZeroOrGreater(3): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        // ASMHelper.asmIf(3): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        pub fn sin(f: f32) -> f32 {
            return f.sin();
        }
        pub fn cos(f: f32) -> f32 {
            return f.cos();
        }
        pub fn asin(f: f32) -> f32 {
            return (((f as f64).asin()) as f32);
        }
        pub fn acos(f: f32) -> f32 {
            return (((f as f64).acos()) as f32);
        }
        pub fn tan(f: f32) -> f32 {
            return (((f as f64).tan()) as f32);
        }
        pub fn atan(f: f32) -> f32 {
            return (((f as f64).atan()) as f32);
        }
        pub fn abs(f: f32) -> f32 {
            return f.abs();
        }
        pub fn floor(f: f32) -> f32 {
            return (((f as f64).floor() as i32) as f32);
        }
        pub fn ceil(f: f32) -> f32 {
            return (((f as f64).ceil() as i32) as f32);
        }
        pub fn round(f: f32) -> f32 {
            return (j_round_f(f) as f32);
        }
        // ASMHelper.log(1): not translated, it calls EMFManager.getInstance, which isn't translated.
        pub fn _log(f: f32) -> f32 {
            return (((f as f64).ln()) as f32);
        }
        // ASMHelper.log_ASM(2): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        pub fn exp(f: f32) -> f32 {
            return (((f as f64).exp()) as f32);
        }
        pub fn torad(f: f32) -> f32 {
            return f * ((std::f64::consts::PI / 180.0f64) as f32);
        }
        // ASMHelper.torad_ASM(2): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        pub fn todeg(f: f32) -> f32 {
            return f * (180.0f32 / (std::f64::consts::PI as f32));
        }
        // ASMHelper.todeg_ASM(2): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        pub fn frac(f: f32) -> f32 {
            return j_frac((f as f64)) as f32;
        }
        pub fn signum(f: f32) -> f32 {
            return j_signum_f(f);
        }
        // ASMHelper.sqrt(1): not translated, it calls EMFManager.getInstance, which isn't translated.
        pub fn _sqrt(f: f32) -> f32 {
            return f.sqrt();
        }
        // ASMHelper.sqrt_ASM(2): not translated, it uses `MethodVisitor` (org.objectweb.asm).
        pub fn fmod(f: f32, f2: f32) -> f32 {
            return (((f as i32).rem_euclid((f2 as i32))) as f32);
        }
        pub fn pow(f: f32, f2: f32) -> f32 {
            return (((f as f64).powf((f2 as f64))) as f32);
        }
        pub fn atan2(f: f32, f2: f32) -> f32 {
            return (((f as f64).atan2((f2 as f64))) as f32);
        }
        pub fn clamp(f: f32, f2: f32, f3: f32) -> f32 {
            return j_clamp(f, f2, f3);
        }
        pub fn lerp(f: f32, f2: f32, f3: f32) -> f32 {
            return j_lerp(f, f2, f3);
        }
        pub fn wrapdeg(f: f32) -> f32 {
            return j_wrap_degrees((f as f64)) as f32;
        }
        pub fn wraprad(f: f32) -> f32 {
            return asm_helper::torad(j_wrap_degrees((asm_helper::todeg(f) as f64)) as f32);
        }
        // ASMHelper.degdiff(2): not translated, it calls Minecraft's `Mth`.degreesDifferenceAbs.
        // ASMHelper.raddiff(2): not translated, it calls Minecraft's `Mth`.degreesDifferenceAbs.
        // ASMHelper.catmullrom(5): not translated, it calls Minecraft's `Mth`.catmullrom.
        pub fn between(a: f32, b: f32, c: f32) -> bool {
            return (!(a > c)) && (!(a < b));
        }
        pub fn equals(x: f32, y: f32, epsilon: f32) -> bool {
            return ((y - x).abs()) <= epsilon;
        }
        // ASMHelper.notPausedNotValidation(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
        pub fn quadratic_bezier(t: f32, p0: f32, p1: f32, p2: f32) -> f32 {
            let one_minus_t: f32 = 1.0f32 - t;
            return (((one_minus_t * one_minus_t) * p0) + (((2.0f32 * one_minus_t) * t) * p1)) + ((t * t) * p2);
        }
        pub fn cubic_bezier(t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
            let one_minus_t: f32 = 1.0f32 - t;
            let one_minus_t_squared: f32 = one_minus_t * one_minus_t;
            let t_squared: f32 = t * t;
            return ((((one_minus_t_squared * one_minus_t) * p0) + (((3.0f32 * one_minus_t_squared) * t) * p1)) + (((3.0f32 * one_minus_t) * t_squared) * p2)) + ((t_squared * t) * p3);
        }
        pub fn hermite_interpolation(t: f32, p0: f32, p1: f32, m0: f32, m1: f32) -> f32 {
            let t_squared: f32 = t * t;
            let t_cubed: f32 = t_squared * t;
            let h00: f32 = ((2.0f32 * t_cubed) - (3.0f32 * t_squared)) + 1.0f32;
            let h10: f32 = (t_cubed - (2.0f32 * t_squared)) + t;
            let h01: f32 = ((-2.0f32) * t_cubed) + (3.0f32 * t_squared);
            let h11: f32 = t_cubed - t_squared;
            return (((h00 * p0) + (h10 * m0)) + (h01 * p1)) + (h11 * m1);
        }
        pub fn ease_in_quad(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + ((delta * t) * t);
        }
        pub fn ease_out_quad(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + ((delta * (-t)) * (t - 2.0f32));
        }
        pub fn ease_in_out_quad(mut t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            t /= 1.0f32;
            return if (t as f64) < 0.5f64 { start + (delta * ((2.0f32 * t) * t)) } else { start + (delta * ((((-2.0f32) * t) * (t - 2.0f32)) - 1.0f32)) };
        }
        pub fn ease_in_cubic(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (((delta * t) * t) * t);
        }
        // ASMHelper.easeOutCubic(3): not translated, `--` inside an expression.
        // ASMHelper.easeInOutCubic(3): not translated, `--` inside an expression.
        pub fn ease_in_quart(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + ((((delta * t) * t) * t) * t);
        }
        // ASMHelper.easeOutQuart(3): not translated, `--` inside an expression.
        // ASMHelper.easeInOutQuart(3): not translated, `--` inside an expression.
        pub fn ease_in_quint(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (((((delta * t) * t) * t) * t) * t);
        }
        // ASMHelper.easeOutQuint(3): not translated, `--` inside an expression.
        // ASMHelper.easeInOutQuint(3): not translated, `--` inside an expression.
        pub fn ease_in_sine(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * (1.0f32 - (((((t as f64) * std::f64::consts::PI) / 2.0f64).cos()) as f32)));
        }
        pub fn ease_out_sine(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * (((((t as f64) * std::f64::consts::PI) / 2.0f64).sin()) as f32));
        }
        pub fn ease_in_out_sine(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * (((-0.5f64) * (((std::f64::consts::PI * (t as f64)).cos()) - 1.0f64)) as f32));
        }
        pub fn ease_in_expo(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * ((2.0f64.powf(((10.0f32 * (t - 1.0f32)) as f64))) as f32));
        }
        pub fn ease_out_expo(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * (((-(2.0f64.powf((((-10.0f32) * t) as f64)))) + 1.0f64) as f32));
        }
        // ASMHelper.easeInOutExpo(3): not translated, `--` inside an expression.
        pub fn ease_in_circ(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * ((-((((1.0f32 - (t * t)) as f64).sqrt()) - 1.0f64)) as f32));
        }
        pub fn ease_out_circ(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            let t_minus1: f32 = t - 1.0f32;
            return start + (delta * ((((1.0f32 - (t_minus1 * t_minus1)) as f64).sqrt()) as f32));
        }
        pub fn ease_in_out_circ(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            let t_times2: f32 = t * 2.0f32;
            if t_times2 < 1.0f32 {
                return start + (delta * (((-0.5f64) * ((((1.0f32 - (t_times2 * t_times2)) as f64).sqrt()) - 1.0f64)) as f32));
            }
            let t_times2_minus2: f32 = t_times2 - 2.0f32;
            return start + (delta * ((0.5f64 * ((((1.0f32 - (t_times2_minus2 * t_times2_minus2)) as f64).sqrt()) + 1.0f64)) as f32));
        }
        // ASMHelper.easeInElastic(3): not translated, `--` inside an expression.
        pub fn ease_out_elastic(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * ((((2.0f64.powf((((-10.0f32) * t) as f64))) * (((((t as f64) - 0.075f64) * (std::f64::consts::PI * 2.0f64)) / 0.3f64).sin())) + 1.0f64) as f32));
        }
        // ASMHelper.easeInOutElastic(3): not translated, `--` inside an expression.
        // ASMHelper.easeInBounce(3): not translated, it calls ASMHelper.easeOutBounce, which isn't translated.
        // ASMHelper.easeOutBounce(3): not translated, an assignment inside an expression.
        // ASMHelper.easeInOutBounce(3): not translated, it calls ASMHelper.easeOutBounce, which isn't translated.
        pub fn ease_in_back(t: f32, start: f32, end: f32) -> f32 {
            let delta: f32 = end - start;
            return start + (delta * ((t * t) * ((2.70158f32 * t) - 1.70158f32)));
        }
        // ASMHelper.easeOutBack(3): not translated, `--` inside an expression.
        // ASMHelper.easeInOutBack(3): not translated, `--` inside an expression.
        // ASMHelper.functionName(2): not translated, it uses `Class`.
        // ASMHelper.fromMethod(2): not translated, it uses `Class`.
        // ASMHelper.methodDescriptor(1): not translated, it uses `Method` (java.lang.reflect).
        // ASMHelper.typeDescriptor(1): not translated, it uses `Class`.
    }

    /// `ASMHelper.ThrowingRunnable` (traben/entity_model_features/models/animation/math/asm/ASMHelper.java)
    pub mod asm_helper_throwing_runnable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ASMHelper.ThrowingRunnable.run(0): not translated, it has no body.
    }

    /// `ASMParser` (traben/entity_model_features/models/animation/math/asm/ASMParser.java)
    pub mod asm_parser {
        use super::*;
        pub static ID: LazyLock<std::sync::atomic::AtomicI64> = LazyLock::new(|| std::sync::atomic::AtomicI64::new(0i64));
        pub const CHUNK_INSNS: i32 = 2500;
        pub static P_BASTORE_FLOAT_TO_INT: LazyLock<JRegex> = LazyLock::new(|| JRegex::with_flags("bastore.*Reason:.*Type float .* is not assignable to integer", 32));
        pub static P_FASTORE_INT_TO_FLOAT: LazyLock<JRegex> = LazyLock::new(|| JRegex::with_flags("fastore.*Reason:.*Type integer .* is not assignable to float", 32));
        pub static P_FLOAT_TO_INT: LazyLock<JRegex> = LazyLock::new(|| JRegex::with_flags("Reason:.*Type float .* is not assignable to integer", 32));
        pub static P_INT_TO_FLOAT: LazyLock<JRegex> = LazyLock::new(|| JRegex::with_flags("Reason:.*Type integer .* is not assignable to float", 32));
        #[derive(Clone, Debug)]
        pub struct ASMParser {
        }
        impl ASMParser {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // ASMParser.getOrNull(2): not translated, it uses OldEMFAnimationHandler, which isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMParser.splitAndGetOrNull(4): not translated, it uses ASMAnimationHandler, which isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // ASMParser.compileOrNull(2): not translated, its body couldn't be read: an assert.
        // ASMParser.setupClass(1): not translated, it uses `ClassWriter` (org.objectweb.asm).
        // ASMParser.handleParseException(2): not translated, it uses AnimLineData, which isn't translated (it uses EMFModelPart, which isn't translated (it extends Minecraft's `ModelPart`)).
    }

    /// `ASMParser.ASMExecutor` (traben/entity_model_features/models/animation/math/asm/ASMParser.java)
    pub mod asm_parser_asm_executor {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ASMParser.ASMExecutor.execute(2): not translated, it has no body.
    }

    /// `ASMParser.NeedToSplitAnimationsException` (traben/entity_model_features/models/animation/math/asm/ASMParser.java)
    pub mod asm_parser_need_to_split_animations_exception {
        use super::*;
        // The class itself isn't translated: it extends `EMFException` (traben.entity_model_features). Its static parts are below.
        // ASMParser.NeedToSplitAnimationsException.NeedToSplitAnimationsException(0): not translated, its class isn't translated (it extends `EMFException` (traben.entity_model_features)).
    }

    /// `ASMVariableHandler` (traben/entity_model_features/models/animation/math/asm/ASMVariableHandler.java)
    pub mod asm_variable_handler {
        use super::*;
        // The class itself isn't translated: it uses `Stack` (java.util). Its static parts are below.
        // ASMVariableHandler.getLocalVarIndex(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.popLocalVarIndex(1): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.getFloatVarList(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.getBoolVarList(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.scope(1): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.scopeFloat(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.scopeBool(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.scopePop(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.verifyEndOfParse(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.isScopeBool(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.isScopeFloat(0): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.isReadVarName(1): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.isWriteVarName(1): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.getAndAssignVarIndex(2): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.asmVisitFrameCounter(1): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.asmVisitVar(2): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.asmStoreVar(2): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.asmInvertBoolean(1): not translated, its class isn't translated (it uses `Stack` (java.util)).
        // ASMVariableHandler.asmNegateFloat(1): not translated, its class isn't translated (it uses `Stack` (java.util)).
    }

    /// `ASMVisitable` (traben/entity_model_features/models/animation/math/asm/ASMVisitable.java)
    pub mod asm_visitable {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ASMVisitable.asmVisit(2): not translated, it has no body.
    }

    /// `MultiASMAnimationHandler` (traben/entity_model_features/models/animation/math/asm/MultiASMAnimationHandler.java)
    pub mod multi_asm_animation_handler {
        use super::*;
        // The class itself isn't translated: it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation). Its static parts are below.
        // MultiASMAnimationHandler.MultiASMAnimationHandler(3): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // MultiASMAnimationHandler.animateInner(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // MultiASMAnimationHandler.finishAndValidate(0): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
    }

    /// `EMFMathException` (traben/entity_model_features/models/animation/math/EMFMathException.java)
    pub mod emf_math_exception {
        use super::*;
        // The class itself isn't translated: it extends `EMFException` (traben.entity_model_features). Its static parts are below.
        // EMFMathException.EMFMathException(1): not translated, its class isn't translated (it extends `EMFException` (traben.entity_model_features)).
    }

    /// `MathBinaryExpressionComponent` (traben/entity_model_features/models/animation/math/expression_tree/MathBinaryExpressionComponent.java)
    pub mod math_binary_expression_component {
        use super::*;
        // The class itself isn't translated: it extends `MathValue`. Its static parts are below.
        // MathBinaryExpressionComponent.MathBinaryExpressionComponent(3): not translated, its class isn't translated (it extends `MathValue`).
        // MathBinaryExpressionComponent.getOptimizedExpression(3): not translated, it uses MathComponent, which isn't translated (it's an interface).
        // MathBinaryExpressionComponent.asmVisit(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathBinaryExpressionComponent.getResultSupplier(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathBinaryExpressionComponent.getResult(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathBinaryExpressionComponent.toString(0): not translated, its class isn't translated (it extends `MathValue`).
    }

    /// `MathComponent` (traben/entity_model_features/models/animation/math/expression_tree/MathComponent.java)
    pub mod math_component {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MathComponent.getResult(0): not translated, it has no body.
        // MathComponent.isConstant(0): not translated, its class isn't translated (it's an interface).
        // MathComponent.asmVisit(2): not translated, it has no body.
        // MathComponent.toConstant(0): not translated, its class isn't translated (it's an interface).
        // MathComponent.negativeDelegate(1): not translated, its body couldn't be read: an anonymous class.
        // MathComponent.invertedBooleanDelegate(1): not translated, its body couldn't be read: an anonymous class.
        // MathComponent.validateBooleanDelegate(1): not translated, its body couldn't be read: an anonymous class.
    }

    /// `MathConstant` (traben/entity_model_features/models/animation/math/expression_tree/MathConstant.java)
    pub mod math_constant {
        use super::*;
        // The class itself isn't translated: it extends `MathValue`. Its static parts are below.
        // ZERO_CONST: not translated, it uses MathConstant, which isn't translated (it extends `MathValue`).
        // FALSE_CONST: not translated, it uses MathConstant, which isn't translated (it extends `MathValue`).
        // MathConstant.MathConstant(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathConstant.MathConstant(1): not translated, its class isn't translated (it extends `MathValue`).
        // MathConstant.getResultSupplier(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathConstant.isConstant(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathConstant.toString(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathConstant.getResult(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathConstant.asmVisit(2): not translated, its class isn't translated (it extends `MathValue`).
    }

    /// `MathExpressionParser` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java)
    pub mod math_expression_parser {
        use super::*;
        // The class itself isn't translated: it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`). Its static parts are below.
        // NULL_EXPRESSION: not translated, its value couldn't be read: an anonymous class.
        // BOOLEAN_COMPARATOR_ACTIONS: not translated, it uses MathOperator, which isn't translated (its constants carry values).
        // BOOLEAN_LOGICAL_ACTIONS: not translated, it uses MathOperator, which isn't translated (its constants carry values).
        // MULTIPLICATION_ACTIONS: not translated, it uses MathOperator, which isn't translated (its constants carry values).
        // ADDITION_ACTIONS: not translated, it uses MathOperator, which isn't translated (its constants carry values).
        // MathExpressionParser.MathExpressionParser(4): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.getOptimizedExpression(3): not translated, it uses MathComponent, which isn't translated (it's an interface).
        // MathExpressionParser.getOptimizedExpression(4): not translated, it uses MathComponent, which isn't translated (it's an interface).
        // MathExpressionParser.readBracketContents(1): not translated, an Iterator passed as a parameter.
        // MathExpressionParser.readLastSingleBooleanAction(3): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.readDoubleBooleanAction(3): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.readMethodOrBrackets(2): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.readVariableOrConstant(1): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.validateAndOptimize(0): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.getNegativeNext(0): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.validateCalculationAndOptimize(0): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.optimiseTheseActionsIntoBinaryComponents(2): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
        // MathExpressionParser.toString(0): not translated, its class isn't translated (it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`)).
    }

    /// `MathExpressionParser.CalculationList` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java)
    pub mod math_expression_parser_calculation_list {
        use super::*;
        // The class itself isn't translated: it extends `ArrayList` (java.util). Its static parts are below.
        // MathExpressionParser.CalculationList.CalculationList(1): not translated, its class isn't translated (it extends `ArrayList` (java.util)).
        // MathExpressionParser.CalculationList.CalculationList(0): not translated, its class isn't translated (it extends `ArrayList` (java.util)).
        // MathExpressionParser.CalculationList.getLast(0): not translated, its class isn't translated (it extends `ArrayList` (java.util)).
    }

    /// `MathExpressionParser.RollingReader` (traben/entity_model_features/models/animation/math/expression_tree/MathExpressionParser.java)
    pub mod math_expression_parser_rolling_reader {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct RollingReader {
            pub builder: String,
        }
        impl RollingReader {
            pub fn new() -> Self {
                let mut s = Self { builder: String::new() };
                s
            }
            pub fn clear(&mut self) {
                self.builder = String::new();
            }
            pub fn write(&mut self, ch: char) {
                self.builder.push_str(&(format!("{}", ch)));
            }
            pub fn read(&mut self) -> String {
                let result: String = self.builder.to_string();
                self.clear();
                return result.to_string();
            }
            pub fn to_string(&self) -> String {
                return self.builder.to_string();
            }
            pub fn is_empty(&self) -> bool {
                return self.builder.is_empty();
            }
        }
    }

    /// `MathMethod` (traben/entity_model_features/models/animation/math/expression_tree/MathMethod.java)
    pub mod math_method {
        use super::*;
        // The class itself isn't translated: it extends `MathValue`. Its static parts are below.
        // MathMethod.MathMethod(3): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.MathMethod(4): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.MathMethod(5): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.parseArg(3): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.parseAllArgs(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.of(4): not translated, it uses MathMethod, which isn't translated (it extends `MathValue`).
        // MathMethod.getArgsList(1): not translated, Stream.iterator.
        // MathMethod.getOptimizedExpression(4): not translated, it uses MathComponent, which isn't translated (it's an interface).
        // MathMethod.setOptimizedAlternativeToThis(1): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.canOptimizeForConstantArgs(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.setSupplierAndOptimize(1): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.setSupplierAndOptimize(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.setSupplierAndOptimize(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.invertSupplierBoolean(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.setOptimizedIfPossible(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.getResultSupplier(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.asmVisit(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.asmVisitInner(2): not translated, it has no body.
        // MathMethod.isRawStringArg(1): not translated, its class isn't translated (it extends `MathValue`).
        // MathMethod.hasCorrectArgCount(1): not translated, it has no body.
    }

    /// `MathOperator` (traben/entity_model_features/models/animation/math/expression_tree/MathOperator.java)
    pub mod math_operator {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // MathOperator.getAction(1): not translated, it uses MathOperator, which isn't translated (its constants carry values).
        // MathOperator.execute(2): not translated, its class isn't translated (its constants carry values).
        // MathOperator.isConstant(0): not translated, its class isn't translated (its constants carry values).
        // MathOperator.getResult(0): not translated, its class isn't translated (its constants carry values).
        // MathOperator.isScopeBool(0): not translated, its class isn't translated (its constants carry values).
        // MathOperator.isEqualsType(0): not translated, its class isn't translated (its constants carry values).
        // MathOperator.asmVisit(2): not translated, its class isn't translated (its constants carry values).
        // MathOperator.asmCompare(2): not translated, it uses `MethodVisitor` (org.objectweb.asm).
    }

    /// `MathValue` (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java)
    pub mod math_value {
        use super::*;
        // The class itself isn't translated: it implements `MathComponent`. Its static parts are below.
        pub static TRUE: LazyLock<f32> = LazyLock::new(|| f32::INFINITY);
        pub static FALSE: LazyLock<f32> = LazyLock::new(|| f32::NEG_INFINITY);
        // MathValue.MathValue(1): not translated, its class isn't translated (it implements `MathComponent`).
        // MathValue.MathValue(0): not translated, its class isn't translated (it implements `MathComponent`).
        // MathValue.fromBoolean(1): not translated, it's overloaded with the same number of parameters.
        pub fn to_boolean(value: f32) -> bool {
            if value == f32::NEG_INFINITY {
                return false;
            } else if value == f32::INFINITY {
                return true;
            } else {
                panic!("{}", format!("Value [{}] is not a boolean", j_float((value as f64))));
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        pub fn validate_boolean(value: f32) -> f32 {
            math_value::to_boolean(value);
            return value;
        }
        // MathValue.invertBoolean(1): not translated, it's overloaded with the same number of parameters.
        // MathValue.invertBoolean(1): not translated, it's overloaded with the same number of parameters.
        // MathValue.invertBoolean(1): not translated, it's overloaded with the same number of parameters.
        // MathValue.fromBoolean(1): not translated, it's overloaded with the same number of parameters.
        // MathValue.invertBoolean(1): not translated, it's overloaded with the same number of parameters.
        pub fn is_boolean(value: f32) -> bool {
            return (value == f32::INFINITY) || (value == f32::NEG_INFINITY);
        }
        // MathValue.getResultSupplier(0): not translated, it has no body.
        // MathValue.getResult(0): not translated, its class isn't translated (it implements `MathComponent`).
    }

    /// `MathValue.ResultSupplier` (traben/entity_model_features/models/animation/math/expression_tree/MathValue.java)
    pub mod math_value_result_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MathValue.ResultSupplier.get(0): not translated, it has no body.
    }

    /// `MathVariable` (traben/entity_model_features/models/animation/math/expression_tree/MathVariable.java)
    pub mod math_variable {
        use super::*;
        // The class itself isn't translated: it extends `MathValue`. Its static parts are below.
        // MathVariable.MathVariable(3): not translated, its class isn't translated (it extends `MathValue`).
        // MathVariable.MathVariable(2): not translated, its class isn't translated (it extends `MathValue`).
        // MathVariable.getOptimizedVariable(3): not translated, it uses MathComponent, which isn't translated (it's an interface).
        // MathVariable.getResultSupplier(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathVariable.toString(0): not translated, its class isn't translated (it extends `MathValue`).
        // MathVariable.asmVisit(2): not translated, its class isn't translated (it extends `MathValue`).
    }

    /// `OldEMFAnimationHandler` (traben/entity_model_features/models/animation/math/expression_tree/OldEMFAnimationHandler.java)
    pub mod old_emf_animation_handler {
        use super::*;
        // The class itself isn't translated: it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation). Its static parts are below.
        // OldEMFAnimationHandler.OldEMFAnimationHandler(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // OldEMFAnimationHandler.addParsedLine(2): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // OldEMFAnimationHandler.getLastResultGetter(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // OldEMFAnimationHandler.prevResultsOfEntity(0): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // OldEMFAnimationHandler.lastResult(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // OldEMFAnimationHandler.finishAndValidate(0): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
        // OldEMFAnimationHandler.animateInner(1): not translated, its class isn't translated (it extends `EMFAnimationHandler` (traben.entity_model_features.models.animation)).
    }

    /// `CatchMethod` (traben/entity_model_features/models/animation/math/methods/emf/CatchMethod.java)
    pub mod catch_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // CatchMethod.CatchMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // CatchMethod.canOptimizeForConstantArgs(0): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // CatchMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // CatchMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `IfBMethod` (traben/entity_model_features/models/animation/math/methods/emf/IfBMethod.java)
    pub mod if_b_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // IfBMethod.IfBMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // IfBMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // IfBMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // IfBMethod.deepIf(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `KeyframeLoopMethod` (traben/entity_model_features/models/animation/math/methods/emf/KeyframeLoopMethod.java)
    pub mod keyframe_loop_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // KeyframeLoopMethod.KeyframeLoopMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // KeyframeLoopMethod.keyFrameLoopStatic(2): not translated, it calls Minecraft's `Mth`.catmullrom.
        // KeyframeLoopMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // KeyframeLoopMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `KeyframeMethod` (traben/entity_model_features/models/animation/math/methods/emf/KeyframeMethod.java)
    pub mod keyframe_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // KeyframeMethod.KeyframeMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // KeyframeMethod.keyFrameStatic(2): not translated, it calls Minecraft's `Mth`.catmullrom.
        // KeyframeMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // KeyframeMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `NBTMethod` (traben/entity_model_features/models/animation/math/methods/emf/NBTMethod.java)
    pub mod nbt_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // CACHE: not translated, it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties).
        // NULL: not translated, it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties).
        // NBTMethod.NBTMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // NBTMethod.nullz(0): not translated, it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties).
        // NBTMethod.nbtMethodStatic(2): not translated, it uses `NBTProperty` (traben.entity_texture_features.features.property_reading.properties.optifine_properties).
        // NBTMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // NBTMethod.canOptimizeForConstantArgs(0): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // NBTMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // NBTMethod.isRawStringArg(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `RandomBMethod` (traben/entity_model_features/models/animation/math/methods/emf/RandomBMethod.java)
    pub mod random_b_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // RandomBMethod.RandomBMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // RandomBMethod.nextValue(1): not translated, it calls RandomMethod.nextValue, which isn't translated.
        // RandomBMethod.nextValueBasic(0): not translated, it calls RandomMethod.nextValueBasic, which isn't translated.
        // RandomBMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // RandomBMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `MethodRegistry` (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java)
    pub mod method_registry {
        use super::*;
        // The class itself isn't translated: it uses MethodFactory, which isn't translated (it's an interface). Its static parts are below.
        // INSTANCE: not translated, it uses MethodRegistry, which isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.MethodRegistry(0): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        pub fn emf_translation_key(key: &str) -> String {
            return format!("entity_model_features.config.function_explanation.{}", key);
        }
        // MethodRegistry.getInstance(0): not translated, it uses MethodRegistry, which isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.getMethodExplanationTranslationKeys(0): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerHelperMethodFactory(1): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerHelperMethodFactory(2): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerSimpleMethodFactory(4): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerSimpleMethodFactory(3): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerSimpleMethodFactory(3): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerSimpleMethodFactory(3): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerSimpleMultiMethodFactory(3): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.register(3): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerAndWrapMethodFactory(2): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.registerAndWrapMethodFactory(3): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.containsMethod(1): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
        // MethodRegistry.getMethodFactory(1): not translated, its class isn't translated (it uses MethodFactory, which isn't translated (it's an interface)).
    }

    /// `MethodRegistry.MethodFactory` (traben/entity_model_features/models/animation/math/methods/MethodRegistry.java)
    pub mod method_registry_method_factory {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // MethodRegistry.MethodFactory.getMethod(3): not translated, it has no body.
    }

    /// `IfMethod` (traben/entity_model_features/models/animation/math/methods/optifine/IfMethod.java)
    pub mod if_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // IfMethod.IfMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // IfMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // IfMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // IfMethod.deepIf(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `InMethod` (traben/entity_model_features/models/animation/math/methods/optifine/InMethod.java)
    pub mod in_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // InMethod.InMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // InMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // InMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `MaxMethod` (traben/entity_model_features/models/animation/math/methods/optifine/MaxMethod.java)
    pub mod max_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // MaxMethod.MaxMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // MaxMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // MaxMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `MinMethod` (traben/entity_model_features/models/animation/math/methods/optifine/MinMethod.java)
    pub mod min_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // MinMethod.MinMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // MinMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // MinMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `PrintBMethod` (traben/entity_model_features/models/animation/math/methods/optifine/PrintBMethod.java)
    pub mod print_b_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // PrintBMethod.PrintBMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintBMethod.getPrintCount(0): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintBMethod.printStaticOne(2): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // PrintBMethod.printStatic(4): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // PrintBMethod.canOptimizeForConstantArgs(0): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintBMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintBMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintBMethod.isRawStringArg(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `PrintMethod` (traben/entity_model_features/models/animation/math/methods/optifine/PrintMethod.java)
    pub mod print_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // PrintMethod.PrintMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintMethod.getPrintCount(0): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintMethod.printStaticOne(2): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // PrintMethod.printStatic(4): not translated, it calls Minecraft's `Minecraft`.getInstance.
        // PrintMethod.canOptimizeForConstantArgs(0): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // PrintMethod.isRawStringArg(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `RandomMethod` (traben/entity_model_features/models/animation/math/methods/optifine/RandomMethod.java)
    pub mod random_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // RandomMethod.RandomMethod(3): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // RandomMethod.nextValue(1): not translated, it calls `Float`.floatToIntBits.
        // RandomMethod.nextValueBasic(0): not translated, Math.random (no random numbers without a crate).
        pub fn optifine_int_hash(mut x: i32) -> i32 {
            x = (x ^ 61) ^ (x >> 16);
            x += x << 3;
            x ^= x >> 4;
            x *= 668265261;
            return x ^ (x >> 15);
        }
        // RandomMethod.canOptimizeForConstantArgs(0): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // RandomMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // RandomMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `SimpleMethod` (traben/entity_model_features/models/animation/math/methods/SimpleMethod.java)
    pub mod simple_method {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // SimpleMethod.SimpleMethod(5): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // SimpleMethod.makeFactory(3): not translated, it uses MethodFactory, which isn't translated (it's an interface).
        // SimpleMethod.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // SimpleMethod.getParameterCount(1): not translated, it uses `Method` (java.lang.reflect).
        // SimpleMethod.getStringArgs(1): not translated, it uses `Method` (java.lang.reflect).
        // SimpleMethod.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `BiFunctionMethods` (traben/entity_model_features/models/animation/math/methods/simple/BiFunctionMethods.java)
    pub mod bi_function_methods {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // BiFunctionMethods.BiFunctionMethods(4): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // BiFunctionMethods.makeFactory(2): not translated, it uses MethodFactory, which isn't translated (it's an interface).
        // BiFunctionMethods.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // BiFunctionMethods.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `FunctionMethods` (traben/entity_model_features/models/animation/math/methods/simple/FunctionMethods.java)
    pub mod function_methods {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // FunctionMethods.FunctionMethods(4): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // FunctionMethods.makeFactory(2): not translated, it uses MethodFactory, which isn't translated (it's an interface).
        // FunctionMethods.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // FunctionMethods.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `MultiFunctionMethods` (traben/entity_model_features/models/animation/math/methods/simple/MultiFunctionMethods.java)
    pub mod multi_function_methods {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // MultiFunctionMethods.MultiFunctionMethods(4): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // MultiFunctionMethods.makeFactory(2): not translated, it uses MethodFactory, which isn't translated (it's an interface).
        // MultiFunctionMethods.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // MultiFunctionMethods.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `TriFunctionMethods` (traben/entity_model_features/models/animation/math/methods/simple/TriFunctionMethods.java)
    pub mod tri_function_methods {
        use super::*;
        // The class itself isn't translated: it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree). Its static parts are below.
        // TriFunctionMethods.TriFunctionMethods(4): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // TriFunctionMethods.makeFactory(2): not translated, it uses MethodFactory, which isn't translated (it's an interface).
        // TriFunctionMethods.asmVisitInner(2): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
        // TriFunctionMethods.hasCorrectArgCount(1): not translated, its class isn't translated (it extends `MathMethod` (traben.entity_model_features.models.animation.math.expression_tree)).
    }

    /// `EMFModelOrRenderVariable` (traben/entity_model_features/models/animation/math/variables/EMFModelOrRenderVariable.java)
    pub mod emf_model_or_render_variable {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EMFModelOrRenderVariable.getRenderVariable(1): not translated, it uses EMFModelOrRenderVariable, which isn't translated (its constants carry values).
        // EMFModelOrRenderVariable.get(1): not translated, it uses EMFModelOrRenderVariable, which isn't translated (its constants carry values).
        // EMFModelOrRenderVariable.isRenderVariable(0): not translated, its class isn't translated (its constants carry values).
        // EMFModelOrRenderVariable.isBoolean(0): not translated, its class isn't translated (its constants carry values).
        // EMFModelOrRenderVariable.getValue(1): not translated, it has no body.
        // EMFModelOrRenderVariable.getValue(0): not translated, its class isn't translated (its constants carry values).
        // EMFModelOrRenderVariable.setValue(2): not translated, it has no body.
    }

    /// `GlobalVariableFactory` (traben/entity_model_features/models/animation/math/variables/factories/GlobalVariableFactory.java)
    pub mod global_variable_factory {
        use super::*;
        // The class itself isn't translated: it extends `UniqueVariableFactory`. Its static parts are below.
        pub static GLOBAL_VARIABLES: LazyLock<JCell<JMap<String, f32>>> = LazyLock::new(|| JCell::new(JMap::new()));
        pub fn clear() {
            global_variable_factory::GLOBAL_VARIABLES.with(|o| o.clear());
        }
        pub fn set_global_variable(key: &str, value: f32) {
            { let a_0 = key.to_string(); let a_1 = value; global_variable_factory::GLOBAL_VARIABLES.with(|o| o.insert(a_0, a_1)) };
        }
        pub fn get_global_variable(key: &str) -> f32 {
            return { let a_0 = key.to_string(); let a_1 = 0.0f32; global_variable_factory::GLOBAL_VARIABLES.with(|o| o.get(&a_0).cloned().unwrap_or(a_1)) };
        }
        // GlobalVariableFactory.getSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // GlobalVariableFactory.getASMBoolSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // GlobalVariableFactory.getASMFloatSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // GlobalVariableFactory.createsThisVariable(1): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // GlobalVariableFactory.getExplanationTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // GlobalVariableFactory.getTitleTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
    }

    /// `ModelPartVariableFactory` (traben/entity_model_features/models/animation/math/variables/factories/ModelPartVariableFactory.java)
    pub mod model_part_variable_factory {
        use super::*;
        // The class itself isn't translated: it extends `UniqueVariableFactory`. Its static parts are below.
        // ModelPartVariableFactory.getSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelPartVariableFactory.getASMBoolSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelPartVariableFactory.getASMFloatSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelPartVariableFactory.createsThisVariable(1): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelPartVariableFactory.getExplanationTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelPartVariableFactory.getTitleTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
    }

    /// `ModelVariableFactory` (traben/entity_model_features/models/animation/math/variables/factories/ModelVariableFactory.java)
    pub mod model_variable_factory {
        use super::*;
        // The class itself isn't translated: it extends `UniqueVariableFactory`. Its static parts are below.
        // ModelVariableFactory.getSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelVariableFactory.createsThisVariable(1): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelVariableFactory.getExplanationTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // ModelVariableFactory.getTitleTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
    }

    /// `RenderVariableFactory` (traben/entity_model_features/models/animation/math/variables/factories/RenderVariableFactory.java)
    pub mod render_variable_factory {
        use super::*;
        // The class itself isn't translated: it extends `UniqueVariableFactory`. Its static parts are below.
        // RenderVariableFactory.getSupplierOrNull(2): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // RenderVariableFactory.createsThisVariable(1): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // RenderVariableFactory.getExplanationTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
        // RenderVariableFactory.getTitleTranslationKey(0): not translated, its class isn't translated (it extends `UniqueVariableFactory`).
    }

    /// `UniqueVariableFactory` (traben/entity_model_features/models/animation/math/variables/factories/UniqueVariableFactory.java)
    pub mod unique_variable_factory {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct UniqueVariableFactory {
        }
        impl UniqueVariableFactory {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // UniqueVariableFactory.equals(1): not translated, it uses `Object`.
            // UniqueVariableFactory.getSupplierOrNull(2): not translated, it has no body.
            // UniqueVariableFactory.getASMBoolSupplierOrNull(2): not translated, it uses AnimSetupContext, which isn't translated (it implements `AutoCloseable`).
            // UniqueVariableFactory.getASMFloatSupplierOrNull(2): not translated, it uses ResultSupplier, which isn't translated (it's an interface).
            // UniqueVariableFactory.createsThisVariable(1): not translated, it has no body.
            // UniqueVariableFactory.getExplanationTranslationKey(0): not translated, it has no body.
            // UniqueVariableFactory.getTitleTranslationKey(0): not translated, it has no body.
            // UniqueVariableFactory.printing(0): not translated, it calls EMF.config, which isn't translated.
        }
    }

    /// `VariableRegistry` (traben/entity_model_features/models/animation/math/variables/VariableRegistry.java)
    pub mod variable_registry {
        use super::*;
        // The class itself isn't translated: it uses MathComponent, which isn't translated (it's an interface). Its static parts are below.
        // INSTANCE: not translated, it uses VariableRegistry, which isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.getASMVarBoolOrDefault(2): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.getASMVarFloatOrDefault(2): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.VariableRegistry(0): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        pub fn emf_translation_key(key: &str) -> String {
            return format!("entity_model_features.config.variable_explanation.{}", key);
        }
        // VariableRegistry.getInstance(0): not translated, it uses VariableRegistry, which isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.getSingletonVariableExplanationTranslationKeys(0): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.getUniqueVariableFactories(0): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.registerContextVariable(1): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.registerSimpleFloatVariable(2): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.registerSimpleFloatVariable(3): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.registerSimpleSpellingErrorWarning(1): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.registerSimpleBoolVariable(2): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.registerSimpleBoolVariable(3): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.getVariable(3): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
        // VariableRegistry.printing(0): not translated, its class isn't translated (it uses MathComponent, which isn't translated (it's an interface)).
    }

    /// `EMFBipedPose` (traben/entity_model_features/models/animation/state/EMFBipedPose.java)
    pub mod emf_biped_pose {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EMFBipedPose {
            pub head: emf_biped_pose_pose_part::PosePart,
            pub body: emf_biped_pose_pose_part::PosePart,
            pub left_arm: emf_biped_pose_pose_part::PosePart,
            pub right_arm: emf_biped_pose_pose_part::PosePart,
            pub left_leg: emf_biped_pose_pose_part::PosePart,
            pub right_leg: emf_biped_pose_pose_part::PosePart,
            pub root: emf_biped_pose_pose_part::PosePart,
            pub apply_to_submit: bool,
        }
        impl EMFBipedPose {
            // EMFBipedPose.EMFBipedPose(1): not translated, it uses Minecraft's `HumanoidModel`.
            // EMFBipedPose.applyTo(1): not translated, it uses Minecraft's `HumanoidModel`.
        }
    }

    /// `EMFBipedPose.PosePart` (traben/entity_model_features/models/animation/state/EMFBipedPose.java)
    pub mod emf_biped_pose_pose_part {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PosePart {
            pub x_rot: f32,
            pub y_rot: f32,
            pub z_rot: f32,
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub x_scale: f32,
            pub y_scale: f32,
            pub z_scale: f32,
        }
        impl PosePart {
            // EMFBipedPose.PosePart.PosePart(1): not translated, it uses Minecraft's `ModelPart`.
            // EMFBipedPose.PosePart.applyTo(1): not translated, it uses Minecraft's `ModelPart`.
        }
    }

    /// `EMFEntityRenderState` (traben/entity_model_features/models/animation/state/EMFEntityRenderState.java)
    pub mod emf_entity_render_state {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EMFEntityRenderState.emfEntity(0): not translated, it has no body.
        // EMFEntityRenderState.isManualPlayerState(0): not translated, it has no body.
        // EMFEntityRenderState.setManualPlayerState(1): not translated, it has no body.
        // EMFEntityRenderState.manualPlayerState(0): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
        // EMFEntityRenderState.prevX(0): not translated, it has no body.
        // EMFEntityRenderState.x(0): not translated, it has no body.
        // EMFEntityRenderState.prevY(0): not translated, it has no body.
        // EMFEntityRenderState.y(0): not translated, it has no body.
        // EMFEntityRenderState.prevZ(0): not translated, it has no body.
        // EMFEntityRenderState.z(0): not translated, it has no body.
        // EMFEntityRenderState.prevPitch(0): not translated, it has no body.
        // EMFEntityRenderState.pitch(0): not translated, it has no body.
        // EMFEntityRenderState.isTouchingWater(0): not translated, it has no body.
        // EMFEntityRenderState.isOnFire(0): not translated, it has no body.
        // EMFEntityRenderState.hasVehicle(0): not translated, it has no body.
        // EMFEntityRenderState.isOnGround(0): not translated, it has no body.
        // EMFEntityRenderState.isAlive(0): not translated, it has no body.
        // EMFEntityRenderState.isGlowing(0): not translated, it has no body.
        // EMFEntityRenderState.isInLava(0): not translated, it has no body.
        // EMFEntityRenderState.isInvisible(0): not translated, it has no body.
        // EMFEntityRenderState.hasPassengers(0): not translated, it has no body.
        // EMFEntityRenderState.isSneaking(0): not translated, it has no body.
        // EMFEntityRenderState.isSprinting(0): not translated, it has no body.
        // EMFEntityRenderState.isWet(0): not translated, it has no body.
        // EMFEntityRenderState.age(0): not translated, it has no body.
        // EMFEntityRenderState.yaw(0): not translated, it has no body.
        // EMFEntityRenderState.emfVelocity(0): not translated, it has no body.
        // EMFEntityRenderState.typeString(0): not translated, it has no body.
        // EMFEntityRenderState.variableMap(0): not translated, it has no body.
        // EMFEntityRenderState.layerFactory(0): not translated, it has no body.
        // EMFEntityRenderState.setLayerFactory(1): not translated, it has no body.
        // EMFEntityRenderState.setBipedPose(1): not translated, it has no body.
        // EMFEntityRenderState.getBipedPose(0): not translated, it has no body.
        // EMFEntityRenderState.isFirstPersonHand(0): not translated, it has no body.
        // EMFEntityRenderState.setIsFirstPersonHand(1): not translated, it has no body.
        // EMFEntityRenderState.shadowSize(0): not translated, it has no body.
        // EMFEntityRenderState.setShadowSize(1): not translated, it has no body.
        // EMFEntityRenderState.shadowOpacity(0): not translated, it has no body.
        // EMFEntityRenderState.setShadowOpacity(1): not translated, it has no body.
        // EMFEntityRenderState.shadowX(0): not translated, it has no body.
        // EMFEntityRenderState.setShadowX(1): not translated, it has no body.
        // EMFEntityRenderState.shadowZ(0): not translated, it has no body.
        // EMFEntityRenderState.setShadowZ(1): not translated, it has no body.
        // EMFEntityRenderState.limbAngle(0): not translated, it has no body.
        // EMFEntityRenderState.setLimbAngle(1): not translated, it has no body.
        // EMFEntityRenderState.limbDistance(0): not translated, it has no body.
        // EMFEntityRenderState.setLimbDistance(1): not translated, it has no body.
        // EMFEntityRenderState.headYaw(0): not translated, it has no body.
        // EMFEntityRenderState.setHeadYaw(1): not translated, it has no body.
        // EMFEntityRenderState.headPitch(0): not translated, it has no body.
        // EMFEntityRenderState.setHeadPitch(1): not translated, it has no body.
        // EMFEntityRenderState.needsToModifyShadow(0): not translated, its class isn't translated (it's an interface).
        // EMFEntityRenderState.fireX(0): not translated, it has no body.
        // EMFEntityRenderState.setFireX(1): not translated, it has no body.
        // EMFEntityRenderState.fireY(0): not translated, it has no body.
        // EMFEntityRenderState.setFireY(1): not translated, it has no body.
        // EMFEntityRenderState.fireZ(0): not translated, it has no body.
        // EMFEntityRenderState.setFireZ(1): not translated, it has no body.
        // EMFEntityRenderState.fireHeight(0): not translated, it has no body.
        // EMFEntityRenderState.setFireHeight(1): not translated, it has no body.
        // EMFEntityRenderState.fireScale(0): not translated, it has no body.
        // EMFEntityRenderState.setFireScale(1): not translated, it has no body.
        // EMFEntityRenderState.needsToModifyFire(0): not translated, its class isn't translated (it's an interface).
        // EMFEntityRenderState.skipModelVariate(0): not translated, it has no body.
        // EMFEntityRenderState.setSkipModelVariate(1): not translated, it has no body.
        // EMFEntityRenderState.isSubmit(0): not translated, it has no body.
        // EMFEntityRenderState.setSubmit(1): not translated, it has no body.
        // EMFEntityRenderState.getEMFStateSnapshot(0): not translated, it has no body.
        // EMFEntityRenderState.setEMFStateSnapshot(1): not translated, it has no body.
        // EMFEntityRenderState.preSubmitActivate(2): not translated, its class isn't translated (it's an interface).
        // EMFEntityRenderState.activate(1): not translated, its class isn't translated (it's an interface).
        // EMFEntityRenderState.deactivate(1): not translated, its class isn't translated (it's an interface).
        // EMFEntityRenderState.from(1): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
    }

    /// `EMFEntityRenderStateViaReference` (traben/entity_model_features/models/animation/state/EMFEntityRenderStateViaReference.java)
    pub mod emf_entity_render_state_via_reference {
        use super::*;
        // The class itself isn't translated: it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state). Its static parts are below.
        // EMFEntityRenderStateViaReference.EMFEntityRenderStateViaReference(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.emfEntity(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isManualPlayerState(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setManualPlayerState(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.prevX(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.x(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.prevY(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.y(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.prevZ(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.z(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.prevPitch(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.pitch(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isTouchingWater(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isOnFire(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.hasVehicle(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isOnGround(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isAlive(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isGlowing(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isInLava(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isInvisible(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.hasPassengers(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isSneaking(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isSprinting(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isWet(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.age(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.yaw(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.emfVelocity(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.typeString(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.variableMap(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.layerFactory(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setLayerFactory(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setBipedPose(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.getBipedPose(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isFirstPersonHand(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setIsFirstPersonHand(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.skipModelVariate(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setSkipModelVariate(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.isSubmit(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setSubmit(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.getEMFStateSnapshot(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setEMFStateSnapshot(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.shadowSize(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setShadowSize(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.shadowOpacity(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setShadowOpacity(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.shadowX(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setShadowX(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.shadowZ(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setShadowZ(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.limbAngle(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setLimbAngle(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.limbDistance(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setLimbDistance(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.headYaw(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setHeadYaw(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.headPitch(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setHeadPitch(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.fireX(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setFireX(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.fireY(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setFireY(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.fireZ(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setFireZ(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.fireScale(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setFireScale(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.fireHeight(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.setFireHeight(1): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
        // EMFEntityRenderStateViaReference.toString(0): not translated, its class isn't translated (it extends `ETFEntityRenderStateViaReference` (traben.entity_texture_features.features.state)).
    }

    /// `EMFState` (traben/entity_model_features/models/animation/state/EMFState.java)
    pub mod emf_state {
        use super::*;
        pub static ANIMATION_HOOKS: LazyLock<JCell<Vec<emf_animation_api_emf_animation_hook::EMFAnimationHook>>> = LazyLock::new(|| JCell::new(Vec::new()));
        pub static FRAME_COUNTER: JCell<f32> = JCell::new(0.0f32);
        pub static IS_LAYER_PHASE: JCell<bool> = JCell::new(false);
        pub static IS_MAIN_PHASE: JCell<bool> = JCell::new(false);
        pub static IS_IN_SHOULDER_METHOD: JCell<bool> = JCell::new(false);
        pub static IS_LEFT_SHOULDER: JCell<bool> = JCell::new(false);
        pub static IS_IN_GUI: JCell<bool> = JCell::new(false);
        pub static IS_IN_GROUND_OVERRIDE: JCell<bool> = JCell::new(false);
        pub static IS_ON_HEAD: JCell<bool> = JCell::new(false);
        pub static IS_IN_HAND: JCell<bool> = JCell::new(false);
        pub static IS_IN_LEFT_HAND: LazyLock<JCell<Option<bool>>> = LazyLock::new(|| JCell::new(None));
        pub static IS_IN_HAND_ITEM_LAYER_TRANSFORM: JCell<bool> = JCell::new(false);
        pub static HAS_DONE_ARM_OVERRIDE: LazyLock<JCell<Option<bool>>> = LazyLock::new(|| JCell::new(None));
        pub static IS_IN_ITEM_FRAME: JCell<bool> = JCell::new(false);
        pub static MODEL_VARIATION_IGNORES_VISIBILITY: JCell<bool> = JCell::new(false);
        // SNAPSHOTS: not translated, a EMFState.EMFStateStaticSnapshot with no value (null in Java).
        pub static ANNOUNCE_MODELS: JCell<bool> = JCell::new(false);
        // entitiesToForceVanillaModel: not translated, it uses `UUID` (java.util).
        // forceVanillaModelListeners: not translated, it uses EMFEntity, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct EMFState {
        }
        impl EMFState {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EMFState.state(0): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
        // EMFState.emfEntity(0): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFState.clear(0): not translated, it calls `ETFState` (traben.entity_texture_features.features.state).clear.
        // EMFState.incFrameCount(0): not translated, it calls IrisShadowPassDetection.getInstance, which isn't translated.
        pub fn get_frame_counter() -> f32 {
            return emf_state::FRAME_COUNTER.get();
        }
        pub fn clear_frame_statics() {
            emf_state::IS_LAYER_PHASE.set(false);
            emf_state::IS_MAIN_PHASE.set(false);
            emf_state::IS_IN_SHOULDER_METHOD.set(false);
            emf_state::IS_LEFT_SHOULDER.set(false);
            emf_state::IS_IN_GUI.set(false);
            emf_state::MODEL_VARIATION_IGNORES_VISIBILITY.set(false);
            emf_state::IS_IN_GROUND_OVERRIDE.set(false);
            emf_state::IS_ON_HEAD.set(false);
            emf_state::IS_IN_HAND.set(false);
            emf_state::IS_IN_LEFT_HAND.set(None);
            emf_state::IS_IN_ITEM_FRAME.set(false);
            emf_state::IS_IN_HAND_ITEM_LAYER_TRANSFORM.set(false);
            emf_state::HAS_DONE_ARM_OVERRIDE.set(None);
        }
        // EMFState.captureStatics(0): not translated, it uses EMFState.SNAPSHOTS, which isn't translated (a EMFState.EMFStateStaticSnapshot with no value (null in Java)).
        // EMFState.anounceModels(1): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
        pub fn entry_and_value(entry: &str, value: &str) -> String {
            return format!("\n\u{a7}6 - {}:\u{a7}r {}", entry, value);
        }
        // EMFState.isEntityForcedToVanillaModel(1): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
        // EMFState.getLayerFromRecentFactoryOrETFOverrideOrTranslucent(1): not translated, it uses Minecraft's `RenderType`.
    }

    /// `EMFState.EMFStateStaticSnapshot` (traben/entity_model_features/models/animation/state/EMFState.java)
    pub mod emf_state_emf_state_static_snapshot {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct EMFStateStaticSnapshot {
            pub is_in_shoulder_method: bool,
            pub is_left_shoulder: bool,
            pub is_in_gui: bool,
            pub is_in_ground_override: bool,
            pub is_on_head: bool,
            pub is_in_hand: bool,
            pub is_in_item_frame: bool,
        }
        impl EMFStateStaticSnapshot {
            pub fn new(is_in_shoulder_method: bool, is_left_shoulder: bool, is_in_gui: bool, is_in_ground_override: bool, is_on_head: bool, is_in_hand: bool, is_in_item_frame: bool) -> Self {
                let mut s = Self { is_in_shoulder_method: is_in_shoulder_method, is_left_shoulder: is_left_shoulder, is_in_gui: is_in_gui, is_in_ground_override: is_in_ground_override, is_on_head: is_on_head, is_in_hand: is_in_hand, is_in_item_frame: is_in_item_frame };
                s.is_in_shoulder_method = is_in_shoulder_method;
                s.is_left_shoulder = is_left_shoulder;
                s.is_in_gui = is_in_gui;
                s.is_in_ground_override = is_in_ground_override;
                s.is_on_head = is_on_head;
                s.is_in_hand = is_in_hand;
                s.is_in_item_frame = is_in_item_frame;
                s
            }
            // EMFState.EMFStateStaticSnapshot.isDefault(0): not translated, `this` used as a value.
            pub fn restore_statics(&mut self) {
                self.is_in_shoulder_method = self.is_in_shoulder_method;
                self.is_left_shoulder = self.is_left_shoulder;
                self.is_in_gui = self.is_in_gui;
                self.is_in_ground_override = self.is_in_ground_override;
                self.is_on_head = self.is_on_head;
                self.is_in_hand = self.is_in_hand;
                self.is_in_item_frame = self.is_in_item_frame;
            }
            pub fn is_in_shoulder_method(&self) -> bool {
                return self.is_in_shoulder_method;
            }
            pub fn is_left_shoulder(&self) -> bool {
                return self.is_left_shoulder;
            }
            pub fn is_in_gui(&self) -> bool {
                return self.is_in_gui;
            }
            pub fn is_in_ground_override(&self) -> bool {
                return self.is_in_ground_override;
            }
            pub fn is_on_head(&self) -> bool {
                return self.is_on_head;
            }
            pub fn is_in_hand(&self) -> bool {
                return self.is_in_hand;
            }
            pub fn is_in_item_frame(&self) -> bool {
                return self.is_in_item_frame;
            }
        }
    }

    /// `EMFModelMappings` (traben/entity_model_features/models/EMFModelMappings.java)
    pub mod emf_model_mappings {
        use super::*;
        pub static UNKNOWN_MODEL_MAP_CACHE: LazyLock<JMap<String, JMap<String, String>>> = LazyLock::new(|| JMap::new());
        pub static OPTIFINE_MODEL_MAP_CACHE: LazyLock<JCell<JMap<String, JMap<String, String>>>> = LazyLock::new(|| JCell::new(JMap::new()));
        // genericNonPlayerBiped: not translated, it calls EMFModelMappings.partMapping, which isn't translated.
        #[derive(Clone, Debug)]
        pub struct EMFModelMappings {
        }
        impl EMFModelMappings {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EMFModelMappings.initOptifineMappings(0): not translated, its body couldn't be read: `var`.
        // EMFModelMappings.partMapping(1): not translated, Minecraft's entry kept in a variable or field.
        // EMFModelMappings.partMapping(2): not translated, Minecraft's entry kept in a variable or field.
        // EMFModelMappings.isKnownMapping(1): not translated, it uses EMFModel_ID, which isn't translated (it implements `Comparable`).
        // EMFModelMappings.getMapOf(2): not translated, it uses EMFModel_ID, which isn't translated (it implements `Comparable`).
        // EMFModelMappings.getMapOf(5): not translated, it uses EMFModel_ID, which isn't translated (it implements `Comparable`).
        // EMFModelMappings.exportKnown(5): not translated, it uses EMFModel_ID, which isn't translated (it implements `Comparable`).
        pub fn get_known_map(mob_name: &str) -> JMap<String, String> {
            return ({ let a_0 = mob_name.to_string(); emf_model_mappings::OPTIFINE_MODEL_MAP_CACHE.with(|o| o.get(&a_0).cloned()) }).expect("null");
        }
        // EMFModelMappings.exploreProvidedEntityModelAndExportIfNeeded(5): not translated, it uses Minecraft's `ModelPart`.
        // EMFModelMappings.printModel(3): not translated, it calls `ETF` (traben.entity_texture_features).getConfigDirectory.
        // EMFModelMappings.addArrayProperty(3): not translated, it's overloaded with the same number of parameters.
        // EMFModelMappings.addArrayProperty(3): not translated, it's overloaded with the same number of parameters.
        // EMFModelMappings.partPrinter(5): not translated, it uses Minecraft's `ModelPart`.
        // EMFModelMappings.getChildByName(5): not translated, it uses Minecraft's `ModelPart`.
        // EMFModelMappings.mapThisAndChildren(5): not translated, it uses Minecraft's `ModelPart`.
    }

    /// `EMFModelMappings.OptifineMapper` (traben/entity_model_features/models/EMFModelMappings.java)
    pub mod emf_model_mappings_optifine_mapper {
        use super::*;
        pub static EXCEPTIONS: LazyLock<JSet<String>> = LazyLock::new(|| JSet::from_iter(["helmet".to_string(), "chestplate".to_string(), "leggings".to_string(), "boots".to_string()]));
        #[derive(Clone, Debug)]
        pub struct OptifineMapper {
            pub model_names: Vec<String>,
        }
        impl OptifineMapper {
            pub fn new(model_names: &[String]) -> Self {
                let mut s = Self { model_names: Vec::new() };
                s.model_names = model_names.to_vec();
                s
            }
            // EMFModelMappings.OptifineMapper.parts(1): not translated, it calls EMFUtils.logError, which isn't translated.
        }
        pub fn models(model_names: &[String]) -> emf_model_mappings_optifine_mapper::OptifineMapper {
            return emf_model_mappings_optifine_mapper::OptifineMapper::new(model_names);
        }
    }

    /// `EMFModelMappings.PartAndOffsets` (traben/entity_model_features/models/EMFModelMappings.java)
    pub mod emf_model_mappings_part_and_offsets {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `ModelPart`. Its static parts are below.
        // EMFModelMappings.PartAndOffsets.PartAndOffsets(4): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // EMFModelMappings.PartAndOffsets.part(0): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // EMFModelMappings.PartAndOffsets.x(0): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // EMFModelMappings.PartAndOffsets.y(0): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
        // EMFModelMappings.PartAndOffsets.z(0): not translated, its class isn't translated (it uses Minecraft's `ModelPart`).
    }

    /// `EMFModel_ID` (traben/entity_model_features/models/EMFModel_ID.java)
    pub mod emf_model_id {
        use super::*;
        // The class itself isn't translated: it implements `Comparable`. Its static parts are below.
        // EMFModel_ID.setCacheID(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.resetCacheID(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.getCacheID(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.EMFModel_ID(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.EMFModel_ID(2): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.EMFModel_ID(3): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.setFileName(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.equals(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.hashCode(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.toString(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.setBoth(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.areBothSame(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.setBoth(2): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.getfileName(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.hasFallbackModels(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.forEachFallback(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.getNextFallbackModel(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.addFallbackModel(2): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.addFallbackModel(3): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.pushNewMainModelAddingOldAsFallback(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.pushNewMainModelAndMapIdAddingOldAsFallback(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.pushNewMainModelAndMapIdAddingOldAsFallback(2): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.addFallbackModel(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.setMapIdAndAddFallbackModel(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.setMapIdAndAddFallbackModel(2): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.getNamespace(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.finishAndPrepAutomatedFallbacks(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.assertNamespaceAndCreateDeprecatedModdedFileName(2): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.getMapId(0): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.compareTo(1): not translated, its class isn't translated (it implements `Comparable`).
        // EMFModel_ID.getDisplayFileName(0): not translated, its class isn't translated (it implements `Comparable`).
    }

    /// `EMFModel_ID.FallbackModel` (traben/entity_model_features/models/EMFModel_ID.java)
    pub mod emf_model_id_fallback_model {
        use super::*;
        #[derive(Clone, Debug, PartialEq)]
        pub struct FallbackModel {
            pub namespace: String,
            pub file_name: Option<String>,
        }
        impl FallbackModel {
            pub fn new(namespace: &str, file_name: Option<String>) -> Self {
                let mut s = Self { namespace: String::new(), file_name: file_name.clone() };
                s.namespace = namespace.to_string();
                s.file_name = file_name.clone();
                s
            }
            pub fn namespace(&self) -> String {
                return self.namespace.to_string();
            }
            pub fn file_name(&self) -> Option<String> {
                return self.file_name.clone();
            }
        }
    }

    /// `EMFPartialArmor` (traben/entity_model_features/models/EMFPartialArmor.java)
    pub mod emf_partial_armor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EMFPartialArmor {
            pub has_custom: bool,
        }
        impl EMFPartialArmor {
            pub fn new() -> Self {
                let mut s = Self { has_custom: false };
                s
            }
            pub fn has_custom(&self) -> bool {
                return self.has_custom;
            }
        }
    }

    /// `IEMFModel` (traben/entity_model_features/models/IEMFModel.java)
    pub mod iemf_model {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEMFModel.emf$isEMFModel(0): not translated, it has no body.
        // IEMFModel.emf$getEMFRootModel(0): not translated, it has no body.
    }

    /// `IEMFModelNameContainer` (traben/entity_model_features/models/IEMFModelNameContainer.java)
    pub mod iemf_model_name_container {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEMFModelNameContainer.emf$insertKnownMappings(1): not translated, it has no body.
    }

    /// `EMFBoxData` (traben/entity_model_features/models/jem_objects/EMFBoxData.java)
    pub mod emf_box_data {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct EMFBoxData {
            pub texture_offset: Vec<f32>,
            pub uv_down: Vec<f32>,
            pub uv_up: Vec<f32>,
            pub uv_front: Vec<f32>,
            pub uv_back: Vec<f32>,
            pub uv_left: Vec<f32>,
            pub uv_right: Vec<f32>,
            pub uv_north: Vec<f32>,
            pub uv_south: Vec<f32>,
            pub uv_west: Vec<f32>,
            pub uv_east: Vec<f32>,
            pub coordinates: Vec<f32>,
            pub size_add: f32,
            pub size_add_x: f32,
            pub size_add_y: f32,
            pub size_add_z: f32,
            pub sizes_add: Vec<f32>,
        }
        impl EMFBoxData {
            pub fn new() -> Self {
                let mut s = Self { texture_offset: vec![0.0f32; 0], uv_down: vec![0.0f32; 0], uv_up: vec![0.0f32; 0], uv_front: vec![0.0f32; 0], uv_back: vec![0.0f32; 0], uv_left: vec![0.0f32; 0], uv_right: vec![0.0f32; 0], uv_north: vec![0.0f32; 0], uv_south: vec![0.0f32; 0], uv_west: vec![0.0f32; 0], uv_east: vec![0.0f32; 0], coordinates: vec![0.0f32; 0], size_add: 0.0f32, size_add_x: 0.0f32, size_add_y: 0.0f32, size_add_z: 0.0f32, sizes_add: vec![0.0f32; 0] };
                s
            }
            // EMFBoxData.prepare(3): not translated, a catch block that uses its exception.
            // EMFBoxData.validateUV(2): not translated, it calls `Arrays` (java.util).toString.
            pub fn check_and_fix_uv_legacy_directions(&mut self) {
                if (self.uv_front.len() as i32) == 4 {
                    self.uv_north = self.uv_front.to_vec();
                }
                if (self.uv_back.len() as i32) == 4 {
                    self.uv_west = self.uv_back.to_vec();
                }
                if (self.uv_left.len() as i32) == 4 {
                    self.uv_north = self.uv_left.to_vec();
                }
                if (self.uv_right.len() as i32) == 4 {
                    self.uv_east = self.uv_right.to_vec();
                }
            }
            // EMFBoxData.toString(0): not translated, it calls `Arrays` (java.util).toString.
        }
    }

    /// `EMFJemData` (traben/entity_model_features/models/jem_objects/EMFJemData.java)
    pub mod emf_jem_data {
        use super::*;
        // The class itself isn't translated: it uses `LinkedList` (java.util). Its static parts are below.
        pub static WHITESPACE: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("\\s"));
        pub static THIS: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("(?<=\\W|^)this(?=\\W)"));
        pub static PART: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("(?<=\\W|^)part(?=\\W)"));
        // EMFJemData.getAllTopLevelAnimationsByVanillaPartName(0): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
        // EMFJemData.getMobModelIDInfo(0): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
        // EMFJemData.getCustomTexture(0): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
        // EMFJemData.validateJemTexture(1): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
        // EMFJemData.validateJemTexture(2): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
        // EMFJemData.validateResourcePathAndExists(2): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
        // EMFJemData.prepare(2): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
        // EMFJemData.processAnimAndKeyString(2): not translated, it uses EMFPartData, which isn't translated (it holds an object of its own class (a linked structure)).
        // EMFJemData.toString(0): not translated, its class isn't translated (it uses `LinkedList` (java.util)).
    }

    /// `EMFPartData` (traben/entity_model_features/models/jem_objects/EMFPartData.java)
    pub mod emf_part_data {
        use super::*;
        // The class itself isn't translated: it holds an object of its own class (a linked structure). Its static parts are below.
        // EMFPartData.getCustomTexture(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EMFPartData.equals(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EMFPartData.hashCode(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EMFPartData.copyFrom(1): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EMFPartData.getAttachments(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EMFPartData.prepare(2): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
        // EMFPartData.toString(0): not translated, its class isn't translated (it holds an object of its own class (a linked structure)).
    }

    /// `EMFModelPart` (traben/entity_model_features/models/parts/EMFModelPart.java)
    pub mod emf_model_part {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ModelPart`. Its static parts are below.
        // EMFModelPart.getRoot(0): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.EMFModelPart(3): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.getAttachmentPositioner(1): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.render(5): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.lines(0): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.renderDebugTinted(5): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.renderWithTextureOverride(5): not translated, its body couldn't be read: an anonymous class.
        // EMFModelPart.renderTextureOverrideWithoutReset(5): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.renderLikeVanilla(5): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.testForBuildingException(1): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.renderLikeETF(5): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.renderBoxes(3): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.debugBoxColor(0): not translated, it has no body.
        // EMFModelPart.compile(5): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.simplePrintChildren(1): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.toStringShort(0): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.toString(0): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.getVanillaModelPartsOfCurrentState(0): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
        // EMFModelPart.getAllChildPartsAsAnimationMap(3): not translated, its class isn't translated (it extends Minecraft's `ModelPart`).
    }

    /// `EMFModelPartCustom` (traben/entity_model_features/models/parts/EMFModelPartCustom.java)
    pub mod emf_model_part_custom {
        use super::*;
        // The class itself isn't translated: it extends `EMFModelPart`. Its static parts are below.
        // EMFModelPartCustom.EMFModelPartCustom(5): not translated, its class isn't translated (it extends `EMFModelPart`).
        // EMFModelPartCustom.resetPose(0): not translated, its class isn't translated (it extends `EMFModelPart`).
        // EMFModelPartCustom.getCuboidsFromData(1): not translated, it uses Minecraft's `Cube`.
        // EMFModelPartCustom.getChildrenFromData(3): not translated, it uses Minecraft's `ModelPart`.
        // EMFModelPartCustom.createCuboidsFromBoxDataV3(1): not translated, it uses Minecraft's `Cube`.
        // EMFModelPartCustom.toString(0): not translated, its class isn't translated (it extends `EMFModelPart`).
        // EMFModelPartCustom.toStringShort(0): not translated, its class isn't translated (it extends `EMFModelPart`).
        // EMFModelPartCustom.getAttachmentPositioner(1): not translated, its class isn't translated (it extends `EMFModelPart`).
        // EMFModelPartCustom.debugBoxColor(0): not translated, its class isn't translated (it extends `EMFModelPart`).
        // EMFModelPartCustom.renderWithTextureOverride(5): not translated, its class isn't translated (it extends `EMFModelPart`).
    }

    /// `EMFModelPartCustom.EMFCube` (traben/entity_model_features/models/parts/EMFModelPartCustom.java)
    pub mod emf_model_part_custom_emf_cube {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Cube`. Its static parts are below.
        // EMFModelPartCustom.EMFCube.EMFCube(16): not translated, its body couldn't be read: an anonymous class.
        // EMFModelPartCustom.EMFCube.EMFCube(20): not translated, its body couldn't be read: an anonymous class.
        // EMFModelPartCustom.EMFCube.compile(5): not translated, its class isn't translated (it extends Minecraft's `Cube`).
    }

    /// `EMFModelPartRoot` (traben/entity_model_features/models/parts/EMFModelPartRoot.java)
    pub mod emf_model_part_root {
        use super::*;
        // The class itself isn't translated: it extends `EMFModelPartVanilla`. Its static parts are below.
        // EMFModelPartRoot.getPositionerForAttachment(1): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.getRoot(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.EMFModelPartRoot(5): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.debugBoxColor(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.getAllVanillaPartsEMF(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.getAllVanillaPartsByNameEMF(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.oneTimeRunnable(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.registerModelRunnableWithEntityTypeContext(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.doVariantCheck(1): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.checkIfShouldExpireEntity(2): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.addAndSetVariantOfJem(2): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.discoverAndInitVariants(1): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.setVariant1ToVanilla0(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.hasAnimation(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.setVariantStateTo(1): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.animate(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.animateNoPause(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.receiveAnimationHandler(2): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.getTopLevelJemTexture(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.resetVanillaPartsToDefaults(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.toString(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
        // EMFModelPartRoot.toStringShort(0): not translated, its class isn't translated (it extends `EMFModelPartVanilla`).
    }

    /// `EMFModelPartVanilla` (traben/entity_model_features/models/parts/EMFModelPartVanilla.java)
    pub mod emf_model_part_vanilla {
        use super::*;
        // The class itself isn't translated: it extends `EMFModelPartWithState`. Its static parts are below.
        // EMFModelPartVanilla.EMFModelPartVanilla(5): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
        // EMFModelPartVanilla.getAllEMFCustomChildren(0): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
        // EMFModelPartVanilla.translateAndRotate(1): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
        // EMFModelPartVanilla.debugBoxColor(0): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
        // EMFModelPartVanilla.render(5): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
        // EMFModelPartVanilla.setHideInTheseStates(1): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
        // EMFModelPartVanilla.toString(0): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
        // EMFModelPartVanilla.toStringShort(0): not translated, its class isn't translated (it extends `EMFModelPartWithState`).
    }

    /// `EBEConfigModifier` (traben/entity_model_features/mod_compat/EBEConfigModifier.java)
    pub mod ebe_config_modifier {
        use super::*;
        // The class itself isn't translated: it implements `BiConsumer` (java.util.function). Its static parts are below.
        pub static CHESTS_DISABLED: JCell<bool> = JCell::new(false);
        pub static SHULKER_BOXES_DISABLED: JCell<bool> = JCell::new(false);
        pub static BELLS_DISABLED: JCell<bool> = JCell::new(false);
        pub static BEDS_DISABLED: JCell<bool> = JCell::new(false);
        pub static SIGNS_DISABLED: JCell<bool> = JCell::new(false);
        pub static DECORATED_POTS_DISABLED: JCell<bool> = JCell::new(false);
        // ebeReloader: not translated, a function kept in a variable or field (only parameters can be).
        // EBEConfigModifier.modifyEBEConfig(1): not translated, it uses EBEConfigModifier.ebeReloader, which isn't translated (a function kept in a variable or field (only parameters can be)).
        // EBEConfigModifier.accept(2): not translated, its class isn't translated (it implements `BiConsumer` (java.util.function)).
        // EBEConfigModifier.accept(1): not translated, its class isn't translated (it implements `BiConsumer` (java.util.function)).
    }

    /// `IrisShadowPassDetection` (traben/entity_model_features/mod_compat/IrisShadowPassDetection.java)
    pub mod iris_shadow_pass_detection {
        use super::*;
        pub static INSTANCE: LazyLock<JCell<Option<iris_shadow_pass_detection::IrisShadowPassDetection>>> = LazyLock::new(|| JCell::new(None));
        #[derive(Clone, Debug)]
        pub struct IrisShadowPassDetection {
        }
        impl IrisShadowPassDetection {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
            // IrisShadowPassDetection.inShadowPass(0): not translated, it has no body.
        }
        // IrisShadowPassDetection.getInstance(0): not translated, its body couldn't be read: an anonymous class.
    }

    /// `IrisShadowPassDetection.IrisShadowPassDetectionImpl` (traben/entity_model_features/mod_compat/IrisShadowPassDetection.java)
    pub mod iris_shadow_pass_detection_iris_shadow_pass_detection_impl {
        use super::*;
        // The class itself isn't translated: it extends `IrisShadowPassDetection`. Its static parts are below.
        // IrisShadowPassDetection.IrisShadowPassDetectionImpl.IrisShadowPassDetectionImpl(0): not translated, its class isn't translated (it extends `IrisShadowPassDetection`).
        // IrisShadowPassDetection.IrisShadowPassDetectionImpl.inShadowPass(0): not translated, its class isn't translated (it extends `IrisShadowPassDetection`).
    }

    /// `PALCompat` (traben/entity_model_features/mod_compat/PALCompat.java)
    pub mod pal_compat {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct PALCompat {
        }
        impl PALCompat {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // PALCompat.shouldPauseEntityAnim(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
    }

    /// `EntityVariableBooleanProperty` (traben/entity_model_features/propeties/EntityVariableBooleanProperty.java)
    pub mod entity_variable_boolean_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        // EntityVariableBooleanProperty.EntityVariableBooleanProperty(2): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableBooleanProperty.getPropertyOrNull(2): not translated, it uses EntityVariableBooleanProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableBooleanProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableBooleanProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableBooleanProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `EntityVariableFloatProperty` (traben/entity_model_features/propeties/EntityVariableFloatProperty.java)
    pub mod entity_variable_float_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        // EntityVariableFloatProperty.EntityVariableFloatProperty(2): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableFloatProperty.getPropertyOrNull(2): not translated, it uses EntityVariableFloatProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableFloatProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableFloatProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // EntityVariableFloatProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `EntityVariableFloatProperty.InnerTester` (traben/entity_model_features/propeties/EntityVariableFloatProperty.java)
    pub mod entity_variable_float_property_inner_tester {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // EntityVariableFloatProperty.InnerTester.InnerTester(3): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // EntityVariableFloatProperty.InnerTester.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // EntityVariableFloatProperty.InnerTester.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `GlobalVariableBooleanProperty` (traben/entity_model_features/propeties/GlobalVariableBooleanProperty.java)
    pub mod global_variable_boolean_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        // GlobalVariableBooleanProperty.GlobalVariableBooleanProperty(2): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableBooleanProperty.getPropertyOrNull(2): not translated, it uses GlobalVariableBooleanProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableBooleanProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableBooleanProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableBooleanProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `GlobalVariableFloatProperty` (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java)
    pub mod global_variable_float_property {
        use super::*;
        // The class itself isn't translated: it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties). Its static parts are below.
        // GlobalVariableFloatProperty.GlobalVariableFloatProperty(2): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableFloatProperty.getPropertyOrNull(2): not translated, it uses GlobalVariableFloatProperty, which isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableFloatProperty.testEntityInternal(1): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableFloatProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
        // GlobalVariableFloatProperty.getPrintableRuleInfo(0): not translated, its class isn't translated (it extends `RandomProperty` (traben.entity_texture_features.features.property_reading.properties)).
    }

    /// `GlobalVariableFloatProperty.InnerTester` (traben/entity_model_features/propeties/GlobalVariableFloatProperty.java)
    pub mod global_variable_float_property_inner_tester {
        use super::*;
        // The class itself isn't translated: it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // GlobalVariableFloatProperty.InnerTester.InnerTester(3): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // GlobalVariableFloatProperty.InnerTester.getRangeValueFromEntity(1): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // GlobalVariableFloatProperty.InnerTester.getPropertyIds(0): not translated, its class isn't translated (it extends `FloatRangeFromStringArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ModelRuleIndexProperty` (traben/entity_model_features/propeties/ModelRuleIndexProperty.java)
    pub mod model_rule_index_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ModelRuleIndexProperty.ModelRuleIndexProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModelRuleIndexProperty.getPropertyOrNull(2): not translated, it uses ModelRuleIndexProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModelRuleIndexProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModelRuleIndexProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `ModelSuffixProperty` (traben/entity_model_features/propeties/ModelSuffixProperty.java)
    pub mod model_suffix_property {
        use super::*;
        // The class itself isn't translated: it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties). Its static parts are below.
        // ModelSuffixProperty.ModelSuffixProperty(2): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModelSuffixProperty.getPropertyOrNull(2): not translated, it uses ModelSuffixProperty, which isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModelSuffixProperty.getPropertyIds(0): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
        // ModelSuffixProperty.getValueFromEntity(1): not translated, its class isn't translated (it extends `SimpleIntegerArrayProperty` (traben.entity_texture_features.features.property_reading.properties.generic_properties)).
    }

    /// `EMFAnimationPauseHandler` (traben/entity_model_features/utils/EMFAnimationPauseHandler.java)
    pub mod emf_animation_pause_handler {
        use super::*;
        // entitiesPausedParts: not translated, it uses `UUID` (java.util).
        // entitiesPaused: not translated, it uses `UUID` (java.util).
        // pauseListeners: not translated, it uses EMFEntity, which isn't translated (it's an interface).
        pub static CHECKED_IF_I_EMOTE_PLAYER_EXISTS: JCell<bool> = JCell::new(false);
        // iEmotePlayerEntityType: not translated, it uses `Class`.
        // isPlayingEmoteMethod: not translated, it uses `Method` (java.lang.reflect).
        pub static SKIP_ESSENTIAL: JCell<bool> = JCell::new(false);
        // essentialIsPoseModifiedMethod: not translated, it uses `Method` (java.lang.reflect).
        #[derive(Clone, Debug)]
        pub struct EMFAnimationPauseHandler {
        }
        impl EMFAnimationPauseHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // EMFAnimationPauseHandler.shouldAnimationsPause(1): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
        // EMFAnimationPauseHandler.isPlayerEmoting_KosmX_mod(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationPauseHandler.getIEmotePlayerEntityType(0): not translated, it uses `Class`.
        // EMFAnimationPauseHandler.getIsPlayingEmoteMethod(0): not translated, it uses `Method` (java.lang.reflect).
        // EMFAnimationPauseHandler.isPlayerEmoting_Essential(1): not translated, it uses EMFEntity, which isn't translated (it's an interface).
        // EMFAnimationPauseHandler.getEntityPartsAnimPaused(0): not translated, it uses Minecraft's `ModelPart`.
    }

    /// `EMFDirectoryHandler` (traben/entity_model_features/utils/EMFDirectoryHandler.java)
    pub mod emf_directory_handler {
        use super::*;
        // The class itself isn't translated: it uses EMFDirectory, which isn't translated (its constants carry values). Its static parts are below.
        // EMFDirectoryHandler.EMFDirectoryHandler(4): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.basic(1): not translated, it uses EMFDirectoryHandler, which isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getDirectoryManagerOrNull(4): not translated, it uses EMFDirectoryHandler, which isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getFileNameWithType(0): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getRelativeDirectoryLocationNoValidation(1): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.packIndex(0): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.validForThisBase(1): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.foundModel(0): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getResourceOrNull(3): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getPackId(1): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getPackIndex(2): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getFinalFileLocation(0): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.getRelativeFilePossiblyEMFOverridden(1): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
        // EMFDirectoryHandler.toString(0): not translated, its class isn't translated (it uses EMFDirectory, which isn't translated (its constants carry values)).
    }

    /// `EMFDirectoryHandler.EMFDirectory` (traben/entity_model_features/utils/EMFDirectoryHandler.java)
    pub mod emf_directory_handler_emf_directory {
        use super::*;
        // The class itself isn't translated: its constants carry values. Its static parts are below.
        // EMFDirectoryHandler.EMFDirectory.getAsDirectory(2): not translated, it has no body.
        // EMFDirectoryHandler.EMFDirectory.fallback(0): not translated, its class isn't translated (its constants carry values).
        // EMFDirectoryHandler.EMFDirectory.override(0): not translated, its class isn't translated (its constants carry values).
    }

    /// `EMFEntity` (traben/entity_model_features/utils/EMFEntity.java)
    pub mod emf_entity {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // EMFEntity.emf$prevX(0): not translated, it has no body.
        // EMFEntity.emf$getX(0): not translated, it has no body.
        // EMFEntity.emf$prevY(0): not translated, it has no body.
        // EMFEntity.emf$getY(0): not translated, it has no body.
        // EMFEntity.emf$prevZ(0): not translated, it has no body.
        // EMFEntity.emf$getZ(0): not translated, it has no body.
        // EMFEntity.emf$prevPitch(0): not translated, it has no body.
        // EMFEntity.emf$getPitch(0): not translated, it has no body.
        // EMFEntity.emf$isTouchingWater(0): not translated, it has no body.
        // EMFEntity.emf$isOnFire(0): not translated, it has no body.
        // EMFEntity.emf$hasVehicle(0): not translated, it has no body.
        // EMFEntity.emf$isOnGround(0): not translated, it has no body.
        // EMFEntity.emf$isAlive(0): not translated, it has no body.
        // EMFEntity.emf$isGlowing(0): not translated, it has no body.
        // EMFEntity.emf$isInLava(0): not translated, it has no body.
        // EMFEntity.emf$isInvisible(0): not translated, it has no body.
        // EMFEntity.emf$hasPassengers(0): not translated, it has no body.
        // EMFEntity.emf$isSneaking(0): not translated, it has no body.
        // EMFEntity.emf$isSprinting(0): not translated, it has no body.
        // EMFEntity.emf$isWet(0): not translated, it has no body.
        // EMFEntity.emf$age(0): not translated, it has no body.
        // EMFEntity.emf$getYaw(0): not translated, it has no body.
        // EMFEntity.emf$getVelocity(0): not translated, it has no body.
        // EMFEntity.emf$getTypeString(0): not translated, it has no body.
        // EMFEntity.emf$getVariableMap(0): not translated, it has no body.
    }

    /// `EMFLODHandler` (traben/entity_model_features/utils/EMFLODHandler.java)
    pub mod emflod_handler {
        use super::*;
        pub static LOD_ENTITY_TIMERS: LazyLock<JMap<String, i32>> = LazyLock::new(|| JMap::new());
        pub static LOD_FRAME_SKIPPING: LazyLock<JCell<Option<bool>>> = LazyLock::new(|| JCell::new(None));
        pub static LAST_FOV: JCell<f64> = JCell::new(70.0f64);
        #[derive(Clone, Debug)]
        pub struct EMFLODHandler {
        }
        impl EMFLODHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn set_null_lod_frame_skipping() {
            emflod_handler::LOD_FRAME_SKIPPING.set(None);
        }
        // EMFLODHandler.getLODFactorOfEntity(1): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
        // EMFLODHandler.isLODSkippingThisFrame(1): not translated, it uses `ETF` (traben.entity_texture_features).IRIS_DETECTED.
        // EMFLODHandler.distanceOfEntityFrom(2): not translated, it uses Minecraft's `BlockPos`.
        // EMFLODHandler.emfState(0): not translated, it uses EMFEntityRenderState, which isn't translated (it's an interface).
        // EMFLODHandler.emfEntity(0): not translated, it uses EMFEntity, which isn't translated (it's an interface).
    }

    /// `EMFResourceCaching` (traben/entity_model_features/utils/EMFResourceCaching.java)
    pub mod emf_resource_caching {
        use super::*;
        pub static RESOURCE_EXISTENCE_CACHE: LazyLock<JCell<JMap<String, bool>>> = LazyLock::new(|| JCell::new(JMap::new()));
        pub static IS_POPULATED: JCell<bool> = JCell::new(false);
        #[derive(Clone, Debug)]
        pub struct EMFResourceCaching {
        }
        impl EMFResourceCaching {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn clear_cache() {
            emf_resource_caching::RESOURCE_EXISTENCE_CACHE.with(|o| o.clear());
            emf_resource_caching::IS_POPULATED.set(false);
        }
        // EMFResourceCaching.populateCacheIfNeeded(1): not translated, it uses Minecraft's `ResourceManager`.
        // EMFResourceCaching.scanPrefix(2): not translated, it uses Minecraft's `ResourceManager`.
        // EMFResourceCaching.resourceExists(2): not translated, it uses Minecraft's `ResourceManager`.
    }

    /// `EMFUtils` (traben/entity_model_features/utils/EMFUtils.java)
    pub mod emf_utils {
        use super::*;
        pub const MOD_ID_SHORT: &str = "EMF";
        // LOGGER: the logger. Log calls become comments.
        #[derive(Clone, Debug)]
        pub struct EMFUtils {
        }
        impl EMFUtils {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        pub fn res_1(full_path: &str) -> JIdentifier {
            return JIdentifier::parse(&full_path);
        }
        pub fn res_2(namespace: &str, path: &str) -> JIdentifier {
            return JIdentifier::new(&namespace, &path);
        }
        // EMFUtils.getArrowOrNull(1): not translated, it uses EMFModelPartRoot, which isn't translated (it extends `EMFModelPartVanilla`).
        // EMFUtils.renderLineBox(7): not translated, it uses Minecraft's `Pose`.
        // EMFUtils.renderLineBox(15): not translated, it uses Minecraft's `Pose`.
        pub fn override_message(original_class: &str, overridden_class_from_mod: &str, was_reverted: bool) {
            // log:
            if was_reverted {
                // log:
            }
        }
        // EMFUtils.log(1): not translated, it calls EMFUtils.log, which isn't translated.
        // EMFUtils.log(2): not translated, it calls EMFUtils.log, which isn't translated.
        // EMFUtils.log(3): not translated, it uses Minecraft's `LocalPlayer`.
        // EMFUtils.chat(1): not translated, it uses Minecraft's `LocalPlayer`.
        // EMFUtils.logWarn(1): not translated, it calls EMFUtils.logWarn, which isn't translated.
        // EMFUtils.logWarn(2): not translated, it uses Minecraft's `LocalPlayer`.
        // EMFUtils.logError(1): not translated, it calls EMFUtils.logError, which isn't translated.
        // EMFUtils.logError(2): not translated, it uses Minecraft's `LocalPlayer`.
        // EMFUtils.readModelPart(1): not translated, it uses EMFPartData, which isn't translated (it holds an object of its own class (a linked structure)).
        pub fn get_id_unique(known: &JSet<String>, desired: &str) -> String {
            let mut desired: String = desired.to_string();
            while (known.contains(&*desired)) || (desired.trim().is_empty()) {
                desired = format!("{}#", desired);
            }
            return desired.to_string();
        }
    }

    /// `IEMFCuboidDataSupplier` (traben/entity_model_features/utils/IEMFCuboidDataSupplier.java)
    pub mod iemf_cuboid_data_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEMFCuboidDataSupplier.emf$getTextureUV(0): not translated, it has no body.
        // IEMFCuboidDataSupplier.emf$getTextureXY(0): not translated, it has no body.
        // IEMFCuboidDataSupplier.emf$getSizeAdd(0): not translated, it has no body.
        // IEMFCuboidDataSupplier.emf$getMirror(0): not translated, it has no body.
    }

    /// `IEMFCustomModelHolder` (traben/entity_model_features/utils/IEMFCustomModelHolder.java)
    pub mod iemf_custom_model_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEMFCustomModelHolder.emf$hasModel(0): not translated, its class isn't translated (it's an interface).
        // IEMFCustomModelHolder.emf$getModel(0): not translated, it has no body.
        // IEMFCustomModelHolder.emf$setModel(1): not translated, it has no body.
    }

    /// `IEMFTextureSizeSupplier` (traben/entity_model_features/utils/IEMFTextureSizeSupplier.java)
    pub mod iemf_texture_size_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEMFTextureSizeSupplier.emf$getTextureSize(0): not translated, it has no body.
        // IEMFTextureSizeSupplier.emf$setTextureSize(1): not translated, it has no body.
    }

    /// `IEMFUnmodifiedLayerRootGetter` (traben/entity_model_features/utils/IEMFUnmodifiedLayerRootGetter.java)
    pub mod iemf_unmodified_layer_root_getter {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEMFUnmodifiedLayerRootGetter.emf$getUnmodifiedRoots(0): not translated, it has no body.
        // IEMFUnmodifiedLayerRootGetter.emf$setUnmodifiedRoots(1): not translated, it has no body.
    }

    /// `IEMFWolfCollarHolder` (traben/entity_model_features/utils/IEMFWolfCollarHolder.java)
    pub mod iemf_wolf_collar_holder {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IEMFWolfCollarHolder.emf$hasCollarModel(0): not translated, its class isn't translated (it's an interface).
        // IEMFWolfCollarHolder.emf$getCollarModel(0): not translated, it has no body.
        // IEMFWolfCollarHolder.emf$setCollarModel(1): not translated, it has no body.
    }

    /// `UEntityTypes` (traben/entity_model_features/utils/UEntityTypes.java)
    pub mod u_entity_types {
        use super::*;
        // CHEST: not translated, it uses Minecraft's `BlockEntityType`.
        // TRAPPED_CHEST: not translated, it uses Minecraft's `BlockEntityType`.
        // ENDER_CHEST: not translated, it uses Minecraft's `BlockEntityType`.
        // SHULKER_BOX: not translated, it uses Minecraft's `BlockEntityType`.
        // BELL: not translated, it uses Minecraft's `BlockEntityType`.
        // SIGN: not translated, it uses Minecraft's `BlockEntityType`.
        // DECORATED_POT: not translated, it uses Minecraft's `BlockEntityType`.
        // ENCHANTING_TABLE: not translated, it uses Minecraft's `BlockEntityType`.
        // LECTERN: not translated, it uses Minecraft's `BlockEntityType`.
        // SPECTRAL_ARROW: not translated, it uses Minecraft's `EntityType`.
        // BREEZE_WIND_CHARGE: not translated, it uses Minecraft's `EntityType`.
        // VILLAGER: not translated, it uses Minecraft's `EntityType`.
        // HORSE: not translated, it uses Minecraft's `EntityType`.
        // BLAZE: not translated, it uses Minecraft's `EntityType`.
        // CAMEL_HUSK: not translated, it uses Minecraft's `EntityType`.
        #[derive(Clone, Debug)]
        pub struct UEntityTypes {
        }
        impl UEntityTypes {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
    }

    
    // ---- Java regular expressions (java.util.regex), a small backtracking engine ----
    // Bundles can't add the regex crate, so this covers what mods use: literals, `.`, classes with ranges and
    // \d \s \w, groups (capturing, `(?:`, `(?<name>`), lookahead, alternation, greedy/lazy quantifiers, ^ $ \b.
    // Positions are in chars, like the translated String methods.
    
    #[derive(Clone, Debug)]
    enum JClassItem {
        Range(char, char),
        Digit(bool),
        Space(bool),
        Word(bool),
    }
    
    #[derive(Clone, Debug)]
    enum JNode {
        Char(char),
        Any,
        Class(Vec<JClassItem>, bool),
        Start,
        End,
        WordB(bool),
        Group(Box<JNode>, Option<usize>),
        Look(Box<JNode>, bool),
        Alt(Vec<JNode>),
        Concat(Vec<JNode>),
        Repeat(Box<JNode>, u32, u32, bool),
        Backref(usize),
    }
    
    type JCaps = Vec<Option<(usize, usize)>>;
    
    /// A compiled `Pattern`.
    #[derive(Clone, Debug)]
    pub struct JRegex {
        node: JNode,
        groups: usize,
        names: Vec<(String, usize)>,
        ci: bool,
    }
    
    fn j_is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    
    fn j_is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }
    
    struct JRegexParser<'a> {
        p: &'a [char],
        i: usize,
        groups: usize,
        names: Vec<(String, usize)>,
    }
    
    impl JRegexParser<'_> {
        fn peek(&self) -> Option<char> {
            self.p.get(self.i).copied()
        }
        fn eat(&mut self, c: char) -> bool {
            if self.peek() == Some(c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn alt(&mut self) -> JNode {
            let mut arms = vec![self.concat()];
            while self.eat('|') {
                arms.push(self.concat());
            }
            if arms.len() == 1 { arms.pop().unwrap() } else { JNode::Alt(arms) }
        }
        fn concat(&mut self) -> JNode {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == '|' || c == ')' {
                    break;
                }
                let atom = self.atom();
                items.push(self.quantified(atom));
            }
            JNode::Concat(items)
        }
        fn number(&mut self) -> Option<u32> {
            let start = self.i;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
            if start == self.i { None } else { self.p[start..self.i].iter().collect::<String>().parse().ok() }
        }
        fn quantified(&mut self, atom: JNode) -> JNode {
            let (min, max) = match self.peek() {
                Some('*') => (0, u32::MAX),
                Some('+') => (1, u32::MAX),
                Some('?') => (0, 1),
                Some('{') => {
                    let save = self.i;
                    self.i += 1;
                    let Some(lo) = self.number() else {
                        self.i = save;
                        return atom;
                    };
                    let hi = if self.eat(',') { self.number().unwrap_or(u32::MAX) } else { lo };
                    if !self.eat('}') {
                        panic!("bad repetition in regex");
                    }
                    self.i -= 1;
                    (lo, hi)
                }
                _ => return atom,
            };
            self.i += 1;
            let greedy = !self.eat('?');
            if greedy {
                self.eat('+');
            }
            self.quantified(JNode::Repeat(Box::new(atom), min, max, greedy))
        }
        fn escape(&mut self, in_class: bool) -> JClassItem {
            let c = self.peek().expect("regex ends with a backslash");
            self.i += 1;
            let lit = |c: char| JClassItem::Range(c, c);
            match c {
                'd' => JClassItem::Digit(true),
                'D' => JClassItem::Digit(false),
                's' => JClassItem::Space(true),
                'S' => JClassItem::Space(false),
                'w' => JClassItem::Word(true),
                'W' => JClassItem::Word(false),
                't' => lit('\t'),
                'n' => lit('\n'),
                'r' => lit('\r'),
                'f' => lit('\u{0C}'),
                'e' => lit('\u{1B}'),
                '0' => lit('\0'),
                'u' => {
                    let hex: String = self.p[self.i..self.i + 4].iter().collect();
                    self.i += 4;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\u escape")).unwrap_or('\u{FFFD}'))
                }
                'x' => {
                    let hex: String = self.p[self.i..self.i + 2].iter().collect();
                    self.i += 2;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\x escape")).unwrap_or('\u{FFFD}'))
                }
                c if !in_class && c.is_ascii_alphabetic() => panic!("unsupported regex escape \\{c}"),
                c => lit(c),
            }
        }
        fn class(&mut self) -> JNode {
            let negated = self.eat('^');
            let mut items = Vec::new();
            let mut first = true;
            loop {
                let c = self.peek().expect("unclosed [ in regex");
                if c == ']' && !first {
                    self.i += 1;
                    break;
                }
                first = false;
                if c == '[' {
                    panic!("nested character classes aren't supported");
                }
                self.i += 1;
                let item = if c == '\\' { self.escape(true) } else { JClassItem::Range(c, c) };
                // a-z
                if let JClassItem::Range(lo, _) = item
                    && self.peek() == Some('-')
                    && self.p.get(self.i + 1).is_some_and(|&n| n != ']')
                {
                    self.i += 1;
                    let hc = self.peek().unwrap();
                    self.i += 1;
                    let hi = if hc == '\\' {
                        match self.escape(true) {
                            JClassItem::Range(h, _) => h,
                            _ => panic!("bad range in regex"),
                        }
                    } else {
                        hc
                    };
                    items.push(JClassItem::Range(lo, hi));
                    continue;
                }
                items.push(item);
            }
            JNode::Class(items, negated)
        }
        fn atom(&mut self) -> JNode {
            let c = self.peek().unwrap();
            self.i += 1;
            match c {
                '.' => JNode::Any,
                '^' => JNode::Start,
                '$' => JNode::End,
                '[' => self.class(),
                '(' => {
                    let mut index = None;
                    let mut look = None;
                    if self.eat('?') {
                        if self.eat(':') {
                        } else if self.eat('=') {
                            look = Some(false);
                        } else if self.eat('!') {
                            look = Some(true);
                        } else if self.eat('<') {
                            let start = self.i;
                            while self.peek().is_some_and(|c| c != '>') {
                                self.i += 1;
                            }
                            let name: String = self.p[start..self.i].iter().collect();
                            self.i += 1;
                            self.groups += 1;
                            self.names.push((name, self.groups));
                            index = Some(self.groups);
                        } else {
                            panic!("unsupported regex group (?{:?}", self.peek());
                        }
                    } else {
                        self.groups += 1;
                        index = Some(self.groups);
                    }
                    let inner = self.alt();
                    if !self.eat(')') {
                        panic!("unclosed ( in regex");
                    }
                    match look {
                        Some(neg) => JNode::Look(Box::new(inner), neg),
                        None => JNode::Group(Box::new(inner), index),
                    }
                }
                '\\' => match self.peek() {
                    Some('b') => {
                        self.i += 1;
                        JNode::WordB(true)
                    }
                    Some('B') => {
                        self.i += 1;
                        JNode::WordB(false)
                    }
                    Some('A') => {
                        self.i += 1;
                        JNode::Start
                    }
                    Some('z') | Some('Z') => {
                        self.i += 1;
                        JNode::End
                    }
                    Some(d) if d.is_ascii_digit() && d != '0' => {
                        self.i += 1;
                        JNode::Backref(d as usize - '0' as usize)
                    }
                    Some('Q') => {
                        self.i += 1;
                        let mut lits = Vec::new();
                        while self.i < self.p.len() && !(self.p[self.i] == '\\' && self.p.get(self.i + 1) == Some(&'E')) {
                            lits.push(JNode::Char(self.p[self.i]));
                            self.i += 1;
                        }
                        self.i = (self.i + 2).min(self.p.len());
                        JNode::Concat(lits)
                    }
                    _ => match self.escape(false) {
                        JClassItem::Range(c, _) => JNode::Char(c),
                        other => JNode::Class(vec![other], false),
                    },
                },
                c => JNode::Char(c),
            }
        }
    }
    
    impl JRegex {
        /// `Pattern.compile(p)`. Java throws PatternSyntaxException on a bad pattern; this panics.
        pub fn new(p: &str) -> Self {
            Self::with_flags(p, 0)
        }
        /// `Pattern.compile(p, flags)`: CASE_INSENSITIVE (2) is honoured.
        pub fn with_flags(p: &str, flags: i32) -> Self {
            let chars: Vec<char> = p.chars().collect();
            let (chars, ci) = match chars.strip_prefix(&['(', '?', 'i', ')']) {
                Some(rest) => (rest.to_vec(), true),
                None => (chars, flags & 2 != 0),
            };
            let mut parser = JRegexParser { p: &chars, i: 0, groups: 0, names: Vec::new() };
            let node = parser.alt();
            if parser.i != chars.len() {
                panic!("unmatched ) in regex {p:?}");
            }
            JRegex { node, groups: parser.groups, names: parser.names, ci }
        }
    
        fn same(&self, a: char, b: char) -> bool {
            a == b || self.ci && (j_lower_char(a) == j_lower_char(b) || j_upper_char(a) == j_upper_char(b))
        }
    
        fn in_class(&self, items: &[JClassItem], c: char) -> bool {
            items.iter().any(|it| match *it {
                JClassItem::Range(lo, hi) => {
                    (lo..=hi).contains(&c) || self.ci && ((lo..=hi).contains(&j_lower_char(c)) || (lo..=hi).contains(&j_upper_char(c)))
                }
                JClassItem::Digit(yes) => c.is_ascii_digit() == yes,
                JClassItem::Space(yes) => j_is_space(c) == yes,
                JClassItem::Word(yes) => j_is_word(c) == yes,
            })
        }
    
        fn m(&self, n: &JNode, s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match n {
                JNode::Char(c) => i < s.len() && self.same(s[i], *c) && k(i + 1, caps),
                JNode::Any => i < s.len() && !matches!(s[i], '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}') && k(i + 1, caps),
                JNode::Class(items, neg) => i < s.len() && self.in_class(items, s[i]) != *neg && k(i + 1, caps),
                JNode::Start => i == 0 && k(i, caps),
                JNode::End => (i == s.len() || i + 1 == s.len() && s[i] == '\n') && k(i, caps),
                JNode::WordB(yes) => {
                    let before = i > 0 && j_is_word(s[i - 1]);
                    let after = i < s.len() && j_is_word(s[i]);
                    (before != after) == *yes && k(i, caps)
                }
                JNode::Group(inner, index) => match *index {
                    None => self.m(inner, s, i, caps, k),
                    Some(g) => self.m(inner, s, i, caps, &mut |j, caps: &mut JCaps| {
                        let prev = caps[g];
                        caps[g] = Some((i, j));
                        if k(j, caps) {
                            return true;
                        }
                        caps[g] = prev;
                        false
                    }),
                },
                JNode::Look(inner, neg) => {
                    let mut probe = caps.clone();
                    let found = self.m(inner, s, i, &mut probe, &mut |_, _| true);
                    if found == *neg {
                        return false;
                    }
                    if !neg {
                        *caps = probe;
                    }
                    k(i, caps)
                }
                JNode::Alt(arms) => arms.iter().any(|a| self.m(a, s, i, caps, k)),
                JNode::Concat(items) => self.seq(items, s, i, caps, k),
                JNode::Repeat(inner, min, max, greedy) => self.rep(inner, *min, *max, *greedy, 0, s, i, caps, k),
                JNode::Backref(g) => match caps.get(*g).copied().flatten() {
                    Some((a, b)) => {
                        let len = b - a;
                        i + len <= s.len() && (0..len).all(|d| self.same(s[a + d], s[i + d])) && k(i + len, caps)
                    }
                    None => false,
                },
            }
        }
    
        fn seq(&self, items: &[JNode], s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match items.split_first() {
                None => k(i, caps),
                Some((first, rest)) => self.m(first, s, i, caps, &mut |j, caps: &mut JCaps| self.seq(rest, s, j, caps, k)),
            }
        }
    
        #[allow(clippy::too_many_arguments)]
        fn rep(&self, n: &JNode, min: u32, max: u32, greedy: bool, count: u32, s: &[char], i: usize, caps: &mut JCaps,
               k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            let more = |caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool| {
                count < max
                    && self.m(n, s, i, caps, &mut |j, caps: &mut JCaps| {
                        // An empty pass can't make progress: stop repeating once the minimum is met.
                        (j != i || count < min) && self.rep(n, min, max, greedy, count + 1, s, j, caps, k)
                    })
            };
            if count < min {
                return more(caps, k);
            }
            if greedy { more(caps, k) || k(i, caps) } else { k(i, caps) || more(caps, k) }
        }
    
        /// A match starting exactly at `i`; `whole`: it must also end at the end of the text.
        fn at(&self, s: &[char], i: usize, whole: bool) -> Option<JCaps> {
            let mut caps: JCaps = vec![None; self.groups + 1];
            let mut end = None;
            let ok = self.m(&self.node, s, i, &mut caps, &mut |j, _| {
                if whole && j != s.len() {
                    return false;
                }
                end = Some(j);
                true
            });
            if !ok {
                return None;
            }
            caps[0] = Some((i, end.unwrap()));
            Some(caps)
        }
    
        fn find_from(&self, s: &[char], from: usize) -> Option<JCaps> {
            (from..=s.len()).find_map(|i| self.at(s, i, false))
        }
    
        /// `String.matches` / `Pattern.matches`: the whole text.
        pub fn matches(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.at(&cs, 0, true).is_some()
        }
    
        /// `Pattern.matcher(s).find()` as a test.
        pub fn is_found(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.find_from(&cs, 0).is_some()
        }
    
        fn expand(&self, s: &[char], caps: &JCaps, rep: &str, out: &mut String) {
            let r: Vec<char> = rep.chars().collect();
            let mut i = 0;
            while i < r.len() {
                match r[i] {
                    '\\' if i + 1 < r.len() => {
                        out.push(r[i + 1]);
                        i += 2;
                    }
                    '$' if i + 1 < r.len() && r[i + 1].is_ascii_digit() => {
                        let g = r[i + 1] as usize - '0' as usize;
                        if let Some((a, b)) = caps.get(g).copied().flatten() {
                            out.extend(&s[a..b]);
                        } else if g > self.groups {
                            panic!("No group {g}");
                        }
                        i += 2;
                    }
                    '$' if r.get(i + 1) == Some(&'{') => {
                        let close = r[i..].iter().position(|&c| c == '}').expect("bad ${name} in replacement") + i;
                        let name: String = r[i + 2..close].iter().collect();
                        let g = self.names.iter().find(|(n, _)| *n == name).map(|(_, g)| *g).expect("no group with that name");
                        if let Some((a, b)) = caps[g] {
                            out.extend(&s[a..b]);
                        }
                        i = close + 1;
                    }
                    c => {
                        out.push(c);
                        i += 1;
                    }
                }
            }
        }
    
        fn replace(&self, s: &str, rep: &str, all: bool) -> String {
            let cs: Vec<char> = s.chars().collect();
            let mut out = String::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                out.extend(&cs[pos..a]);
                self.expand(&cs, &caps, rep, &mut out);
                pos = b;
                from = if b == a { b + 1 } else { b };
                if b == a && a < cs.len() {
                    out.push(cs[a]);
                    pos = a + 1;
                }
                if !all {
                    break;
                }
            }
            if pos <= cs.len() {
                out.extend(&cs[pos..]);
            }
            out
        }
    
        /// `String.replaceAll` / `Matcher.replaceAll`, with Java's `$1` and `\$` in the replacement.
        pub fn replace_all(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, true)
        }
    
        pub fn replace_first(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, false)
        }
    
        /// `String.split(regex, limit)` / `Pattern.split`, with Java's rules for empty parts.
        pub fn split(&self, s: &str, limit: i32) -> Vec<String> {
            let cs: Vec<char> = s.chars().collect();
            let mut parts: Vec<String> = Vec::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                if limit > 0 && parts.len() as i32 == limit - 1 {
                    break;
                }
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                if b == 0 {
                    // A zero-width match at the start makes no leading empty part.
                    from = 1;
                    continue;
                }
                if a == b && a >= cs.len() {
                    break;
                }
                parts.push(cs[pos..a].iter().collect());
                pos = b;
                from = if a == b { b + 1 } else { b };
            }
            parts.push(cs[pos.min(cs.len())..].iter().collect());
            if limit == 0 {
                while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
                    parts.pop();
                }
                if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
                    parts.clear();
                }
            }
            parts
        }
    }
    
    /// A `Matcher`: a regex over one text, with the last match.
    #[derive(Clone, Debug)]
    pub struct JMatcher {
        re: JRegex,
        text: Vec<char>,
        from: usize,
        caps: Option<JCaps>,
    }
    
    impl JMatcher {
        pub fn new(re: &JRegex, s: &str) -> Self {
            JMatcher { re: re.clone(), text: s.chars().collect(), from: 0, caps: None }
        }
        pub fn find(&mut self) -> bool {
            if self.from > self.text.len() {
                self.caps = None;
                return false;
            }
            self.caps = self.re.find_from(&self.text, self.from);
            if let Some((a, b)) = self.caps.as_ref().and_then(|c| c[0]) {
                self.from = if a == b { b + 1 } else { b };
                true
            } else {
                self.from = self.text.len() + 1;
                false
            }
        }
        pub fn find_at(&mut self, start: i32) -> bool {
            self.from = start as usize;
            self.find()
        }
        pub fn matches(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, true);
            self.caps.is_some()
        }
        pub fn looking_at(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, false);
            self.caps.is_some()
        }
        fn span(&self, g: usize) -> Option<(usize, usize)> {
            let caps = self.caps.as_ref().expect("No match found");
            assert!(g < caps.len(), "No group {g}");
            caps[g]
        }
        /// `group(n)`: null (None) when that group took no part in the match.
        pub fn group(&self, g: i32) -> Option<String> {
            self.span(g as usize).map(|(a, b)| self.text[a..b].iter().collect())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let g = self.re.names.iter().find(|(n, _)| n == name).map(|(_, g)| *g).expect("No group with that name");
            self.group(g as i32)
        }
        pub fn start(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(a, _)| a as i32)
        }
        pub fn end(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(_, b)| b as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.re.groups as i32
        }
        pub fn replace_all(&self, rep: &str) -> String {
            self.re.replace_all(&self.text.iter().collect::<String>(), rep)
        }
    }
    
    #[cfg(test)]
    mod jregex_tests {
        use super::*;
    
        #[test]
        fn groups_and_lazy() {
            let re = JRegex::new("^(?:\\S )?(?:VERY SPECIAL|COMMON|RARE|EPIC)(?: DUNGEON)?(?: (.+?))?(?: \\S)?$");
            let mut m = JMatcher::new(&re, "EPIC DUNGEON SWORD");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("SWORD"));
            let mut m = JMatcher::new(&re, "a RARE FISHING ROD b");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("FISHING ROD"));
            let mut m = JMatcher::new(&re, "COMMON");
            assert!(m.matches());
            assert_eq!(m.group(1), None);
        }
    
        #[test]
        fn classes_counts_find() {
            let re = JRegex::new("server/[0-9A-Fa-f]{8}/(.+)");
            let mut m = JMatcher::new(&re, "server/0aF3b2c1/pack.zip");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("pack.zip"));
            assert!(!JRegex::new("server/[0-9A-Fa-f]{8}/(.+)").matches("server/xyz/a"));
            let re = JRegex::new("\"id\"\\s*:\\s*\"([a-z0-9_.-]+:[a-z0-9_./-]+)\"");
            let mut m = JMatcher::new(&re, "{\"id\": \"minecraft:item/a\", \"id\":\"x:y\"}");
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("minecraft:item/a"));
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("x:y"));
            assert!(!m.find());
        }
    
        #[test]
        fn split_and_replace() {
            assert_eq!(JRegex::new("\\s*,\\s*").split("a , b,c,,", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new(",").split("a,b,c", 2), vec!["a", "b,c"]);
            assert_eq!(JRegex::new("").split("abc", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new("(\\w+)@(\\w+)").replace_all("x a@b y c@d", "$2.$1"), "x b.a y d.c");
            assert_eq!(JRegex::new("a*").replace_all("baaac", "-"), "-b--c-");
            assert!(JRegex::with_flags("abc", 2).matches("AbC"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "entity_model_features");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }
}
