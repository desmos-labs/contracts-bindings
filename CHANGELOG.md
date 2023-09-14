# Changelog

## Version 3.0.0

### Breaking changes
With this version, the following changes have been made:
- `Post` structure now requires `owner` field

### Features
- ([\#303](https://github.com/desmos-labs/desmos-bindings.git/pull/303)) Supported `x/tokenfactory` feature

### Dependencies
- ([\#246](https://github.com/desmos-labs/desmos-bindings.git/pull/246)) Bumped strum_macros to 0.25.1 (#246) 
- ([\#295](https://github.com/desmos-labs/desmos-bindings.git/pull/295)) Bumped quote to 1.0.33 (#295)
- ([\#300](https://github.com/desmos-labs/desmos-bindings.git/pull/300)) Bumped proto to Desmos v6
- ([\#307](https://github.com/desmos-labs/desmos-bindings.git/pull/307)) Bumped serde to 1.0.188 (#307)
- ([\#309](https://github.com/desmos-labs/desmos-bindings.git/pull/309)) Bumped schemars to 0.8.13 (#309)
- ([\#310](https://github.com/desmos-labs/desmos-bindings.git/pull/310)) Bumped chrono to 0.4.27 (#310)
- ([\#315](https://github.com/desmos-labs/desmos-bindings.git/pull/315)) Bumped prost-types to 0.12.0
- ([\#316](https://github.com/desmos-labs/desmos-bindings.git/pull/316)) Bumped prost to 0.12.0
- ([\#317](https://github.com/desmos-labs/desmos-bindings.git/pull/317)) Bumped cosmwasm-std to v1.4.0
- ([\#319](https://github.com/desmos-labs/desmos-bindings.git/pull/319)) Bumped cosmwasm-schema to v1.4.0
- ([\#328](https://github.com/desmos-labs/desmos-bindings.git/pull/328)) Bumped syn to 2.0.32 (#328)
- ([\#329](https://github.com/desmos-labs/desmos-bindings.git/pull/329)) Bumped base64 to 0.21.4 (#329)
- ([\#331](https://github.com/desmos-labs/desmos-bindings.git/pull/331)) Bumped proc-macro2 1.0.66 to 1.0.67 (#331)

### Other
- ([\#291](https://github.com/desmos-labs/desmos-bindings.git/pull/291)) Used `parse_err` constructor instead of literal
- ([\#296](https://github.com/desmos-labs/desmos-bindings.git/pull/296)) Disabled backtraces feature

## Version 2.0.1
### Dependencies
- ([\#205](https://github.com/desmos-labs/desmos-bindings.git/pull/205)) Bump prettyplease from 0.2.5 to 0.2.6
- ([\#206](https://github.com/desmos-labs/desmos-bindings.git/pull/206)) Bump base64 from 0.21.0 to 0.21.2
- ([\#208](https://github.com/desmos-labs/desmos-bindings.git/pull/208)) Bump quote from 1.0.27 to 1.0.28
- ([\#211](https://github.com/desmos-labs/desmos-bindings.git/pull/211)) Bump log from 0.4.17 to 0.4.18
- ([\#215](https://github.com/desmos-labs/desmos-bindings.git/pull/215)) Bump chrono from 0.4.24 to 0.4.26
- ([\#218](https://github.com/desmos-labs/desmos-bindings.git/pull/218)) Bump regex from 1.8.1 to 1.8.4
- ([\#219](https://github.com/desmos-labs/desmos-bindings.git/pull/219)) Bump cosmwasm-schema from 1.2.5 to 1.2.6
- ([\#220](https://github.com/desmos-labs/desmos-bindings.git/pull/220)) Bump cosmwasm-std from 1.2.5 to 1.2.6
- ([\#223](https://github.com/desmos-labs/desmos-bindings.git/pull/223)) Bump cw-multi-test from 0.16.4 to 0.16.5
- ([\#224](https://github.com/desmos-labs/desmos-bindings.git/pull/224)) Bump proc-macro2 from 1.0.58 to 1.0.60

## Version 2.0.0
### Breaking changes
With this version, all the methods now are base on stargate feature. For this reason, the following changes have been made:
- `DepsMut<DesmosQuery>` and `Deps<DesmosQuery>` have to be replaced into `DepMut<Empty>` and `Deps<Empty>`
- `Response<DesmosMsg>` has to be replaced into `Response<Empty>`
- all the modules message builders have to be replace to the new version builders, i.e., `ProfilesMsg`, `PostsMsg` and etc.
- all the modules queriers have to be replace to the new version queriers, i.e., `ProfilesQuerier`, `PostsQuerier` and etc.
- `mock_apps` library has to be removed since cw_multi_test does not support stargate feature

### Features
- ([\#116](https://github.com/desmos-labs/desmos-bindings/pull/116)) Added stargate support
- ([\#116](https://github.com/desmos-labs/desmos-bindings/pull/116)) Updated subspace from v2 to v3
- ([\#213](https://github.com/desmos-labs/desmos-bindings/pull/213)) Added subspace v3 new methods: `MsgGrantTreasuryAuthorization`, `MsgGrantAllowance`, `MsgRevokeAllowance`, `QueryUserAllowances` and `QueryGroupAllowances`.

### Dependencies
- ([\#194](https://github.com/desmos-labs/desmos-bindings/pull/119)) Bumped cosmwasm-std to 1.2.5

## Version 1.2.1
### Features
- ([\#110](https://github.com/desmos-labs/desmos-bindings/pull/110)) Removed msg,query and mocks features

### Dependencies
- ([\#119](https://github.com/desmos-labs/desmos-bindings/pull/119)) Bumped cosmwasm-std to 1.1.9

## Version 1.2.0
### Bug Fixes
- ([\#95](https://github.com/desmos-labs/desmos-bindings/pull/95)) Fix field namings of models and messages naming
- ([\#101](https://github.com/desmos-labs/desmos-bindings/pull/101)) Added missing `initial_members` field of `CreateUserGroup` message

## Version 1.1.1
## Bug Fixes
- ([\#68](https://github.com/desmos-labs/desmos-bindings/pull/68)) Added missing features tags to `MockDesmosQuerier`

## Version 1.1.0
### Bug Fixes
- ([\#37](https://github.com/desmos-labs/desmos-bindings/pull/37)) Fixed mocks cfg tags

### Features
- ([\#38](https://github.com/desmos-labs/desmos-bindings/pull/38)) Improve desmos mock apps lib
- ([\#58](https://github.com/desmos-labs/desmos-bindings/pull/58)) Added MockDesmosQuerier utility to mock desmos query responses in unit tests

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