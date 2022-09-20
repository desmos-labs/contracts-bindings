# Changelog

## Version 1.1.0
### Bug Fixes
- ([\#37](https://github.com/desmos-labs/desmos-bindings.git/pull/37)) Fixed mocks cfg tags

### Features
- ([\#38](https://github.com/desmos-labs/desmos-bindings.git/pull/38)) Improve desmos mock apps lib
- ([\#58](https://github.com/desmos-labs/desmos-bindings.git/pull/58)) Added MockDesmosQuerier utility to mock desmos query responses in unit tests

### Dependencies
- ([\#41](https://github.com/desmos-labs/desmos-bindings/pull/41)) Bumped serde_json to 1.0.85
- ([\#42](https://github.com/desmos-labs/desmos-bindings/pull/42)) Bumped thiserror to 1.0.35
- ([\#45](https://github.com/desmos-labs/desmos-bindings/pull/45)) Bumped anyhow to 1.0.65
- ([\#50](https://github.com/desmos-labs/desmos-bindings/pull/50)) Bumped serde to 1.0.144
- ([\#51](https://github.com/desmos-labs/desmos-bindings/pull/51)) Bumped cw-controllers to 0.15.0
- ([\#52](https://github.com/desmos-labs/desmos-bindings/pull/52)) Bumped cw-multi-test to 0.15.0
- ([\#53](https://github.com/desmos-labs/desmos-bindings/pull/53)) Bumped cosmwasm-storage to 1.1.0
- ([\#54](https://github.com/desmos-labs/desmos-bindings/pull/54)) Bumped cw2 to 0.15.0
- ([\#55](https://github.com/desmos-labs/desmos-bindings/pull/55)) Bumped cw-storage-plus to 0.15.0
- ([\#60](https://github.com/desmos-labs/desmos-bindings/pull/60)) Bumped cosmwasm-std to 1.1.1 
- ([\#61](https://github.com/desmos-labs/desmos-bindings/pull/61)) Bumped cosmwasm-schema to 1.1.0

## Version 1.0.3
### Bug Fixes
[\#36](https://github.com/desmos-labs/desmos-bindings/pull/36)) Removed duplicated cases in the mock apps execution
[\#37](https://github.com/desmos-labs/desmos-bindings/pull/37)) Fixed mocks cfg tags

## Version 1.0.2
### Bug Fixes
* ([\#33](https://github.com/desmos-labs/desmos-bindings/pull/33)) Fixed mocks mod importing issue
* ([\#35](https://github.com/desmos-labs/desmos-bindings/pull/33)) Fixed mocks `OwnedDeps` to returns `DesmosQuery` in generic parameter

### Dependencies
* ([\#32](https://github.com/desmos-labs/desmos-bindings/pull/32)) Updated desmos bindings to match desemos v4.3.0

## Version 1.0.1

* ([\#29](https://github.com/desmos-labs/desmos-bindings/pull/29)) Added integration test library `mock_apps`
* ([\#30](https://github.com/desmos-labs/desmos-bindings/pull/30)) Replaced `mocks` feature into `test` config

## Version 1.0.0

* Implemented all modules for cosmwasm 