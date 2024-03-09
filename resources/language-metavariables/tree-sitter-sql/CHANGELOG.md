# Changelog

All notable changes to this project will be documented in this file. See [commit-and-tag-version](https://github.com/absolute-version/commit-and-tag-version) for commit guidelines.

## [0.2.0](https://github.com/derekstride/tree-sitter-sql/compare/v0.1.1...v0.2.0) (2023-12-15)


### ⚠ BREAKING CHANGES

* The `(ordered_columns)` node in the `create_index`
statement has been replaced with `(index_fields)`, with `(field)` child
nodes which can have `column: (_)`, `expression: (_)` or `function:
(_)` named child nodes.

### Features

* Add complex index fields to CREATE INDEX statements ([97806c5](https://github.com/derekstride/tree-sitter-sql/commit/97806c5474f330260a6ae41c7614264a56bfddd1))
* postgresql expression subscripts ([c54eff2](https://github.com/derekstride/tree-sitter-sql/commit/c54eff259d5ec577593b46f67646188836c48d1e))


### Bug Fixes

* **ci:** pin node-gyp to v10 ([0aaf90a](https://github.com/derekstride/tree-sitter-sql/commit/0aaf90a094a8c98efab920fb170dc8300c194e39))
* ensure scanner.c is included in npm package ([ec6bb68](https://github.com/derekstride/tree-sitter-sql/commit/ec6bb68ecb1478211bf6864e161464572c9cf59e))
* The scannar can now scan tag name with any (expected '$') ASCII char instead of only letters. ([dbe4c04](https://github.com/derekstride/tree-sitter-sql/commit/dbe4c04cd5fd78216ffc992658663ff2bf7d2471))

## [0.1.1](https://github.com/derekstride/tree-sitter-sql/compare/v0.1.0...v0.1.1) (2023-11-22)


### Features

* Add REFERENCES column constraint to CREATE TABLE statements ([379f473](https://github.com/derekstride/tree-sitter-sql/commit/379f473698aaef8485b4fa56f316752353d565a8))


### Bug Fixes

* remove typo in npm publish workflow ([a1cf63b](https://github.com/derekstride/tree-sitter-sql/commit/a1cf63b4c919770eebe13e357f231f4279022631))

## 0.1.0 (2023-11-14)


### Features

* add `binary` and `string` datatypes for hive ([2132493](https://github.com/derekstride/tree-sitter-sql/commit/21324932c60c9abde47ee36a799b0d2b4da91fdb))
* add `default_timestamp` option ([6ba8901](https://github.com/derekstride/tree-sitter-sql/commit/6ba8901c3874cca64f66402e281503b8b4b457e2))
* add `filter` to `aggregate_functions` ([7a5ad78](https://github.com/derekstride/tree-sitter-sql/commit/7a5ad785b42dd459eb0deb599396118f0e1f14bb))
* add `insert overwrite` from spark sql ([65b6f18](https://github.com/derekstride/tree-sitter-sql/commit/65b6f1821ba81c5c9f84be236b659291580e92c4))
* Add AST-fields for row terminated/escaped-by and lines terminated-by ([d2f0f66](https://github.com/derekstride/tree-sitter-sql/commit/d2f0f6695fffa4ec1c81fc2060eddf83161f9ee3))
* add binary between expression ([a8622bb](https://github.com/derekstride/tree-sitter-sql/commit/a8622bbf8d5319eadaa7f760c5c3ad34c6efe392))
* add bit datatype ([fb1895e](https://github.com/derekstride/tree-sitter-sql/commit/fb1895ecfe76ff01a805af2c1119c9ae828e41b0))
* add CAD database, Alter index, CAD USER/ROLE/GROUP ([ce4cadc](https://github.com/derekstride/tree-sitter-sql/commit/ce4cadc9692e8bd5a178fb0b83f61742bda0572e))
* add CAD sequence, variant for alter index/database ([56e6d6b](https://github.com/derekstride/tree-sitter-sql/commit/56e6d6bd54c5baafcb3ce7eaeaa83234c57bac19))
* Add checking for MISSING and UNEXPECTED in addition to ERROR in the test ([9308ce9](https://github.com/derekstride/tree-sitter-sql/commit/9308ce9e23a07fe18c03ee84994778a8c31b1895))
* Add create extension statement. ([87c1401](https://github.com/derekstride/tree-sitter-sql/commit/87c1401d39f3218a8dd9b716ebd82856279a1912))
* Add create trigger statement for postgres. ([e0922d7](https://github.com/derekstride/tree-sitter-sql/commit/e0922d7a68ed3c6736ccd11edbdeb920cb9f6c1a))
* add custom type handling ([742b68f](https://github.com/derekstride/tree-sitter-sql/commit/742b68f6c99c38e5cd0748fcc0b543f0d9785a42))
* Add dollar quoted string. ([c16e729](https://github.com/derekstride/tree-sitter-sql/commit/c16e72925bf168ae9201d492109c060fdbc02bcb))
* add drop index statement ([781d58d](https://github.com/derekstride/tree-sitter-sql/commit/781d58dc18379712f28aa5fe698c248239ba0be3))
* add explain, truncate and alter/drop/create schema ([d4acced](https://github.com/derekstride/tree-sitter-sql/commit/d4accedfc7585bd85132969df8eaea057059286c))
* add float parsing ([cd17cd0](https://github.com/derekstride/tree-sitter-sql/commit/cd17cd0ada2c53a3d6ad48d5dc1674f90c2bdbe0))
* Add MariaDB/MySQL `ADD|CHANGE|MODIFY ... FIRST|AFTER ...` ([393e0d3](https://github.com/derekstride/tree-sitter-sql/commit/393e0d35aad922a3bf4faa4ceccad2e401f7b2ab)), closes [#83](https://github.com/derekstride/tree-sitter-sql/issues/83)
* Add MariaDB/MySQL `ALTER TABLE ... CHANGE ...` ([1124c51](https://github.com/derekstride/tree-sitter-sql/commit/1124c5151320e6c1107ef7afc7f61b91690f0ebe)), closes [#82](https://github.com/derekstride/tree-sitter-sql/issues/82)
* Add MariaDB/MySQL `ALTER TABLE ... MODIFY ...` ([409be47](https://github.com/derekstride/tree-sitter-sql/commit/409be47ae61daeac205be5448b1b69fb702e4915)), closes [#89](https://github.com/derekstride/tree-sitter-sql/issues/89)
* Add missing (postgres) operators. ([16f299b](https://github.com/derekstride/tree-sitter-sql/commit/16f299b9678110f2b3a893c572bce5db236ec6d5))
* add more options to pg views ([3cb1ee6](https://github.com/derekstride/tree-sitter-sql/commit/3cb1ee6265872cc83c49999333be81cfbf34f3e1))
* Add more string litral formats (from postgres) and numbers foramts ([cab76b7](https://github.com/derekstride/tree-sitter-sql/commit/cab76b7d5797480e734c62a2a0217221dc3ed83b))
* add mysql rename statement ([0707eaa](https://github.com/derekstride/tree-sitter-sql/commit/0707eaa00bcc0f751c82b044752dfae9ca0911f1))
* add natural join ([d12b869](https://github.com/derekstride/tree-sitter-sql/commit/d12b869001ae9ad748616fe472367d2514702cf1))
* add natural join ([64c263d](https://github.com/derekstride/tree-sitter-sql/commit/64c263d1cf23489704ffd5e9c798b2fa675b424b))
* Add ONLY for ALTER TABLE statement. ([1a2b0da](https://github.com/derekstride/tree-sitter-sql/commit/1a2b0da94ee34e1057bee1180f16154b3b6a937b))
* add optional interval direction ([66d5b2c](https://github.com/derekstride/tree-sitter-sql/commit/66d5b2c00612d3c2ef7335a5fb444964b3281113))
* add parenthesized select and cte select statements ([b645f8c](https://github.com/derekstride/tree-sitter-sql/commit/b645f8ca4fed12abaabe6b36df70632f05f45110))
* add parse floats ([b6ea645](https://github.com/derekstride/tree-sitter-sql/commit/b6ea6458bb192862d241ef594208cb47b7577f7c))
* add recursive cte ([c0f7f31](https://github.com/derekstride/tree-sitter-sql/commit/c0f7f31d0412fb07d058a3d0386830c966f7d565))
* Add set operation (select ... union select ...) for insert statements. ([970b548](https://github.com/derekstride/tree-sitter-sql/commit/970b548bfdaa5833782f46d3235fb58d7d53170a))
* Add storage parameters to table definition. ([531d243](https://github.com/derekstride/tree-sitter-sql/commit/531d24333cf3e271a7d3a664804fddf977b0250b))
* add t-sql types ([c8825be](https://github.com/derekstride/tree-sitter-sql/commit/c8825be63436f05afdaa6618436db73c55608a98))
* add unsigned int|integer ([7a8c988](https://github.com/derekstride/tree-sitter-sql/commit/7a8c98877e8e11c9b79de4e8b6eb338ef0d96576))
* add unsigned integer casts ([d19d5fb](https://github.com/derekstride/tree-sitter-sql/commit/d19d5fb8581d5177ffb48014e661d58d67441309))
* add vacuum ([bf8edb4](https://github.com/derekstride/tree-sitter-sql/commit/bf8edb45a75d6660450658a7f7c13080ecbea45b))
* adds `merge into` upsert statements ([61760bb](https://github.com/derekstride/tree-sitter-sql/commit/61760bbd21d1ca3bbdb3aefdfc3d5ca524a7d834))
* allow parenthesis around CTE ([0f3f25e](https://github.com/derekstride/tree-sitter-sql/commit/0f3f25e19492e66bcb77dc02a9466c21000f535a))
* Change $._key_constraint to use $.ordered_columns instead of a simple list ([2f8b9dd](https://github.com/derekstride/tree-sitter-sql/commit/2f8b9dd57cf6d6d85eb547beb5888c0c37e19ff8))
* Change literal lexing to use regexp to reduce parser number of states. ([269580d](https://github.com/derekstride/tree-sitter-sql/commit/269580df94d62d5f8b9af93807ce3732e5f110c2))
* create function ([2964438](https://github.com/derekstride/tree-sitter-sql/commit/296443875ba40e61c5485d4c6759efb79ba094db))
* mariadb supports `limit` in `group_concat` ([bb372ec](https://github.com/derekstride/tree-sitter-sql/commit/bb372ec7f8a71b866b1b117c9e0645f904daf62e))
* nested common table expressions ([9d66f30](https://github.com/derekstride/tree-sitter-sql/commit/9d66f30807f804482485546e36c968751ee72aca))
* optimize athena tables ([f3c3515](https://github.com/derekstride/tree-sitter-sql/commit/f3c3515e044f8384badc43000a44326acbec53a7))
* optimize metadata on impala/hive ([b620971](https://github.com/derekstride/tree-sitter-sql/commit/b620971a9aed83af9a8323cb336f2946d895641c))
* Optional `COLUMN` in `ALTER TABLE` ([a2533b2](https://github.com/derekstride/tree-sitter-sql/commit/a2533b217a40c905abc40371dcc6a1b970f8378c))
* **pg:** add interval casting and selection ([c7ff747](https://github.com/derekstride/tree-sitter-sql/commit/c7ff7470f3ec2cb46bd6e3bcdff4a6b9a81eb2d8))
* **pg:** create columns as array and matrices ([3877ef6](https://github.com/derekstride/tree-sitter-sql/commit/3877ef644fbf177c9f90c290e3a1f7ffcd8a1d5a))
* **sets:** Dedicated node with optional parens for set operations ([899321d](https://github.com/derekstride/tree-sitter-sql/commit/899321db55eef43c8f08c501692effb9e34563ed))
* Support negative integers as literals ([756ffc2](https://github.com/derekstride/tree-sitter-sql/commit/756ffc20c19c2cfb01c52c26938fe10b43d9fd4d))
* support NOT IN ([1f61923](https://github.com/derekstride/tree-sitter-sql/commit/1f61923e42d2fc929f28eb8b9204597fbe9b01a5))
* support single quotes in string literals ([3e8a84f](https://github.com/derekstride/tree-sitter-sql/commit/3e8a84f6bbdbb3880dcda7625c0852f03a772e00))
* Test `ALTER TABLE` with elided `COLUMN` keyword ([bd25587](https://github.com/derekstride/tree-sitter-sql/commit/bd25587ac5f4ea43907d73962545e05a5fcfbdb8))
* **tests:** Add highlight tests ([418c981](https://github.com/derekstride/tree-sitter-sql/commit/418c98179bde94efc6609259c22c66498f172c7c))
* tsql multiple add columns in alter statement ([97fc151](https://github.com/derekstride/tree-sitter-sql/commit/97fc15170af2a829977e23118cdde2bf0b84d76a))


### Bug Fixes

* _set_value choice ([17b3f10](https://github.com/derekstride/tree-sitter-sql/commit/17b3f10857aa142d0366ea4d05021bf94a7dca51))
* `check option` as extra keyword nodes (for better highlighting) ([8528a09](https://github.com/derekstride/tree-sitter-sql/commit/8528a097f5a23232192bed3b14db4309c328f37b))
* `generated always` should accept expressions ([1695b30](https://github.com/derekstride/tree-sitter-sql/commit/1695b3051a4adc2a2604e14d0965db53db0ac09f))
* add comment ([2e48958](https://github.com/derekstride/tree-sitter-sql/commit/2e489587cf007f35f35d5e44bca40f14af6385f4))
* add cte to create tab/view/mat_view ([bfe1823](https://github.com/derekstride/tree-sitter-sql/commit/bfe182325cf7d644a5c56188052a4ff74952c0a1))
* Add missing option 'no maxvalue' and 'no minvalue' to CREATE SEQUENCE. ([bec206c](https://github.com/derekstride/tree-sitter-sql/commit/bec206cb5b5a1f6a429759816ee78da52897d904))
* alias _qualified_field into assignment ([e7f1c4a](https://github.com/derekstride/tree-sitter-sql/commit/e7f1c4aa7ed56e84da76c7097ca00d0e60625a35))
* Alter table with unique constraint with postgres syntaxe work and keep the ([23cca53](https://github.com/derekstride/tree-sitter-sql/commit/23cca5374a0f646fadd5ea064c0627ea4fe9d1d8))
* alter/create sequence with no cache ([bcd61d6](https://github.com/derekstride/tree-sitter-sql/commit/bcd61d6e8c14c7abcd0bfaf5432995e127a9f179))
* assignment change from identifier to field ([713ff4d](https://github.com/derekstride/tree-sitter-sql/commit/713ff4d8635c9b282073bfb23127c87c438523fa))
* case when with between statement ([bdf5dd0](https://github.com/derekstride/tree-sitter-sql/commit/bdf5dd05321f6a27c8201f1181c85199b12d93cf))
* Change string lexer to have two token dollar_quoted_string_start_tag and ([646c3ab](https://github.com/derekstride/tree-sitter-sql/commit/646c3abd1ac0ac3e751019a9fe5b0028ce273b4d))
* Change the dollar string lexer to produce 3 token dollar_quoted_string_start_tag, dollar_quoted_string_end_tag and ([39215fd](https://github.com/derekstride/tree-sitter-sql/commit/39215fd4f253ed6c9ebaee2e2ded537be7e8bc43))
* **column:** better anonymous node for columns ([727df9f](https://github.com/derekstride/tree-sitter-sql/commit/727df9fb73910ca693fd8617046dc4cdf9619409))
* Correct CREATE TYPE to accept also a schema as identifier. ([c223823](https://github.com/derekstride/tree-sitter-sql/commit/c223823ca9a3bf4bb886895e6ce464487e760a9b))
* correct test condition ([cde7200](https://github.com/derekstride/tree-sitter-sql/commit/cde7200e1fc0bbeedc2acd6e5f4a89973083d480))
* Correct tests. ([9e17b93](https://github.com/derekstride/tree-sitter-sql/commit/9e17b932d45460135459b1798c8b138e2c522035))
* correct window clause placement ([c67da1b](https://github.com/derekstride/tree-sitter-sql/commit/c67da1b46e89a8871ddd3d0a0cd3c5d1be217637))
* distinct and order by in invocation ([fae579b](https://github.com/derekstride/tree-sitter-sql/commit/fae579bc42eb5b7e93aa22e6f5ce00ae734c1ba2))
* impala partitioning variation ([eed38f2](https://github.com/derekstride/tree-sitter-sql/commit/eed38f2f0d438f2740255adb4d33bbec51f164d8))
* kludge around statement delimiters for now ([30668da](https://github.com/derekstride/tree-sitter-sql/commit/30668dadce311be867659833fe4d36ccef1a5273))
* Lift `ALTER TABLE` combination restriction (only Postgres) ([be20ed0](https://github.com/derekstride/tree-sitter-sql/commit/be20ed053a6d0a6125e485f4796a488f91df987d)), closes [/github.com/DerekStride/tree-sitter-sql/issues/82#issuecomment-1416227381](https://github.com/derekstride//github.com/DerekStride/tree-sitter-sql/issues/82/issues/issuecomment-1416227381)
* Make timestamp type a parametric type. ([2f5d0cd](https://github.com/derekstride/tree-sitter-sql/commit/2f5d0cd2c304724e2735d0a5c5cbf14055eef67a))
* Missing identifier as value of a SET statement. ([1e3f464](https://github.com/derekstride/tree-sitter-sql/commit/1e3f4643a37a796c3414f64c4c48c2c89032a84a))
* Modify custom type to accept a schema prefix. ([f75b8ea](https://github.com/derekstride/tree-sitter-sql/commit/f75b8eaf97b8a37c406f1c60376b38b95d8e4293))
* Move the expect tests to fail from functions.txt into errors.txt. ([9cc4850](https://github.com/derekstride/tree-sitter-sql/commit/9cc4850a3aa32982ce3ea264386c4a251d22697d))
* parses parens in create table statement ([b3af454](https://github.com/derekstride/tree-sitter-sql/commit/b3af4542f46250f4c14eaa6c8e4bbc0038c3ea8d))
* partially quoted expression ([dea9b69](https://github.com/derekstride/tree-sitter-sql/commit/dea9b69483e75f6a717474927f7ec1d13cc99caf))
* pg `array` function ([6df5ec8](https://github.com/derekstride/tree-sitter-sql/commit/6df5ec8cd67d30864b9649a97cfdef97b327d263))
* remove duplicated keyword ([8d80c7b](https://github.com/derekstride/tree-sitter-sql/commit/8d80c7bec363777f3e1e917b3d100ca264e13ff7))
* Remove identifier alias from cast builtin function ([e4e43ba](https://github.com/derekstride/tree-sitter-sql/commit/e4e43ba742a2ee88cbb24dbf305a7daadd583873))
* Remove identifier alias from count builtin function ([eac9da2](https://github.com/derekstride/tree-sitter-sql/commit/eac9da216bff9a377738637649ea2ef32fd021bb))
* replace `lua-match` with `match` in highlights.scm ([e69a1e6](https://github.com/derekstride/tree-sitter-sql/commit/e69a1e6c47baeb83b58a564160c310506f59cfc4))
* Replace identifier with literal for hive storage location and cache pool ([4216ecf](https://github.com/derekstride/tree-sitter-sql/commit/4216ecf07b4350b2068c5a418cf6c9b790f4ecf9))
* set create table precedence to avert conflicts between ([a1a7275](https://github.com/derekstride/tree-sitter-sql/commit/a1a72754ecfbe858ddfef4b3611da714efbcd33a))
* subquery unions & invalid tests ([0f02bbd](https://github.com/derekstride/tree-sitter-sql/commit/0f02bbdef30c0ec6e2227ee5a546f62d4abb1207))
* support qualified all_fields ([bb7d8c4](https://github.com/derekstride/tree-sitter-sql/commit/bb7d8c48206d4d16a2b57b14fc1a2c20182ca825))
* use keyword_language in grammar ([07e6245](https://github.com/derekstride/tree-sitter-sql/commit/07e6245be1c0fd77dfef4936401bf32baf144249))
