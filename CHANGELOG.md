# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.5.1](https://github.com/tirithen/clipper2/compare/v0.5.0...v0.5.1) (2025-01-29)


### Features

* impl Default for points and paths ([cc77f7b](https://github.com/tirithen/clipper2/commit/cc77f7b4f9560a048cdf27764b079622c130927a))

## [0.5.0](https://github.com/tirithen/clipper2/compare/v0.4.1...v0.5.0) (2025-01-21)


### ⚠ BREAKING CHANGES

* **Path:** Path<P>::inflate now returns Paths<P> instead of
Path<P>. The method will no longer panic when 0 points remain after
inflating with a negative number, instead a Paths<p> struct with length
0 will be returned in those cases.

### Features

* **Path:** Path<P>::inflate return Paths<P> ([cbb999b](https://github.com/tirithen/clipper2/commit/cbb999bcb964afed4d36f455711def0fe3346f55))

### [0.4.1](https://github.com/tirithen/clipper2/compare/v0.4.0...v0.4.1) (2024-07-30)


### Features

* add .push for Path and Paths ([3764676](https://github.com/tirithen/clipper2/commit/37646761f20c803e667f98f8ede7a68f49af6df3))


### Bug Fixes

* add bug fixes from Clipper2 C++ library ([58d2653](https://github.com/tirithen/clipper2/commit/58d2653ab6eee4a6841ce544b17cf2a73f1bad11))

## [0.4.0](https://github.com/tirithen/clipper2/compare/v0.3.0...v0.4.0) (2024-06-04)


### ⚠ BREAKING CHANGES

* Use .translate instead of .offset for Paths and Path
structs.
* Removes the custom iterators PathIterator and
PathsIterator, instead rely on the standard iterator types from vec and
slice.

### Features

* improve the iterator impls for Paths and Path ([0c93f5d](https://github.com/tirithen/clipper2/commit/0c93f5da4c7c5c2a19acb5b0de2fa11217727c82))
* remove depr. .offset method from Paths/Path ([4ee9fd4](https://github.com/tirithen/clipper2/commit/4ee9fd4d8196c2bdf841353b5675719bcf58a9d6))

## [0.3.0](https://github.com/tirithen/clipper2/compare/v0.2.3...v0.3.0) (2024-06-01)


### ⚠ BREAKING CHANGES

* scale now takes two arguments, allowing separate x and
y scaling factors

### Features

* implement IntoIterator for Path/Paths ([846602c](https://github.com/tirithen/clipper2/commit/846602c96a8a103d7097fc97d445f2e9f01c3c85))
* **Path:** add .rectangle method ([a1dbb5c](https://github.com/tirithen/clipper2/commit/a1dbb5cbaf16e5415585cba11f60bb4efe28b144))
* **serde:** ser./deser. Path and Paths ([b9800b7](https://github.com/tirithen/clipper2/commit/b9800b71deb805c2142153ea96b238f2f0a75c7c))
* support calculating signed path areas ([b1c6386](https://github.com/tirithen/clipper2/commit/b1c63862ef4effcbbec87861af8564f7b1d8bad1))
* support scaling around a point ([ba6dec3](https://github.com/tirithen/clipper2/commit/ba6dec3d14c92754c7294e9f0a4d9cab6d261925))

### [0.2.3](https://github.com/tirithen/clipper2/compare/v0.2.2...v0.2.3) (2024-05-13)


### Features

* expose clipper builder, add path methods ([0702f67](https://github.com/tirithen/clipper2/commit/0702f679425e20c8f833cb5ac52e9210432aebb5))

### [0.2.2](https://github.com/tirithen/clipper2/compare/v0.2.1...v0.2.2) (2024-05-07)


### Features

* **path:** add .flip_x and .flip_y to path structs ([6323292](https://github.com/tirithen/clipper2/commit/6323292bd0514cb1eeb544c799fb472cf9b2cf90))
* **path:** add .rotate(rad) method to Path/Paths ([150715a](https://github.com/tirithen/clipper2/commit/150715aeea21b2246efbbb99bff4b2f808fb120f))
* **path:** add .scale(scale) method to Path/Paths ([447ed8d](https://github.com/tirithen/clipper2/commit/447ed8dbfd6e5da23e1789c9a16c7522d6a8ba83))
* **path:** rename .offset(x,y) to .translate(x,y) ([06bcfb3](https://github.com/tirithen/clipper2/commit/06bcfb3d769e25e807d268e64110d09538c4662a))


### Bug Fixes

* **path:** keep path bounds centered during flip ([d87993e](https://github.com/tirithen/clipper2/commit/d87993e19d578872bc3f6df520f90fcaa736a47f))

### [0.2.1](https://github.com/tirithen/clipper2/compare/v0.2.0...v0.2.1) (2024-05-03)


### Features

* add .offset(x, y) method to Path and Paths ([be676f5](https://github.com/tirithen/clipper2/commit/be676f5beebbe0b18e1422a3852bea30a856eb96))
* add bounds struct to path/paths ([3d541f8](https://github.com/tirithen/clipper2/commit/3d541f8219d474d800e2578fde2675a950fcfdf9))
* add bounds struct to path/paths ([b17ccfd](https://github.com/tirithen/clipper2/commit/b17ccfd524c1bd5f16ae3d911cd1c71c04ce2802))
* **Paths:** add from Vec<Path<P>> for Paths ([39ea7a1](https://github.com/tirithen/clipper2/commit/39ea7a1658ac0982f7043da2d428b12ac16e6333))
* **simplify:** add simplify function ([418b98f](https://github.com/tirithen/clipper2/commit/418b98f54333db977460a2c931486f08f554fea2))

## [0.2.0](https://github.com/tirithen/clipper2/compare/v0.1.2...v0.2.0) (2024-04-29)


### ⚠ BREAKING CHANGES

* The API has been replaced, see
https://docs.rs/clipper2/latest/clipper2/ or `examples/` directory for
details.

### Features

* swap out ffi mappings, custom scaling, ref. ([5d1e7d2](https://github.com/tirithen/clipper2/commit/5d1e7d2189d236ecaf8f01d3fd3a815589f293fd))
* windows support and example ([468e9aa](https://github.com/tirithen/clipper2/commit/468e9aaae6e3aedcaa3d5a1d582c4a2be1062af7))

### [0.1.2](https://github.com/tirithen/clipper2/compare/v0.1.1...v0.1.2) (2024-03-17)


### Features

* add intersect, union, difference, and xor ops ([83e6408](https://github.com/tirithen/clipper2/commit/83e64084b069b452fe753f4262ce48677b121754))

### [0.1.1](https://github.com/tirithen/clipper2/compare/v0.1.0...v0.1.1) (2024-03-03)

## 0.1.0 (2024-03-03)


### Features

* **inflate:** expose inflate offsetting function ([1e842e2](https://github.com/tirithen/clipper2/commit/1e842e2756634752fdfcc38500509a901e01fd99))
