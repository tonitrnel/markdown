# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [14.1.1] - 2026-01-11

### Security

- Fixed regression from v13 in linkify inline rule. Specific patterns could
  cause high CPU use. Thanks to @ltduc147 for report.

## [14.1.0] - 2024-03-19

### Changed

- Updated CM spec compatibility to 0.31.2, #1009.

### Fixed

- Fixed quadratic complexity when parsing references, #996.
- Fixed quadratic output size with pathological user input in tables, #1000.

## [14.0.0] - 2023-12-08

### Changed

- Drop ancient browsers support (use `.fromCodePoint` and other features).
- Rewrite to ESM (including all plugins/deps). CJS fallback still available.
  No signatures changed, except `markdown-it-emoji` plugin.
- Dropped `dist/` folder from repo, build on package publish.
- Set `punicode.js` as external dependency.

### Fixed

- Html tokens inside img alt are now rendered as their original text, #896.
- Hardbreaks inside img alt are now rendered as newlines.

## [13.0.2] - 2023-09-26

### Security

- Fixed crash/infinite loop caused by linkify inline rule, #957.

### Fixed

- Throw an error if 3rd party plugin doesn't increment `line` or `pos` counters
  (previously, markdown-it would likely go into infinite loop instead), #847.

## [13.0.1] - 2022-05-03

### Fixed

- Bumped `linkify-it` to 4.0.1. That should fix some hangs, caused by wrong
  data, returned from `linkify-it`.

## [13.0.0] - 2022-04-22

### Added

- Added a new token type `text_special` to store escaped characters, same as `text` but
  unaffected by replacement plugins (smartquotes, typographer, linkifier, etc.).
- Added a new rule `text_join` in `core` ruler. Text replacement plugins may choose to
  insert themselves before it.

### Changed

- `(p)` is no longer replaced with § by typographer (conflicts with ℗), #763.
- `text_collapse` rule is renamed to `fragments_join`.

### Fixed

- Smartquotes, typographic replacements and plain text links can now be escaped
  with backslash (e.g. `\(c)` or `google\.com` are no longer replaced).
- Fixed collision of emphasis and linkifier (so `http://example.org/foo._bar_-_baz`
  is now a single link, not emphasized). Emails and fuzzy links are not affected by this.

## [12.3.2] - 2022-01-08

### Security

- Fix possible ReDOS in newline rule. Thanks to @MakeNowJust.

## [12.3.1] - 2022-01-07

### Fixed

- Fix corner case when tab prevents paragraph continuation in lists, #830.

## [12.3.0] - 2021-12-09

### Changed

- `StateInline.delimiters[].jump` is removed.

### Fixed

- Fixed quadratic complexity in pathological `***<10k stars>***a***<10k stars>***` case.

## [12.2.0] - 2021-08-02

### Added

- Ordered lists: add order value to token info.

### Fixed

- Always suffix indented code block with a newline, #799.

## [12.1.0] - 2021-07-01

### Changed

- Updated CM spec compatibility to 0.30.

## [12.0.6] - 2021-04-16

### Fixed

- Newline in `alt` should be rendered, #775.

## [12.0.5] - 2021-04-15

### Fixed

- HTML block tags with `===` inside are no longer incorrectly interpreted as headers, #772.
- Fix table/list parsing ambiguity, #767.

## [12.0.4] - 2020-12-20

### Fixed

- Fix crash introduced in `12.0.3` when processing strikethrough (`~~`) and similar plugins, #742.
- Avoid fenced token mutation, #745.

## [12.0.3] - 2020-12-07

### Fixed

- `[](<foo<bar>)` is no longer a valid link.
- `[](url (xxx())` is no longer a valid link.
- `[](url\ xxx)` is no longer a valid link.
- Fix performance issues when parsing links (#732, #734), backticks, (#733, #736),
  emphases (#735), and autolinks (#737).
- Allow newline in `<? ... ?>` in an inline context.
- Allow `<meta>` html tag to appear in an inline context.

## [12.0.2] - 2020-10-23

### Fixed

- Three pipes (`|\n|\n|`) are no longer rendered as a table with no columns, #724.

## [12.0.1] - 2020-10-19

### Fixed

- Fix tables inside lists indented with tabs, #721.

## [12.0.0] - 2020-10-14

### Added

- `.gitattributes`, force unix eol under windows, for development.

### Changed

- Added 3rd argument to `highlight(code, lang, attrs)`, #626.
- Rewrite tables according to latest GFM spec, #697.
- Use `rollup.js` to browserify sources.
- Drop `bower.json` (bower reached EOL).
- Deps bump.
- Tune `specsplit.js` options.
- Drop `Makefile` in favour of npm scrips.

### Fixed

- Fix mappings for table rows (amended fix made in 11.0.1), #705.
- `%25` is no longer decoded in beautified urls, #720.

## [11.0.1] - 2020-09-14

### Fixed

- Fix blockquote lazy newlines, #696.
- Fix missed mappings for table rows, #705.

## [11.0.0] - 2020-05-20

### Changed

- Bumped `linkify-it` to 3.0.0, #661 + allow unlimited `.` inside links.
- Dev deps bump.
- Switch to `nyc` for coverage reports.
- Partially moved tasks from Makefile to npm scripts.
- Automate web update on npm publish.

### Fixed

- Fix em- and en-dashes not being typographed when separated by 1 char, #624.
- Allow opening quote after another punctuation char in typographer, #641.
- Assorted wording & typo fixes.

## [10.0.0] - 2019-09-11

### Security

- Fix quadratic parse time for some combinations of pairs, #583. Algorithm is
  now similar to one in reference implementation.

### Changed

- Minor internal structs change, to make pairs parse more effective (cost is
  linear now). If you use external "pairs" extensions, you need sync those with
  "official ones". Without update, old code will work, but can cause invalid
  result in rare case. This is the only reason of major version bump. With high probability you don't need to change your code, only update version dependency.
- Updated changelog format.
- Deps bump.

## [9.1.0] - 2019-08-11

### Changed

- Remove extra characters from line break check. Leave only 0x0A & 0x0D, as in
  CommonMark spec, #581.

## [9.0.1] - 2019-07-12

### Fixed

- Fix possible corruption of open/close tag levels, #466

## [9.0.0] - 2019-07-09

### Changed

- Updated CM spec compatibility to 0.29.
- Update Travis-CI node version to actual (8 & latest).
- Deps bump.

## [8.4.2] - 2018-02-15

### Fixed

- Fix `--no-html` CLI option, #476.

## [8.4.1] - 2018-02-15

### Fixed

- Fix smartquotes around softbreaks, #430.

## [8.4.0] - 2017-08-24

### Changed

- Updated CM spec compatibility to 0.28.

## [8.3.2] - 2017-08-03

### Fixed

- Fix blockquote termination inside lists, #386.

## [8.3.1] - 2017-03-06

### Fixed

- Fix blockquote termination by list item, #338.

## [8.3.0] - 2017-02-16

### Changed

- Remove tabs at the beginning of the line in paragraphs.
- Better error message for bad input type, #324.

### Fixed

- Fix table indentation issues, #325, #224.
- Fix blockquote termination inside indented lists, #329.

## [8.2.2] - 2016-12-15

### Added

- Add `-o` / `--output` option to CLI, #312.

## [8.2.1] - 2016-12-02

### Fixed

- Add missed h2..h6 to whitelisted block tags.

## [8.2.0] - 2016-12-01

### Changed

- Updated CM spec compatibility to 0.27 (no significant changes).

### Fixed

- Fix backticks handle inside tables, #303.
- Fix edge case for fenced blocks with `~~~` in info, #301.
- Fix fallback to reference if link is not valid, #302.

## [8.1.0] - 2016-11-03

### Changed

- Make link parse helpers (`md.helpers`) pluggable, #299.

## [8.0.1] - 2016-10-18

### Fixed

- Tables: allow tab characters in markup

## [8.0.0] - 2016-09-16

### Changed

- Benchmarks src cleanup.
- Remove testing in old nodes (but still use es5).
- Updated CM spec compatibility to 0.26 (see list below):
- Two consecutive newlines no longer terminate a list.
- Ordered list terminating a paragraph can now only start with 1.
- Adjust emphasis algorithm (`*foo**bar**baz*` is now parsed as `<strong>`
  inside `<em>`).
- Fix tab width calculation inside lists and blockquotes.

## [7.0.1] - 2016-08-16

### Fixed

- Fence renderer: fix concat of class array, #276.
- Code renderer: do not render double space before attrs, #275.
- Replacer: disable replacements inside autolinks, #272.

## [7.0.0] - 2016-06-22

### Changed

- Bump `linkify-it` dependency to 2.0.0. `---` no longer terminates
  autodetected links by default. `md.linkifier.set('---', true)` will return old
  behaviour.
- Major version bumped, because internals or `linkify-it` was changed.
  You will not be affected anyhow, if not used direct access to
  `require('linkify-it/re')` for customizations.

## [6.1.1] - 2016-06-21

### Changed

- Render `code_inline` & `code_block` attributes if exist.

## [6.1.0] - 2016-06-19

### Changed

- Updated `fence` renderer to not mutate token. Token stream should be
  immutable after renderer call.

## [6.0.5] - 2016-06-01

### Fixed

- Process `\r` the same way as `\n` and `\r\n\`, #252.

## [6.0.4] - 2016-05-30

### Added

- Added `Token.attrGet()` method for convenience, #251.

## [6.0.3] - 2016-05-30

### Security

- Security fix: possible ReDOS in `linkify-it` (forced bump of `linkify-it` &
  `uc-micro` dependencies). New installs will use fixed packages automatically,
  but we bumped `markdown-it` version for sure & for web builds.

## [6.0.2] - 2016-05-16

### Fixed

- Fix: should not escape twice content of image alt attribute, #246.

## [6.0.1] - 2016-04-02

### Fixed

- Improve support of missing values in tables, #224.

## [6.0.0] - 2016-02-11

### Changed

- Maintenance release. Version bump caused by notable changes in CM spec
  (multiline setext headers, no spaces inside links, ...). API was not changed.
- Fit CM 0.24 spec requirements.

### Fixed

- Fixed nesting limit check in inline blocks, #197.
- Fixed posible tail loss in CLI ouput.

## [5.1.0] - 2016-01-07

### Added

- Token: added `.attrSet()` & `.attrJoin()` methods.
- Highlighter: allow wrapper override (if result starts with "<pre").

## [5.0.3] - 2016-01-04

### Fixed

- Allow single column and mismatched columns count in tables.
- Smartquotes: take into account adjacent tokens.
- Fill `content` property in image token with `alt` source.

## [5.0.2] - 2015-11-20

### Fixed

- Fix meta information (`token.markup` and `token.info`) for autolink tokens.

## [5.0.1] - 2015-10-30

### Fixed

- Improved tables compatibility with github, #120.

## [5.0.0] - 2015-10-05

### Changed

- Internal API change. Due to new CM spec requirements, we had to update
  internals. That should not touch ordinary users, but can affect some external
  plugins. If you are plugin developper - see migration guide:
  https://github.com/markdown-it/markdown-it/blob/master/docs/5.0_migration.md.
- Updated CM spec compatibility to 0.22 (see list below).
- Keep tabs (don't replace with spaces).
- Don't wrap iframes with paragraphs.
- Rewritten emphasis algorithm.

### Fixed

- Fix closure compiler collisions (don't use reserved words), #159.

## [4.4.0] - 2015-07-18

### Changed

- Updated HTML blocks logic to CM 0.21 spec.
- Minor fixes.

## [4.3.1] - 2015-07-15

### Security

- Fix class name injection in fence renderer.

### Fixed

- Allow numbered lists starting from zero.

## [4.3.0] - 2015-06-29

### Changed

- `linkify-it` dependency update (1.2.0). Now accepts dash at the end of links.

## [4.2.2] - 2015-06-10

### Changed

- CM spec 0.20.

### Added

- Added support for multichar substituition in smartquites, #115.

### Fixed

- Fixed code block render inside blockquites, #116.
- Doc fixes.

## [4.2.1] - 2015-05-01

### Changed

- Minor emphasis update to match CM spec 0.19.

## [4.2.0] - 2015-04-21

### Changed

- Bumped [linkify-it](https://github.com/markdown-it/linkify-it) version to
  1.1.0. Now links with IP hosts and without protocols are not linkified by
  default, due possible collisions with some version numbers patterns (0.5.0.0).
  You still can return back old behaviour by `md.linkify.set({ fuzzyIP: true })`.

## [4.1.2] - 2015-04-19

### Changed

- Bumped linkifier version. More strict 2-chars tald support for links without
  schema. Should not linkify things like `io.js` and `node.js`.

## [4.1.1] - 2015-04-15

### Fixed

- Improved pipe chars support in table cells, #86 (thanks to @jbt).

## [4.1.0] - 2015-03-31

### Security

- Disabled `data:` URLs by default (except some image mimes), to avoid
  possible XSS. Version bumped, because features changed (formally). If you did
  not used `data:` URLs, consider this version as 4.0.4 (no API changes).

### Changed

- Simplified link validator code. Now more easy to understand and to copy
  into your projects for customization.

## [4.0.3] - 2015-03-25

### Changed

- Updated linkifier.
- Smartquotes rule cleanup (#76).

### Fixed

- Fixed replacements rule bug in PhantomJS (#77).

## [4.0.2] - 2015-03-22

### Fixed

- Fixed emphasis `marker` fields in tokens (#69).
- Fixed html block tokens with numbers in name (#74).

## [4.0.1] - 2015-03-13

### Added

- Added custom container plugin demo.

### Changed

- Updated `linkify-it` version.

## [4.0.0] - 2015-03-11

### Changed

- Breaking internal API changes. See [v4 migration notes](https://github.com/markdown-it/markdown-it/blob/master/docs/4.0_migration.md). In usual case you will need to update plugins.
- Token internals changed
- Unified the most of renderer methods.
- Changed tokens creation - use `state.push(...)` (see sources)
- Moved `normalizeUrl()` to root class as `.normalizeLink()` &
  added `normalizeLinkText()` method.
- Moved `.validateUrl()` to root class and simplified logic - no more need to
  replace entities.
- Joined md unescape & replace entities logic to `utils.unescapeAll()`.
- Removed `replaceEntities()` in `utils`.
- `md.utils.lib` now exposes useful libs for plugins.
- Use entities data from external package.

### Fixed

- Fixed emphasis regression, caused by CM v0.18 spec (#65).

## [3.1.0] - 2015-03-05

### Changed

- Spec conformance update to 0.18.
- Significantly improved autolinking quality (use `linkify-it` package), #2.

### Fixed

- Rewritten links normalizer to solve different edge cases (use `mdurl`
  package), #29.
- Moved link title entities replace out of renderer.
- Fixed escaped entities in links (`foo\&amp;/bar`).
- Improved smartquotes logic, #61.

## [3.0.7] - 2015-02-22

### Added

- Added basic CLI support.

### Changed

- Use external package for unicode data (punctuation).

### Fixed

- Added \v \f to valid whitespaces.
- Match table columns count by header.

## [3.0.6] - 2015-02-12

### Added

- Sync scroll result => source in demo.

### Changed

- Moved `normalizeReference()` to utils.

### Fixed

- Fixed hang on long vertical list of links. Appeared in 3.0.5. See #54 for
  details. Thanks to @fengmk2 for report!
- Table lines now can have escaped pipe char `\|` (#5).

## [3.0.5] - 2015-02-06

### Changed

- Significantly improved tests coverage (with dead code removal and other
  related things).

### Fixed

- Fixed link validator - could skip some kind of javascript links with uppercase
  digital entities (thanks to @opennota)

## [3.0.4] - 2015-01-13

### Changed

- Improved errors processing in url normalizer (for broken sequences).
- Improved nesting limit processing in inline parser.
- Reorganized tests & improved coverage.
- Show inline diffs for failed tests.

## [3.0.3] - 2015-01-11

### Fixed

- Fixed punctuation check in emphasis.

## [3.0.2] - 2015-01-09

### Fixed

- Allow dashes in HTML tag names (needed for custom HTML tags).

## [3.0.1] - 2015-01-07

### Changed

- Added # to terminator chars.

### Fixed

- Improved link encoder - fix invalid surrogates to avoid errors.

## [3.0.0] - 2015-01-04

### Changed

- Big split. All "rare" rules moved to external plugins (deflist, abbr, footnote,
  sub, sup, ins, mark).
- Updated CM spec conformance to v0.15 (better unicode support).
- Added `md` (parser instance) link to all state objects (instead of former
  options/parser).
- References/Footnotes/Abbrs moved to `block` chain.
- Input normalization moved to `core` chain.
- Splitted links and images to separate rules.
- Renamed some rules.
- Removed `full` preset. Not needed anymore.
- enable/disable methods now throw by default on invalid rules (exceptions can
  be disabled).
- Replace NULL characters with 0xFFFD instead of strip.
- Removed custom fences sugar (overcomplication).
- Rewritten link components parse helpers.
- More functions in `md.utils`.

### Fixed

- Fixed inline html comments & cdata parse.

## [2.2.1] - 2014-12-29

### Added

- Added development info.

### Changed

- .use() now pass any number of params to plugins.

### Fixed

- Fixed line breaks in definitions lists.

## [2.2.0] - 2014-12-28

### Added

- API docs.
- Added 'zero' preset.

### Changed

- Updated CM spec conformance to v0.13.

### Fixed

- Fixed several crashes, when some basic rules are disabled
  (block termination check, references check).

## [2.1.3] - 2014-12-24

### Added

- Added curring to `set`/`configure`/`enable`/`disable` methods.

### Changed

- Demo rework - now can include plugins.
- Docs update.

## [2.1.2] - 2014-12-23

### Changed

- Exposed helpers into parser instances (for plugins).
- Removed utils from global export - been in instances seems enougth.
- Refactored demo & added markdown-it-emoji to it.

## [2.1.1] - 2014-12-22

### Changed

- Refreshed browser builds, missed in prev release.
- Minor changes.

## [2.1.0] - 2014-12-21

### Changed

- Separated method to enable rules by whitelist (enableOnly).
- Changed second param of enable/disable ruler methods.
- Shortcuts in main class for bulk enable/disable rules.
- ASCII-friendly browserified files.
- Separate package for spec tests.

## [2.0.0] - 2014-12-20

### Changed

- New project name & home! Now it's `markdown-it`,
- Sugar for constructor call - `new` is not mandatory now.
- Renamed presets folder (configs -> presets).

[14.1.1]: https://github.com/markdown-it/markdown-it/compare/14.1.0...14.1.1
[14.1.0]: https://github.com/markdown-it/markdown-it/compare/14.0.0...14.1.0
[14.0.0]: https://github.com/markdown-it/markdown-it/compare/13.0.2...14.0.0
[13.0.2]: https://github.com/markdown-it/markdown-it/compare/13.0.1...13.0.2
[13.0.1]: https://github.com/markdown-it/markdown-it/compare/13.0.0...13.0.1
[13.0.0]: https://github.com/markdown-it/markdown-it/compare/12.3.2...13.0.0
[12.3.2]: https://github.com/markdown-it/markdown-it/compare/12.3.1...12.3.2
[12.3.1]: https://github.com/markdown-it/markdown-it/compare/12.3.0...12.3.1
[12.3.0]: https://github.com/markdown-it/markdown-it/compare/12.2.0...12.3.0
[12.2.0]: https://github.com/markdown-it/markdown-it/compare/12.1.0...12.2.0
[12.1.0]: https://github.com/markdown-it/markdown-it/compare/12.0.6...12.1.0
[12.0.6]: https://github.com/markdown-it/markdown-it/compare/12.0.5...12.0.6
[12.0.5]: https://github.com/markdown-it/markdown-it/compare/12.0.4...12.0.5
[12.0.4]: https://github.com/markdown-it/markdown-it/compare/12.0.3...12.0.4
[12.0.3]: https://github.com/markdown-it/markdown-it/compare/12.0.2...12.0.3
[12.0.2]: https://github.com/markdown-it/markdown-it/compare/12.0.1...12.0.2
[12.0.1]: https://github.com/markdown-it/markdown-it/compare/12.0.0...12.0.1
[12.0.0]: https://github.com/markdown-it/markdown-it/compare/11.0.1...12.0.0
[11.0.1]: https://github.com/markdown-it/markdown-it/compare/11.0.0...11.0.1
[11.0.0]: https://github.com/markdown-it/markdown-it/compare/10.0.0...11.0.0
[10.0.0]: https://github.com/markdown-it/markdown-it/compare/9.1.0...10.0.0
[9.1.0]: https://github.com/markdown-it/markdown-it/compare/9.0.1...9.1.0
[9.0.1]: https://github.com/markdown-it/markdown-it/compare/9.0.0...9.0.1
[9.0.0]: https://github.com/markdown-it/markdown-it/compare/8.4.2...9.0.0
[8.4.2]: https://github.com/markdown-it/markdown-it/compare/8.4.1...8.4.2
[8.4.1]: https://github.com/markdown-it/markdown-it/compare/8.4.0...8.4.1
[8.4.0]: https://github.com/markdown-it/markdown-it/compare/8.3.2...8.4.0
[8.3.2]: https://github.com/markdown-it/markdown-it/compare/8.3.1...8.3.2
[8.3.1]: https://github.com/markdown-it/markdown-it/compare/8.3.0...8.3.1
[8.3.0]: https://github.com/markdown-it/markdown-it/compare/8.2.2...8.3.0
[8.2.2]: https://github.com/markdown-it/markdown-it/compare/8.2.1...8.2.2
[8.2.1]: https://github.com/markdown-it/markdown-it/compare/8.2.0...8.2.1
[8.2.0]: https://github.com/markdown-it/markdown-it/compare/8.1.0...8.2.0
[8.1.0]: https://github.com/markdown-it/markdown-it/compare/8.0.1...8.1.0
[8.0.1]: https://github.com/markdown-it/markdown-it/compare/8.0.0...8.0.1
[8.0.0]: https://github.com/markdown-it/markdown-it/compare/7.0.1...8.0.0
[7.0.1]: https://github.com/markdown-it/markdown-it/compare/7.0.0...7.0.1
[7.0.0]: https://github.com/markdown-it/markdown-it/compare/6.1.1...7.0.0
[6.1.1]: https://github.com/markdown-it/markdown-it/compare/6.1.0...6.1.1
[6.1.0]: https://github.com/markdown-it/markdown-it/compare/6.0.5...6.1.0
[6.0.5]: https://github.com/markdown-it/markdown-it/compare/6.0.4...6.0.5
[6.0.4]: https://github.com/markdown-it/markdown-it/compare/6.0.3...6.0.4
[6.0.3]: https://github.com/markdown-it/markdown-it/compare/6.0.2...6.0.3
[6.0.2]: https://github.com/markdown-it/markdown-it/compare/6.0.1...6.0.2
[6.0.1]: https://github.com/markdown-it/markdown-it/compare/6.0.0...6.0.1
[6.0.0]: https://github.com/markdown-it/markdown-it/compare/5.1.0...6.0.0
[5.1.0]: https://github.com/markdown-it/markdown-it/compare/5.0.3...5.1.0
[5.0.3]: https://github.com/markdown-it/markdown-it/compare/5.0.2...5.0.3
[5.0.2]: https://github.com/markdown-it/markdown-it/compare/5.0.1...5.0.2
[5.0.1]: https://github.com/markdown-it/markdown-it/compare/5.0.0...5.0.1
[5.0.0]: https://github.com/markdown-it/markdown-it/compare/4.4.0...5.0.0
[4.4.0]: https://github.com/markdown-it/markdown-it/compare/4.3.1...4.4.0
[4.3.1]: https://github.com/markdown-it/markdown-it/compare/4.3.0...4.3.1
[4.3.0]: https://github.com/markdown-it/markdown-it/compare/4.2.2...4.3.0
[4.2.2]: https://github.com/markdown-it/markdown-it/compare/4.2.1...4.2.2
[4.2.1]: https://github.com/markdown-it/markdown-it/compare/4.2.0...4.2.1
[4.2.0]: https://github.com/markdown-it/markdown-it/compare/4.1.2...4.2.0
[4.1.2]: https://github.com/markdown-it/markdown-it/compare/4.1.1...4.1.2
[4.1.1]: https://github.com/markdown-it/markdown-it/compare/4.1.0...4.1.1
[4.1.0]: https://github.com/markdown-it/markdown-it/compare/4.0.3...4.1.0
[4.0.3]: https://github.com/markdown-it/markdown-it/compare/4.0.2...4.0.3
[4.0.2]: https://github.com/markdown-it/markdown-it/compare/4.0.1...4.0.2
[4.0.1]: https://github.com/markdown-it/markdown-it/compare/4.0.0...4.0.1
[4.0.0]: https://github.com/markdown-it/markdown-it/compare/3.1.0...4.0.0
[3.1.0]: https://github.com/markdown-it/markdown-it/compare/3.0.7...3.1.0
[3.0.7]: https://github.com/markdown-it/markdown-it/compare/3.0.6...3.0.7
[3.0.6]: https://github.com/markdown-it/markdown-it/compare/3.0.5...3.0.6
[3.0.5]: https://github.com/markdown-it/markdown-it/compare/3.0.4...3.0.5
[3.0.4]: https://github.com/markdown-it/markdown-it/compare/3.0.3...3.0.4
[3.0.3]: https://github.com/markdown-it/markdown-it/compare/3.0.2...3.0.3
[3.0.2]: https://github.com/markdown-it/markdown-it/compare/3.0.1...3.0.2
[3.0.1]: https://github.com/markdown-it/markdown-it/compare/3.0.0...3.0.1
[3.0.0]: https://github.com/markdown-it/markdown-it/compare/2.2.1...3.0.0
[2.2.1]: https://github.com/markdown-it/markdown-it/compare/2.2.0...2.2.1
[2.2.0]: https://github.com/markdown-it/markdown-it/compare/2.1.3...2.2.0
[2.1.3]: https://github.com/markdown-it/markdown-it/compare/2.1.2...2.1.3
[2.1.2]: https://github.com/markdown-it/markdown-it/compare/2.1.1...2.1.2
[2.1.1]: https://github.com/markdown-it/markdown-it/compare/2.1.0...2.1.1
[2.1.0]: https://github.com/markdown-it/markdown-it/compare/2.0.0...2.1.0
[2.0.0]: https://github.com/markdown-it/markdown-it/releases/tag/2.0.0

# markdown-it design principles

## Data flow

Input data is parsed via nested chains of rules. There are 3 nested chains --
`core`, `block`, & `inline`:

```
core
    core.rule1 (normalize)
    ...
    core.ruleX

    block
        block.rule1 (blockquote)
        ...
        block.ruleX

    core.ruleX1 (intermediate rule that applies on block tokens, nothing yet)
    ...
    core.ruleXX

    inline (applied to each block token with "inline" type)
        inline.rule1 (text)
        ...
        inline.ruleX

    core.ruleYY (applies to all tokens)
    ... (abbreviation, footnote, typographer, linkifier)
```

The result of parsing is a token stream that will be passed to the renderer to generate HTML content.

These tokens can themselves be parsed again to generate more tokens (ex: a `list` token can be divided into multiple `inline` tokens).

An `env` object can be used alongside tokens to inject external variables into your parsers and renderers.

Each chain (`core`, `block`, & `inline`) uses an independent `state` object when parsing data so that each parsing operation is independent and can be disabled on the fly.

## Token stream

Instead of a traditional AST, we use more low-level data representation -- _tokens_.
The difference is simple:

- Tokens are a simple sequence (an array).
- Opening and closing tags are separate.
- There are special token objects, "inline containers", that have nested tokens.
  These are sequences with inline markup, such as bold, italic, text, etc.

See the [`Token`](https://github.com/markdown-it/markdown-it/blob/master/lib/token.mjs) class
for details about each token's content.

In total, a token stream is:

- On the top level -- an array of paired or single "block" tokens:
  - open/close for headers, lists, blockquotes, paragraphs, etc.
  - code blocks, fenced blocks, horizontal rules, HTML blocks, inline containers
- Each inline token has a `children` property with a nested token stream for inline content:
  - open/close for bold, italic, links, inline code, etc.
  - text, line breaks

Why not an AST? It's not needed for our tasks. We follow the KISS principle.
If you wish, you can call a parser without a renderer and convert the token stream
into an AST.

More details about tokens:

- [`Renderer` source](https://github.com/markdown-it/markdown-it/blob/master/lib/renderer.mjs)
- [`Token` source](https://github.com/markdown-it/markdown-it/blob/master/lib/token.mjs)
- [Live demo](https://markdown-it.github.io/) - type your text and click the `debug` tab.

## Rules

Rules are functions, doing "magic" with parser `state` objects. A rule is associated with one or more _chains_ and is unique. For instance, a `blockquote` token is associated with the `blockquote`, `paragraph`, `heading`, and `list` chains.

Rules are managed by name via [`Ruler`](https://markdown-it.github.io/markdown-it/#Ruler) instances and can be enabled and disabled from [`MarkdownIt`](https://markdown-it.github.io/markdown-it/#MarkdownIt)'s methods.

Note that some rules have a `validation mode` -- in this mode, rules do not
modify the token stream and only look ahead for the end of a token. It's one
important design principle -- a token stream is "write only" on the `block` & `inline` parse stages.

Parsers are designed to keep rules independent of each other. You can safely enable/disable them or
add new ones. There are no universal recipes for how to create new rules -- the design of
distributed state machines with good data isolation is a tricky business. However, you
can investigate existing rules & plugins to see possible approaches.

In complex cases you can try to ask for help in the [issue tracker](https://github.com/markdown-it/markdown-it/issues).
The condition is very simple -- it should be clear from your ticket that you studied the docs, sources,
and tried to do something yourself. We never reject with help to real developers.

## Renderer

After the token stream is generated, it's passed to a [`Renderer`](https://markdown-it.github.io/markdown-it/#Renderer).
It then iterates through all the tokens, passing each to a rule with the same name as its token type.

Renderer rules are located in `md.renderer.rules[name]` and are simple functions
with the same signature:

```js
function (tokens, idx, options, env, renderer) {
  // ...
  return htmlResult;
}
```

In many cases, that allows easy output changes even without parser intrusion.
For example, let's convert every image that uses a Vimeo link into a player iframe:

```js
var md = require("markdown-it")();

var defaultRender = md.renderer.rules.image,
  vimeoRE = /^https?:\/\/(www\.)?vimeo.com\/(\d+)($|\/)/;

md.renderer.rules.image = function (tokens, idx, options, env, self) {
  var src = tokens[idx].attrGet("src");

  if (vimeoRE.test(src)) {
    var id = src.match(vimeoRE)[2];

    return (
      '<div class="embed-responsive embed-responsive-16by9">\n' +
      '  <iframe class="embed-responsive-item" src="//player.vimeo.com/video/' +
      id +
      '"></iframe>\n' +
      "</div>\n"
    );
  }

  // Pass the token to the default renderer.
  return defaultRender(tokens, idx, options, env, self);
};
```

Here is another example on how to add `target="_blank"` to all links:

```js
// Remember the old renderer if overridden, or proxy to the default renderer.
var defaultRender =
  md.renderer.rules.link_open ||
  function (tokens, idx, options, env, self) {
    return self.renderToken(tokens, idx, options);
  };

md.renderer.rules.link_open = function (tokens, idx, options, env, self) {
  // Add a new `target` attribute, or replace the value of the existing one.
  tokens[idx].attrSet("target", "_blank");

  // Pass the token to the default renderer.
  return defaultRender(tokens, idx, options, env, self);
};
```

Note that if you need to add attributes, you can do so without a renderer override.
For example, you can update tokens in the `core` chain. This is slower than a direct
renderer override, but it can be more simple. Let's use the
[`markdown-it-for-inline`](https://github.com/markdown-it/markdown-it-for-inline) plugin
to do the same thing as in previous example:

```js
var iterator = require("markdown-it-for-inline");

var md = require("markdown-it")().use(
  iterator,
  "url_new_win",
  "link_open",
  function (tokens, idx) {
    tokens[idx].attrSet("target", "_blank");
  },
);
```

You also can write your own renderer to generate formats other than HTML, such as
JSON and XML. You can even use it to generate an AST.

## Summary

This was mentioned in [Data flow](#data-flow), but let's repeat the sequence again:

1. Blocks are parsed, and the top level of each token stream is filled with block tokens.
2. Content in inline containers is parsed, filling their `children` properties.
3. Rendering happens.

And somewhere in between, you can apply additional transformations.

Source code for each chain can be seen in the following files:

- [`parser_core.mjs`](https://github.com/markdown-it/markdown-it/blob/master/lib/parser_core.mjs)
- [`parser_block.mjs`](https://github.com/markdown-it/markdown-it/blob/master/lib/parser_block.mjs)
- [`parser_inline.mjs`](https://github.com/markdown-it/markdown-it/blob/master/lib/parser_inline.mjs)

Also, you can change output directly in a [`Renderer`](https://markdown-it.github.io/markdown-it/#Renderer) for many simple cases.

# pulldown-cmark

[![Tests](https://github.com/pulldown-cmark/pulldown-cmark/actions/workflows/rust.yml/badge.svg)](https://github.com/pulldown-cmark/pulldown-cmark/actions/workflows/rust.yml)
[![Docs](https://docs.rs/pulldown-cmark/badge.svg)](https://docs.rs/pulldown-cmark)
[![Crates.io](https://img.shields.io/crates/v/pulldown-cmark.svg?maxAge=2592000)](https://crates.io/crates/pulldown-cmark)

[Documentation](https://docs.rs/pulldown-cmark/)

This library is a pull parser for [CommonMark](http://commonmark.org/), written
in [Rust](http://www.rust-lang.org/). It comes with a simple command-line tool,
useful for rendering to HTML, and is also designed to be easy to use from as
a library.

It is designed to be:

- Fast; a bare minimum of allocation and copying
- Safe; written in pure Rust with no unsafe blocks (except in the opt-in SIMD feature)
- Versatile; in particular source-maps are supported
- Correct; the goal is 100% compliance with the [CommonMark spec](http://spec.commonmark.org/)

Further, it optionally supports parsing footnotes,
[Github flavored tables](https://github.github.com/gfm/#tables-extension-),
[Github flavored task lists](https://github.github.com/gfm/#task-list-items-extension-) and
[strikethrough](https://github.github.com/gfm/#strikethrough-extension-).

Rustc 1.71.1 or newer is required to build the crate.

## Example

Example usage:

```rust
// Create parser with example Markdown text.
let markdown_input = "hello world";
let parser = pulldown_cmark::Parser::new(markdown_input);

// Write to a new String buffer.
let mut html_output = String::new();
pulldown_cmark::html::push_html(&mut html_output, parser);
assert_eq!(&html_output, "<p>hello world</p>\n");
```

## Why a pull parser?

There are many parsers for Markdown and its variants, but to my knowledge none
use pull parsing. Pull parsing has become popular for XML, especially for
memory-conscious applications, because it uses dramatically less memory than
constructing a document tree, but is much easier to use than push parsers. Push
parsers are notoriously difficult to use, and also often error-prone because of
the need for user to delicately juggle state in a series of callbacks.

In a clean design, the parsing and rendering stages are neatly separated, but
this is often sacrificed in the name of performance and expedience. Many Markdown
implementations mix parsing and rendering together, and even designs that try
to separate them (such as the popular [hoedown](https://github.com/hoedown/hoedown)),
make the assumption that the rendering process can be fully represented as a
serialized string.

Pull parsing is in some sense the most versatile architecture. It's possible to
drive a push interface, also with minimal memory, and quite straightforward to
construct an AST. Another advantage is that source-map information (the mapping
between parsed blocks and offsets within the source text) is readily available;
you can call `into_offset_iter()` to create an iterator that yields `(Event, Range)`
pairs, where the second element is the event's corresponding range in the source
document.

While manipulating ASTs is the most flexible way to transform documents,
operating on iterators is surprisingly easy, and quite efficient. Here, for
example, is the code to transform soft line breaks into hard breaks:

```rust
let parser = parser.map(|event| match event {
    Event::SoftBreak => Event::HardBreak,
    _ => event
});
```

Or expanding an abbreviation in text:

```rust
let parser = parser.map(|event| match event {
    Event::Text(text) => Event::Text(text.replace("abbr", "abbreviation").into()),
    _ => event
});
```

Another simple example is code to determine the max nesting level:

```rust
let mut max_nesting = 0;
let mut level = 0;
for event in parser {
    match event {
        Event::Start(_) => {
            level += 1;
            max_nesting = std::cmp::max(max_nesting, level);
        }
        Event::End(_) => level -= 1,
        _ => ()
    }
}
```

Note that consecutive text events can happen due to the manner in which the
parser evaluates the source. A utility `TextMergeStream` exists to improve
the comfort of iterating the events:

```rust
use pulldown_cmark::{Event, Parser, Options};

let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";

let iterator = TextMergeStream::new(Parser::new(markdown_input));

for event in iterator {
    match event {
        Event::Text(text) => println!("{}", text),
        _ => {}
    }
}
```

There are some basic but fully functional examples of the usage of the crate in the
`examples` directory of this repository.

## Using Rust idiomatically

A lot of the internal scanning code is written at a pretty low level (it
pretty much scans byte patterns for the bits of syntax), but the external
interface is designed to be idiomatic Rust.

Pull parsers are at heart an iterator of events (start and end tags, text,
and other bits and pieces). The parser data structure implements the
Rust Iterator trait directly, and Event is an enum. Thus, you can use the
full power and expressivity of Rust's iterator infrastructure, including
for loops and `map` (as in the examples above), collecting the events into
a vector (for recording, playback, and manipulation), and more.

Further, the `Text` event (representing text) is a small copy-on-write string.
The vast majority of text fragments are just
slices of the source document. For these, copy-on-write gives a convenient
representation that requires no allocation or copying, but allocated
strings are available when they're needed. Thus, when rendering text to
HTML, most text is copied just once, from the source document to the
HTML buffer.

When using the pulldown-cmark's own HTML renderer, make sure to write to a buffered
target like a `Vec<u8>` or `String`. Since it performs many (very) small writes, writing
directly to stdout, files, or sockets is detrimental to performance. Such writers can
be wrapped in a [`BufWriter`](https://doc.rust-lang.org/std/io/struct.BufWriter.html).

## Build options

By default, the binary is built as well. If you don't want/need it, then build like this:

```bash
> cargo build --no-default-features
```

Or add this package as dependency of your project using `cargo add`:

```bash
> cargo add pulldown-cmark --no-default-features
```

SIMD accelerated scanners are available for the x64 platform from version 0.5 onwards. To
enable them, build with simd feature:

```bash
> cargo build --release --features simd
```

Or add this package as dependency of your project with the feature using `cargo add`:

```bash
> cargo add pulldown-cmark --no-default-features --features=simd
```

For a higher release performance you may want this configuration in your profile release:

```
lto = true
codegen-units = 1
panic = "abort"
```

### `no_std` support

`no_std` support can be enabled by compiling with `--no-default-features` to
disable `std` support and `--features hashbrown` for `Hash` collections that are only
defined in `std` for internal usages in crate. For example:

```toml
[dependencies]
pulldown-cmark = { version = "*", default-features = false, features = ["hashbrown", "other features"] }
```

To support both `std` and `no_std` builds in project, you can use the following
in your `Cargo.toml`:

```toml
[features]
default = ["std", "other features"]

std = ["pulldown-cmark/std"]
hashbrown = ["pulldown-cmark/hashbrown"]
other_features = []
[dependencies]
pulldown-cmark = { version = "*", default-features = false }
```

## Authors

The main author is Raph Levien. The implementation of the new design (v0.3+) was
completed by Marcus Klaas de Vries. Since 2023, the development has been driven
by Martín Pozo, Michael Howell, Roope Salmi and Martin Geisler.

## License

This software is under the MIT license. See details in [license file](./LICENSE).

## Contributions

We gladly accept contributions via GitHub pull requests. Please see
[CONTRIBUTING.md](CONTRIBUTING.md) for more details.

简单来说，**Fuzz 测试（模糊测试，Fuzzing）** 就像是给程序找一个“极其手欠且不按常理出牌”的测试员。

它不通过正常的测试用例（比如输入正确的用户名和密码），而是通过向程序输入**大量的、随机的、异常的、甚至是非法的**数据，观察程序是否会崩溃（Crash）、挂起（Hang）或者触发内存错误。

---

### 1. Fuzz 测试的核心流程

Fuzz 测试不是盲目地乱撞，现代的 Fuzzing 通常是一个闭环的自动化过程：

1. **生成数据 (Seed Generation)**：准备一些基础的输入文件（如一个合法的 PDF 或是字符串）。
2. **变异 (Mutation/Generation)**：通过算法对数据进行微调，比如把某个字节从 `0` 变成 `1`，或者重复某个字符一万次。
3. **喂给程序 (Execution)**：将变异后的数据作为输入启动目标程序。
4. **监控状态 (Monitoring)**：观察程序有没有报错、有没有内存溢出。
5. **反馈与演进 (Feedback Loop)**：如果某个输入让程序走到了以前没跑过的代码路径，测试器会记下这个输入，并在此基础上继续变异。

---

### 2. 为什么需要 Fuzz 测试？

传统的单元测试（Unit Testing）是“人类思维”的体现，我们只能测出我们**能想到**的 Bug。

而 Fuzz 测试的优势在于：

- **突破盲点**：它能构造出人类程序员根本想不到的怪异输入。
- **自动化程度极高**：只要算力够，它可以 7x24 小时不停地跑，自动发现代码深处的逻辑漏洞。
- **安全防线**：它是发现 **Buffer Overflow（缓冲区溢出）**、**SQL 注入**和 **远程代码执行 (RCE)** 等高危漏洞的神器。

---

### 3. Fuzz 测试的“流派”

| 类型                      | 特点                                                           | 比喻                                       |
| ------------------------- | -------------------------------------------------------------- | ------------------------------------------ |
| **黑盒 Fuzz (Black-box)** | 完全不知道程序内部结构，只管喂数据。                           | 蒙着眼睛往墙上扔泥巴。                     |
| **白盒 Fuzz (White-box)** | 深入分析程序逻辑（符号执行），针对性生成数据。                 | 拿着图纸寻找地基最薄弱的地方。             |
| **灰盒 Fuzz (Grey-box)**  | **目前的主流**。通过“代码覆盖率”来引导变异，哪没跑过就往哪钻。 | 拿着手电筒在迷宫里摸索，哪没去过就往哪走。 |

---

### 4. 常见的 Fuzz 工具

如果你想在自己的项目里试试，可以关注这些顶级工具：

- **AFL (American Fuzzy Lop)**：灰盒 Fuzz 的鼻祖，极其经典。
- **libFuzzer**：集成在 LLVM/Clang 编译器中的工具，适合 C/C++ 项目。
- **OSS-Fuzz**：Google 维护的开源项目，专门帮开源软件免费跑 Fuzz。

---

> **有趣的事实**：很多顶级的安全漏洞（比如心脏滴血 Heartbleed）如果当年早点用 Fuzz 测试，可能在发布前就被揪出来了。

**你想针对哪种语言（比如 C++、Go 或 Rust）了解具体的 Fuzz 工具用法吗？我可以为你展示一个简单的代码示例。**

获取 Markdown 解析器的基准测试（Benchmark）数据源，通常需要从**合规性（正确性）**、**真实场景（平均性能）**和**病态输入（极端性能）**三个维度来构建。

以下是获取这些数据源的具体途径和方法：

---

### 1. 标准合规性数据集 (Correctness)

这类数据主要用于验证解析器是否符合 Markdown 规范（如 CommonMark 或 GFM）。

- **CommonMark Spec**: 官方提供的标准测试用例。
- **获取方式**: 访问 [CommonMark GitHub 仓库](https://github.com/commonmark/commonmark-spec)，下载 `spec.json`。它包含了数百个测试用例，涵盖了各种复杂的嵌套和边界情况。

- **GFM (GitHub Flavored Markdown)**: 如果你的解析器需要支持 GitHub 扩展（如任务列表、表格）。
- **获取方式**: 查阅 [GitHub Spec](https://github.github.com/gfm/)，通常在 `cmark-gfm` 等实现的源码仓库中可以找到对应的测试套件。

### 2. 真实世界数据集 (Real-world Performance)

基准测试不能只跑几行代码，需要大量的真实文本来模拟用户的实际使用场景。

- **GitHub READMEs**: 获取 GitHub 上热门仓库的 `README.md`。
- **获取方式**: 使用 [GitHub API](https://docs.github.com/en/rest) 批量爬取，或者使用 Google 的 [BigQuery GitHub 公共数据集](https://cloud.google.com/blog/topics/public-datasets/github-on-bigquery-analyze-all-the-open-source-code) 导出大量的 `.md` 文件。

- **Stack Overflow 数据归档**: 许多回答是使用 Markdown 编写的。
- **获取方式**: 通过 [Stack Exchange Data Dump](https://www.google.com/search?q=https://archive.org/details/stackexchange) 下载原始数据，提取其中的 Markdown 文本。

- **维基百科 (Wikipedia)**: 虽然维基百科使用 MediaWiki 语法，但可以通过 Pandoc 转换为 Markdown。
- **获取方式**: 下载维基百科的 XML Dump，利用 [Pandoc](https://pandoc.org/) 批量转化为 Markdown，作为长文本压力测试的来源。

### 3. 病态/极端情况数据集 (Pathological/Stress Test)

专门用于检测解析器是否存在 **ReDoS（正则表达式拒绝服务攻击）** 或递归深度问题的输入。

- **CommonMark Pathological Tests**: 官方提供的专门针对已知性能陷阱的测试用例。
- **获取方式**: 在 [commonmark.js](https://www.google.com/search?q=https://github.com/commonmark/commonmark.js/blob/master/test/pathological.js) 的测试目录中，可以找到如深层嵌套括号、超长强调符等测试输入。

- **Security/XSS Fuzzing**: 测试解析器在面对非法 HTML 注入时的表现。
- **获取方式**: 参考 [OWASP XSS Filter Evasion Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/XSS_Filter_Evasion_Cheat_Sheet.html)，将其中的 Payload 嵌入 Markdown 进行测试。

### 4. 借鉴现有基准测试工具

不要重造轮子，可以直接从已有的成熟解析器的基准测试模块中“借用”数据。

- **cmark (C)**: [bench 目录](https://github.com/commonmark/cmark/tree/master/bench) 包含了一些代表性的测试文本。
- **pulldown-cmark (Rust)**: 这个库以高性能著称，其 [benches](https://www.google.com/search?q=https://github.com/raphlinus/pulldown-cmark/tree/master/benches) 目录下的输入文件非常有参考价值。
- **markdown-it (JS)**: 提供了一套完整的 [benchmark 流程](https://github.com/markdown-it/markdown-it/tree/master/benchmark) 和样本文本。

---

### 总结建议：如何构建你的 Benchmark？

| 数据类型     | 推荐来源                                   | 目的                                  |
| ------------ | ------------------------------------------ | ------------------------------------- |
| **标准用例** | `commonmark-spec/spec.json`                | 确保解析没出错                        |
| **长文本**   | 将 50 个热门 README 合并成一个 1MB+ 的文件 | 测试吞吐量 (Throughput)               |
| **小文件**   | 准备 1000 个不同风格的小片段               | 测试初始化和解析延迟 (Latency)        |
| **极端构造** | `[[[[...]]]]` (1000层嵌套)                 | 测试是否会崩溃 (Stack Overflow/ReDoS) |

### 💡 下一步操作

如果你需要直接上手的代码，**你想用哪种语言（如 Python, JS, Go）来运行这个基准测试？** 我可以为你提供一个简单的脚本，演示如何自动下载并加载这些 `spec.json` 数据。

# CommonMark

CommonMark is a rationalized version of Markdown syntax,
with a [spec][the spec] and BSD-licensed reference
implementations in C and JavaScript.

[Try it now!](https://spec.commonmark.org/dingus/)

[the spec]: https://spec.commonmark.org/

For more details, see <https://commonmark.org>.

This repository contains the spec itself, along with tools for
running tests against the spec, and for creating HTML and PDF versions
of the spec.

The reference implementations live in separate repositories:

- <https://github.com/commonmark/cmark> (C)
- <https://github.com/commonmark/commonmark.js> (JavaScript)

There is a list of third-party libraries
in a dozen different languages
[here](https://github.com/commonmark/CommonMark/wiki/List-of-CommonMark-Implementations).

## Running tests against the spec

[The spec] contains over 500 embedded examples which serve as conformance
tests. To run the tests using an executable `$PROG`:

    python3 test/spec_tests.py --program $PROG

If you want to extract the raw test data from the spec without
actually running the tests, you can do:

    python3 test/spec_tests.py --dump-tests

and you'll get all the tests in JSON format.

JavaScript developers may find it more convenient to use the
[`commonmark-spec` npm package], which is published from this
repository. It exports an array `tests` of JSON objects with
the format

```json
{
  "markdown": "Foo\nBar\n---\n",
  "html": "<h2>Foo\nBar</h2>\n",
  "section": "Setext headings",
  "number": 65
}
```

[`commonmark-spec` npm package]: https://www.npmjs.com/package/commonmark-spec

## The spec

The source of [the spec] is `spec.txt`. This is basically a Markdown
file, with code examples written in a shorthand form:

    ```````````````````````````````` example
    Markdown source
    .
    expected HTML output
    ````````````````````````````````

To build an HTML version of the spec, do `make spec.html`. To build a
PDF version, do `make spec.pdf`. For both versions, you must
have the lua rock `lcmark` installed: after installing lua and
lua rocks, `luarocks install lcmark`. For the PDF you must also
have xelatex installed.

The spec is written from the point of view of the human writer, not
the computer reader. It is not an algorithm---an English translation of
a computer program---but a declarative description of what counts as a block
quote, a code block, and each of the other structural elements that can
make up a Markdown document.

Because John Gruber's [canonical syntax
description](https://daringfireball.net/projects/markdown/syntax) leaves
many aspects of the syntax undetermined, writing a precise spec requires
making a large number of decisions, many of them somewhat arbitrary.
In making them, we have appealed to existing conventions and
considerations of simplicity, readability, expressive power, and
consistency. We have tried to ensure that "normal" documents in the many
incompatible existing implementations of Markdown will render, as far as
possible, as their authors intended. And we have tried to make the rules
for different elements work together harmoniously. In places where
different decisions could have been made (for example, the rules
governing list indentation), we have explained the rationale for
our choices. In a few cases, we have departed slightly from the canonical
syntax description, in ways that we think further the goals of Markdown
as stated in that description.

For the most part, we have limited ourselves to the basic elements
described in Gruber's canonical syntax description, eschewing extensions
like footnotes and definition lists. It is important to get the core
right before considering such things. However, we have included a visible
syntax for line breaks and fenced code blocks.

## Differences from original Markdown

There are only a few places where this spec says things that contradict
the canonical syntax description:

- It allows all punctuation symbols to be backslash-escaped,
  not just the symbols with special meanings in Markdown. We found
  that it was just too hard to remember which symbols could be
  escaped.

- It introduces an alternative syntax for hard line
  breaks, a backslash at the end of the line, supplementing the
  two-spaces-at-the-end-of-line rule. This is motivated by persistent
  complaints about the “invisible” nature of the two-space rule.

- Link syntax has been made a bit more predictable (in a
  backwards-compatible way). For example, `Markdown.pl` allows single
  quotes around a title in inline links, but not in reference links.
  This kind of difference is really hard for users to remember, so the
  spec allows single quotes in both contexts.

- The rule for HTML blocks differs, though in most real cases it
  shouldn't make a difference. (See the section on HTML Blocks
  for details.) The spec's proposal makes it easy to include Markdown
  inside HTML block-level tags, if you want to, but also allows you to
  exclude this. It also makes parsing much easier, avoiding
  expensive backtracking.

- It does not collapse adjacent bird-track blocks into a single
  blockquote:

      > these are two

      > blockquotes

      > this is a single
      >
      > blockquote with two paragraphs

- Rules for content in lists differ in a few respects, though (as with
  HTML blocks), most lists in existing documents should render as
  intended. There is some discussion of the choice points and
  differences in the subsection of List Items entitled Motivation.
  We think that the spec's proposal does better than any existing
  implementation in rendering lists the way a human writer or reader
  would intuitively understand them. (We could give numerous examples
  of perfectly natural looking lists that nearly every existing
  implementation flubs up.)

- Changing bullet characters, or changing from bullets to numbers or
  vice versa, starts a new list. We think that is almost always going
  to be the writer's intent.

- The number that begins an ordered list item may be followed by
  either `.` or `)`. Changing the delimiter style starts a new
  list.

- The start number of an ordered list is significant.

- Fenced code blocks are supported, delimited by either
  backticks (` ``` `) or tildes (`~~~`).

## Contributing

There is a [forum for discussing
CommonMark](https://talk.commonmark.org); you should use it instead of
github issues for questions and possibly open-ended discussions.
Use the [github issue tracker](https://github.com/commonmark/CommonMark/issues)
only for simple, clear, actionable issues.

## Authors

The spec was written by John MacFarlane, drawing on

- his experience writing and maintaining Markdown implementations in several
  languages, including the first Markdown parser not based on regular
  expression substitutions ([pandoc](https://github.com/jgm/pandoc)) and
  the first markdown parsers based on PEG grammars
  ([peg-markdown](https://github.com/jgm/peg-markdown),
  [lunamark](https://github.com/jgm/lunamark))
- a detailed examination of the differences between existing Markdown
  implementations using [BabelMark 2](https://johnmacfarlane.net/babelmark2/),
  and
- extensive discussions with David Greenspan, Jeff Atwood, Vicent
  Marti, Neil Williams, and Benjamin Dumke-von der Ehe.

Since the first announcement, many people have contributed ideas.
Kārlis Gaņģis was especially helpful in refining the rules for
emphasis, strong emphasis, links, and images.

# Tokio

A runtime for writing reliable, asynchronous, and slim applications with
the Rust programming language. It is:

- **Fast**: Tokio's zero-cost abstractions give you bare-metal
  performance.

- **Reliable**: Tokio leverages Rust's ownership, type system, and
  concurrency model to reduce bugs and ensure thread safety.

- **Scalable**: Tokio has a minimal footprint, and handles backpressure
  and cancellation naturally.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]
[![Discord chat][discord-badge]][discord-url]

[crates-badge]: https://img.shields.io/crates/v/tokio.svg
[crates-url]: https://crates.io/crates/tokio
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/tokio-rs/tokio/blob/master/LICENSE
[actions-badge]: https://github.com/tokio-rs/tokio/workflows/CI/badge.svg
[actions-url]: https://github.com/tokio-rs/tokio/actions?query=workflow%3ACI+branch%3Amaster
[discord-badge]: https://img.shields.io/discord/500028886025895936.svg?logo=discord&style=flat-square
[discord-url]: https://discord.gg/tokio

[Website](https://tokio.rs) |
[Guides](https://tokio.rs/tokio/tutorial) |
[API Docs](https://docs.rs/tokio/latest/tokio) |
[Chat](https://discord.gg/tokio)

## Overview

Tokio is an event-driven, non-blocking I/O platform for writing
asynchronous applications with the Rust programming language. At a high
level, it provides a few major components:

- A multithreaded, work-stealing based task [scheduler].
- A reactor backed by the operating system's event queue (epoll, kqueue,
  IOCP, etc.).
- Asynchronous [TCP and UDP][net] sockets.

These components provide the runtime components necessary for building
an asynchronous application.

[net]: https://docs.rs/tokio/latest/tokio/net/index.html
[scheduler]: https://docs.rs/tokio/latest/tokio/runtime/index.html

## Example

A basic TCP echo server with Tokio.

Make sure you enable the full features of the tokio crate on Cargo.toml:

```toml
[dependencies]
tokio = { version = "1.49.0", features = ["full"] }
```

Then, on your main.rs:

```rust,no_run
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(0) => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
```

More examples can be found [here][examples]. For a larger "real world" example, see the
[mini-redis] repository.

[examples]: https://github.com/tokio-rs/tokio/tree/master/examples
[mini-redis]: https://github.com/tokio-rs/mini-redis/

To see a list of the available feature flags that can be enabled, check our
[docs][feature-flag-docs].

## Getting Help

First, see if the answer to your question can be found in the [Guides] or the
[API documentation]. If the answer is not there, there is an active community in
the [Tokio Discord server][chat]. We would be happy to try to answer your
question. You can also ask your question on [the discussions page][discussions].

[Guides]: https://tokio.rs/tokio/tutorial
[API documentation]: https://docs.rs/tokio/latest/tokio
[chat]: https://discord.gg/tokio
[discussions]: https://github.com/tokio-rs/tokio/discussions
[feature-flag-docs]: https://docs.rs/tokio/#feature-flags

## Contributing

:balloon: Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][guide] to help you get involved in the Tokio
project.

[guide]: https://github.com/tokio-rs/tokio/blob/master/docs/contributing/README.md

## Related Projects

In addition to the crates in this repository, the Tokio project also maintains
several other libraries, including:

- [`axum`]: A web application framework that focuses on ergonomics and modularity.

- [`hyper`]: A fast and correct HTTP/1.1 and HTTP/2 implementation for Rust.

- [`tonic`]: A gRPC over HTTP/2 implementation focused on high performance, interoperability, and flexibility.

- [`warp`]: A super-easy, composable, web server framework for warp speeds.

- [`tower`]: A library of modular and reusable components for building robust networking clients and servers.

- [`tracing`] (formerly `tokio-trace`): A framework for application-level tracing and async-aware diagnostics.

- [`mio`]: A low-level, cross-platform abstraction over OS I/O APIs that powers `tokio`.

- [`bytes`]: Utilities for working with bytes, including efficient byte buffers.

- [`loom`]: A testing tool for concurrent Rust code.

[`axum`]: https://github.com/tokio-rs/axum
[`warp`]: https://github.com/seanmonstar/warp
[`hyper`]: https://github.com/hyperium/hyper
[`tonic`]: https://github.com/hyperium/tonic
[`tower`]: https://github.com/tower-rs/tower
[`loom`]: https://github.com/tokio-rs/loom
[`tracing`]: https://github.com/tokio-rs/tracing
[`mio`]: https://github.com/tokio-rs/mio
[`bytes`]: https://github.com/tokio-rs/bytes

## Changelog

The Tokio repository contains multiple crates. Each crate has its own changelog.

- `tokio` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio/CHANGELOG.md)
- `tokio-util` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-util/CHANGELOG.md)
- `tokio-stream` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-stream/CHANGELOG.md)
- `tokio-macros` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-macros/CHANGELOG.md)
- `tokio-test` - [view changelog](https://github.com/tokio-rs/tokio/blob/master/tokio-test/CHANGELOG.md)

## Supported Rust Versions

<!--
When updating this, also update:
- .github/workflows/ci.yml
- CONTRIBUTING.md
- README.md
- tokio/README.md
- tokio/Cargo.toml
- tokio-util/Cargo.toml
- tokio-test/Cargo.toml
- tokio-stream/Cargo.toml
-->

Tokio will keep a rolling MSRV (minimum supported rust version) policy of **at
least** 6 months. When increasing the MSRV, the new Rust version must have been
released at least six months ago. The current MSRV is 1.71.

Note that the MSRV is not increased automatically, and only as part of a minor
release. The MSRV history for past minor releases can be found below:

- 1.48 to now - Rust 1.71
- 1.39 to 1.47 - Rust 1.70
- 1.30 to 1.38 - Rust 1.63
- 1.27 to 1.29 - Rust 1.56
- 1.17 to 1.26 - Rust 1.49
- 1.15 to 1.16 - Rust 1.46
- 1.0 to 1.14 - Rust 1.45

Note that although we try to avoid the situation where a dependency transitively
increases the MSRV of Tokio, we do not guarantee that this does not happen.
However, every minor release will have some set of versions of dependencies that
works with the MSRV of that minor release.

## Release schedule

Tokio doesn't follow a fixed release schedule, but we typically make one minor
release each month. We make patch releases for bugfixes as necessary.

## Bug patching policy

For the purposes of making patch releases with bugfixes, we have designated
certain minor releases as LTS (long term support) releases. Whenever a bug
warrants a patch release with a fix for the bug, it will be backported and
released as a new patch release for each LTS minor version. Our current LTS
releases are:

- `1.43.x` - LTS release until March 2026. (MSRV 1.70)
- `1.47.x` - LTS release until September 2026. (MSRV 1.70)

Each LTS release will continue to receive backported fixes for at least a year.
If you wish to use a fixed minor release in your project, we recommend that you
use an LTS release.

To use a fixed minor version, you can specify the version with a tilde. For
example, to specify that you wish to use the newest `1.43.x` patch release, you
can use the following dependency specification:

```text
tokio = { version = "~1.43", features = [...] }
```

### Previous LTS releases

- `1.8.x` - LTS release until February 2022.
- `1.14.x` - LTS release until June 2022.
- `1.18.x` - LTS release until June 2023.
- `1.20.x` - LTS release until September 2023.
- `1.25.x` - LTS release until March 2024.
- `1.32.x` - LTS release until September 2024.
- `1.36.x` - LTS release until March 2025.
- `1.38.x` - LTS release until July 2025.

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/tokio-rs/tokio/blob/master/LICENSE

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Tokio by you shall be licensed as MIT, without any additional
terms or conditions.

# 1.49.0 (January 3rd, 2026)

### Added

- net: add support for `TCLASS` option on IPv6 ([#7781])
- runtime: stabilize `runtime::id::Id` ([#7125])
- task: implement `Extend` for `JoinSet` ([#7195])
- task: stabilize the `LocalSet::id()` ([#7776])

### Changed

- net: deprecate `{TcpStream,TcpSocket}::set_linger` ([#7752])

### Fixed

- macros: fix the hygiene issue of `join!` and `try_join!` ([#7766])
- runtime: revert "replace manual vtable definitions with Wake" ([#7699])
- sync: return `TryRecvError::Disconnected` from `Receiver::try_recv` after `Receiver::close` ([#7686])
- task: remove unnecessary trait bounds on the `Debug` implementation ([#7720])

### Unstable

- fs: handle `EINTR` in `fs::write` for io-uring ([#7786])
- fs: support io-uring with `tokio::fs::read` ([#7696])
- runtime: disable io-uring on `EPERM` ([#7724])
- time: add alternative timer for better multicore scalability ([#7467])

### Documented

- docs: fix a typos in `bounded.rs` and `park.rs` ([#7817])
- io: add `SyncIoBridge` cross-references to `copy` and `copy_buf` ([#7798])
- io: doc that `AsyncWrite` does not inherit from `std::io::Write` ([#7705])
- metrics: clarify that `num_alive_tasks` is not strongly consistent ([#7614])
- net: clarify the cancellation safety of the `TcpStream::peek` ([#7305])
- net: clarify the drop behavior of `unix::OwnedWriteHalf` ([#7742])
- net: clarify the platform-dependent backlog in `TcpSocket` docs ([#7738])
- runtime: mention `LocalRuntime` in `new_current_thread` docs ([#7820])
- sync: add missing period to `mpsc::Sender::try_send` docs ([#7721])
- sync: clarify the cancellation safety of `oneshot::Receiver` ([#7780])
- sync: improve the docs for the `errors` of mpsc ([#7722])
- task: add example for `spawn_local` usage on local runtime ([#7689])

[#7125]: https://github.com/tokio-rs/tokio/pull/7125
[#7195]: https://github.com/tokio-rs/tokio/pull/7195
[#7305]: https://github.com/tokio-rs/tokio/pull/7305
[#7467]: https://github.com/tokio-rs/tokio/pull/7467
[#7614]: https://github.com/tokio-rs/tokio/pull/7614
[#7686]: https://github.com/tokio-rs/tokio/pull/7686
[#7689]: https://github.com/tokio-rs/tokio/pull/7689
[#7696]: https://github.com/tokio-rs/tokio/pull/7696
[#7699]: https://github.com/tokio-rs/tokio/pull/7699
[#7705]: https://github.com/tokio-rs/tokio/pull/7705
[#7720]: https://github.com/tokio-rs/tokio/pull/7720
[#7721]: https://github.com/tokio-rs/tokio/pull/7721
[#7722]: https://github.com/tokio-rs/tokio/pull/7722
[#7724]: https://github.com/tokio-rs/tokio/pull/7724
[#7738]: https://github.com/tokio-rs/tokio/pull/7738
[#7742]: https://github.com/tokio-rs/tokio/pull/7742
[#7752]: https://github.com/tokio-rs/tokio/pull/7752
[#7766]: https://github.com/tokio-rs/tokio/pull/7766
[#7776]: https://github.com/tokio-rs/tokio/pull/7776
[#7780]: https://github.com/tokio-rs/tokio/pull/7780
[#7781]: https://github.com/tokio-rs/tokio/pull/7781
[#7786]: https://github.com/tokio-rs/tokio/pull/7786
[#7798]: https://github.com/tokio-rs/tokio/pull/7798
[#7817]: https://github.com/tokio-rs/tokio/pull/7817
[#7820]: https://github.com/tokio-rs/tokio/pull/7820

# 1.48.0 (October 14th, 2025)

The MSRV is increased to 1.71.

### Added

- fs: add `File::max_buf_size` ([#7594])
- io: export `Chain` of `AsyncReadExt::chain` ([#7599])
- net: add `SocketAddr::as_abstract_name` ([#7491])
- net: add `TcpStream::quickack` and `TcpStream::set_quickack` ([#7490])
- net: implement `AsRef<Self>` for `TcpStream` and `UnixStream` ([#7573])
- task: add `LocalKey::try_get` ([#7666])
- task: implement `Ord` for `task::Id` ([#7530])

### Changed

- deps: bump windows-sys to version 0.61 ([#7645])
- fs: preserve `max_buf_size` when cloning a `File` ([#7593])
- macros: suppress `clippy::unwrap_in_result` in `#[tokio::main]` ([#7651])
- net: remove `PollEvented` noise from Debug formats ([#7675])
- process: upgrade `Command::spawn_with` to use `FnOnce` ([#7511])
- sync: remove inner mutex in `SetOnce` ([#7554])
- sync: use `UnsafeCell::get_mut` in `Mutex::get_mut` and `RwLock::get_mut` ([#7569])
- time: reduce the generated code size of `Timeout<T>::poll` ([#7535])

### Fixed

- macros: fix hygiene issue in `join!` and `try_join!` ([#7638])
- net: fix copy/paste errors in udp peek methods ([#7604])
- process: fix error when runtime is shut down on nightly-2025-10-12 ([#7672])
- runtime: use release ordering in `wake_by_ref()` even if already woken ([#7622])
- sync: close the `broadcast::Sender` in `broadcast::Sender::new()` ([#7629])
- sync: fix implementation of unused `RwLock::try_*` methods ([#7587])

### Unstable

- tokio: use cargo features instead of `--cfg` flags for `taskdump` and `io_uring` ([#7655], [#7621])
- fs: support `io_uring` in `fs::write` ([#7567])
- fs: support `io_uring` with `File::open()` ([#7617])
- fs: support `io_uring` with `OpenOptions` ([#7321])
- macros: add `local` runtime flavor ([#7375], [#7597])

### Documented

- io: clarify the zero capacity case of `AsyncRead::poll_read` ([#7580])
- io: fix typos in the docs of `AsyncFd` readiness guards ([#7583])
- net: clarify socket gets closed on drop ([#7526])
- net: clarify the behavior of `UCred::pid()` on Cygwin ([#7611])
- net: clarify the supported platform of `set_reuseport()` and `reuseport()` ([#7628])
- net: qualify that `SO_REUSEADDR` is only set on Unix ([#7533])
- runtime: add guide for choosing between runtime types ([#7635])
- runtime: clarify the behavior of `Handle::block_on` ([#7665])
- runtime: clarify the edge case of `Builder::global_queue_interval()` ([#7605])
- sync: clarify bounded channel panic behavior ([#7641])
- sync: clarify the behavior of `tokio::sync::watch::Receiver` ([#7584])
- sync: document cancel safety on `SetOnce::wait` ([#7506])
- sync: fix the docs of `parking_lot` feature flag ([#7663])
- sync: improve the docs of `UnboundedSender::send` ([#7661])
- sync: improve the docs of `sync::watch` ([#7601])
- sync: reword allocation failure paragraph in broadcast docs ([#7595])
- task: clarify the behavior of several `spawn_local` methods ([#7669])
- task: clarify the task ID reuse guarantees ([#7577])
- task: improve the example of `poll_proceed` ([#7586])

[#7321]: https://github.com/tokio-rs/tokio/pull/7321
[#7375]: https://github.com/tokio-rs/tokio/pull/7375
[#7490]: https://github.com/tokio-rs/tokio/pull/7490
[#7491]: https://github.com/tokio-rs/tokio/pull/7491
[#7494]: https://github.com/tokio-rs/tokio/pull/7494
[#7506]: https://github.com/tokio-rs/tokio/pull/7506
[#7511]: https://github.com/tokio-rs/tokio/pull/7511
[#7526]: https://github.com/tokio-rs/tokio/pull/7526
[#7530]: https://github.com/tokio-rs/tokio/pull/7530
[#7533]: https://github.com/tokio-rs/tokio/pull/7533
[#7535]: https://github.com/tokio-rs/tokio/pull/7535
[#7554]: https://github.com/tokio-rs/tokio/pull/7554
[#7567]: https://github.com/tokio-rs/tokio/pull/7567
[#7569]: https://github.com/tokio-rs/tokio/pull/7569
[#7573]: https://github.com/tokio-rs/tokio/pull/7573
[#7577]: https://github.com/tokio-rs/tokio/pull/7577
[#7580]: https://github.com/tokio-rs/tokio/pull/7580
[#7583]: https://github.com/tokio-rs/tokio/pull/7583
[#7584]: https://github.com/tokio-rs/tokio/pull/7584
[#7586]: https://github.com/tokio-rs/tokio/pull/7586
[#7587]: https://github.com/tokio-rs/tokio/pull/7587
[#7593]: https://github.com/tokio-rs/tokio/pull/7593
[#7594]: https://github.com/tokio-rs/tokio/pull/7594
[#7595]: https://github.com/tokio-rs/tokio/pull/7595
[#7597]: https://github.com/tokio-rs/tokio/pull/7597
[#7599]: https://github.com/tokio-rs/tokio/pull/7599
[#7601]: https://github.com/tokio-rs/tokio/pull/7601
[#7604]: https://github.com/tokio-rs/tokio/pull/7604
[#7605]: https://github.com/tokio-rs/tokio/pull/7605
[#7611]: https://github.com/tokio-rs/tokio/pull/7611
[#7617]: https://github.com/tokio-rs/tokio/pull/7617
[#7621]: https://github.com/tokio-rs/tokio/pull/7621
[#7622]: https://github.com/tokio-rs/tokio/pull/7622
[#7628]: https://github.com/tokio-rs/tokio/pull/7628
[#7629]: https://github.com/tokio-rs/tokio/pull/7629
[#7635]: https://github.com/tokio-rs/tokio/pull/7635
[#7638]: https://github.com/tokio-rs/tokio/pull/7638
[#7641]: https://github.com/tokio-rs/tokio/pull/7641
[#7645]: https://github.com/tokio-rs/tokio/pull/7645
[#7651]: https://github.com/tokio-rs/tokio/pull/7651
[#7655]: https://github.com/tokio-rs/tokio/pull/7655
[#7661]: https://github.com/tokio-rs/tokio/pull/7661
[#7663]: https://github.com/tokio-rs/tokio/pull/7663
[#7665]: https://github.com/tokio-rs/tokio/pull/7665
[#7666]: https://github.com/tokio-rs/tokio/pull/7666
[#7669]: https://github.com/tokio-rs/tokio/pull/7669
[#7672]: https://github.com/tokio-rs/tokio/pull/7672
[#7675]: https://github.com/tokio-rs/tokio/pull/7675

# 1.47.3 (Januar 3rd, 2026)

### Fixed

- sync: return `TryRecvError::Disconnected` from `Receiver::try_recv` after `Receiver::close` ([#7686])

# 1.47.2 (October 14th, 2025)

### Fixed

- runtime: use release ordering in `wake_by_ref()` even if already woken ([#7622])
- sync: close the `broadcast::Sender` in `broadcast::Sender::new()` ([#7629])
- macros: fix hygiene issue in `join!` and `try_join!` ([#7638])
- process: fix error when runtime is shut down on nightly-2025-10-12 ([#7672])

[#7622]: https://github.com/tokio-rs/tokio/pull/7622
[#7629]: https://github.com/tokio-rs/tokio/pull/7629
[#7638]: https://github.com/tokio-rs/tokio/pull/7638
[#7672]: https://github.com/tokio-rs/tokio/pull/7672

# 1.47.1 (August 1st, 2025)

### Fixed

- process: fix panic from spurious pidfd wakeup ([#7494])
- sync: fix broken link of Python `asyncio.Event` in `SetOnce` docs ([#7485])

[#7485]: https://github.com/tokio-rs/tokio/pull/7485

# 1.47.0 (July 25th, 2025)

This release adds `poll_proceed` and `cooperative` to the `coop` module for
cooperative scheduling, adds `SetOnce` to the `sync` module which provides
similar functionality to [`std::sync::OnceLock`], and adds a new method
`sync::Notify::notified_owned()` which returns an `OwnedNotified` without
a lifetime parameter.

## Added

- coop: add `cooperative` and `poll_proceed` ([#7405])
- sync: add `SetOnce` ([#7418])
- sync: add `sync::Notify::notified_owned()` ([#7465])

## Changed

- deps: upgrade windows-sys 0.52 → 0.59 ([#7117])
- deps: update to socket2 v0.6 ([#7443])
- sync: improve `AtomicWaker::wake` performance ([#7450])

## Documented

- metrics: fix listed feature requirements for some metrics ([#7449])
- runtime: improve safety comments of `Readiness<'_>` ([#7415])

[#7117]: https://github.com/tokio-rs/tokio/pull/7117
[#7405]: https://github.com/tokio-rs/tokio/pull/7405
[#7415]: https://github.com/tokio-rs/tokio/pull/7415
[#7418]: https://github.com/tokio-rs/tokio/pull/7418
[#7443]: https://github.com/tokio-rs/tokio/pull/7443
[#7449]: https://github.com/tokio-rs/tokio/pull/7449
[#7450]: https://github.com/tokio-rs/tokio/pull/7450
[#7465]: https://github.com/tokio-rs/tokio/pull/7465

# 1.46.1 (July 4th, 2025)

This release fixes incorrect spawn locations in runtime task hooks for tasks
spawned using `tokio::spawn` rather than `Runtime::spawn`. This issue only
affected the spawn location in `TaskMeta::spawned_at`, and did not affect task
locations in Tracing events.

## Unstable

- runtime: add `TaskMeta::spawned_at` tracking where a task was spawned
  ([#7440])

[#7440]: https://github.com/tokio-rs/tokio/pull/7440

# 1.46.0 (July 2nd, 2025)

## Fixed

- net: fixed `TcpStream::shutdown` incorrectly returning an error on macOS ([#7290])

## Added

- sync: `mpsc::OwnedPermit::{same_channel, same_channel_as_sender}` methods ([#7389])
- macros: `biased` option for `join!` and `try_join!`, similar to `select!` ([#7307])
- net: support for cygwin ([#7393])
- net: support `pipe::OpenOptions::read_write` on Android ([#7426])
- net: add `Clone` implementation for `net::unix::SocketAddr` ([#7422])

## Changed

- runtime: eliminate unnecessary lfence while operating on `queue::Local<T>` ([#7340])
- task: disallow blocking in `LocalSet::{poll,drop}` ([#7372])

## Unstable

- runtime: add `TaskMeta::spawn_location` tracking where a task was spawned ([#7417])
- runtime: removed borrow from `LocalOptions` parameter to `runtime::Builder::build_local` ([#7346])

## Documented

- io: clarify behavior of seeking when `start_seek` is not used ([#7366])
- io: document cancellation safety of `AsyncWriteExt::flush` ([#7364])
- net: fix docs for `recv_buffer_size` method ([#7336])
- net: fix broken link of `RawFd` in `TcpSocket` docs ([#7416])
- net: update `AsRawFd` doc link to current Rust stdlib location ([#7429])
- readme: fix double period in reactor description ([#7363])
- runtime: add doc note that `on_*_task_poll` is unstable ([#7311])
- sync: update broadcast docs on allocation failure ([#7352])
- time: add a missing panic scenario of `time::advance` ([#7394])

[#7290]: https://github.com/tokio-rs/tokio/pull/7290
[#7307]: https://github.com/tokio-rs/tokio/pull/7307
[#7311]: https://github.com/tokio-rs/tokio/pull/7311
[#7336]: https://github.com/tokio-rs/tokio/pull/7336
[#7340]: https://github.com/tokio-rs/tokio/pull/7340
[#7346]: https://github.com/tokio-rs/tokio/pull/7346
[#7352]: https://github.com/tokio-rs/tokio/pull/7352
[#7363]: https://github.com/tokio-rs/tokio/pull/7363
[#7364]: https://github.com/tokio-rs/tokio/pull/7364
[#7366]: https://github.com/tokio-rs/tokio/pull/7366
[#7372]: https://github.com/tokio-rs/tokio/pull/7372
[#7389]: https://github.com/tokio-rs/tokio/pull/7389
[#7393]: https://github.com/tokio-rs/tokio/pull/7393
[#7394]: https://github.com/tokio-rs/tokio/pull/7394
[#7416]: https://github.com/tokio-rs/tokio/pull/7416
[#7422]: https://github.com/tokio-rs/tokio/pull/7422
[#7426]: https://github.com/tokio-rs/tokio/pull/7426
[#7429]: https://github.com/tokio-rs/tokio/pull/7429
[#7417]: https://github.com/tokio-rs/tokio/pull/7417

# 1.45.1 (May 24th, 2025)

This fixes a regression on the wasm32-unknown-unknown target, where code that
previously did not panic due to calls to `Instant::now()` started failing. This
is due to the stabilization of the first time-based metric.

### Fixed

- Disable time-based metrics on wasm32-unknown-unknown ([#7322])

[#7322]: https://github.com/tokio-rs/tokio/pull/7322

# 1.45.0 (May 5th, 2025)

### Added

- metrics: stabilize `worker_total_busy_duration`, `worker_park_count`, and
  `worker_unpark_count` ([#6899], [#7276])
- process: add `Command::spawn_with` ([#7249])

### Changed

- io: do not require `Unpin` for some trait impls ([#7204])
- rt: mark `runtime::Handle` as unwind safe ([#7230])
- time: revert internal sharding implementation ([#7226])

### Unstable

- rt: remove alt multi-threaded runtime ([#7275])

[#6899]: https://github.com/tokio-rs/tokio/pull/6899
[#7276]: https://github.com/tokio-rs/tokio/pull/7276
[#7249]: https://github.com/tokio-rs/tokio/pull/7249
[#7204]: https://github.com/tokio-rs/tokio/pull/7204
[#7230]: https://github.com/tokio-rs/tokio/pull/7230
[#7226]: https://github.com/tokio-rs/tokio/pull/7226
[#7275]: https://github.com/tokio-rs/tokio/pull/7275

# 1.44.2 (April 5th, 2025)

This release fixes a soundness issue in the broadcast channel. The channel
accepts values that are `Send` but `!Sync`. Previously, the channel called
`clone()` on these values without synchronizing. This release fixes the channel
by synchronizing calls to `.clone()` (Thanks Austin Bonander for finding and
reporting the issue).

### Fixed

- sync: synchronize `clone()` call in broadcast channel ([#7232])

[#7232]: https://github.com/tokio-rs/tokio/pull/7232

# 1.44.1 (March 13th, 2025)

### Fixed

- rt: skip defer queue in `block_in_place` context ([#7216])

[#7216]: https://github.com/tokio-rs/tokio/pull/7216

# 1.44.0 (March 7th, 2025)

This release changes the `from_std` method on sockets to panic if a blocking
socket is provided. We determined this change is not a breaking change as Tokio is not
intended to operate using blocking sockets. Doing so results in runtime hangs and
should be considered a bug. Accidentally passing a blocking socket to Tokio is one
of the most common user mistakes. If this change causes an issue for you, please
comment on [#7172].

### Added

- coop: add `task::coop` module ([#7116])
- process: add `Command::get_kill_on_drop()` ([#7086])
- sync: add `broadcast::Sender::closed` ([#6685], [#7090])
- sync: add `broadcast::WeakSender` ([#7100])
- sync: add `oneshot::Receiver::is_empty()` ([#7153])
- sync: add `oneshot::Receiver::is_terminated()` ([#7152])

### Fixed

- fs: empty reads on `File` should not start a background read ([#7139])
- process: calling `start_kill` on exited child should not fail ([#7160])
- signal: fix `CTRL_CLOSE`, `CTRL_LOGOFF`, `CTRL_SHUTDOWN` on windows ([#7122])
- sync: properly handle panic during mpsc drop ([#7094])

### Changes

- runtime: clean up magic number in registration set ([#7112])
- coop: make coop yield using waker defer strategy ([#7185])
- macros: make `select!` budget-aware ([#7164])
- net: panic when passing a blocking socket to `from_std` ([#7166])
- io: clean up buffer casts ([#7142])

### Changes to unstable APIs

- rt: add before and after task poll callbacks ([#7120])
- tracing: make the task tracing API unstable public ([#6972])

### Documented

- docs: fix nesting of sections in top-level docs ([#7159])
- fs: rename symlink and hardlink parameter names ([#7143])
- io: swap reader/writer in simplex doc test ([#7176])
- macros: docs about `select!` alternatives ([#7110])
- net: rename the argument for `send_to` ([#7146])
- process: add example for reading `Child` stdout ([#7141])
- process: clarify `Child::kill` behavior ([#7162])
- process: fix grammar of the `ChildStdin` struct doc comment ([#7192])
- runtime: consistently use `worker_threads` instead of `core_threads` ([#7186])

[#6685]: https://github.com/tokio-rs/tokio/pull/6685
[#6972]: https://github.com/tokio-rs/tokio/pull/6972
[#7086]: https://github.com/tokio-rs/tokio/pull/7086
[#7090]: https://github.com/tokio-rs/tokio/pull/7090
[#7094]: https://github.com/tokio-rs/tokio/pull/7094
[#7100]: https://github.com/tokio-rs/tokio/pull/7100
[#7110]: https://github.com/tokio-rs/tokio/pull/7110
[#7112]: https://github.com/tokio-rs/tokio/pull/7112
[#7116]: https://github.com/tokio-rs/tokio/pull/7116
[#7120]: https://github.com/tokio-rs/tokio/pull/7120
[#7122]: https://github.com/tokio-rs/tokio/pull/7122
[#7139]: https://github.com/tokio-rs/tokio/pull/7139
[#7141]: https://github.com/tokio-rs/tokio/pull/7141
[#7142]: https://github.com/tokio-rs/tokio/pull/7142
[#7143]: https://github.com/tokio-rs/tokio/pull/7143
[#7146]: https://github.com/tokio-rs/tokio/pull/7146
[#7152]: https://github.com/tokio-rs/tokio/pull/7152
[#7153]: https://github.com/tokio-rs/tokio/pull/7153
[#7159]: https://github.com/tokio-rs/tokio/pull/7159
[#7160]: https://github.com/tokio-rs/tokio/pull/7160
[#7162]: https://github.com/tokio-rs/tokio/pull/7162
[#7164]: https://github.com/tokio-rs/tokio/pull/7164
[#7166]: https://github.com/tokio-rs/tokio/pull/7166
[#7172]: https://github.com/tokio-rs/tokio/pull/7172
[#7176]: https://github.com/tokio-rs/tokio/pull/7176
[#7185]: https://github.com/tokio-rs/tokio/pull/7185
[#7186]: https://github.com/tokio-rs/tokio/pull/7186
[#7192]: https://github.com/tokio-rs/tokio/pull/7192

# 1.43.4 (January 3rd, 2026)

### Fixed

- sync: return `TryRecvError::Disconnected` from `Receiver::try_recv` after `Receiver::close` ([#7686])

[#7686]: https://github.com/tokio-rs/tokio/pull/7686

# 1.43.3 (October 14th, 2025)

### Fixed

- runtime: use release ordering in `wake_by_ref()` even if already woken ([#7622])
- sync: close the `broadcast::Sender` in `broadcast::Sender::new()` ([#7629])
- process: fix error when runtime is shut down on nightly-2025-10-12 ([#7672])

[#7622]: https://github.com/tokio-rs/tokio/pull/7622
[#7629]: https://github.com/tokio-rs/tokio/pull/7629
[#7672]: https://github.com/tokio-rs/tokio/pull/7672

# 1.43.2 (August 1st, 2025)

### Fixed

- process: fix panic from spurious pidfd wakeup ([#7494])

[#7494]: https://github.com/tokio-rs/tokio/pull/7494

# 1.43.1 (April 5th, 2025)

This release fixes a soundness issue in the broadcast channel. The channel
accepts values that are `Send` but `!Sync`. Previously, the channel called
`clone()` on these values without synchronizing. This release fixes the channel
by synchronizing calls to `.clone()` (Thanks Austin Bonander for finding and
reporting the issue).

### Fixed

- sync: synchronize `clone()` call in broadcast channel ([#7232])

[#7232]: https://github.com/tokio-rs/tokio/pull/7232

# 1.43.0 (Jan 8th, 2025)

### Added

- net: add `UdpSocket::peek` methods ([#7068])
- net: add support for Haiku OS ([#7042])
- process: add `Command::into_std()` ([#7014])
- signal: add `SignalKind::info` on illumos ([#6995])
- signal: add support for realtime signals on illumos ([#7029])

### Fixed

- io: don't call `set_len` before initializing vector in `Blocking` ([#7054])
- macros: suppress `clippy::needless_return` in `#[tokio::main]` ([#6874])
- runtime: fix thread parking on WebAssembly ([#7041])

### Changes

- chore: use unsync loads for `unsync_load` ([#7073])
- io: use `Buf::put_bytes` in `Repeat` read impl ([#7055])
- task: drop the join waker of a task eagerly ([#6986])

### Changes to unstable APIs

- metrics: improve flexibility of H2Histogram Configuration ([#6963])
- taskdump: add accessor methods for backtrace ([#6975])

### Documented

- io: clarify `ReadBuf::uninit` allows initialized buffers as well ([#7053])
- net: fix ambiguity in `TcpStream::try_write_vectored` docs ([#7067])
- runtime: fix `LocalRuntime` doc links ([#7074])
- sync: extend documentation for `watch::Receiver::wait_for` ([#7038])
- sync: fix typos in `OnceCell` docs ([#7047])

[#6874]: https://github.com/tokio-rs/tokio/pull/6874
[#6963]: https://github.com/tokio-rs/tokio/pull/6963
[#6975]: https://github.com/tokio-rs/tokio/pull/6975
[#6986]: https://github.com/tokio-rs/tokio/pull/6986
[#6995]: https://github.com/tokio-rs/tokio/pull/6995
[#7014]: https://github.com/tokio-rs/tokio/pull/7014
[#7029]: https://github.com/tokio-rs/tokio/pull/7029
[#7038]: https://github.com/tokio-rs/tokio/pull/7038
[#7041]: https://github.com/tokio-rs/tokio/pull/7041
[#7042]: https://github.com/tokio-rs/tokio/pull/7042
[#7047]: https://github.com/tokio-rs/tokio/pull/7047
[#7053]: https://github.com/tokio-rs/tokio/pull/7053
[#7054]: https://github.com/tokio-rs/tokio/pull/7054
[#7055]: https://github.com/tokio-rs/tokio/pull/7055
[#7067]: https://github.com/tokio-rs/tokio/pull/7067
[#7068]: https://github.com/tokio-rs/tokio/pull/7068
[#7073]: https://github.com/tokio-rs/tokio/pull/7073
[#7074]: https://github.com/tokio-rs/tokio/pull/7074

# 1.42.1 (April 8th, 2025)

This release fixes a soundness issue in the broadcast channel. The channel
accepts values that are `Send` but `!Sync`. Previously, the channel called
`clone()` on these values without synchronizing. This release fixes the channel
by synchronizing calls to `.clone()` (Thanks Austin Bonander for finding and
reporting the issue).

### Fixed

- sync: synchronize `clone()` call in broadcast channel ([#7232])

[#7232]: https://github.com/tokio-rs/tokio/pull/7232

# 1.42.0 (Dec 3rd, 2024)

### Added

- io: add `AsyncFd::{try_io, try_io_mut}` ([#6967])

### Fixed

- io: avoid `ptr->ref->ptr` roundtrip in RegistrationSet ([#6929])
- runtime: do not defer `yield_now` inside `block_in_place` ([#6999])

### Changes

- io: simplify io readiness logic ([#6966])

### Documented

- net: fix docs for `tokio::net::unix::{pid_t, gid_t, uid_t}` ([#6791])
- time: fix a typo in `Instant` docs ([#6982])

[#6791]: https://github.com/tokio-rs/tokio/pull/6791
[#6929]: https://github.com/tokio-rs/tokio/pull/6929
[#6966]: https://github.com/tokio-rs/tokio/pull/6966
[#6967]: https://github.com/tokio-rs/tokio/pull/6967
[#6982]: https://github.com/tokio-rs/tokio/pull/6982
[#6999]: https://github.com/tokio-rs/tokio/pull/6999

# 1.41.1 (Nov 7th, 2024)

### Fixed

- metrics: fix bug with wrong number of buckets for the histogram ([#6957])
- net: display `net` requirement for `net::UdpSocket` in docs ([#6938])
- net: fix typo in `TcpStream` internal comment ([#6944])

[#6957]: https://github.com/tokio-rs/tokio/pull/6957
[#6938]: https://github.com/tokio-rs/tokio/pull/6938
[#6944]: https://github.com/tokio-rs/tokio/pull/6944

# 1.41.0 (Oct 22nd, 2024)

### Added

- metrics: stabilize `global_queue_depth` ([#6854], [#6918])
- net: add conversions for unix `SocketAddr` ([#6868])
- sync: add `watch::Sender::sender_count` ([#6836])
- sync: add `mpsc::Receiver::blocking_recv_many` ([#6867])
- task: stabilize `Id` apis ([#6793], [#6891])

### Added (unstable)

- metrics: add H2 Histogram option to improve histogram granularity ([#6897])
- metrics: rename some histogram apis ([#6924])
- runtime: add `LocalRuntime` ([#6808])

### Changed

- runtime: box futures larger than 16k on release mode ([#6826])
- sync: add `#[must_use]` to `Notified` ([#6828])
- sync: make `watch` cooperative ([#6846])
- sync: make `broadcast::Receiver` cooperative ([#6870])
- task: add task size to tracing instrumentation ([#6881])
- wasm: enable `cfg_fs` for `wasi` target ([#6822])

### Fixed

- net: fix regression of abstract socket path in unix socket ([#6838])

### Documented

- io: recommend `OwnedFd` with `AsyncFd` ([#6821])
- io: document cancel safety of `AsyncFd` methods ([#6890])
- macros: render more comprehensible documentation for `join` and `try_join` ([#6814], [#6841])
- net: fix swapped examples for `TcpSocket::set_nodelay` and `TcpSocket::nodelay` ([#6840])
- sync: document runtime compatibility ([#6833])

[#6793]: https://github.com/tokio-rs/tokio/pull/6793
[#6808]: https://github.com/tokio-rs/tokio/pull/6808
[#6810]: https://github.com/tokio-rs/tokio/pull/6810
[#6814]: https://github.com/tokio-rs/tokio/pull/6814
[#6821]: https://github.com/tokio-rs/tokio/pull/6821
[#6822]: https://github.com/tokio-rs/tokio/pull/6822
[#6826]: https://github.com/tokio-rs/tokio/pull/6826
[#6828]: https://github.com/tokio-rs/tokio/pull/6828
[#6833]: https://github.com/tokio-rs/tokio/pull/6833
[#6836]: https://github.com/tokio-rs/tokio/pull/6836
[#6838]: https://github.com/tokio-rs/tokio/pull/6838
[#6840]: https://github.com/tokio-rs/tokio/pull/6840
[#6841]: https://github.com/tokio-rs/tokio/pull/6841
[#6846]: https://github.com/tokio-rs/tokio/pull/6846
[#6854]: https://github.com/tokio-rs/tokio/pull/6854
[#6867]: https://github.com/tokio-rs/tokio/pull/6867
[#6868]: https://github.com/tokio-rs/tokio/pull/6868
[#6870]: https://github.com/tokio-rs/tokio/pull/6870
[#6881]: https://github.com/tokio-rs/tokio/pull/6881
[#6890]: https://github.com/tokio-rs/tokio/pull/6890
[#6891]: https://github.com/tokio-rs/tokio/pull/6891
[#6897]: https://github.com/tokio-rs/tokio/pull/6897
[#6918]: https://github.com/tokio-rs/tokio/pull/6918
[#6924]: https://github.com/tokio-rs/tokio/pull/6924

# 1.40.0 (August 30th, 2024)

### Added

- io: add `util::SimplexStream` ([#6589])
- process: stabilize `Command::process_group` ([#6731])
- sync: add `{TrySendError,SendTimeoutError}::into_inner` ([#6755])
- task: add `JoinSet::join_all` ([#6784])

### Added (unstable)

- runtime: add `Builder::{on_task_spawn, on_task_terminate}` ([#6742])

### Changed

- io: use vectored io for `write_all_buf` when possible ([#6724])
- runtime: prevent niche-optimization to avoid triggering miri ([#6744])
- sync: mark mpsc types as `UnwindSafe` ([#6783])
- sync,time: make `Sleep` and `BatchSemaphore` instrumentation explicit roots ([#6727])
- task: use `NonZeroU64` for `task::Id` ([#6733])
- task: include panic message when printing `JoinError` ([#6753])
- task: add `#[must_use]` to `JoinHandle::abort_handle` ([#6762])
- time: eliminate timer wheel allocations ([#6779])

### Documented

- docs: clarify that `[build]` section doesn't go in Cargo.toml ([#6728])
- io: clarify zero remaining capacity case ([#6790])
- macros: improve documentation for `select!` ([#6774])
- sync: document mpsc channel allocation behavior ([#6773])

[#6589]: https://github.com/tokio-rs/tokio/pull/6589
[#6724]: https://github.com/tokio-rs/tokio/pull/6724
[#6727]: https://github.com/tokio-rs/tokio/pull/6727
[#6728]: https://github.com/tokio-rs/tokio/pull/6728
[#6731]: https://github.com/tokio-rs/tokio/pull/6731
[#6733]: https://github.com/tokio-rs/tokio/pull/6733
[#6742]: https://github.com/tokio-rs/tokio/pull/6742
[#6744]: https://github.com/tokio-rs/tokio/pull/6744
[#6753]: https://github.com/tokio-rs/tokio/pull/6753
[#6755]: https://github.com/tokio-rs/tokio/pull/6755
[#6762]: https://github.com/tokio-rs/tokio/pull/6762
[#6773]: https://github.com/tokio-rs/tokio/pull/6773
[#6774]: https://github.com/tokio-rs/tokio/pull/6774
[#6779]: https://github.com/tokio-rs/tokio/pull/6779
[#6783]: https://github.com/tokio-rs/tokio/pull/6783
[#6784]: https://github.com/tokio-rs/tokio/pull/6784
[#6790]: https://github.com/tokio-rs/tokio/pull/6790

# 1.39.3 (August 17th, 2024)

This release fixes a regression where the unix socket api stopped accepting
the abstract socket namespace. ([#6772])

[#6772]: https://github.com/tokio-rs/tokio/pull/6772

# 1.39.2 (July 27th, 2024)

This release fixes a regression where the `select!` macro stopped accepting
expressions that make use of temporary lifetime extension. ([#6722])

[#6722]: https://github.com/tokio-rs/tokio/pull/6722

# 1.39.1 (July 23rd, 2024)

This release reverts "time: avoid traversing entries in the time wheel twice"
because it contains a bug. ([#6715])

[#6715]: https://github.com/tokio-rs/tokio/pull/6715

# 1.39.0 (July 23rd, 2024)

Yanked. Please use 1.39.1 instead.

- This release bumps the MSRV to 1.70. ([#6645])
- This release upgrades to mio v1. ([#6635])
- This release upgrades to windows-sys v0.52 ([#6154])

### Added

- io: implement `AsyncSeek` for `Empty` ([#6663])
- metrics: stabilize `num_alive_tasks` ([#6619], [#6667])
- process: add `Command::as_std_mut` ([#6608])
- sync: add `watch::Sender::same_channel` ([#6637])
- sync: add `{Receiver,UnboundedReceiver}::{sender_strong_count,sender_weak_count}` ([#6661])
- sync: implement `Default` for `watch::Sender` ([#6626])
- task: implement `Clone` for `AbortHandle` ([#6621])
- task: stabilize `consume_budget` ([#6622])

### Changed

- io: improve panic message of `ReadBuf::put_slice()` ([#6629])
- io: read during write in `copy_bidirectional` and `copy` ([#6532])
- runtime: replace `num_cpus` with `available_parallelism` ([#6709])
- task: avoid stack overflow when passing large future to `block_on` ([#6692])
- time: avoid traversing entries in the time wheel twice ([#6584])
- time: support `IntoFuture` with `timeout` ([#6666])
- macros: support `IntoFuture` with `join!` and `select!` ([#6710])

### Fixed

- docs: fix docsrs builds with the fs feature enabled ([#6585])
- io: only use short-read optimization on known-to-be-compatible platforms ([#6668])
- time: fix overflow panic when using large durations with `Interval` ([#6612])

### Added (unstable)

- macros: allow `unhandled_panic` behavior for `#[tokio::main]` and `#[tokio::test]` ([#6593])
- metrics: add `spawned_tasks_count` ([#6114])
- metrics: add `worker_park_unpark_count` ([#6696])
- metrics: add worker thread id ([#6695])

### Documented

- io: update `tokio::io::stdout` documentation ([#6674])
- macros: typo fix in `join.rs` and `try_join.rs` ([#6641])
- runtime: fix typo in `unhandled_panic` ([#6660])
- task: document behavior of `JoinSet::try_join_next` when all tasks are running ([#6671])

[#6114]: https://github.com/tokio-rs/tokio/pull/6114
[#6154]: https://github.com/tokio-rs/tokio/pull/6154
[#6532]: https://github.com/tokio-rs/tokio/pull/6532
[#6584]: https://github.com/tokio-rs/tokio/pull/6584
[#6585]: https://github.com/tokio-rs/tokio/pull/6585
[#6593]: https://github.com/tokio-rs/tokio/pull/6593
[#6608]: https://github.com/tokio-rs/tokio/pull/6608
[#6612]: https://github.com/tokio-rs/tokio/pull/6612
[#6619]: https://github.com/tokio-rs/tokio/pull/6619
[#6621]: https://github.com/tokio-rs/tokio/pull/6621
[#6622]: https://github.com/tokio-rs/tokio/pull/6622
[#6626]: https://github.com/tokio-rs/tokio/pull/6626
[#6629]: https://github.com/tokio-rs/tokio/pull/6629
[#6635]: https://github.com/tokio-rs/tokio/pull/6635
[#6637]: https://github.com/tokio-rs/tokio/pull/6637
[#6641]: https://github.com/tokio-rs/tokio/pull/6641
[#6645]: https://github.com/tokio-rs/tokio/pull/6645
[#6660]: https://github.com/tokio-rs/tokio/pull/6660
[#6661]: https://github.com/tokio-rs/tokio/pull/6661
[#6663]: https://github.com/tokio-rs/tokio/pull/6663
[#6666]: https://github.com/tokio-rs/tokio/pull/6666
[#6667]: https://github.com/tokio-rs/tokio/pull/6667
[#6668]: https://github.com/tokio-rs/tokio/pull/6668
[#6671]: https://github.com/tokio-rs/tokio/pull/6671
[#6674]: https://github.com/tokio-rs/tokio/pull/6674
[#6692]: https://github.com/tokio-rs/tokio/pull/6692
[#6695]: https://github.com/tokio-rs/tokio/pull/6695
[#6696]: https://github.com/tokio-rs/tokio/pull/6696
[#6709]: https://github.com/tokio-rs/tokio/pull/6709
[#6710]: https://github.com/tokio-rs/tokio/pull/6710

# 1.38.2 (April 2nd, 2025)

This release fixes a soundness issue in the broadcast channel. The channel
accepts values that are `Send` but `!Sync`. Previously, the channel called
`clone()` on these values without synchronizing. This release fixes the channel
by synchronizing calls to `.clone()` (Thanks Austin Bonander for finding and
reporting the issue).

### Fixed

- sync: synchronize `clone()` call in broadcast channel ([#7232])

[#7232]: https://github.com/tokio-rs/tokio/pull/7232

# 1.38.1 (July 16th, 2024)

This release fixes the bug identified as ([#6682]), which caused timers not
to fire when they should.

### Fixed

- time: update `wake_up` while holding all the locks of sharded time wheels ([#6683])

[#6682]: https://github.com/tokio-rs/tokio/pull/6682
[#6683]: https://github.com/tokio-rs/tokio/pull/6683

# 1.38.0 (May 30th, 2024)

This release marks the beginning of stabilization for runtime metrics. It
stabilizes `RuntimeMetrics::worker_count`. Future releases will continue to
stabilize more metrics.

### Added

- fs: add `File::create_new` ([#6573])
- io: add `copy_bidirectional_with_sizes` ([#6500])
- io: implement `AsyncBufRead` for `Join` ([#6449])
- net: add Apple visionOS support ([#6465])
- net: implement `Clone` for `NamedPipeInfo` ([#6586])
- net: support QNX OS ([#6421])
- sync: add `Notify::notify_last` ([#6520])
- sync: add `mpsc::Receiver::{capacity,max_capacity}` ([#6511])
- sync: add `split` method to the semaphore permit ([#6472], [#6478])
- task: add `tokio::task::join_set::Builder::spawn_blocking` ([#6578])
- wasm: support rt-multi-thread with wasm32-wasi-preview1-threads ([#6510])

### Changed

- macros: make `#[tokio::test]` append `#[test]` at the end of the attribute list ([#6497])
- metrics: fix `blocking_threads` count ([#6551])
- metrics: stabilize `RuntimeMetrics::worker_count` ([#6556])
- runtime: move task out of the `lifo_slot` in `block_in_place` ([#6596])
- runtime: panic if `global_queue_interval` is zero ([#6445])
- sync: always drop message in destructor for oneshot receiver ([#6558])
- sync: instrument `Semaphore` for task dumps ([#6499])
- sync: use FIFO ordering when waking batches of wakers ([#6521])
- task: make `LocalKey::get` work with Clone types ([#6433])
- tests: update nix and mio-aio dev-dependencies ([#6552])
- time: clean up implementation ([#6517])
- time: lazily init timers on first poll ([#6512])
- time: remove the `true_when` field in `TimerShared` ([#6563])
- time: use sharding for timer implementation ([#6534])

### Fixed

- taskdump: allow building taskdump docs on non-unix machines ([#6564])
- time: check for overflow in `Interval::poll_tick` ([#6487])
- sync: fix incorrect `is_empty` on mpsc block boundaries ([#6603])

### Documented

- fs: rewrite file system docs ([#6467])
- io: fix `stdin` documentation ([#6581])
- io: fix obsolete reference in `ReadHalf::unsplit()` documentation ([#6498])
- macros: render more comprehensible documentation for `select!` ([#6468])
- net: add missing types to module docs ([#6482])
- net: fix misleading `NamedPipeServer` example ([#6590])
- sync: add examples for `SemaphorePermit`, `OwnedSemaphorePermit` ([#6477])
- sync: document that `Barrier::wait` is not cancel safe ([#6494])
- sync: explain relation between `watch::Sender::{subscribe,closed}` ([#6490])
- task: clarify that you can't abort `spawn_blocking` tasks ([#6571])
- task: fix a typo in doc of `LocalSet::run_until` ([#6599])
- time: fix test-util requirement for pause and resume in docs ([#6503])

[#6421]: https://github.com/tokio-rs/tokio/pull/6421
[#6433]: https://github.com/tokio-rs/tokio/pull/6433
[#6445]: https://github.com/tokio-rs/tokio/pull/6445
[#6449]: https://github.com/tokio-rs/tokio/pull/6449
[#6465]: https://github.com/tokio-rs/tokio/pull/6465
[#6467]: https://github.com/tokio-rs/tokio/pull/6467
[#6468]: https://github.com/tokio-rs/tokio/pull/6468
[#6472]: https://github.com/tokio-rs/tokio/pull/6472
[#6477]: https://github.com/tokio-rs/tokio/pull/6477
[#6478]: https://github.com/tokio-rs/tokio/pull/6478
[#6482]: https://github.com/tokio-rs/tokio/pull/6482
[#6487]: https://github.com/tokio-rs/tokio/pull/6487
[#6490]: https://github.com/tokio-rs/tokio/pull/6490
[#6494]: https://github.com/tokio-rs/tokio/pull/6494
[#6497]: https://github.com/tokio-rs/tokio/pull/6497
[#6498]: https://github.com/tokio-rs/tokio/pull/6498
[#6499]: https://github.com/tokio-rs/tokio/pull/6499
[#6500]: https://github.com/tokio-rs/tokio/pull/6500
[#6503]: https://github.com/tokio-rs/tokio/pull/6503
[#6510]: https://github.com/tokio-rs/tokio/pull/6510
[#6511]: https://github.com/tokio-rs/tokio/pull/6511
[#6512]: https://github.com/tokio-rs/tokio/pull/6512
[#6517]: https://github.com/tokio-rs/tokio/pull/6517
[#6520]: https://github.com/tokio-rs/tokio/pull/6520
[#6521]: https://github.com/tokio-rs/tokio/pull/6521
[#6534]: https://github.com/tokio-rs/tokio/pull/6534
[#6551]: https://github.com/tokio-rs/tokio/pull/6551
[#6552]: https://github.com/tokio-rs/tokio/pull/6552
[#6556]: https://github.com/tokio-rs/tokio/pull/6556
[#6558]: https://github.com/tokio-rs/tokio/pull/6558
[#6563]: https://github.com/tokio-rs/tokio/pull/6563
[#6564]: https://github.com/tokio-rs/tokio/pull/6564
[#6571]: https://github.com/tokio-rs/tokio/pull/6571
[#6573]: https://github.com/tokio-rs/tokio/pull/6573
[#6578]: https://github.com/tokio-rs/tokio/pull/6578
[#6581]: https://github.com/tokio-rs/tokio/pull/6581
[#6586]: https://github.com/tokio-rs/tokio/pull/6586
[#6590]: https://github.com/tokio-rs/tokio/pull/6590
[#6596]: https://github.com/tokio-rs/tokio/pull/6596
[#6599]: https://github.com/tokio-rs/tokio/pull/6599
[#6603]: https://github.com/tokio-rs/tokio/pull/6603

# 1.37.0 (March 28th, 2024)

### Added

- fs: add `set_max_buf_size` to `tokio::fs::File` ([#6411])
- io: add `try_new` and `try_with_interest` to `AsyncFd` ([#6345])
- sync: add `forget_permits` method to semaphore ([#6331])
- sync: add `is_closed`, `is_empty`, and `len` to mpsc receivers ([#6348])
- sync: add a `rwlock()` method to owned `RwLock` guards ([#6418])
- sync: expose strong and weak counts of mpsc sender handles ([#6405])
- sync: implement `Clone` for `watch::Sender` ([#6388])
- task: add `TaskLocalFuture::take_value` ([#6340])
- task: implement `FromIterator` for `JoinSet` ([#6300])

### Changed

- io: make `io::split` use a mutex instead of a spinlock ([#6403])

### Fixed

- docs: fix docsrs build without net feature ([#6360])
- macros: allow select with only else branch ([#6339])
- runtime: fix leaking registration entries when os registration fails ([#6329])

### Documented

- io: document cancel safety of `AsyncBufReadExt::fill_buf` ([#6431])
- io: document cancel safety of `AsyncReadExt`'s primitive read functions ([#6337])
- runtime: add doc link from `Runtime` to `#[tokio::main]` ([#6366])
- runtime: make the `enter` example deterministic ([#6351])
- sync: add Semaphore example for limiting the number of outgoing requests ([#6419])
- sync: fix missing period in broadcast docs ([#6377])
- sync: mark `mpsc::Sender::downgrade` with `#[must_use]` ([#6326])
- sync: reorder `const_new` before `new_with` ([#6392])
- sync: update watch channel docs ([#6395])
- task: fix documentation links ([#6336])

### Changed (unstable)

- runtime: include task `Id` in taskdumps ([#6328])
- runtime: panic if `unhandled_panic` is enabled when not supported ([#6410])

[#6300]: https://github.com/tokio-rs/tokio/pull/6300
[#6326]: https://github.com/tokio-rs/tokio/pull/6326
[#6328]: https://github.com/tokio-rs/tokio/pull/6328
[#6329]: https://github.com/tokio-rs/tokio/pull/6329
[#6331]: https://github.com/tokio-rs/tokio/pull/6331
[#6336]: https://github.com/tokio-rs/tokio/pull/6336
[#6337]: https://github.com/tokio-rs/tokio/pull/6337
[#6339]: https://github.com/tokio-rs/tokio/pull/6339
[#6340]: https://github.com/tokio-rs/tokio/pull/6340
[#6345]: https://github.com/tokio-rs/tokio/pull/6345
[#6348]: https://github.com/tokio-rs/tokio/pull/6348
[#6351]: https://github.com/tokio-rs/tokio/pull/6351
[#6360]: https://github.com/tokio-rs/tokio/pull/6360
[#6366]: https://github.com/tokio-rs/tokio/pull/6366
[#6377]: https://github.com/tokio-rs/tokio/pull/6377
[#6388]: https://github.com/tokio-rs/tokio/pull/6388
[#6392]: https://github.com/tokio-rs/tokio/pull/6392
[#6395]: https://github.com/tokio-rs/tokio/pull/6395
[#6403]: https://github.com/tokio-rs/tokio/pull/6403
[#6405]: https://github.com/tokio-rs/tokio/pull/6405
[#6410]: https://github.com/tokio-rs/tokio/pull/6410
[#6411]: https://github.com/tokio-rs/tokio/pull/6411
[#6418]: https://github.com/tokio-rs/tokio/pull/6418
[#6419]: https://github.com/tokio-rs/tokio/pull/6419
[#6431]: https://github.com/tokio-rs/tokio/pull/6431

# 1.36.0 (February 2nd, 2024)

### Added

- io: add `tokio::io::Join` ([#6220])
- io: implement `AsyncWrite` for `Empty` ([#6235])
- net: add support for anonymous unix pipes ([#6127])
- net: add `UnixSocket` ([#6290])
- net: expose keepalive option on `TcpSocket` ([#6311])
- sync: add `{Receiver,UnboundedReceiver}::poll_recv_many` ([#6236])
- sync: add `Sender::{try_,}reserve_many` ([#6205])
- sync: add `watch::Receiver::mark_unchanged` ([#6252])
- task: add `JoinSet::try_join_next` ([#6280])

### Changed

- io: make `copy` cooperative ([#6265])
- io: make `repeat` and `sink` cooperative ([#6254])
- io: simplify check for empty slice ([#6293])
- process: use pidfd on Linux when available ([#6152])
- sync: use AtomicBool in broadcast channel future ([#6298])

### Documented

- io: clarify `clear_ready` docs ([#6304])
- net: document that `*Fd` traits on `TcpSocket` are unix-only ([#6294])
- sync: document FIFO behavior of `tokio::sync::Mutex` ([#6279])
- chore: typographic improvements ([#6262])
- runtime: remove obsolete comment ([#6303])
- task: fix typo ([#6261])

[#6220]: https://github.com/tokio-rs/tokio/pull/6220
[#6235]: https://github.com/tokio-rs/tokio/pull/6235
[#6127]: https://github.com/tokio-rs/tokio/pull/6127
[#6290]: https://github.com/tokio-rs/tokio/pull/6290
[#6311]: https://github.com/tokio-rs/tokio/pull/6311
[#6236]: https://github.com/tokio-rs/tokio/pull/6236
[#6205]: https://github.com/tokio-rs/tokio/pull/6205
[#6252]: https://github.com/tokio-rs/tokio/pull/6252
[#6280]: https://github.com/tokio-rs/tokio/pull/6280
[#6265]: https://github.com/tokio-rs/tokio/pull/6265
[#6254]: https://github.com/tokio-rs/tokio/pull/6254
[#6293]: https://github.com/tokio-rs/tokio/pull/6293
[#6238]: https://github.com/tokio-rs/tokio/pull/6238
[#6152]: https://github.com/tokio-rs/tokio/pull/6152
[#6298]: https://github.com/tokio-rs/tokio/pull/6298
[#6262]: https://github.com/tokio-rs/tokio/pull/6262
[#6303]: https://github.com/tokio-rs/tokio/pull/6303
[#6261]: https://github.com/tokio-rs/tokio/pull/6261
[#6304]: https://github.com/tokio-rs/tokio/pull/6304
[#6294]: https://github.com/tokio-rs/tokio/pull/6294
[#6279]: https://github.com/tokio-rs/tokio/pull/6279

# 1.35.1 (December 19, 2023)

This is a forward part of a change that was backported to 1.25.3.

### Fixed

- io: add budgeting to `tokio::runtime::io::registration::async_io` ([#6221])

[#6221]: https://github.com/tokio-rs/tokio/pull/6221

# 1.35.0 (December 8th, 2023)

### Added

- net: add Apple watchOS support ([#6176])

### Changed

- io: drop the `Sized` requirements from `AsyncReadExt.read_buf` ([#6169])
- runtime: make `Runtime` unwind safe ([#6189])
- runtime: reduce the lock contention in task spawn ([#6001])
- tokio: update nix dependency to 0.27.1 ([#6190])

### Fixed

- chore: make `--cfg docsrs` work without net feature ([#6166])
- chore: use relaxed load for `unsync_load` on miri ([#6179])
- runtime: handle missing context on wake ([#6148])
- taskdump: fix taskdump cargo config example ([#6150])
- taskdump: skip notified tasks during taskdumps ([#6194])
- tracing: avoid creating resource spans with current parent, use a None parent instead ([#6107])
- tracing: make task span explicit root ([#6158])

### Documented

- io: flush in `AsyncWriteExt` examples ([#6149])
- runtime: document fairness guarantees and current behavior ([#6145])
- task: document cancel safety of `LocalSet::run_until` ([#6147])

[#6001]: https://github.com/tokio-rs/tokio/pull/6001
[#6107]: https://github.com/tokio-rs/tokio/pull/6107
[#6144]: https://github.com/tokio-rs/tokio/pull/6144
[#6145]: https://github.com/tokio-rs/tokio/pull/6145
[#6147]: https://github.com/tokio-rs/tokio/pull/6147
[#6148]: https://github.com/tokio-rs/tokio/pull/6148
[#6149]: https://github.com/tokio-rs/tokio/pull/6149
[#6150]: https://github.com/tokio-rs/tokio/pull/6150
[#6158]: https://github.com/tokio-rs/tokio/pull/6158
[#6166]: https://github.com/tokio-rs/tokio/pull/6166
[#6169]: https://github.com/tokio-rs/tokio/pull/6169
[#6176]: https://github.com/tokio-rs/tokio/pull/6176
[#6179]: https://github.com/tokio-rs/tokio/pull/6179
[#6189]: https://github.com/tokio-rs/tokio/pull/6189
[#6190]: https://github.com/tokio-rs/tokio/pull/6190
[#6194]: https://github.com/tokio-rs/tokio/pull/6194

# 1.34.0 (November 19th, 2023)

### Fixed

- io: allow `clear_readiness` after io driver shutdown ([#6067])
- io: fix integer overflow in `take` ([#6080])
- io: fix I/O resource hang ([#6134])
- sync: fix `broadcast::channel` link ([#6100])

### Changed

- macros: use `::core` qualified imports instead of `::std` inside `tokio::test` macro ([#5973])

### Added

- fs: update cfg attr in `fs::read_dir` to include `aix` ([#6075])
- sync: add `mpsc::Receiver::recv_many` ([#6010])
- tokio: added vita target support ([#6094])

[#5973]: https://github.com/tokio-rs/tokio/pull/5973
[#6067]: https://github.com/tokio-rs/tokio/pull/6067
[#6080]: https://github.com/tokio-rs/tokio/pull/6080
[#6134]: https://github.com/tokio-rs/tokio/pull/6134
[#6100]: https://github.com/tokio-rs/tokio/pull/6100
[#6075]: https://github.com/tokio-rs/tokio/pull/6075
[#6010]: https://github.com/tokio-rs/tokio/pull/6010
[#6094]: https://github.com/tokio-rs/tokio/pull/6094

# 1.33.0 (October 9, 2023)

### Fixed

- io: mark `Interest::add` with `#[must_use]` ([#6037])
- runtime: fix cache line size for RISC-V ([#5994])
- sync: prevent lock poisoning in `watch::Receiver::wait_for` ([#6021])
- task: fix `spawn_local` source location ([#5984])

### Changed

- sync: use Acquire/Release orderings instead of SeqCst in `watch` ([#6018])

### Added

- fs: add vectored writes to `tokio::fs::File` ([#5958])
- io: add `Interest::remove` method ([#5906])
- io: add vectored writes to `DuplexStream` ([#5985])
- net: add Apple tvOS support ([#6045])
- sync: add `?Sized` bound to `{MutexGuard,OwnedMutexGuard}::map` ([#5997])
- sync: add `watch::Receiver::mark_unseen` ([#5962], [#6014], [#6017])
- sync: add `watch::Sender::new` ([#5998])
- sync: add const fn `OnceCell::from_value` ([#5903])

### Removed

- remove unused `stats` feature ([#5952])

### Documented

- add missing backticks in code examples ([#5938], [#6056])
- fix typos ([#5988], [#6030])
- process: document that `Child::wait` is cancel safe ([#5977])
- sync: add examples for `Semaphore` ([#5939], [#5956], [#5978], [#6031], [#6032], [#6050])
- sync: document that `broadcast` capacity is a lower bound ([#6042])
- sync: document that `const_new` is not instrumented ([#6002])
- sync: improve cancel-safety documentation for `mpsc::Sender::send` ([#5947])
- sync: improve docs for `watch` channel ([#5954])
- taskdump: render taskdump documentation on docs.rs ([#5972])

### Unstable

- taskdump: fix potential deadlock ([#6036])

[#5903]: https://github.com/tokio-rs/tokio/pull/5903
[#5906]: https://github.com/tokio-rs/tokio/pull/5906
[#5938]: https://github.com/tokio-rs/tokio/pull/5938
[#5939]: https://github.com/tokio-rs/tokio/pull/5939
[#5947]: https://github.com/tokio-rs/tokio/pull/5947
[#5952]: https://github.com/tokio-rs/tokio/pull/5952
[#5954]: https://github.com/tokio-rs/tokio/pull/5954
[#5956]: https://github.com/tokio-rs/tokio/pull/5956
[#5958]: https://github.com/tokio-rs/tokio/pull/5958
[#5960]: https://github.com/tokio-rs/tokio/pull/5960
[#5962]: https://github.com/tokio-rs/tokio/pull/5962
[#5971]: https://github.com/tokio-rs/tokio/pull/5971
[#5972]: https://github.com/tokio-rs/tokio/pull/5972
[#5977]: https://github.com/tokio-rs/tokio/pull/5977
[#5978]: https://github.com/tokio-rs/tokio/pull/5978
[#5984]: https://github.com/tokio-rs/tokio/pull/5984
[#5985]: https://github.com/tokio-rs/tokio/pull/5985
[#5988]: https://github.com/tokio-rs/tokio/pull/5988
[#5994]: https://github.com/tokio-rs/tokio/pull/5994
[#5997]: https://github.com/tokio-rs/tokio/pull/5997
[#5998]: https://github.com/tokio-rs/tokio/pull/5998
[#6002]: https://github.com/tokio-rs/tokio/pull/6002
[#6014]: https://github.com/tokio-rs/tokio/pull/6014
[#6017]: https://github.com/tokio-rs/tokio/pull/6017
[#6018]: https://github.com/tokio-rs/tokio/pull/6018
[#6021]: https://github.com/tokio-rs/tokio/pull/6021
[#6030]: https://github.com/tokio-rs/tokio/pull/6030
[#6031]: https://github.com/tokio-rs/tokio/pull/6031
[#6032]: https://github.com/tokio-rs/tokio/pull/6032
[#6036]: https://github.com/tokio-rs/tokio/pull/6036
[#6037]: https://github.com/tokio-rs/tokio/pull/6037
[#6042]: https://github.com/tokio-rs/tokio/pull/6042
[#6045]: https://github.com/tokio-rs/tokio/pull/6045
[#6050]: https://github.com/tokio-rs/tokio/pull/6050
[#6056]: https://github.com/tokio-rs/tokio/pull/6056
[#6058]: https://github.com/tokio-rs/tokio/pull/6058

# 1.32.1 (December 19, 2023)

This is a forward part of a change that was backported to 1.25.3.

### Fixed

- io: add budgeting to `tokio::runtime::io::registration::async_io` ([#6221])

[#6221]: https://github.com/tokio-rs/tokio/pull/6221

# 1.32.0 (August 16, 2023)

### Fixed

- sync: fix potential quadratic behavior in `broadcast::Receiver` ([#5925])

### Added

- process: stabilize `Command::raw_arg` ([#5930])
- io: enable awaiting error readiness ([#5781])

### Unstable

- rt(alt): improve scalability of alt runtime as the number of cores grows ([#5935])

[#5925]: https://github.com/tokio-rs/tokio/pull/5925
[#5930]: https://github.com/tokio-rs/tokio/pull/5930
[#5781]: https://github.com/tokio-rs/tokio/pull/5781
[#5935]: https://github.com/tokio-rs/tokio/pull/5935

# 1.31.0 (August 10, 2023)

### Fixed

- io: delegate `WriteHalf::poll_write_vectored` ([#5914])

### Unstable

- rt(alt): fix memory leak in unstable next-gen scheduler prototype ([#5911])
- rt: expose mean task poll time metric ([#5927])

[#5914]: https://github.com/tokio-rs/tokio/pull/5914
[#5911]: https://github.com/tokio-rs/tokio/pull/5911
[#5927]: https://github.com/tokio-rs/tokio/pull/5927

# 1.30.0 (August 9, 2023)

This release bumps the MSRV of Tokio to 1.63. ([#5887])

### Changed

- tokio: reduce LLVM code generation ([#5859])
- io: support `--cfg mio_unsupported_force_poll_poll` flag ([#5881])
- sync: make `const_new` methods always available ([#5885])
- sync: avoid false sharing in mpsc channel ([#5829])
- rt: pop at least one task from inject queue ([#5908])

### Added

- sync: add `broadcast::Sender::new` ([#5824])
- net: implement `UCred` for espidf ([#5868])
- fs: add `File::options()` ([#5869])
- time: implement extra reset variants for `Interval` ([#5878])
- process: add `{ChildStd*}::into_owned_{fd, handle}` ([#5899])

### Removed

- tokio: removed unused `tokio_*` cfgs ([#5890])
- remove build script to speed up compilation ([#5887])

### Documented

- sync: mention lagging in docs for `broadcast::send` ([#5820])
- runtime: expand on sharing runtime docs ([#5858])
- io: use vec in example for `AsyncReadExt::read_exact` ([#5863])
- time: mark `Sleep` as `!Unpin` in docs ([#5916])
- process: fix `raw_arg` not showing up in docs ([#5865])

### Unstable

- rt: add runtime ID ([#5864])
- rt: initial implementation of new threaded runtime ([#5823])

[#5820]: https://github.com/tokio-rs/tokio/pull/5820
[#5823]: https://github.com/tokio-rs/tokio/pull/5823
[#5824]: https://github.com/tokio-rs/tokio/pull/5824
[#5829]: https://github.com/tokio-rs/tokio/pull/5829
[#5858]: https://github.com/tokio-rs/tokio/pull/5858
[#5859]: https://github.com/tokio-rs/tokio/pull/5859
[#5863]: https://github.com/tokio-rs/tokio/pull/5863
[#5864]: https://github.com/tokio-rs/tokio/pull/5864
[#5865]: https://github.com/tokio-rs/tokio/pull/5865
[#5868]: https://github.com/tokio-rs/tokio/pull/5868
[#5869]: https://github.com/tokio-rs/tokio/pull/5869
[#5878]: https://github.com/tokio-rs/tokio/pull/5878
[#5881]: https://github.com/tokio-rs/tokio/pull/5881
[#5885]: https://github.com/tokio-rs/tokio/pull/5885
[#5887]: https://github.com/tokio-rs/tokio/pull/5887
[#5890]: https://github.com/tokio-rs/tokio/pull/5890
[#5899]: https://github.com/tokio-rs/tokio/pull/5899
[#5908]: https://github.com/tokio-rs/tokio/pull/5908
[#5916]: https://github.com/tokio-rs/tokio/pull/5916

# 1.29.1 (June 29, 2023)

### Fixed

- rt: fix nesting two `block_in_place` with a `block_on` between ([#5837])

[#5837]: https://github.com/tokio-rs/tokio/pull/5837

# 1.29.0 (June 27, 2023)

Technically a breaking change, the `Send` implementation is removed from
`runtime::EnterGuard`. This change fixes a bug and should not impact most users.

### Breaking

- rt: `EnterGuard` should not be `Send` ([#5766])

### Fixed

- fs: reduce blocking ops in `fs::read_dir` ([#5653])
- rt: fix possible starvation ([#5686], [#5712])
- rt: fix stacked borrows issue in `JoinSet` ([#5693])
- rt: panic if `EnterGuard` dropped incorrect order ([#5772])
- time: do not overflow to signal value ([#5710])
- fs: wait for in-flight ops before cloning `File` ([#5803])

### Changed

- rt: reduce time to poll tasks scheduled from outside the runtime ([#5705], [#5720])

### Added

- net: add uds doc alias for unix sockets ([#5659])
- rt: add metric for number of tasks ([#5628])
- sync: implement more traits for channel errors ([#5666])
- net: add nodelay methods on TcpSocket ([#5672])
- sync: add `broadcast::Receiver::blocking_recv` ([#5690])
- process: add `raw_arg` method to `Command` ([#5704])
- io: support PRIORITY epoll events ([#5566])
- task: add `JoinSet::poll_join_next` ([#5721])
- net: add support for Redox OS ([#5790])

### Unstable

- rt: add the ability to dump task backtraces ([#5608], [#5676], [#5708], [#5717])
- rt: instrument task poll times with a histogram ([#5685])

[#5766]: https://github.com/tokio-rs/tokio/pull/5766
[#5653]: https://github.com/tokio-rs/tokio/pull/5653
[#5686]: https://github.com/tokio-rs/tokio/pull/5686
[#5712]: https://github.com/tokio-rs/tokio/pull/5712
[#5693]: https://github.com/tokio-rs/tokio/pull/5693
[#5772]: https://github.com/tokio-rs/tokio/pull/5772
[#5710]: https://github.com/tokio-rs/tokio/pull/5710
[#5803]: https://github.com/tokio-rs/tokio/pull/5803
[#5705]: https://github.com/tokio-rs/tokio/pull/5705
[#5720]: https://github.com/tokio-rs/tokio/pull/5720
[#5659]: https://github.com/tokio-rs/tokio/pull/5659
[#5628]: https://github.com/tokio-rs/tokio/pull/5628
[#5666]: https://github.com/tokio-rs/tokio/pull/5666
[#5672]: https://github.com/tokio-rs/tokio/pull/5672
[#5690]: https://github.com/tokio-rs/tokio/pull/5690
[#5704]: https://github.com/tokio-rs/tokio/pull/5704
[#5566]: https://github.com/tokio-rs/tokio/pull/5566
[#5721]: https://github.com/tokio-rs/tokio/pull/5721
[#5790]: https://github.com/tokio-rs/tokio/pull/5790
[#5608]: https://github.com/tokio-rs/tokio/pull/5608
[#5676]: https://github.com/tokio-rs/tokio/pull/5676
[#5708]: https://github.com/tokio-rs/tokio/pull/5708
[#5717]: https://github.com/tokio-rs/tokio/pull/5717
[#5685]: https://github.com/tokio-rs/tokio/pull/5685

# 1.28.2 (May 28, 2023)

Forward ports 1.18.6 changes.

### Fixed

- deps: disable default features for mio ([#5728])

[#5728]: https://github.com/tokio-rs/tokio/pull/5728

# 1.28.1 (May 10th, 2023)

This release fixes a mistake in the build script that makes `AsFd`
implementations unavailable on Rust 1.63. ([#5677])

[#5677]: https://github.com/tokio-rs/tokio/pull/5677

# 1.28.0 (April 25th, 2023)

### Added

- io: add `AsyncFd::async_io` ([#5542])
- io: impl BufMut for ReadBuf ([#5590])
- net: add `recv_buf` for `UdpSocket` and `UnixDatagram` ([#5583])
- sync: add `OwnedSemaphorePermit::semaphore` ([#5618])
- sync: add `same_channel` to broadcast channel ([#5607])
- sync: add `watch::Receiver::wait_for` ([#5611])
- task: add `JoinSet::spawn_blocking` and `JoinSet::spawn_blocking_on` ([#5612])

### Changed

- deps: update windows-sys to 0.48 ([#5591])
- io: make `read_to_end` not grow unnecessarily ([#5610])
- macros: make entrypoints more efficient ([#5621])
- sync: improve Debug impl for `RwLock` ([#5647])
- sync: reduce contention in `Notify` ([#5503])

### Fixed

- net: support `get_peer_cred` on AIX ([#5065])
- sync: avoid deadlocks in `broadcast` with custom wakers ([#5578])

### Documented

- sync: fix typo in `Semaphore::MAX_PERMITS` ([#5645])
- sync: fix typo in `tokio::sync::watch::Sender` docs ([#5587])

[#5065]: https://github.com/tokio-rs/tokio/pull/5065
[#5503]: https://github.com/tokio-rs/tokio/pull/5503
[#5542]: https://github.com/tokio-rs/tokio/pull/5542
[#5578]: https://github.com/tokio-rs/tokio/pull/5578
[#5583]: https://github.com/tokio-rs/tokio/pull/5583
[#5587]: https://github.com/tokio-rs/tokio/pull/5587
[#5590]: https://github.com/tokio-rs/tokio/pull/5590
[#5591]: https://github.com/tokio-rs/tokio/pull/5591
[#5607]: https://github.com/tokio-rs/tokio/pull/5607
[#5610]: https://github.com/tokio-rs/tokio/pull/5610
[#5611]: https://github.com/tokio-rs/tokio/pull/5611
[#5612]: https://github.com/tokio-rs/tokio/pull/5612
[#5618]: https://github.com/tokio-rs/tokio/pull/5618
[#5621]: https://github.com/tokio-rs/tokio/pull/5621
[#5645]: https://github.com/tokio-rs/tokio/pull/5645
[#5647]: https://github.com/tokio-rs/tokio/pull/5647

# 1.27.0 (March 27th, 2023)

This release bumps the MSRV of Tokio to 1.56. ([#5559])

### Added

- io: add `async_io` helper method to sockets ([#5512])
- io: add implementations of `AsFd`/`AsHandle`/`AsSocket` ([#5514], [#5540])
- net: add `UdpSocket::peek_sender()` ([#5520])
- sync: add `RwLockWriteGuard::{downgrade_map, try_downgrade_map}` ([#5527])
- task: add `JoinHandle::abort_handle` ([#5543])

### Changed

- io: use `memchr` from `libc` ([#5558])
- macros: accept path as crate rename in `#[tokio::main]` ([#5557])
- macros: update to syn 2.0.0 ([#5572])
- time: don't register for a wakeup when `Interval` returns `Ready` ([#5553])

### Fixed

- fs: fuse std iterator in `ReadDir` ([#5555])
- tracing: fix `spawn_blocking` location fields ([#5573])
- time: clean up redundant check in `Wheel::poll()` ([#5574])

### Documented

- macros: define cancellation safety ([#5525])
- io: add details to docs of `tokio::io::copy[_buf]` ([#5575])
- io: refer to `ReaderStream` and `StreamReader` in module docs ([#5576])

[#5512]: https://github.com/tokio-rs/tokio/pull/5512
[#5514]: https://github.com/tokio-rs/tokio/pull/5514
[#5520]: https://github.com/tokio-rs/tokio/pull/5520
[#5525]: https://github.com/tokio-rs/tokio/pull/5525
[#5527]: https://github.com/tokio-rs/tokio/pull/5527
[#5540]: https://github.com/tokio-rs/tokio/pull/5540
[#5543]: https://github.com/tokio-rs/tokio/pull/5543
[#5553]: https://github.com/tokio-rs/tokio/pull/5553
[#5555]: https://github.com/tokio-rs/tokio/pull/5555
[#5557]: https://github.com/tokio-rs/tokio/pull/5557
[#5558]: https://github.com/tokio-rs/tokio/pull/5558
[#5559]: https://github.com/tokio-rs/tokio/pull/5559
[#5572]: https://github.com/tokio-rs/tokio/pull/5572
[#5573]: https://github.com/tokio-rs/tokio/pull/5573
[#5574]: https://github.com/tokio-rs/tokio/pull/5574
[#5575]: https://github.com/tokio-rs/tokio/pull/5575
[#5576]: https://github.com/tokio-rs/tokio/pull/5576

# 1.26.0 (March 1st, 2023)

### Fixed

- macros: fix empty `join!` and `try_join!` ([#5504])
- sync: don't leak tracing spans in mutex guards ([#5469])
- sync: drop wakers after unlocking the mutex in Notify ([#5471])
- sync: drop wakers outside lock in semaphore ([#5475])

### Added

- fs: add `fs::try_exists` ([#4299])
- net: add types for named unix pipes ([#5351])
- sync: add `MappedOwnedMutexGuard` ([#5474])

### Changed

- chore: update windows-sys to 0.45 ([#5386])
- net: use Message Read Mode for named pipes ([#5350])
- sync: mark lock guards with `#[clippy::has_significant_drop]` ([#5422])
- sync: reduce contention in watch channel ([#5464])
- time: remove cache padding in timer entries ([#5468])
- time: Improve `Instant::now()` perf with test-util ([#5513])

### Internal Changes

- io: use `poll_fn` in `copy_bidirectional` ([#5486])
- net: refactor named pipe builders to not use bitfields ([#5477])
- rt: remove Arc from Clock ([#5434])
- sync: make `notify_waiters` calls atomic ([#5458])
- time: don't store deadline twice in sleep entries ([#5410])

### Unstable

- metrics: add a new metric for budget exhaustion yields ([#5517])

### Documented

- io: improve AsyncFd example ([#5481])
- runtime: document the nature of the main future ([#5494])
- runtime: remove extra period in docs ([#5511])
- signal: updated Documentation for Signals ([#5459])
- sync: add doc aliases for `blocking_*` methods ([#5448])
- sync: fix docs for Send/Sync bounds in broadcast ([#5480])
- sync: document drop behavior for channels ([#5497])
- task: clarify what happens to spawned work during runtime shutdown ([#5394])
- task: clarify `process::Command` docs ([#5413])
- task: fix wording with 'unsend' ([#5452])
- time: document immediate completion guarantee for timeouts ([#5509])
- tokio: document supported platforms ([#5483])

[#4299]: https://github.com/tokio-rs/tokio/pull/4299
[#5350]: https://github.com/tokio-rs/tokio/pull/5350
[#5351]: https://github.com/tokio-rs/tokio/pull/5351
[#5386]: https://github.com/tokio-rs/tokio/pull/5386
[#5394]: https://github.com/tokio-rs/tokio/pull/5394
[#5410]: https://github.com/tokio-rs/tokio/pull/5410
[#5413]: https://github.com/tokio-rs/tokio/pull/5413
[#5422]: https://github.com/tokio-rs/tokio/pull/5422
[#5434]: https://github.com/tokio-rs/tokio/pull/5434
[#5448]: https://github.com/tokio-rs/tokio/pull/5448
[#5452]: https://github.com/tokio-rs/tokio/pull/5452
[#5458]: https://github.com/tokio-rs/tokio/pull/5458
[#5459]: https://github.com/tokio-rs/tokio/pull/5459
[#5464]: https://github.com/tokio-rs/tokio/pull/5464
[#5468]: https://github.com/tokio-rs/tokio/pull/5468
[#5469]: https://github.com/tokio-rs/tokio/pull/5469
[#5471]: https://github.com/tokio-rs/tokio/pull/5471
[#5474]: https://github.com/tokio-rs/tokio/pull/5474
[#5475]: https://github.com/tokio-rs/tokio/pull/5475
[#5477]: https://github.com/tokio-rs/tokio/pull/5477
[#5480]: https://github.com/tokio-rs/tokio/pull/5480
[#5481]: https://github.com/tokio-rs/tokio/pull/5481
[#5483]: https://github.com/tokio-rs/tokio/pull/5483
[#5486]: https://github.com/tokio-rs/tokio/pull/5486
[#5494]: https://github.com/tokio-rs/tokio/pull/5494
[#5497]: https://github.com/tokio-rs/tokio/pull/5497
[#5504]: https://github.com/tokio-rs/tokio/pull/5504
[#5509]: https://github.com/tokio-rs/tokio/pull/5509
[#5511]: https://github.com/tokio-rs/tokio/pull/5511
[#5513]: https://github.com/tokio-rs/tokio/pull/5513
[#5517]: https://github.com/tokio-rs/tokio/pull/5517

# 1.25.3 (December 17th, 2023)

### Fixed

- io: add budgeting to `tokio::runtime::io::registration::async_io` ([#6221])

[#6221]: https://github.com/tokio-rs/tokio/pull/6221

# 1.25.2 (September 22, 2023)

Forward ports 1.20.6 changes.

### Changed

- io: use `memchr` from `libc` ([#5960])

[#5960]: https://github.com/tokio-rs/tokio/pull/5960

# 1.25.1 (May 28, 2023)

Forward ports 1.18.6 changes.

### Fixed

- deps: disable default features for mio ([#5728])

[#5728]: https://github.com/tokio-rs/tokio/pull/5728

# 1.25.0 (January 28, 2023)

### Fixed

- rt: fix runtime metrics reporting ([#5330])

### Added

- sync: add `broadcast::Sender::len` ([#5343])

### Changed

- fs: increase maximum read buffer size to 2MiB ([#5397])

[#5330]: https://github.com/tokio-rs/tokio/pull/5330
[#5343]: https://github.com/tokio-rs/tokio/pull/5343
[#5397]: https://github.com/tokio-rs/tokio/pull/5397

# 1.24.2 (January 17, 2023)

Forward ports 1.18.5 changes.

### Fixed

- io: fix unsoundness in `ReadHalf::unsplit` ([#5375])

[#5375]: https://github.com/tokio-rs/tokio/pull/5375

# 1.24.1 (January 6, 2022)

This release fixes a compilation failure on targets without `AtomicU64` when using rustc older than 1.63. ([#5356])

[#5356]: https://github.com/tokio-rs/tokio/pull/5356

# 1.24.0 (January 5, 2022)

### Fixed

- rt: improve native `AtomicU64` support detection ([#5284])

### Added

- rt: add configuration option for max number of I/O events polled from the OS
  per tick ([#5186])
- rt: add an environment variable for configuring the default number of worker
  threads per runtime instance ([#4250])

### Changed

- sync: reduce MPSC channel stack usage ([#5294])
- io: reduce lock contention in I/O operations ([#5300])
- fs: speed up `read_dir()` by chunking operations ([#5309])
- rt: use internal `ThreadId` implementation ([#5329])
- test: don't auto-advance time when a `spawn_blocking` task is running ([#5115])

[#5186]: https://github.com/tokio-rs/tokio/pull/5186
[#5294]: https://github.com/tokio-rs/tokio/pull/5294
[#5284]: https://github.com/tokio-rs/tokio/pull/5284
[#4250]: https://github.com/tokio-rs/tokio/pull/4250
[#5300]: https://github.com/tokio-rs/tokio/pull/5300
[#5329]: https://github.com/tokio-rs/tokio/pull/5329
[#5115]: https://github.com/tokio-rs/tokio/pull/5115
[#5309]: https://github.com/tokio-rs/tokio/pull/5309

# 1.23.1 (January 4, 2022)

This release forward ports changes from 1.18.4.

### Fixed

- net: fix Windows named pipe server builder to maintain option when toggling
  pipe mode ([#5336]).

[#5336]: https://github.com/tokio-rs/tokio/pull/5336

# 1.23.0 (December 5, 2022)

### Fixed

- net: fix Windows named pipe connect ([#5208])
- io: support vectored writes for `ChildStdin` ([#5216])
- io: fix `async fn ready()` false positive for OS-specific events ([#5231])

### Changed

- runtime: `yield_now` defers task until after driver poll ([#5223])
- runtime: reduce amount of codegen needed per spawned task ([#5213])
- windows: replace `winapi` dependency with `windows-sys` ([#5204])

[#5208]: https://github.com/tokio-rs/tokio/pull/5208
[#5216]: https://github.com/tokio-rs/tokio/pull/5216
[#5213]: https://github.com/tokio-rs/tokio/pull/5213
[#5204]: https://github.com/tokio-rs/tokio/pull/5204
[#5223]: https://github.com/tokio-rs/tokio/pull/5223
[#5231]: https://github.com/tokio-rs/tokio/pull/5231

# 1.22.0 (November 17, 2022)

### Added

- runtime: add `Handle::runtime_flavor` ([#5138])
- sync: add `Mutex::blocking_lock_owned` ([#5130])
- sync: add `Semaphore::MAX_PERMITS` ([#5144])
- sync: add `merge()` to semaphore permits ([#4948])
- sync: add `mpsc::WeakUnboundedSender` ([#5189])

### Added (unstable)

- process: add `Command::process_group` ([#5114])
- runtime: export metrics about the blocking thread pool ([#5161])
- task: add `task::id()` and `task::try_id()` ([#5171])

### Fixed

- macros: don't take ownership of futures in macros ([#5087])
- runtime: fix Stacked Borrows violation in `LocalOwnedTasks` ([#5099])
- runtime: mitigate ABA with 32-bit queue indices when possible ([#5042])
- task: wake local tasks to the local queue when woken by the same thread ([#5095])
- time: panic in release mode when `mark_pending` called illegally ([#5093])
- runtime: fix typo in expect message ([#5169])
- runtime: fix `unsync_load` on atomic types ([#5175])
- task: elaborate safety comments in task deallocation ([#5172])
- runtime: fix `LocalSet` drop in thread local ([#5179])
- net: remove libc type leakage in a public API ([#5191])
- runtime: update the alignment of `CachePadded` ([#5106])

### Changed

- io: make `tokio::io::copy` continue filling the buffer when writer stalls ([#5066])
- runtime: remove `coop::budget` from `LocalSet::run_until` ([#5155])
- sync: make `Notify` panic safe ([#5154])

### Documented

- io: fix doc for `write_i8` to use signed integers ([#5040])
- net: fix doc typos for TCP and UDP `set_tos` methods ([#5073])
- net: fix function name in `UdpSocket::recv` documentation ([#5150])
- sync: typo in `TryLockError` for `RwLock::try_write` ([#5160])
- task: document that spawned tasks execute immediately ([#5117])
- time: document return type of `timeout` ([#5118])
- time: document that `timeout` checks only before poll ([#5126])
- sync: specify return type of `oneshot::Receiver` in docs ([#5198])

### Internal changes

- runtime: use const `Mutex::new` for globals ([#5061])
- runtime: remove `Option` around `mio::Events` in io driver ([#5078])
- runtime: remove a conditional compilation clause ([#5104])
- runtime: remove a reference to internal time handle ([#5107])
- runtime: misc time driver cleanup ([#5120])
- runtime: move signal driver to runtime module ([#5121])
- runtime: signal driver now uses I/O driver directly ([#5125])
- runtime: start decoupling I/O driver and I/O handle ([#5127])
- runtime: switch `io::handle` refs with scheduler:Handle ([#5128])
- runtime: remove Arc from I/O driver ([#5134])
- runtime: use signal driver handle via `scheduler::Handle` ([#5135])
- runtime: move internal clock fns out of context ([#5139])
- runtime: remove `runtime::context` module ([#5140])
- runtime: keep driver cfgs in `driver.rs` ([#5141])
- runtime: add `runtime::context` to unify thread-locals ([#5143])
- runtime: rename some confusing internal variables/fns ([#5151])
- runtime: move `coop` mod into `runtime` ([#5152])
- runtime: move budget state to context thread-local ([#5157])
- runtime: move park logic into runtime module ([#5158])
- runtime: move `Runtime` into its own file ([#5159])
- runtime: unify entering a runtime with `Handle::enter` ([#5163])
- runtime: remove handle reference from each scheduler ([#5166])
- runtime: move `enter` into `context` ([#5167])
- runtime: combine context and entered thread-locals ([#5168])
- runtime: fix accidental unsetting of current handle ([#5178])
- runtime: move `CoreStage` methods to `Core` ([#5182])
- sync: name mpsc semaphore types ([#5146])

[#4948]: https://github.com/tokio-rs/tokio/pull/4948
[#5040]: https://github.com/tokio-rs/tokio/pull/5040
[#5042]: https://github.com/tokio-rs/tokio/pull/5042
[#5061]: https://github.com/tokio-rs/tokio/pull/5061
[#5066]: https://github.com/tokio-rs/tokio/pull/5066
[#5073]: https://github.com/tokio-rs/tokio/pull/5073
[#5078]: https://github.com/tokio-rs/tokio/pull/5078
[#5087]: https://github.com/tokio-rs/tokio/pull/5087
[#5093]: https://github.com/tokio-rs/tokio/pull/5093
[#5095]: https://github.com/tokio-rs/tokio/pull/5095
[#5099]: https://github.com/tokio-rs/tokio/pull/5099
[#5104]: https://github.com/tokio-rs/tokio/pull/5104
[#5106]: https://github.com/tokio-rs/tokio/pull/5106
[#5107]: https://github.com/tokio-rs/tokio/pull/5107
[#5114]: https://github.com/tokio-rs/tokio/pull/5114
[#5117]: https://github.com/tokio-rs/tokio/pull/5117
[#5118]: https://github.com/tokio-rs/tokio/pull/5118
[#5120]: https://github.com/tokio-rs/tokio/pull/5120
[#5121]: https://github.com/tokio-rs/tokio/pull/5121
[#5125]: https://github.com/tokio-rs/tokio/pull/5125
[#5126]: https://github.com/tokio-rs/tokio/pull/5126
[#5127]: https://github.com/tokio-rs/tokio/pull/5127
[#5128]: https://github.com/tokio-rs/tokio/pull/5128
[#5130]: https://github.com/tokio-rs/tokio/pull/5130
[#5134]: https://github.com/tokio-rs/tokio/pull/5134
[#5135]: https://github.com/tokio-rs/tokio/pull/5135
[#5138]: https://github.com/tokio-rs/tokio/pull/5138
[#5138]: https://github.com/tokio-rs/tokio/pull/5138
[#5139]: https://github.com/tokio-rs/tokio/pull/5139
[#5140]: https://github.com/tokio-rs/tokio/pull/5140
[#5141]: https://github.com/tokio-rs/tokio/pull/5141
[#5143]: https://github.com/tokio-rs/tokio/pull/5143
[#5144]: https://github.com/tokio-rs/tokio/pull/5144
[#5144]: https://github.com/tokio-rs/tokio/pull/5144
[#5146]: https://github.com/tokio-rs/tokio/pull/5146
[#5150]: https://github.com/tokio-rs/tokio/pull/5150
[#5151]: https://github.com/tokio-rs/tokio/pull/5151
[#5152]: https://github.com/tokio-rs/tokio/pull/5152
[#5154]: https://github.com/tokio-rs/tokio/pull/5154
[#5155]: https://github.com/tokio-rs/tokio/pull/5155
[#5157]: https://github.com/tokio-rs/tokio/pull/5157
[#5158]: https://github.com/tokio-rs/tokio/pull/5158
[#5159]: https://github.com/tokio-rs/tokio/pull/5159
[#5160]: https://github.com/tokio-rs/tokio/pull/5160
[#5161]: https://github.com/tokio-rs/tokio/pull/5161
[#5163]: https://github.com/tokio-rs/tokio/pull/5163
[#5166]: https://github.com/tokio-rs/tokio/pull/5166
[#5167]: https://github.com/tokio-rs/tokio/pull/5167
[#5168]: https://github.com/tokio-rs/tokio/pull/5168
[#5169]: https://github.com/tokio-rs/tokio/pull/5169
[#5171]: https://github.com/tokio-rs/tokio/pull/5171
[#5172]: https://github.com/tokio-rs/tokio/pull/5172
[#5175]: https://github.com/tokio-rs/tokio/pull/5175
[#5178]: https://github.com/tokio-rs/tokio/pull/5178
[#5179]: https://github.com/tokio-rs/tokio/pull/5179
[#5182]: https://github.com/tokio-rs/tokio/pull/5182
[#5189]: https://github.com/tokio-rs/tokio/pull/5189
[#5191]: https://github.com/tokio-rs/tokio/pull/5191
[#5198]: https://github.com/tokio-rs/tokio/pull/5198

# 1.21.2 (September 27, 2022)

This release removes the dependency on the `once_cell` crate to restore the MSRV
of 1.21.x, which is the latest minor version at the time of release. ([#5048])

[#5048]: https://github.com/tokio-rs/tokio/pull/5048

# 1.21.1 (September 13, 2022)

### Fixed

- net: fix dependency resolution for socket2 ([#5000])
- task: ignore failure to set TLS in `LocalSet` Drop ([#4976])

[#4976]: https://github.com/tokio-rs/tokio/pull/4976
[#5000]: https://github.com/tokio-rs/tokio/pull/5000

# 1.21.0 (September 2, 2022)

This release is the first release of Tokio to intentionally support WASM. The
`sync,macros,io-util,rt,time` features are stabilized on WASM. Additionally the
wasm32-wasi target is given unstable support for the `net` feature.

### Added

- net: add `device` and `bind_device` methods to TCP/UDP sockets ([#4882])
- net: add `tos` and `set_tos` methods to TCP and UDP sockets ([#4877])
- net: add security flags to named pipe `ServerOptions` ([#4845])
- signal: add more windows signal handlers ([#4924])
- sync: add `mpsc::Sender::max_capacity` method ([#4904])
- sync: implement Weak version of `mpsc::Sender` ([#4595])
- task: add `LocalSet::enter` ([#4765])
- task: stabilize `JoinSet` and `AbortHandle` ([#4920])
- tokio: add `track_caller` to public APIs ([#4805], [#4848], [#4852])
- wasm: initial support for `wasm32-wasi` target ([#4716])

### Fixed

- miri: improve miri compatibility by avoiding temporary references in `linked_list::Link` impls ([#4841])
- signal: don't register write interest on signal pipe ([#4898])
- sync: add `#[must_use]` to lock guards ([#4886])
- sync: fix hang when calling `recv` on closed and reopened broadcast channel ([#4867])
- task: propagate attributes on task-locals ([#4837])

### Changed

- fs: change panic to error in `File::start_seek` ([#4897])
- io: reduce syscalls in `poll_read` ([#4840])
- process: use blocking threadpool for child stdio I/O ([#4824])
- signal: make `SignalKind` methods const ([#4956])

### Internal changes

- rt: extract `basic_scheduler::Config` ([#4935])
- rt: move I/O driver into `runtime` module ([#4942])
- rt: rename internal scheduler types ([#4945])

### Documented

- chore: fix typos and grammar ([#4858], [#4894], [#4928])
- io: fix typo in `AsyncSeekExt::rewind` docs ([#4893])
- net: add documentation to `try_read()` for zero-length buffers ([#4937])
- runtime: remove incorrect panic section for `Builder::worker_threads` ([#4849])
- sync: doc of `watch::Sender::send` improved ([#4959])
- task: add cancel safety docs to `JoinHandle` ([#4901])
- task: expand on cancellation of `spawn_blocking` ([#4811])
- time: clarify that the first tick of `Interval::tick` happens immediately ([#4951])

### Unstable

- rt: add unstable option to disable the LIFO slot ([#4936])
- task: fix incorrect signature in `Builder::spawn_on` ([#4953])
- task: make `task::Builder::spawn*` methods fallible ([#4823])

[#4595]: https://github.com/tokio-rs/tokio/pull/4595
[#4716]: https://github.com/tokio-rs/tokio/pull/4716
[#4765]: https://github.com/tokio-rs/tokio/pull/4765
[#4805]: https://github.com/tokio-rs/tokio/pull/4805
[#4811]: https://github.com/tokio-rs/tokio/pull/4811
[#4823]: https://github.com/tokio-rs/tokio/pull/4823
[#4824]: https://github.com/tokio-rs/tokio/pull/4824
[#4837]: https://github.com/tokio-rs/tokio/pull/4837
[#4840]: https://github.com/tokio-rs/tokio/pull/4840
[#4841]: https://github.com/tokio-rs/tokio/pull/4841
[#4845]: https://github.com/tokio-rs/tokio/pull/4845
[#4848]: https://github.com/tokio-rs/tokio/pull/4848
[#4849]: https://github.com/tokio-rs/tokio/pull/4849
[#4852]: https://github.com/tokio-rs/tokio/pull/4852
[#4858]: https://github.com/tokio-rs/tokio/pull/4858
[#4867]: https://github.com/tokio-rs/tokio/pull/4867
[#4877]: https://github.com/tokio-rs/tokio/pull/4877
[#4882]: https://github.com/tokio-rs/tokio/pull/4882
[#4886]: https://github.com/tokio-rs/tokio/pull/4886
[#4893]: https://github.com/tokio-rs/tokio/pull/4893
[#4894]: https://github.com/tokio-rs/tokio/pull/4894
[#4897]: https://github.com/tokio-rs/tokio/pull/4897
[#4898]: https://github.com/tokio-rs/tokio/pull/4898
[#4901]: https://github.com/tokio-rs/tokio/pull/4901
[#4904]: https://github.com/tokio-rs/tokio/pull/4904
[#4920]: https://github.com/tokio-rs/tokio/pull/4920
[#4924]: https://github.com/tokio-rs/tokio/pull/4924
[#4928]: https://github.com/tokio-rs/tokio/pull/4928
[#4935]: https://github.com/tokio-rs/tokio/pull/4935
[#4936]: https://github.com/tokio-rs/tokio/pull/4936
[#4937]: https://github.com/tokio-rs/tokio/pull/4937
[#4942]: https://github.com/tokio-rs/tokio/pull/4942
[#4945]: https://github.com/tokio-rs/tokio/pull/4945
[#4951]: https://github.com/tokio-rs/tokio/pull/4951
[#4953]: https://github.com/tokio-rs/tokio/pull/4953
[#4956]: https://github.com/tokio-rs/tokio/pull/4956
[#4959]: https://github.com/tokio-rs/tokio/pull/4959

# 1.20.6 (September 22, 2023)

This is a backport of a change from 1.27.0.

### Changed

- io: use `memchr` from `libc` ([#5960])

[#5960]: https://github.com/tokio-rs/tokio/pull/5960

# 1.20.5 (May 28, 2023)

Forward ports 1.18.6 changes.

### Fixed

- deps: disable default features for mio ([#5728])

[#5728]: https://github.com/tokio-rs/tokio/pull/5728

# 1.20.4 (January 17, 2023)

Forward ports 1.18.5 changes.

### Fixed

- io: fix unsoundness in `ReadHalf::unsplit` ([#5375])

[#5375]: https://github.com/tokio-rs/tokio/pull/5375

# 1.20.3 (January 3, 2022)

This release forward ports changes from 1.18.4.

### Fixed

- net: fix Windows named pipe server builder to maintain option when toggling
  pipe mode ([#5336]).

[#5336]: https://github.com/tokio-rs/tokio/pull/5336

# 1.20.2 (September 27, 2022)

This release removes the dependency on the `once_cell` crate to restore the MSRV
of the 1.20.x LTS release. ([#5048])

[#5048]: https://github.com/tokio-rs/tokio/pull/5048

# 1.20.1 (July 25, 2022)

### Fixed

- chore: fix version detection in build script ([#4860])

[#4860]: https://github.com/tokio-rs/tokio/pull/4860

# 1.20.0 (July 12, 2022)

### Added

- tokio: add `track_caller` to public APIs ([#4772], [#4791], [#4793], [#4806], [#4808])
- sync: Add `has_changed` method to `watch::Ref` ([#4758])

### Changed

- time: remove `src/time/driver/wheel/stack.rs` ([#4766])
- rt: clean up arguments passed to basic scheduler ([#4767])
- net: be more specific about winapi features ([#4764])
- tokio: use const initialized thread locals where possible ([#4677])
- task: various small improvements to LocalKey ([#4795])

### Documented

- fs: warn about performance pitfall ([#4762])
- chore: fix spelling ([#4769])
- sync: document spurious failures in oneshot ([#4777])
- sync: add warning for watch in non-Send futures ([#4741])
- chore: fix typo ([#4798])

### Unstable

- joinset: rename `join_one` to `join_next` ([#4755])
- rt: unhandled panic config for current thread rt ([#4770])

[#4677]: https://github.com/tokio-rs/tokio/pull/4677
[#4741]: https://github.com/tokio-rs/tokio/pull/4741
[#4755]: https://github.com/tokio-rs/tokio/pull/4755
[#4758]: https://github.com/tokio-rs/tokio/pull/4758
[#4762]: https://github.com/tokio-rs/tokio/pull/4762
[#4764]: https://github.com/tokio-rs/tokio/pull/4764
[#4766]: https://github.com/tokio-rs/tokio/pull/4766
[#4767]: https://github.com/tokio-rs/tokio/pull/4767
[#4769]: https://github.com/tokio-rs/tokio/pull/4769
[#4770]: https://github.com/tokio-rs/tokio/pull/4770
[#4772]: https://github.com/tokio-rs/tokio/pull/4772
[#4777]: https://github.com/tokio-rs/tokio/pull/4777
[#4791]: https://github.com/tokio-rs/tokio/pull/4791
[#4793]: https://github.com/tokio-rs/tokio/pull/4793
[#4795]: https://github.com/tokio-rs/tokio/pull/4795
[#4798]: https://github.com/tokio-rs/tokio/pull/4798
[#4806]: https://github.com/tokio-rs/tokio/pull/4806
[#4808]: https://github.com/tokio-rs/tokio/pull/4808

# 1.19.2 (June 6, 2022)

This release fixes another bug in `Notified::enable`. ([#4751])

[#4751]: https://github.com/tokio-rs/tokio/pull/4751

# 1.19.1 (June 5, 2022)

This release fixes a bug in `Notified::enable`. ([#4747])

[#4747]: https://github.com/tokio-rs/tokio/pull/4747

# 1.19.0 (June 3, 2022)

### Added

- runtime: add `is_finished` method for `JoinHandle` and `AbortHandle` ([#4709])
- runtime: make global queue and event polling intervals configurable ([#4671])
- sync: add `Notified::enable` ([#4705])
- sync: add `watch::Sender::send_if_modified` ([#4591])
- sync: add resubscribe method to broadcast::Receiver ([#4607])
- net: add `take_error` to `TcpSocket` and `TcpStream` ([#4739])

### Changed

- io: refactor out usage of Weak in the io handle ([#4656])

### Fixed

- macros: avoid starvation in `join!` and `try_join!` ([#4624])

### Documented

- runtime: clarify semantics of tasks outliving `block_on` ([#4729])
- time: fix example for `MissedTickBehavior::Burst` ([#4713])

### Unstable

- metrics: correctly update atomics in `IoDriverMetrics` ([#4725])
- metrics: fix compilation with unstable, process, and rt, but without net ([#4682])
- task: add `#[track_caller]` to `JoinSet`/`JoinMap` ([#4697])
- task: add `Builder::{spawn_on, spawn_local_on, spawn_blocking_on}` ([#4683])
- task: add `consume_budget` for cooperative scheduling ([#4498])
- task: add `join_set::Builder` for configuring `JoinSet` tasks ([#4687])
- task: update return value of `JoinSet::join_one` ([#4726])

[#4498]: https://github.com/tokio-rs/tokio/pull/4498
[#4591]: https://github.com/tokio-rs/tokio/pull/4591
[#4607]: https://github.com/tokio-rs/tokio/pull/4607
[#4624]: https://github.com/tokio-rs/tokio/pull/4624
[#4656]: https://github.com/tokio-rs/tokio/pull/4656
[#4671]: https://github.com/tokio-rs/tokio/pull/4671
[#4682]: https://github.com/tokio-rs/tokio/pull/4682
[#4683]: https://github.com/tokio-rs/tokio/pull/4683
[#4687]: https://github.com/tokio-rs/tokio/pull/4687
[#4697]: https://github.com/tokio-rs/tokio/pull/4697
[#4705]: https://github.com/tokio-rs/tokio/pull/4705
[#4709]: https://github.com/tokio-rs/tokio/pull/4709
[#4713]: https://github.com/tokio-rs/tokio/pull/4713
[#4725]: https://github.com/tokio-rs/tokio/pull/4725
[#4726]: https://github.com/tokio-rs/tokio/pull/4726
[#4729]: https://github.com/tokio-rs/tokio/pull/4729
[#4739]: https://github.com/tokio-rs/tokio/pull/4739

# 1.18.6 (May 28, 2023)

### Fixed

- deps: disable default features for mio ([#5728])

[#5728]: https://github.com/tokio-rs/tokio/pull/5728

# 1.18.5 (January 17, 2023)

### Fixed

- io: fix unsoundness in `ReadHalf::unsplit` ([#5375])

[#5375]: https://github.com/tokio-rs/tokio/pull/5375

# 1.18.4 (January 3, 2022)

### Fixed

- net: fix Windows named pipe server builder to maintain option when toggling
  pipe mode ([#5336]).

[#5336]: https://github.com/tokio-rs/tokio/pull/5336

# 1.18.3 (September 27, 2022)

This release removes the dependency on the `once_cell` crate to restore the MSRV
of the 1.18.x LTS release. ([#5048])

[#5048]: https://github.com/tokio-rs/tokio/pull/5048

# 1.18.2 (May 5, 2022)

Add missing features for the `winapi` dependency. ([#4663])

[#4663]: https://github.com/tokio-rs/tokio/pull/4663

# 1.18.1 (May 2, 2022)

The 1.18.0 release broke the build for targets without 64-bit atomics when
building with `tokio_unstable`. This release fixes that. ([#4649])

[#4649]: https://github.com/tokio-rs/tokio/pull/4649

# 1.18.0 (April 27, 2022)

This release adds a number of new APIs in `tokio::net`, `tokio::signal`, and
`tokio::sync`. In addition, it adds new unstable APIs to `tokio::task` (`Id`s
for uniquely identifying a task, and `AbortHandle` for remotely cancelling a
task), as well as a number of bugfixes.

### Fixed

- blocking: add missing `#[track_caller]` for `spawn_blocking` ([#4616])
- macros: fix `select` macro to process 64 branches ([#4519])
- net: fix `try_io` methods not calling Mio's `try_io` internally ([#4582])
- runtime: recover when OS fails to spawn a new thread ([#4485])

### Added

- net: add `UdpSocket::peer_addr` ([#4611])
- net: add `try_read_buf` method for named pipes ([#4626])
- signal: add `SignalKind` `Hash`/`Eq` impls and `c_int` conversion ([#4540])
- signal: add support for signals up to `SIGRTMAX` ([#4555])
- sync: add `watch::Sender::send_modify` method ([#4310])
- sync: add `broadcast::Receiver::len` method ([#4542])
- sync: add `watch::Receiver::same_channel` method ([#4581])
- sync: implement `Clone` for `RecvError` types ([#4560])

### Changed

- update `mio` to 0.8.1 ([#4582])
- macros: rename `tokio::select!`'s internal `util` module ([#4543])
- runtime: use `Vec::with_capacity` when building runtime ([#4553])

### Documented

- improve docs for `tokio_unstable` ([#4524])
- runtime: include more documentation for thread_pool/worker ([#4511])
- runtime: update `Handle::current`'s docs to mention `EnterGuard` ([#4567])
- time: clarify platform specific timer resolution ([#4474])
- signal: document that `Signal::recv` is cancel-safe ([#4634])
- sync: `UnboundedReceiver` close docs ([#4548])

### Unstable

The following changes only apply when building with `--cfg tokio_unstable`:

- task: add `task::Id` type ([#4630])
- task: add `AbortHandle` type for cancelling tasks in a `JoinSet` ([#4530],
  [#4640])
- task: fix missing `doc(cfg(...))` attributes for `JoinSet` ([#4531])
- task: fix broken link in `AbortHandle` RustDoc ([#4545])
- metrics: add initial IO driver metrics ([#4507])

[#4616]: https://github.com/tokio-rs/tokio/pull/4616
[#4519]: https://github.com/tokio-rs/tokio/pull/4519
[#4582]: https://github.com/tokio-rs/tokio/pull/4582
[#4485]: https://github.com/tokio-rs/tokio/pull/4485
[#4613]: https://github.com/tokio-rs/tokio/pull/4613
[#4611]: https://github.com/tokio-rs/tokio/pull/4611
[#4626]: https://github.com/tokio-rs/tokio/pull/4626
[#4540]: https://github.com/tokio-rs/tokio/pull/4540
[#4555]: https://github.com/tokio-rs/tokio/pull/4555
[#4310]: https://github.com/tokio-rs/tokio/pull/4310
[#4542]: https://github.com/tokio-rs/tokio/pull/4542
[#4581]: https://github.com/tokio-rs/tokio/pull/4581
[#4560]: https://github.com/tokio-rs/tokio/pull/4560
[#4631]: https://github.com/tokio-rs/tokio/pull/4631
[#4582]: https://github.com/tokio-rs/tokio/pull/4582
[#4543]: https://github.com/tokio-rs/tokio/pull/4543
[#4553]: https://github.com/tokio-rs/tokio/pull/4553
[#4524]: https://github.com/tokio-rs/tokio/pull/4524
[#4511]: https://github.com/tokio-rs/tokio/pull/4511
[#4567]: https://github.com/tokio-rs/tokio/pull/4567
[#4474]: https://github.com/tokio-rs/tokio/pull/4474
[#4634]: https://github.com/tokio-rs/tokio/pull/4634
[#4548]: https://github.com/tokio-rs/tokio/pull/4548
[#4630]: https://github.com/tokio-rs/tokio/pull/4630
[#4530]: https://github.com/tokio-rs/tokio/pull/4530
[#4640]: https://github.com/tokio-rs/tokio/pull/4640
[#4531]: https://github.com/tokio-rs/tokio/pull/4531
[#4545]: https://github.com/tokio-rs/tokio/pull/4545
[#4507]: https://github.com/tokio-rs/tokio/pull/4507

# 1.17.0 (February 16, 2022)

This release updates the minimum supported Rust version (MSRV) to 1.49, the
`mio` dependency to v0.8, and the (optional) `parking_lot` dependency to v0.12.
Additionally, it contains several bug fixes, as well as internal refactoring and
performance improvements.

### Fixed

- time: prevent panicking in `sleep` with large durations ([#4495])
- time: eliminate potential panics in `Instant` arithmetic on platforms where
  `Instant::now` is not monotonic ([#4461])
- io: fix `DuplexStream` not participating in cooperative yielding ([#4478])
- rt: fix potential double panic when dropping a `JoinHandle` ([#4430])

### Changed

- update minimum supported Rust version to 1.49 ([#4457])
- update `parking_lot` dependency to v0.12.0 ([#4459])
- update `mio` dependency to v0.8 ([#4449])
- rt: remove an unnecessary lock in the blocking pool ([#4436])
- rt: remove an unnecessary enum in the basic scheduler ([#4462])
- time: use bit manipulation instead of modulo to improve performance ([#4480])
- net: use `std::future::Ready` instead of our own `Ready` future ([#4271])
- replace deprecated `atomic::spin_loop_hint` with `hint::spin_loop` ([#4491])
- fix miri failures in intrusive linked lists ([#4397])

### Documented

- io: add an example for `tokio::process::ChildStdin` ([#4479])

### Unstable

The following changes only apply when building with `--cfg tokio_unstable`:

- task: fix missing location information in `tracing` spans generated by
  `spawn_local` ([#4483])
- task: add `JoinSet` for managing sets of tasks ([#4335])
- metrics: fix compilation error on MIPS ([#4475])
- metrics: fix compilation error on arm32v7 ([#4453])

[#4495]: https://github.com/tokio-rs/tokio/pull/4495
[#4461]: https://github.com/tokio-rs/tokio/pull/4461
[#4478]: https://github.com/tokio-rs/tokio/pull/4478
[#4430]: https://github.com/tokio-rs/tokio/pull/4430
[#4457]: https://github.com/tokio-rs/tokio/pull/4457
[#4459]: https://github.com/tokio-rs/tokio/pull/4459
[#4449]: https://github.com/tokio-rs/tokio/pull/4449
[#4462]: https://github.com/tokio-rs/tokio/pull/4462
[#4436]: https://github.com/tokio-rs/tokio/pull/4436
[#4480]: https://github.com/tokio-rs/tokio/pull/4480
[#4271]: https://github.com/tokio-rs/tokio/pull/4271
[#4491]: https://github.com/tokio-rs/tokio/pull/4491
[#4397]: https://github.com/tokio-rs/tokio/pull/4397
[#4479]: https://github.com/tokio-rs/tokio/pull/4479
[#4483]: https://github.com/tokio-rs/tokio/pull/4483
[#4335]: https://github.com/tokio-rs/tokio/pull/4335
[#4475]: https://github.com/tokio-rs/tokio/pull/4475
[#4453]: https://github.com/tokio-rs/tokio/pull/4453

# 1.16.1 (January 28, 2022)

This release fixes a bug in [#4428] with the change [#4437].

[#4428]: https://github.com/tokio-rs/tokio/pull/4428
[#4437]: https://github.com/tokio-rs/tokio/pull/4437

# 1.16.0 (January 27, 2022)

Fixes a soundness bug in `io::Take` ([#4428]). The unsoundness is exposed when
leaking memory in the given `AsyncRead` implementation and then overwriting the
supplied buffer:

```rust
impl AsyncRead for Buggy {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>
    ) -> Poll<Result<()>> {
      let new_buf = vec![0; 5].leak();
      *buf = ReadBuf::new(new_buf);
      buf.put_slice(b"hello");
      Poll::Ready(Ok(()))
    }
}
```

Also, this release includes improvements to the multi-threaded scheduler that
can increase throughput by up to 20% in some cases ([#4383]).

### Fixed

- io: **soundness** don't expose uninitialized memory when using `io::Take` in edge case ([#4428])
- fs: ensure `File::write` results in a `write` syscall when the runtime shuts down ([#4316])
- process: drop pipe after child exits in `wait_with_output` ([#4315])
- rt: improve error message when spawning a thread fails ([#4398])
- rt: reduce false-positive thread wakups in the multi-threaded scheduler ([#4383])
- sync: don't inherit `Send` from `parking_lot::*Guard` ([#4359])

### Added

- net: `TcpSocket::linger()` and `set_linger()` ([#4324])
- net: impl `UnwindSafe` for socket types ([#4384])
- rt: impl `UnwindSafe` for `JoinHandle` ([#4418])
- sync: `watch::Receiver::has_changed()` ([#4342])
- sync: `oneshot::Receiver::blocking_recv()` ([#4334])
- sync: `RwLock` blocking operations ([#4425])

### Unstable

The following changes only apply when building with `--cfg tokio_unstable`

- rt: **breaking change** overhaul runtime metrics API ([#4373])

[#4428]: https://github.com/tokio-rs/tokio/pull/4428
[#4316]: https://github.com/tokio-rs/tokio/pull/4316
[#4315]: https://github.com/tokio-rs/tokio/pull/4315
[#4398]: https://github.com/tokio-rs/tokio/pull/4398
[#4383]: https://github.com/tokio-rs/tokio/pull/4383
[#4359]: https://github.com/tokio-rs/tokio/pull/4359
[#4324]: https://github.com/tokio-rs/tokio/pull/4324
[#4384]: https://github.com/tokio-rs/tokio/pull/4384
[#4418]: https://github.com/tokio-rs/tokio/pull/4418
[#4342]: https://github.com/tokio-rs/tokio/pull/4342
[#4334]: https://github.com/tokio-rs/tokio/pull/4334
[#4425]: https://github.com/tokio-rs/tokio/pull/4425
[#4373]: https://github.com/tokio-rs/tokio/pull/4373

# 1.15.0 (December 15, 2021)

### Fixed

- io: add cooperative yielding support to `io::empty()` ([#4300])
- time: make timeout robust against budget-depleting tasks ([#4314])

### Changed

- update minimum supported Rust version to 1.46.

### Added

- time: add `Interval::reset()` ([#4248])
- io: add explicit lifetimes to `AsyncFdReadyGuard` ([#4267])
- process: add `Command::as_std()` ([#4295])

### Added (unstable)

- tracing: instrument `tokio::sync` types ([#4302])

[#4302]: https://github.com/tokio-rs/tokio/pull/4302
[#4300]: https://github.com/tokio-rs/tokio/pull/4300
[#4295]: https://github.com/tokio-rs/tokio/pull/4295
[#4267]: https://github.com/tokio-rs/tokio/pull/4267
[#4248]: https://github.com/tokio-rs/tokio/pull/4248
[#4314]: https://github.com/tokio-rs/tokio/pull/4314

# 1.14.0 (November 15, 2021)

### Fixed

- macros: fix compiler errors when using `mut` patterns in `select!` ([#4211])
- sync: fix a data race between `oneshot::Sender::send` and awaiting a
  `oneshot::Receiver` when the oneshot has been closed ([#4226])
- sync: make `AtomicWaker` panic safe ([#3689])
- runtime: fix basic scheduler dropping tasks outside a runtime context
  ([#4213])

### Added

- stats: add `RuntimeStats::busy_duration_total` ([#4179], [#4223])

### Changed

- io: updated `copy` buffer size to match `std::io::copy` ([#4209])

### Documented

- io: rename buffer to file in doc-test ([#4230])
- sync: fix Notify example ([#4212])

[#4211]: https://github.com/tokio-rs/tokio/pull/4211
[#4226]: https://github.com/tokio-rs/tokio/pull/4226
[#3689]: https://github.com/tokio-rs/tokio/pull/3689
[#4213]: https://github.com/tokio-rs/tokio/pull/4213
[#4179]: https://github.com/tokio-rs/tokio/pull/4179
[#4223]: https://github.com/tokio-rs/tokio/pull/4223
[#4209]: https://github.com/tokio-rs/tokio/pull/4209
[#4230]: https://github.com/tokio-rs/tokio/pull/4230
[#4212]: https://github.com/tokio-rs/tokio/pull/4212

# 1.13.1 (November 15, 2021)

### Fixed

- sync: fix a data race between `oneshot::Sender::send` and awaiting a
  `oneshot::Receiver` when the oneshot has been closed ([#4226])

[#4226]: https://github.com/tokio-rs/tokio/pull/4226

# 1.13.0 (October 29, 2021)

### Fixed

- sync: fix `Notify` to clone the waker before locking its waiter list ([#4129])
- tokio: add riscv32 to non atomic64 architectures ([#4185])

### Added

- net: add `poll_{recv,send}_ready` methods to `udp` and `uds_datagram` ([#4131])
- net: add `try_*`, `readable`, `writable`, `ready`, and `peer_addr` methods to split halves ([#4120])
- sync: add `blocking_lock` to `Mutex` ([#4130])
- sync: add `watch::Sender::send_replace` ([#3962], [#4195])
- sync: expand `Debug` for `Mutex<T>` impl to unsized `T` ([#4134])
- tracing: instrument time::Sleep ([#4072])
- tracing: use structured location fields for spawned tasks ([#4128])

### Changed

- io: add assert in `copy_bidirectional` that `poll_write` is sensible ([#4125])
- macros: use qualified syntax when polling in `select!` ([#4192])
- runtime: handle `block_on` wakeups better ([#4157])
- task: allocate callback on heap immediately in debug mode ([#4203])
- tokio: assert platform-minimum requirements at build time ([#3797])

### Documented

- docs: conversion of doc comments to indicative mood ([#4174])
- docs: add returning on the first error example for `try_join!` ([#4133])
- docs: fixing broken links in `tokio/src/lib.rs` ([#4132])
- signal: add example with background listener ([#4171])
- sync: add more oneshot examples ([#4153])
- time: document `Interval::tick` cancel safety ([#4152])

[#3797]: https://github.com/tokio-rs/tokio/pull/3797
[#3962]: https://github.com/tokio-rs/tokio/pull/3962
[#4072]: https://github.com/tokio-rs/tokio/pull/4072
[#4120]: https://github.com/tokio-rs/tokio/pull/4120
[#4125]: https://github.com/tokio-rs/tokio/pull/4125
[#4128]: https://github.com/tokio-rs/tokio/pull/4128
[#4129]: https://github.com/tokio-rs/tokio/pull/4129
[#4130]: https://github.com/tokio-rs/tokio/pull/4130
[#4131]: https://github.com/tokio-rs/tokio/pull/4131
[#4132]: https://github.com/tokio-rs/tokio/pull/4132
[#4133]: https://github.com/tokio-rs/tokio/pull/4133
[#4134]: https://github.com/tokio-rs/tokio/pull/4134
[#4152]: https://github.com/tokio-rs/tokio/pull/4152
[#4153]: https://github.com/tokio-rs/tokio/pull/4153
[#4157]: https://github.com/tokio-rs/tokio/pull/4157
[#4171]: https://github.com/tokio-rs/tokio/pull/4171
[#4174]: https://github.com/tokio-rs/tokio/pull/4174
[#4185]: https://github.com/tokio-rs/tokio/pull/4185
[#4192]: https://github.com/tokio-rs/tokio/pull/4192
[#4195]: https://github.com/tokio-rs/tokio/pull/4195
[#4203]: https://github.com/tokio-rs/tokio/pull/4203

# 1.12.0 (September 21, 2021)

### Fixed

- mpsc: ensure `try_reserve` error is consistent with `try_send` ([#4119])
- mpsc: use `spin_loop_hint` instead of `yield_now` ([#4115])
- sync: make `SendError` field public ([#4097])

### Added

- io: add POSIX AIO on FreeBSD ([#4054])
- io: add convenience method `AsyncSeekExt::rewind` ([#4107])
- runtime: add tracing span for `block_on` futures ([#4094])
- runtime: callback when a worker parks and unparks ([#4070])
- sync: implement `try_recv` for mpsc channels ([#4113])

### Documented

- docs: clarify CPU-bound tasks on Tokio ([#4105])
- mpsc: document spurious failures on `poll_recv` ([#4117])
- mpsc: document that `PollSender` impls `Sink` ([#4110])
- task: document non-guarantees of `yield_now` ([#4091])
- time: document paused time details better ([#4061], [#4103])

[#4027]: https://github.com/tokio-rs/tokio/pull/4027
[#4054]: https://github.com/tokio-rs/tokio/pull/4054
[#4061]: https://github.com/tokio-rs/tokio/pull/4061
[#4070]: https://github.com/tokio-rs/tokio/pull/4070
[#4091]: https://github.com/tokio-rs/tokio/pull/4091
[#4094]: https://github.com/tokio-rs/tokio/pull/4094
[#4097]: https://github.com/tokio-rs/tokio/pull/4097
[#4103]: https://github.com/tokio-rs/tokio/pull/4103
[#4105]: https://github.com/tokio-rs/tokio/pull/4105
[#4107]: https://github.com/tokio-rs/tokio/pull/4107
[#4110]: https://github.com/tokio-rs/tokio/pull/4110
[#4113]: https://github.com/tokio-rs/tokio/pull/4113
[#4115]: https://github.com/tokio-rs/tokio/pull/4115
[#4117]: https://github.com/tokio-rs/tokio/pull/4117
[#4119]: https://github.com/tokio-rs/tokio/pull/4119

# 1.11.0 (August 31, 2021)

### Fixed

- time: don't panic when Instant is not monotonic ([#4044])
- io: fix panic in `fill_buf` by not calling `poll_fill_buf` twice ([#4084])

### Added

- watch: add `watch::Sender::subscribe` ([#3800])
- process: add `from_std` to `ChildStd*` ([#4045])
- stats: initial work on runtime stats ([#4043])

### Changed

- tracing: change span naming to new console convention ([#4042])
- io: speed-up waking by using uninitialized array ([#4055], [#4071], [#4075])

### Documented

- time: make Sleep examples easier to find ([#4040])

[#3800]: https://github.com/tokio-rs/tokio/pull/3800
[#4040]: https://github.com/tokio-rs/tokio/pull/4040
[#4042]: https://github.com/tokio-rs/tokio/pull/4042
[#4043]: https://github.com/tokio-rs/tokio/pull/4043
[#4044]: https://github.com/tokio-rs/tokio/pull/4044
[#4045]: https://github.com/tokio-rs/tokio/pull/4045
[#4055]: https://github.com/tokio-rs/tokio/pull/4055
[#4071]: https://github.com/tokio-rs/tokio/pull/4071
[#4075]: https://github.com/tokio-rs/tokio/pull/4075
[#4084]: https://github.com/tokio-rs/tokio/pull/4084

# 1.10.1 (August 24, 2021)

### Fixed

- runtime: fix leak in UnownedTask ([#4063])

[#4063]: https://github.com/tokio-rs/tokio/pull/4063

# 1.10.0 (August 12, 2021)

### Added

- io: add `(read|write)_f(32|64)[_le]` methods ([#4022])
- io: add `fill_buf` and `consume` to `AsyncBufReadExt` ([#3991])
- process: add `Child::raw_handle()` on windows ([#3998])

### Fixed

- doc: fix non-doc builds with `--cfg docsrs` ([#4020])
- io: flush eagerly in `io::copy` ([#4001])
- runtime: a debug assert was sometimes triggered during shutdown ([#4005])
- sync: use `spin_loop_hint` instead of `yield_now` in mpsc ([#4037])
- tokio: the test-util feature depends on rt, sync, and time ([#4036])

### Changes

- runtime: reorganize parts of the runtime ([#3979], [#4005])
- signal: make windows docs for signal module show up on unix builds ([#3770])
- task: quickly send task to heap on debug mode ([#4009])

### Documented

- io: document cancellation safety of `AsyncBufReadExt` ([#3997])
- sync: document when `watch::send` fails ([#4021])

[#3770]: https://github.com/tokio-rs/tokio/pull/3770
[#3979]: https://github.com/tokio-rs/tokio/pull/3979
[#3991]: https://github.com/tokio-rs/tokio/pull/3991
[#3997]: https://github.com/tokio-rs/tokio/pull/3997
[#3998]: https://github.com/tokio-rs/tokio/pull/3998
[#4001]: https://github.com/tokio-rs/tokio/pull/4001
[#4005]: https://github.com/tokio-rs/tokio/pull/4005
[#4009]: https://github.com/tokio-rs/tokio/pull/4009
[#4020]: https://github.com/tokio-rs/tokio/pull/4020
[#4021]: https://github.com/tokio-rs/tokio/pull/4021
[#4022]: https://github.com/tokio-rs/tokio/pull/4022
[#4036]: https://github.com/tokio-rs/tokio/pull/4036
[#4037]: https://github.com/tokio-rs/tokio/pull/4037

# 1.9.0 (July 22, 2021)

### Added

- net: allow customized I/O operations for `TcpStream` ([#3888])
- sync: add getter for the mutex from a guard ([#3928])
- task: expose nameable future for `TaskLocal::scope` ([#3273])

### Fixed

- Fix leak if output of future panics on drop ([#3967])
- Fix leak in `LocalSet` ([#3978])

### Changes

- runtime: reorganize parts of the runtime ([#3909], [#3939], [#3950], [#3955], [#3980])
- sync: clean up `OnceCell` ([#3945])
- task: remove mutex in `JoinError` ([#3959])

[#3273]: https://github.com/tokio-rs/tokio/pull/3273
[#3888]: https://github.com/tokio-rs/tokio/pull/3888
[#3909]: https://github.com/tokio-rs/tokio/pull/3909
[#3928]: https://github.com/tokio-rs/tokio/pull/3928
[#3934]: https://github.com/tokio-rs/tokio/pull/3934
[#3939]: https://github.com/tokio-rs/tokio/pull/3939
[#3945]: https://github.com/tokio-rs/tokio/pull/3945
[#3950]: https://github.com/tokio-rs/tokio/pull/3950
[#3955]: https://github.com/tokio-rs/tokio/pull/3955
[#3959]: https://github.com/tokio-rs/tokio/pull/3959
[#3967]: https://github.com/tokio-rs/tokio/pull/3967
[#3978]: https://github.com/tokio-rs/tokio/pull/3978
[#3980]: https://github.com/tokio-rs/tokio/pull/3980

# 1.8.3 (July 26, 2021)

This release backports two fixes from 1.9.0

### Fixed

- Fix leak if output of future panics on drop ([#3967])
- Fix leak in `LocalSet` ([#3978])

[#3967]: https://github.com/tokio-rs/tokio/pull/3967
[#3978]: https://github.com/tokio-rs/tokio/pull/3978

# 1.8.2 (July 19, 2021)

Fixes a missed edge case from 1.8.1.

### Fixed

- runtime: drop canceled future on next poll ([#3965])

[#3965]: https://github.com/tokio-rs/tokio/pull/3965

# 1.8.1 (July 6, 2021)

Forward ports 1.5.1 fixes.

### Fixed

- runtime: remotely abort tasks on `JoinHandle::abort` ([#3934])

[#3934]: https://github.com/tokio-rs/tokio/pull/3934

# 1.8.0 (July 2, 2021)

### Added

- io: add `get_{ref,mut}` methods to `AsyncFdReadyGuard` and `AsyncFdReadyMutGuard` ([#3807])
- io: efficient implementation of vectored writes for `BufWriter` ([#3163])
- net: add ready/try methods to `NamedPipe{Client,Server}` ([#3866], [#3899])
- sync: add `watch::Receiver::borrow_and_update` ([#3813])
- sync: implement `From<T>` for `OnceCell<T>` ([#3877])
- time: allow users to specify Interval behavior when delayed ([#3721])

### Added (unstable)

- rt: add `tokio::task::Builder` ([#3881])

### Fixed

- net: handle HUP event with `UnixStream` ([#3898])

### Documented

- doc: document cancellation safety ([#3900])
- time: add wait alias to sleep ([#3897])
- time: document auto-advancing behavior of runtime ([#3763])

[#3163]: https://github.com/tokio-rs/tokio/pull/3163
[#3721]: https://github.com/tokio-rs/tokio/pull/3721
[#3763]: https://github.com/tokio-rs/tokio/pull/3763
[#3807]: https://github.com/tokio-rs/tokio/pull/3807
[#3813]: https://github.com/tokio-rs/tokio/pull/3813
[#3866]: https://github.com/tokio-rs/tokio/pull/3866
[#3877]: https://github.com/tokio-rs/tokio/pull/3877
[#3881]: https://github.com/tokio-rs/tokio/pull/3881
[#3897]: https://github.com/tokio-rs/tokio/pull/3897
[#3898]: https://github.com/tokio-rs/tokio/pull/3898
[#3899]: https://github.com/tokio-rs/tokio/pull/3899
[#3900]: https://github.com/tokio-rs/tokio/pull/3900

# 1.7.2 (July 6, 2021)

Forward ports 1.5.1 fixes.

### Fixed

- runtime: remotely abort tasks on `JoinHandle::abort` ([#3934])

[#3934]: https://github.com/tokio-rs/tokio/pull/3934

# 1.7.1 (June 18, 2021)

### Fixed

- runtime: fix early task shutdown during runtime shutdown ([#3870])

[#3870]: https://github.com/tokio-rs/tokio/pull/3870

# 1.7.0 (June 15, 2021)

### Added

- net: add named pipes on windows ([#3760])
- net: add `TcpSocket` from `std::net::TcpStream` conversion ([#3838])
- sync: add `receiver_count` to `watch::Sender` ([#3729])
- sync: export `sync::notify::Notified` future publicly ([#3840])
- tracing: instrument task wakers ([#3836])

### Fixed

- macros: suppress `clippy::default_numeric_fallback` lint in generated code ([#3831])
- runtime: immediately drop new tasks when runtime is shut down ([#3752])
- sync: deprecate unused `mpsc::RecvError` type ([#3833])

### Documented

- io: clarify EOF condition for `AsyncReadExt::read_buf` ([#3850])
- io: clarify limits on return values of `AsyncWrite::poll_write` ([#3820])
- sync: add examples to Semaphore ([#3808])

[#3729]: https://github.com/tokio-rs/tokio/pull/3729
[#3752]: https://github.com/tokio-rs/tokio/pull/3752
[#3760]: https://github.com/tokio-rs/tokio/pull/3760
[#3808]: https://github.com/tokio-rs/tokio/pull/3808
[#3820]: https://github.com/tokio-rs/tokio/pull/3820
[#3831]: https://github.com/tokio-rs/tokio/pull/3831
[#3833]: https://github.com/tokio-rs/tokio/pull/3833
[#3836]: https://github.com/tokio-rs/tokio/pull/3836
[#3838]: https://github.com/tokio-rs/tokio/pull/3838
[#3840]: https://github.com/tokio-rs/tokio/pull/3840
[#3850]: https://github.com/tokio-rs/tokio/pull/3850

# 1.6.3 (July 6, 2021)

Forward ports 1.5.1 fixes.

### Fixed

- runtime: remotely abort tasks on `JoinHandle::abort` ([#3934])

[#3934]: https://github.com/tokio-rs/tokio/pull/3934

# 1.6.2 (June 14, 2021)

### Fixes

- test: sub-ms `time:advance` regression introduced in 1.6 ([#3852])

[#3852]: https://github.com/tokio-rs/tokio/pull/3852

# 1.6.1 (May 28, 2021)

This release reverts [#3518] because it doesn't work on some kernels due to
a kernel bug. ([#3803])

[#3518]: https://github.com/tokio-rs/tokio/issues/3518
[#3803]: https://github.com/tokio-rs/tokio/issues/3803

# 1.6.0 (May 14, 2021)

### Added

- fs: try doing a non-blocking read before punting to the threadpool ([#3518])
- io: add `write_all_buf` to `AsyncWriteExt` ([#3737])
- io: implement `AsyncSeek` for `BufReader`, `BufWriter`, and `BufStream` ([#3491])
- net: support non-blocking vectored I/O ([#3761])
- sync: add `mpsc::Sender::{reserve_owned, try_reserve_owned}` ([#3704])
- sync: add a `MutexGuard::map` method that returns a `MappedMutexGuard` ([#2472])
- time: add getter for Interval's period ([#3705])

### Fixed

- io: wake pending writers on `DuplexStream` close ([#3756])
- process: avoid redundant effort to reap orphan processes ([#3743])
- signal: use `std::os::raw::c_int` instead of `libc::c_int` on public API ([#3774])
- sync: preserve permit state in `notify_waiters` ([#3660])
- task: update `JoinHandle` panic message ([#3727])
- time: prevent `time::advance` from going too far ([#3712])

### Documented

- net: hide `net::unix::datagram` module from docs ([#3775])
- process: updated example ([#3748])
- sync: `Barrier` doc should use task, not thread ([#3780])
- task: update documentation on `block_in_place` ([#3753])

[#2472]: https://github.com/tokio-rs/tokio/pull/2472
[#3491]: https://github.com/tokio-rs/tokio/pull/3491
[#3518]: https://github.com/tokio-rs/tokio/pull/3518
[#3660]: https://github.com/tokio-rs/tokio/pull/3660
[#3704]: https://github.com/tokio-rs/tokio/pull/3704
[#3705]: https://github.com/tokio-rs/tokio/pull/3705
[#3712]: https://github.com/tokio-rs/tokio/pull/3712
[#3727]: https://github.com/tokio-rs/tokio/pull/3727
[#3737]: https://github.com/tokio-rs/tokio/pull/3737
[#3743]: https://github.com/tokio-rs/tokio/pull/3743
[#3748]: https://github.com/tokio-rs/tokio/pull/3748
[#3753]: https://github.com/tokio-rs/tokio/pull/3753
[#3756]: https://github.com/tokio-rs/tokio/pull/3756
[#3761]: https://github.com/tokio-rs/tokio/pull/3761
[#3774]: https://github.com/tokio-rs/tokio/pull/3774
[#3775]: https://github.com/tokio-rs/tokio/pull/3775
[#3780]: https://github.com/tokio-rs/tokio/pull/3780

# 1.5.1 (July 6, 2021)

### Fixed

- runtime: remotely abort tasks on `JoinHandle::abort` ([#3934])

[#3934]: https://github.com/tokio-rs/tokio/pull/3934

# 1.5.0 (April 12, 2021)

### Added

- io: add `AsyncSeekExt::stream_position` ([#3650])
- io: add `AsyncWriteExt::write_vectored` ([#3678])
- io: add a `copy_bidirectional` utility ([#3572])
- net: implement `IntoRawFd` for `TcpSocket` ([#3684])
- sync: add `OnceCell` ([#3591])
- sync: add `OwnedRwLockReadGuard` and `OwnedRwLockWriteGuard` ([#3340])
- sync: add `Semaphore::is_closed` ([#3673])
- sync: add `mpsc::Sender::capacity` ([#3690])
- sync: allow configuring `RwLock` max reads ([#3644])
- task: add `sync_scope` for `LocalKey` ([#3612])

### Fixed

- chore: try to avoid `noalias` attributes on intrusive linked list ([#3654])
- rt: fix panic in `JoinHandle::abort()` when called from other threads ([#3672])
- sync: don't panic in `oneshot::try_recv` ([#3674])
- sync: fix notifications getting dropped on receiver drop ([#3652])
- sync: fix `Semaphore` permit overflow calculation ([#3644])

### Documented

- io: clarify requirements of `AsyncFd` ([#3635])
- runtime: fix unclear docs for `{Handle,Runtime}::block_on` ([#3628])
- sync: document that `Semaphore` is fair ([#3693])
- sync: improve doc on blocking mutex ([#3645])

[#3340]: https://github.com/tokio-rs/tokio/pull/3340
[#3572]: https://github.com/tokio-rs/tokio/pull/3572
[#3591]: https://github.com/tokio-rs/tokio/pull/3591
[#3612]: https://github.com/tokio-rs/tokio/pull/3612
[#3628]: https://github.com/tokio-rs/tokio/pull/3628
[#3635]: https://github.com/tokio-rs/tokio/pull/3635
[#3644]: https://github.com/tokio-rs/tokio/pull/3644
[#3645]: https://github.com/tokio-rs/tokio/pull/3645
[#3650]: https://github.com/tokio-rs/tokio/pull/3650
[#3652]: https://github.com/tokio-rs/tokio/pull/3652
[#3654]: https://github.com/tokio-rs/tokio/pull/3654
[#3672]: https://github.com/tokio-rs/tokio/pull/3672
[#3673]: https://github.com/tokio-rs/tokio/pull/3673
[#3674]: https://github.com/tokio-rs/tokio/pull/3674
[#3678]: https://github.com/tokio-rs/tokio/pull/3678
[#3684]: https://github.com/tokio-rs/tokio/pull/3684
[#3690]: https://github.com/tokio-rs/tokio/pull/3690
[#3693]: https://github.com/tokio-rs/tokio/pull/3693

# 1.4.0 (March 20, 2021)

### Added

- macros: introduce biased argument for `select!` ([#3603])
- runtime: add `Handle::block_on` ([#3569])

### Fixed

- runtime: avoid unnecessary polling of `block_on` future ([#3582])
- runtime: fix memory leak/growth when creating many runtimes ([#3564])
- runtime: mark `EnterGuard` with `must_use` ([#3609])

### Documented

- chore: mention fix for building docs in contributing guide ([#3618])
- doc: add link to `PollSender` ([#3613])
- doc: alias sleep to delay ([#3604])
- sync: improve `Mutex` FIFO explanation ([#3615])
- timer: fix double newline in module docs ([#3617])

[#3564]: https://github.com/tokio-rs/tokio/pull/3564
[#3613]: https://github.com/tokio-rs/tokio/pull/3613
[#3618]: https://github.com/tokio-rs/tokio/pull/3618
[#3617]: https://github.com/tokio-rs/tokio/pull/3617
[#3582]: https://github.com/tokio-rs/tokio/pull/3582
[#3615]: https://github.com/tokio-rs/tokio/pull/3615
[#3603]: https://github.com/tokio-rs/tokio/pull/3603
[#3609]: https://github.com/tokio-rs/tokio/pull/3609
[#3604]: https://github.com/tokio-rs/tokio/pull/3604
[#3569]: https://github.com/tokio-rs/tokio/pull/3569

# 1.3.0 (March 9, 2021)

### Added

- coop: expose an `unconstrained()` opt-out ([#3547])
- net: add `into_std` for net types without it ([#3509])
- sync: add `same_channel` method to `mpsc::Sender` ([#3532])
- sync: add `{try_,}acquire_many_owned` to `Semaphore` ([#3535])
- sync: add back `RwLockWriteGuard::map` and `RwLockWriteGuard::try_map` ([#3348])

### Fixed

- sync: allow `oneshot::Receiver::close` after successful `try_recv` ([#3552])
- time: do not panic on `timeout(Duration::MAX)` ([#3551])

### Documented

- doc: doc aliases for pre-1.0 function names ([#3523])
- io: fix typos ([#3541])
- io: note the EOF behavior of `read_until` ([#3536])
- io: update `AsyncRead::poll_read` doc ([#3557])
- net: update `UdpSocket` splitting doc ([#3517])
- runtime: add link to `LocalSet` on `new_current_thread` ([#3508])
- runtime: update documentation of thread limits ([#3527])
- sync: do not recommend `join_all` for `Barrier` ([#3514])
- sync: documentation for `oneshot` ([#3592])
- sync: rename `notify` to `notify_one` ([#3526])
- time: fix typo in `Sleep` doc ([#3515])
- time: sync `interval.rs` and `time/mod.rs` docs ([#3533])

[#3348]: https://github.com/tokio-rs/tokio/pull/3348
[#3508]: https://github.com/tokio-rs/tokio/pull/3508
[#3509]: https://github.com/tokio-rs/tokio/pull/3509
[#3514]: https://github.com/tokio-rs/tokio/pull/3514
[#3515]: https://github.com/tokio-rs/tokio/pull/3515
[#3517]: https://github.com/tokio-rs/tokio/pull/3517
[#3523]: https://github.com/tokio-rs/tokio/pull/3523
[#3526]: https://github.com/tokio-rs/tokio/pull/3526
[#3527]: https://github.com/tokio-rs/tokio/pull/3527
[#3532]: https://github.com/tokio-rs/tokio/pull/3532
[#3533]: https://github.com/tokio-rs/tokio/pull/3533
[#3535]: https://github.com/tokio-rs/tokio/pull/3535
[#3536]: https://github.com/tokio-rs/tokio/pull/3536
[#3541]: https://github.com/tokio-rs/tokio/pull/3541
[#3547]: https://github.com/tokio-rs/tokio/pull/3547
[#3551]: https://github.com/tokio-rs/tokio/pull/3551
[#3552]: https://github.com/tokio-rs/tokio/pull/3552
[#3557]: https://github.com/tokio-rs/tokio/pull/3557
[#3592]: https://github.com/tokio-rs/tokio/pull/3592

# 1.2.0 (February 5, 2021)

### Added

- signal: make `Signal::poll_recv` method public ([#3383])

### Fixed

- time: make `test-util` paused time fully deterministic ([#3492])

### Documented

- sync: link to new broadcast and watch wrappers ([#3504])

[#3383]: https://github.com/tokio-rs/tokio/pull/3383
[#3492]: https://github.com/tokio-rs/tokio/pull/3492
[#3504]: https://github.com/tokio-rs/tokio/pull/3504

# 1.1.1 (January 29, 2021)

Forward ports 1.0.3 fix.

### Fixed

- io: memory leak during shutdown ([#3477]).

# 1.1.0 (January 22, 2021)

### Added

- net: add `try_read_buf` and `try_recv_buf` ([#3351])
- mpsc: Add `Sender::try_reserve` function ([#3418])
- sync: add `RwLock` `try_read` and `try_write` methods ([#3400])
- io: add `ReadBuf::inner_mut` ([#3443])

### Changed

- macros: improve `select!` error message ([#3352])
- io: keep track of initialized bytes in `read_to_end` ([#3426])
- runtime: consolidate errors for context missing ([#3441])

### Fixed

- task: wake `LocalSet` on `spawn_local` ([#3369])
- sync: fix panic in broadcast::Receiver drop ([#3434])

### Documented

- stream: link to new `Stream` wrappers in `tokio-stream` ([#3343])
- docs: mention that `test-util` feature is not enabled with full ([#3397])
- process: add documentation to process::Child fields ([#3437])
- io: clarify `AsyncFd` docs about changes of the inner fd ([#3430])
- net: update datagram docs on splitting ([#3448])
- time: document that `Sleep` is not `Unpin` ([#3457])
- sync: add link to `PollSemaphore` ([#3456])
- task: add `LocalSet` example ([#3438])
- sync: improve bounded `mpsc` documentation ([#3458])

[#3343]: https://github.com/tokio-rs/tokio/pull/3343
[#3351]: https://github.com/tokio-rs/tokio/pull/3351
[#3352]: https://github.com/tokio-rs/tokio/pull/3352
[#3369]: https://github.com/tokio-rs/tokio/pull/3369
[#3397]: https://github.com/tokio-rs/tokio/pull/3397
[#3400]: https://github.com/tokio-rs/tokio/pull/3400
[#3418]: https://github.com/tokio-rs/tokio/pull/3418
[#3426]: https://github.com/tokio-rs/tokio/pull/3426
[#3430]: https://github.com/tokio-rs/tokio/pull/3430
[#3434]: https://github.com/tokio-rs/tokio/pull/3434
[#3437]: https://github.com/tokio-rs/tokio/pull/3437
[#3438]: https://github.com/tokio-rs/tokio/pull/3438
[#3441]: https://github.com/tokio-rs/tokio/pull/3441
[#3443]: https://github.com/tokio-rs/tokio/pull/3443
[#3448]: https://github.com/tokio-rs/tokio/pull/3448
[#3456]: https://github.com/tokio-rs/tokio/pull/3456
[#3457]: https://github.com/tokio-rs/tokio/pull/3457
[#3458]: https://github.com/tokio-rs/tokio/pull/3458

# 1.0.3 (January 28, 2021)

### Fixed

- io: memory leak during shutdown ([#3477]).

[#3477]: https://github.com/tokio-rs/tokio/pull/3477

# 1.0.2 (January 14, 2021)

### Fixed

- io: soundness in `read_to_end` ([#3428]).

[#3428]: https://github.com/tokio-rs/tokio/pull/3428

# 1.0.1 (December 25, 2020)

This release fixes a soundness hole caused by the combination of `RwLockWriteGuard::map`
and `RwLockWriteGuard::downgrade` by removing the `map` function. This is a breaking
change, but breaking changes are allowed under our semver policy when they are required
to fix a soundness hole. (See [this RFC][semver] for more.)

Note that we have chosen not to do a deprecation cycle or similar because Tokio 1.0.0 was
released two days ago, and therefore the impact should be minimal.

Due to the soundness hole, we have also yanked Tokio version 1.0.0.

### Removed

- sync: remove `RwLockWriteGuard::map` and `RwLockWriteGuard::try_map` ([#3345])

### Fixed

- docs: remove stream feature from docs ([#3335])

[semver]: https://github.com/rust-lang/rfcs/blob/master/text/1122-language-semver.md#soundness-changes
[#3335]: https://github.com/tokio-rs/tokio/pull/3335
[#3345]: https://github.com/tokio-rs/tokio/pull/3345

# 1.0.0 (December 23, 2020)

Commit to the API and long-term support.

### Fixed

- sync: spurious wakeup in `watch` ([#3234]).

### Changed

- io: rename `AsyncFd::with_io()` to `try_io()` ([#3306])
- fs: avoid OS specific `*Ext` traits in favor of conditionally defining the fn ([#3264]).
- fs: `Sleep` is `!Unpin` ([#3278]).
- net: pass `SocketAddr` by value ([#3125]).
- net: `TcpStream::poll_peek` takes `ReadBuf` ([#3259]).
- rt: rename `runtime::Builder::max_threads()` to `max_blocking_threads()` ([#3287]).
- time: require `current_thread` runtime when calling `time::pause()` ([#3289]).

### Removed

- remove `tokio::prelude` ([#3299]).
- io: remove `AsyncFd::with_poll()` ([#3306]).
- net: remove `{Tcp,Unix}Stream::shutdown()` in favor of `AsyncWrite::shutdown()` ([#3298]).
- stream: move all stream utilities to `tokio-stream` until `Stream` is added to
  `std` ([#3277]).
- sync: mpsc `try_recv()` due to unexpected behavior ([#3263]).
- tracing: make unstable as `tracing-core` is not 1.0 yet ([#3266]).

### Added

- fs: `poll_*` fns to `DirEntry` ([#3308]).
- io: `poll_*` fns to `io::Lines`, `io::Split` ([#3308]).
- io: `_mut` method variants to `AsyncFd` ([#3304]).
- net: `poll_*` fns to `UnixDatagram` ([#3223]).
- net: `UnixStream` readiness and non-blocking ops ([#3246]).
- sync: `UnboundedReceiver::blocking_recv()` ([#3262]).
- sync: `watch::Sender::borrow()` ([#3269]).
- sync: `Semaphore::close()` ([#3065]).
- sync: `poll_recv` fns to `mpsc::Receiver`, `mpsc::UnboundedReceiver` ([#3308]).
- time: `poll_tick` fn to `time::Interval` ([#3316]).

[#3065]: https://github.com/tokio-rs/tokio/pull/3065
[#3125]: https://github.com/tokio-rs/tokio/pull/3125
[#3223]: https://github.com/tokio-rs/tokio/pull/3223
[#3234]: https://github.com/tokio-rs/tokio/pull/3234
[#3246]: https://github.com/tokio-rs/tokio/pull/3246
[#3259]: https://github.com/tokio-rs/tokio/pull/3259
[#3262]: https://github.com/tokio-rs/tokio/pull/3262
[#3263]: https://github.com/tokio-rs/tokio/pull/3263
[#3264]: https://github.com/tokio-rs/tokio/pull/3264
[#3266]: https://github.com/tokio-rs/tokio/pull/3266
[#3269]: https://github.com/tokio-rs/tokio/pull/3269
[#3277]: https://github.com/tokio-rs/tokio/pull/3277
[#3278]: https://github.com/tokio-rs/tokio/pull/3278
[#3287]: https://github.com/tokio-rs/tokio/pull/3287
[#3289]: https://github.com/tokio-rs/tokio/pull/3289
[#3298]: https://github.com/tokio-rs/tokio/pull/3298
[#3299]: https://github.com/tokio-rs/tokio/pull/3299
[#3304]: https://github.com/tokio-rs/tokio/pull/3304
[#3306]: https://github.com/tokio-rs/tokio/pull/3306
[#3308]: https://github.com/tokio-rs/tokio/pull/3308
[#3316]: https://github.com/tokio-rs/tokio/pull/3316

# 0.3.6 (December 14, 2020)

### Fixed

- rt: fix deadlock in shutdown ([#3228])
- rt: fix panic in task abort when off rt ([#3159])
- sync: make `add_permits` panic with usize::MAX >> 3 permits ([#3188])
- time: Fix race condition in timer drop ([#3229])
- watch: fix spurious wakeup ([#3244])

### Added

- example: add back udp-codec example ([#3205])
- net: add `TcpStream::into_std` ([#3189])

[#3159]: https://github.com/tokio-rs/tokio/pull/3159
[#3188]: https://github.com/tokio-rs/tokio/pull/3188
[#3189]: https://github.com/tokio-rs/tokio/pull/3189
[#3205]: https://github.com/tokio-rs/tokio/pull/3205
[#3228]: https://github.com/tokio-rs/tokio/pull/3228
[#3229]: https://github.com/tokio-rs/tokio/pull/3229
[#3244]: https://github.com/tokio-rs/tokio/pull/3244

# 0.3.5 (November 30, 2020)

### Fixed

- rt: fix `shutdown_timeout(0)` ([#3196]).
- time: fixed race condition with small sleeps ([#3069]).

### Added

- io: `AsyncFd::with_interest()` ([#3167]).
- signal: `CtrlC` stream on windows ([#3186]).

[#3069]: https://github.com/tokio-rs/tokio/pull/3069
[#3167]: https://github.com/tokio-rs/tokio/pull/3167
[#3186]: https://github.com/tokio-rs/tokio/pull/3186
[#3196]: https://github.com/tokio-rs/tokio/pull/3196

# 0.3.4 (November 18, 2020)

### Fixed

- stream: `StreamMap` `Default` impl bound ([#3093]).
- io: `AsyncFd::into_inner()` should deregister the FD ([#3104]).

### Changed

- meta: `parking_lot` feature enabled with `full` ([#3119]).

### Added

- io: `AsyncWrite` vectored writes ([#3149]).
- net: TCP/UDP readiness and non-blocking ops ([#3130], [#2743], [#3138]).
- net: TCP socket option (linger, send/recv buf size) ([#3145], [#3143]).
- net: PID field in `UCred` with solaris/illumos ([#3085]).
- rt: `runtime::Handle` allows spawning onto a runtime ([#3079]).
- sync: `Notify::notify_waiters()` ([#3098]).
- sync: `acquire_many()`, `try_acquire_many()` to `Semaphore` ([#3067]).

[#2743]: https://github.com/tokio-rs/tokio/pull/2743
[#3067]: https://github.com/tokio-rs/tokio/pull/3067
[#3079]: https://github.com/tokio-rs/tokio/pull/3079
[#3085]: https://github.com/tokio-rs/tokio/pull/3085
[#3093]: https://github.com/tokio-rs/tokio/pull/3093
[#3098]: https://github.com/tokio-rs/tokio/pull/3098
[#3104]: https://github.com/tokio-rs/tokio/pull/3104
[#3119]: https://github.com/tokio-rs/tokio/pull/3119
[#3130]: https://github.com/tokio-rs/tokio/pull/3130
[#3138]: https://github.com/tokio-rs/tokio/pull/3138
[#3143]: https://github.com/tokio-rs/tokio/pull/3143
[#3145]: https://github.com/tokio-rs/tokio/pull/3145
[#3149]: https://github.com/tokio-rs/tokio/pull/3149

# 0.3.3 (November 2, 2020)

Fixes a soundness hole by adding a missing `Send` bound to
`Runtime::spawn_blocking()`.

### Fixed

- rt: include missing `Send`, fixing soundness hole ([#3089]).
- tracing: avoid huge trace span names ([#3074]).

### Added

- net: `TcpSocket::reuseport()`, `TcpSocket::set_reuseport()` ([#3083]).
- net: `TcpSocket::reuseaddr()` ([#3093]).
- net: `TcpSocket::local_addr()` ([#3093]).
- net: add pid to `UCred` ([#2633]).

[#2633]: https://github.com/tokio-rs/tokio/pull/2633
[#3074]: https://github.com/tokio-rs/tokio/pull/3074
[#3083]: https://github.com/tokio-rs/tokio/pull/3083
[#3089]: https://github.com/tokio-rs/tokio/pull/3089
[#3093]: https://github.com/tokio-rs/tokio/pull/3093

# 0.3.2 (October 27, 2020)

Adds `AsyncFd` as a replacement for v0.2's `PollEvented`.

### Fixed

- io: fix a potential deadlock when shutting down the I/O driver ([#2903]).
- sync: `RwLockWriteGuard::downgrade()` bug ([#2957]).

### Added

- io: `AsyncFd` for receiving readiness events on raw FDs ([#2903]).
- net: `poll_*` function on `UdpSocket` ([#2981]).
- net: `UdpSocket::take_error()` ([#3051]).
- sync: `oneshot::Sender::poll_closed()` ([#3032]).

[#2903]: https://github.com/tokio-rs/tokio/pull/2903
[#2957]: https://github.com/tokio-rs/tokio/pull/2957
[#2981]: https://github.com/tokio-rs/tokio/pull/2981
[#3032]: https://github.com/tokio-rs/tokio/pull/3032
[#3051]: https://github.com/tokio-rs/tokio/pull/3051

# 0.3.1 (October 21, 2020)

This release fixes an use-after-free in the IO driver. Additionally, the `read_buf`
and `write_buf` methods have been added back to the IO traits, as the bytes crate
is now on track to reach version 1.0 together with Tokio.

### Fixed

- net: fix use-after-free ([#3019]).
- fs: ensure buffered data is written on shutdown ([#3009]).

### Added

- io: `copy_buf()` ([#2884]).
- io: `AsyncReadExt::read_buf()`, `AsyncReadExt::write_buf()` for working with
  `Buf`/`BufMut` ([#3003]).
- rt: `Runtime::spawn_blocking()` ([#2980]).
- sync: `watch::Sender::is_closed()` ([#2991]).

[#2884]: https://github.com/tokio-rs/tokio/pull/2884
[#2980]: https://github.com/tokio-rs/tokio/pull/2980
[#2991]: https://github.com/tokio-rs/tokio/pull/2991
[#3003]: https://github.com/tokio-rs/tokio/pull/3003
[#3009]: https://github.com/tokio-rs/tokio/pull/3009
[#3019]: https://github.com/tokio-rs/tokio/pull/3019

# 0.3.0 (October 15, 2020)

This represents a 1.0 beta release. APIs are polished and future-proofed. APIs
not included for 1.0 stabilization have been removed.

Biggest changes are:

- I/O driver internal rewrite. The windows implementation includes significant
  changes.
- Runtime API is polished, especially with how it interacts with feature flag
  combinations.
- Feature flags are simplified
  - `rt-core` and `rt-util` are combined to `rt`
  - `rt-threaded` is renamed to `rt-multi-thread` to match builder API
  - `tcp`, `udp`, `uds`, `dns` are combined to `net`.
  - `parking_lot` is included with `full`

### Changes

- meta: Minimum supported Rust version is now 1.45.
- io: `AsyncRead` trait now takes `ReadBuf` in order to safely handle reading
  into uninitialized memory ([#2758]).
- io: Internal I/O driver storage is now able to compact ([#2757]).
- rt: `Runtime::block_on` now takes `&self` ([#2782]).
- sync: `watch` reworked to decouple receiving a change notification from
  receiving the value ([#2814], [#2806]).
- sync: `Notify::notify` is renamed to `notify_one` ([#2822]).
- process: `Child::kill` is now an `async fn` that cleans zombies ([#2823]).
- sync: use `const fn` constructors as possible ([#2833], [#2790])
- signal: reduce cross-thread notification ([#2835]).
- net: tcp,udp,uds types support operations with `&self` ([#2828], [#2919], [#2934]).
- sync: blocking `mpsc` channel supports `send` with `&self` ([#2861]).
- time: rename `delay_for` and `delay_until` to `sleep` and `sleep_until` ([#2826]).
- io: upgrade to `mio` 0.7 ([#2893]).
- io: `AsyncSeek` trait is tweaked ([#2885]).
- fs: `File` operations take `&self` ([#2930]).
- rt: runtime API, and `#[tokio::main]` macro polish ([#2876])
- rt: `Runtime::enter` uses an RAII guard instead of a closure ([#2954]).
- net: the `from_std` function on all sockets no longer sets socket into non-blocking mode ([#2893])

### Added

- sync: `map` function to lock guards ([#2445]).
- sync: `blocking_recv` and `blocking_send` fns to `mpsc` for use outside of Tokio ([#2685]).
- rt: `Builder::thread_name_fn` for configuring thread names ([#1921]).
- fs: impl `FromRawFd` and `FromRawHandle` for `File` ([#2792]).
- process: `Child::wait` and `Child::try_wait` ([#2796]).
- rt: support configuring thread keep-alive duration ([#2809]).
- rt: `task::JoinHandle::abort` forcibly cancels a spawned task ([#2474]).
- sync: `RwLock` write guard to read guard downgrading ([#2733]).
- net: add `poll_*` functions that take `&self` to all net types ([#2845])
- sync: `get_mut()` for `Mutex`, `RwLock` ([#2856]).
- sync: `mpsc::Sender::closed()` waits for `Receiver` half to close ([#2840]).
- sync: `mpsc::Sender::is_closed()` returns true if `Receiver` half is closed ([#2726]).
- stream: `iter` and `iter_mut` to `StreamMap` ([#2890]).
- net: implement `AsRawSocket` on windows ([#2911]).
- net: `TcpSocket` creates a socket without binding or listening ([#2920]).

### Removed

- io: vectored ops are removed from `AsyncRead`, `AsyncWrite` traits ([#2882]).
- io: `mio` is removed from the public API. `PollEvented` and` Registration` are
  removed ([#2893]).
- io: remove `bytes` from public API. `Buf` and `BufMut` implementation are
  removed ([#2908]).
- time: `DelayQueue` is moved to `tokio-util` ([#2897]).

### Fixed

- io: `stdout` and `stderr` buffering on windows ([#2734]).

[#1921]: https://github.com/tokio-rs/tokio/pull/1921
[#2445]: https://github.com/tokio-rs/tokio/pull/2445
[#2474]: https://github.com/tokio-rs/tokio/pull/2474
[#2685]: https://github.com/tokio-rs/tokio/pull/2685
[#2726]: https://github.com/tokio-rs/tokio/pull/2726
[#2733]: https://github.com/tokio-rs/tokio/pull/2733
[#2734]: https://github.com/tokio-rs/tokio/pull/2734
[#2757]: https://github.com/tokio-rs/tokio/pull/2757
[#2758]: https://github.com/tokio-rs/tokio/pull/2758
[#2782]: https://github.com/tokio-rs/tokio/pull/2782
[#2790]: https://github.com/tokio-rs/tokio/pull/2790
[#2792]: https://github.com/tokio-rs/tokio/pull/2792
[#2796]: https://github.com/tokio-rs/tokio/pull/2796
[#2806]: https://github.com/tokio-rs/tokio/pull/2806
[#2809]: https://github.com/tokio-rs/tokio/pull/2809
[#2814]: https://github.com/tokio-rs/tokio/pull/2814
[#2822]: https://github.com/tokio-rs/tokio/pull/2822
[#2823]: https://github.com/tokio-rs/tokio/pull/2823
[#2826]: https://github.com/tokio-rs/tokio/pull/2826
[#2828]: https://github.com/tokio-rs/tokio/pull/2828
[#2833]: https://github.com/tokio-rs/tokio/pull/2833
[#2835]: https://github.com/tokio-rs/tokio/pull/2835
[#2840]: https://github.com/tokio-rs/tokio/pull/2840
[#2845]: https://github.com/tokio-rs/tokio/pull/2845
[#2856]: https://github.com/tokio-rs/tokio/pull/2856
[#2861]: https://github.com/tokio-rs/tokio/pull/2861
[#2876]: https://github.com/tokio-rs/tokio/pull/2876
[#2882]: https://github.com/tokio-rs/tokio/pull/2882
[#2885]: https://github.com/tokio-rs/tokio/pull/2885
[#2890]: https://github.com/tokio-rs/tokio/pull/2890
[#2893]: https://github.com/tokio-rs/tokio/pull/2893
[#2897]: https://github.com/tokio-rs/tokio/pull/2897
[#2908]: https://github.com/tokio-rs/tokio/pull/2908
[#2911]: https://github.com/tokio-rs/tokio/pull/2911
[#2919]: https://github.com/tokio-rs/tokio/pull/2919
[#2920]: https://github.com/tokio-rs/tokio/pull/2920
[#2930]: https://github.com/tokio-rs/tokio/pull/2930
[#2934]: https://github.com/tokio-rs/tokio/pull/2934
[#2954]: https://github.com/tokio-rs/tokio/pull/2954

# 0.2.22 (July 21, 2020)

### Fixes

- docs: misc improvements ([#2572], [#2658], [#2663], [#2656], [#2647], [#2630], [#2487], [#2621],
  [#2624], [#2600], [#2623], [#2622], [#2577], [#2569], [#2589], [#2575], [#2540], [#2564], [#2567],
  [#2520], [#2521], [#2493])
- rt: allow calls to `block_on` inside calls to `block_in_place` that are
  themselves inside `block_on` ([#2645])
- net: fix non-portable behavior when dropping `TcpStream` `OwnedWriteHalf` ([#2597])
- io: improve stack usage by allocating large buffers on directly on the heap
  ([#2634])
- io: fix unsound pin projection in `AsyncReadExt::read_buf` and
  `AsyncWriteExt::write_buf` ([#2612])
- io: fix unnecessary zeroing for `AsyncRead` implementors ([#2525])
- io: Fix `BufReader` not correctly forwarding `poll_write_buf` ([#2654])
- io: fix panic in `AsyncReadExt::read_line` ([#2541])

### Changes

- coop: returning `Poll::Pending` no longer decrements the task budget ([#2549])

### Added

- io: little-endian variants of `AsyncReadExt` and `AsyncWriteExt` methods
  ([#1915])
- task: add [`tracing`] instrumentation to spawned tasks ([#2655])
- sync: allow unsized types in `Mutex` and `RwLock` (via `default` constructors)
  ([#2615])
- net: add `ToSocketAddrs` implementation for `&[SocketAddr]` ([#2604])
- fs: add `OpenOptionsExt` for `OpenOptions` ([#2515])
- fs: add `DirBuilder` ([#2524])

[`tracing`]: https://crates.io/crates/tracing
[#1915]: https://github.com/tokio-rs/tokio/pull/1915
[#2487]: https://github.com/tokio-rs/tokio/pull/2487
[#2493]: https://github.com/tokio-rs/tokio/pull/2493
[#2515]: https://github.com/tokio-rs/tokio/pull/2515
[#2520]: https://github.com/tokio-rs/tokio/pull/2520
[#2521]: https://github.com/tokio-rs/tokio/pull/2521
[#2524]: https://github.com/tokio-rs/tokio/pull/2524
[#2525]: https://github.com/tokio-rs/tokio/pull/2525
[#2540]: https://github.com/tokio-rs/tokio/pull/2540
[#2541]: https://github.com/tokio-rs/tokio/pull/2541
[#2549]: https://github.com/tokio-rs/tokio/pull/2549
[#2564]: https://github.com/tokio-rs/tokio/pull/2564
[#2567]: https://github.com/tokio-rs/tokio/pull/2567
[#2569]: https://github.com/tokio-rs/tokio/pull/2569
[#2572]: https://github.com/tokio-rs/tokio/pull/2572
[#2575]: https://github.com/tokio-rs/tokio/pull/2575
[#2577]: https://github.com/tokio-rs/tokio/pull/2577
[#2589]: https://github.com/tokio-rs/tokio/pull/2589
[#2597]: https://github.com/tokio-rs/tokio/pull/2597
[#2600]: https://github.com/tokio-rs/tokio/pull/2600
[#2604]: https://github.com/tokio-rs/tokio/pull/2604
[#2612]: https://github.com/tokio-rs/tokio/pull/2612
[#2615]: https://github.com/tokio-rs/tokio/pull/2615
[#2621]: https://github.com/tokio-rs/tokio/pull/2621
[#2622]: https://github.com/tokio-rs/tokio/pull/2622
[#2623]: https://github.com/tokio-rs/tokio/pull/2623
[#2624]: https://github.com/tokio-rs/tokio/pull/2624
[#2630]: https://github.com/tokio-rs/tokio/pull/2630
[#2634]: https://github.com/tokio-rs/tokio/pull/2634
[#2645]: https://github.com/tokio-rs/tokio/pull/2645
[#2647]: https://github.com/tokio-rs/tokio/pull/2647
[#2654]: https://github.com/tokio-rs/tokio/pull/2654
[#2655]: https://github.com/tokio-rs/tokio/pull/2655
[#2656]: https://github.com/tokio-rs/tokio/pull/2656
[#2658]: https://github.com/tokio-rs/tokio/pull/2658
[#2663]: https://github.com/tokio-rs/tokio/pull/2663

# 0.2.21 (May 13, 2020)

### Fixes

- macros: disambiguate built-in `#[test]` attribute in macro expansion ([#2503])
- rt: `LocalSet` and task budgeting ([#2462]).
- rt: task budgeting with `block_in_place` ([#2502]).
- sync: release `broadcast` channel memory without sending a value ([#2509]).
- time: notify when resetting a `Delay` to a time in the past ([#2290])

### Added

- io: `get_mut`, `get_ref`, and `into_inner` to `Lines` ([#2450]).
- io: `mio::Ready` argument to `PollEvented` ([#2419]).
- os: illumos support ([#2486]).
- rt: `Handle::spawn_blocking` ([#2501]).
- sync: `OwnedMutexGuard` for `Arc<Mutex<T>>` ([#2455]).

[#2290]: https://github.com/tokio-rs/tokio/pull/2290
[#2419]: https://github.com/tokio-rs/tokio/pull/2419
[#2450]: https://github.com/tokio-rs/tokio/pull/2450
[#2455]: https://github.com/tokio-rs/tokio/pull/2455
[#2462]: https://github.com/tokio-rs/tokio/pull/2462
[#2486]: https://github.com/tokio-rs/tokio/pull/2486
[#2501]: https://github.com/tokio-rs/tokio/pull/2501
[#2502]: https://github.com/tokio-rs/tokio/pull/2502
[#2503]: https://github.com/tokio-rs/tokio/pull/2503
[#2509]: https://github.com/tokio-rs/tokio/pull/2509

# 0.2.20 (April 28, 2020)

### Fixes

- sync: `broadcast` closing the channel no longer requires capacity ([#2448]).
- rt: regression when configuring runtime with `max_threads` less than number of CPUs ([#2457]).

[#2448]: https://github.com/tokio-rs/tokio/pull/2448
[#2457]: https://github.com/tokio-rs/tokio/pull/2457

# 0.2.19 (April 24, 2020)

### Fixes

- docs: misc improvements ([#2400], [#2405], [#2414], [#2420], [#2423], [#2426], [#2427], [#2434], [#2436], [#2440]).
- rt: support `block_in_place` in more contexts ([#2409], [#2410]).
- stream: no panic in `merge()` and `chain()` when using `size_hint()` ([#2430]).
- task: include visibility modifier when defining a task-local ([#2416]).

### Added

- rt: `runtime::Handle::block_on` ([#2437]).
- sync: owned `Semaphore` permit ([#2421]).
- tcp: owned split ([#2270]).

[#2270]: https://github.com/tokio-rs/tokio/pull/2270
[#2400]: https://github.com/tokio-rs/tokio/pull/2400
[#2405]: https://github.com/tokio-rs/tokio/pull/2405
[#2409]: https://github.com/tokio-rs/tokio/pull/2409
[#2410]: https://github.com/tokio-rs/tokio/pull/2410
[#2414]: https://github.com/tokio-rs/tokio/pull/2414
[#2416]: https://github.com/tokio-rs/tokio/pull/2416
[#2420]: https://github.com/tokio-rs/tokio/pull/2420
[#2421]: https://github.com/tokio-rs/tokio/pull/2421
[#2423]: https://github.com/tokio-rs/tokio/pull/2423
[#2426]: https://github.com/tokio-rs/tokio/pull/2426
[#2427]: https://github.com/tokio-rs/tokio/pull/2427
[#2430]: https://github.com/tokio-rs/tokio/pull/2430
[#2434]: https://github.com/tokio-rs/tokio/pull/2434
[#2436]: https://github.com/tokio-rs/tokio/pull/2436
[#2437]: https://github.com/tokio-rs/tokio/pull/2437
[#2440]: https://github.com/tokio-rs/tokio/pull/2440

# 0.2.18 (April 12, 2020)

### Fixes

- task: `LocalSet` was incorrectly marked as `Send` ([#2398])
- io: correctly report `WriteZero` failure in `write_int` ([#2334])

[#2334]: https://github.com/tokio-rs/tokio/pull/2334
[#2398]: https://github.com/tokio-rs/tokio/pull/2398

# 0.2.17 (April 9, 2020)

### Fixes

- rt: bug in work-stealing queue ([#2387])

### Changes

- rt: threadpool uses logical CPU count instead of physical by default ([#2391])

[#2387]: https://github.com/tokio-rs/tokio/pull/2387
[#2391]: https://github.com/tokio-rs/tokio/pull/2391

# 0.2.16 (April 3, 2020)

### Fixes

- sync: fix a regression where `Mutex`, `Semaphore`, and `RwLock` futures no
  longer implement `Sync` ([#2375])
- fs: fix `fs::copy` not copying file permissions ([#2354])

### Added

- time: added `deadline` method to `delay_queue::Expired` ([#2300])
- io: added `StreamReader` ([#2052])

[#2052]: https://github.com/tokio-rs/tokio/pull/2052
[#2300]: https://github.com/tokio-rs/tokio/pull/2300
[#2354]: https://github.com/tokio-rs/tokio/pull/2354
[#2375]: https://github.com/tokio-rs/tokio/pull/2375

# 0.2.15 (April 2, 2020)

### Fixes

- rt: fix queue regression ([#2362]).

### Added

- sync: Add disarm to `mpsc::Sender` ([#2358]).

[#2358]: https://github.com/tokio-rs/tokio/pull/2358
[#2362]: https://github.com/tokio-rs/tokio/pull/2362

# 0.2.14 (April 1, 2020)

### Fixes

- rt: concurrency bug in scheduler ([#2273]).
- rt: concurrency bug with shell runtime ([#2333]).
- test-util: correct pause/resume of time ([#2253]).
- time: `DelayQueue` correct wakeup after `insert` ([#2285]).

### Added

- io: impl `RawFd`, `AsRawHandle` for std io types ([#2335]).
- rt: automatic cooperative task yielding ([#2160], [#2343], [#2349]).
- sync: `RwLock::into_inner` ([#2321]).

### Changed

- sync: semaphore, mutex internals rewritten to avoid allocations ([#2325]).

[#2160]: https://github.com/tokio-rs/tokio/pull/2160
[#2253]: https://github.com/tokio-rs/tokio/pull/2253
[#2273]: https://github.com/tokio-rs/tokio/pull/2273
[#2285]: https://github.com/tokio-rs/tokio/pull/2285
[#2321]: https://github.com/tokio-rs/tokio/pull/2321
[#2325]: https://github.com/tokio-rs/tokio/pull/2325
[#2333]: https://github.com/tokio-rs/tokio/pull/2333
[#2335]: https://github.com/tokio-rs/tokio/pull/2335
[#2343]: https://github.com/tokio-rs/tokio/pull/2343
[#2349]: https://github.com/tokio-rs/tokio/pull/2349

# 0.2.13 (February 28, 2020)

### Fixes

- macros: unresolved import in `pin!` ([#2281]).

[#2281]: https://github.com/tokio-rs/tokio/pull/2281

# 0.2.12 (February 27, 2020)

### Fixes

- net: `UnixStream::poll_shutdown` should call `shutdown(Write)` ([#2245]).
- process: Wake up read and write on `EPOLLERR` ([#2218]).
- rt: potential deadlock when using `block_in_place` and shutting down the
  runtime ([#2119]).
- rt: only detect number of CPUs if `core_threads` not specified ([#2238]).
- sync: reduce `watch::Receiver` struct size ([#2191]).
- time: succeed when setting delay of `$MAX-1` ([#2184]).
- time: avoid having to poll `DelayQueue` after inserting new delay ([#2217]).

### Added

- macros: `pin!` variant that assigns to identifier and pins ([#2274]).
- net: impl `Stream` for `Listener` types ([#2275]).
- rt: `Runtime::shutdown_timeout` waits for runtime to shutdown for specified
  duration ([#2186]).
- stream: `StreamMap` merges streams and can insert / remove streams at
  runtime ([#2185]).
- stream: `StreamExt::skip()` skips a fixed number of items ([#2204]).
- stream: `StreamExt::skip_while()` skips items based on a predicate ([#2205]).
- sync: `Notify` provides basic `async` / `await` task notification ([#2210]).
- sync: `Mutex::into_inner` retrieves guarded data ([#2250]).
- sync: `mpsc::Sender::send_timeout` sends, waiting for up to specified duration
  for channel capacity ([#2227]).
- time: impl `Ord` and `Hash` for `Instant` ([#2239]).

[#2119]: https://github.com/tokio-rs/tokio/pull/2119
[#2184]: https://github.com/tokio-rs/tokio/pull/2184
[#2185]: https://github.com/tokio-rs/tokio/pull/2185
[#2186]: https://github.com/tokio-rs/tokio/pull/2186
[#2191]: https://github.com/tokio-rs/tokio/pull/2191
[#2204]: https://github.com/tokio-rs/tokio/pull/2204
[#2205]: https://github.com/tokio-rs/tokio/pull/2205
[#2210]: https://github.com/tokio-rs/tokio/pull/2210
[#2217]: https://github.com/tokio-rs/tokio/pull/2217
[#2218]: https://github.com/tokio-rs/tokio/pull/2218
[#2227]: https://github.com/tokio-rs/tokio/pull/2227
[#2238]: https://github.com/tokio-rs/tokio/pull/2238
[#2239]: https://github.com/tokio-rs/tokio/pull/2239
[#2245]: https://github.com/tokio-rs/tokio/pull/2245
[#2250]: https://github.com/tokio-rs/tokio/pull/2250
[#2274]: https://github.com/tokio-rs/tokio/pull/2274
[#2275]: https://github.com/tokio-rs/tokio/pull/2275

# 0.2.11 (January 27, 2020)

### Fixes

- docs: misc fixes and tweaks ([#2155], [#2103], [#2027], [#2167], [#2175]).
- macros: handle generics in `#[tokio::main]` method ([#2177]).
- sync: `broadcast` potential lost notifications ([#2135]).
- rt: improve "no runtime" panic messages ([#2145]).

### Added

- optional support for using `parking_lot` internally ([#2164]).
- fs: `fs::copy`, an async version of `std::fs::copy` ([#2079]).
- macros: `select!` waits for the first branch to complete ([#2152]).
- macros: `join!` waits for all branches to complete ([#2158]).
- macros: `try_join!` waits for all branches to complete or the first error ([#2169]).
- macros: `pin!` pins a value to the stack ([#2163]).
- net: `ReadHalf::poll()` and `ReadHalf::poll_peak` ([#2151])
- stream: `StreamExt::timeout()` sets a per-item max duration ([#2149]).
- stream: `StreamExt::fold()` applies a function, producing a single value. ([#2122]).
- sync: impl `Eq`, `PartialEq` for `oneshot::RecvError` ([#2168]).
- task: methods for inspecting the `JoinError` cause ([#2051]).

[#2027]: https://github.com/tokio-rs/tokio/pull/2027
[#2051]: https://github.com/tokio-rs/tokio/pull/2051
[#2079]: https://github.com/tokio-rs/tokio/pull/2079
[#2103]: https://github.com/tokio-rs/tokio/pull/2103
[#2122]: https://github.com/tokio-rs/tokio/pull/2122
[#2135]: https://github.com/tokio-rs/tokio/pull/2135
[#2145]: https://github.com/tokio-rs/tokio/pull/2145
[#2149]: https://github.com/tokio-rs/tokio/pull/2149
[#2151]: https://github.com/tokio-rs/tokio/pull/2151
[#2152]: https://github.com/tokio-rs/tokio/pull/2152
[#2155]: https://github.com/tokio-rs/tokio/pull/2155
[#2158]: https://github.com/tokio-rs/tokio/pull/2158
[#2163]: https://github.com/tokio-rs/tokio/pull/2163
[#2164]: https://github.com/tokio-rs/tokio/pull/2164
[#2167]: https://github.com/tokio-rs/tokio/pull/2167
[#2168]: https://github.com/tokio-rs/tokio/pull/2168
[#2169]: https://github.com/tokio-rs/tokio/pull/2169
[#2175]: https://github.com/tokio-rs/tokio/pull/2175
[#2177]: https://github.com/tokio-rs/tokio/pull/2177

# 0.2.10 (January 21, 2020)

### Fixes

- `#[tokio::main]` when `rt-core` feature flag is not enabled ([#2139]).
- remove `AsyncBufRead` from `BufStream` impl block ([#2108]).
- potential undefined behavior when implementing `AsyncRead` incorrectly ([#2030]).

### Added

- `BufStream::with_capacity` ([#2125]).
- impl `From` and `Default` for `RwLock` ([#2089]).
- `io::ReadHalf::is_pair_of` checks if provided `WriteHalf` is for the same
  underlying object ([#1762], [#2144]).
- `runtime::Handle::try_current()` returns a handle to the current runtime ([#2118]).
- `stream::empty()` returns an immediately ready empty stream ([#2092]).
- `stream::once(val)` returns a stream that yields a single value: `val` ([#2094]).
- `stream::pending()` returns a stream that never becomes ready ([#2092]).
- `StreamExt::chain()` sequences a second stream after the first completes ([#2093]).
- `StreamExt::collect()` transform a stream into a collection ([#2109]).
- `StreamExt::fuse` ends the stream after the first `None` ([#2085]).
- `StreamExt::merge` combines two streams, yielding values as they become ready ([#2091]).
- Task-local storage ([#2126]).

[#1762]: https://github.com/tokio-rs/tokio/pull/1762
[#2030]: https://github.com/tokio-rs/tokio/pull/2030
[#2085]: https://github.com/tokio-rs/tokio/pull/2085
[#2089]: https://github.com/tokio-rs/tokio/pull/2089
[#2091]: https://github.com/tokio-rs/tokio/pull/2091
[#2092]: https://github.com/tokio-rs/tokio/pull/2092
[#2093]: https://github.com/tokio-rs/tokio/pull/2093
[#2094]: https://github.com/tokio-rs/tokio/pull/2094
[#2108]: https://github.com/tokio-rs/tokio/pull/2108
[#2109]: https://github.com/tokio-rs/tokio/pull/2109
[#2118]: https://github.com/tokio-rs/tokio/pull/2118
[#2125]: https://github.com/tokio-rs/tokio/pull/2125
[#2126]: https://github.com/tokio-rs/tokio/pull/2126
[#2139]: https://github.com/tokio-rs/tokio/pull/2139
[#2144]: https://github.com/tokio-rs/tokio/pull/2144

# 0.2.9 (January 9, 2020)

### Fixes

- `AsyncSeek` impl for `File` ([#1986]).
- rt: shutdown deadlock in `threaded_scheduler` ([#2074], [#2082]).
- rt: memory ordering when dropping `JoinHandle` ([#2044]).
- docs: misc API documentation fixes and improvements.

[#1986]: https://github.com/tokio-rs/tokio/pull/1986
[#2044]: https://github.com/tokio-rs/tokio/pull/2044
[#2074]: https://github.com/tokio-rs/tokio/pull/2074
[#2082]: https://github.com/tokio-rs/tokio/pull/2082

# 0.2.8 (January 7, 2020)

### Fixes

- depend on new version of `tokio-macros`.

# 0.2.7 (January 7, 2020)

### Fixes

- potential deadlock when dropping `basic_scheduler` Runtime.
- calling `spawn_blocking` from within a `spawn_blocking` ([#2006]).
- storing a `Runtime` instance in a thread-local ([#2011]).
- miscellaneous documentation fixes.
- rt: fix `Waker::will_wake` to return true when tasks match ([#2045]).
- test-util: `time::advance` runs pending tasks before changing the time ([#2059]).

### Added

- `net::lookup_host` maps a `T: ToSocketAddrs` to a stream of `SocketAddrs` ([#1870]).
- `process::Child` fields are made public to match `std` ([#2014]).
- impl `Stream` for `sync::broadcast::Receiver` ([#2012]).
- `sync::RwLock` provides an asynchronous read-write lock ([#1699]).
- `runtime::Handle::current` returns the handle for the current runtime ([#2040]).
- `StreamExt::filter` filters stream values according to a predicate ([#2001]).
- `StreamExt::filter_map` simultaneously filter and map stream values ([#2001]).
- `StreamExt::try_next` convenience for streams of `Result<T, E>` ([#2005]).
- `StreamExt::take` limits a stream to a specified number of values ([#2025]).
- `StreamExt::take_while` limits a stream based on a predicate ([#2029]).
- `StreamExt::all` tests if every element of the stream matches a predicate ([#2035]).
- `StreamExt::any` tests if any element of the stream matches a predicate ([#2034]).
- `task::LocalSet.await` runs spawned tasks until the set is idle ([#1971]).
- `time::DelayQueue::len` returns the number entries in the queue ([#1755]).
- expose runtime options from the `#[tokio::main]` and `#[tokio::test]` ([#2022]).

[#1699]: https://github.com/tokio-rs/tokio/pull/1699
[#1755]: https://github.com/tokio-rs/tokio/pull/1755
[#1870]: https://github.com/tokio-rs/tokio/pull/1870
[#1971]: https://github.com/tokio-rs/tokio/pull/1971
[#2001]: https://github.com/tokio-rs/tokio/pull/2001
[#2005]: https://github.com/tokio-rs/tokio/pull/2005
[#2006]: https://github.com/tokio-rs/tokio/pull/2006
[#2011]: https://github.com/tokio-rs/tokio/pull/2011
[#2012]: https://github.com/tokio-rs/tokio/pull/2012
[#2014]: https://github.com/tokio-rs/tokio/pull/2014
[#2022]: https://github.com/tokio-rs/tokio/pull/2022
[#2025]: https://github.com/tokio-rs/tokio/pull/2025
[#2029]: https://github.com/tokio-rs/tokio/pull/2029
[#2034]: https://github.com/tokio-rs/tokio/pull/2034
[#2035]: https://github.com/tokio-rs/tokio/pull/2035
[#2040]: https://github.com/tokio-rs/tokio/pull/2040
[#2045]: https://github.com/tokio-rs/tokio/pull/2045
[#2059]: https://github.com/tokio-rs/tokio/pull/2059

# 0.2.6 (December 19, 2019)

### Fixes

- `fs::File::seek` API regression ([#1991]).

[#1991]: https://github.com/tokio-rs/tokio/pull/1991

# 0.2.5 (December 18, 2019)

### Added

- `io::AsyncSeek` trait ([#1924]).
- `Mutex::try_lock` ([#1939])
- `mpsc::Receiver::try_recv` and `mpsc::UnboundedReceiver::try_recv` ([#1939]).
- `writev` support for `TcpStream` ([#1956]).
- `time::throttle` for throttling streams ([#1949]).
- implement `Stream` for `time::DelayQueue` ([#1975]).
- `sync::broadcast` provides a fan-out channel ([#1943]).
- `sync::Semaphore` provides an async semaphore ([#1973]).
- `stream::StreamExt` provides stream utilities ([#1962]).

### Fixes

- deadlock risk while shutting down the runtime ([#1972]).
- panic while shutting down the runtime ([#1978]).
- `sync::MutexGuard` debug output ([#1961]).
- misc doc improvements ([#1933], [#1934], [#1940], [#1942]).

### Changes

- runtime threads are configured with `runtime::Builder::core_threads` and
  `runtime::Builder::max_threads`. `runtime::Builder::num_threads` is
  deprecated ([#1977]).

[#1924]: https://github.com/tokio-rs/tokio/pull/1924
[#1933]: https://github.com/tokio-rs/tokio/pull/1933
[#1934]: https://github.com/tokio-rs/tokio/pull/1934
[#1939]: https://github.com/tokio-rs/tokio/pull/1939
[#1940]: https://github.com/tokio-rs/tokio/pull/1940
[#1942]: https://github.com/tokio-rs/tokio/pull/1942
[#1943]: https://github.com/tokio-rs/tokio/pull/1943
[#1949]: https://github.com/tokio-rs/tokio/pull/1949
[#1956]: https://github.com/tokio-rs/tokio/pull/1956
[#1961]: https://github.com/tokio-rs/tokio/pull/1961
[#1962]: https://github.com/tokio-rs/tokio/pull/1962
[#1972]: https://github.com/tokio-rs/tokio/pull/1972
[#1973]: https://github.com/tokio-rs/tokio/pull/1973
[#1975]: https://github.com/tokio-rs/tokio/pull/1975
[#1977]: https://github.com/tokio-rs/tokio/pull/1977
[#1978]: https://github.com/tokio-rs/tokio/pull/1978

# 0.2.4 (December 6, 2019)

### Fixes

- `sync::Mutex` deadlock when `lock()` future is dropped early ([#1898]).

[#1898]: https://github.com/tokio-rs/tokio/pull/1898

# 0.2.3 (December 6, 2019)

### Added

- read / write integers using `AsyncReadExt` and `AsyncWriteExt` ([#1863]).
- `read_buf` / `write_buf` for reading / writing `Buf` / `BufMut` ([#1881]).
- `TcpStream::poll_peek` - pollable API for performing TCP peek ([#1864]).
- `sync::oneshot::error::TryRecvError` provides variants to detect the error
  kind ([#1874]).
- `LocalSet::block_on` accepts `!'static` task ([#1882]).
- `task::JoinError` is now `Sync` ([#1888]).
- impl conversions between `tokio::time::Instant` and
  `std::time::Instant` ([#1904]).

### Fixes

- calling `spawn_blocking` after runtime shutdown ([#1875]).
- `LocalSet` drop infinite loop ([#1892]).
- `LocalSet` hang under load ([#1905]).
- improved documentation ([#1865], [#1866], [#1868], [#1874], [#1876], [#1911]).

[#1863]: https://github.com/tokio-rs/tokio/pull/1863
[#1864]: https://github.com/tokio-rs/tokio/pull/1864
[#1865]: https://github.com/tokio-rs/tokio/pull/1865
[#1866]: https://github.com/tokio-rs/tokio/pull/1866
[#1868]: https://github.com/tokio-rs/tokio/pull/1868
[#1874]: https://github.com/tokio-rs/tokio/pull/1874
[#1875]: https://github.com/tokio-rs/tokio/pull/1875
[#1876]: https://github.com/tokio-rs/tokio/pull/1876
[#1881]: https://github.com/tokio-rs/tokio/pull/1881
[#1882]: https://github.com/tokio-rs/tokio/pull/1882
[#1888]: https://github.com/tokio-rs/tokio/pull/1888
[#1892]: https://github.com/tokio-rs/tokio/pull/1892
[#1904]: https://github.com/tokio-rs/tokio/pull/1904
[#1905]: https://github.com/tokio-rs/tokio/pull/1905
[#1911]: https://github.com/tokio-rs/tokio/pull/1911

# 0.2.2 (November 29, 2019)

### Fixes

- scheduling with `basic_scheduler` ([#1861]).
- update `spawn` panic message to specify that a task scheduler is required ([#1839]).
- API docs example for `runtime::Builder` to include a task scheduler ([#1841]).
- general documentation ([#1834]).
- building on illumos/solaris ([#1772]).
- panic when dropping `LocalSet` ([#1843]).
- API docs mention the required Cargo features for `Builder::{basic, threaded}_scheduler` ([#1858]).

### Added

- impl `Stream` for `signal::unix::Signal` ([#1849]).
- API docs for platform specific behavior of `signal::ctrl_c` and `signal::unix::Signal` ([#1854]).
- API docs for `signal::unix::Signal::{recv, poll_recv}` and `signal::windows::CtrlBreak::{recv, poll_recv}` ([#1854]).
- `File::into_std` and `File::try_into_std` methods ([#1856]).

[#1772]: https://github.com/tokio-rs/tokio/pull/1772
[#1834]: https://github.com/tokio-rs/tokio/pull/1834
[#1839]: https://github.com/tokio-rs/tokio/pull/1839
[#1841]: https://github.com/tokio-rs/tokio/pull/1841
[#1843]: https://github.com/tokio-rs/tokio/pull/1843
[#1849]: https://github.com/tokio-rs/tokio/pull/1849
[#1854]: https://github.com/tokio-rs/tokio/pull/1854
[#1856]: https://github.com/tokio-rs/tokio/pull/1856
[#1858]: https://github.com/tokio-rs/tokio/pull/1858
[#1861]: https://github.com/tokio-rs/tokio/pull/1861

# 0.2.1 (November 26, 2019)

### Fixes

- API docs for `TcpListener::incoming`, `UnixListener::incoming` ([#1831]).

### Added

- `tokio::task::LocalSet` provides a strategy for spawning `!Send` tasks ([#1733]).
- export `tokio::time::Elapsed` ([#1826]).
- impl `AsRawFd`, `AsRawHandle` for `tokio::fs::File` ([#1827]).

[#1733]: https://github.com/tokio-rs/tokio/pull/1733
[#1826]: https://github.com/tokio-rs/tokio/pull/1826
[#1827]: https://github.com/tokio-rs/tokio/pull/1827
[#1831]: https://github.com/tokio-rs/tokio/pull/1831

# 0.2.0 (November 26, 2019)

A major breaking change. Most implementation and APIs have changed one way or
another. This changelog entry contains a highlight

### Changed

- APIs are updated to use `async / await`.
- most `tokio-*` crates are collapsed into this crate.
- Scheduler is rewritten.
- `tokio::spawn` returns a `JoinHandle`.
- A single I/O / timer is used per runtime.
- I/O driver uses a concurrent slab for allocating state.
- components are made available via feature flag.
- Use `bytes` 0.5
- `tokio::codec` is moved to `tokio-util`.

### Removed

- Standalone `timer` and `net` drivers are removed, use `Runtime` instead
- `current_thread` runtime is removed, use `tokio::runtime::Runtime` with
  `basic_scheduler` instead.

# 0.1.21 (May 30, 2019)

### Changed

- Bump `tokio-trace-core` version to 0.2 ([#1111]).

[#1111]: https://github.com/tokio-rs/tokio/pull/1111

# 0.1.20 (May 14, 2019)

### Added

- `tokio::runtime::Builder::panic_handler` allows configuring handling
  panics on the runtime ([#1055]).

[#1055]: https://github.com/tokio-rs/tokio/pull/1055

# 0.1.19 (April 22, 2019)

### Added

- Re-export `tokio::sync::Mutex` primitive ([#964]).

[#964]: https://github.com/tokio-rs/tokio/pull/964

# 0.1.18 (March 22, 2019)

### Added

- `TypedExecutor` re-export and implementations ([#993]).

[#993]: https://github.com/tokio-rs/tokio/pull/993

# 0.1.17 (March 13, 2019)

### Added

- Propagate trace subscriber in the runtime ([#966]).

[#966]: https://github.com/tokio-rs/tokio/pull/966

# 0.1.16 (March 1, 2019)

### Fixed

- async-await: track latest nightly changes ([#940]).

### Added

- `sync::Watch`, a single value broadcast channel ([#922]).
- Async equivalent of read / write file helpers being added to `std` ([#896]).

[#896]: https://github.com/tokio-rs/tokio/pull/896
[#922]: https://github.com/tokio-rs/tokio/pull/922
[#940]: https://github.com/tokio-rs/tokio/pull/940

# 0.1.15 (January 24, 2019)

### Added

- Re-export tokio-sync APIs ([#839]).
- Stream enumerate combinator ([#832]).

[#832]: https://github.com/tokio-rs/tokio/pull/832
[#839]: https://github.com/tokio-rs/tokio/pull/839

# 0.1.14 (January 6, 2019)

- Use feature flags to break up the crate, allowing users to pick & choose
  components ([#808]).
- Export `UnixDatagram` and `UnixDatagramFramed` ([#772]).

[#772]: https://github.com/tokio-rs/tokio/pull/772
[#808]: https://github.com/tokio-rs/tokio/pull/808

# 0.1.13 (November 21, 2018)

- Fix `Runtime::reactor()` when no tasks are spawned ([#721]).
- `runtime::Builder` no longer uses deprecated methods ([#749]).
- Provide `after_start` and `before_stop` configuration settings for
  `Runtime` ([#756]).
- Implement throttle stream combinator ([#736]).

[#721]: https://github.com/tokio-rs/tokio/pull/721
[#736]: https://github.com/tokio-rs/tokio/pull/736
[#749]: https://github.com/tokio-rs/tokio/pull/749
[#756]: https://github.com/tokio-rs/tokio/pull/756

# 0.1.12 (October 23, 2018)

- runtime: expose `keep_alive` on runtime builder ([#676]).
- runtime: create a reactor per worker thread ([#660]).
- codec: fix panic in `LengthDelimitedCodec` ([#682]).
- io: re-export `tokio_io::io::read` function ([#689]).
- runtime: check for executor re-entry in more places ([#708]).

[#660]: https://github.com/tokio-rs/tokio/pull/660
[#676]: https://github.com/tokio-rs/tokio/pull/676
[#682]: https://github.com/tokio-rs/tokio/pull/682
[#689]: https://github.com/tokio-rs/tokio/pull/689
[#708]: https://github.com/tokio-rs/tokio/pull/708

# 0.1.11 (September 28, 2018)

- Fix `tokio-async-await` dependency ([#675]).

[#675]: https://github.com/tokio-rs/tokio/pull/675

# 0.1.10 (September 27, 2018)

- Fix minimal versions

# 0.1.9 (September 27, 2018)

- Experimental async/await improvements ([#661]).
- Re-export `TaskExecutor` from `tokio-current-thread` ([#652]).
- Improve `Runtime` builder API ([#645]).
- `tokio::run` panics when called from the context of an executor
  ([#646]).
- Introduce `StreamExt` with a `timeout` helper ([#573]).
- Move `length_delimited` into `tokio` ([#575]).
- Re-organize `tokio::net` module ([#548]).
- Re-export `tokio-current-thread::spawn` in current_thread runtime
  ([#579]).

[#548]: https://github.com/tokio-rs/tokio/pull/548
[#573]: https://github.com/tokio-rs/tokio/pull/573
[#575]: https://github.com/tokio-rs/tokio/pull/575
[#579]: https://github.com/tokio-rs/tokio/pull/579
[#645]: https://github.com/tokio-rs/tokio/pull/645
[#646]: https://github.com/tokio-rs/tokio/pull/646
[#652]: https://github.com/tokio-rs/tokio/pull/652
[#661]: https://github.com/tokio-rs/tokio/pull/661

# 0.1.8 (August 23, 2018)

- Extract tokio::executor::current_thread to a sub crate ([#370])
- Add `Runtime::block_on` ([#398])
- Add `runtime::current_thread::block_on_all` ([#477])
- Misc documentation improvements ([#450])
- Implement `std::error::Error` for error types ([#501])

[#370]: https://github.com/tokio-rs/tokio/pull/370
[#398]: https://github.com/tokio-rs/tokio/pull/398
[#450]: https://github.com/tokio-rs/tokio/pull/450
[#477]: https://github.com/tokio-rs/tokio/pull/477
[#501]: https://github.com/tokio-rs/tokio/pull/501

# 0.1.7 (June 6, 2018)

- Add `Runtime::block_on` for concurrent runtime ([#391]).
- Provide handle to `current_thread::Runtime` that allows spawning tasks from
  other threads ([#340]).
- Provide `clock::now()`, a configurable source of time ([#381]).

[#340]: https://github.com/tokio-rs/tokio/pull/340
[#381]: https://github.com/tokio-rs/tokio/pull/381
[#391]: https://github.com/tokio-rs/tokio/pull/391

# 0.1.6 (May 2, 2018)

- Add asynchronous filesystem APIs ([#323]).
- Add "current thread" runtime variant ([#308]).
- `CurrentThread`: Expose inner `Park` instance.
- Improve fairness of `CurrentThread` executor ([#313]).

[#308]: https://github.com/tokio-rs/tokio/pull/308
[#313]: https://github.com/tokio-rs/tokio/pull/313
[#323]: https://github.com/tokio-rs/tokio/pull/323

# 0.1.5 (March 30, 2018)

- Provide timer API ([#266])

[#266]: https://github.com/tokio-rs/tokio/pull/266

# 0.1.4 (March 22, 2018)

- Fix build on FreeBSD ([#218])
- Shutdown the Runtime when the handle is dropped ([#214])
- Set Runtime thread name prefix for worker threads ([#232])
- Add builder for Runtime ([#234])
- Extract TCP and UDP types into separate crates ([#224])
- Optionally support futures 0.2.

[#214]: https://github.com/tokio-rs/tokio/pull/214
[#218]: https://github.com/tokio-rs/tokio/pull/218
[#224]: https://github.com/tokio-rs/tokio/pull/224
[#232]: https://github.com/tokio-rs/tokio/pull/232
[#234]: https://github.com/tokio-rs/tokio/pull/234

# 0.1.3 (March 09, 2018)

- Fix `CurrentThread::turn` to block on idle ([#212]).

[#212]: https://github.com/tokio-rs/tokio/pull/212

# 0.1.2 (March 09, 2018)

- Introduce Tokio Runtime ([#141])
- Provide `CurrentThread` for more flexible usage of current thread executor ([#141]).
- Add Lio for platforms that support it ([#142]).
- I/O resources now lazily bind to the reactor ([#160]).
- Extract Reactor to dedicated crate ([#169])
- Add facade to sub crates and add prelude ([#166]).
- Switch TCP/UDP fns to poll\_ -> Poll<...> style ([#175])

[#141]: https://github.com/tokio-rs/tokio/pull/141
[#142]: https://github.com/tokio-rs/tokio/pull/142
[#160]: https://github.com/tokio-rs/tokio/pull/160
[#166]: https://github.com/tokio-rs/tokio/pull/166
[#169]: https://github.com/tokio-rs/tokio/pull/169
[#175]: https://github.com/tokio-rs/tokio/pull/175

# 0.1.1 (February 09, 2018)

- Doc fixes

# 0.1.0 (February 07, 2018)

- Initial crate released based on [RFC](https://github.com/tokio-rs/tokio-rfcs/pull/3).

# axum

`axum` is an HTTP routing and request-handling library that focuses on ergonomics and modularity.

[![Build status](https://github.com/tokio-rs/axum/actions/workflows/CI.yml/badge.svg?branch=main)](https://github.com/tokio-rs/axum/actions/workflows/CI.yml)
[![Crates.io](https://img.shields.io/crates/v/axum)](https://crates.io/crates/axum)
[![Documentation](https://docs.rs/axum/badge.svg)][docs]

More information about this crate can be found in the [crate documentation][docs].

## High level features

- Route requests to handlers with a macro free API.
- Declaratively parse requests using extractors.
- Simple and predictable error handling model.
- Generate responses with minimal boilerplate.
- Take full advantage of the [`tower`] and [`tower-http`] ecosystem of
  middleware, services, and utilities.

In particular the last point is what sets `axum` apart from other libraries / frameworks.
`axum` doesn't have its own middleware system but instead uses
[`tower::Service`]. This means `axum` gets timeouts, tracing, compression,
authorization, and more, for free. It also enables you to share middleware with
applications written using [`hyper`] or [`tonic`].

## ⚠ Breaking changes ⚠

We are currently working towards axum 0.9 so the `main` branch contains breaking
changes. See the [`0.8.x`] branch for what's released to crates.io.

[`0.8.x`]: https://github.com/tokio-rs/axum/tree/v0.8.x

## Usage example

```rust
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await;
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
```

You can find this [example][readme-example] as well as other example projects in
the [example directory][examples].

See the [crate documentation][docs] for way more examples.

## Performance

`axum` is a relatively thin layer on top of [`hyper`] and adds very little
overhead. So `axum`'s performance is comparable to [`hyper`]. You can find
benchmarks [here](https://github.com/programatik29/rust-web-benchmarks) and
[here](https://web-frameworks-benchmark.netlify.app/result?l=rust).

## Safety

This crate uses `#![forbid(unsafe_code)]` to ensure everything is implemented in
100% safe Rust.

## Minimum supported Rust version

axum's MSRV is 1.80.

## Examples

The [examples] folder contains various examples of how to use `axum`. The
[docs] also provide lots of code snippets and examples. For full-fledged examples, check out community-maintained [showcases] or [tutorials].

## Getting Help

In the `axum`'s repo we also have a [number of examples][examples] showing how
to put everything together. Community-maintained [showcases] and [tutorials] also demonstrate how to use `axum` for real-world applications. You're also welcome to ask in the [Discord channel][chat] or open a [discussion] with your question.

## Community projects

See [here][ecosystem] for a list of community maintained crates and projects
built with `axum`.

## Contributing

🎈 Thanks for your help improving the project! We are so happy to have
you! We have a [contributing guide][contributing] to help you get involved in the
`axum` project.

## License

This project is licensed under the [MIT license][license].

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `axum` by you, shall be licensed as MIT, without any
additional terms or conditions.

[readme-example]: https://github.com/tokio-rs/axum/tree/main/examples/readme
[examples]: https://github.com/tokio-rs/axum/tree/main/examples
[docs]: https://docs.rs/axum
[`tower`]: https://crates.io/crates/tower
[`hyper`]: https://crates.io/crates/hyper
[`tower-http`]: https://crates.io/crates/tower-http
[`tonic`]: https://crates.io/crates/tonic
[contributing]: https://github.com/tokio-rs/axum/blob/main/CONTRIBUTING.md
[chat]: https://discord.gg/tokio
[discussion]: https://github.com/tokio-rs/axum/discussions/new?category=q-a
[`tower::Service`]: https://docs.rs/tower/latest/tower/trait.Service.html
[ecosystem]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md
[showcases]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#project-showcase
[tutorials]: https://github.com/tokio-rs/axum/blob/main/ECOSYSTEM.md#tutorials
[license]: https://github.com/tokio-rs/axum/blob/main/axum/LICENSE

## Paths for Referring to an Item in the Module Tree

To show Rust where to find an item in a module tree, we use a path in the same
way we use a path when navigating a filesystem. To call a function, we need to
know its path.

A path can take two forms:

- An _absolute path_ is the full path starting from a crate root; for code
  from an external crate, the absolute path begins with the crate name, and for
  code from the current crate, it starts with the literal `crate`.
- A _relative path_ starts from the current module and uses `self`, `super`, or
  an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers
separated by double colons (`::`).

Returning to Listing 7-1, say we want to call the `add_to_waitlist` function.
This is the same as asking: What’s the path of the `add_to_waitlist` function?
Listing 7-3 contains Listing 7-1 with some of the modules and functions removed.

We’ll show two ways to call the `add_to_waitlist` function from a new function,
`eat_at_restaurant`, defined in the crate root. These paths are correct, but
there’s another problem remaining that will prevent this example from compiling
as is. We’ll explain why in a bit.

The `eat_at_restaurant` function is part of our library crate’s public API, so
we mark it with the `pub` keyword. In the [“Exposing Paths with the `pub`
Keyword”][pub]<!-- ignore --> section, we’ll go into more detail about `pub`.

<Listing number="7-3" file-name="src/lib.rs" caption="Calling the `add_to_waitlist` function using absolute and relative paths">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-03/src/lib.rs}}
```

</Listing>

The first time we call the `add_to_waitlist` function in `eat_at_restaurant`,
we use an absolute path. The `add_to_waitlist` function is defined in the same
crate as `eat_at_restaurant`, which means we can use the `crate` keyword to
start an absolute path. We then include each of the successive modules until we
make our way to `add_to_waitlist`. You can imagine a filesystem with the same
structure: We’d specify the path `/front_of_house/hosting/add_to_waitlist` to
run the `add_to_waitlist` program; using the `crate` name to start from the
crate root is like using `/` to start from the filesystem root in your shell.

The second time we call `add_to_waitlist` in `eat_at_restaurant`, we use a
relative path. The path starts with `front_of_house`, the name of the module
defined at the same level of the module tree as `eat_at_restaurant`. Here the
filesystem equivalent would be using the path
`front_of_house/hosting/add_to_waitlist`. Starting with a module name means
that the path is relative.

Choosing whether to use a relative or absolute path is a decision you’ll make
based on your project, and it depends on whether you’re more likely to move
item definition code separately from or together with the code that uses the
item. For example, if we moved the `front_of_house` module and the
`eat_at_restaurant` function into a module named `customer_experience`, we’d
need to update the absolute path to `add_to_waitlist`, but the relative path
would still be valid. However, if we moved the `eat_at_restaurant` function
separately into a module named `dining`, the absolute path to the
`add_to_waitlist` call would stay the same, but the relative path would need to
be updated. Our preference in general is to specify absolute paths because it’s
more likely we’ll want to move code definitions and item calls independently of
each other.

Let’s try to compile Listing 7-3 and find out why it won’t compile yet! The
errors we get are shown in Listing 7-4.

<Listing number="7-4" caption="Compiler errors from building the code in Listing 7-3">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-03/output.txt}}
```

</Listing>

The error messages say that module `hosting` is private. In other words, we
have the correct paths for the `hosting` module and the `add_to_waitlist`
function, but Rust won’t let us use them because it doesn’t have access to the
private sections. In Rust, all items (functions, methods, structs, enums,
modules, and constants) are private to parent modules by default. If you want
to make an item like a function or struct private, you put it in a module.

Items in a parent module can’t use the private items inside child modules, but
items in child modules can use the items in their ancestor modules. This is
because child modules wrap and hide their implementation details, but the child
modules can see the context in which they’re defined. To continue with our
metaphor, think of the privacy rules as being like the back office of a
restaurant: What goes on in there is private to restaurant customers, but
office managers can see and do everything in the restaurant they operate.

Rust chose to have the module system function this way so that hiding inner
implementation details is the default. That way, you know which parts of the
inner code you can change without breaking the outer code. However, Rust does
give you the option to expose inner parts of child modules’ code to outer
ancestor modules by using the `pub` keyword to make an item public.

### Exposing Paths with the `pub` Keyword

Let’s return to the error in Listing 7-4 that told us the `hosting` module is
private. We want the `eat_at_restaurant` function in the parent module to have
access to the `add_to_waitlist` function in the child module, so we mark the
`hosting` module with the `pub` keyword, as shown in Listing 7-5.

<Listing number="7-5" file-name="src/lib.rs" caption="Declaring the `hosting` module as `pub` to use it from `eat_at_restaurant`">

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-05/src/lib.rs:here}}
```

</Listing>

Unfortunately, the code in Listing 7-5 still results in compiler errors, as
shown in Listing 7-6.

<Listing number="7-6" caption="Compiler errors from building the code in Listing 7-5">

```console
{{#include ../listings/ch07-managing-growing-projects/listing-07-05/output.txt}}
```

</Listing>

What happened? Adding the `pub` keyword in front of `mod hosting` makes the
module public. With this change, if we can access `front_of_house`, we can
access `hosting`. But the _contents_ of `hosting` are still private; making the
module public doesn’t make its contents public. The `pub` keyword on a module
only lets code in its ancestor modules refer to it, not access its inner code.
Because modules are containers, there’s not much we can do by only making the
module public; we need to go further and choose to make one or more of the
items within the module public as well.

The errors in Listing 7-6 say that the `add_to_waitlist` function is private.
The privacy rules apply to structs, enums, functions, and methods as well as
modules.

Let’s also make the `add_to_waitlist` function public by adding the `pub`
keyword before its definition, as in Listing 7-7.

<Listing number="7-7" file-name="src/lib.rs" caption="Adding the `pub` keyword to `mod hosting` and `fn add_to_waitlist` lets us call the function from `eat_at_restaurant`.">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-07/src/lib.rs:here}}
```

</Listing>

Now the code will compile! To see why adding the `pub` keyword lets us use
these paths in `eat_at_restaurant` with respect to the privacy rules, let’s
look at the absolute and the relative paths.

In the absolute path, we start with `crate`, the root of our crate’s module
tree. The `front_of_house` module is defined in the crate root. While
`front_of_house` isn’t public, because the `eat_at_restaurant` function is
defined in the same module as `front_of_house` (that is, `eat_at_restaurant`
and `front_of_house` are siblings), we can refer to `front_of_house` from
`eat_at_restaurant`. Next is the `hosting` module marked with `pub`. We can
access the parent module of `hosting`, so we can access `hosting`. Finally, the
`add_to_waitlist` function is marked with `pub`, and we can access its parent
module, so this function call works!

In the relative path, the logic is the same as the absolute path except for the
first step: Rather than starting from the crate root, the path starts from
`front_of_house`. The `front_of_house` module is defined within the same module
as `eat_at_restaurant`, so the relative path starting from the module in which
`eat_at_restaurant` is defined works. Then, because `hosting` and
`add_to_waitlist` are marked with `pub`, the rest of the path works, and this
function call is valid!

If you plan to share your library crate so that other projects can use your
code, your public API is your contract with users of your crate that determines
how they can interact with your code. There are many considerations around
managing changes to your public API to make it easier for people to depend on
your crate. These considerations are beyond the scope of this book; if you’re
interested in this topic, see [the Rust API Guidelines][api-guidelines].

> #### Best Practices for Packages with a Binary and a Library
>
> We mentioned that a package can contain both a _src/main.rs_ binary crate
> root as well as a _src/lib.rs_ library crate root, and both crates will have
> the package name by default. Typically, packages with this pattern of
> containing both a library and a binary crate will have just enough code in the
> binary crate to start an executable that calls code defined in the library
> crate. This lets other projects benefit from the most functionality that the
> package provides because the library crate’s code can be shared.
>
> The module tree should be defined in _src/lib.rs_. Then, any public items can
> be used in the binary crate by starting paths with the name of the package.
> The binary crate becomes a user of the library crate just like a completely
> external crate would use the library crate: It can only use the public API.
> This helps you design a good API; not only are you the author, but you’re
> also a client!
>
> In [Chapter 12][ch12]<!-- ignore -->, we’ll demonstrate this organizational
> practice with a command line program that will contain both a binary crate
> and a library crate.

### Starting Relative Paths with `super`

We can construct relative paths that begin in the parent module, rather than
the current module or the crate root, by using `super` at the start of the
path. This is like starting a filesystem path with the `..` syntax that means
to go to the parent directory. Using `super` allows us to reference an item
that we know is in the parent module, which can make rearranging the module
tree easier when the module is closely related to the parent but the parent
might be moved elsewhere in the module tree someday.

Consider the code in Listing 7-8 that models the situation in which a chef
fixes an incorrect order and personally brings it out to the customer. The
function `fix_incorrect_order` defined in the `back_of_house` module calls the
function `deliver_order` defined in the parent module by specifying the path to
`deliver_order`, starting with `super`.

<Listing number="7-8" file-name="src/lib.rs" caption="Calling a function using a relative path starting with `super`">

```rust,noplayground,test_harness
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-08/src/lib.rs}}
```

</Listing>

The `fix_incorrect_order` function is in the `back_of_house` module, so we can
use `super` to go to the parent module of `back_of_house`, which in this case
is `crate`, the root. From there, we look for `deliver_order` and find it.
Success! We think the `back_of_house` module and the `deliver_order` function
are likely to stay in the same relationship to each other and get moved
together should we decide to reorganize the crate’s module tree. Therefore, we
used `super` so that we’ll have fewer places to update code in the future if
this code gets moved to a different module.

### Making Structs and Enums Public

We can also use `pub` to designate structs and enums as public, but there are a
few extra details to the usage of `pub` with structs and enums. If we use `pub`
before a struct definition, we make the struct public, but the struct’s fields
will still be private. We can make each field public or not on a case-by-case
basis. In Listing 7-9, we’ve defined a public `back_of_house::Breakfast` struct
with a public `toast` field but a private `seasonal_fruit` field. This models
the case in a restaurant where the customer can pick the type of bread that
comes with a meal, but the chef decides which fruit accompanies the meal based
on what’s in season and in stock. The available fruit changes quickly, so
customers can’t choose the fruit or even see which fruit they’ll get.

<Listing number="7-9" file-name="src/lib.rs" caption="A struct with some public fields and some private fields">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-09/src/lib.rs}}
```

</Listing>

Because the `toast` field in the `back_of_house::Breakfast` struct is public,
in `eat_at_restaurant` we can write and read to the `toast` field using dot
notation. Notice that we can’t use the `seasonal_fruit` field in
`eat_at_restaurant`, because `seasonal_fruit` is private. Try uncommenting the
line modifying the `seasonal_fruit` field value to see what error you get!

Also, note that because `back_of_house::Breakfast` has a private field, the
struct needs to provide a public associated function that constructs an
instance of `Breakfast` (we’ve named it `summer` here). If `Breakfast` didn’t
have such a function, we couldn’t create an instance of `Breakfast` in
`eat_at_restaurant`, because we couldn’t set the value of the private
`seasonal_fruit` field in `eat_at_restaurant`.

In contrast, if we make an enum public, all of its variants are then public. We
only need the `pub` before the `enum` keyword, as shown in Listing 7-10.

<Listing number="7-10" file-name="src/lib.rs" caption="Designating an enum as public makes all its variants public.">

```rust,noplayground
{{#rustdoc_include ../listings/ch07-managing-growing-projects/listing-07-10/src/lib.rs}}
```

</Listing>

Because we made the `Appetizer` enum public, we can use the `Soup` and `Salad`
variants in `eat_at_restaurant`.

Enums aren’t very useful unless their variants are public; it would be annoying
to have to annotate all enum variants with `pub` in every case, so the default
for enum variants is to be public. Structs are often useful without their
fields being public, so struct fields follow the general rule of everything
being private by default unless annotated with `pub`.

There’s one more situation involving `pub` that we haven’t covered, and that is
our last module system feature: the `use` keyword. We’ll cover `use` by itself
first, and then we’ll show how to combine `pub` and `use`.

[pub]: ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#exposing-paths-with-the-pub-keyword
[api-guidelines]: https://rust-lang.github.io/api-guidelines/
[ch12]: ch12-00-an-io-project.html

## Functions

Functions are prevalent in Rust code. You’ve already seen one of the most
important functions in the language: the `main` function, which is the entry
point of many programs. You’ve also seen the `fn` keyword, which allows you to
declare new functions.

Rust code uses _snake case_ as the conventional style for function and variable
names, in which all letters are lowercase and underscores separate words.
Here’s a program that contains an example function definition:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-16-functions/src/main.rs}}
```

We define a function in Rust by entering `fn` followed by a function name and a
set of parentheses. The curly brackets tell the compiler where the function
body begins and ends.

We can call any function we’ve defined by entering its name followed by a set
of parentheses. Because `another_function` is defined in the program, it can be
called from inside the `main` function. Note that we defined `another_function`
_after_ the `main` function in the source code; we could have defined it before
as well. Rust doesn’t care where you define your functions, only that they’re
defined somewhere in a scope that can be seen by the caller.

Let’s start a new binary project named _functions_ to explore functions
further. Place the `another_function` example in _src/main.rs_ and run it. You
should see the following output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-16-functions/output.txt}}
```

The lines execute in the order in which they appear in the `main` function.
First the “Hello, world!” message prints, and then `another_function` is called
and its message is printed.

### Parameters

We can define functions to have _parameters_, which are special variables that
are part of a function’s signature. When a function has parameters, you can
provide it with concrete values for those parameters. Technically, the concrete
values are called _arguments_, but in casual conversation, people tend to use
the words _parameter_ and _argument_ interchangeably for either the variables
in a function’s definition or the concrete values passed in when you call a
function.

In this version of `another_function` we add a parameter:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/src/main.rs}}
```

Try running this program; you should get the following output:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-17-functions-with-parameters/output.txt}}
```

The declaration of `another_function` has one parameter named `x`. The type of
`x` is specified as `i32`. When we pass `5` in to `another_function`, the
`println!` macro puts `5` where the pair of curly brackets containing `x` was
in the format string.

In function signatures, you _must_ declare the type of each parameter. This is
a deliberate decision in Rust’s design: Requiring type annotations in function
definitions means the compiler almost never needs you to use them elsewhere in
the code to figure out what type you mean. The compiler is also able to give
more-helpful error messages if it knows what types the function expects.

When defining multiple parameters, separate the parameter declarations with
commas, like this:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/src/main.rs}}
```

This example creates a function named `print_labeled_measurement` with two
parameters. The first parameter is named `value` and is an `i32`. The second is
named `unit_label` and is type `char`. The function then prints text containing
both the `value` and the `unit_label`.

Let’s try running this code. Replace the program currently in your _functions_
project’s _src/main.rs_ file with the preceding example and run it using `cargo
run`:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-18-functions-with-multiple-parameters/output.txt}}
```

Because we called the function with `5` as the value for `value` and `'h'` as
the value for `unit_label`, the program output contains those values.

### Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an
expression. So far, the functions we’ve covered haven’t included an ending
expression, but you have seen an expression as part of a statement. Because
Rust is an expression-based language, this is an important distinction to
understand. Other languages don’t have the same distinctions, so let’s look at
what statements and expressions are and how their differences affect the bodies
of functions.

- _Statements_ are instructions that perform some action and do not return
  a value.
- _Expressions_ evaluate to a resultant value.

Let’s look at some examples.

We’ve actually already used statements and expressions. Creating a variable and
assigning a value to it with the `let` keyword is a statement. In Listing 3-1,
`let y = 6;` is a statement.

<Listing number="3-1" file-name="src/main.rs" caption="A `main` function declaration containing one statement">

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/listing-03-01/src/main.rs}}
```

</Listing>

Function definitions are also statements; the entire preceding example is a
statement in itself. (As we’ll see shortly, calling a function is not a
statement, though.)

Statements do not return values. Therefore, you can’t assign a `let` statement
to another variable, as the following code tries to do; you’ll get an error:

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/src/main.rs}}
```

When you run this program, the error you’ll get looks like this:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-19-statements-vs-expressions/output.txt}}
```

The `let y = 6` statement does not return a value, so there isn’t anything for
`x` to bind to. This is different from what happens in other languages, such as
C and Ruby, where the assignment returns the value of the assignment. In those
languages, you can write `x = y = 6` and have both `x` and `y` have the value
`6`; that is not the case in Rust.

Expressions evaluate to a value and make up most of the rest of the code that
you’ll write in Rust. Consider a math operation, such as `5 + 6`, which is an
expression that evaluates to the value `11`. Expressions can be part of
statements: In Listing 3-1, the `6` in the statement `let y = 6;` is an
expression that evaluates to the value `6`. Calling a function is an
expression. Calling a macro is an expression. A new scope block created with
curly brackets is an expression, for example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-20-blocks-are-expressions/src/main.rs}}
```

This expression:

```rust,ignore
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to `4`. That value gets bound to `y`
as part of the `let` statement. Note the `x + 1` line without a semicolon at
the end, which is unlike most of the lines you’ve seen so far. Expressions do
not include ending semicolons. If you add a semicolon to the end of an
expression, you turn it into a statement, and it will then not return a value.
Keep this in mind as you explore function return values and expressions next.

### Functions with Return Values

Functions can return values to the code that calls them. We don’t name return
values, but we must declare their type after an arrow (`->`). In Rust, the
return value of the function is synonymous with the value of the final
expression in the block of the body of a function. You can return early from a
function by using the `return` keyword and specifying a value, but most
functions return the last expression implicitly. Here’s an example of a
function that returns a value:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/src/main.rs}}
```

There are no function calls, macros, or even `let` statements in the `five`
function—just the number `5` by itself. That’s a perfectly valid function in
Rust. Note that the function’s return type is specified too, as `-> i32`. Try
running this code; the output should look like this:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-21-function-return-values/output.txt}}
```

The `5` in `five` is the function’s return value, which is why the return type
is `i32`. Let’s examine this in more detail. There are two important bits:
First, the line `let x = five();` shows that we’re using the return value of a
function to initialize a variable. Because the function `five` returns a `5`,
that line is the same as the following:

```rust
let x = 5;
```

Second, the `five` function has no parameters and defines the type of the
return value, but the body of the function is a lonely `5` with no semicolon
because it’s an expression whose value we want to return.

Let’s look at another example:

<span class="filename">Filename: src/main.rs</span>

```rust
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-22-function-parameter-and-return/src/main.rs}}
```

Running this code will print `The value of x is: 6`. But what happens if we
place a semicolon at the end of the line containing `x + 1`, changing it from
an expression to a statement?

<span class="filename">Filename: src/main.rs</span>

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/src/main.rs}}
```

Compiling this code will produce an error, as follows:

```console
{{#include ../listings/ch03-common-programming-concepts/no-listing-23-statements-dont-return-values/output.txt}}
```

The main error message, `mismatched types`, reveals the core issue with this
code. The definition of the function `plus_one` says that it will return an
`i32`, but statements don’t evaluate to a value, which is expressed by `()`,
the unit type. Therefore, nothing is returned, which contradicts the function
definition and results in an error. In this output, Rust provides a message to
possibly help rectify this issue: It suggests removing the semicolon, which
would fix the error.

<p align="center">
  <br>
  <br>
  <a href="https://vite.dev" target="_blank" rel="noopener noreferrer">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://vite.dev/vite-light.svg">
      <source media="(prefers-color-scheme: light)" srcset="https://vite.dev/vite-dark.svg">
      <img alt="vite logo" src="https://vite.dev/vite-dark.svg" height="60">
    </picture>
  </a>
  <br>
  <br>
</p>
<br/>
<p align="center">
  <a href="https://npmjs.com/package/vite"><img src="https://img.shields.io/npm/v/vite.svg" alt="npm package"></a>
  <a href="https://nodejs.org/en/about/previous-releases"><img src="https://img.shields.io/node/v/vite.svg" alt="node compatibility"></a>
  <a href="https://github.com/vitejs/vite/actions/workflows/ci.yml"><img src="https://github.com/vitejs/vite/actions/workflows/ci.yml/badge.svg?branch=main" alt="build status"></a>
  <a href="https://chat.vite.dev"><img src="https://img.shields.io/badge/chat-discord-blue?style=flat&logo=discord" alt="discord chat"></a>
</p>
<br/>

# Vite ⚡

> Next Generation Frontend Tooling

- 💡 Instant Server Start
- ⚡️ Lightning Fast HMR
- 🛠️ Rich Features
- 📦 Optimized Build
- 🔩 Universal Plugin Interface
- 🔑 Fully Typed APIs

Vite (French word for "quick", pronounced [`/viːt/`](https://cdn.jsdelivr.net/gh/vitejs/vite@main/docs/public/vite.mp3), like "veet") is a new breed of frontend build tooling that significantly improves the frontend development experience. It consists of two major parts:

- A dev server that serves your source files over [native ES modules](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Modules), with [rich built-in features](https://vite.dev/guide/features.html) and astonishingly fast [Hot Module Replacement (HMR)](https://vite.dev/guide/features.html#hot-module-replacement).

- A [build command](https://vite.dev/guide/build.html) that bundles your code with [Rollup](https://rollupjs.org), pre-configured to output highly optimized static assets for production.

In addition, Vite is highly extensible via its [Plugin API](https://vite.dev/guide/api-plugin.html) and [JavaScript API](https://vite.dev/guide/api-javascript.html) with full typing support.

[Read the Docs to Learn More](https://vite.dev).

## Packages

| Package                                         | Version (click for changelogs)                                                                                                    |
| ----------------------------------------------- | :-------------------------------------------------------------------------------------------------------------------------------- |
| [vite](packages/vite)                           | [![vite version](https://img.shields.io/npm/v/vite.svg?label=%20)](packages/vite/CHANGELOG.md)                                    |
| [@vitejs/plugin-legacy](packages/plugin-legacy) | [![plugin-legacy version](https://img.shields.io/npm/v/@vitejs/plugin-legacy.svg?label=%20)](packages/plugin-legacy/CHANGELOG.md) |
| [create-vite](packages/create-vite)             | [![create-vite version](https://img.shields.io/npm/v/create-vite.svg?label=%20)](packages/create-vite/CHANGELOG.md)               |

## Contribution

See [Contributing Guide](CONTRIBUTING.md).

## License

[MIT](LICENSE).

## Sponsors

<p align="center">
  <a target="_blank" href="https://github.com/sponsors/yyx990803">
    <img alt="sponsors" src="https://sponsors.vuejs.org/vite.svg?v2">
  </a>
</p>

## [8.0.0-beta.15](https://github.com/vitejs/vite/compare/v8.0.0-beta.14...v8.0.0-beta.15) (2026-02-19)
### Features

* update rolldown to 1.0.0-rc.5 ([#21660](https://github.com/vitejs/vite/issues/21660)) ([b3ddbc5](https://github.com/vitejs/vite/commit/b3ddbc54ee5b836852b09811c8e920b2b2cde7cb))

### Bug Fixes

* **dev:** only treat EADDRINUSE as port conflict in wildcard pre-check ([#21642](https://github.com/vitejs/vite/issues/21642)) ([e54e25f](https://github.com/vitejs/vite/commit/e54e25fbb9b721b2c655d17e35706e070c92ff70))
* **dev:** prevent concurrent server restarts ([#21636](https://github.com/vitejs/vite/issues/21636)) ([8ce23a3](https://github.com/vitejs/vite/commit/8ce23a3b6e1eb86eef2b50c1bfbad072bbf9a03a))
* **dev:** return "502 Bad Gateway" on proxy failures instead of 500 ([#21652](https://github.com/vitejs/vite/issues/21652)) ([e240df2](https://github.com/vitejs/vite/commit/e240df2ea4accd11631aac0f361e846a2e3140b0))

### Performance Improvements

* **ssr:** skip circular import check for already-evaluated modules ([#21632](https://github.com/vitejs/vite/issues/21632)) ([235140b](https://github.com/vitejs/vite/commit/235140b2d519e866fc28f88fe8155a5091630daf))
* use tsconfig cache for oxc transform in dev ([#21643](https://github.com/vitejs/vite/issues/21643)) ([57ff177](https://github.com/vitejs/vite/commit/57ff177575bef6bee81a250e853d2c99affa0015))

### Miscellaneous Chores

* **deps:** remove `fdir` and `@rollup/plugin-commonjs`  ([#21639](https://github.com/vitejs/vite/issues/21639)) ([5abffd5](https://github.com/vitejs/vite/commit/5abffd5d04bf586a60970588a14d7e3b79445093))
* **deps:** update dependency @rollup/plugin-alias to v6 ([#21097](https://github.com/vitejs/vite/issues/21097)) ([44b5bdf](https://github.com/vitejs/vite/commit/44b5bdfcf2b2c1b73563ed0526c48584b756360f))

## [8.0.0-beta.14](https://github.com/vitejs/vite/compare/v8.0.0-beta.13...v8.0.0-beta.14) (2026-02-12)
### Features

* update rolldown to 1.0.0-rc.4 ([#21617](https://github.com/vitejs/vite/issues/21617)) ([1ee5c7f](https://github.com/vitejs/vite/commit/1ee5c7f796c24d7319fbd5258bbdce4968859efe))
* **wasm:** add SSR support for `.wasm?init` ([#21102](https://github.com/vitejs/vite/issues/21102)) ([216a3b5](https://github.com/vitejs/vite/commit/216a3b53c610918027a7713a0d5495628f77d306))

### Bug Fixes

* clear tsconfig cache only when tsconfig.json is cached ([#21622](https://github.com/vitejs/vite/issues/21622)) ([50c9675](https://github.com/vitejs/vite/commit/50c9675aa6c488b9887b7849a3397b7b29d1bd74))
* **deps:** update all non-major dependencies ([#21594](https://github.com/vitejs/vite/issues/21594)) ([becdc5d](https://github.com/vitejs/vite/commit/becdc5dcc49efa3769c92e9929fb2280fd776206))
* **lib:** CSS injection point error with nested name IIFE output ([#21606](https://github.com/vitejs/vite/issues/21606)) ([5003de6](https://github.com/vitejs/vite/commit/5003de6253ffdb23d1a52b1b5e06281d34f3a6ec))
* **module-runner:** incorrect column with `sourcemapInterceptor: "prepareStackTrace"` ([#21562](https://github.com/vitejs/vite/issues/21562)) ([416c095](https://github.com/vitejs/vite/commit/416c0959ebd63db622c6579b53065e95f09c63f8))
* **module-runner:** prevent crash on negative column in stacktrace ([#21585](https://github.com/vitejs/vite/issues/21585)) ([a075590](https://github.com/vitejs/vite/commit/a075590c4091240a6f0caca6b052500fd122f041))
* rolldownOptions/rollupOptions merging at environment level ([#21612](https://github.com/vitejs/vite/issues/21612)) ([db2ecc7](https://github.com/vitejs/vite/commit/db2ecc7675c3932fc9e127b726ab8b0cab25f75c))

### Miscellaneous Chores

* fix broken link for future deprecations ([#21603](https://github.com/vitejs/vite/issues/21603)) ([25f4501](https://github.com/vitejs/vite/commit/25f45013b94e50acc5c3e476691aa2210b33cae4))
* update `customResolver` deprecation message to mention `enforce: 'pre'` ([#21576](https://github.com/vitejs/vite/issues/21576)) ([2ce34d5](https://github.com/vitejs/vite/commit/2ce34d5580ed118db6361696e6283c1fea74e685))

### Code Refactoring

* enable some native plugins even with enable native plugin false ([#21608](https://github.com/vitejs/vite/issues/21608)) ([5a4f692](https://github.com/vitejs/vite/commit/5a4f6924260ef0f2683177a99935160badea3f3b))
* use `rolldown/utils` ([#21577](https://github.com/vitejs/vite/issues/21577)) ([e56103f](https://github.com/vitejs/vite/commit/e56103f180216306de738769303f31ad4c078b26))
* use internal devtools config ([#21609](https://github.com/vitejs/vite/issues/21609)) ([9aea20f](https://github.com/vitejs/vite/commit/9aea20f4a190e0e1c7edc656361d636cd6ce642f))
* use parseEnv ([#21586](https://github.com/vitejs/vite/issues/21586)) ([f859d2c](https://github.com/vitejs/vite/commit/f859d2cdfcc18f139775c208be068461a91602e5))
* **wasm:** remove native wasm helper plugin usage ([#21566](https://github.com/vitejs/vite/issues/21566)) ([71a86be](https://github.com/vitejs/vite/commit/71a86be6d9b9ea0329e92f20671f4db1f020874d))

### Tests

* test case for catching invalid package resolution error ([#21601](https://github.com/vitejs/vite/issues/21601)) ([c9b9359](https://github.com/vitejs/vite/commit/c9b9359fe88fc4b8a69a0d5c5a7eed8961fb6e57))

## [8.0.0-beta.13](https://github.com/vitejs/vite/compare/v8.0.0-beta.12...v8.0.0-beta.13) (2026-02-05)
### Features

* integrate devtools ([#21331](https://github.com/vitejs/vite/issues/21331)) ([acbf507](https://github.com/vitejs/vite/commit/acbf507bcb05f9cd9525c765431b3e0ed97328e4))
* update rolldown to 1.0.0-rc.3 ([#21554](https://github.com/vitejs/vite/issues/21554)) ([43358e9](https://github.com/vitejs/vite/commit/43358e97cd6485513f25ee11133333cba05841e3))

### Bug Fixes

* **scanner:** respect tsconfig.json ([#21547](https://github.com/vitejs/vite/issues/21547)) ([c6c04db](https://github.com/vitejs/vite/commit/c6c04db9c67d1b390d40fd1fd026d49204957f8d))

### Miscellaneous Chores

* update rolldown-plugin-dts to 0.22.1 ([#21559](https://github.com/vitejs/vite/issues/21559)) ([77aab4b](https://github.com/vitejs/vite/commit/77aab4b7f1e3a2131477659c909a3fbe02faa0a0))

### Code Refactoring

* deprecate `customResolver` in `resolve.alias` ([#21476](https://github.com/vitejs/vite/issues/21476)) ([81275c9](https://github.com/vitejs/vite/commit/81275c907211ac766013e6232c2cdf559534bed1))
* remove unnecessary `@rolldown/pluginutils` ([#21560](https://github.com/vitejs/vite/issues/21560)) ([c367b62](https://github.com/vitejs/vite/commit/c367b62693f19040e64d14915877f0b05b8ac7ae))

### Tests

* **bundled-dev:** add worker test cases ([#21557](https://github.com/vitejs/vite/issues/21557)) ([569bc98](https://github.com/vitejs/vite/commit/569bc98d6bc42fbd1835c1c24a493776030b6cb4))

## [8.0.0-beta.12](https://github.com/vitejs/vite/compare/v8.0.0-beta.11...v8.0.0-beta.12) (2026-02-03)
### Features

* **manifest:** add `assets` field for standalone CSS entry points ([#21015](https://github.com/vitejs/vite/issues/21015)) ([f289b9b](https://github.com/vitejs/vite/commit/f289b9b0ce7821b1554b878d083c426e7a695b59))

### Bug Fixes

* avoid registering customization hook for import meta resolver multiple times ([#21518](https://github.com/vitejs/vite/issues/21518)) ([8bb3203](https://github.com/vitejs/vite/commit/8bb32036792a6f522f5c947112f3d688add755a0))
* **config:** avoid watching rolldown runtime virtual module ([#21545](https://github.com/vitejs/vite/issues/21545)) ([d18b139](https://github.com/vitejs/vite/commit/d18b13957b3bec08eae5a9ff80340488c8150d46))
* **deps:** update all non-major dependencies ([#21540](https://github.com/vitejs/vite/issues/21540)) ([9ebaeaa](https://github.com/vitejs/vite/commit/9ebaeaac094db996b1d12665052633c20ac8a9cf))
* populate originalFileNames when resolving CSS asset paths ([#21542](https://github.com/vitejs/vite/issues/21542)) ([8b47ff7](https://github.com/vitejs/vite/commit/8b47ff76d28630b4dc39c77fbd2762b4c36ad23d))

### Miscellaneous Chores

* **deps:** update dependency rolldown-plugin-dts to ^0.21.8 ([#21539](https://github.com/vitejs/vite/issues/21539)) ([33881cb](https://github.com/vitejs/vite/commit/33881cb34f4587919713975d13ce255ef744472d))

## [8.0.0-beta.11](https://github.com/vitejs/vite/compare/v8.0.0-beta.10...v8.0.0-beta.11) (2026-01-29)
### Features

* update rolldown to 1.0.0-rc.2 ([#21512](https://github.com/vitejs/vite/issues/21512)) ([fa136a9](https://github.com/vitejs/vite/commit/fa136a9e68921f3ca396e0870193fe805fbfb7b4))

### Bug Fixes

* **deps:** update all non-major dependencies ([#21488](https://github.com/vitejs/vite/issues/21488)) ([2b32ca2](https://github.com/vitejs/vite/commit/2b32ca24fe9d742901c2cb5c88e6b1fd734f8c73))
* disable `tsconfig` option when loading config ([#21517](https://github.com/vitejs/vite/issues/21517)) ([5025c35](https://github.com/vitejs/vite/commit/5025c358d119aa0b60d0505f9dd705950ad897f6))
* **optimizer:** map relative `new URL` paths to correct relative file location ([#21434](https://github.com/vitejs/vite/issues/21434)) ([ca96cbc](https://github.com/vitejs/vite/commit/ca96cbc8eff23091c288f9eaf1944af2de3c564f))

### Documentation

* bulk of typo fixes ([#21507](https://github.com/vitejs/vite/issues/21507)) ([80755da](https://github.com/vitejs/vite/commit/80755dacab296cd2083fef29e09280ceb810a943))

### Miscellaneous Chores

* add missing versions to changelog ([#21515](https://github.com/vitejs/vite/issues/21515)) ([4bfb239](https://github.com/vitejs/vite/commit/4bfb239686a17343bc46c0d7c968e28b0d64041f))
* **deps:** update rolldown-related dependencies ([#21487](https://github.com/vitejs/vite/issues/21487)) ([5863e51](https://github.com/vitejs/vite/commit/5863e513fab6b481cfb42da86202f9db728c077d))

### Code Refactoring

* enable some native plugins even with enable native plugin false ([#21511](https://github.com/vitejs/vite/issues/21511)) ([b40292c](https://github.com/vitejs/vite/commit/b40292ce6a7dbbbbac9c6dae5f126b7f44c3e1b7))
* remove `experimental.enableNativePlugin: 'resolver'` ([#21510](https://github.com/vitejs/vite/issues/21510)) ([f9d9213](https://github.com/vitejs/vite/commit/f9d92130fa79c638f77a3a8e6e55506f185d5a49))
* use `import.meta.dirname` everywhere ([#21509](https://github.com/vitejs/vite/issues/21509)) ([7becf5f](https://github.com/vitejs/vite/commit/7becf5f8fe9041cff60f495ef975faaba68f9eb2))

## [8.0.0-beta.10](https://github.com/vitejs/vite/compare/v8.0.0-beta.9...v8.0.0-beta.10) (2026-01-24)
### Bug Fixes

* avoid using deprecated `output.inlineDynamicImport` option ([#21464](https://github.com/vitejs/vite/issues/21464)) ([471ce62](https://github.com/vitejs/vite/commit/471ce6275663f068afa241a55711fd646d482385))
* use separate hook object for each environment ([#21472](https://github.com/vitejs/vite/issues/21472)) ([66347f6](https://github.com/vitejs/vite/commit/66347f6df0e723d9d03ea31ab41ab5b767ad15ba))

### Documentation

* update `build.dynamicImportVarsOptions` ([#21477](https://github.com/vitejs/vite/issues/21477)) ([54ce2ed](https://github.com/vitejs/vite/commit/54ce2ed15a95619bd18ac6609b7d7b5f42b4965d))

## [8.0.0-beta.9](https://github.com/vitejs/vite/compare/v8.0.0-beta.8...v8.0.0-beta.9) (2026-01-22)
### Features

* **bundled-dev:** support worker in initial bundle ([#21415](https://github.com/vitejs/vite/issues/21415)) ([f3d3149](https://github.com/vitejs/vite/commit/f3d31499c714fe5c5acf8355520624c662f9d79f))
* **dev:** detect port conflicts on wildcard hosts ([#21381](https://github.com/vitejs/vite/issues/21381)) ([b0dd5a9](https://github.com/vitejs/vite/commit/b0dd5a993fd2f95c8cb2190a3ca4296bc9e06359))
* shortcuts case insensitive ([#21224](https://github.com/vitejs/vite/issues/21224)) ([7796ade](https://github.com/vitejs/vite/commit/7796aded764bca987abfec8ab0ad0438c5a5e7eb))
* update rolldown to 1.0.0-rc.1 ([#21463](https://github.com/vitejs/vite/issues/21463)) ([ff9dd7f](https://github.com/vitejs/vite/commit/ff9dd7fef0d3c898e317fca84a629828f3e28936))
* warn if `envPrefix` contains spaces ([#21292](https://github.com/vitejs/vite/issues/21292)) ([9fcde3c](https://github.com/vitejs/vite/commit/9fcde3c870896a62fbca19be8ee14efab9393f4a))

### Bug Fixes

* **deps:** update all non-major dependencies ([#21440](https://github.com/vitejs/vite/issues/21440)) ([1835995](https://github.com/vitejs/vite/commit/18359959cb2960a2fb2b9a340e5ae27d122a1501))
* **dev:** avoid event emitter leak caused by `server.listen` callback ([#21451](https://github.com/vitejs/vite/issues/21451)) ([602d786](https://github.com/vitejs/vite/commit/602d7865db2b12835c8225f3e87076bef4e247b9))
* lazy hook filter should work ([#21443](https://github.com/vitejs/vite/issues/21443)) ([bc0c207](https://github.com/vitejs/vite/commit/bc0c207f537789d10d55caa4ee3697aa923b8426))
* **optimizer:** skip `rolldownCjsExternalPlugin` for `platform: neutral` ([#21452](https://github.com/vitejs/vite/issues/21452)) ([d2fc4be](https://github.com/vitejs/vite/commit/d2fc4be0447e384e18e557b70f7c345d5bcea941))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#21390](https://github.com/vitejs/vite/issues/21390)) ([be9dd4e](https://github.com/vitejs/vite/commit/be9dd4e08d899f9ed27f2bdcb81bf27d018377a6))
* fix typo in plugin.ts comment ([#21435](https://github.com/vitejs/vite/issues/21435)) ([d31fc66](https://github.com/vitejs/vite/commit/d31fc6685b4dde33062bf4dfe46e0502de4e1449))

### Code Refactoring

* **optimizer:** simplify `rolldownCjsExternalPlugin` ([#21450](https://github.com/vitejs/vite/issues/21450)) ([ebda8fd](https://github.com/vitejs/vite/commit/ebda8fd3c14f60e63d13d22102cb3d79a12f47a9))

## [8.0.0-beta.8](https://github.com/vitejs/vite/compare/v8.0.0-beta.7...v8.0.0-beta.8) (2026-01-15)
### ⚠ BREAKING CHANGES

* remove `import.meta.hot.accept` resolution fallback (#21382)

### Features

* update rolldown to 1.0.0-beta.60 ([#21408](https://github.com/vitejs/vite/issues/21408)) ([c33aa7c](https://github.com/vitejs/vite/commit/c33aa7cfd142a0dd38ed89589fc7b04cf8866791))

### Bug Fixes

* **deps:** update all non-major dependencies ([#21389](https://github.com/vitejs/vite/issues/21389)) ([30f48df](https://github.com/vitejs/vite/commit/30f48df33ec9e9bd0b8164461eede5574398370b))
* **deps:** update esbuild peerDependency version ([#21398](https://github.com/vitejs/vite/issues/21398)) ([4266c97](https://github.com/vitejs/vite/commit/4266c978083b3afa8d09ac3d3a110ee79f8efde2))
* **hmr:** trigger prune event when last import is removed ([#20781](https://github.com/vitejs/vite/issues/20781)) ([#21093](https://github.com/vitejs/vite/issues/21093)) ([7576735](https://github.com/vitejs/vite/commit/757673528c64945b77aee4a8e01669ccd0644973))
* **module-runner:** use `process.getBuiltinModule` instead of `import('node:module')` ([#21402](https://github.com/vitejs/vite/issues/21402)) ([6633bcb](https://github.com/vitejs/vite/commit/6633bcb94149a2923cb6419aa481c5384bcf9310))
* support .env file mounts (FIFOs) ([#21365](https://github.com/vitejs/vite/issues/21365)) ([6e6f82a](https://github.com/vitejs/vite/commit/6e6f82a067acc6e158be3b82edb3d7d2888f9af2))

### Code Refactoring

* remove `import.meta.hot.accept` resolution fallback ([#21382](https://github.com/vitejs/vite/issues/21382)) ([71d0797](https://github.com/vitejs/vite/commit/71d0797a719440f2a09b3364bfcf18576c2b67fb))

## [8.0.0-beta.7](https://github.com/vitejs/vite/compare/v8.0.0-beta.6...v8.0.0-beta.7) (2026-01-08)
### Features

* update rolldown to 1.0.0-beta.59 ([#21374](https://github.com/vitejs/vite/issues/21374)) ([0037943](https://github.com/vitejs/vite/commit/00379439fa62383460b056d587d0366597c19ab4))

### Bug Fixes

* **css:** stylus Evaluator support ([#21376](https://github.com/vitejs/vite/issues/21376)) ([cf9ace1](https://github.com/vitejs/vite/commit/cf9ace1b40b2767b9b9cbbabb084fe2e32afc535))

## [8.0.0-beta.6](https://github.com/vitejs/vite/compare/v8.0.0-beta.5...v8.0.0-beta.6) (2026-01-07)
### Features

* add `ignoreOutdatedRequests` option to `optimizeDeps` ([#21364](https://github.com/vitejs/vite/issues/21364)) ([b2e75aa](https://github.com/vitejs/vite/commit/b2e75aabe93e3219f40fa5ad8755d53cdd2439b5))
* add ios to default esbuild targets ([#21342](https://github.com/vitejs/vite/issues/21342)) ([daae6e9](https://github.com/vitejs/vite/commit/daae6e9f5dd223258a9e7a9a7fa22c8a4564902f))
* update rolldown to 1.0.0-beta.58 ([#21354](https://github.com/vitejs/vite/issues/21354)) ([ba40cef](https://github.com/vitejs/vite/commit/ba40cef16d20590f7115d4d628d9b79fa0783473))

### Bug Fixes

* **deps:** update all non-major dependencies ([#21321](https://github.com/vitejs/vite/issues/21321)) ([9bc7c2e](https://github.com/vitejs/vite/commit/9bc7c2ed4f387fb982b84d1988a26af8990096f7))
* **import-analysis:** avoid cjs interop for built browser external module ([#21333](https://github.com/vitejs/vite/issues/21333)) ([dc5a2fb](https://github.com/vitejs/vite/commit/dc5a2fb86f10c69b0ba6bc1831d9a29c79754ba2))

### Miscellaneous Chores

* replace caniuse link for ES2024 ([#21355](https://github.com/vitejs/vite/issues/21355)) ([2ba4e99](https://github.com/vitejs/vite/commit/2ba4e990192845e01c733aa186c9599cdb5bb8fe))

## [8.0.0-beta.5](https://github.com/vitejs/vite/compare/v8.0.0-beta.4...v8.0.0-beta.5) (2025-12-25)
### Features

* update rolldown to 1.0.0-beta.57 ([#21335](https://github.com/vitejs/vite/issues/21335)) ([d5412ef](https://github.com/vitejs/vite/commit/d5412ef4c472bc5fef4ed69cfee4ef4a929c6be9))

### Bug Fixes

* **worker:** handle `new Worker(..., new URL(import.meta.url))` with trailing comma ([#21325](https://github.com/vitejs/vite/issues/21325)) ([4a47241](https://github.com/vitejs/vite/commit/4a472418c02a0821900678778752c2d361bae3bd))

### Code Refactoring

* **optimizer:** remove dead code ([#21334](https://github.com/vitejs/vite/issues/21334)) ([e9a2cdb](https://github.com/vitejs/vite/commit/e9a2cdbb7d96a3f8e15d25774708d4f4ab626bb9))

## [8.0.0-beta.4](https://github.com/vitejs/vite/compare/v8.0.0-beta.3...v8.0.0-beta.4) (2025-12-22)
### Features

* **css:** support es2024 build target for lightningcss ([#21294](https://github.com/vitejs/vite/issues/21294)) ([bd33b8e](https://github.com/vitejs/vite/commit/bd33b8e08768fdcef0b09e3eefa649fdcafdd397))
* update rolldown to 1.0.0-beta.56 ([#21323](https://github.com/vitejs/vite/issues/21323)) ([9847a63](https://github.com/vitejs/vite/commit/9847a634cf36de2e6ac0043ffd22cefb1b5951bd))

### Bug Fixes

* detect `import.meta.resolve` when formatted across multiple lines ([#21312](https://github.com/vitejs/vite/issues/21312)) ([130e718](https://github.com/vitejs/vite/commit/130e7181a55c524383c63bbfb1749d0ff7185cad))

## [8.0.0-beta.3](https://github.com/vitejs/vite/compare/v8.0.0-beta.2...v8.0.0-beta.3) (2025-12-18)
### Features

* introduce v2 native plugins and enable it by default ([#21268](https://github.com/vitejs/vite/issues/21268)) ([42f2ab3](https://github.com/vitejs/vite/commit/42f2ab3aec7cd0e03e195611b1e1ddabbedc9d61))
* **ssr:** avoid errors when rewriting already rewritten stacktrace ([#21269](https://github.com/vitejs/vite/issues/21269)) ([98d9a33](https://github.com/vitejs/vite/commit/98d9a33274d9ac90780786afa612d916feddf2e3))
* update rolldown to 1.0.0-beta.55 ([#21300](https://github.com/vitejs/vite/issues/21300)) ([2c8db85](https://github.com/vitejs/vite/commit/2c8db858d7081e898f63ce9569c3f19a91a10956))

### Bug Fixes

* allow no-cors requests for non-script tag requests ([#21299](https://github.com/vitejs/vite/issues/21299)) ([ef3d596](https://github.com/vitejs/vite/commit/ef3d59648fd9dd3f9b3118d09d216dc0afcb8c33))
* **deps:** update all non-major dependencies ([#21285](https://github.com/vitejs/vite/issues/21285)) ([4635b2e](https://github.com/vitejs/vite/commit/4635b2e90f833d1048d76381e20208c0e0841e97))
* unreachable error when building with `experimental.bundledDev` is enabled ([#21296](https://github.com/vitejs/vite/issues/21296)) ([e81c183](https://github.com/vitejs/vite/commit/e81c183f8c8ccaf7774ef0d0ee125bf63dbf30b4))

## [8.0.0-beta.2](https://github.com/vitejs/vite/compare/v8.0.0-beta.1...v8.0.0-beta.2) (2025-12-12)
### Features

* update rolldown to 1.0.0-beta.54 ([#21267](https://github.com/vitejs/vite/issues/21267)) ([c751172](https://github.com/vitejs/vite/commit/c75117213cb1d2d13554fbc26a75e8df191c27eb))

### Bug Fixes

* **deps:** update all non-major dependencies ([#21231](https://github.com/vitejs/vite/issues/21231)) ([859789c](https://github.com/vitejs/vite/commit/859789c856412dfa67969232ddda1df754febf40))
* don't strip base from imports ([#21221](https://github.com/vitejs/vite/issues/21221)) ([7da742b](https://github.com/vitejs/vite/commit/7da742b478d2309c7d8de4cb55614a6476f350b4))

### Documentation

* clarify the pronunciation of `vite` in IPA symbols ([#21238](https://github.com/vitejs/vite/issues/21238)) ([9b1d4d6](https://github.com/vitejs/vite/commit/9b1d4d6f348c8899bd7651bd802f583e99b901ee))
* ensure https links ([#21266](https://github.com/vitejs/vite/issues/21266)) ([2eb259a](https://github.com/vitejs/vite/commit/2eb259a84859c7656718258afed08eb80670f530))

## [8.0.0-beta.1](https://github.com/vitejs/vite/compare/v8.0.0-beta.0...v8.0.0-beta.1) (2025-12-08)
### Features

* add a warning that is output when a plugin sets esbuild related options ([#21218](https://github.com/vitejs/vite/issues/21218)) ([200646b](https://github.com/vitejs/vite/commit/200646b14397bfb80e9b29d2e4b33fcfc72d6b2c))
* highly experimental full bundle mode ([#21235](https://github.com/vitejs/vite/issues/21235)) ([83d8c99](https://github.com/vitejs/vite/commit/83d8c99753d8bd5c1ea9b7a00e6998c865dad4e2))
* print esbuild options when both esbuild and oxc options are set ([#21216](https://github.com/vitejs/vite/issues/21216)) ([08ae87b](https://github.com/vitejs/vite/commit/08ae87b14a3ce5f7cb3f1a382f497d36d0c2e01b))

### Bug Fixes

* allow exiting process before optimizer cleanup is done ([#21170](https://github.com/vitejs/vite/issues/21170)) ([55ceffc](https://github.com/vitejs/vite/commit/55ceffc8976b8bb8c819f5b47419f8499ba3f843))
* plugin shortcut support ([#21211](https://github.com/vitejs/vite/issues/21211)) ([6a3aca0](https://github.com/vitejs/vite/commit/6a3aca084356316811ff62cbedb5a410a249e789))

### Miscellaneous Chores

* cleanup changelog ([#21202](https://github.com/vitejs/vite/issues/21202)) ([8c8c56e](https://github.com/vitejs/vite/commit/8c8c56e1eb465e6dcd0c1b40f187228edc0e2be4))
* **deps:** update rolldown-related dependencies ([#21230](https://github.com/vitejs/vite/issues/21230)) ([9349446](https://github.com/vitejs/vite/commit/9349446e9344bd81ccfb37af482f479cd1b59bbc))
* fix spelling error ([#21223](https://github.com/vitejs/vite/issues/21223)) ([cc10e20](https://github.com/vitejs/vite/commit/cc10e207ae87ac122fc1efbb5ab01b516eb9cce8))

## [8.0.0-beta.0](https://github.com/vitejs/vite/compare/v7.2.6...v8.0.0-beta.0) (2025-12-03)
### ⚠ BREAKING CHANGES

* update default browser target (#21193)
* the epic `rolldown-vite` merge (#21189)

### Features

* update default browser target ([#21193](https://github.com/vitejs/vite/issues/21193)) ([8c3dd06](https://github.com/vitejs/vite/commit/8c3dd06bd9903bf0e6bc51f3554eea8cb6b26903))
* the epic `rolldown-vite` merge ([#21189](https://github.com/vitejs/vite/issues/21189)) ([4a7f8d4](https://github.com/vitejs/vite/commit/4a7f8d43e6b14b89fef278c3ea86f9e3f64b7fc2))

### Rolldown-Vite changelogs

See [rolldown-vite changelog](https://github.com/vitejs/rolldown-vite/blob/v7.2.10/packages/vite/CHANGELOG.md)

## <small>[7.3.1](https://github.com/vitejs/vite/compare/v7.3.0...v7.3.1) (2026-01-07)</small>
### Features

* add `ignoreOutdatedRequests` option to `optimizeDeps` ([#21364](https://github.com/vitejs/vite/issues/21364)) ([9d39d37](https://github.com/vitejs/vite/commit/9d39d373a7b4e0a93322b70b9dbeb202af06af3e))

## [7.3.0](https://github.com/vitejs/vite/compare/v7.2.7...v7.3.0) (2025-12-15)
### Features

* **deps:** update esbuild from ^0.25.0 to ^0.27.0 ([#21183](https://github.com/vitejs/vite/issues/21183)) ([cff26ec](https://github.com/vitejs/vite/commit/cff26ec0fc13373d7125a5eac6cb01fe63fee4b1))

## <small>[7.2.7](https://github.com/vitejs/vite/compare/v7.2.6...v7.2.7) (2025-12-08)</small>
### Bug Fixes

* plugin shortcut support ([#21211](https://github.com/vitejs/vite/issues/21211)) ([721f163](https://github.com/vitejs/vite/commit/721f16343d9555ae8fc71a2e5354b22e12ff0dc3))

## <small>[7.2.6](https://github.com/vitejs/vite/compare/v7.2.5...v7.2.6) (2025-12-01)</small>
## <small>[7.2.5](https://github.com/vitejs/vite/compare/v7.2.4...v7.2.5) (2025-12-01)</small>
### Bug Fixes

* **config:** handle shebang properly ([#21158](https://github.com/vitejs/vite/issues/21158)) ([df5a30d](https://github.com/vitejs/vite/commit/df5a30d2690a2ebc4824a79becdcef30538dc602))
* **deps:** update all non-major dependencies ([#21146](https://github.com/vitejs/vite/issues/21146)) ([a3cd262](https://github.com/vitejs/vite/commit/a3cd262f37228967e455617e982b35fccc49ffe9))
* **deps:** update all non-major dependencies ([#21175](https://github.com/vitejs/vite/issues/21175)) ([72e398a](https://github.com/vitejs/vite/commit/72e398a46d8d2f54fbcbeb9ff0dceab346aeb642))
* fix `external: true` merging ([#21164](https://github.com/vitejs/vite/issues/21164)) ([5ef557a](https://github.com/vitejs/vite/commit/5ef557a96c4a1f2b3a3aa25c12df3ee87b4a03f5))
* shortcuts not rebound after server restart ([#21166](https://github.com/vitejs/vite/issues/21166)) ([3765f7b](https://github.com/vitejs/vite/commit/3765f7baea36234bf3816eeed38776d27bfd3649))

### Performance Improvements

* **deps:** replace debug with obug ([#21137](https://github.com/vitejs/vite/issues/21137)) ([203a551](https://github.com/vitejs/vite/commit/203a5512a42a1031f685993f5d9cbae5f328354f))

### Documentation

* clarify manifest.json `imports` field is JS chunks only ([#21136](https://github.com/vitejs/vite/issues/21136)) ([46d3077](https://github.com/vitejs/vite/commit/46d3077f2b63771cc50230bc907c48f5773c00fb))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#21174](https://github.com/vitejs/vite/issues/21174)) ([74559c9](https://github.com/vitejs/vite/commit/74559c947483a8ee24da052ac2d9568f7cb3546a))

## <small>[7.2.4](https://github.com/vitejs/vite/compare/v7.2.3...v7.2.4) (2025-11-20)</small>
### Bug Fixes

* revert "perf(deps): replace debug with obug ([#21107](https://github.com/vitejs/vite/issues/21107))" ([2d66b7b](https://github.com/vitejs/vite/commit/2d66b7b14aa6dfd62f3d6a59ee8382ed5ca6fd32))

## <small>[7.2.3](https://github.com/vitejs/vite/compare/v7.2.2...v7.2.3) (2025-11-20)</small>
### Bug Fixes

* allow multiple `bindCLIShortcuts` calls with shortcut merging ([#21103](https://github.com/vitejs/vite/issues/21103)) ([5909efd](https://github.com/vitejs/vite/commit/5909efd8fbfd1bf1eab65427aea0613124b2797a))
* **deps:** update all non-major dependencies ([#21096](https://github.com/vitejs/vite/issues/21096)) ([6a34ac3](https://github.com/vitejs/vite/commit/6a34ac3422686e7cf7cc9a25d299cb8e5a8d92a0))
* **deps:** update all non-major dependencies ([#21128](https://github.com/vitejs/vite/issues/21128)) ([4f8171e](https://github.com/vitejs/vite/commit/4f8171eb3046bd70c83964689897dab4c6b58bc0))

### Performance Improvements

* **deps:** replace debug with obug ([#21107](https://github.com/vitejs/vite/issues/21107)) ([acfe939](https://github.com/vitejs/vite/commit/acfe939e1f7c303c34b0b39b883cc302da767fa2))

### Miscellaneous Chores

* **deps:** update dependency @rollup/plugin-commonjs to v29 ([#21099](https://github.com/vitejs/vite/issues/21099)) ([02ceaec](https://github.com/vitejs/vite/commit/02ceaec45e17bef19159188a28d9196fed1761be))
* **deps:** update rolldown-related dependencies ([#21095](https://github.com/vitejs/vite/issues/21095)) ([39a0a15](https://github.com/vitejs/vite/commit/39a0a15fd24ed37257c48b795097a3794e54d255))
* **deps:** update rolldown-related dependencies ([#21127](https://github.com/vitejs/vite/issues/21127)) ([5029720](https://github.com/vitejs/vite/commit/50297208452241061cb44d09a4bbdf77a11ac01e))

## <small>[7.2.2](https://github.com/vitejs/vite/compare/v7.2.1...v7.2.2) (2025-11-07)</small>
### Bug Fixes

* revert "refactor: use fs.cpSync ([#21019](https://github.com/vitejs/vite/issues/21019))" ([#21081](https://github.com/vitejs/vite/issues/21081)) ([728c8ee](https://github.com/vitejs/vite/commit/728c8eeebc0ad7ba48e680f46bbdb55020b2e152))

## <small>[7.2.1](https://github.com/vitejs/vite/compare/v7.2.0...v7.2.1) (2025-11-06)</small>
### Bug Fixes

* **worker:** some worker asset was missing ([#21074](https://github.com/vitejs/vite/issues/21074)) ([82d2d6c](https://github.com/vitejs/vite/commit/82d2d6ccf5a69c3d67e44e3704a0c3eb34db2236))

### Code Refactoring

* **build:** rename `indexOfMatchInSlice` to `findPreloadMarker` ([#21054](https://github.com/vitejs/vite/issues/21054)) ([f83264f](https://github.com/vitejs/vite/commit/f83264f5d97caa8ba80276a9ee9e82cb0e69c4ca))

## [7.2.0](https://github.com/vitejs/vite/compare/v7.2.0-beta.1...v7.2.0) (2025-11-05)
### Bug Fixes

* **css:** fallback to sass when sass-embedded platform binary is missing ([#21002](https://github.com/vitejs/vite/issues/21002)) ([b1fd616](https://github.com/vitejs/vite/commit/b1fd6161886caeb31ac646d6544116d37efe46d0))
* **module-runner:** make `getBuiltins` response JSON serializable ([#21029](https://github.com/vitejs/vite/issues/21029)) ([ad5b3bf](https://github.com/vitejs/vite/commit/ad5b3bf6f3ad7b24886718c5f5de32eee923ae11))
* **types:** add undefined to optional properties for exactOptionalProperties type compatibility ([#21040](https://github.com/vitejs/vite/issues/21040)) ([2833c55](https://github.com/vitejs/vite/commit/2833c5576a87be2db450c195ccf64dfc8925a15b))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#21047](https://github.com/vitejs/vite/issues/21047)) ([e3a6a83](https://github.com/vitejs/vite/commit/e3a6a83406943bc59a9916cae3f25ab33c2b5802))

## [7.2.0-beta.1](https://github.com/vitejs/vite/compare/v7.2.0-beta.0...v7.2.0-beta.1) (2025-10-29)
### Bug Fixes

* increase stream reset rate limit for HTTP2 ([#21024](https://github.com/vitejs/vite/issues/21024)) ([4f44f22](https://github.com/vitejs/vite/commit/4f44f22f7f4595d74c76778bd522387138775055))
* **optimizer:** externalize virtual modules for html like files ([#21001](https://github.com/vitejs/vite/issues/21001)) ([e5af352](https://github.com/vitejs/vite/commit/e5af352d8e1a9f187159137f836db5bedbd68a66))

### Documentation

* clarify the values are escaped automatically ([#21017](https://github.com/vitejs/vite/issues/21017)) ([246df13](https://github.com/vitejs/vite/commit/246df134dd58441e1e40dd361cf42419d05ea7a5))

### Code Refactoring

* use `fs.cpSync` ([#21019](https://github.com/vitejs/vite/issues/21019)) ([a2df778](https://github.com/vitejs/vite/commit/a2df77812814b927880bc4d68aafa8c8fa47daf0))

## [7.2.0-beta.0](https://github.com/vitejs/vite/compare/v7.1.11...v7.2.0-beta.0) (2025-10-28)
### Features

* add `import.meta.resolve` support for ESM config (bundle config loader) ([#20962](https://github.com/vitejs/vite/issues/20962)) ([f86789a](https://github.com/vitejs/vite/commit/f86789a6e237bd0e31cde3a3f09bdef45bfa7d1c))
* add `perEnvironmentWatchChangeDuringDev` ([#20996](https://github.com/vitejs/vite/issues/20996)) ([a5e98e6](https://github.com/vitejs/vite/commit/a5e98e695ee4152127977abb506029dc8f7544fb))
* add vite client connect events ([#20978](https://github.com/vitejs/vite/issues/20978)) ([543d87c](https://github.com/vitejs/vite/commit/543d87c2cd1ec629f19de56a903a15185f20db1f))
* **build:** emit license ([#18546](https://github.com/vitejs/vite/issues/18546)) ([b42c3fb](https://github.com/vitejs/vite/commit/b42c3fb2cb75bb4fdf7557cb35946564d6dc4384))
* **dev:** support HTTP2 even if proxy feature is used ([#20869](https://github.com/vitejs/vite/issues/20869)) ([fc21af7](https://github.com/vitejs/vite/commit/fc21af7a42dd559a95f54b6165d34f36883eaa7f))
* **lib:** enable minification but keep pure annotations for es output with terser ([#20522](https://github.com/vitejs/vite/issues/20522)) ([df997d0](https://github.com/vitejs/vite/commit/df997d0cfca8e1dad04ac1bf8119caa2d2e4c1fc))
* **optimizer:** add rush lockfile support ([#20833](https://github.com/vitejs/vite/issues/20833)) ([718ca2d](https://github.com/vitejs/vite/commit/718ca2d708dbeb393839932437a6b161851ca24c))
* **utils:** support multiple certificates in resolveServerUrls ([#20707](https://github.com/vitejs/vite/issues/20707)) ([24513e5](https://github.com/vitejs/vite/commit/24513e567c643d5f6fb61af6298aa3fc2b166b90))

### Bug Fixes

* **build:** ensure amd bundles request `require` to be injected ([#20861](https://github.com/vitejs/vite/issues/20861)) ([bb85bd7](https://github.com/vitejs/vite/commit/bb85bd751e4568c707612b708deaba67f8af4ca3))
* **build:** replace `names` in the manifest with unmangled `name` for CSS assets ([#20585](https://github.com/vitejs/vite/issues/20585)) ([4abf056](https://github.com/vitejs/vite/commit/4abf0566024a70c38a0eb5bf614f72189038247d))
* **deps:** downgrade commonjs plugin to 28.0.6 to avoid rollup/plugins[#1909](https://github.com/vitejs/vite/issues/1909) ([#20988](https://github.com/vitejs/vite/issues/20988)) ([856e683](https://github.com/vitejs/vite/commit/856e683885ed53ec6044897451608bc6518baef6))
* **deps:** update all non-major dependencies ([#21008](https://github.com/vitejs/vite/issues/21008)) ([185641e](https://github.com/vitejs/vite/commit/185641e3cdae29277c41eb8028f6eac542215f01))
* disable optional peer dep handling for `nodeResolveWithVite` ([#20989](https://github.com/vitejs/vite/issues/20989)) ([ca18b23](https://github.com/vitejs/vite/commit/ca18b233d43a8f31883726ca565940ad1dc85f38))
* handle query parameters for `/@vite/*` modules ([#20998](https://github.com/vitejs/vite/issues/20998)) ([6843a6a](https://github.com/vitejs/vite/commit/6843a6ae49df8ca523104a8ccfb9a8f9602b3881))
* **module-runner:** resolve `resolvedSources` correctly ([#20959](https://github.com/vitejs/vite/issues/20959)) ([c4f6039](https://github.com/vitejs/vite/commit/c4f6039436657db50c610aa17eaf821dbd4ad57d))
* **resolve:** match resolved subpath import path's relative prefix with regex (fix [#20972](https://github.com/vitejs/vite/issues/20972)) ([#20973](https://github.com/vitejs/vite/issues/20973)) ([ff2d83e](https://github.com/vitejs/vite/commit/ff2d83e2e7a3f7eba72f41b40686912f1e4b6843))
* update build log to include environment name ([#20987](https://github.com/vitejs/vite/issues/20987)) ([77c25c1](https://github.com/vitejs/vite/commit/77c25c16ba9f3568e55fd4135f57c70f984d3fdd))
* use esm entrypoint for css preprocessors and terser ([#20918](https://github.com/vitejs/vite/issues/20918)) ([1460824](https://github.com/vitejs/vite/commit/14608241cc4c821e7a392f6d92ef291a926bd94d))

### Performance Improvements

* **module-runner:** add client-side builtin module check ([#20924](https://github.com/vitejs/vite/issues/20924)) ([ccffead](https://github.com/vitejs/vite/commit/ccffead5f8340bf9e7d88fed4b3c0ac5a74f5926))
* **module-runner:** use `module.registerHooks` when available ([#20980](https://github.com/vitejs/vite/issues/20980)) ([9c8a780](https://github.com/vitejs/vite/commit/9c8a7801c5ad29da4c3822c414223593c526e855))
* run `dev.createEnvironment` and `build.createEnvironment` concurrently ([#20699](https://github.com/vitejs/vite/issues/20699)) ([d636220](https://github.com/vitejs/vite/commit/d6362207ba817a210cbcea1253a174542a6c8c68))
* **worker:** rebuild only when affected ([#20559](https://github.com/vitejs/vite/issues/20559)) ([5cd3890](https://github.com/vitejs/vite/commit/5cd38906afe1fab4ec967b18b21c05a80ac49c92))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20965](https://github.com/vitejs/vite/issues/20965)) ([6ad5424](https://github.com/vitejs/vite/commit/6ad5424f0bcabd8943c34d341cf9ed2e64b3d53c))
* **deps:** update rolldown-related dependencies ([#21009](https://github.com/vitejs/vite/issues/21009)) ([7cec1ad](https://github.com/vitejs/vite/commit/7cec1addc257eb5bd0856adbe1e4306f0bb83e22))
* fix merge error ([075caa0](https://github.com/vitejs/vite/commit/075caa065bab659f45411cfe9b3b02e23ebe5406))

### Code Refactoring

* remove `listenersForEvents` variable ([#20977](https://github.com/vitejs/vite/issues/20977)) ([4d01112](https://github.com/vitejs/vite/commit/4d01112415a9c3d06f2fe143a9987e50c4d47f36))
* remove unused `invoke` parameter ([#20991](https://github.com/vitejs/vite/issues/20991)) ([29cdb39](https://github.com/vitejs/vite/commit/29cdb390374689e4dec9017b21fefe88b6ce4203))

### Build System

* normalize license repo url ([#21010](https://github.com/vitejs/vite/issues/21010)) ([bb65468](https://github.com/vitejs/vite/commit/bb65468f3c6213fdd870c085610ec575af66b03c))
* use isolated declarations ([#20928](https://github.com/vitejs/vite/issues/20928)) ([dad7643](https://github.com/vitejs/vite/commit/dad76436e96e1bac2baa805a0a3c1861e88e050c))

## <small>[7.1.11](https://github.com/vitejs/vite/compare/v7.1.10...v7.1.11) (2025-10-20)</small>
### Bug Fixes

* **dev:** trim trailing slash before `server.fs.deny` check ([#20968](https://github.com/vitejs/vite/issues/20968)) ([f479cc5](https://github.com/vitejs/vite/commit/f479cc57c425ed41ceb434fecebd63931b1ed4ed))

### Miscellaneous Chores

* **deps:** update all non-major dependencies ([#20966](https://github.com/vitejs/vite/issues/20966)) ([6fb41a2](https://github.com/vitejs/vite/commit/6fb41a260bda443685e719ea4765d3faca3db944))

### Code Refactoring

* use subpath imports for types module reference ([#20921](https://github.com/vitejs/vite/issues/20921)) ([d0094af](https://github.com/vitejs/vite/commit/d0094af639d9ebbb51d4e00910b74f23eb8fe131))

### Build System

* remove cjs reference in files field ([#20945](https://github.com/vitejs/vite/issues/20945)) ([ef411ce](https://github.com/vitejs/vite/commit/ef411cee2696af3ba791879fdae9aad165f178b2))
* remove hash from built filenames ([#20946](https://github.com/vitejs/vite/issues/20946)) ([a817307](https://github.com/vitejs/vite/commit/a81730754d655d1371ce0f4354af1c84e12f9f2d))

## <small>[7.1.10](https://github.com/vitejs/vite/compare/v7.1.9...v7.1.10) (2025-10-14)</small>
### Bug Fixes

* **css:** avoid duplicate style for server rendered stylesheet link and client inline style during dev ([#20767](https://github.com/vitejs/vite/issues/20767)) ([3a92bc7](https://github.com/vitejs/vite/commit/3a92bc79b306a01b8aaf37f80b2239eaf6e488e7))
* **css:** respect emitAssets when cssCodeSplit=false ([#20883](https://github.com/vitejs/vite/issues/20883)) ([d3e7eee](https://github.com/vitejs/vite/commit/d3e7eeefa91e1992f47694d16fe4dbe708c4d80e))
* **deps:** update all non-major dependencies ([879de86](https://github.com/vitejs/vite/commit/879de86935a31b4e47ab907ddd859366518ce268))
* **deps:** update all non-major dependencies ([#20894](https://github.com/vitejs/vite/issues/20894)) ([3213f90](https://github.com/vitejs/vite/commit/3213f90ff0d8f274bcec65f40aac6dfcff1ac244))
* **dev:** allow aliases starting with `//` ([#20760](https://github.com/vitejs/vite/issues/20760)) ([b95fa2a](https://github.com/vitejs/vite/commit/b95fa2aa7564eda4c9f05ee7616a2dbada35e463))
* **dev:** remove timestamp query consistently ([#20887](https://github.com/vitejs/vite/issues/20887)) ([6537d15](https://github.com/vitejs/vite/commit/6537d15591619d7e1cfc1e50599bec16cd88340f))
* **esbuild:** inject esbuild helpers correctly for esbuild 0.25.9+ ([#20906](https://github.com/vitejs/vite/issues/20906)) ([446eb38](https://github.com/vitejs/vite/commit/446eb386329ef682d614c77958a542f2dc222880))
* normalize path before calling `fileToBuiltUrl` ([#20898](https://github.com/vitejs/vite/issues/20898)) ([73b6d24](https://github.com/vitejs/vite/commit/73b6d243e0398ee5d8d44c7d24162f4a0f4b1cf1))
* preserve original sourcemap file field when combining sourcemaps ([#20926](https://github.com/vitejs/vite/issues/20926)) ([c714776](https://github.com/vitejs/vite/commit/c714776aa1dcc24299a81c1495cbcbb1b1ef1dd3))

### Documentation

* correct `WebSocket` spelling ([#20890](https://github.com/vitejs/vite/issues/20890)) ([29e98dc](https://github.com/vitejs/vite/commit/29e98dc3efe35efbd978523367c05db7d2e7a278))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20923](https://github.com/vitejs/vite/issues/20923)) ([a5e3b06](https://github.com/vitejs/vite/commit/a5e3b064fa7ca981cb6f15f8e88806b36a99b8bf))

## <small>[7.1.9](https://github.com/vitejs/vite/compare/v7.1.8...v7.1.9) (2025-10-03)</small>
### Reverts

* **server:** drain stdin when not interactive ([#20885](https://github.com/vitejs/vite/issues/20885)) ([12d72b0](https://github.com/vitejs/vite/commit/12d72b0538ef1540bfb0f1dd8a44b75deaa3464e))

## <small>[7.1.8](https://github.com/vitejs/vite/compare/v7.1.7...v7.1.8) (2025-10-02)</small>
### Bug Fixes

* **css:** improve url escape characters handling ([#20847](https://github.com/vitejs/vite/issues/20847)) ([24a61a3](https://github.com/vitejs/vite/commit/24a61a3f5404279e91f7ceebf7449a5e874f9d56))
* **deps:** update all non-major dependencies ([#20855](https://github.com/vitejs/vite/issues/20855)) ([788a183](https://github.com/vitejs/vite/commit/788a183afce57de13f5656f0cf42cdf6fdc3ebaa))
* **deps:** update artichokie to 0.4.2 ([#20864](https://github.com/vitejs/vite/issues/20864)) ([e670799](https://github.com/vitejs/vite/commit/e670799e123dca78e1a63aeb06dbadade3d5ab51))
* **dev:** skip JS responses for document requests ([#20866](https://github.com/vitejs/vite/issues/20866)) ([6bc6c4d](https://github.com/vitejs/vite/commit/6bc6c4dbc23501577d3919dc841454eb2eb14a54))
* **glob:** fix HMR for array patterns with exclusions ([#20872](https://github.com/vitejs/vite/issues/20872)) ([63e040f](https://github.com/vitejs/vite/commit/63e040f1ca6b635a007eb40aa7c8b891e8cc5799))
* keep ids for virtual modules as-is ([#20808](https://github.com/vitejs/vite/issues/20808)) ([d4eca98](https://github.com/vitejs/vite/commit/d4eca986d679c77bd449db20fd99d8255985b550))
* **server:** drain stdin when not interactive ([#20837](https://github.com/vitejs/vite/issues/20837)) ([bb950e9](https://github.com/vitejs/vite/commit/bb950e92b372f9a52245e9542cf9d9700d23ef8c))
* **server:** improve malformed URL handling in middlewares ([#20830](https://github.com/vitejs/vite/issues/20830)) ([d65a983](https://github.com/vitejs/vite/commit/d65a9831c984e562c5bf2b5f427de16f6e1bd931))

### Documentation

* **create-vite:** provide deno example ([#20747](https://github.com/vitejs/vite/issues/20747)) ([fdb758a](https://github.com/vitejs/vite/commit/fdb758a51796b1ab605437b2eee778a84e87e169))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20810](https://github.com/vitejs/vite/issues/20810)) ([ea68a88](https://github.com/vitejs/vite/commit/ea68a8868c7ee249213057f8a81c3f92a9839dde))
* **deps:** update rolldown-related dependencies ([#20854](https://github.com/vitejs/vite/issues/20854)) ([4dd06fd](https://github.com/vitejs/vite/commit/4dd06fdc8d643059c2abf88188eae7c4877aab6e))
* update url of `create-react-app` license ([#20865](https://github.com/vitejs/vite/issues/20865)) ([166a178](https://github.com/vitejs/vite/commit/166a178f45b6e48db27b5626559f5ec3358c2fb4))

## <small>[7.1.7](https://github.com/vitejs/vite/compare/v7.1.6...v7.1.7) (2025-09-22)</small>
### Bug Fixes

* **build:** fix ssr environment `emitAssets: true` when `sharedConfigBuild: true` ([#20787](https://github.com/vitejs/vite/issues/20787)) ([4c4583c](https://github.com/vitejs/vite/commit/4c4583ce7a13306e0853901570c5d95517fe81da))
* **client:** use CSP nonce when rendering error overlay ([#20791](https://github.com/vitejs/vite/issues/20791)) ([9bc9d12](https://github.com/vitejs/vite/commit/9bc9d1258f550e9d8f5e530cd27aecb1bee32bdb))
* **deps:** update all non-major dependencies ([#20811](https://github.com/vitejs/vite/issues/20811)) ([9f2247c](https://github.com/vitejs/vite/commit/9f2247c066cac75746356c9391845235445a154b))
* **glob:** handle glob imports from folders starting with dot ([#20800](https://github.com/vitejs/vite/issues/20800)) ([105abe8](https://github.com/vitejs/vite/commit/105abe87c412cf0f83859ba41fed869221cbb3e0))
* **hmr:** trigger prune event when import is removed from non hmr module ([#20768](https://github.com/vitejs/vite/issues/20768)) ([9f32b1d](https://github.com/vitejs/vite/commit/9f32b1dc710991c53a9f665c8d0d6945f342bf92))
* **hmr:** wait for `import.meta.hot.prune` callbacks to complete before running other HMRs ([#20698](https://github.com/vitejs/vite/issues/20698)) ([98a3484](https://github.com/vitejs/vite/commit/98a3484733443ee529870477a6ab6a03572e3cbc))

## <small>[7.1.6](https://github.com/vitejs/vite/compare/v7.1.5...v7.1.6) (2025-09-18)</small>
### Bug Fixes

* **deps:** update all non-major dependencies ([#20773](https://github.com/vitejs/vite/issues/20773)) ([88af2ae](https://github.com/vitejs/vite/commit/88af2ae7df77160e7d11a9fa147a4967c8499f13))
* **esbuild:** inject esbuild helper functions with minified `$` variables correctly ([#20761](https://github.com/vitejs/vite/issues/20761)) ([7e8e004](https://github.com/vitejs/vite/commit/7e8e0043d60379e11da481d9cc3c3556c9756ac0))
* fallback terser to main thread when nameCache is provided ([#20750](https://github.com/vitejs/vite/issues/20750)) ([a679a64](https://github.com/vitejs/vite/commit/a679a643404c95556dda2670643e14eca9c585bd))
* **types:** strict env typings fail when `skipLibCheck` is `false` ([#20755](https://github.com/vitejs/vite/issues/20755)) ([cc54e29](https://github.com/vitejs/vite/commit/cc54e294746d3eac868de96f85d98dd0fa0cda11))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20675](https://github.com/vitejs/vite/issues/20675)) ([a67bb5f](https://github.com/vitejs/vite/commit/a67bb5fbec5f3e42151dc7e3166858d0d33533de))
* **deps:** update rolldown-related dependencies ([#20772](https://github.com/vitejs/vite/issues/20772)) ([d785e72](https://github.com/vitejs/vite/commit/d785e72f2ead705e8b2416c0a5097878fced3435))

## <small>[7.1.5](https://github.com/vitejs/vite/compare/v7.1.4...v7.1.5) (2025-09-08)</small>
### Bug Fixes

* apply `fs.strict` check to HTML files ([#20736](https://github.com/vitejs/vite/issues/20736)) ([14015d7](https://github.com/vitejs/vite/commit/14015d794f69accba68798bd0e15135bc51c9c1e))
* **deps:** update all non-major dependencies ([#20732](https://github.com/vitejs/vite/issues/20732)) ([122bfba](https://github.com/vitejs/vite/commit/122bfbabeb1f095ce7cabd30893e5531e9a007c4))
* upgrade sirv to 3.0.2 ([#20735](https://github.com/vitejs/vite/issues/20735)) ([09f2b52](https://github.com/vitejs/vite/commit/09f2b52e8d5907f26602653caf41b3a56692600d))

## <small>[7.1.4](https://github.com/vitejs/vite/compare/v7.1.3...v7.1.4) (2025-09-01)</small>
### Bug Fixes

* add missing awaits ([#20697](https://github.com/vitejs/vite/issues/20697)) ([79d10ed](https://github.com/vitejs/vite/commit/79d10ed6341ba7a751d007b7ad113a9b8be9c853))
* **deps:** update all non-major dependencies ([#20676](https://github.com/vitejs/vite/issues/20676)) ([5a274b2](https://github.com/vitejs/vite/commit/5a274b29df83744cf0ce4dafd94029d2a9e01135))
* **deps:** update all non-major dependencies ([#20709](https://github.com/vitejs/vite/issues/20709)) ([0401feb](https://github.com/vitejs/vite/commit/0401feba17e60bd7e976c5643128a0da49670a83))
* pass rollup watch options when building in watch mode ([#20674](https://github.com/vitejs/vite/issues/20674)) ([f367453](https://github.com/vitejs/vite/commit/f367453ca2825bc8a390d41c5d13b161756f2b41))

### Miscellaneous Chores

* remove unused constants entry from rolldown.config.ts ([#20710](https://github.com/vitejs/vite/issues/20710)) ([537fcf9](https://github.com/vitejs/vite/commit/537fcf91862a1bf51e70ce6fe9b414319dd3a675))

### Code Refactoring

* remove unnecessary `minify` parameter from `finalizeCss` ([#20701](https://github.com/vitejs/vite/issues/20701)) ([8099582](https://github.com/vitejs/vite/commit/8099582e5364f907f2bc6cb8e2d52ae0c4d937e4))

## <small>[7.1.3](https://github.com/vitejs/vite/compare/v7.1.2...v7.1.3) (2025-08-19)</small>
### Features

* **cli:** add Node.js version warning for unsupported versions ([#20638](https://github.com/vitejs/vite/issues/20638)) ([a1be1bf](https://github.com/vitejs/vite/commit/a1be1bf0905b9086e5f1370c63d76a7fa4a195ec))
* generate code frame for parse errors thrown by terser ([#20642](https://github.com/vitejs/vite/issues/20642)) ([a9ba017](https://github.com/vitejs/vite/commit/a9ba0174a58b949373d6b4240bc69180dff0b780))
* support long lines in `generateCodeFrame` ([#20640](https://github.com/vitejs/vite/issues/20640)) ([1559577](https://github.com/vitejs/vite/commit/15595773170c2a07f2efdccee05964fb87c19ae6))

### Bug Fixes

* **deps:** update all non-major dependencies ([#20634](https://github.com/vitejs/vite/issues/20634)) ([4851cab](https://github.com/vitejs/vite/commit/4851cab3ba818b5f0f82eef3796b61d4b12768f1))
* **optimizer:** incorrect incompatible error ([#20439](https://github.com/vitejs/vite/issues/20439)) ([446fe83](https://github.com/vitejs/vite/commit/446fe83033686dd38d13b786a217b8277b5c5f09))
* support multiline new URL(..., import.meta.url) expressions ([#20644](https://github.com/vitejs/vite/issues/20644)) ([9ccf142](https://github.com/vitejs/vite/commit/9ccf142764d48292aa33e5ca6f020a7d55b97f61))

### Performance Improvements

* **cli:** dynamically import `resolveConfig` ([#20646](https://github.com/vitejs/vite/issues/20646)) ([f691f57](https://github.com/vitejs/vite/commit/f691f57e46118328e00174160ceab2101b7256ca))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20633](https://github.com/vitejs/vite/issues/20633)) ([98b92e8](https://github.com/vitejs/vite/commit/98b92e8c4b10ae87c48292a8ac09b01ca81a02cf))

### Code Refactoring

* replace startsWith with strict equality ([#20603](https://github.com/vitejs/vite/issues/20603)) ([42816de](https://github.com/vitejs/vite/commit/42816dee0e177dded1c9de4d9099089ec4acef96))
* use `import` in worker threads ([#20641](https://github.com/vitejs/vite/issues/20641)) ([530687a](https://github.com/vitejs/vite/commit/530687a344c51daf3115d1c134586bbde58356e0))

### Tests

* remove `checkNodeVersion` test ([#20647](https://github.com/vitejs/vite/issues/20647)) ([731d3e6](https://github.com/vitejs/vite/commit/731d3e61f444f6c5e611f67b531416ed6450f90f))

## <small>[7.1.2](https://github.com/vitejs/vite/compare/v7.1.1...v7.1.2) (2025-08-12)</small>
### Bug Fixes

* **client:** add `[vite]` prefixes to debug logs ([#20595](https://github.com/vitejs/vite/issues/20595)) ([7cdef61](https://github.com/vitejs/vite/commit/7cdef612a65da5363905723f77516b6745ac9a94))
* **config:** make debugger work with bundle loader ([#20573](https://github.com/vitejs/vite/issues/20573)) ([c583927](https://github.com/vitejs/vite/commit/c583927bee657f15f63fdf80468fbe6a74eacdec))
* **deps:** update all non-major dependencies ([#20587](https://github.com/vitejs/vite/issues/20587)) ([20d4817](https://github.com/vitejs/vite/commit/20d48172a0352d32f766b3c878d52a8944fdbf6e))
* don't consider ids with `npm:` prefix as a built-in module ([#20558](https://github.com/vitejs/vite/issues/20558)) ([ab33803](https://github.com/vitejs/vite/commit/ab33803f2c831a82ddee637ad62e0c4ceeb663f1))
* **hmr:** watch non-inlined assets referenced by CSS ([#20581](https://github.com/vitejs/vite/issues/20581)) ([b7d494b](https://github.com/vitejs/vite/commit/b7d494bf60af3ef7316d87266bb3ebf56617d5fd))
* **module-runner:** prevent crash when sourceMappingURL pattern appears in string literals ([#20554](https://github.com/vitejs/vite/issues/20554)) ([2770478](https://github.com/vitejs/vite/commit/2770478d1c190d3e3de34ef9a3d2c493c06e9933))

### Miscellaneous Chores

* **deps:** migrate to `@jridgewell/remapping` from `@ampproject/remapping` ([#20577](https://github.com/vitejs/vite/issues/20577)) ([0a6048a](https://github.com/vitejs/vite/commit/0a6048aba4523f451edf29ae4037d252cc963815))
* **deps:** update rolldown-related dependencies ([#20586](https://github.com/vitejs/vite/issues/20586)) ([77632c5](https://github.com/vitejs/vite/commit/77632c55db51cd6d03bcf24a1cef8d21058100a3))

## <small>[7.1.1](https://github.com/vitejs/vite/compare/v7.1.0...v7.1.1) (2025-08-08)</small>
### Bug Fixes

* **deps:** update `launch-editor-middleware` ([#20569](https://github.com/vitejs/vite/issues/20569)) ([826b394](https://github.com/vitejs/vite/commit/826b394e0efd033d2fe88126fe9a28da9573bd8f))

### Miscellaneous Chores

* fix changelog beta links ([#20561](https://github.com/vitejs/vite/issues/20561)) ([2e0c21a](https://github.com/vitejs/vite/commit/2e0c21a07ec5ca7ed5eaa1b6a7d44682fa467a06))
* update 7.1 changelog ([#20560](https://github.com/vitejs/vite/issues/20560)) ([d8869b8](https://github.com/vitejs/vite/commit/d8869b84208879c7aa6a0268ec073a34760c0d80))

## [7.1.0](https://github.com/vitejs/vite/compare/v7.1.0-beta.1...v7.1.0) (2025-08-07)
### Features

* support files with more than 1000 lines by `generateCodeFrame` ([#20508](https://github.com/vitejs/vite/issues/20508)) ([e7d0b2a](https://github.com/vitejs/vite/commit/e7d0b2afa56840dabbbad10015dc04083caaf248))
* add `import.meta.main` support in config (bundle config loader) ([#20516](https://github.com/vitejs/vite/issues/20516)) ([5d3e3c2](https://github.com/vitejs/vite/commit/5d3e3c2ae5a2174941fd09fd7842794a287c3ab7))
* **optimizer:** improve dependency optimization error messages with esbuild formatMessages ([#20525](https://github.com/vitejs/vite/issues/20525)) ([d17cfed](https://github.com/vitejs/vite/commit/d17cfeda0741e4476570700a00b7b37917c97700))
* **ssr:** add `import.meta.main` support for Node.js module runner ([#20517](https://github.com/vitejs/vite/issues/20517)) ([794a8f2](https://github.com/vitejs/vite/commit/794a8f230218a3b1e148defc5a2d7a67409177ff))
* add `future: 'warn'` ([#20473](https://github.com/vitejs/vite/issues/20473)) ([e6aaf17](https://github.com/vitejs/vite/commit/e6aaf17ca21544572941957ce71bd8dbdc94e402))
* add `removeServerPluginContainer` future deprecation ([#20437](https://github.com/vitejs/vite/issues/20437)) ([c1279e7](https://github.com/vitejs/vite/commit/c1279e75401ac6ea1d0678da88414a76ff36b6fe))
* add `removeServerReloadModule` future deprecation ([#20436](https://github.com/vitejs/vite/issues/20436)) ([6970d17](https://github.com/vitejs/vite/commit/6970d1740cebd56af696abf60f30adb0c060f578))
* add `server.warmupRequest` to future deprecation ([#20431](https://github.com/vitejs/vite/issues/20431)) ([8ad388a](https://github.com/vitejs/vite/commit/8ad388aeab0dc79e4bc14859b91174427805a46b))
* add `ssrFixStacktrace` / `ssrRewriteStacktrace` to `removeSsrLoadModule` future deprecation ([#20435](https://github.com/vitejs/vite/issues/20435)) ([8c8f587](https://github.com/vitejs/vite/commit/8c8f5879ead251705c2c363f5b8b94f618fbf374))
* **client:** ping from SharedWorker ([#19057](https://github.com/vitejs/vite/issues/19057)) ([5c97c22](https://github.com/vitejs/vite/commit/5c97c22548476e5f80856ece1d80b9234a7e6ecb))
* **dev:** add `this.fs` support ([#20301](https://github.com/vitejs/vite/issues/20301)) ([0fe3f2f](https://github.com/vitejs/vite/commit/0fe3f2f7c325c5990f1059c28b66b24e1b8fd5d3))
* export `defaultExternalConditions` ([#20279](https://github.com/vitejs/vite/issues/20279)) ([344d302](https://github.com/vitejs/vite/commit/344d30243b107852b133175e947a0410ea703f00))
* implement `removePluginHookSsrArgument` future deprecation ([#20433](https://github.com/vitejs/vite/issues/20433)) ([95927d9](https://github.com/vitejs/vite/commit/95927d9c0ba1cb0b3bd8c900f039c099f8e29f90))
* implement `removeServerHot` future deprecation ([#20434](https://github.com/vitejs/vite/issues/20434)) ([259f45d](https://github.com/vitejs/vite/commit/259f45d0698a184d6ecc352b610001fa1acdcee1))
* resolve server URLs before calling other listeners ([#19981](https://github.com/vitejs/vite/issues/19981)) ([45f6443](https://github.com/vitejs/vite/commit/45f6443a935258d8eee62874f0695b8c1c60a481))
* **ssr:** resolve externalized packages with `resolve.externalConditions` and add `module-sync` to default external condition ([#20409](https://github.com/vitejs/vite/issues/20409)) ([c669c52](https://github.com/vitejs/vite/commit/c669c524e6008a4902169f4b2f865e892297acf3))
* **ssr:** support `import.meta.resolve` in module runner ([#20260](https://github.com/vitejs/vite/issues/20260)) ([62835f7](https://github.com/vitejs/vite/commit/62835f7c06d37802f0bc2abbf58bbaeaa8c73ce5))

### Bug Fixes

* **css:** avoid warnings for `image-set` containing `__VITE_ASSET__` ([#20520](https://github.com/vitejs/vite/issues/20520)) ([f1a2635](https://github.com/vitejs/vite/commit/f1a2635e6977a3eda681bec036f64f07686dad0d))
* **css:** empty CSS entry points should generate CSS files, not JS files ([#20518](https://github.com/vitejs/vite/issues/20518)) ([bac9f3e](https://github.com/vitejs/vite/commit/bac9f3ecf84ae5c5add6ef224ae057508247f89e))
* **dev:** denied request stalled when requested concurrently ([#20503](https://github.com/vitejs/vite/issues/20503)) ([64a52e7](https://github.com/vitejs/vite/commit/64a52e70d9250b16aa81ce2df27c23fe56907257))
* **manifest:** initialize `entryCssAssetFileNames` as an empty Set ([#20542](https://github.com/vitejs/vite/issues/20542)) ([6a46cda](https://github.com/vitejs/vite/commit/6a46cdac5dece70296d1179640958deeeb2e6c19))
* skip prepareOutDirPlugin in workers ([#20556](https://github.com/vitejs/vite/issues/20556)) ([97d5111](https://github.com/vitejs/vite/commit/97d5111645a395dae48b16b110bc76c1ee8956c8))
* **asset:** only watch existing files for `new URL(, import.meta.url)` ([#20507](https://github.com/vitejs/vite/issues/20507)) ([1b211fd](https://github.com/vitejs/vite/commit/1b211fd1beccd0fc13bec700815abaa9f54147e8))
* **client:** keep ping on WS constructor error ([#20512](https://github.com/vitejs/vite/issues/20512)) ([3676da5](https://github.com/vitejs/vite/commit/3676da5bc5b2b69b28619b8521fca94d30468fe5))
* **deps:** update all non-major dependencies ([#20537](https://github.com/vitejs/vite/issues/20537)) ([fc9a9d3](https://github.com/vitejs/vite/commit/fc9a9d3f1493caa3d614f64e0a61fd5684f0928b))
* don't resolve as relative for specifiers starting with a dot ([#20528](https://github.com/vitejs/vite/issues/20528)) ([c5a10ec](https://github.com/vitejs/vite/commit/c5a10ec004130bec17cf42760b76d1d404008fa3))
* **html:** allow control character in input stream ([#20483](https://github.com/vitejs/vite/issues/20483)) ([c12a4a7](https://github.com/vitejs/vite/commit/c12a4a76a299237a0a13b885c72fdda6e4a3c9b7))
* merge old and new `noExternal: true` correctly ([#20502](https://github.com/vitejs/vite/issues/20502)) ([9ebe4a5](https://github.com/vitejs/vite/commit/9ebe4a514a2e48e3fe194f16b0556a45ff38077a))
* **deps:** update all non-major dependencies ([#20489](https://github.com/vitejs/vite/issues/20489)) ([f6aa04a](https://github.com/vitejs/vite/commit/f6aa04a52d486c8881f666c450caa3dab3c6bba1))
* **dev:** denied requests overly ([#20410](https://github.com/vitejs/vite/issues/20410)) ([4be5270](https://github.com/vitejs/vite/commit/4be5270b27f7e6323f1771974b4b3520d86600e4))
* **hmr:** register css deps as `type: asset` ([#20391](https://github.com/vitejs/vite/issues/20391)) ([7eac8dd](https://github.com/vitejs/vite/commit/7eac8ddb65033b8c001d6c6bc46aaeeefb79680a))
* **optimizer:** discover correct jsx runtime during scan ([#20495](https://github.com/vitejs/vite/issues/20495)) ([10d48bb](https://github.com/vitejs/vite/commit/10d48bb2e30824d217e415a58cea9e69c2820c2a))
* **preview:** set correct host for `resolvedUrls` ([#20496](https://github.com/vitejs/vite/issues/20496)) ([62b3e0d](https://github.com/vitejs/vite/commit/62b3e0d95c143e2f8b4e88d99c381d23663025ee))
* **worker:** resolve WebKit compat with inline workers by deferring blob URL revocation ([#20460](https://github.com/vitejs/vite/issues/20460)) ([8033e5b](https://github.com/vitejs/vite/commit/8033e5bf8d3ff43995d0620490ed8739c59171dd))

### Performance Improvements

* **client:** reduce reload debounce ([#20429](https://github.com/vitejs/vite/issues/20429)) ([22ad43b](https://github.com/vitejs/vite/commit/22ad43b4bf2435efe78a65b84e8469b23521900a))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20536](https://github.com/vitejs/vite/issues/20536)) ([8be2787](https://github.com/vitejs/vite/commit/8be278748a92b128c49a24619d8d537dd2b08ceb))
* **deps:** update dependency parse5 to v8 ([#20490](https://github.com/vitejs/vite/issues/20490)) ([744582d](https://github.com/vitejs/vite/commit/744582d0187c50045fb6cf229e3fab13093af08e))
* format ([f20addc](https://github.com/vitejs/vite/commit/f20addc5363058f5fd797e5bc71fab3877ed0a76))
* stablize `cssScopeTo` ([#19592](https://github.com/vitejs/vite/issues/19592)) ([ced1343](https://github.com/vitejs/vite/commit/ced13433fb71e2101850a4da1b0ef70cbc38b804))

### Code Refactoring

* use hook filters in the worker plugin ([#20527](https://github.com/vitejs/vite/issues/20527)) ([958cdf2](https://github.com/vitejs/vite/commit/958cdf24f882be6953ca20912dd30c84213b069b))
* extract prepareOutDir as a plugin ([#20373](https://github.com/vitejs/vite/issues/20373)) ([2c4af1f](https://github.com/vitejs/vite/commit/2c4af1f90b3ac98df6f4585a329528e6bd850462))
* extract resolve rollup options ([#20375](https://github.com/vitejs/vite/issues/20375)) ([61a9778](https://github.com/vitejs/vite/commit/61a97780e6c54adb87345cb8c1f5f0d8e9ca5c05))
* rewrite openchrome.applescript to JXA ([#20424](https://github.com/vitejs/vite/issues/20424)) ([7979f9d](https://github.com/vitejs/vite/commit/7979f9da555aa16bd221b32ea78ce8cb5292fac4))
* use `http-proxy-3` ([#20402](https://github.com/vitejs/vite/issues/20402)) ([26d9872](https://github.com/vitejs/vite/commit/26d987232aad389733a7635b92122bb1d78dfcad))
* use hook filters in internal plugins ([#20358](https://github.com/vitejs/vite/issues/20358)) ([f19c4d7](https://github.com/vitejs/vite/commit/f19c4d72de142814994e30120aa4ad57552cb874))
* use hook filters in internal resolve plugin ([#20480](https://github.com/vitejs/vite/issues/20480)) ([acd2a13](https://github.com/vitejs/vite/commit/acd2a13c2d80e8c5c721bcf9738dfc03346cbfe1))

### Tests

* detect ts support via `process.features` ([#20544](https://github.com/vitejs/vite/issues/20544)) ([856d3f0](https://github.com/vitejs/vite/commit/856d3f06e6889979f630c8453fa385f01d8adaba))
* fix unimportant errors in test-unit ([#20545](https://github.com/vitejs/vite/issues/20545)) ([1f23554](https://github.com/vitejs/vite/commit/1f235545b14a51d41b19a49da4a7e3a8e8eb5d10))

### Beta Changelogs

#### [7.1.0-beta.1](https://github.com/vitejs/vite/compare/v7.1.0-beta.0...v7.1.0-beta.1) (2025-08-05)

See [7.1.0-beta.1 changelog](https://github.com/vitejs/vite/blob/v7.1.0-beta.1/packages/vite/CHANGELOG.md)

#### [7.1.0-beta.0](https://github.com/vitejs/vite/compare/v7.0.6...v7.1.0-beta.0) (2025-07-30)

See [7.1.0-beta.0 changelog](https://github.com/vitejs/vite/blob/v7.1.0-beta.0/packages/vite/CHANGELOG.md)


## <small>[7.0.6](https://github.com/vitejs/vite/compare/v7.0.5...v7.0.6) (2025-07-24)</small>
### Bug Fixes

* **deps:** update all non-major dependencies ([#20442](https://github.com/vitejs/vite/issues/20442)) ([e49f505](https://github.com/vitejs/vite/commit/e49f50599d852eec644e79b074b4648e2dff1e5d))
* **dev:** incorrect sourcemap when optimized CJS is imported ([#20458](https://github.com/vitejs/vite/issues/20458)) ([ead2dec](https://github.com/vitejs/vite/commit/ead2dec74170ad26db8a18bbd68f075efaceb0e3))
* **module-runner:** normalize file:// on windows ([#20449](https://github.com/vitejs/vite/issues/20449)) ([1c9cb49](https://github.com/vitejs/vite/commit/1c9cb493f0467c463113d301b00ce07cbe4b6f58))
* respond with correct headers and status code for HEAD requests ([#20421](https://github.com/vitejs/vite/issues/20421)) ([23d04fc](https://github.com/vitejs/vite/commit/23d04fc2d8a4fcf7c2011418693d6000748aa655))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20441](https://github.com/vitejs/vite/issues/20441)) ([f689d61](https://github.com/vitejs/vite/commit/f689d613429ae9452c74f8bc482d8cc2584ea6b8))
* remove some files from prettier ignore ([#20459](https://github.com/vitejs/vite/issues/20459)) ([8403f69](https://github.com/vitejs/vite/commit/8403f69551131b5c39bfaf242ffac2e5efcd1dd6))

### Code Refactoring

* use environment transform request ([#20430](https://github.com/vitejs/vite/issues/20430)) ([24e6a0c](https://github.com/vitejs/vite/commit/24e6a0c3165557396db6ab59d3001e037c76ce32))

## <small>[7.0.5](https://github.com/vitejs/vite/compare/v7.0.4...v7.0.5) (2025-07-17)</small>
### Bug Fixes

* **deps:** update all non-major dependencies ([#20406](https://github.com/vitejs/vite/issues/20406)) ([1a1cc8a](https://github.com/vitejs/vite/commit/1a1cc8a435a21996255b3e5cc75ed4680de2a7f3))
* remove special handling for `Accept: text/html` ([#20376](https://github.com/vitejs/vite/issues/20376)) ([c9614b9](https://github.com/vitejs/vite/commit/c9614b9c378be4a32e84f37be71a8becce52af7b))
* watch assets referenced by `new URL(, import.meta.url)` ([#20382](https://github.com/vitejs/vite/issues/20382)) ([6bc8bf6](https://github.com/vitejs/vite/commit/6bc8bf634d4a2c9915da9813963dd80a4186daeb))

### Miscellaneous Chores

* **deps:** update dependency rolldown to ^1.0.0-beta.27 ([#20405](https://github.com/vitejs/vite/issues/20405)) ([1165667](https://github.com/vitejs/vite/commit/1165667b271fb1fb76584278e72a85d564c9bb09))

### Code Refactoring

* use `foo.endsWith("bar")` instead of `/bar$/.test(foo)` ([#20413](https://github.com/vitejs/vite/issues/20413)) ([862e192](https://github.com/vitejs/vite/commit/862e192d21f66039635a998724bdc6b94fd293a0))

## <small>[7.0.4](https://github.com/vitejs/vite/compare/v7.0.3...v7.0.4) (2025-07-10)</small>
### Bug Fixes

* allow resolving bare specifiers to relative paths for entries ([#20379](https://github.com/vitejs/vite/issues/20379)) ([324669c](https://github.com/vitejs/vite/commit/324669c2d84966a822b1b2c134c9830a90bed271))

### Build System

* remove `@oxc-project/runtime` devDep ([#20389](https://github.com/vitejs/vite/issues/20389)) ([5e29602](https://github.com/vitejs/vite/commit/5e29602f6fe4bf28f6e7c869a214dee6957f855c))

## <small>[7.0.3](https://github.com/vitejs/vite/compare/v7.0.2...v7.0.3) (2025-07-08)</small>
### Bug Fixes

* **client:** protect against window being defined but addEv undefined ([#20359](https://github.com/vitejs/vite/issues/20359)) ([31d1467](https://github.com/vitejs/vite/commit/31d1467cf0da1e1dca623e6df0d345b30fae0c3d))
* **define:** replace optional values ([#20338](https://github.com/vitejs/vite/issues/20338)) ([9465ae1](https://github.com/vitejs/vite/commit/9465ae1378b456e08659a22286bee6bce8edeedc))
* **deps:** update all non-major dependencies ([#20366](https://github.com/vitejs/vite/issues/20366)) ([43ac73d](https://github.com/vitejs/vite/commit/43ac73da27b3907c701e95e6a7d28fde659729ec))

### Miscellaneous Chores

* **deps:** update dependency dotenv to v17 ([#20325](https://github.com/vitejs/vite/issues/20325)) ([45040d4](https://github.com/vitejs/vite/commit/45040d48076302eeb101f8d07bbcd04758fde8a4))
* **deps:** update dependency rolldown to ^1.0.0-beta.24 ([#20365](https://github.com/vitejs/vite/issues/20365)) ([5ab25e7](https://github.com/vitejs/vite/commit/5ab25e73a2ea2a2e2c0469350288a183dfb57030))
* use `n/prefer-node-protocol` rule ([#20368](https://github.com/vitejs/vite/issues/20368)) ([38bb268](https://github.com/vitejs/vite/commit/38bb268cde15541321f36016e77d61eecb707298))

### Code Refactoring

* minor changes to reduce diff between normal Vite and rolldown-vite ([#20354](https://github.com/vitejs/vite/issues/20354)) ([2e8050e](https://github.com/vitejs/vite/commit/2e8050e4cd8835673baf07375b7db35128144222))

## <small>[7.0.2](https://github.com/vitejs/vite/compare/v7.0.1...v7.0.2) (2025-07-04)</small>
### Bug Fixes

* **css:** resolve relative paths in sass, revert [#20300](https://github.com/vitejs/vite/issues/20300) ([#20349](https://github.com/vitejs/vite/issues/20349)) ([db8bd41](https://github.com/vitejs/vite/commit/db8bd412a8b783fe8e9f82d1a822b0534abbf5a3))

## <small>[7.0.1](https://github.com/vitejs/vite/compare/v7.0.0...v7.0.1) (2025-07-03)</small>
### Bug Fixes

* **css:** skip resolving resolved paths in sass ([#20300](https://github.com/vitejs/vite/issues/20300)) ([ac528a4](https://github.com/vitejs/vite/commit/ac528a44c384fefb6f10c3f531df93b5ac39324c))
* **deps:** update all non-major dependencies ([#20324](https://github.com/vitejs/vite/issues/20324)) ([3e81af3](https://github.com/vitejs/vite/commit/3e81af38a80c7617aba6bf3300d8b4267570f9cf))
* **types:** add a global interface for Worker ([#20243](https://github.com/vitejs/vite/issues/20243)) ([37bdfc1](https://github.com/vitejs/vite/commit/37bdfc18f4c5bed053a38c5d717df33036acdd62))

### Miscellaneous Chores

* **deps:** update rolldown-related dependencies ([#20323](https://github.com/vitejs/vite/issues/20323)) ([30d2f1b](https://github.com/vitejs/vite/commit/30d2f1b38c72387ffdca3ee4746730959a020b59))
* fix typos and grammatical errors across documentation and comments ([#20337](https://github.com/vitejs/vite/issues/20337)) ([c1c951d](https://github.com/vitejs/vite/commit/c1c951dcc32ec9f133b03ebbceddd749fc14f1e9))
* group commits by category in changelog ([#20310](https://github.com/vitejs/vite/issues/20310)) ([41e83f6](https://github.com/vitejs/vite/commit/41e83f62b1adb65f5af4c1ec006de1c845437edc))
* rearrange 7.0 changelog ([#20280](https://github.com/vitejs/vite/issues/20280)) ([eafd28a](https://github.com/vitejs/vite/commit/eafd28ac88d5908cbc3e0a047ed7a12094386436))

## [7.0.0](https://github.com/vitejs/vite/compare/v7.0.0-beta.2...v7.0.0) (2025-06-24)

![Vite 7 is out!](../../docs/public/og-image-announcing-vite7.png)

Today, we're excited to announce the release of the next Vite major:

- **[Vite 7.0 announcement blog post](https://vite.dev/blog/announcing-vite7.html)**
- [Docs](https://vite.dev/) (translations: [简体中文](https://cn.vite.dev/), [日本語](https://ja.vite.dev/), [Español](https://es.vite.dev/), [Português](https://pt.vite.dev/), [한국어](https://ko.vite.dev/), [Deutsch](https://de.vite.dev/), [فارسی](https://fa.vite.dev/))
- [Migration Guide](https://vite.dev/guide/migration.html)

### ⚠ BREAKING CHANGES

* **ssr:** don't access `Object` variable in ssr transformed code (#19996)
* remove `experimental.skipSsrTransform` option (#20038)
* remove `HotBroadcaster` (#19988)
* **css:** always use sass compiler API (#19978)
* bump `build.target` and name it `baseline-widely-available` (#20007)
* bump required node version to 20.19+, 22.12+ and remove cjs build (#20032)
* **css:** remove sass legacy API support (#19977)
* remove deprecated `HotBroadcaster` related types (#19987)
* remove deprecated no-op type only properties (#19985)
* remove node 18 support (#19972)
* remove deprecated hook-level `enforce`/`transform` from `transformIndexHtml` hook (#19349)
* remove deprecated splitVendorChunkPlugin (#19255)

### Features

* **types:** use terser types from terser package ([#20274](https://github.com/vitejs/vite/issues/20274)) ([a5799fa](https://github.com/vitejs/vite/commit/a5799fa74c6190ecbb2da3d280136ff32463afc6))
* apply some middlewares before `configurePreviewServer` hook ([#20224](https://github.com/vitejs/vite/issues/20224)) ([b989c42](https://github.com/vitejs/vite/commit/b989c42cf84378e6cb93970de739941f0d56d6f6))
* apply some middlewares before `configureServer` hook ([#20222](https://github.com/vitejs/vite/issues/20222)) ([f5cc4c0](https://github.com/vitejs/vite/commit/f5cc4c0ded337670b439e51bc95f173e2b5cf9ad))
* add base option to import.meta.glob ([#20163](https://github.com/vitejs/vite/issues/20163)) ([253d6c6](https://github.com/vitejs/vite/commit/253d6c6df2ebe3c4a88dabb6cec000128681561f))
* add `this.meta.viteVersion` ([#20088](https://github.com/vitejs/vite/issues/20088)) ([f55bf41](https://github.com/vitejs/vite/commit/f55bf41e91f8dfe829a46e58f0035b19c8ab6a25))
* allow passing down resolved config to vite's `createServer` ([#19894](https://github.com/vitejs/vite/issues/19894)) ([c1ae9bd](https://github.com/vitejs/vite/commit/c1ae9bd4a0542b4703ae7766ad61d072e8b833bd))
* buildApp hook ([#19971](https://github.com/vitejs/vite/issues/19971)) ([5da659d](https://github.com/vitejs/vite/commit/5da659de902f0a2d6d8beefbf269128383b63887))
* **build:** provide names for asset entrypoints ([#19912](https://github.com/vitejs/vite/issues/19912)) ([c4e01dc](https://github.com/vitejs/vite/commit/c4e01dc5ab0f1708383c39d28ce62e12b8f374fc))
* bump `build.target` and name it `baseline-widely-available` ([#20007](https://github.com/vitejs/vite/issues/20007)) ([4a8aa82](https://github.com/vitejs/vite/commit/4a8aa82556eb2b9e54673a6aac77873e0eb27fa9))
* **client:** support opening fileURL in editor ([#20040](https://github.com/vitejs/vite/issues/20040)) ([1bde4d2](https://github.com/vitejs/vite/commit/1bde4d25243cd9beaadb01413e896fef562626ef))
* make PluginContext available for Vite-specific hooks ([#19936](https://github.com/vitejs/vite/issues/19936)) ([7063839](https://github.com/vitejs/vite/commit/7063839d47dfd4ac6be1247ba68e414ffe287b00))
* resolve environments plugins at config time ([#20120](https://github.com/vitejs/vite/issues/20120)) ([f6a28d5](https://github.com/vitejs/vite/commit/f6a28d5f792ba5cc4dc236e3e6edd05199cabcc8))
* stabilize `css.preprocessorMaxWorkers` and default to `true` ([#19992](https://github.com/vitejs/vite/issues/19992)) ([70aee13](https://github.com/vitejs/vite/commit/70aee139ea802478bad56e5e441f187140bcf0cc))
* stabilize `optimizeDeps.noDiscovery` ([#19984](https://github.com/vitejs/vite/issues/19984)) ([6d2dcb4](https://github.com/vitejs/vite/commit/6d2dcb494db9f40565f11b50bdbb8c1b7245697d))

### Bug Fixes

* **deps:** update all non-major dependencies ([#20271](https://github.com/vitejs/vite/issues/20271)) ([6b64d63](https://github.com/vitejs/vite/commit/6b64d63d700154de2c00270300b671cef8863708))
* keep `import.meta.url` in bundled Vite ([#20235](https://github.com/vitejs/vite/issues/20235)) ([3bf3a8a](https://github.com/vitejs/vite/commit/3bf3a8ab00e5a0dfab0bb5741cb871ea30b72651))
* **module-runner:** export `ssrExportNameKey` ([#20266](https://github.com/vitejs/vite/issues/20266)) ([ac302a7](https://github.com/vitejs/vite/commit/ac302a729062dbfc67f762b3c4af46b7893c214f))
* **module-runner:** expose `normalizeModuleId` ([#20277](https://github.com/vitejs/vite/issues/20277)) ([9b98dcb](https://github.com/vitejs/vite/commit/9b98dcbf75546240e1609185828e18a77bac8c8d))
* **deps:** update all non-major dependencies ([#20181](https://github.com/vitejs/vite/issues/20181)) ([d91d4f7](https://github.com/vitejs/vite/commit/d91d4f7ad55edbcb4a51fc23376cbff89f776d30))
* **deps:** update all non-major dependencies ([#20212](https://github.com/vitejs/vite/issues/20212)) ([a80339b](https://github.com/vitejs/vite/commit/a80339b1798607dd7389f42964272181cf9eb453))
* align dynamic import detection ([#20115](https://github.com/vitejs/vite/issues/20115)) ([1ea2222](https://github.com/vitejs/vite/commit/1ea2222302f128c4000289683480d8311ea34223))
* applyToEnvironment after configResolved ([#20170](https://github.com/vitejs/vite/issues/20170)) ([a330b80](https://github.com/vitejs/vite/commit/a330b805b0733fadd1f7d586218c2aafcbb41a7f))
* **deps:** update all non-major dependencies ([#20141](https://github.com/vitejs/vite/issues/20141)) ([89ca65b](https://github.com/vitejs/vite/commit/89ca65ba1d849046dccdea52e9eca980f331be26))
* handle dynamic import with `.then(m => m.a)` ([#20117](https://github.com/vitejs/vite/issues/20117)) ([7b7410a](https://github.com/vitejs/vite/commit/7b7410abab7c95880d943e46bd1a16dcb1a893fc))
* **hmr:** use monotonicDateNow for timestamp ([#20158](https://github.com/vitejs/vite/issues/20158)) ([8d26785](https://github.com/vitejs/vite/commit/8d26785b8c3f5295ca0c1519dda1ddae9096fc73))
* **optimizer:** align relative `build.rollupOptions.input` resolution with rollup ([#20080](https://github.com/vitejs/vite/issues/20080)) ([9759c29](https://github.com/vitejs/vite/commit/9759c29a8985da1a51de452d741850f0bf2ef7ef))
* **ssr:** don't access `Object` variable in ssr transformed code ([#19996](https://github.com/vitejs/vite/issues/19996)) ([fceff60](https://github.com/vitejs/vite/commit/fceff60dc81730f7768b57f14e7a112facff387d))
* **types:** prefer sass-embedded types over sass types for `preprocessorOptions.sass` (fix [#20150](https://github.com/vitejs/vite/issues/20150)) ([#20166](https://github.com/vitejs/vite/issues/20166)) ([7db56be](https://github.com/vitejs/vite/commit/7db56be237dd1e1e875518475421d5c90cf950da))
* virtual svg module ([#20144](https://github.com/vitejs/vite/issues/20144)) ([7dfcb31](https://github.com/vitejs/vite/commit/7dfcb316ee64aca0a98a1d2905deb1dfd113ae6d))
* **client:** render the last part of the stacktrace ([#20039](https://github.com/vitejs/vite/issues/20039)) ([c7c1743](https://github.com/vitejs/vite/commit/c7c17434968848f1471179c10a5fc9d2804add8b))
* **cli:** make `cleanGlobalCLIOptions()` clean `--force` ([#19999](https://github.com/vitejs/vite/issues/19999)) ([d4a171a](https://github.com/vitejs/vite/commit/d4a171afd387000789172a94c94a1c33c0856f85))
* **css:** remove alias exclude logic from rebaseUrl ([#20100](https://github.com/vitejs/vite/issues/20100)) ([44c6d01](https://github.com/vitejs/vite/commit/44c6d0111f95c8aa44d6a09a768e8cf02232ed29))
* **css:** sass rebase url in relative imported modules ([#20067](https://github.com/vitejs/vite/issues/20067)) ([261fad9](https://github.com/vitejs/vite/commit/261fad9b8e6380c84b8692b3fbe18d6f37d367bd))
* **css:** should not wrap with double quote when the url rebase feature bailed out ([#20068](https://github.com/vitejs/vite/issues/20068)) ([a33d0c7](https://github.com/vitejs/vite/commit/a33d0c7d65d9fff9acd5de0cf3c4d371297b3990))
* **deps:** update all non-major dependencies ([#19953](https://github.com/vitejs/vite/issues/19953)) ([ac8e1fb](https://github.com/vitejs/vite/commit/ac8e1fb289a06fc0671dab1f4ef68e508e34360e))
* **deps:** update all non-major dependencies ([#20061](https://github.com/vitejs/vite/issues/20061)) ([7b58856](https://github.com/vitejs/vite/commit/7b588563636a6f735a6e25832f33fc08572b25d9))
* importing an optional peer dep should throw an runtime error ([#20029](https://github.com/vitejs/vite/issues/20029)) ([d0221cd](https://github.com/vitejs/vite/commit/d0221cd7383c18d67a5ef594da52e6aa5fc4d87b))
* merge `environments.*.resolve.noExternal` properly ([#20077](https://github.com/vitejs/vite/issues/20077)) ([daf4a25](https://github.com/vitejs/vite/commit/daf4a25a1c0a37c992606e6ae159e13190c2e101))
* merge `server.allowedHosts: true` correctly ([#20138](https://github.com/vitejs/vite/issues/20138)) ([2ade756](https://github.com/vitejs/vite/commit/2ade756c9549a52d804797d45da37c8429a51fd3))
* **optimizer:** non object module.exports for Node builtin modules in CJS external facade ([#20048](https://github.com/vitejs/vite/issues/20048)) ([00ac6e4](https://github.com/vitejs/vite/commit/00ac6e410eeb15719fe020fd497f0336e7fd1aa8))
* **optimizer:** show error when `computeEntries` failed ([#20079](https://github.com/vitejs/vite/issues/20079)) ([b742b46](https://github.com/vitejs/vite/commit/b742b46f8308a71c1d2aa426eade0c50cbf1480f))
* treat all `optimizeDeps.entries` values as globs ([#20045](https://github.com/vitejs/vite/issues/20045)) ([1422395](https://github.com/vitejs/vite/commit/142239588d6752c5b91d435aee9b4a6c00b7f924))
* **types:** expose additional PluginContext types ([#20129](https://github.com/vitejs/vite/issues/20129)) ([b6df9aa](https://github.com/vitejs/vite/commit/b6df9aac3320cd953f6d45ad9245a7b564f67cc1))

### Performance Improvements

* **utils:** improve performance of `numberToPos` ([#20244](https://github.com/vitejs/vite/issues/20244)) ([3f46901](https://github.com/vitejs/vite/commit/3f469012ad38e3cb330adc74a8b3ec88561c822e))

### Documentation

* tiny typo ([#20110](https://github.com/vitejs/vite/issues/20110)) ([d20fc2c](https://github.com/vitejs/vite/commit/d20fc2cdc9700513425b18b625e01224f61e4eab))

### Miscellaneous Chores

* "indentity" → "identity" in test description ([#20225](https://github.com/vitejs/vite/issues/20225)) ([ea9aed7](https://github.com/vitejs/vite/commit/ea9aed7ebcb7f4be542bd2a384cbcb5a1e7b31bd))
* **deps:** update rolldown-related dependencies ([#20270](https://github.com/vitejs/vite/issues/20270)) ([f7377c3](https://github.com/vitejs/vite/commit/f7377c3eae6323bd3237ff5de5ae55c879fe7325))
* typos in comments ([#20259](https://github.com/vitejs/vite/issues/20259)) ([b135918](https://github.com/vitejs/vite/commit/b135918b91e8381c50bd2d076d40e9a65fe68bfe))
* **deps:** update rolldown-related dependencies ([#20182](https://github.com/vitejs/vite/issues/20182)) ([6172f41](https://github.com/vitejs/vite/commit/6172f410b44cbae8d052997bb1819a6197a4d397))
* **deps:** update rolldown-related dependencies ([#20211](https://github.com/vitejs/vite/issues/20211)) ([b13b7f5](https://github.com/vitejs/vite/commit/b13b7f5e21fe05c3214766b3de584a026fbfe144))
* add a way to disable source maps when developing Vite ([#20168](https://github.com/vitejs/vite/issues/20168)) ([3a30c0a](https://github.com/vitejs/vite/commit/3a30c0a084a1b92a6265f8900df89e5102418e5e))
* **deps:** update rolldown-related dependencies ([#20140](https://github.com/vitejs/vite/issues/20140)) ([0387447](https://github.com/vitejs/vite/commit/03874471e3de14e7d2f474ecb354499e7f5eb418))
* fix source map support when developing Vite ([#20167](https://github.com/vitejs/vite/issues/20167)) ([279ab0d](https://github.com/vitejs/vite/commit/279ab0dc954c5e986810b78efa7fe898945f8f21))
* use destructuring alias in buildEnvironment function ([#19472](https://github.com/vitejs/vite/issues/19472)) ([501572a](https://github.com/vitejs/vite/commit/501572a9a3e1e22ab7e19afb5b13d3f54da67c37))
* declare version range for peer dependencies ([#19979](https://github.com/vitejs/vite/issues/19979)) ([c9bfd57](https://github.com/vitejs/vite/commit/c9bfd578f4c56314c6c6d6f34e49fe494ae11072))
* deprecate `ResolvedConfig.createResolver` and recommend `createIdResolver` ([#20031](https://github.com/vitejs/vite/issues/20031)) ([d101d64](https://github.com/vitejs/vite/commit/d101d64722f82ed681b833bfd3fb394eeb496e21))
* fix comment for `devEnvironmentOptions.moduleRunnerTransform` ([#20035](https://github.com/vitejs/vite/issues/20035)) ([338081d](https://github.com/vitejs/vite/commit/338081df9649f68484416d199113fc67abbb6cd5))
* generate dts internally by rolldown-plugin-dts ([#20093](https://github.com/vitejs/vite/issues/20093)) ([a66afa3](https://github.com/vitejs/vite/commit/a66afa33bd92e2be6ee1d52b8fffa49da266adab))
* remove deprecated splitVendorChunkPlugin ([#19255](https://github.com/vitejs/vite/issues/19255)) ([91a92c7](https://github.com/vitejs/vite/commit/91a92c7e1eaf55cd5d5cfa49c546e130045e7dee))
* remove node 18 support ([#19972](https://github.com/vitejs/vite/issues/19972)) ([00b8a98](https://github.com/vitejs/vite/commit/00b8a98f36376804437e1342265453915ae613de))
* remove redundant word in comment ([#20139](https://github.com/vitejs/vite/issues/20139)) ([9b2964d](https://github.com/vitejs/vite/commit/9b2964df79d31b17e6b387e7fc082753f8ee5774))
* remove unused deps ([#20097](https://github.com/vitejs/vite/issues/20097)) ([d11ae6b](https://github.com/vitejs/vite/commit/d11ae6bca808407a9f0fb4f9c1cb8496a705c2d7))
* rename rollup to rolldown where appropriate ([#20096](https://github.com/vitejs/vite/issues/20096)) ([306e250](https://github.com/vitejs/vite/commit/306e250a94e12584b4182db8ec531750b3d9e3ba))
* speed up typechecking ([#20131](https://github.com/vitejs/vite/issues/20131)) ([a357c19](https://github.com/vitejs/vite/commit/a357c1987f332519d7bacafebc5620c7ab534d8f))
* use plugin hooks filter for `patch-types` plugin for bundling vite ([#20089](https://github.com/vitejs/vite/issues/20089)) ([c127955](https://github.com/vitejs/vite/commit/c12795522fd95d3535100293f4cf53c53c3f301f))
* use rolldown to bundle Vite itself ([#19925](https://github.com/vitejs/vite/issues/19925)) ([7753b02](https://github.com/vitejs/vite/commit/7753b028848d9e23bcea5f00565207f2d1de8291))
* use rolldown-plugin-dts for dts bundling ([#19990](https://github.com/vitejs/vite/issues/19990)) ([449d7f3](https://github.com/vitejs/vite/commit/449d7f30a85ae70eb0037fdab0b1ebf2e4927a24))

### Code Refactoring

* **worker:** set virtual file content in load hook ([#20160](https://github.com/vitejs/vite/issues/20160)) ([0d60667](https://github.com/vitejs/vite/commit/0d60667e03d91cc0d48dd2cdbd8154d94e0aba74))
* bump required node version to 20.19+, 22.12+ and remove cjs build ([#20032](https://github.com/vitejs/vite/issues/20032)) ([2b80243](https://github.com/vitejs/vite/commit/2b80243fada75378e80475028fdcc78f4432bd6f))
* **css:** always use sass compiler API ([#19978](https://github.com/vitejs/vite/issues/19978)) ([3bfe5c5](https://github.com/vitejs/vite/commit/3bfe5c5ff96af0a0624c8f14503ef87a0c0850ed))
* **css:** remove sass legacy API support ([#19977](https://github.com/vitejs/vite/issues/19977)) ([6eaccc9](https://github.com/vitejs/vite/commit/6eaccc9009d718a1afcff2af587e81eb959f5b60))
* merge `src/node/publicUtils.ts` to `src/node/index.ts` ([#20086](https://github.com/vitejs/vite/issues/20086)) ([999a1ed](https://github.com/vitejs/vite/commit/999a1ed8dff5117b2fd205c4e5384b6ac2ede80e))
* remove `experimental.skipSsrTransform` option ([#20038](https://github.com/vitejs/vite/issues/20038)) ([6c3dd8e](https://github.com/vitejs/vite/commit/6c3dd8e46fa77060603679cda91a4c8d01d095ab))
* remove `HotBroadcaster` ([#19988](https://github.com/vitejs/vite/issues/19988)) ([cda8c94](https://github.com/vitejs/vite/commit/cda8c947934466da27e874b6c064451cf73f03e5))
* remove `options?.ssr` support in clientInjectionsPlugin ([#19589](https://github.com/vitejs/vite/issues/19589)) ([88e0076](https://github.com/vitejs/vite/commit/88e00765dbd3de4cb073c722dce3e8ef60c3a50e))
* remove backward compat for calling internal plugins directly ([#20001](https://github.com/vitejs/vite/issues/20001)) ([9072a72](https://github.com/vitejs/vite/commit/9072a726731eccee32d38f04747fda8793ccc82a))
* remove deprecated `HotBroadcaster` related types ([#19987](https://github.com/vitejs/vite/issues/19987)) ([86b5e00](https://github.com/vitejs/vite/commit/86b5e0030bf204f8f2db0cf8ee895ab3ecf154b8))
* remove deprecated env api properties ([#19986](https://github.com/vitejs/vite/issues/19986)) ([52e5a1b](https://github.com/vitejs/vite/commit/52e5a1b32d0ce7604b633f001a352124e3ec623a))
* remove deprecated hook-level `enforce`/`transform` from `transformIndexHtml` hook ([#19349](https://github.com/vitejs/vite/issues/19349)) ([6198b9d](https://github.com/vitejs/vite/commit/6198b9d2a32f7bd17b3332525a98c06d9a425fb1))
* remove deprecated no-op type only properties ([#19985](https://github.com/vitejs/vite/issues/19985)) ([9151c24](https://github.com/vitejs/vite/commit/9151c2400f6ab494f73d78aea4435b7c1ef5fc30))
* remove no-op `legacy.proxySsrExternalModules` ([#20013](https://github.com/vitejs/vite/issues/20013)) ([a37ac83](https://github.com/vitejs/vite/commit/a37ac836ac4da8e854d98c65450f12acb921aa98))
* **ssr:** remove ssrTransform line offset preservation ([#19829](https://github.com/vitejs/vite/issues/19829)) ([61b6b96](https://github.com/vitejs/vite/commit/61b6b96b191c6071b9c574ad4c795f97f2646f18))
* use `hostValidationMiddleware` ([#20019](https://github.com/vitejs/vite/issues/20019)) ([83bf90e](https://github.com/vitejs/vite/commit/83bf90edd5856ed6e27051e3e9a6032e02242b18))
* use `mergeWithDefaults` for experimental option ([#20012](https://github.com/vitejs/vite/issues/20012)) ([98c5741](https://github.com/vitejs/vite/commit/98c57419426201596a962746436e5ad1aeef4eac))
* use hook filters from rollup ([#19755](https://github.com/vitejs/vite/issues/19755)) ([0d18fc1](https://github.com/vitejs/vite/commit/0d18fc1dc65f5c8d855808f23754c0c4902f07d9))

### Tests

* correct esbuild `useDefineForClassFields` test ([#20143](https://github.com/vitejs/vite/issues/20143)) ([d90796e](https://github.com/vitejs/vite/commit/d90796ece7d30d1879d74c422628be30d1c90a7f))
* skip writing files in build hook filter test ([#20076](https://github.com/vitejs/vite/issues/20076)) ([bf8b07d](https://github.com/vitejs/vite/commit/bf8b07da3e64dc4de446a9b24a33d5822a7736b9))

### Continuous Integration

* run tests on Node 24 as well ([#20049](https://github.com/vitejs/vite/issues/20049)) ([1fe07d3](https://github.com/vitejs/vite/commit/1fe07d3716012992dd7b2e78d8380add0b606a97))

### Beta Changelogs


#### [7.0.0-beta.2](https://github.com/vitejs/vite/compare/v7.0.0-beta.1...v7.0.0-beta.2) (2025-06-17)

See [7.0.0-beta.2 changelog](https://github.com/vitejs/vite/blob/v7.0.0-beta.2/packages/vite/CHANGELOG.md)


#### [7.0.0-beta.1](https://github.com/vitejs/vite/compare/v7.0.0-beta.0...v7.0.0-beta.1) (2025-06-10)

See [7.0.0-beta.1 changelog](https://github.com/vitejs/vite/blob/v7.0.0-beta.1/packages/vite/CHANGELOG.md)


#### [7.0.0-beta.0](https://github.com/vitejs/vite/compare/6.3.5...v7.0.0-beta.0) (2025-06-02)

See [7.0.0-beta.0 changelog](https://github.com/vitejs/vite/blob/v7.0.0-beta.0/packages/vite/CHANGELOG.md)


## <small>[6.3.5](https://github.com/vitejs/vite/compare/v6.3.4...v6.3.5) (2025-05-05)</small>
### Bug Fixes

* **ssr:** handle uninitialized export access as undefined ([#19959](https://github.com/vitejs/vite/issues/19959)) ([fd38d07](https://github.com/vitejs/vite/commit/fd38d076fe2455aac1e00a7b15cd51159bf12bb5))

## <small>[6.3.4](https://github.com/vitejs/vite/compare/v6.3.3...v6.3.4) (2025-04-30)</small>
### Bug Fixes

* check static serve file inside sirv ([#19965](https://github.com/vitejs/vite/issues/19965)) ([c22c43d](https://github.com/vitejs/vite/commit/c22c43de612eebb6c182dd67850c24e4fab8cacb))
* **optimizer:** return plain object when using `require` to import externals in optimized dependencies ([#19940](https://github.com/vitejs/vite/issues/19940)) ([efc5eab](https://github.com/vitejs/vite/commit/efc5eab253419fde0a6a48b8d2f233063d6a9643))

### Code Refactoring

* remove duplicate plugin context type ([#19935](https://github.com/vitejs/vite/issues/19935)) ([d6d01c2](https://github.com/vitejs/vite/commit/d6d01c2292fa4f9603e05b95d81c8724314c20e0))

## <small>[6.3.3](https://github.com/vitejs/vite/compare/v6.3.2...v6.3.3) (2025-04-24)</small>
### Bug Fixes

* **assets:** ensure ?no-inline is not included in the asset url in the production environment ([#19496](https://github.com/vitejs/vite/issues/19496)) ([16a73c0](https://github.com/vitejs/vite/commit/16a73c05d35daa34117a173784895546212db5f4))
* **css:** resolve relative imports in sass properly on Windows ([#19920](https://github.com/vitejs/vite/issues/19920)) ([ffab442](https://github.com/vitejs/vite/commit/ffab44270488f54ae344801024474b597249071b))
* **deps:** update all non-major dependencies ([#19899](https://github.com/vitejs/vite/issues/19899)) ([a4b500e](https://github.com/vitejs/vite/commit/a4b500ef9ccc9b19a2882156a9ba8397e69bc6b2))
* ignore malformed uris in transform middleware ([#19853](https://github.com/vitejs/vite/issues/19853)) ([e4d5201](https://github.com/vitejs/vite/commit/e4d520141bcd83ad61f16767348b4a813bf9340a))
* **ssr:** fix execution order of re-export ([#19841](https://github.com/vitejs/vite/issues/19841)) ([ed29dee](https://github.com/vitejs/vite/commit/ed29dee2eb2e3573b2bc337e1a9124c65222a1e5))
* **ssr:** fix live binding of default export declaration and hoist exports getter ([#19842](https://github.com/vitejs/vite/issues/19842)) ([80a91ff](https://github.com/vitejs/vite/commit/80a91ff82426a4c88d54b9f5ec9a4205cb13899b))

### Performance Improvements

* skip sourcemap generation for renderChunk hook of import-analysis-build plugin ([#19921](https://github.com/vitejs/vite/issues/19921)) ([55cfd04](https://github.com/vitejs/vite/commit/55cfd04b10f98cde7a96814a69b9813543ea79c2))

### Tests

* **ssr:** test `ssrTransform` re-export deps and test stacktrace with first line ([#19629](https://github.com/vitejs/vite/issues/19629)) ([9399cda](https://github.com/vitejs/vite/commit/9399cdaf8c3b2efd5f4015d57dc3b0e4e5b91a9d))

## <small>[6.3.2](https://github.com/vitejs/vite/compare/v6.3.1...v6.3.2) (2025-04-18)</small>
### Features

* **css:** improve lightningcss messages ([#19880](https://github.com/vitejs/vite/issues/19880)) ([c713f79](https://github.com/vitejs/vite/commit/c713f79b5a4bd98542d8dbe4c85ba4cce9b1f358))

### Bug Fixes

* **css:** respect `css.lightningcss` option in css minification process ([#19879](https://github.com/vitejs/vite/issues/19879)) ([b5055e0](https://github.com/vitejs/vite/commit/b5055e0dd4c0e084115c3dbfead5736a54807e0c))
* **deps:** update all non-major dependencies ([#19698](https://github.com/vitejs/vite/issues/19698)) ([bab4cb9](https://github.com/vitejs/vite/commit/bab4cb92248adf6b9b18df12b2bf03889b0bd1eb))
* match default asserts case insensitive ([#19852](https://github.com/vitejs/vite/issues/19852)) ([cbdab1d](https://github.com/vitejs/vite/commit/cbdab1d6a30e07263ec51b2ca042369e736adec6))
* open first url if host does not match any urls ([#19886](https://github.com/vitejs/vite/issues/19886)) ([6abbdce](https://github.com/vitejs/vite/commit/6abbdce3d77990409e12380e72c7ec9dd3f8bec5))

## <small>[6.3.1](https://github.com/vitejs/vite/compare/v6.3.0...v6.3.1) (2025-04-17)</small>
### Bug Fixes

* avoid using `Promise.allSettled` in preload function ([#19805](https://github.com/vitejs/vite/issues/19805)) ([35c7f35](https://github.com/vitejs/vite/commit/35c7f35e2b67f2158ededf2af58ecec53b3f16c5))
* backward compat for internal plugin `transform` calls ([#19878](https://github.com/vitejs/vite/issues/19878)) ([a152b7c](https://github.com/vitejs/vite/commit/a152b7cbac72e05668f8fc23074d531ecebb77a5))

## [6.3.0](https://github.com/vitejs/vite/compare/v6.3.0-beta.2...v6.3.0) (2025-04-16)
### Bug Fixes

* **hmr:** avoid infinite loop happening with `hot.invalidate` in circular deps ([#19870](https://github.com/vitejs/vite/issues/19870)) ([d4ee5e8](https://github.com/vitejs/vite/commit/d4ee5e8655a85f4d6bebc695b063d69406ab53ac))
* **preview:** use host url to open browser ([#19836](https://github.com/vitejs/vite/issues/19836)) ([5003434](https://github.com/vitejs/vite/commit/50034340401b4043bb0b158f18ffb7ae1b7f5c86))

## [6.3.0-beta.2](https://github.com/vitejs/vite/compare/v6.3.0-beta.1...v6.3.0-beta.2) (2025-04-11)
### Bug Fixes

* addWatchFile doesn't work if base is specified (fixes [#19792](https://github.com/vitejs/vite/issues/19792)) ([#19794](https://github.com/vitejs/vite/issues/19794)) ([8bed1de](https://github.com/vitejs/vite/commit/8bed1de5710f2a097af0e22a196545446d98f988))
* correct the behavior when multiple transform filter options are specified ([#19818](https://github.com/vitejs/vite/issues/19818)) ([7200dee](https://github.com/vitejs/vite/commit/7200deec91a501fb84734e23906f80808734540c))
* **css:** remove empty chunk imports correctly when chunk file name contained special characters ([#19814](https://github.com/vitejs/vite/issues/19814)) ([b125172](https://github.com/vitejs/vite/commit/b1251720d47f15615ea354991cdaa90d9a94aae5))
* **dev:** make query selector regexes more inclusive (fix [#19213](https://github.com/vitejs/vite/issues/19213)) ([#19767](https://github.com/vitejs/vite/issues/19767)) ([f530a72](https://github.com/vitejs/vite/commit/f530a72246ec8e73b1f2ba767f6c108e9ac9712a))
* fs check with svg and relative paths ([#19782](https://github.com/vitejs/vite/issues/19782)) ([62d7e81](https://github.com/vitejs/vite/commit/62d7e81ee189d65899bb65f3263ddbd85247b647))
* **hmr:** run HMR handler sequentially ([#19793](https://github.com/vitejs/vite/issues/19793)) ([380c10e](https://github.com/vitejs/vite/commit/380c10e665e78ef732a8d7b6c8f60a1226fc4c3b))
* keep entry asset files imported by other files ([#19779](https://github.com/vitejs/vite/issues/19779)) ([2fa1495](https://github.com/vitejs/vite/commit/2fa149580118a6b7988593dea9e2bf2ee679506c))
* **module-runner:** allow already resolved id as entry ([#19768](https://github.com/vitejs/vite/issues/19768)) ([e2e11b1](https://github.com/vitejs/vite/commit/e2e11b15a6083777ee521e26a3f79c3859abd411))
* reject requests with `#` in request-target ([#19830](https://github.com/vitejs/vite/issues/19830)) ([175a839](https://github.com/vitejs/vite/commit/175a83909f02d3b554452a7bd02b9f340cdfef70))
* **types:** remove the `keepProcessEnv` from the `DefaultEnvironmentOptions` type ([#19796](https://github.com/vitejs/vite/issues/19796)) ([36935b5](https://github.com/vitejs/vite/commit/36935b58eabde46ab845e121e21525df5ad65ff1))
* unbundle `fdir` to fix `commonjsOptions.dynamicRequireTargets` ([#19791](https://github.com/vitejs/vite/issues/19791)) ([71227be](https://github.com/vitejs/vite/commit/71227be9aab52c1c5df59afba4539646204eff74))

### Performance Improvements

* **css:** avoid constructing `renderedModules` ([#19775](https://github.com/vitejs/vite/issues/19775)) ([59d0b35](https://github.com/vitejs/vite/commit/59d0b35b30f3a38be33c0a9bdc177945b6f7eb1b))

### Documentation

* **vite:** fix description of `transformIndexHtml` hook ([#19799](https://github.com/vitejs/vite/issues/19799)) ([a0e1a04](https://github.com/vitejs/vite/commit/a0e1a0402648e0df60fb928ffd97b0230999990d))

### Miscellaneous Chores

* remove unused eslint directive ([#19781](https://github.com/vitejs/vite/issues/19781)) ([cb4f5b4](https://github.com/vitejs/vite/commit/cb4f5b4b6bb7dc96812b126ccc475d1e2c3f7f92))

### Code Refactoring

* simplify pluginFilter implementation ([#19828](https://github.com/vitejs/vite/issues/19828)) ([0a0c50a](https://github.com/vitejs/vite/commit/0a0c50a7ed38017469ed6dcec941c2d8d0efd0d0))

### Tests

* tweak generateCodeFrame test ([#19812](https://github.com/vitejs/vite/issues/19812)) ([8fe3538](https://github.com/vitejs/vite/commit/8fe3538d9095384c670815dc42ef67e051f3246f))

## [6.3.0-beta.1](https://github.com/vitejs/vite/compare/v6.3.0-beta.0...v6.3.0-beta.1) (2025-04-03)
### Features

* **env:** add false option for envDir to disable env loading ([#19503](https://github.com/vitejs/vite/issues/19503)) ([bca89e1](https://github.com/vitejs/vite/commit/bca89e153e58edd2b506807958557a21edacfaf8))
* **types:** make CustomPluginOptionsVite backward compatible ([#19760](https://github.com/vitejs/vite/issues/19760)) ([821edf1](https://github.com/vitejs/vite/commit/821edf196f281b90af0742647a3feaf3226be439))

### Bug Fixes

* align plugin hook filter behavior with pluginutils ([#19736](https://github.com/vitejs/vite/issues/19736)) ([0bbdd2c](https://github.com/vitejs/vite/commit/0bbdd2c1338624fa0e76c81648989f8f9a5b36d7))
* fs check in transform middleware ([#19761](https://github.com/vitejs/vite/issues/19761)) ([5967313](https://github.com/vitejs/vite/commit/59673137c45ac2bcfad1170d954347c1a17ab949))
* **hmr:** throw non-standard error info causes logical error ([#19776](https://github.com/vitejs/vite/issues/19776)) ([6b648c7](https://github.com/vitejs/vite/commit/6b648c73ae33a57f648af87204a325335afffca8))

### Performance Improvements

* only bundle node version `debug` ([#19715](https://github.com/vitejs/vite/issues/19715)) ([e435aae](https://github.com/vitejs/vite/commit/e435aae22ffda441a24332cd79226bfca55326aa))

### Miscellaneous Chores

* fix some typos in comment ([#19728](https://github.com/vitejs/vite/issues/19728)) ([35ee848](https://github.com/vitejs/vite/commit/35ee84808af3a5443019e36cba351af859113695))

## [6.3.0-beta.0](https://github.com/vitejs/vite/compare/v6.2.2...v6.3.0-beta.0) (2025-03-26)
### Features

* **config:** improve bad character warning ([#19683](https://github.com/vitejs/vite/issues/19683)) ([998303b](https://github.com/vitejs/vite/commit/998303b438734e8219715fe6883b97fb10404c16))
* **css:** support preprocessor with lightningcss ([#19071](https://github.com/vitejs/vite/issues/19071)) ([d3450ca](https://github.com/vitejs/vite/commit/d3450cae614af4c2b866903411b6d765df3e5a48))
* **experimental:** add fetchable environment interface ([#19664](https://github.com/vitejs/vite/issues/19664)) ([c5b7191](https://github.com/vitejs/vite/commit/c5b71915099cfbc15447a166f35620fa0e05c023))
* implement hook filters ([#19602](https://github.com/vitejs/vite/issues/19602)) ([04d58b4](https://github.com/vitejs/vite/commit/04d58b42ae69547f04ef8fcd574b1ee1b654dc32))
* **types:** expose `CustomPluginOptionsVite` type ([#19557](https://github.com/vitejs/vite/issues/19557)) ([15abc01](https://github.com/vitejs/vite/commit/15abc01241b0da5c4af6aa59b0bc936ccab0f0b4))
* **types:** make ImportMetaEnv strictly available ([#19077](https://github.com/vitejs/vite/issues/19077)) ([6cf5141](https://github.com/vitejs/vite/commit/6cf51417cdfc26f100c00c910e00829e48dec79c))
* **types:** type hints for hmr events ([#19579](https://github.com/vitejs/vite/issues/19579)) ([95424b2](https://github.com/vitejs/vite/commit/95424b26892b005f438169d0ea426cb1a3176bf2))
* warn if `define['process.env']` contains `path` key with a value ([#19517](https://github.com/vitejs/vite/issues/19517)) ([832b2c4](https://github.com/vitejs/vite/commit/832b2c409ebbb2ba1480e6ae4630c7f047c160d4))

### Bug Fixes

* add back `.mts` to default `resolve.extensions` ([#19701](https://github.com/vitejs/vite/issues/19701)) ([ae91bd0](https://github.com/vitejs/vite/commit/ae91bd0ad10942898c3d7aa8181249fb9682a4fe))
* **css:** parse image-set without space after comma correctly ([#19661](https://github.com/vitejs/vite/issues/19661)) ([d0d4c66](https://github.com/vitejs/vite/commit/d0d4c66bd539a5232005ac7ad63ec19f0794f2a5))
* **css:** scoped css order with non-scoped css ([#19678](https://github.com/vitejs/vite/issues/19678)) ([a3a94ab](https://github.com/vitejs/vite/commit/a3a94abb200c0bb1ed8bc4abb539a9ea27ce1a84))
* **deps:** update all non-major dependencies ([#19649](https://github.com/vitejs/vite/issues/19649)) ([f4e712f](https://github.com/vitejs/vite/commit/f4e712ff861f8a9504594a4a5e6d35a7547e5a7e))
* fs raw query with query separators ([#19702](https://github.com/vitejs/vite/issues/19702)) ([262b5ec](https://github.com/vitejs/vite/commit/262b5ec7ae4981208339b7b87fefbd3dd8465851))
* **optimizer:** fix incorrect picomatch usage in filter() ([#19646](https://github.com/vitejs/vite/issues/19646)) ([300280d](https://github.com/vitejs/vite/commit/300280d52203b6c1d8867d956f7d5c991e2e9dfb))
* **ssr:** hoist export to handle cyclic import better ([#18983](https://github.com/vitejs/vite/issues/18983)) ([8c04c69](https://github.com/vitejs/vite/commit/8c04c69a52c7b66d551d384ac34bb10ab1522f68))

### Miscellaneous Chores

* **deps:** unbundle tinyglobby ([#19487](https://github.com/vitejs/vite/issues/19487)) ([a5ea6f0](https://github.com/vitejs/vite/commit/a5ea6f09ba79f4a5b72117899bccaa43613a777f))

### Code Refactoring

* `[hookName].handler` in plugins ([#19586](https://github.com/vitejs/vite/issues/19586)) ([9827df2](https://github.com/vitejs/vite/commit/9827df2195905e5eb04b46dce357d12c3dff4876))
* **reporter:** only call modulesReporter when logLevel is info ([#19708](https://github.com/vitejs/vite/issues/19708)) ([7249553](https://github.com/vitejs/vite/commit/7249553625b667b6affb448d5acb7d6f457640f6))

## <small>[6.2.2](https://github.com/vitejs/vite/compare/v6.2.1...v6.2.2) (2025-03-14)</small>
### Features

* show friendly error for malformed `base` ([#19616](https://github.com/vitejs/vite/issues/19616)) ([2476391](https://github.com/vitejs/vite/commit/2476391b2854aaa67d0ed317b6d0c462e68028f7))
* **worker:** show asset filename conflict warning ([#19591](https://github.com/vitejs/vite/issues/19591)) ([367d968](https://github.com/vitejs/vite/commit/367d968fbf584e9f0e17192b816e92e8045c6217))

### Bug Fixes

* await client buildStart on top level buildStart ([#19624](https://github.com/vitejs/vite/issues/19624)) ([b31faab](https://github.com/vitejs/vite/commit/b31faab2a81b839e4b747baeb9c7a7cbb724f8d2))
* **css:** inline css correctly for double quote use strict ([#19590](https://github.com/vitejs/vite/issues/19590)) ([d0aa833](https://github.com/vitejs/vite/commit/d0aa833296668fc420a27a1ea88ecdbdeacdbce7))
* **deps:** update all non-major dependencies ([#19613](https://github.com/vitejs/vite/issues/19613)) ([363d691](https://github.com/vitejs/vite/commit/363d691b4995d72f26a14eb59ed88a9483b1f931))
* **indexHtml:** ensure correct URL when querying module graph ([#19601](https://github.com/vitejs/vite/issues/19601)) ([dc5395a](https://github.com/vitejs/vite/commit/dc5395a27e44066ef7725278c4057d9f1071a53f))
* **preview:** use preview https config, not server ([#19633](https://github.com/vitejs/vite/issues/19633)) ([98b3160](https://github.com/vitejs/vite/commit/98b3160fa5916189e756cd7c5aae87e0d8f1978e))
* **ssr:** use optional chaining to prevent "undefined is not an object" happening in `ssrRewriteStacktrace` ([#19612](https://github.com/vitejs/vite/issues/19612)) ([4309755](https://github.com/vitejs/vite/commit/43097550a1aa8ff633c39fb197b5f9ac1222119b))

### Miscellaneous Chores

* extend commit hash correctly when ambigious with a non-commit object ([#19600](https://github.com/vitejs/vite/issues/19600)) ([89a6287](https://github.com/vitejs/vite/commit/89a62873243805518b672212db7e317989c5c197))

## <small>[6.2.1](https://github.com/vitejs/vite/compare/v6.2.0...v6.2.1) (2025-03-07)</small>
### Features

* add `*?url&no-inline` type and warning for `.json?inline` / `.json?no-inline` ([#19566](https://github.com/vitejs/vite/issues/19566)) ([c0d3667](https://github.com/vitejs/vite/commit/c0d36677cd305e8fa89153ed6305f0e0df43d289))

### Bug Fixes

* **css:** stabilize css module hashes with lightningcss in dev mode ([#19481](https://github.com/vitejs/vite/issues/19481)) ([92125b4](https://github.com/vitejs/vite/commit/92125b41e4caa3e862bf5fd9b1941546f25d9bf2))
* **deps:** update all non-major dependencies ([#19555](https://github.com/vitejs/vite/issues/19555)) ([f612e0f](https://github.com/vitejs/vite/commit/f612e0fdf6810317b61fcca1ded125755f261d78))
* **reporter:** fix incorrect bundle size calculation with non-ASCII characters ([#19561](https://github.com/vitejs/vite/issues/19561)) ([437c0ed](https://github.com/vitejs/vite/commit/437c0ed8baa6739bbe944779b9e7515f9035046a))
* **sourcemap:** combine sourcemaps with multiple sources without matched source ([#18971](https://github.com/vitejs/vite/issues/18971)) ([e3f6ae1](https://github.com/vitejs/vite/commit/e3f6ae14f7a93118d7341de7379967f815725c4b))
* **ssr:** named export should overwrite export all ([#19534](https://github.com/vitejs/vite/issues/19534)) ([2fd2fc1](https://github.com/vitejs/vite/commit/2fd2fc110738622651d361488767734cc23c34dd))

### Performance Improvements

* flush compile cache after 10s ([#19537](https://github.com/vitejs/vite/issues/19537)) ([6c8a5a2](https://github.com/vitejs/vite/commit/6c8a5a27e645a182f5b03a4ed6aa726eab85993f))

### Miscellaneous Chores

* **css:** move environment destructuring after condition check ([#19492](https://github.com/vitejs/vite/issues/19492)) ([c9eda23](https://github.com/vitejs/vite/commit/c9eda2348c244d591d23f131c6ddf262b256cbf0))
* **html:** remove unnecessary value check ([#19491](https://github.com/vitejs/vite/issues/19491)) ([797959f](https://github.com/vitejs/vite/commit/797959f01da583b85a0be1dc89f762fd01d138db))

### Code Refactoring

* remove `isBuild` check from preAliasPlugin ([#19587](https://github.com/vitejs/vite/issues/19587)) ([c9e086d](https://github.com/vitejs/vite/commit/c9e086d35ac35ee1c6d85d48369e8a67a2ba6bfe))
* restore endsWith usage ([#19554](https://github.com/vitejs/vite/issues/19554)) ([6113a96](https://github.com/vitejs/vite/commit/6113a9670cc9b7d29fe0bffe033f7823e36ded00))
* use `applyToEnvironment` in internal plugins ([#19588](https://github.com/vitejs/vite/issues/19588)) ([f678442](https://github.com/vitejs/vite/commit/f678442d5701a00648a745956f9d884247e4e710))

### Tests

* add glob import test case ([#19516](https://github.com/vitejs/vite/issues/19516)) ([aa1d807](https://github.com/vitejs/vite/commit/aa1d8075cc7ce7fbba62fea9e37ccb9b304fc039))
* convert config playground to unit tests ([#19568](https://github.com/vitejs/vite/issues/19568)) ([c0e68da](https://github.com/vitejs/vite/commit/c0e68da4774f3487e9ba0c4d4d2c5e76bdc890ea))
* convert resolve-config playground to unit tests ([#19567](https://github.com/vitejs/vite/issues/19567)) ([db5fb48](https://github.com/vitejs/vite/commit/db5fb48f5d4c1ee411e59c1e9b70d62fdb9d53d2))

## [6.2.0](https://github.com/vitejs/vite/compare/v6.2.0-beta.1...v6.2.0) (2025-02-25)
### Bug Fixes

* **deps:** update all non-major dependencies ([#19501](https://github.com/vitejs/vite/issues/19501)) ([c94c9e0](https://github.com/vitejs/vite/commit/c94c9e052127cf4796374de1d698ec60b2973dfa))
* **worker:** string interpolation in dynamic worker options ([#19476](https://github.com/vitejs/vite/issues/19476)) ([07091a1](https://github.com/vitejs/vite/commit/07091a1e804e5934208ef0b6324a04317dd0d815))

### Miscellaneous Chores

* use unicode cross icon instead of x ([#19497](https://github.com/vitejs/vite/issues/19497)) ([5c70296](https://github.com/vitejs/vite/commit/5c70296ffb22fe5a0f4039835aa14feb096b4a97))

## [6.2.0-beta.1](https://github.com/vitejs/vite/compare/v6.2.0-beta.0...v6.2.0-beta.1) (2025-02-21)
### Bug Fixes

* **css:** temporary add `?.` after `this.getModuleInfo` in `vite:css-post` ([#19478](https://github.com/vitejs/vite/issues/19478)) ([12b0b8a](https://github.com/vitejs/vite/commit/12b0b8a953ad7d08ba0540cb4f5cb26a7fa69da2))

## [6.2.0-beta.0](https://github.com/vitejs/vite/compare/v6.1.1...v6.2.0-beta.0) (2025-02-21)
### Features

* **css:** allow scoping css to importers exports ([#19418](https://github.com/vitejs/vite/issues/19418)) ([3ebd838](https://github.com/vitejs/vite/commit/3ebd83833f723dde64098bc617c61b37adb3ad01))
* show `mode` on server start and add env debugger ([#18808](https://github.com/vitejs/vite/issues/18808)) ([c575b82](https://github.com/vitejs/vite/commit/c575b825596ccaedfac1cfecbb9a464e5e584a60))
* use host url to open browser ([#19414](https://github.com/vitejs/vite/issues/19414)) ([f6926ca](https://github.com/vitejs/vite/commit/f6926caa1f2c9433ca544172378412795722d8e1))

### Miscellaneous Chores

* bump esbuild to 0.25.0 ([#19389](https://github.com/vitejs/vite/issues/19389)) ([73987f2](https://github.com/vitejs/vite/commit/73987f22ec3f2df0d36154f1766ca7a7dc4c2460))

## <small>[6.1.1](https://github.com/vitejs/vite/compare/v6.1.0...v6.1.1) (2025-02-19)</small>
### Features

* add support for injecting debug IDs ([#18763](https://github.com/vitejs/vite/issues/18763)) ([0ff556a](https://github.com/vitejs/vite/commit/0ff556a6d9b55bff7cac17396ce7d4397becacaa))

### Bug Fixes

* **css:** run rewrite plugin if postcss plugin exists ([#19371](https://github.com/vitejs/vite/issues/19371)) ([bcdb51a](https://github.com/vitejs/vite/commit/bcdb51a1ac082f4e8ed6f820787d6745dfaa972d))
* **deps:** bump tsconfck ([#19375](https://github.com/vitejs/vite/issues/19375)) ([746a583](https://github.com/vitejs/vite/commit/746a583d42592a31e1e8e80cc790a7c9e6acf58e))
* **deps:** update all non-major dependencies ([#19392](https://github.com/vitejs/vite/issues/19392)) ([60456a5](https://github.com/vitejs/vite/commit/60456a54fe90872dbd4bed332ecbd85bc88deb92))
* **deps:** update all non-major dependencies ([#19440](https://github.com/vitejs/vite/issues/19440)) ([ccac73d](https://github.com/vitejs/vite/commit/ccac73d9d0e92c7232f09207d1d6b893e823ed8e))
* ensure `.[cm]?[tj]sx?` static assets are JS mime ([#19453](https://github.com/vitejs/vite/issues/19453)) ([e7ba55e](https://github.com/vitejs/vite/commit/e7ba55e7d57ad97ab43682b152159e29fa4b3753))
* **html:** ignore malformed src attrs ([#19397](https://github.com/vitejs/vite/issues/19397)) ([aff7812](https://github.com/vitejs/vite/commit/aff7812f0aed059c05ca36c86bf907d25964119a))
* ignore `*.ipv4` address in cert ([#19416](https://github.com/vitejs/vite/issues/19416)) ([973283b](https://github.com/vitejs/vite/commit/973283bf84c3dca42e2e20a9f9b8761011878b8b))
* **worker:** fix web worker type detection ([#19462](https://github.com/vitejs/vite/issues/19462)) ([edc65ea](https://github.com/vitejs/vite/commit/edc65eafa332b57ce44835deb7d7707e2d036c24))

### Miscellaneous Chores

* update 6.1.0 changelog ([#19363](https://github.com/vitejs/vite/issues/19363)) ([fa7c211](https://github.com/vitejs/vite/commit/fa7c211bf3e51269f8a8601e5994fb3ebb6859f9))

### Code Refactoring

* remove custom .jxl mime ([#19457](https://github.com/vitejs/vite/issues/19457)) ([0c85464](https://github.com/vitejs/vite/commit/0c854645bd17960abbe8f01b602d1a1da1a2b9fd))

## [6.1.0](https://github.com/vitejs/vite/compare/v6.1.0-beta.2...v6.1.0) (2025-02-05)
### Features

* show hosts in cert in CLI ([#19317](https://github.com/vitejs/vite/issues/19317)) ([a5e306f](https://github.com/vitejs/vite/commit/a5e306f2fc34fc70d543028c319367ff9b232ea0))
* support for env var for defining allowed hosts ([#19325](https://github.com/vitejs/vite/issues/19325)) ([4d88f6c](https://github.com/vitejs/vite/commit/4d88f6c9391f96275b1359f1343ee2ec3e1adb7b))
* use native runtime to import the config ([#19178](https://github.com/vitejs/vite/issues/19178)) ([7c2a794](https://github.com/vitejs/vite/commit/7c2a7942cc8494a98fbc2b0235d91faf25242d30))
* print `port` in the logged error message after failed WS connection with `EADDRINUSE` ([#19212](https://github.com/vitejs/vite/issues/19212)) ([14027b0](https://github.com/vitejs/vite/commit/14027b0f2a9b01c14815c38aab22baf5b29594bb))
* add support for `.jxl` ([#18855](https://github.com/vitejs/vite/issues/18855)) ([57b397c](https://github.com/vitejs/vite/commit/57b397c4aa3d3c657e0117c2468800d627049c8d))
* add the `builtins` environment `resolve` ([#18584](https://github.com/vitejs/vite/issues/18584)) ([2c2d521](https://github.com/vitejs/vite/commit/2c2d521abfd7a3263b5082f9420738ad0ef67c71))
* call Logger for plugin logs in build ([#13757](https://github.com/vitejs/vite/issues/13757)) ([bf3e410](https://github.com/vitejs/vite/commit/bf3e41082932f4bf7d828e18ab0346b2ac8b59c9))
* **css:** add friendly errors for IE hacks that are not supported by lightningcss ([#19072](https://github.com/vitejs/vite/issues/19072)) ([caad985](https://github.com/vitejs/vite/commit/caad985abca6450d56ca3d4e27e1e859fe8909b9))
* export `defaultAllowedOrigins` for user-land config and 3rd party plugins ([#19259](https://github.com/vitejs/vite/issues/19259)) ([dc8946b](https://github.com/vitejs/vite/commit/dc8946b9f6483ca7d63df3a5cbba307f1c21041e))
* expose createServerModuleRunnerTransport ([#18730](https://github.com/vitejs/vite/issues/18730)) ([8c24ee4](https://github.com/vitejs/vite/commit/8c24ee4b4fcfa16fdd8bb699643a92ee81f9c92b))
* **optimizer:** support bun text lockfile ([#18403](https://github.com/vitejs/vite/issues/18403)) ([05b005f](https://github.com/vitejs/vite/commit/05b005fc25a1e8dda749fb14149aa2f3c988b6a1))
* **reporter:** add `wasm` to the compressible assets regex ([#19085](https://github.com/vitejs/vite/issues/19085)) ([ce84142](https://github.com/vitejs/vite/commit/ce84142110584eadfccbd6ce9319573358af31a6))
* support async for proxy.bypass ([#18940](https://github.com/vitejs/vite/issues/18940)) ([a6b9587](https://github.com/vitejs/vite/commit/a6b958741bd97d631aba21aa5925bbf2bca65dac))
* support log related functions in dev ([#18922](https://github.com/vitejs/vite/issues/18922)) ([3766004](https://github.com/vitejs/vite/commit/3766004289fde3300d1278fcf35f3bb980d9785f))
* use module runner to import the config ([#18637](https://github.com/vitejs/vite/issues/18637)) ([b7e0e42](https://github.com/vitejs/vite/commit/b7e0e42098dd2d42285a9d3c4f39c48a580367e7))
* **worker:** support dynamic worker option fields ([#19010](https://github.com/vitejs/vite/issues/19010)) ([d0c3523](https://github.com/vitejs/vite/commit/d0c35232c6ccbcf448941328df34d15e9f73919b))

### Bug Fixes

* avoid builtStart during vite optimize ([#19356](https://github.com/vitejs/vite/issues/19356)) ([fdb36e0](https://github.com/vitejs/vite/commit/fdb36e076969c763d4249f6db890f8bf26e9f5d1))
* **build:** fix stale build manifest on watch rebuild ([#19361](https://github.com/vitejs/vite/issues/19361)) ([fcd5785](https://github.com/vitejs/vite/commit/fcd578587b2fbdef0ff8de8a0d97c9fc6da19ce1))
* allow expanding env vars in reverse order ([#19352](https://github.com/vitejs/vite/issues/19352)) ([3f5f2bd](https://github.com/vitejs/vite/commit/3f5f2bddf142b2d1b162d4553d26f1ff0758b10d))
* avoid packageJson without name in `resolveLibCssFilename` ([#19324](https://github.com/vitejs/vite/issues/19324)) ([f183bdf](https://github.com/vitejs/vite/commit/f183bdf2a799e703672ab1887d707ce120053eb2))
* **html:** fix css disorder when building multiple entry html ([#19143](https://github.com/vitejs/vite/issues/19143)) ([e7b4ba3](https://github.com/vitejs/vite/commit/e7b4ba37f90a033036326b45023a1753584dd259))
* **css:** less `[@plugin](https://github.com/plugin)` imports of JS files treated as CSS and rebased (fix [#19268](https://github.com/vitejs/vite/issues/19268)) ([#19269](https://github.com/vitejs/vite/issues/19269)) ([602b373](https://github.com/vitejs/vite/commit/602b373dcdc755816ce28913873f70550347e936))
* **deps:** update all non-major dependencies ([#19296](https://github.com/vitejs/vite/issues/19296)) ([2bea7ce](https://github.com/vitejs/vite/commit/2bea7cec4b7fddbd5f2fb6090a7eaf5ae7ca0f1b))
* don't call buildStart hooks for `vite optimize` ([#19347](https://github.com/vitejs/vite/issues/19347)) ([19ffad0](https://github.com/vitejs/vite/commit/19ffad0a5aaf8c0ff55409e746048431b8b6640d))
* don't call next middleware if user sent response in proxy.bypass ([#19318](https://github.com/vitejs/vite/issues/19318)) ([7e6364d](https://github.com/vitejs/vite/commit/7e6364de2b0f3bf65aefaf451646ca500bad8239))
* **resolve:** preserve hash/search of file url ([#19300](https://github.com/vitejs/vite/issues/19300)) ([d1e1b24](https://github.com/vitejs/vite/commit/d1e1b24c57328b5a808b981829503caa6ffadb56))
* **resolve:** warn if node-like builtin was imported when `resolve.builtin` is empty ([#19312](https://github.com/vitejs/vite/issues/19312)) ([b7aba0b](https://github.com/vitejs/vite/commit/b7aba0bc925f6d672bbb6a1e6c8c5c123a3bef55))
* respect top-level `server.preTransformRequests` ([#19272](https://github.com/vitejs/vite/issues/19272)) ([12aaa58](https://github.com/vitejs/vite/commit/12aaa585bc3fac403bf93f48ea117482cc7f43b1))
* **ssr:** fix transform error due to export all id scope ([#19331](https://github.com/vitejs/vite/issues/19331)) ([e28bce2](https://github.com/vitejs/vite/commit/e28bce244918dac27b26d4e428f86b323a1c51ba))
* **ssr:** pretty print plugin error in `ssrLoadModule` ([#19290](https://github.com/vitejs/vite/issues/19290)) ([353c467](https://github.com/vitejs/vite/commit/353c467610e2d92c0929fa4abd03f2cbd26e34ed))
* use `nodeLikeBuiltins` for `ssr.target: 'webworker'` without `noExternal: true` ([#19313](https://github.com/vitejs/vite/issues/19313)) ([9fc31b6](https://github.com/vitejs/vite/commit/9fc31b6e4d4f2a5bd9711d4f84dcb55061ebead0))
* change ResolvedConfig type to interface to allow extending it ([#19210](https://github.com/vitejs/vite/issues/19210)) ([bc851e3](https://github.com/vitejs/vite/commit/bc851e31d88cb26a2cba3fa46763bcd368e8df36))
* correctly resolve hmr dep ids and fallback to url  ([#18840](https://github.com/vitejs/vite/issues/18840)) ([b84498b](https://github.com/vitejs/vite/commit/b84498b6def7d57ff6719da2d2baf6e29f0bb819))
* **deps:** update all non-major dependencies ([#19190](https://github.com/vitejs/vite/issues/19190)) ([f2c07db](https://github.com/vitejs/vite/commit/f2c07dbfc874b46f6e09bb04996d0514663e4544))
* **hmr:** register inlined assets as a dependency of CSS file ([#18979](https://github.com/vitejs/vite/issues/18979)) ([eb22a74](https://github.com/vitejs/vite/commit/eb22a74d29813d30be48d4413d785eedb0064b2c))
* make `--force` work for all environments ([#18901](https://github.com/vitejs/vite/issues/18901)) ([51a42c6](https://github.com/vitejs/vite/commit/51a42c6b6a285fb1f092be5bbd2e18cd1fe2b214))
* **resolve:** support resolving TS files by JS extension specifiers in JS files ([#18889](https://github.com/vitejs/vite/issues/18889)) ([612332b](https://github.com/vitejs/vite/commit/612332b9bbe8d489265aea31c9c9a712319abc51))
* **ssr:** combine empty source mappings ([#19226](https://github.com/vitejs/vite/issues/19226)) ([ba03da2](https://github.com/vitejs/vite/commit/ba03da2a8c9ea6b26533cbcc4e50d58dc36499e2))
* use loc.file from rollup errors if available ([#19222](https://github.com/vitejs/vite/issues/19222)) ([ce3fe23](https://github.com/vitejs/vite/commit/ce3fe236de625de745643e127e27f2a5b52c6d2e))
* **utils:** clone `RegExp` values with `new RegExp` instead of `structuredClone` (fix [#19245](https://github.com/vitejs/vite/issues/19245), fix [#18875](https://github.com/vitejs/vite/issues/18875)) ([#19247](https://github.com/vitejs/vite/issues/19247)) ([56ad2be](https://github.com/vitejs/vite/commit/56ad2bef0353a4d00cd18789de7f4e7e5329d663))

### Performance Improvements

* **css:** only run postcss when needed ([#19061](https://github.com/vitejs/vite/issues/19061)) ([30194fa](https://github.com/vitejs/vite/commit/30194fa1e41dda6470aa20f2bb34655c4bfd9cd1))

### Documentation

* rephrase browser range and features relation ([#19286](https://github.com/vitejs/vite/issues/19286)) ([97569ef](https://github.com/vitejs/vite/commit/97569efd9d26b5c24d3a702d3171426f97c403cc))
* update `build.manifest` jsdocs ([#19332](https://github.com/vitejs/vite/issues/19332)) ([4583781](https://github.com/vitejs/vite/commit/45837817dea1fd76fbc3dcf05ca7fcd46daa7b23))

### Code Refactoring

* deprecate `vite optimize` command ([#19348](https://github.com/vitejs/vite/issues/19348)) ([6e0e3c0](https://github.com/vitejs/vite/commit/6e0e3c0b990f1132db923e4599e18b270baa3a93))

### Miscellaneous Chores

* update deprecate links domain ([#19353](https://github.com/vitejs/vite/issues/19353)) ([2b2299c](https://github.com/vitejs/vite/commit/2b2299cbac37548a163f0523c0cb92eb70a9aacf))
* **deps:** update dependency strip-literal to v3 ([#19231](https://github.com/vitejs/vite/issues/19231)) ([1172d65](https://github.com/vitejs/vite/commit/1172d655c19e689e03e6a6346eefe3ac7cc5baad))
* remove outdated code comment about `scanImports` not being used in ssr ([#19285](https://github.com/vitejs/vite/issues/19285)) ([fbbc6da](https://github.com/vitejs/vite/commit/fbbc6da186d72b7c2ad1efce22d42d302f673516))
* unneeded name in lockfileFormats ([#19275](https://github.com/vitejs/vite/issues/19275)) ([96092cb](https://github.com/vitejs/vite/commit/96092cb566ee50881edb391187d33f71af8f47b1))

### Beta Changelogs


#### [6.1.0-beta.2](https://github.com/vitejs/vite/compare/v6.1.0-beta.1...v6.1.0-beta.2) (2025-02-04)

See [6.1.0-beta.2 changelog](https://github.com/vitejs/vite/blob/v6.1.0-beta.2/packages/vite/CHANGELOG.md)


#### [6.1.0-beta.1](https://github.com/vitejs/vite/compare/v6.1.0-beta.0...v6.1.0-beta.1) (2025-02-04)

See [6.1.0-beta.1 changelog](https://github.com/vitejs/vite/blob/v6.1.0-beta.1/packages/vite/CHANGELOG.md)


#### [6.1.0-beta.0](https://github.com/vitejs/vite/compare/v6.0.11...v6.1.0-beta.0) (2025-01-24)

See [6.1.0-beta.0 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.10/packages/vite/CHANGELOG.md)

## <small>[6.0.11](https://github.com/vitejs/vite/compare/v6.0.10...v6.0.11) (2025-01-21)</small>
### Bug Fixes

* `preview.allowedHosts` with specific values was not respected ([#19246](https://github.com/vitejs/vite/issues/19246)) ([aeb3ec8](https://github.com/vitejs/vite/commit/aeb3ec84a288d6be227a1284607f13428a4f14a1))
* allow CORS from loopback addresses by default ([#19249](https://github.com/vitejs/vite/issues/19249)) ([3d03899](https://github.com/vitejs/vite/commit/3d038997377a30022b6a6b7916e0b4b5d8b9a363))

## <small>[6.0.10](https://github.com/vitejs/vite/compare/v6.0.9...v6.0.10) (2025-01-20)</small>
### Bug Fixes

* try parse `server.origin` URL ([#19241](https://github.com/vitejs/vite/issues/19241)) ([2495022](https://github.com/vitejs/vite/commit/2495022420fda05ee389c2dcf26921b21e2aed3b))

## <small>[6.0.9](https://github.com/vitejs/vite/compare/v6.0.8...v6.0.9) (2025-01-20)</small>
### ⚠ BREAKING CHANGES

* check host header to prevent DNS rebinding attacks and introduce `server.allowedHosts`
* default `server.cors: false` to disallow fetching from untrusted origins

### Bug Fixes

* check host header to prevent DNS rebinding attacks and introduce `server.allowedHosts` ([bd896fb](https://github.com/vitejs/vite/commit/bd896fb5f312fc0ff1730166d1d142fc0d34ba6d))
* default `server.cors: false` to disallow fetching from untrusted origins ([b09572a](https://github.com/vitejs/vite/commit/b09572acc939351f4e4c50ddf793017a92c678b1))
* verify token for HMR WebSocket connection ([029dcd6](https://github.com/vitejs/vite/commit/029dcd6d77d3e3ef10bc38e9a0829784d9760fdb))

## <small>[6.0.8](https://github.com/vitejs/vite/compare/v6.0.7...v6.0.8) (2025-01-20)</small>
### Bug Fixes

* avoid SSR HMR for HTML files ([#19193](https://github.com/vitejs/vite/issues/19193)) ([3bd55bc](https://github.com/vitejs/vite/commit/3bd55bcb7e831d2c4f66c90d7bbb3e1fbf7a02b6))
* build time display 7m 60s ([#19108](https://github.com/vitejs/vite/issues/19108)) ([cf0d2c8](https://github.com/vitejs/vite/commit/cf0d2c8e232a1af716c71cdd2218d180f7ecc02b))
* **deps:** update all non-major dependencies ([#19098](https://github.com/vitejs/vite/issues/19098)) ([8639538](https://github.com/vitejs/vite/commit/8639538e6498d1109da583ad942c1472098b5919))
* don't resolve URL starting with double slash ([#19059](https://github.com/vitejs/vite/issues/19059)) ([35942cd](https://github.com/vitejs/vite/commit/35942cde11fd8a68fa89bf25f7aa1ddb87d775b2))
* ensure `server.close()` only called once ([#19204](https://github.com/vitejs/vite/issues/19204)) ([db81c2d](https://github.com/vitejs/vite/commit/db81c2dada961f40c0882b5182adf2f34bb5c178))
* **optimizer:** use correct default install state path for yarn PnP ([#19119](https://github.com/vitejs/vite/issues/19119)) ([e690d8b](https://github.com/vitejs/vite/commit/e690d8bb1e5741e81df5b7a6a5c8c3c1c971fa41))
* resolve.conditions in ResolvedConfig was `defaultServerConditions` ([#19174](https://github.com/vitejs/vite/issues/19174)) ([ad75c56](https://github.com/vitejs/vite/commit/ad75c56dce5618a3a416e18f9a5c3880d437a107))
* tree shake stringified JSON imports ([#19189](https://github.com/vitejs/vite/issues/19189)) ([f2aed62](https://github.com/vitejs/vite/commit/f2aed62d0bf1b66e870ee6b4aab80cd1702793ab))
* **types:** improve `ESBuildOptions.include / exclude` type to allow `readonly (string | RegExp)[]` ([#19146](https://github.com/vitejs/vite/issues/19146)) ([ea53e70](https://github.com/vitejs/vite/commit/ea53e7095297ea4192490fd58556414cc59a8975))
* use shared sigterm callback ([#19203](https://github.com/vitejs/vite/issues/19203)) ([47039f4](https://github.com/vitejs/vite/commit/47039f4643179be31a8d7c7fbff83c5c13deb787))

### Miscellaneous Chores

* **deps:** update dependency pathe to v2 ([#19139](https://github.com/vitejs/vite/issues/19139)) ([71506f0](https://github.com/vitejs/vite/commit/71506f0a8deda5254cb49c743cd439dfe42859ce))

## <small>[6.0.7](https://github.com/vitejs/vite/compare/v6.0.6...v6.0.7) (2025-01-02)</small>
### Features

* **css:** show lightningcss warnings ([#19076](https://github.com/vitejs/vite/issues/19076)) ([b07c036](https://github.com/vitejs/vite/commit/b07c036faf6849fe5ffd03125f25dc00f460f8ba))

### Bug Fixes

* fix `minify` when `builder.sharedPlugins: true` ([#19025](https://github.com/vitejs/vite/issues/19025)) ([f7b1964](https://github.com/vitejs/vite/commit/f7b1964d3a93a21f80b61638fa6ae9606d0a6f4f))
* **html:** error while removing `vite-ignore` attribute for inline script ([#19062](https://github.com/vitejs/vite/issues/19062)) ([a492253](https://github.com/vitejs/vite/commit/a4922537a8d705da7769d30626a0d846511fc124))
* skip the plugin if it has been called before with the same id and importer ([#19016](https://github.com/vitejs/vite/issues/19016)) ([b178c90](https://github.com/vitejs/vite/commit/b178c90c7d175ea31f8b67dccad3918f820357a4))
* **ssr:** fix semicolon injection by ssr transform ([#19097](https://github.com/vitejs/vite/issues/19097)) ([1c102d5](https://github.com/vitejs/vite/commit/1c102d517de52531faf5765632703977a17de65a))

### Performance Improvements

* skip globbing for static path in warmup ([#19107](https://github.com/vitejs/vite/issues/19107)) ([677508b](https://github.com/vitejs/vite/commit/677508bf8268a7b8661e5557a3d0a2a76cab8bd1))

## <small>[6.0.6](https://github.com/vitejs/vite/compare/v6.0.5...v6.0.6) (2024-12-26)</small>
### Bug Fixes

* **css:** resolve style tags in HTML files correctly for lightningcss ([#19001](https://github.com/vitejs/vite/issues/19001)) ([afff05c](https://github.com/vitejs/vite/commit/afff05c03266fc76d5ab8928215c89f5992f40f8))
* **css:** show correct error when unknown placeholder is used for CSS modules pattern in lightningcss ([#19070](https://github.com/vitejs/vite/issues/19070)) ([9290d85](https://github.com/vitejs/vite/commit/9290d85b5d2ad64991bd296157cb3bcb959c341d))
* replace runner-side path normalization with `fetchModule`-side resolve ([#18361](https://github.com/vitejs/vite/issues/18361)) ([9f10261](https://github.com/vitejs/vite/commit/9f10261e7609098b832fd0fb23a64840b3a0d1a0))
* **resolve:** handle package.json with UTF-8 BOM ([#19000](https://github.com/vitejs/vite/issues/19000)) ([902567a](https://github.com/vitejs/vite/commit/902567ac5327e915ce65d090045fa4922ef9f2b5))
* **ssrTransform:** preserve line offset when transforming imports ([#19004](https://github.com/vitejs/vite/issues/19004)) ([1aa434e](https://github.com/vitejs/vite/commit/1aa434e8017012bf0939b2ff1a3a66b4bd12b76d))

### Reverts

* unpin esbuild version ([#19043](https://github.com/vitejs/vite/issues/19043)) ([8bfe247](https://github.com/vitejs/vite/commit/8bfe247511517c631a26f3931bb3c93a7b0b7446))

### Miscellaneous Chores

* fix typo in comment ([#19067](https://github.com/vitejs/vite/issues/19067)) ([eb06ec3](https://github.com/vitejs/vite/commit/eb06ec30bb02ced66274f0fc6e90aff2bb20c632))
* update comment about `build.target` ([#19047](https://github.com/vitejs/vite/issues/19047)) ([0e9e81f](https://github.com/vitejs/vite/commit/0e9e81f622f13d78ee238c0fa72ba920e23419f4))

### Tests

* **ssr:** test virtual module with query ([#19044](https://github.com/vitejs/vite/issues/19044)) ([a1f4b46](https://github.com/vitejs/vite/commit/a1f4b46896cb4b442b54a8336db8eca6df9ee02d))

## <small>[6.0.5](https://github.com/vitejs/vite/compare/v6.0.4...v6.0.5) (2024-12-20)</small>
### Bug Fixes

* esbuild regression (pin to 0.24.0) ([#19027](https://github.com/vitejs/vite/issues/19027)) ([4359e0d](https://github.com/vitejs/vite/commit/4359e0d5b33afd6259a4dcef787cc2670e963126))

## <small>[6.0.4](https://github.com/vitejs/vite/compare/v6.0.3...v6.0.4) (2024-12-19)</small>
### Bug Fixes

* `this.resolve` skipSelf should not skip for different `id` or `import` ([#18903](https://github.com/vitejs/vite/issues/18903)) ([4727320](https://github.com/vitejs/vite/commit/472732057cb2273908e1fca8aa7dc18a7e1f7c74))
* **css:** escape double quotes in `url()` when lightningcss is used ([#18997](https://github.com/vitejs/vite/issues/18997)) ([3734f80](https://github.com/vitejs/vite/commit/3734f8099e3922c189497ce404fe7ff2f8929ae1))
* **css:** root relative import in sass modern API on Windows ([#18945](https://github.com/vitejs/vite/issues/18945)) ([c4b532c](https://github.com/vitejs/vite/commit/c4b532cc900bf988073583511f57bd581755d5e3))
* **css:** skip non css in custom sass importer ([#18970](https://github.com/vitejs/vite/issues/18970)) ([21680bd](https://github.com/vitejs/vite/commit/21680bdf9ca7c12f677136b56e47f46469db8be2))
* **deps:** update all non-major dependencies ([#18967](https://github.com/vitejs/vite/issues/18967)) ([d88d000](https://github.com/vitejs/vite/commit/d88d0004a8e891ca6026d356695e0b319caa7fce))
* **deps:** update all non-major dependencies ([#18996](https://github.com/vitejs/vite/issues/18996)) ([2b4f115](https://github.com/vitejs/vite/commit/2b4f115129fb3fbd730a92078acb724f8527b7f7))
* fallback terser to main thread when function options are used ([#18987](https://github.com/vitejs/vite/issues/18987)) ([12b612d](https://github.com/vitejs/vite/commit/12b612d8be2a18456fd94a2f0291d32d1ffb29d4))
* merge client and ssr values for `pluginContainer.getModuleInfo` ([#18895](https://github.com/vitejs/vite/issues/18895)) ([258cdd6](https://github.com/vitejs/vite/commit/258cdd637d1ee80a3c4571685135e89fe283f3a6))
* **optimizer:** keep NODE_ENV as-is when keepProcessEnv is `true` ([#18899](https://github.com/vitejs/vite/issues/18899)) ([8a6bb4e](https://github.com/vitejs/vite/commit/8a6bb4e11d5c1b61511ae1e5ed3ae3c65a33b2dc))
* **ssr:** recreate ssrCompatModuleRunner on restart ([#18973](https://github.com/vitejs/vite/issues/18973)) ([7d6dd5d](https://github.com/vitejs/vite/commit/7d6dd5d1d655d173668192509f63ac4ebf7af299))

### Miscellaneous Chores

* better validation error message for dts build ([#18948](https://github.com/vitejs/vite/issues/18948)) ([63b82f1](https://github.com/vitejs/vite/commit/63b82f1e29a00d06a82144fd03ea8d6eff114290))
* **deps:** update all non-major dependencies ([#18916](https://github.com/vitejs/vite/issues/18916)) ([ef7a6a3](https://github.com/vitejs/vite/commit/ef7a6a35e6827b92445e5a0c2c0022616efc80dd))
* **deps:** update dependency @rollup/plugin-node-resolve to v16 ([#18968](https://github.com/vitejs/vite/issues/18968)) ([62fad6d](https://github.com/vitejs/vite/commit/62fad6d79f83daf916dde866909a2a3dd0c79583))

### Code Refactoring

* make internal invoke event to use the same interface with `handleInvoke` ([#18902](https://github.com/vitejs/vite/issues/18902)) ([27f691b](https://github.com/vitejs/vite/commit/27f691b0c7dca2259108fe6b79583b459429bf7f))
* simplify manifest plugin code ([#18890](https://github.com/vitejs/vite/issues/18890)) ([1bfe21b](https://github.com/vitejs/vite/commit/1bfe21b9440f318c940f90e425a18588595225fd))

### Tests

* test `ModuleRunnerTransport` `invoke` API ([#18865](https://github.com/vitejs/vite/issues/18865)) ([e5f5301](https://github.com/vitejs/vite/commit/e5f5301924b775837b2a1253c37f76555bce3e3e))
* test output hash changes ([#18898](https://github.com/vitejs/vite/issues/18898)) ([bfbb130](https://github.com/vitejs/vite/commit/bfbb130fccefbe7e3880f09defb4fceacce39481))

## <small>[6.0.3](https://github.com/vitejs/vite/compare/v6.0.2...v6.0.3) (2024-12-05)</small>
### Bug Fixes

* **config:** bundle files referenced with imports field ([#18887](https://github.com/vitejs/vite/issues/18887)) ([2b5926a](https://github.com/vitejs/vite/commit/2b5926a0e79ce47d22536d38eed2629d326caca0))
* **config:** make stacktrace path correct when sourcemap is enabled ([#18833](https://github.com/vitejs/vite/issues/18833)) ([20fdf21](https://github.com/vitejs/vite/commit/20fdf210ee0ac0824b2db74876527cb7f378a9e8))
* **css:** rewrite url when image-set and url exist at the same time ([#18868](https://github.com/vitejs/vite/issues/18868)) ([d59efd8](https://github.com/vitejs/vite/commit/d59efd8dfd1c5bf2e7c45c7cdb1c0abc2a05ba02))
* **deps:** update all non-major dependencies ([#18853](https://github.com/vitejs/vite/issues/18853)) ([5c02236](https://github.com/vitejs/vite/commit/5c0223636fa277d5daeb4d93c3f32d9f3cd69fc5))
* handle postcss load unhandled rejections ([#18886](https://github.com/vitejs/vite/issues/18886)) ([d5fb653](https://github.com/vitejs/vite/commit/d5fb653c15903ccf84a093f212da86f0327a9a6f))
* **html:** allow unexpected question mark in tag name ([#18852](https://github.com/vitejs/vite/issues/18852)) ([1b54e50](https://github.com/vitejs/vite/commit/1b54e506a44420d0c8a9e000cf45b1c4f5e33026))
* make handleInvoke interface compatible with invoke ([#18876](https://github.com/vitejs/vite/issues/18876)) ([a1dd396](https://github.com/vitejs/vite/commit/a1dd396da856401a12c921d0cd2c4e97cb63f1b5))
* make result interfaces for `ModuleRunnerTransport[#invoke](https://github.com/vitejs/vite/issues/invoke)` more explicit ([#18851](https://github.com/vitejs/vite/issues/18851)) ([a75fc31](https://github.com/vitejs/vite/commit/a75fc3193d5e8d8756dfb3a046873e9c222bb6c8))
* merge `environments.ssr.resolve` with root `ssr` config ([#18857](https://github.com/vitejs/vite/issues/18857)) ([3104331](https://github.com/vitejs/vite/commit/310433106e1e8a0c39dc397e3eace8a71a2416c2))
* **module-runner:** decode uri for file url passed to import ([#18837](https://github.com/vitejs/vite/issues/18837)) ([88e49aa](https://github.com/vitejs/vite/commit/88e49aa0418cb3f6b579b744ba59daeda68432f3))
* no permission to create vite config file ([#18844](https://github.com/vitejs/vite/issues/18844)) ([ff47778](https://github.com/vitejs/vite/commit/ff47778004d609dbeef7f192783e6f253dd66237))
* remove CSS import in CJS correctly in some cases ([#18885](https://github.com/vitejs/vite/issues/18885)) ([690a36f](https://github.com/vitejs/vite/commit/690a36ffdb7d6f6568f35a304b4904e7aa475f17))

### Miscellaneous Chores

* fix duplicate attributes issue number in comment ([#18860](https://github.com/vitejs/vite/issues/18860)) ([ffee618](https://github.com/vitejs/vite/commit/ffee61893cfe9f2b0db4aecf9ddb62ca79c80458))

### Code Refactoring

* fix logic errors found by no-unnecessary-condition rule ([#18891](https://github.com/vitejs/vite/issues/18891)) ([ea802f8](https://github.com/vitejs/vite/commit/ea802f8f8bcf3771a35c1eaf687378613fbabb24))

## <small>[6.0.2](https://github.com/vitejs/vite/compare/v6.0.1...v6.0.2) (2024-12-02)</small>
### Features

* **css:** format lightningcss error ([#18818](https://github.com/vitejs/vite/issues/18818)) ([dac7992](https://github.com/vitejs/vite/commit/dac7992e8725234007c7515f86f543992874c7b8))

### Bug Fixes

* **css:** referencing aliased svg asset with lightningcss enabled errored ([#18819](https://github.com/vitejs/vite/issues/18819)) ([ae68958](https://github.com/vitejs/vite/commit/ae6895869157e48b32088f0a1f85d2fddb2d713f))
* don't store temporary vite config file in `node_modules` if deno ([#18823](https://github.com/vitejs/vite/issues/18823)) ([a20267b](https://github.com/vitejs/vite/commit/a20267bb93118468a2e20f0f77b77ed7bfa94165))
* **manifest:** use `style.css` as a key for the style file for `cssCodesplit: false` ([#18820](https://github.com/vitejs/vite/issues/18820)) ([ec51115](https://github.com/vitejs/vite/commit/ec511152558cb573acf55e88e5244bdead1b5a17))
* **optimizer:** resolve all promises when cancelled ([#18826](https://github.com/vitejs/vite/issues/18826)) ([d6e6194](https://github.com/vitejs/vite/commit/d6e6194706f0e3a889caa9303de2293cc0f131b2))
* **resolve:** don't set builtinModules to `external` by default ([#18821](https://github.com/vitejs/vite/issues/18821)) ([2250ffa](https://github.com/vitejs/vite/commit/2250ffac62e55c89232d745d2f99ece539be9195))
* **ssr:** set `ssr.target: 'webworker'` defaults as fallback ([#18827](https://github.com/vitejs/vite/issues/18827)) ([b39e696](https://github.com/vitejs/vite/commit/b39e69638b3e2e658ff6712be83b549b28103c3d))

### Miscellaneous Chores

* run typecheck in unit tests ([#18858](https://github.com/vitejs/vite/issues/18858)) ([49f20bb](https://github.com/vitejs/vite/commit/49f20bb77749ec7b44344fd9c42d593ae20c78f0))
* update broken links in changelog ([#18802](https://github.com/vitejs/vite/issues/18802)) ([cb754f8](https://github.com/vitejs/vite/commit/cb754f8acc1b579dae9fe70a08e3ef53984402cc))
* update broken links in changelog ([#18804](https://github.com/vitejs/vite/issues/18804)) ([47ec49f](https://github.com/vitejs/vite/commit/47ec49ffa170cac5d04cf2eef01f45e0b5ccde03))

### Code Refactoring

* make properties of ResolvedServerOptions and ResolvedPreviewOptions required ([#18796](https://github.com/vitejs/vite/issues/18796)) ([51a5569](https://github.com/vitejs/vite/commit/51a5569e66bd7f0de79ac14b9e902d1382ccd0aa))

## <small>[6.0.1](https://github.com/vitejs/vite/compare/v6.0.0...v6.0.1) (2024-11-27)</small>
### Bug Fixes

* default empty server `proxy` prevents starting http2 server ([#18788](https://github.com/vitejs/vite/issues/18788)) ([bbaf514](https://github.com/vitejs/vite/commit/bbaf514fb718952e0f17a15545c593125f1d1b9c))
* **manifest:** do not override existing js manifest entry  ([#18776](https://github.com/vitejs/vite/issues/18776)) ([3b0837e](https://github.com/vitejs/vite/commit/3b0837e0b997e14dacc347719353b8b0cea35bda))
* **server:** close _ssrCompatModuleRunner on server close ([#18784](https://github.com/vitejs/vite/issues/18784)) ([9b4c410](https://github.com/vitejs/vite/commit/9b4c410dddb80c8858549355e175735976a82134))
* **server:** skip hot channel client normalization for wsServer  ([#18782](https://github.com/vitejs/vite/issues/18782)) ([cc7670a](https://github.com/vitejs/vite/commit/cc7670abaffeda1338cf3acfef2bc41a38c223a0))
* **worker:** fix `applyToEnvironment` hooks on worker build ([#18793](https://github.com/vitejs/vite/issues/18793)) ([0c6cdb0](https://github.com/vitejs/vite/commit/0c6cdb0f88d32ce041272977e786006008223f44))

### Reverts

* update moduleResolution value casing ([#18409](https://github.com/vitejs/vite/issues/18409)) ([#18774](https://github.com/vitejs/vite/issues/18774)) ([b0fc6e3](https://github.com/vitejs/vite/commit/b0fc6e3c2591a30360d3714263cf7cc0e2acbfdf))

### Miscellaneous Chores

* flat v6 config file ([#18777](https://github.com/vitejs/vite/issues/18777)) ([c7b3308](https://github.com/vitejs/vite/commit/c7b330832675ee6385ee1a8750762e496c8e18e6))
* split changelog ([#18787](https://github.com/vitejs/vite/issues/18787)) ([8542632](https://github.com/vitejs/vite/commit/8542632b3b205b61999b6d998928d5fb17ba90c4))
* update changelog for v6 ([#18773](https://github.com/vitejs/vite/issues/18773)) ([b254fac](https://github.com/vitejs/vite/commit/b254fac4aa35a3522aeafb3259e60acd050aeb51))

## [6.0.0](https://github.com/vitejs/vite/compare/v6.0.0-beta.10...v6.0.0) (2024-11-26)

![Vite 6 is out!](../../docs/public/og-image-announcing-vite6.png)

Today, we're taking another big step in Vite's story. The Vite [team](https://vite.dev/team), [contributors](https://github.com/vitejs/vite/graphs/contributors), and ecosystem partners are excited to announce the release of the next Vite major:

- **[Vite 6.0 announcement blog post](https://vite.dev/blog/announcing-vite6.html)**
- [Docs](https://vite.dev/)
- Translations: [简体中文](https://cn.vite.dev/), [日本語](https://ja.vite.dev/), [Español](https://es.vite.dev/), [Português](https://pt.vite.dev/), [한국어](https://ko.vite.dev/), [Deutsch](https://de.vite.dev/)
- [Migration Guide](https://vite.dev/guide/migration.html)

We want to thank the more than [1K contributors to Vite Core](https://github.com/vitejs/vite/graphs/contributors) and the maintainers and contributors of Vite plugins, integrations, tools, and translations that have helped us craft this new major. We invite you to get involved and help us improve Vite for the whole ecosystem. Learn more at our [Contributing Guide](https://github.com/vitejs/vite/blob/main/CONTRIBUTING.md).

### ⚠ BREAKING CHANGES

* drop node 21 support in version ranges (#18729)
* **deps:** update dependency dotenv-expand to v12 (#18697)
* **resolve:** allow removing conditions (#18395)
* **html:** support more asset sources (#11138)
* remove fs.cachedChecks option (#18493)
* proxy bypass with WebSocket (#18070)
* **css:** remove default import in ssr dev (#17922)
* **lib:** use package name for css output file name (#18488)
* update to chokidar v4 (#18453)
* support `file://` resolution (#18422)
* **deps:** update postcss-load-config to v6 (#15235)
* **css:** change default sass api to modern/modern-compiler (#17937)
* **css:** load postcss config within workspace root only (#18440)
* default `build.cssMinify` to `'esbuild'` for SSR (#15637)
* **json:** add `json.stringify: 'auto'` and make that the default (#18303)
* bump minimal terser version to 5.16.0 (#18209)
* **deps:** migrate `fast-glob` to `tinyglobby` (#18243)

### Features

* add support for .cur type ([#18680](https://github.com/vitejs/vite/issues/18680)) ([5ec9eed](https://github.com/vitejs/vite/commit/5ec9eedc80bbf39a33b498198ba07ed1bd9cacc7))
* drop node 21 support in version ranges ([#18729](https://github.com/vitejs/vite/issues/18729)) ([a384d8f](https://github.com/vitejs/vite/commit/a384d8fd39162190675abcfea31ba657383a3d03))
* enable HMR by default on ModuleRunner side ([#18749](https://github.com/vitejs/vite/issues/18749)) ([4d2abc7](https://github.com/vitejs/vite/commit/4d2abc7bba95cf516ce7341d5d8f349d61b75224))
* support `module-sync` condition when loading config if enabled ([#18650](https://github.com/vitejs/vite/issues/18650)) ([cf5028d](https://github.com/vitejs/vite/commit/cf5028d4bf0a0d59b4a98323beaadc268204056b))
* add `isSsrTargetWebWorker` flag to `configEnvironment` hook ([#18620](https://github.com/vitejs/vite/issues/18620)) ([3f5fab0](https://github.com/vitejs/vite/commit/3f5fab04aa64c0e9b45068e842f033583b365de0))
* add `ssr.resolve.mainFields` option ([#18646](https://github.com/vitejs/vite/issues/18646)) ([a6f5f5b](https://github.com/vitejs/vite/commit/a6f5f5baca7a5d2064f5f4cb689764ad939fab4b))
* expose default mainFields/conditions ([#18648](https://github.com/vitejs/vite/issues/18648)) ([c12c653](https://github.com/vitejs/vite/commit/c12c653ca5fab354e0f71394e2fbe636dccf6b2f))
* extended applyToEnvironment and perEnvironmentPlugin ([#18544](https://github.com/vitejs/vite/issues/18544)) ([8fa70cd](https://github.com/vitejs/vite/commit/8fa70cdfa65ce8254ab8da8be0d92614126764c0))
* **optimizer:** allow users to specify their esbuild `platform` option ([#18611](https://github.com/vitejs/vite/issues/18611)) ([0924879](https://github.com/vitejs/vite/commit/09248795ca79a7053b803af8977c3422f5cd5824))
* show error when accessing variables not exposed in CJS build ([#18649](https://github.com/vitejs/vite/issues/18649)) ([87c5502](https://github.com/vitejs/vite/commit/87c55022490d4710934c482abf5fbd4fcda9c3c9))
* **asset:** add `?inline` and `?no-inline` queries to control inlining ([#15454](https://github.com/vitejs/vite/issues/15454)) ([9162172](https://github.com/vitejs/vite/commit/9162172e039ae67ad4ee8dce18f04b7444f7d9de))
* **asset:** inline svg in dev if within limit ([#18581](https://github.com/vitejs/vite/issues/18581)) ([f08b146](https://github.com/vitejs/vite/commit/f08b1463db50f39b571faa871d05c92b10f3434c))
* use a single transport for fetchModule and HMR support ([#18362](https://github.com/vitejs/vite/issues/18362)) ([78dc490](https://github.com/vitejs/vite/commit/78dc4902ffef7f316e84d21648b04dc62dd0ae0a))
* **html:** support more asset sources ([#11138](https://github.com/vitejs/vite/issues/11138)) ([8a7af50](https://github.com/vitejs/vite/commit/8a7af50b5ddf72f21098406e9668bc609b323899))
* **resolve:** allow removing conditions ([#18395](https://github.com/vitejs/vite/issues/18395)) ([d002e7d](https://github.com/vitejs/vite/commit/d002e7d05a0f23110f9185b39222819bcdfffc16))
* **html:** support `vite-ignore` attribute to opt-out of processing ([#18494](https://github.com/vitejs/vite/issues/18494)) ([d951310](https://github.com/vitejs/vite/commit/d9513104e21175e1d23e0f614df55cd53291ab4e))
* **lib:** use package name for css output file name ([#18488](https://github.com/vitejs/vite/issues/18488)) ([61cbf6f](https://github.com/vitejs/vite/commit/61cbf6f2cfcd5afc91fe0a0ad56abfc36a32f1ab))
* log complete config in debug mode ([#18289](https://github.com/vitejs/vite/issues/18289)) ([04f6736](https://github.com/vitejs/vite/commit/04f6736fd7ac3da22141929c01a151f5a6fe4e45))
* proxy bypass with WebSocket ([#18070](https://github.com/vitejs/vite/issues/18070)) ([3c9836d](https://github.com/vitejs/vite/commit/3c9836d96f118ff5748916241bc3871a54247ad1))
* support `file://` resolution ([#18422](https://github.com/vitejs/vite/issues/18422)) ([6a7e313](https://github.com/vitejs/vite/commit/6a7e313754dce5faa5cd7c1e2343448cd7f3a2a2))
* update to chokidar v4 ([#18453](https://github.com/vitejs/vite/issues/18453)) ([192d555](https://github.com/vitejs/vite/commit/192d555f88bba7576e8a40cc027e8a11e006079c))
* allow custom `console` in `createLogger` ([#18379](https://github.com/vitejs/vite/issues/18379)) ([0c497d9](https://github.com/vitejs/vite/commit/0c497d9cb63bd4a6bb8e01c0e3b843890a239d23))
* **css:** add more stricter typing of lightningcss ([#18460](https://github.com/vitejs/vite/issues/18460)) ([b9b925e](https://github.com/vitejs/vite/commit/b9b925eb3f911ab63972124dc8ab0455449b925d))
* **css:** change default sass api to modern/modern-compiler ([#17937](https://github.com/vitejs/vite/issues/17937)) ([d4e0442](https://github.com/vitejs/vite/commit/d4e0442f9d6adc70b72ea0713dc8abb4b1f75ae4))
* read `sec-fetch-dest` header to detect JS in transform ([#9981](https://github.com/vitejs/vite/issues/9981)) ([e51dc40](https://github.com/vitejs/vite/commit/e51dc40b5907cf14d7aefaaf01fb8865a852ef15))
* **css:** load postcss config within workspace root only ([#18440](https://github.com/vitejs/vite/issues/18440)) ([d23a493](https://github.com/vitejs/vite/commit/d23a493cc4b54a2e2b2c1337b3b1f0c9b1be311e))
* **json:** add `json.stringify: 'auto'` and make that the default ([#18303](https://github.com/vitejs/vite/issues/18303)) ([b80daa7](https://github.com/vitejs/vite/commit/b80daa7c0970645dca569d572892648f66c6799c))
* add .git to deny list by default ([#18382](https://github.com/vitejs/vite/issues/18382)) ([105ca12](https://github.com/vitejs/vite/commit/105ca12b34e466dc9de838643954a873ac1ce804))
* add `environment::listen` ([#18263](https://github.com/vitejs/vite/issues/18263)) ([4d5f51d](https://github.com/vitejs/vite/commit/4d5f51d13f92cc8224a028c27df12834a0667659))
* enable dependencies discovery and pre-bundling in ssr environments ([#18358](https://github.com/vitejs/vite/issues/18358)) ([9b21f69](https://github.com/vitejs/vite/commit/9b21f69405271f1b864fa934a96adcb0e1a2bc4d))
* restrict characters useable for environment name ([#18255](https://github.com/vitejs/vite/issues/18255)) ([9ab6180](https://github.com/vitejs/vite/commit/9ab6180d3a20be71eb7aedef000f8c4ae3591c40))
* support arbitrary module namespace identifier imports from cjs deps ([#18236](https://github.com/vitejs/vite/issues/18236)) ([4389a91](https://github.com/vitejs/vite/commit/4389a917f8f5e8e67222809fb7b166bb97f6d02c))
* introduce RunnableDevEnvironment ([#18190](https://github.com/vitejs/vite/issues/18190)) ([fb292f2](https://github.com/vitejs/vite/commit/fb292f226f988e80fee4f4aea878eb3d5d229022))
* support `this.environment` in `options` and `onLog` hook ([#18142](https://github.com/vitejs/vite/issues/18142)) ([7722c06](https://github.com/vitejs/vite/commit/7722c061646bc8587f55f560bfe06b2a9643639a))
* **css:** support es2023 build target for lightningcss ([#17998](https://github.com/vitejs/vite/issues/17998)) ([1a76300](https://github.com/vitejs/vite/commit/1a76300cd16827f0640924fdc21747ce140c35fb))
* Environment API ([#16471](https://github.com/vitejs/vite/issues/16471)) ([242f550](https://github.com/vitejs/vite/commit/242f550eb46c93896fca6b55495578921e29a8af))
* expose `EnvironmentOptions` type ([#18080](https://github.com/vitejs/vite/issues/18080)) ([35cf59c](https://github.com/vitejs/vite/commit/35cf59c9d53ef544eb5f2fe2f9ff4d6cb225e63b))

### Bug Fixes

* `createRunnableDevEnvironment` returns `RunnableDevEnvironment`, not `DevEnvironment` ([#18673](https://github.com/vitejs/vite/issues/18673)) ([74221c3](https://github.com/vitejs/vite/commit/74221c391bffd61b9ef39b7c0f9ea2e405913a6f))
* `getModulesByFile` should return a `serverModule` ([#18715](https://github.com/vitejs/vite/issues/18715)) ([b80d5ec](https://github.com/vitejs/vite/commit/b80d5ecbbcc374bd8f32b2ed5ceb3cbfffaae77b))
* catch error in full reload handler ([#18713](https://github.com/vitejs/vite/issues/18713)) ([a10e741](https://github.com/vitejs/vite/commit/a10e7410656d3614cbfd07ba772776ff334a8d60))
* **client:** overlay not appearing when multiple vite clients were loaded ([#18647](https://github.com/vitejs/vite/issues/18647)) ([27d70b5](https://github.com/vitejs/vite/commit/27d70b5fa61f1c1a836d52809549cb57569f42a4))
* **deps:** update all non-major dependencies ([#18691](https://github.com/vitejs/vite/issues/18691)) ([f005461](https://github.com/vitejs/vite/commit/f005461ecce89ada21cb0c021f7af460b5479736))
* **deps:** update dependency dotenv-expand to v12 ([#18697](https://github.com/vitejs/vite/issues/18697)) ([0c658de](https://github.com/vitejs/vite/commit/0c658de41f4c1576c526a8c48a8ea0a019c6311c))
* display pre-transform error details ([#18764](https://github.com/vitejs/vite/issues/18764)) ([554f45f](https://github.com/vitejs/vite/commit/554f45f4d820c57c0874ebe48ef2fddfafdd0750))
* exit code on `SIGTERM` ([#18741](https://github.com/vitejs/vite/issues/18741)) ([cc55e36](https://github.com/vitejs/vite/commit/cc55e36dd39fef134568f53acc66514cbb7175ea))
* expose missing `InterceptorOptions` type ([#18766](https://github.com/vitejs/vite/issues/18766)) ([6252c60](https://github.com/vitejs/vite/commit/6252c6035695365c93773fbe06a4b2a307e86368))
* **html:** fix inline proxy modules invalidation ([#18696](https://github.com/vitejs/vite/issues/18696)) ([8ab04b7](https://github.com/vitejs/vite/commit/8ab04b70ada119fbca2fc5a53c36f233423febbe))
* log error when send in module runner failed ([#18753](https://github.com/vitejs/vite/issues/18753)) ([ba821bb](https://github.com/vitejs/vite/commit/ba821bb63eca6d8a9199ee2253ef2607375f5702))
* **module-runner:** make evaluator optional ([#18672](https://github.com/vitejs/vite/issues/18672)) ([fd1283f](https://github.com/vitejs/vite/commit/fd1283fe27cc1a19b5c7d9d72664832e4daa1bbf))
* **optimizer:** detect npm / yarn / pnpm dependency changes correctly ([#17336](https://github.com/vitejs/vite/issues/17336)) ([#18560](https://github.com/vitejs/vite/issues/18560)) ([818cf3e](https://github.com/vitejs/vite/commit/818cf3e7bf1b6c2dc56e7cd8f056bc1d185c2cd7))
* **optimizer:** trigger onCrawlEnd after manual included deps are registered ([#18733](https://github.com/vitejs/vite/issues/18733)) ([dc60410](https://github.com/vitejs/vite/commit/dc6041099ccd5767764fb8c99a169869bbd13f16))
* **optimizer:** workaround firefox's false warning for no sources source map ([#18665](https://github.com/vitejs/vite/issues/18665)) ([473424e](https://github.com/vitejs/vite/commit/473424ee8d6b743c1565bf0749deb5d9fbedcea7))
* **ssr:** replace `__vite_ssr_identity__` with `(0, ...)` and inject `;` between statements ([#18748](https://github.com/vitejs/vite/issues/18748)) ([94546be](https://github.com/vitejs/vite/commit/94546be18354a457bced5107aa31533b09e304ec))
* cjs build for perEnvironmentState et al ([#18656](https://github.com/vitejs/vite/issues/18656)) ([95c4b3c](https://github.com/vitejs/vite/commit/95c4b3c371dc7fb12c28cb1307f6f389887eb1e1))
* **html:** externalize `rollup.external` scripts correctly ([#18618](https://github.com/vitejs/vite/issues/18618)) ([55461b4](https://github.com/vitejs/vite/commit/55461b43329db6a5e737eab591163a8681ba9230))
* include more modules to prefix-only module list ([#18667](https://github.com/vitejs/vite/issues/18667)) ([5a2103f](https://github.com/vitejs/vite/commit/5a2103f0d486a7725c23c70710b11559c00e9b93))
* **ssr:** format `ssrTransform` parse error  ([#18644](https://github.com/vitejs/vite/issues/18644)) ([d9be921](https://github.com/vitejs/vite/commit/d9be92187cb17d740856af27d0ab60c84e04d58c))
* **ssr:** preserve fetchModule error details ([#18626](https://github.com/vitejs/vite/issues/18626)) ([866a433](https://github.com/vitejs/vite/commit/866a433a34ab2f6d2910506e781b346091de1b9e))
* browser field should not be included by default for `consumer: 'server'` ([#18575](https://github.com/vitejs/vite/issues/18575)) ([87b2347](https://github.com/vitejs/vite/commit/87b2347a13ea8ae8282f0f1e2233212c040bfed8))
* **client:** detect ws close correctly ([#18548](https://github.com/vitejs/vite/issues/18548)) ([637d31b](https://github.com/vitejs/vite/commit/637d31bcc59d964e51f7969093cc369deee88ca1))
* **resolve:** run ensureVersionQuery for SSR ([#18591](https://github.com/vitejs/vite/issues/18591)) ([63207e5](https://github.com/vitejs/vite/commit/63207e5d0fbedc8ddddb7d1faaa8ea9a45a118d4))
* use `server.perEnvironmentStartEndDuringDev` ([#18549](https://github.com/vitejs/vite/issues/18549)) ([fe30349](https://github.com/vitejs/vite/commit/fe30349d350ef08bccd56404ccc3e6d6e0a2e156))
* allow nested dependency selector to be used for `optimizeDeps.include` for SSR ([#18506](https://github.com/vitejs/vite/issues/18506)) ([826c81a](https://github.com/vitejs/vite/commit/826c81a40bb25914d55cd2e96b548f1a2c384a19))
* asset `new URL(,import.meta.url)` match ([#18194](https://github.com/vitejs/vite/issues/18194)) ([5286a90](https://github.com/vitejs/vite/commit/5286a90a3c1b693384f99903582a1f70b7b44945))
* close watcher if it's disabled ([#18521](https://github.com/vitejs/vite/issues/18521)) ([85bd0e9](https://github.com/vitejs/vite/commit/85bd0e9b0dc637c7645f2b56f93071d6e1ec149c))
* **config:** write temporary vite config to node_modules ([#18509](https://github.com/vitejs/vite/issues/18509)) ([72eaef5](https://github.com/vitejs/vite/commit/72eaef5300d20b7163050461733c3208a4013e1e))
* **css:** `cssCodeSplit` uses the current environment configuration ([#18486](https://github.com/vitejs/vite/issues/18486)) ([eefe895](https://github.com/vitejs/vite/commit/eefe8957167681b85f0e1b07bc5feefa307cccb0))
* **json:** don't `json.stringify` arrays ([#18541](https://github.com/vitejs/vite/issues/18541)) ([fa50b03](https://github.com/vitejs/vite/commit/fa50b03390dae280293174f65f850522599b9ab7))
* **less:** prevent rebasing `[@import](https://github.com/import) url(...)` ([#17857](https://github.com/vitejs/vite/issues/17857)) ([aec5fdd](https://github.com/vitejs/vite/commit/aec5fdd72e3aeb2aa26796001b98f3f330be86d1))
* **lib:** only resolve css bundle name if have styles ([#18530](https://github.com/vitejs/vite/issues/18530)) ([5d6dc49](https://github.com/vitejs/vite/commit/5d6dc491b6bb78613694eaf686e2e305b71af5e1))
* **scss:** improve error logs ([#18522](https://github.com/vitejs/vite/issues/18522)) ([3194a6a](https://github.com/vitejs/vite/commit/3194a6a60714a3978f5e4b39d6223f32a8dc01ef))
* `define` in environment config was not working ([#18515](https://github.com/vitejs/vite/issues/18515)) ([052799e](https://github.com/vitejs/vite/commit/052799e8939cfcdd7a7ff48daf45a766bf6cc546))
* **build:** apply resolve.external/noExternal to server environments ([#18495](https://github.com/vitejs/vite/issues/18495)) ([5a967cb](https://github.com/vitejs/vite/commit/5a967cb596c7c4b0548be1d9025bc1e34b36169a))
* **config:** remove error if require resolve to esm ([#18437](https://github.com/vitejs/vite/issues/18437)) ([f886f75](https://github.com/vitejs/vite/commit/f886f75396cdb5a43ec5377bbbaaffc0e8ae03e9))
* consider URLs with any protocol to be external ([#17369](https://github.com/vitejs/vite/issues/17369)) ([a0336bd](https://github.com/vitejs/vite/commit/a0336bd5197bb4427251be4c975e30fb596c658f))
* **css:** remove default import in ssr dev ([#17922](https://github.com/vitejs/vite/issues/17922)) ([eccf663](https://github.com/vitejs/vite/commit/eccf663e35a17458425860895bb30b3b0613ea96))
* use picomatch to align with tinyglobby ([#18503](https://github.com/vitejs/vite/issues/18503)) ([437795d](https://github.com/vitejs/vite/commit/437795db8307ce4491d066bcaaa5bd9432193773))
* **css:** `cssCodeSplit` in `environments.xxx.build` is invalid ([#18464](https://github.com/vitejs/vite/issues/18464)) ([993e71c](https://github.com/vitejs/vite/commit/993e71c4cb227bd8c347b918f52ccd83f85a645a))
* **css:** make sass types work with sass-embedded ([#18459](https://github.com/vitejs/vite/issues/18459)) ([89f8303](https://github.com/vitejs/vite/commit/89f8303e727791aa7be6f35833a708b6a50e9120))
* **deps:** update all non-major dependencies ([#18484](https://github.com/vitejs/vite/issues/18484)) ([2ec12df](https://github.com/vitejs/vite/commit/2ec12df98d07eb4c986737e86a4a9f8066724658))
* handle warmup glob hang ([#18462](https://github.com/vitejs/vite/issues/18462)) ([409fa5c](https://github.com/vitejs/vite/commit/409fa5c9dee0e394bcdc3b111f5b2e4261131ca0))
* **manifest:** non entry CSS chunk src was wrong ([#18133](https://github.com/vitejs/vite/issues/18133)) ([c148676](https://github.com/vitejs/vite/commit/c148676c90dc4823bc6bdeb8ba1e36386c5d9654))
* **module-runner:** delay function eval until module runner instantiation ([#18480](https://github.com/vitejs/vite/issues/18480)) ([472afbd](https://github.com/vitejs/vite/commit/472afbd010db3f1c7a59826c7bf4067191b7f48a))
* **plugins:** noop if config hook returns same config reference ([#18467](https://github.com/vitejs/vite/issues/18467)) ([bd540d5](https://github.com/vitejs/vite/commit/bd540d52eb609ca12dad8e2f3fe8011821bda878))
* return the same instance of ModuleNode for the same EnvironmentModuleNode ([#18455](https://github.com/vitejs/vite/issues/18455)) ([5ead461](https://github.com/vitejs/vite/commit/5ead461b374d76ceb134063477eaf3f97fe3da97))
* set scripts imported by HTML moduleSideEffects=true ([#18411](https://github.com/vitejs/vite/issues/18411)) ([2ebe4b4](https://github.com/vitejs/vite/commit/2ebe4b44430dd311028f72520ac977bb202ce50b))
* use websocket to test server liveness before client reload ([#17891](https://github.com/vitejs/vite/issues/17891)) ([7f9f8c6](https://github.com/vitejs/vite/commit/7f9f8c6851d1eb49a72dcb6c134873148a2e81eb))
* add typing to `CSSOptions.preprocessorOptions` ([#18001](https://github.com/vitejs/vite/issues/18001)) ([7eeb6f2](https://github.com/vitejs/vite/commit/7eeb6f2f97abf5dfc71c225b9cff9779baf2ed2f))
* default `build.cssMinify` to `'esbuild'` for SSR ([#15637](https://github.com/vitejs/vite/issues/15637)) ([f1d3bf7](https://github.com/vitejs/vite/commit/f1d3bf74cc7f12e759442fd7111d07e2c0262a67))
* **dev:** prevent double URL encoding in server.open on macOS ([#18443](https://github.com/vitejs/vite/issues/18443)) ([56b7176](https://github.com/vitejs/vite/commit/56b71768f3ee498962fba898804086299382bb59))
* **preview:** set resolvedUrls null after close ([#18445](https://github.com/vitejs/vite/issues/18445)) ([65014a3](https://github.com/vitejs/vite/commit/65014a32ef618619c5a34b729d67340d9253bdd5))
* **ssr:** inject identity function at the top ([#18449](https://github.com/vitejs/vite/issues/18449)) ([0ab20a3](https://github.com/vitejs/vite/commit/0ab20a3ee26eacf302415b3087732497d0a2f358))
* **ssr:** preserve source maps for hoisted imports (fix [#16355](https://github.com/vitejs/vite/issues/16355)) ([#16356](https://github.com/vitejs/vite/issues/16356)) ([8e382a6](https://github.com/vitejs/vite/commit/8e382a6a1fed2cd41051b81f9cd9c94b484352a5))
* augment hash for CSS files to prevent chromium erroring by loading previous files ([#18367](https://github.com/vitejs/vite/issues/18367)) ([a569f42](https://github.com/vitejs/vite/commit/a569f42ee93229308be7a327b7a71e79f3d58b01))
* **cli:** `--watch` should not override `build.watch` options ([#18390](https://github.com/vitejs/vite/issues/18390)) ([b2965c8](https://github.com/vitejs/vite/commit/b2965c8e9f74410bc8047a05528c74b68a3856d7))
* **css:** don't transform sass function calls with namespace ([#18414](https://github.com/vitejs/vite/issues/18414)) ([dbb2604](https://github.com/vitejs/vite/commit/dbb260499f894d495bcff3dcdf5635d015a2f563))
* **deps:** update `open` dependency to 10.1.0 ([#18349](https://github.com/vitejs/vite/issues/18349)) ([5cca4bf](https://github.com/vitejs/vite/commit/5cca4bfd3202c7aea690acf63f60bfe57fa165de))
* **deps:** update all non-major dependencies ([#18345](https://github.com/vitejs/vite/issues/18345)) ([5552583](https://github.com/vitejs/vite/commit/5552583a2272cd4208b30ad60e99d984e34645f0))
* more robust plugin.sharedDuringBuild ([#18351](https://github.com/vitejs/vite/issues/18351)) ([47b1270](https://github.com/vitejs/vite/commit/47b12706ce2d0c009d6078a61e16e81a04c9f49c))
* **ssr:** `this` in exported function should be `undefined` ([#18329](https://github.com/vitejs/vite/issues/18329)) ([bae6a37](https://github.com/vitejs/vite/commit/bae6a37628c4870f3db92351e8af2a7b4a07e248))
* **worker:** rewrite rollup `output.format` with `worker.format` on worker build error ([#18165](https://github.com/vitejs/vite/issues/18165)) ([dc82334](https://github.com/vitejs/vite/commit/dc823347bb857a9f63eee7e027a52236d7e331e0))
* `injectQuery` double encoding ([#18246](https://github.com/vitejs/vite/issues/18246)) ([2c5f948](https://github.com/vitejs/vite/commit/2c5f948d0646f6a0237570ab5d36b06d31cb94c9))
* add position to import analysis resolve exception ([#18344](https://github.com/vitejs/vite/issues/18344)) ([0fe95d4](https://github.com/vitejs/vite/commit/0fe95d4a71930cf55acd628efef59e6eae0f77f7))
* **assets:** make srcset parsing HTML spec compliant ([#16323](https://github.com/vitejs/vite/issues/16323)) ([#18242](https://github.com/vitejs/vite/issues/18242)) ([0e6d4a5](https://github.com/vitejs/vite/commit/0e6d4a5e23cdfb2ec433f687e455b9827269527c))
* **css:** dont remove JS chunk for pure CSS chunk when the export is used ([#18307](https://github.com/vitejs/vite/issues/18307)) ([889bfc0](https://github.com/vitejs/vite/commit/889bfc0ada6d6cd356bb7a92efdce96298f82fef))
* **deps:** bump tsconfck ([#18322](https://github.com/vitejs/vite/issues/18322)) ([67783b2](https://github.com/vitejs/vite/commit/67783b2d5513e013bf74844186eb9b2b70d17d5c))
* **deps:** update all non-major dependencies ([#18292](https://github.com/vitejs/vite/issues/18292)) ([5cac054](https://github.com/vitejs/vite/commit/5cac0544dca2764f0114aac38e9922a0c13d7ef4))
* destroy the runner when runnable environment is closed ([#18282](https://github.com/vitejs/vite/issues/18282)) ([5212d09](https://github.com/vitejs/vite/commit/5212d09579a82bc09b149c77e996d0e5c3972455))
* handle yarn command fail when root does not exist ([#18141](https://github.com/vitejs/vite/issues/18141)) ([460aaff](https://github.com/vitejs/vite/commit/460aaffbf134a9eda6e092a564afc2eeebf8f935))
* **hmr:** don't try to rewrite imports for direct CSS soft invalidation ([#18252](https://github.com/vitejs/vite/issues/18252)) ([a03bb0e](https://github.com/vitejs/vite/commit/a03bb0e2ba35af314c57fc98600bb76566592239))
* make it easier to configure environment runner ([#18273](https://github.com/vitejs/vite/issues/18273)) ([fb35a78](https://github.com/vitejs/vite/commit/fb35a7800e21ed2c6f9d0f843898afa1fcc87795))
* **middleware-mode:** call all hot.listen when server restart ([#18261](https://github.com/vitejs/vite/issues/18261)) ([007773b](https://github.com/vitejs/vite/commit/007773b550e7c6bcaeb8d88970fd6dfe999d5a4a))
* **optimizer:** don't externalize transitive dep package name with asset extension ([#18152](https://github.com/vitejs/vite/issues/18152)) ([fafc7e2](https://github.com/vitejs/vite/commit/fafc7e28d3395292fbc2f2355417dcc15871ab1e))
* **resolve:** fix resolve cache key for external conditions ([#18332](https://github.com/vitejs/vite/issues/18332)) ([93d286c](https://github.com/vitejs/vite/commit/93d286c4c1af0b379002a6ff495e82bb87acd65c))
* **resolve:** fix resolve cache to consider `conditions` and more ([#18302](https://github.com/vitejs/vite/issues/18302)) ([2017a33](https://github.com/vitejs/vite/commit/2017a330f5576dfc9db1538e0b899a1776cd100a))
* **types:** add more overload to `defineConfig` ([#18299](https://github.com/vitejs/vite/issues/18299)) ([94e34cf](https://github.com/vitejs/vite/commit/94e34cf1dfe6fdb331b6508e830b2cc446000aac))
* asset import should skip handling data URIs ([#18163](https://github.com/vitejs/vite/issues/18163)) ([70813c7](https://github.com/vitejs/vite/commit/70813c7f05fc9a45d102a53514ecac23831e6d6b))
* cache the runnable environment module runner ([#18215](https://github.com/vitejs/vite/issues/18215)) ([95020ab](https://github.com/vitejs/vite/commit/95020ab49e12d143262859e095025cf02423c1d9))
* call `this.hot.close` for non-ws HotChannel ([#18212](https://github.com/vitejs/vite/issues/18212)) ([bad0ccc](https://github.com/vitejs/vite/commit/bad0cccee80c02fa309f274220f6d324d03c3b19))
* close HotChannel on environment close ([#18206](https://github.com/vitejs/vite/issues/18206)) ([2d148e3](https://github.com/vitejs/vite/commit/2d148e347e8fbcc6f0e4e627a20acc81d9ced3e0))
* **config:** treat all files as ESM on deno ([#18081](https://github.com/vitejs/vite/issues/18081)) ([c1ed8a5](https://github.com/vitejs/vite/commit/c1ed8a595a02ec7f8f5a8d23f97b2f21d3834ab1))
* **css:** ensure sass compiler initialized only once ([#18128](https://github.com/vitejs/vite/issues/18128)) ([4cc5322](https://github.com/vitejs/vite/commit/4cc53224e9b207aa6a5a111e40ed0a0464cf37f4))
* **css:** fix lightningcss dep url resolution with custom root ([#18125](https://github.com/vitejs/vite/issues/18125)) ([eb08f60](https://github.com/vitejs/vite/commit/eb08f605ddadef99a5d68f55de143e3e47c91618))
* **css:** fix missing source file warning with sass modern api custom importer ([#18113](https://github.com/vitejs/vite/issues/18113)) ([d7763a5](https://github.com/vitejs/vite/commit/d7763a5615a238cb1b5dceb7bdfc4aac7678fb0a))
* **data-uri:** only match ids starting with `data:` ([#18241](https://github.com/vitejs/vite/issues/18241)) ([ec0efe8](https://github.com/vitejs/vite/commit/ec0efe8a06d0271ef0154f38fb9beabcd4b1bd89))
* **deps:** update all non-major dependencies ([#18170](https://github.com/vitejs/vite/issues/18170)) ([c8aea5a](https://github.com/vitejs/vite/commit/c8aea5ae0af90dc6796ef3bdd612d1eb819f157b))
* **deps:** upgrade rollup 4.22.4+ to ensure avoiding XSS ([#18180](https://github.com/vitejs/vite/issues/18180)) ([ea1d0b9](https://github.com/vitejs/vite/commit/ea1d0b9af9b28b57166d4ca67bece21650221a04))
* **html:** make build-html plugin work with `sharedPlugins` ([#18214](https://github.com/vitejs/vite/issues/18214)) ([34041b9](https://github.com/vitejs/vite/commit/34041b9d8ea39aa9138d0c2417bfbe39cc9aabdc))
* **mixedModuleGraph:** handle undefined id in getModulesByFile ([#18201](https://github.com/vitejs/vite/issues/18201)) ([768a50f](https://github.com/vitejs/vite/commit/768a50f7ac668dbf876feef557d8c0f8ff32b8ff))
* **optimizer:** re-optimize when changing config `webCompatible` ([#18221](https://github.com/vitejs/vite/issues/18221)) ([a44b0a2](https://github.com/vitejs/vite/commit/a44b0a2690812788aaaba00fd3acd2c6fa36669b))
* require serialization for `HMRConnection.send` on implementation side ([#18186](https://github.com/vitejs/vite/issues/18186)) ([9470011](https://github.com/vitejs/vite/commit/9470011570503a917021915c47e6a2f36aae16b5))
* **ssr:** fix source map remapping with multiple sources ([#18150](https://github.com/vitejs/vite/issues/18150)) ([e003a2c](https://github.com/vitejs/vite/commit/e003a2ca73b04648e14ebf40f3616838e2da3d6d))
* use `config.consumer` instead of `options?.ssr` / `config.build.ssr` ([#18140](https://github.com/vitejs/vite/issues/18140)) ([21ec1ce](https://github.com/vitejs/vite/commit/21ec1ce7f041efa5cd781924f7bc536ab406a197))
* **vite:** refactor "module cache" to "evaluated modules", pass down module to "runInlinedModule" ([#18092](https://github.com/vitejs/vite/issues/18092)) ([e83beff](https://github.com/vitejs/vite/commit/e83beff596072f9c7a42f6e2410f154668981d71))
* avoid DOM Clobbering gadget in `getRelativeUrlFromDocument` ([#18115](https://github.com/vitejs/vite/issues/18115)) ([ade1d89](https://github.com/vitejs/vite/commit/ade1d89660e17eedfd35652165b0c26905259fad))
* fs raw query ([#18112](https://github.com/vitejs/vite/issues/18112)) ([9d2413c](https://github.com/vitejs/vite/commit/9d2413c8b64bfb1dfd953340b4e1b5972d5440aa))
* **preload:** throw error preloading module as well ([#18098](https://github.com/vitejs/vite/issues/18098)) ([ba56cf4](https://github.com/vitejs/vite/commit/ba56cf43b5480f8519349f7d7fe60718e9af5f1a))
* allow scanning exports from `script module` in svelte ([#18063](https://github.com/vitejs/vite/issues/18063)) ([7d699aa](https://github.com/vitejs/vite/commit/7d699aa98155cbf281e3f7f6a8796dcb3b4b0fd6))
* **build:** declare `preload-helper` has no side effects ([#18057](https://github.com/vitejs/vite/issues/18057)) ([587ad7b](https://github.com/vitejs/vite/commit/587ad7b17beba50279eaf46b06c5bf5559c4f36e))
* **css:** fallback to mainthread if logger or pkgImporter option is set for sass ([#18071](https://github.com/vitejs/vite/issues/18071)) ([d81dc59](https://github.com/vitejs/vite/commit/d81dc59473b1053bf48c45a9d45f87ee6ecf2c02))
* **dynamicImportVars:** correct glob pattern for paths with parentheses ([#17940](https://github.com/vitejs/vite/issues/17940)) ([2a391a7](https://github.com/vitejs/vite/commit/2a391a7df6e5b4a8d9e8313fba7ddf003df41e12))
* ensure req.url matches moduleByEtag URL to avoid incorrect 304 ([#17997](https://github.com/vitejs/vite/issues/17997)) ([abf04c3](https://github.com/vitejs/vite/commit/abf04c3a84f4d9962a6f9697ca26cd639fa76e87))
* **html:** escape html attribute ([#18067](https://github.com/vitejs/vite/issues/18067)) ([5983f36](https://github.com/vitejs/vite/commit/5983f366d499f74d473097154bbbcc8e51476dc4))
* incorrect environment consumer option resolution ([#18079](https://github.com/vitejs/vite/issues/18079)) ([0e3467e](https://github.com/vitejs/vite/commit/0e3467e503aef45119260fe75b399b26f7a80b66))
* **preload:** allow ignoring dep errors ([#18046](https://github.com/vitejs/vite/issues/18046)) ([3fb2889](https://github.com/vitejs/vite/commit/3fb28896d916e03cef1b5bd6877ac184c7ec8003))
* store backwards compatible `ssrModule` and `ssrError` ([#18031](https://github.com/vitejs/vite/issues/18031)) ([cf8ced5](https://github.com/vitejs/vite/commit/cf8ced56ea4932e917e2c4ef3d04a87f0ab4f20b))

### Performance Improvements

* reduce bundle size for `Object.keys(import.meta.glob(...))` / `Object.values(import.meta.glob(...))` ([#18666](https://github.com/vitejs/vite/issues/18666)) ([ed99a2c](https://github.com/vitejs/vite/commit/ed99a2cd31e8d3c2b791885bcc4b188570539e45))
* **worker:** inline worker without base64 ([#18752](https://github.com/vitejs/vite/issues/18752)) ([90c66c9](https://github.com/vitejs/vite/commit/90c66c95aba3d2edd86637a77adc699f3fd6c1ff))
* remove strip-ansi for a node built-in ([#18630](https://github.com/vitejs/vite/issues/18630)) ([5182272](https://github.com/vitejs/vite/commit/5182272d52fc092a6219c8efe73ecb3f8e65a0b5))
* **css:** skip style.css extraction if code-split css ([#18470](https://github.com/vitejs/vite/issues/18470)) ([34fdb6b](https://github.com/vitejs/vite/commit/34fdb6bef558724330d2411b9666facef669b3a0))
* call `module.enableCompileCache()` ([#18323](https://github.com/vitejs/vite/issues/18323)) ([18f1dad](https://github.com/vitejs/vite/commit/18f1daddd125b07dcb8c32056ee0cec61bd65971))
* use `crypto.hash` when available ([#18317](https://github.com/vitejs/vite/issues/18317)) ([2a14884](https://github.com/vitejs/vite/commit/2a148844cf2382a5377b75066351f00207843352))

### Documentation

* rename `HotUpdateContext` to `HotUpdateOptions` ([#18718](https://github.com/vitejs/vite/issues/18718)) ([824c347](https://github.com/vitejs/vite/commit/824c347fa21aaf5bbf811994385b790db4287ab0))
* add jsdocs to flags in BuilderOptions ([#18516](https://github.com/vitejs/vite/issues/18516)) ([1507068](https://github.com/vitejs/vite/commit/1507068b6d460cf54336fe7e8d3539fdb4564bfb))
* missing changes guides ([#18491](https://github.com/vitejs/vite/issues/18491)) ([5da78a6](https://github.com/vitejs/vite/commit/5da78a6859f3b5c677d896144b915381e4497432))
* update fs.deny default in JSDoc ([#18514](https://github.com/vitejs/vite/issues/18514)) ([1fcc83d](https://github.com/vitejs/vite/commit/1fcc83dd7ade429f889e4ce19d5c67b3e5b46419))
* update homepage ([#18274](https://github.com/vitejs/vite/issues/18274)) ([a99a0aa](https://github.com/vitejs/vite/commit/a99a0aab7c600301a5c314b6071afa46915ce248))
* fix typo in proxy.ts ([#18162](https://github.com/vitejs/vite/issues/18162)) ([49087bd](https://github.com/vitejs/vite/commit/49087bd5738a2cf69ee46b10a74cfd61c18e9959))

### Reverts

* use chokidar v3 ([#18659](https://github.com/vitejs/vite/issues/18659)) ([49783da](https://github.com/vitejs/vite/commit/49783da298bc45f3f3c5ad4ce2fb1260ee8856bb))

### Miscellaneous Chores

* add 5.4.x changelogs ([#18768](https://github.com/vitejs/vite/issues/18768)) ([26b58c8](https://github.com/vitejs/vite/commit/26b58c8130f232dcd4e839a337bbe478352f23ab))
* add some comments about mimes ([#18705](https://github.com/vitejs/vite/issues/18705)) ([f07e9b9](https://github.com/vitejs/vite/commit/f07e9b9d01d790c727edc2497304f07b1ef5d28f))
* **deps:** update all non-major dependencies ([#18746](https://github.com/vitejs/vite/issues/18746)) ([0ad16e9](https://github.com/vitejs/vite/commit/0ad16e92d57453d9e5392c90fd06bda947be9de6))
* **deps:** update all non-major dependencies ([#18634](https://github.com/vitejs/vite/issues/18634)) ([e2231a9](https://github.com/vitejs/vite/commit/e2231a92af46db144b9c94fb57918ac683dc93cb))
* **deps:** update transitive deps ([#18602](https://github.com/vitejs/vite/issues/18602)) ([0c8b152](https://github.com/vitejs/vite/commit/0c8b15238b669b8ab0a3f90bcf2f690d4450e18f))
* tweak build config ([#18622](https://github.com/vitejs/vite/issues/18622)) ([2a88f71](https://github.com/vitejs/vite/commit/2a88f71aef87ed23b155af26f8aca6bb7f65e899))
* add warning for `/` mapping in `resolve.alias` ([#18588](https://github.com/vitejs/vite/issues/18588)) ([a51c254](https://github.com/vitejs/vite/commit/a51c254265bbfe3d77f834fe81a503ce27c05b32))
* **deps:** update all non-major dependencies ([#18562](https://github.com/vitejs/vite/issues/18562)) ([fb227ec](https://github.com/vitejs/vite/commit/fb227ec4402246b5a13e274c881d9de6dd8082dd))
* remove unused `ssr` variable ([#18594](https://github.com/vitejs/vite/issues/18594)) ([23c39fc](https://github.com/vitejs/vite/commit/23c39fc994a6164bc68d69e56f39735a6bb7a71d))
* fix moduleSideEffects in build script on Windows ([#18518](https://github.com/vitejs/vite/issues/18518)) ([25fe9e3](https://github.com/vitejs/vite/commit/25fe9e3b48e29d49e90d6aed5ec3825dceafec18))
* use premove instead of rimraf ([#18499](https://github.com/vitejs/vite/issues/18499)) ([f97a578](https://github.com/vitejs/vite/commit/f97a57893b3a7ddf11ca4c126b6be33cd2d9283b))
* **deps:** update postcss-load-config to v6 ([#15235](https://github.com/vitejs/vite/issues/15235)) ([3a27f62](https://github.com/vitejs/vite/commit/3a27f627df278f6c9778a55f44cb347665b65204))
* **deps:** update dependency picomatch to v4 ([#15876](https://github.com/vitejs/vite/issues/15876)) ([3774881](https://github.com/vitejs/vite/commit/377488178a7ef372d9b76526bb01fd60b97f51df))
* combine deps license with same text ([#18356](https://github.com/vitejs/vite/issues/18356)) ([b5d1a05](https://github.com/vitejs/vite/commit/b5d1a058f9dab6a6b1243c2a0b11d2c421dd3291))
* **create-vite:** mark template files as CC0 ([#18366](https://github.com/vitejs/vite/issues/18366)) ([f6b9074](https://github.com/vitejs/vite/commit/f6b90747eb2b1ad863e5f147a80c75b15e38a51b))
* **deps:** bump TypeScript to 5.6 ([#18254](https://github.com/vitejs/vite/issues/18254)) ([57a0e85](https://github.com/vitejs/vite/commit/57a0e85186b88118bf5f79dd53391676fb91afec))
* **deps:** migrate `fast-glob` to `tinyglobby` ([#18243](https://github.com/vitejs/vite/issues/18243)) ([6f74a3a](https://github.com/vitejs/vite/commit/6f74a3a1b2469a24a86743d16267b0cc3653bc4a))
* **deps:** update all non-major dependencies ([#18404](https://github.com/vitejs/vite/issues/18404)) ([802839d](https://github.com/vitejs/vite/commit/802839d48335a69eb15f71f2cd816d0b6e4d3556))
* **deps:** update dependency sirv to v3 ([#18346](https://github.com/vitejs/vite/issues/18346)) ([5ea4b00](https://github.com/vitejs/vite/commit/5ea4b00a984bc76d0d000f621ab72763a4c9a48b))
* fix grammar ([#18385](https://github.com/vitejs/vite/issues/18385)) ([8030231](https://github.com/vitejs/vite/commit/8030231596edcd688e324ea507dc1ba80564f75c))
* mark builder api experimental ([#18436](https://github.com/vitejs/vite/issues/18436)) ([b57321c](https://github.com/vitejs/vite/commit/b57321cc198ee7b9012f1be632cfd4bea006cd89))
* tiny typo ([#18374](https://github.com/vitejs/vite/issues/18374)) ([7d97a9b](https://github.com/vitejs/vite/commit/7d97a9b2ba11ab566865dcf9ee0350a9e479dfca))
* update moduleResolution value casing ([#18409](https://github.com/vitejs/vite/issues/18409)) ([ff018dc](https://github.com/vitejs/vite/commit/ff018dca959c73481ae5f8328cd77d3b02f02134))
* **deps:** update dependency @rollup/plugin-commonjs to v28 ([#18231](https://github.com/vitejs/vite/issues/18231)) ([78e749e](https://github.com/vitejs/vite/commit/78e749ea9a42e7f82dbca37c26e8ab2a5e6e0c16))
* point deprecation error URLs to main branch docs ([#18321](https://github.com/vitejs/vite/issues/18321)) ([11c0fb1](https://github.com/vitejs/vite/commit/11c0fb1388744624dac40cc267ad21dc7f85cb4e))
* update all url references of vitejs.dev to vite.dev ([#18276](https://github.com/vitejs/vite/issues/18276)) ([7052c8f](https://github.com/vitejs/vite/commit/7052c8f6fc253f0a88ff04a4c18c108f3bfdaa78))
* update built LICENSE ([69b6764](https://github.com/vitejs/vite/commit/69b6764d49dd0d04819a8aa9b4061974e0e00f62))
* update license copyright ([#18278](https://github.com/vitejs/vite/issues/18278)) ([56eb869](https://github.com/vitejs/vite/commit/56eb869a67551a257d20cba00016ea59b1e1a2c4))
* **deps:** update all non-major dependencies ([#18108](https://github.com/vitejs/vite/issues/18108)) ([a73bbaa](https://github.com/vitejs/vite/commit/a73bbaadb512a884924cc884060e50ea6d809d74))
* **deps:** update all non-major dependencies ([#18230](https://github.com/vitejs/vite/issues/18230)) ([c0edd26](https://github.com/vitejs/vite/commit/c0edd26bbfeb9a8d80ebaa420e54fbb7f165bd9b))
* **deps:** update esbuild ([#18173](https://github.com/vitejs/vite/issues/18173)) ([e59e2ca](https://github.com/vitejs/vite/commit/e59e2cacab476305c3cdfb31732c27b174fb8fe2))
* escape template tag in CHANGELOG.md ([#18126](https://github.com/vitejs/vite/issues/18126)) ([caaa683](https://github.com/vitejs/vite/commit/caaa6836e9a104cc9d63b68ad850149687ad104c))
* **optimizer:** fix typo in comment ([#18239](https://github.com/vitejs/vite/issues/18239)) ([b916ab6](https://github.com/vitejs/vite/commit/b916ab601d2ec1c842ea0c6139bf216166010e56))
* **deps:** update all non-major dependencies ([#18050](https://github.com/vitejs/vite/issues/18050)) ([7cac03f](https://github.com/vitejs/vite/commit/7cac03fa5197a72d2e2422bd0243a85a9a18abfc))
* enable some eslint rules ([#18084](https://github.com/vitejs/vite/issues/18084)) ([e9a2746](https://github.com/vitejs/vite/commit/e9a2746ca77473b1814fd05db3d299c074135fe5))
* remove npm-run-all2 ([#18083](https://github.com/vitejs/vite/issues/18083)) ([41180d0](https://github.com/vitejs/vite/commit/41180d02730a7ce7c9b6ec7ac71fc6e750dd22c6))
* silence unnecessary logs during test ([#18052](https://github.com/vitejs/vite/issues/18052)) ([a3ef052](https://github.com/vitejs/vite/commit/a3ef052d408edbec71081fd2f7b3e4b1d4ea0174))

### Code Refactoring

* first character judgment replacement regexp ([#18658](https://github.com/vitejs/vite/issues/18658)) ([58f1df3](https://github.com/vitejs/vite/commit/58f1df3288b0f9584bb413dd34b8d65671258f6f))
* introduce `mergeWithDefaults` and organize how default values for config options are set ([#18550](https://github.com/vitejs/vite/issues/18550)) ([0e1f437](https://github.com/vitejs/vite/commit/0e1f437d53683b57f0157ce3ff0b0f02acabb408))
* **resolve:** remove `allowLinkedExternal` parameter from `tryNodeResolve` ([#18670](https://github.com/vitejs/vite/issues/18670)) ([b74d363](https://github.com/vitejs/vite/commit/b74d3632693b6a829b4d1cdc2a9d4ba8234c093b))
* **resolve:** remove `environmentsOptions` parameter ([#18590](https://github.com/vitejs/vite/issues/18590)) ([3ef0bf1](https://github.com/vitejs/vite/commit/3ef0bf19a3457c46395bdcb2201bbf32807d7231))
* client-only top-level warmup ([#18524](https://github.com/vitejs/vite/issues/18524)) ([a50ff60](https://github.com/vitejs/vite/commit/a50ff6000bca46a6fe429f2c3a98c486ea5ebc8e))
* remove fs.cachedChecks option ([#18493](https://github.com/vitejs/vite/issues/18493)) ([94b0857](https://github.com/vitejs/vite/commit/94b085735372588d5f92c7f4a8cf68e8291f2db0))
* separate tsconfck caches per config in a weakmap ([#17317](https://github.com/vitejs/vite/issues/17317)) ([b9b01d5](https://github.com/vitejs/vite/commit/b9b01d57fdaf5d291c78a8156e17b534c8c51eb4))
* **css:** hide internal preprocessor types and expose types used for options ([#18458](https://github.com/vitejs/vite/issues/18458)) ([c32837c](https://github.com/vitejs/vite/commit/c32837cf868f0fdb97a22a0be8c95c433f4069c8))
* optimizeDeps back to top level ([#18465](https://github.com/vitejs/vite/issues/18465)) ([1ac22de](https://github.com/vitejs/vite/commit/1ac22de41cf5a8647847070eadeac3231c94c3ed))
* top-level createEnvironment is client-only ([#18475](https://github.com/vitejs/vite/issues/18475)) ([6022fc2](https://github.com/vitejs/vite/commit/6022fc2c87e0f59c3e6ccfa307a352a378d8273a))
* use `originalFileNames`/`names` ([#18240](https://github.com/vitejs/vite/issues/18240)) ([f2957c8](https://github.com/vitejs/vite/commit/f2957c84f69c14c882809889fbd0fc66b97ca3e9))
* bump minimal terser version to 5.16.0 ([#18209](https://github.com/vitejs/vite/issues/18209)) ([19ce525](https://github.com/vitejs/vite/commit/19ce525b974328e4668ad8c6540c2a5ea652795b))
* **resolve:** remove `tryEsmOnly` flag ([#18394](https://github.com/vitejs/vite/issues/18394)) ([7cebe38](https://github.com/vitejs/vite/commit/7cebe3847f934ff4875ff3ecc6a96a82bac5f8f4))
* use builder in `build` ([#18432](https://github.com/vitejs/vite/issues/18432)) ([cc61d16](https://github.com/vitejs/vite/commit/cc61d169a4826996f7b2289618c383f8c5c6d470))
* rename runner.destroy() to runner.close() ([#18304](https://github.com/vitejs/vite/issues/18304)) ([cd368f9](https://github.com/vitejs/vite/commit/cd368f9fed393a8649597f0e5d873504a9ac62e2))
* break circular dependencies to fix test-unit ([#18237](https://github.com/vitejs/vite/issues/18237)) ([a577828](https://github.com/vitejs/vite/commit/a577828d826805c5693d773eea4c4179e21f1a16))
* remove `_onCrawlEnd` ([#18207](https://github.com/vitejs/vite/issues/18207)) ([bea0272](https://github.com/vitejs/vite/commit/bea0272decd908cd04ac0a2c87dd0a676f218a1a))
* remove the need for "processSourceMap" ([#18187](https://github.com/vitejs/vite/issues/18187)) ([08ff233](https://github.com/vitejs/vite/commit/08ff23319964903b9f380859c216b10e577ddb6f))
* replace `parse` with `splitFileAndPostfix` ([#18185](https://github.com/vitejs/vite/issues/18185)) ([6f030ec](https://github.com/vitejs/vite/commit/6f030ec15f25a2a1d7d912f1b84d83ebb28a3515))
* use `resolvePackageData` to get rollup version ([#18208](https://github.com/vitejs/vite/issues/18208)) ([220d6ec](https://github.com/vitejs/vite/commit/220d6ec2bf3fc7063eac7c625d4ccda9a4204cb7))
* **create-vite:** use picocolors ([#18085](https://github.com/vitejs/vite/issues/18085)) ([ba37df0](https://github.com/vitejs/vite/commit/ba37df0813ad3864fc4b8c6c0b289a1f2bc00c36))
* remove custom resolveOptions from pre-alias plugin ([#18041](https://github.com/vitejs/vite/issues/18041)) ([6f60adc](https://github.com/vitejs/vite/commit/6f60adc15283c6b25218d2392738671b6ab4b392))
* remove unnecessary escape ([#18044](https://github.com/vitejs/vite/issues/18044)) ([8062d36](https://github.com/vitejs/vite/commit/8062d36773cafaec98196965d33d79887e58f437))

### Build System

* ignore cjs warning ([#18660](https://github.com/vitejs/vite/issues/18660)) ([33b0d5a](https://github.com/vitejs/vite/commit/33b0d5a6ca18e9f7c27b0159decd84fee3859e09))
* reduce package size ([#18517](https://github.com/vitejs/vite/issues/18517)) ([b83f60b](https://github.com/vitejs/vite/commit/b83f60b159f3b6f4a61db180fa03cc5b20bd110f))

### Tests

* simplify `playground/json/__tests__/ssr` ([#18701](https://github.com/vitejs/vite/issues/18701)) ([f731ca2](https://github.com/vitejs/vite/commit/f731ca21ea4cfe38418880f15f6064e156a43a5e))
* update filename regex ([#18593](https://github.com/vitejs/vite/issues/18593)) ([dd25c1a](https://github.com/vitejs/vite/commit/dd25c1ab5d5510b955fa24830bc223cacc855560))
* fix test conflict ([#18446](https://github.com/vitejs/vite/issues/18446)) ([94cd1e6](https://github.com/vitejs/vite/commit/94cd1e6f95e2434d2b52b5c16d50fe0472214634))
* remove unnecessary logs from output ([#18368](https://github.com/vitejs/vite/issues/18368)) ([f50d358](https://github.com/vitejs/vite/commit/f50d3583e2c460bb02c118371a79b5ceac9877f3))
* replace fs mocking in css module compose test ([#18413](https://github.com/vitejs/vite/issues/18413)) ([ddee0ad](https://github.com/vitejs/vite/commit/ddee0ad38fd53993155fc11174d5ee194d6648d8))
* ssr external / resolveId test ([#18327](https://github.com/vitejs/vite/issues/18327)) ([4c5cf91](https://github.com/vitejs/vite/commit/4c5cf91d124d423fe028beecda952125698c1d5d))
* test optimized dep as ssr entry ([#18301](https://github.com/vitejs/vite/issues/18301)) ([466f94a](https://github.com/vitejs/vite/commit/466f94aa6465f0a3b932f55e93660f7cf6cd936e))
* fix server-worker-runner flaky test ([#18247](https://github.com/vitejs/vite/issues/18247)) ([8f82730](https://github.com/vitejs/vite/commit/8f82730b86abed953800ade6e726f70ee55ab7fe))
* move glob test root to reduce snapshot change ([#18053](https://github.com/vitejs/vite/issues/18053)) ([04d7e77](https://github.com/vitejs/vite/commit/04d7e7749496f5d1972338c7de1502c7f6f65cb6))

### Beta Changelogs


#### [6.0.0-beta.10](https://github.com/vitejs/vite/compare/v6.0.0-beta.9...v6.0.0-beta.10) (2024-11-14)

See [6.0.0-beta.10 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.10/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.9](https://github.com/vitejs/vite/compare/v6.0.0-beta.8...v6.0.0-beta.9) (2024-11-07)

See [6.0.0-beta.9 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.9/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.8](https://github.com/vitejs/vite/compare/v6.0.0-beta.7...v6.0.0-beta.8) (2024-11-01)

See [6.0.0-beta.8 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.8/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.7](https://github.com/vitejs/vite/compare/v6.0.0-beta.6...v6.0.0-beta.7) (2024-10-30)

See [6.0.0-beta.7 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.7/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.6](https://github.com/vitejs/vite/compare/v6.0.0-beta.5...v6.0.0-beta.6) (2024-10-28)

See [6.0.0-beta.6 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.6/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.5](https://github.com/vitejs/vite/compare/v6.0.0-beta.4...v6.0.0-beta.5) (2024-10-24)

See [6.0.0-beta.5 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.5/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.4](https://github.com/vitejs/vite/compare/v6.0.0-beta.3...v6.0.0-beta.4) (2024-10-23)

See [6.0.0-beta.4 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.4/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.3](https://github.com/vitejs/vite/compare/v6.0.0-beta.2...v6.0.0-beta.3) (2024-10-15)

See [6.0.0-beta.3 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.3/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.2](https://github.com/vitejs/vite/compare/v6.0.0-beta.1...v6.0.0-beta.2) (2024-10-01)

See [6.0.0-beta.2 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.2/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.1](https://github.com/vitejs/vite/compare/v6.0.0-beta.0...v6.0.0-beta.1) (2024-09-16)

See [6.0.0-beta.1 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.1/packages/vite/CHANGELOG.md)


#### [6.0.0-beta.0](https://github.com/vitejs/vite/compare/v5.4.11...v6.0.0-beta.0) (2024-09-12)

See [6.0.0-beta.0 changelog](https://github.com/vitejs/vite/blob/v6.0.0-beta.0/packages/vite/CHANGELOG.md)



## Previous Changelogs

### 5.4.x (2024-08-07 - 2024-11-11)
See [5.4.11 changelog](https://github.com/vitejs/vite/blob/v5.4.11/packages/vite/CHANGELOG.md)

### 5.3.x (2024-06-13 - 2024-07-25)
See [5.3.5 changelog](https://github.com/vitejs/vite/blob/v5.3.5/packages/vite/CHANGELOG.md)

### 5.2.x (2024-03-20 - 2024-05-28)
See [5.2.12 changelog](https://github.com/vitejs/vite/blob/v5.2.12/packages/vite/CHANGELOG.md)

### 5.1.x (2024-02-08 - 2024-03-11)
See [5.1.6 changelog](https://github.com/vitejs/vite/blob/v5.1.6/packages/vite/CHANGELOG.md)

### 5.0.x (2023-11-16 - 2024-01-05)
See [5.0.11 changelog](https://github.com/vitejs/vite/blob/v5.0.11/packages/vite/CHANGELOG.md)

### 4.5.x (2023-10-18)
See [4.5.0 changelog](https://github.com/vitejs/vite/blob/v4.5.0/packages/vite/CHANGELOG.md)

### 4.4.x (2023-07-06 - 2023-10-05)
See [4.4.11 changelog](https://github.com/vitejs/vite/blob/v4.4.11/packages/vite/CHANGELOG.md)

### 4.3.x (2023-04-20 - 2023-05-26)
See [4.3.0 changelog](https://github.com/vitejs/vite/blob/v4.3.9/packages/vite/CHANGELOG.md)

### 4.2.x (2023-03-16 - 2023-04-18)
See [4.2.2 changelog](https://github.com/vitejs/vite/blob/v4.2.2/packages/vite/CHANGELOG.md)

### 4.1.x (2023-02-02 - 2023-02-21)
See [4.1.4 changelog](https://github.com/vitejs/vite/blob/v4.1.4/packages/vite/CHANGELOG.md)

### 4.0.x (2022-12-09 - 2023-01-03)
See [4.0.4 changelog](https://github.com/vitejs/vite/blob/v4.0.4/packages/vite/CHANGELOG.md)

### 3.2.x (2022-10-26 - 2023-04-18)
See [3.2.6 changelog](https://github.com/vitejs/vite/blob/v3.2.6/packages/vite/CHANGELOG.md)

### 3.1.x (2022-09-05 - 2022-09-19)
See [3.1.3 changelog](https://github.com/vitejs/vite/blob/v3.1.3/packages/vite/CHANGELOG.md)

### 3.0.x (2022-07-13 - 2022-08-19)
See [3.0.9 changelog](https://github.com/vitejs/vite/blob/v3.0.9/packages/vite/CHANGELOG.md)

### 2.9.x (2022-03-30 - 2022-08-12)
See [2.9.15 changelog](https://github.com/vitejs/vite/blob/v2.9.15/packages/vite/CHANGELOG.md)

### 2.8.x (2022-02-09 - 2022-03-01)
See [2.8.6 changelog](https://github.com/vitejs/vite/blob/v2.8.6/packages/vite/CHANGELOG.md)

### 2.7.x (2021-10-28 - 2021-12-28)
See [2.7.13 changelog](https://github.com/vitejs/vite/blob/v2.7.13/packages/vite/CHANGELOG.md)

### 2.6.x (2021-09-20 - 2021-10-27)
See [2.6.14 changelog](https://github.com/vitejs/vite/blob/v2.6.14/packages/vite/CHANGELOG.md)

### 2.5.x (2021-08-03 - 2021-09-13)
See [2.5.10 changelog](https://github.com/vitejs/vite/blob/v2.5.10/packages/vite/CHANGELOG.md)

### 2.4.x (2021-06-27 - 2021-07-27)
See [2.4.4 changelog](https://github.com/vitejs/vite/blob/v2.4.4/packages/vite/CHANGELOG.md)

### 2.3.x (2021-05-11 - 2021-06-19)
See [2.3.8 changelog](https://github.com/vitejs/vite/blob/v2.3.8/packages/vite/CHANGELOG.md)

### 2.2.x (2021-04-19 - 2021-05-03)
See [2.2.4 changelog](https://github.com/vitejs/vite/blob/v2.2.4/packages/vite/CHANGELOG.md)

### 2.1.x (2021-03-15 - 2021-03-31)
See [2.1.5 changelog](https://github.com/vitejs/vite/blob/v2.1.5/packages/vite/CHANGELOG.md)

### 2.0.x (2021-02-16 - 2021-03-02)
See [2.0.5 changelog](https://github.com/vitejs/vite/blob/v2.0.5/packages/vite/CHANGELOG.md)

<p>
  <img src="https://assets.solidjs.com/banner?project=Library&type=core" alt="SolidJS" />
</p>

[![Build Status](https://img.shields.io/github/actions/workflow/status/solidjs/solid/main-ci.yml?branch=main&logo=github&style=for-the-badge)](https://github.com/solidjs/solid/actions/workflows/main-ci.yml)
[![Coverage Status](https://img.shields.io/coveralls/github/solidjs/solid.svg?style=for-the-badge)](https://coveralls.io/github/solidjs/solid?branch=main)

[![NPM Version](https://img.shields.io/npm/v/solid-js.svg?style=for-the-badge)](https://www.npmjs.com/package/solid-js)
[![](https://img.shields.io/npm/dm/solid-js.svg?style=for-the-badge)](https://www.npmjs.com/package/solid-js)
[![Discord](https://img.shields.io/discord/722131463138705510?style=for-the-badge)](https://discord.com/invite/solidjs)
[![Subreddit subscribers](https://img.shields.io/reddit/subreddit-subscribers/solidjs?style=for-the-badge)](https://www.reddit.com/r/solidjs/)

**[Website](https://www.solidjs.com/) • [API Docs](https://docs.solidjs.com/) • [Features Tutorial](https://www.solidjs.com/tutorial/introduction_basics) • [Playground](https://playground.solidjs.com/?version=1.3.13#NobwRAdghgtgpmAXGGUCWEwBowBcCeADgsrgM4Ae2YZA9gK4BOAxiWGjIbY7gAQi9GcCABM4jXgF9eAM0a0YvADo1aAGzQiAtACsyAegDucAEYqA3EogcuPfr2ZCouOAGU0Ac2hqps+YpU6DW09CysrGXoIZlw0WgheAGEGCBdGAAoASn4rXgd4sj5gZhTcLF4yOFxkqNwAXV4AXgcnF3cvKDV0gAZMywT8iELeDEc4eFSm3iymgD4KqprU9JLamYBqXgBGPvCBoVwmBPTcvN4AHhN6XFx43gJiRpUrm-iVXnjEjWYAa0aQUZCCa4SSzU5nfirZaZSTgi76F63CBgga7CCwiBWISicTpGaNebnJZpXj6WblES0Zj0YEAOg8VQAompxsJcAAhfAASREJzAUEIhBUmTRYEkdSAA) • [Discord](https://discord.com/invite/solidjs)**

Solid is a declarative JavaScript library for creating user interfaces. Instead of using a Virtual DOM, it compiles its templates to real DOM nodes and updates them with fine-grained reactions. Declare your state and use it throughout your app, and when a piece of state changes, only the code that depends on it will rerun.

## At a Glance
```tsx
import { createSignal } from "solid-js";
import { render } from "solid-js/web";

function Counter() {
  const [count, setCount] = createSignal(0);
  const doubleCount = () => count() * 2;
  
  console.log("The body of the function runs once...");

  return (
    <>
      <button onClick={() => setCount(c => c + 1)}>
        {doubleCount()}
      </button>
    </>
  );
}

render(Counter, document.getElementById("app")!);
```

Try this code in our [playground](https://playground.solidjs.com/anonymous/0c88df54-91b0-4c88-bd20-e962bde49725)!

<details>
<summary>Explain this!</summary>

```tsx
import { createSignal } from "solid-js";
import { render } from "solid-js/web";

// A component is just a function that returns a DOM node
function Counter() {
  // Create a piece of reactive state, giving us an accessor, count(), and a setter, setCount()
  const [count, setCount] = createSignal(0);
  
  //To create derived state, just wrap an expression in a function
  const doubleCount = () => count() * 2;
  
  console.log("The body of the function runs once...");

  // JSX allows you to write HTML within your JavaScript function and include dynamic expressions using the { } syntax
  // The only part of this that will ever rerender is the doubleCount() text.
  return (
    <>
      <button onClick={() => setCount(c => c + 1)}>
        Increment: {doubleCount()}
      </button>
    </>
  );
}

// The render function mounts a component onto your page
render(Counter, document.getElementById("app")!);
```

Solid compiles your JSX down to efficient real DOM updates. It uses the same reactive primitives (`createSignal`) at runtime but making sure there's as little rerendering as possible. Here's what that looks like in this example:

```js
import { template as _$template } from "solid-js/web";
import { delegateEvents as _$delegateEvents } from "solid-js/web";
import { insert as _$insert } from "solid-js/web";
//The compiler pulls out any static HTML
const _tmpl$ = /*#__PURE__*/_$template(`<button>Increment: `);

import { createSignal, createEffect } from "solid-js";
import { render } from "solid-js/web";

function Counter() {
  const [count, setCount] = createSignal(0);
  
  const doubleCount = () => count() * 2;
  
  console.log("The body of the function runs once...");
  
  return (() => {
    //_el$ is a real DOM node!
    const _el$ = _tmpl$();
    _el$.$$click = () => setCount(c => c + 1);
     //This inserts the count as a child of the button in a way that allows count to update without rerendering the whole button
    _$insert(_el$, doubleCount);
    return _el$;
  })();
}
render(Counter, document.getElementById("app"));
_$delegateEvents(["click"]);
```

</details>

## Key Features

- Fine-grained updates to the real DOM
- Declarative data: model your state as a system with reactive primitives
- Render-once mental model: your components are regular JavaScript functions that run once to set up your view
- Automatic dependency tracking: accessing your reactive state subscribes to it
- [Small](https://dev.to/this-is-learning/javascript-framework-todomvc-size-comparison-504f) and [fast](https://krausest.github.io/js-framework-benchmark/current.html)
- Simple: learn a few powerful concepts that can be reused, combined, and built on top of
- Provides modern framework features like JSX, fragments, Context, Portals, Suspense, streaming SSR, progressive hydration, Error Boundaries and concurrent rendering.
- Naturally debuggable: A `<div>` is a real div, so you can use your browser's devtools to inspect the rendering
- [Web component friendly](https://github.com/solidjs/solid/tree/main/packages/solid-element#readme) and can author custom elements
- Isomorphic: render your components on the client and the server
- Universal: write [custom renderers](https://github.com/solidjs/solid/releases/tag/v1.2.0) to use Solid anywhere
- A growing community and ecosystem with active core team support

<details>
 
<summary>Quick Start</summary>

You can get started with a simple app by running the following in your terminal:

```sh
> npx degit solidjs/templates/js my-app
> cd my-app
> npm i # or yarn or pnpm
> npm run dev # or yarn or pnpm
```

Or for TypeScript:

```sh
> npx degit solidjs/templates/ts my-app
> cd my-app
> npm i # or yarn or pnpm
> npm run dev # or yarn or pnpm
```

This will create a minimal, client-rendered application powered by [Vite](https://vitejs.dev/).

Or you can install the dependencies in your own setup. To use Solid with JSX (_recommended_), run:

```sh
> npm i -D babel-preset-solid
> npm i solid-js
```

The easiest way to get set up is to add `babel-preset-solid` to your `.babelrc`, babel config for webpack, or rollup configuration:

```js
"presets": ["solid"]
```

For TypeScript to work, remember to set your `.tsconfig` to handle Solid's JSX:

```js
"compilerOptions": {
  "jsx": "preserve",
  "jsxImportSource": "solid-js",
}
```

</details>

## Why Solid?

### Performant

Meticulously engineered for performance and with half a decade of research behind it, Solid's performance is almost indistinguishable from optimized vanilla JavaScript (See Solid on the [JS Framework Benchmark](https://krausest.github.io/js-framework-benchmark/current.html)). Solid is [small](https://bundlephobia.com/package/solid-js@1.3.15) and completely tree-shakable, and [fast](https://levelup.gitconnected.com/how-we-wrote-the-fastest-javascript-ui-framework-again-db097ddd99b6) when rendering on the server, too. Whether you're writing a fully client-rendered SPA or a server-rendered app, your users see it faster than ever. ([Read more about Solid's performance](https://dev.to/ryansolid/thinking-granular-how-is-solidjs-so-performant-4g37) from the library's creator.)

### Powerful

Solid is fully-featured with everything you can expect from a modern framework. Performant state management is built-in with Context and Stores: you don't have to reach for a third party library to manage global state (if you don't want to). With Resources, you can use data loaded from the server like any other piece of state and build a responsive UI for it thanks to Suspense and concurrent rendering. And when you're ready to move to the server, Solid has full SSR and serverless support, with streaming and progressive hydration to get to interactive as quickly as possible. (Check out our full [interactive features walkthrough](https://www.solidjs.com/tutorial/introduction_basics).)

### Pragmatic

Do more with less: use simple, composable primitives without hidden rules and gotchas. In Solid, components are just functions - rendering is determined purely by how your state is used - so you're free to organize your code how you like and you don't have to learn a new rendering system. Solid encourages patterns like declarative code and read-write segregation that help keep your project maintainable, but isn't opinionated enough to get in your way.

### Productive

Solid is built on established tools like JSX and TypeScript and integrates with the Vite ecosystem. Solid's bare-metal, minimal abstractions give you direct access to the DOM, making it easy to use your favorite native JavaScript libraries like D3. And the Solid ecosystem is growing fast, with [custom primitives](https://github.com/solidjs-community/solid-primitives), [component libraries](https://kobalte.dev), and build-time utilities that let you [write Solid code in new ways](https://github.com/LXSMNSYC/solid-labels).

## More

Check out our official [documentation](https://docs.solidjs.com) or browse some [examples](https://github.com/solidjs/solid/blob/main/documentation/resources/examples.md)

## Browser Support

SolidJS Core is committed to supporting the last 2 years of modern browsers including Firefox, Safari, Chrome and Edge (for desktop and mobile devices). We do not support IE or similar sunset browsers. For server environments, we support Node LTS and the latest Deno and Cloudflare Worker runtimes.

<img src="https://saucelabs.github.io/images/opensauce/powered-by-saucelabs-badge-gray.svg?sanitize=true" alt="Testing Powered By SauceLabs" width="300"/>

## Community

Come chat with us on [Discord](https://discord.com/invite/solidjs)! Solid's creator and the rest of the core team are active there, and we're always looking for contributions.

### Contributors

<a href="https://github.com/solidjs/solid/graphs/contributors"><img src="https://opencollective.com/solid/contributors.svg?width=890&amp;button=false" style="max-width:100%;"></a>

### Open Collective

Support us with a donation and help us continue our activities. [[Contribute](https://opencollective.com/solid)]

<a href="https://opencollective.com/solid/backer/0/website" target="_blank"><img src="https://opencollective.com/solid/backer/0/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/1/website" target="_blank"><img src="https://opencollective.com/solid/backer/1/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/2/website" target="_blank"><img src="https://opencollective.com/solid/backer/2/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/3/website" target="_blank"><img src="https://opencollective.com/solid/backer/3/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/4/website" target="_blank"><img src="https://opencollective.com/solid/backer/4/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/5/website" target="_blank"><img src="https://opencollective.com/solid/backer/5/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/6/website" target="_blank"><img src="https://opencollective.com/solid/backer/6/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/7/website" target="_blank"><img src="https://opencollective.com/solid/backer/7/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/8/website" target="_blank"><img src="https://opencollective.com/solid/backer/8/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/9/website" target="_blank"><img src="https://opencollective.com/solid/backer/9/avatar.svg"></a>
<a href="https://opencollective.com/solid/backer/10/website" target="_blank"><img src="https://opencollective.com/solid/backer/10/avatar.svg"></a>

### Sponsors

Become a sponsor and get your logo on our README on GitHub with a link to your site. [[Become a sponsor](https://opencollective.com/solid#sponsor)]

<a href="https://opencollective.com/solid/sponsor/0/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/0/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/1/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/1/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/2/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/2/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/3/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/3/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/4/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/4/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/5/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/5/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/6/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/6/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/7/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/7/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/8/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/8/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/9/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/9/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/10/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/10/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/11/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/11/avatar.svg"></a>
<a href="https://opencollective.com/solid/sponsor/12/website" target="_blank"><img src="https://opencollective.com/solid/sponsor/12/avatar.svg"></a>

# Changelog

## 1.8.0 - 2023-10-09

I admit this is not the most exciting release from a feature standpoint. We are in that holding pattern between the end of 1.x and the start of 2.0. We recently made our new reactive experiments public and continue to build those out in public with [@solidjs/signals](https://github.com/solidjs/signals).

This version is more about addressing some of the fundamentals that will help us in other projects like SolidStart while we do the transition. A big part of this is applying what we have learned when doing performance benchmarks for the work that has been funded by [Google Chrome Aurora](https://www.solidjs.com/blog/chrome-supports-solidjs).

Async and Resources need work and are too all in. It is great to have a solution but now that we have a better understanding we need to start breaking things apart into their fundamental pieces.

### De-duping Streaming Serialization

This is the marquee feature of this release and is largely the work of @lxsmnsyc. Solid has been able to serialize promises and do streaming for a couple of years now, but it was very special-cased. Now it is a generic mechanism.

This matters because it means that we have decoupled the promise serialization from Resources, and in so decoupled the whole when the stream is done from them. This opens up things like nested promises.

More so we have a mechanism now that deeply de-dupes data serialized across flushes. This is important for features like Islands where you might pass the same props to multiple Islands across different Suspense boundaries and don't want to send the data more than once. And even examples where that data can be accessed at varying depths (recursive comments in say a Hackernews site).

### Hydration Improvements

Fragments for Hydration have been a bit of a pain and we keep seeming to have different issues reported around element duplication. Most commonly this has been around where there are `lazy` component siblings or where the fragment is top-level. After looking into and fixing an [issue for Astro](https://github.com/withastro/astro/pull/8365) I decided to look at some of the oldest bugs in Solid and found it was a similar bug.

In many cases, the DOM can change throughout Hydration while doing things like streaming but we need to pause and resume hydration because code isn't available yet. While we don't create elements during hydration, getting an accurate snapshot of the DOM for the current state for future list reconciliation is a process we've had a few tries at but in 1.8 we update this in a way that makes sure it doesn't get out of date.

Also in 1.8, we have added some performance improvements to hydration in the form of not redundantly setting attributes or props as the page hydrates similar to how we don't update text. This is all migration towards a future where we don't need to do as much hydration, but it is important to note that values will be kept as they were on the server rather than how they may compute at runtime during hydration.

### Smaller Templates

In 1.7 we removed unnecessary closing tags from template strings. It was a bit painful because we were a bit overzealous at first. While I believe in the end we got to a good place, ultimately all but the simplest reductions have been hidden behind a compiler flag(`omitNestedClosingTags`). Thanks to work from @intrnl we are implementing another template size reduction technique of removing unnecessary quotes. Quotes are actually not required by HTML in some cases and it can add up.

### Other

#### Fix NGINX Server Side Includes

Comments led with `#` are treated as special directives for a few different servers so we've needed to change our open hydration markers to `$`. As usual, your version of Solid and the Babel Plugin should be the same to ensure this matches up.

#### Better Guards on Global Scripts

Solid uses an inline HydrationScript as a way to do processing before the framework and code have loaded. To handle things like event capture and streaming. However, we didn't do a good job of guarding the right thing when multiple were added to the same page, a situation that can happen in Micro-frontends or 3rd party Islands solutions. Now the script guards against duplicate inclusion.

## 1.7.0 - 2023-03-30

Solid has experienced incredible growth in usage the last 6 months. Companies are using it to power production applications and SolidStart Beta has been a big part of that. As a natural part of this growth and increased use at scale we are continuing to learn what works well and what the rough edges in Solid are today.

This v1.7 release marks the beginning of the migration roadmap to v2.0. We are beginning to re-evaluate core APIs and will begin introducing new ones while reasonably deprecating older ones in a manner that eases breaking changes. Our intention is to ease the broader ecosystem into preparing for improvements that a major 2.0 will unlock for the whole community.

### Improved TypeScript

#### Null-Asserted Control Flow

One of the pains of using Solid with TypeScript has been that JSX control flows can't really type narrow. This is true, but starting with the migration to explicit `keyed` in v1.5 we now complete this story by introducing callback forms for `<Show>` and `<Match>` that work when non-keyed.

The main difference is the callback form instead of passing in the value as it does when `keyed`, passes in a function that is type narrowed.

```js
// keyed w/ callback - reruns full callback on change
<Show when={user()} keyed>
  {nonNullUser => <div>{nonNullUser.name}</div>}
</Show>

// non-keyed w/o callback... - only updates the one expression, needs ! assertion
<Show when={user()}>
  <div>{user()!.name}</div>
</Show>

// NEW!
// non-keyed w/ callback - only updates the one expression
<Show when={user()}>
  {nonNullUser => <div>{nonNullUser().name}</div>}
</Show>
```

Keep in mind because we are non-null asserting the input signal so it won't expect null in closures that execute when the condition is no longer satisfied. For this reason the accessor from the callback is special and will throw when attempted to be accessed when the condition is no longer true. This may be unexpected but it is our best attempt to keep TypeScript strict and not present inconsistency in reactivity. Luckily this only applies to things like timers which you should be cleaning up anyway and not things like event handlers. We recommend using the original conditions source in those closures if you must.

#### Better Event Types for Input Elements

This has irked people for a while but we come by it honestly, `target` is gives you a type of `Element` rather than the specific element that is the target. That means no access to `.value` or `.checked`. The reason is there is no way to know at compile time what the target of an event will be. The `currentTarget` will be the element you attach the event to but the target can be anything.

There is a way to work around this though, in that if we know the `currentTarget` is of type that generates the event and that the `currentTarget` is the the type of this element we can assume it is the `target` as well. Not perfect logic but it is what React does and we do too.

Now `onInput`, `onChange`, `onBlur`, `onFocus`, `onFocusIn`, and `onFocusOut` all support more detailed `target` when applied to `HTMLInputElement`, `HTMLTextAreaElement`, and `HTMLSelectElement`.

#### Stricter JSX Elements

Strict JSX elements have been tricky because we have to acknowledge at a certain point that TypeScript is to serve our purposes rather than to represent all possible values that could work. For us the ambiguity lies in functions.

Solid's JSX needs to accept functions to handle dynamic insertion. However, in authoring it leads to awkward situations.

The first you hit the first time use Solid. You create that counter and don't call `count` as a function and it works.

```js
function Counter() {
  const [count, setCount] = createSignal(1);

  return <button onClick={() => setCount(c => c + 1)}>{count}</button>;
}
```

This example works in some places and not others which might lead to the wrong conclusions.

The second place you might hit this is when you get a little further on your journey and decide you need a component to re-render and decide that you can just wrap the whole thing in a function:

```js
function MyComp(props) {
  return () => {
    // look working early returns
    if (props.count > 5) {
      return <div>Maximum Tries</div>;
    }

    return <div>Attempt {props.count}</div>;
  };
}
```

Again this seems fine, except the fact that every time `count` changes you are recreating all the DOM Elements even when it resolves to the same conditional.

Eventually you might even not think twice about passing functions into children of arbitrary components:

```js
<MyComp>
  <MyComp2>
    <MyComp3>{() => <div>{resource()}</div>}</MyComp3>
  </MyComp2>
</MyComp>
```

But what does this do? When is the function called?

As it turns out removing functions from `JSX.Element` type makes all of these scenarios error. Components only expect the values dictated by their types.

```js
function MyLayout(props: { children: JSX.Element }): JSX.Element;

function MyFor<T, U extends JSX.Element>(props: { each: T[],  children: (item: T) => U }): JSX.Element;

// valid
<MyLayout>Hello</MyLayout>
<MyLayout><p>Hello</p></MyLayout>
<MyLayout>{name()}</MyLayout>
<MyLayout>{name() && <p>Hello</p>}</MyLayout>
<MyLayout>{(() => {
  return <p{name()}</p>
})()}</MyLayout>
<MyLayout>{untrack(() => {
  return <p>{name()}</p>
})}</MyLayout>
<MyFor each={users()}>{(user) => <div>{user.name}</div>}</MyFor>

// invalid
<MyLayout>{name}</MyLayout>
<MyLayout>{() => <p>Hello</p>}</MyLayout>
<MyLayout>{() => "Hello"}</MyLayout>
<MyLayout>{() => name() && <p>Hello</p>}</MyLayout>
<MyFor each={users}>{(user) => <div>{user.name}</div>}</MyFor>
<MyFor each={users()}><div>Not a Function</div></MyFor>
```

The tradeoff here is that authoring components you can no longer just return a Signal or Memo without casting. If using JSX you can always return a Fragment.

If not you will need to cast to `unknown as JSX.Element`.

### Better Errors and Cleanup

#### `catchError` replaces `onError`

Error Handling is complicated enough without having to try to guess how they propagate. `onError` admittedly is a lower level primitive but fundamentally had this flaw. It worked by registering an error handler on the parent scope, but left it ambiguous how to handle siblings. Is it a queue? Are they independent?

As a result we are introducing `catchError` in this release which introduces its own scope to catch any errors below it. The first argument in the primitive is similar to the try and the second argument is the catch.

```js
catchError(
  () => {
    // do stuff
    throw new Error("I've Errored");
  },
  err => console.log(err)
);
```

`onError` will still be present until it can be removed in a future major version.

#### Standardized Errors

Error Handling has had many weird edge cases introduced by applications throwing unusual values. In v1.7 we wrap all thrown values that aren't of type `Error` in a `new Error` and attach the original thrown value as `.cause`.

### More Performant Dev Tools

Now that [Solid Dev Tools](https://github.com/thetarnav/solid-devtools) have been stabilizing, we have a much better idea what support we need for them. In so we were able to remove the very costly serialization we were doing for generating unique identifiers. Conventions around naming and exports were streamlined and standardized as well.

### Others

- Smaller compiled output, remove auxilary closing tags
- Support for `prop:` and `attr:` in Spreads
- Don't apply special props (like `readonly`) to custom elements
- Introduced improved serializer, [seroval](https://github.com/lxsmnsyc/seroval)
- Fixed quirks in Solid's treeshaking in Rollup
- Minify inline class and style attributes
- Update `solid-ssr` to type `"module"`

## 1.6.0 - 2022-10-20

Solid v1.6 doesn't bring a ton of new features but brings some big improvements in existing ones.

### Highlights

#### Official Partial Hydration Support

Solid has worked for quite some time in partial hydrated ("Islands") frameworks like Astro, Iles, Solitude, etc.. but now we have added core features to support this effort better. These features are mostly designed for metaframework authors rather than the end user they are exposed through a couple APIs.

`<Hydration />` joins `<NoHydration />` as being a way to resume hydration and hydration ids during server rendering. Now we can stop and start hydratable sections. This is important because it opens up a new optimization.

`createResource` calls under non-hydrating sections do not serialize. That means that resources that are server only stay on the server. The intention is that hydrating Islands can then serialize their `props` coming in. Essentially only shipping the JSON for data actually used on the client.

The power here is static markup can interview dynamic components.

```js
<h1>Server Rendered Header</h1>
<Island>
  <h2>Server Rendered Sub Header</h2>
  <p>{serverOnlyResource().text}</p>
  <DifferentIsland>
    <p>More server-renderd content</p>
  </DifferentIsland>
</Island>
```

Keep in mind Server rendered content like this can only be rendered on the server so to maintain a client navigation with this paradigm requires a special router that handles HTML partials.

Similarly we want the trees to talk to each other so `hydrate` calls now have been expanded to accept a parent `Owner` this will allow Islands to communicate through Contex without shipping the whole tree to browser.

```js
<h1>Server Rendered Header</h1>
<ClientProvider>
  <h2>Server Rendered Sub Header</h2>
  <ClientIslandThatReadsContext />
</ClientProvider>
```

These improvements make it easier to create Partial Hydration solutions on top of Solid, and serve to improve the capabilities of the ones we already have.

#### Native Spread Improvements

Native spreads are something we started at very naively. Simply just iterating an object that has some reactive properties and updating the DOM element. However, this didn't take into consideration two problems.

First properties on objects can change, they can be added or removed, and more so the object itself can be swapped. Since Solid doesn't re-render it needs to keep a fixed reference to the merged properties. Secondly, these are merged. Properties override others. What this means is we need to consider the element holistically to know that the right things are applied.

For Components this was a never a problem since they are just function calls. Unfortunately for native elements this means all those compiler optimizations we do for specific bindings now need to get pulled into this. Which is why we avoided it in the past. But the behavior was too unpredictable.

In 1.6 we have smartened spread to merge properly using similar approach to how process Components. We've also found new ways to optimize the experience. (See below).

### Other Improvements

#### Deproxification

Working on new Spread behavior we realized that while we can't tell from compilation which spreads can change. We can tell at runtime which are proxies. And in so if we only need to merge things which don't swap, and aren't proxies we can avoid making a Proxy.

What is great about this is it has a cascading effect. If component props aren't a proxy, then `splitProps` and `mergeProps` don't need to create them, and so on. While this requires a little extra code it is a real win.

We get a lot request for low end IoT devices because of Solid's incredible performance. In tests Solid outperforms many of the Virtual DOM solutions in this space. However most of them don't support proxies.

So now if you don't use a `Store` or swap out the props object:

```js
// this is fine
<div {...props} />

// these could swap out the object so they make proxies
<div {...props.something} />
// or
<div {...someSignal()} />
```

We don't need to introduce any proxy the user didn't create. This makes Solid a viable option for these low-end devices.

## 1.5.0 - 2022-08-26

### Key Highlights

#### New Batching Behavior

Solid 1.4 patched a long time hole in Solid's behavior. Until that point Stores did not obey batching. However, it shone a light on something that should maybe have been obvious before. Batching behavior which stays in the past is basically broken for mutable data, No Solid only has `createMutable` and `produce` but with these sort of primitives the sole purpose is that you perform a sequence of actions, and batching not making this properly was basically broken. Adding an element to an array then removing another item shouldn't just skip the first operation.

```js
const store = createMutable(["a", "b", "c"]);

const move = store.splice(1, 1);
store.splice(0, 0, ...move);

// solid 1.4
// ["b", "a", "b", "c"];

// solid 1.5
// ["b", "a", "c"];
```

After a bunch of careful thought and auditting we decided that Solid's `batch` function should behave the same as how reactivity propagates in the system once a signal is set. As in we just add observers to a queue to run, but if we read from a derived value that is stale it will evaluate eagerly. In so signals will update immediately in a batch now and any derived value will be on read. The only purpose of it is to group writes that begin outside of the reactive system, like in event handlers.

#### More Powerful Resources

Resources continue to get improvements. A common pattern in Islands frameworks like Astro is to fetch the data from the out side and pass it in. In this case you wouldn't want Solid to do the fetching on initial render or the serialization, but you still may want to pass it to a resource so it updates on any change. For that to work reactivity needs to run in the browser. The whole thing has been awkward to wire up but no longer.

`ssrLoadFrom` field lets you specify where the value comes from during ssr. The default is `server` which fetches on the server and serializes it for client hydration. But `initial` will use the `initialValue` instead and not do any fetching or addtional serialization.

```js
const [user] = createResource(fetchUser, {
  initialValue: globalThis.DATA.user,
  ssrLoadFrom: "initial"
});
```

We've improved TypeScript by adding a new `state` field which covers a more detailed view of the Resource state beyond `loading` and `error`. You can now check whether a Resource is `"unresolved"`, `"pending"`, `"ready"`, `"refreshing"`, or `"error"`.

| state      | value resolved | loading | has error |
| ---------- | -------------- | ------- | --------- |
| unresolved | No             | No      | No        |
| pending    | No             | Yes     | No        |
| ready      | Yes            | No      | No        |
| refreshing | Yes            | Yes     | No        |
| errored    | No             | No      | Yes       |

A widely requested feature has been allowing them to be stores. While higher level APIs are still being determined we now have a way to plugin the internal storage by passing something with the signature of a signal to the new _Experimental_ `storage` option.

```js
function createDeepSignal<T>(value: T): Signal<T> {
  const [store, setStore] = createStore({
    value
  });
  return [
    () => store.value,
    (v: T) => {
      const unwrapped = unwrap(store.value);
      typeof v === "function" && (v = v(unwrapped));
      setStore("value", reconcile(v));
      return store.value;
    }
  ] as Signal<T>;
}

const [resource] = createResource(fetcher, {
  storage: createDeepSignal
});
```

#### Consolidated SSR

This release marks the end of years long effort to merge async and streaming mechanism. Since pre 1.0 these were seperate. Solid's original SSR efforts used reactivity on the server with different compilation. It was easiest to migrate synchronous and streaming rendering and for a time async had a different compilation. We got them on the same compilation 2 years ago but runtimes were different. Piece by piece things have progressed until finally async is now just streaming if flushed at the end.

This means some things have improved across the board. Async triggered Error Boundaries previously were only ever client rendered (throwing an error across the network), but now if they happen any time before sending to the browser they are server rendered. `onCleanup` now runs on the server if a branch changes. Keep in mind this is for rendering effects (like setting a status code) and not true side effects as not all rendering cleans up.

Finally we've had a chance to do a bunch of SSR rendering performance improvements. Including replacing our data serializer with an early copy of Dylan Piercey from [Marko](https://markojs.com)'s upcoming serializer for Marko 6. Which boasts performance improvements of up to 6x `devalue` which we used previously.

#### Keyed Control Flow

Solid's `<Show>` and `<Match>` control flow originally re-rendered based on value change rather than truthy-ness changing. This allowed the children to be "keyed" to the value but lead to over rendering in common cases. Pre 1.0 it was decided to make these only re-render when statement changed from `true` to `false` or vice versa, except for the callback form that was still keyed.

This worked pretty well except it was not obvious that a callback was keyed. So in 1.5 we are making this behavior explicit. If you want keyed you should specify it via attribute:

```js
// re-render whenever user changes

// normal
<Show when={user()} keyed>
  <div>{user().name}</div>
</Show>

// callback
<Show when={user()} keyed>
  {user => <div>{user.name}</div>}
</Show>
```

However, to not be breaking if a callback is present we will assume it's keyed. We still recommend you start adding these attributes (and TS will fail without them).

In the future we will introduce a non-keyed callback form as well so users can benefit from type narrowing in that case as well.

### Other Improvements

### `children.toArray`

Children helper now has the ability to be coerced to an array:

```js
const resolved = children(() => props.children);
resolved.toArray(); // definitely an array
```

#### Better SSR Spreads

Finally fixed spread merging with non-spread properties during SSR, including the ability to merge children.

#### Better Error Handling

We weren't handling falsey errors previously. Now when Solid receives an error that isn't an `Error` object or a string it will coerce it into an `Unknown Error`.

## 1.4.0 - 2022-05-12

### New Features

#### Resource Deferred Streaming

Streaming brings a lot of performance benefits but it also comes with the tradeoff we need to respond with the headers before we can send any content. This means we must set the Response headers early if we want to benefit from streaming. While it's always possible to fetch first and delay rendering that slows down everything. Even our async server rendering doesn't block rendering but instead just waits to respond to the end.

But what if you want to stream but also want to wait on some key data loading so you still have an opportunity to handle the response on the server before sending it to the browser?

We now have the ability to tell Solid's stream renderer to wait for a resource before flushing the stream. That you can opt in by setting `deferStream` option.

```js
// fetches a user and streams content as soon as possible
const [user] = createResource(() => params.id, fetchUser);

// fetches a user but only streams content after this resource has loaded
const [user] = createResource(() => params.id, fetchUser, { deferStream: true });
```

#### Top Level Arrays in Stores

Since Stores were first introduced it has always bugged me that the most common case, creating a list required nesting it under a property to track properly. Thanks to some exploration into proxy traps and iteration we now support top level arrays. In addition to its other modes, the Store setter will accept an array which allows for common operations.

```js
const [todos, setTodos] = createStore([
  { id: 1, title: "Thing I have to do", done: false },
  { id: 2, title: "Learn a New Framework", done: false }
]);

// set at an index
setTodos(1, done, true);

// use an array
setTodos([...todos, { id: 3, title: "New Todo", done: false }])

// iterate over it with <For>
<For each={todos}>{todo => <Todo todo={todo} />}</For>;
```

Through this change we also stopped over execution when listening to specific properties. To support iteration Solid previously would notify the owning object of any array when an was index added/removed or object new property created or deleted on any object.

The one caveat is downstream optimized control flow that untrack index reads on arrays will now need to track the iterated object explicity. Solid exports a `$TRACK` symbol used to subscribe to the object and all its properties.

#### Stale Resource Reads

Suspense and Transitions are amazingly powerful feature but occasionally you want to opt out of the consistency and show things out of date because it will show up faster and some of things you are waiting for are not as high priority. In so you want the Transition to end sooner, but not necessarily stop showing the stale data for part of the screen. It is still preferable to receding back to loading spinner state.

Solid's Resources now support being able to read the value without triggering Suspense. As long as it has loaded previously `latest` property won't cause fallback appear or Transitions to hold. This will always return the `latest` value regardless whether it is stale (ie.. a new value is being fetched) and will reactively update. This is super powerful in Transitions as you can use the Resources own `loading` state to know if it is stale. Since the Transition will hold while the critical data is loading, the loading state will not be applied to the in view screen until that Transition has ended. If the resource is still loading now you can show that it is stale.

```js
const [resource] = createResource(source, fetcher);

// read it as usual
resource();

// read the latest (don't suspend if loaded at least once)
resource.latest;
```

Example: https://codesandbox.io/s/solid-stale-resource-y3fy4l

#### Combining multiple Custom Renderers

The Babel plugin now allows configuring multiple custom renderers at the same time. The primary case it is so a developer can still lever Solid's optimized DOM compilation while using their custom renderer. To make this work specify the tags each renderer is reponsible for. It will try to resolve them in order.

```js
import { HTMLElements, SVGElements } from "solid-js/web";
let solidConfig = {
  moduleName: "solid-js/web",
  // @ts-ignore
  generate: "dynamic",
  renderers: [
    {
      name: "dom",
      moduleName: "solid-js/web",
      elements: [...HTMLElements, ...SVGElements]
    },
    {
      name: "universal",
      moduleName: "solid-three",
      elements: []
    }
  ]
};
```

### Improvements/Fixes

#### Synchronous Top Level `createEffect`

These were originally deferred to a microtask to resemble how effects are queued under a listener. However it is more correct to run immediate like everything else top level.

#### Better Types around Components

This one took the effort of many resident TypeScript experts, but we've now landed on some better types for components. The biggest change is `Component` no longer has an opinion on whether it should have `children` or not. We've added supplementary types `ParentComponent` and `FlowComponent` to denote Components that may have `children` or always have `children`. And we've added `VoidComponent` for those which may never have children.

#### Sources in `createResource` are now Memos

A small change but it was unusual to have refetching trigger a reactive expression outside of a reactive context. Now on refetch it grabs the last source value rather than re-running it.

#### `createMutable` batches array methods like push, pop, etc..

Now these built-ins are batched and more performant. We've also add `modifyMutable` that applies modifiers batched to stores created with `createMutable`.

```js
modifyMutable(state.data.user, reconcile({ firstName: "Jake", middleName: "R" }));
```

#### Stores and mutables now respect batch

Writing to a store or mutable within `batch` (including effects) no longer immediately updates the value, so reading within the same batch gives the old value. This guarantees consistency with memos and other computations, just like signals.

#### Better Support for React JSX transform

We have added support to `solid-js/h` to support the new React JSX transform. You can use it directly in TypeScript by using:

```json
{
  "jsx": "react-jsx",
  "jsxImportSource": "solid-js/h"
}
```

Keep in mind this has all the consequences of not using the custom transform. It means larger library code, slower performance, and worse ergonomics. Remember to wrap your reactive expressions in functions.

#### HyperScript now returns functions

This one is a potentially breaking change, but the current behavior was broken. It was possible(and common) for children to be created before the parents the way JSX worked. This was an oversight on my original design that needs to be fixed, as it breaks context, and disposal logic. So now when you get your results back from `h` you need to call it. Solid's `render` function will handle this automatically.

```js
const getDiv = h("div", "Hello");

document.body.appendChild(getDiv()); // call as a function to have it create the element.
```

### Removals and Deprecations

#### `className`, `htmlFor` deprecated

While they still work for now, Solid will remove support for these React-isms in a future version. They leave us with multiple ways to set the same attribute. This is problematic for trying to merge them. Solid updates independently so it is too easy for these things to trample on each other. Also when optimizing for compilation since with things like Spreads you can't know if the property is present, Solid has to err on the side of caution. This means more code and less performance.

#### Experimental `refetchResources` removed

This primitive ended up being too general to be useful. There are enough cases we can't rely on the refetch everything by default mentality. For that reason we are dropping support of this experimental feature.

## 1.3.0 - 2022-01-05

### New Features

#### HTML Streaming

This release adds support for HTML streaming. Now we not only stream data after the initial shell but the HTML as it finishes. The big benefit is that now for cached results, or times when the network are slow we no longer have to show the placeholder while waiting for JavaScript bundle to load. As soon as the HTML is available it will be streamed and inserted.

With it comes new streaming API `renderToStream`. This is a universal API designed to handle both Node and Web writable streams. It returns an object that mirrors a Readable stream on both platforms that has both `pipe` (node) and `pipeTo` (web). The benefit of this `pipe` API is the user can choose when to insert the content in the output stream whether soon as possible, or `onCompleteShell`, or `onCompleteAll`. This decouples Solid's rendering a from the stream a bit but leaves things open to performance improvements in the future.

```js
// node
const stream = renderToStream(() => <App />).pipe(res);

// web
const stream = renderToStream(() => <App />).pipeTo(writable);
```

#### Error Boundaries on the Server

We've added support for Error Boundaries on the Server for all rendering methods(`renderToString`, `renderToStringAsync`, `renderToStream`). Errors can be caught both from synchronous rendering and from errors that happen in Resource resolution. However, Our approach doesn't guarentee all errors are handled on the server as with streaming it is possible that the Error Boundary has already made it to the browser while a nested Suspense component hasn't settled. If an Error is hit it will propagate up to the top most Suspense Boundary that hasn't been flushed yet. If it is not handled by an Error Boundary before that it will abort rendering, and send the Error to the browser to propagate up to the nearest Error Boundary.

This works now but there is more to explore here in improving Error handling in general with SSR. So look forward to feedback on the feature.

#### Isolated Server Render/Hydration Contexts

Sometimes you want to server render and hydrate multiple Solid apps on the same page. Maybe you are using the Islands architecture with something like [Astro](https://astro.build). We now have the ability to pass a unique `renderId` on all our server rendering methods and to the `hydrate` function. This will isolate all hydration and resource resolution. This means we can use things like server side Suspense in these solutions.

Also now you only need to include the Hydration Script once on the page. Each Island will be responsible for initializing it's own resources.

```js
// on the server
const html = renderToString(() => <Island1 />, { renderId: "island1" });

// for the browser
hydrate(() => <Island1 />, mountEl, { renderId: "island1" });
```

#### `createReaction`

This new primitive is mostly for more advanced use cases and is very helpful for interopt with purely pull based systems (like integrating with React's render cycle). It registers an untracked side effect and returns a tracking function. The tracking function is used to track code block, and the side effect is not fired until the first time any of the dependencies in the tracking code is updated. `track` must be called to track again.

```js
const [s, set] = createSignal("start");

const track = createReaction(() => console.log("something"));

// next time s changes run the reaction
track(() => s());

set("end"); // "something"

set("final"); // no-op as reaction only runs on first update, need to call track again.
```

This primitive is niche for certain use cases but where it is useful it is indispensible (like the next feature which uses a similar API).

#### External Sources (experimental)

Ever wanted to use a third party reactive library directly in Solid, like MobX, Vue Reactivity, or Kairo. We are experimenting with adding native support so reactive atoms from these libraries can be used directly in Solid's primitives and JSX without a wrapper. This feature is still experimental since supporting Transitions and Concurrent Rendering will take some more effort. But we have added `enableExternalSource` enable this feature. Thanks @3Shain for designing this solution.

```js
import { Reaction, makeAutoObservable } from "mobx";
import { enableExternalSource } from "solid-js";
import { render } from "solid-js/web";

let id = 0;
enableExternalSource((fn, trigger) => {
  const reaction = new Reaction(`externalSource@${++id}`, trigger);
  return {
    track: x => {
      let next;
      reaction.track(() => (next = fn(x)));
      return next;
    },
    dispose: () => {
      reaction.dispose();
    }
  };
});

class Timer {
  secondsPassed = 0;

  constructor() {
    makeAutoObservable(this);
  }

  increase() {
    this.secondsPassed += 1;
  }

  reset() {
    this.secondsPassed = 0;
  }
}

// component driven directly off MobX
function App() {
  const timer = new Timer();
  setInterval(() => {
    timer.increase();
  }, 1000);

  return <button onClick={() => timer.reset()}>Seconds passed: {timer.secondsPassed}</button>;
}

render(() => <App />, document.getElementById("app"));
```

#### `refetchResources` (experimental)

In efforts to allow for scaling from simple resources up to cached solutions we are adding some experimental features to `createResource` to work with library writers to develop the best patterns. Caching is always a tricky problem and with SSR and streaming being part of the equation the core framework needs at minimum to provide some hooks into orchestrating them.

Sometimes it's valuable to trigger `refetch` across many resources. Now you can.

```js
import { createResource, refetchResources } from "solid-js";

const userCache = {};

function MyComponent(props) {
  const [data] = createResource(
    () => props.id,
    (userId, { refetching }) => {
      const cached = userCache[userId];

      // return cached value if available and not refetching
      if (cached && !refetching) return cached;
      return fetchUser(userId);
    }
  );
}

// somewhere else
refetchResources();
```

You can also pass a parameter to `refetchResources` to provide additional information to the `refetching` info of the fetcher. This could be used for conditional cache invalidation. Like only refetch resources related to `users`. This mechanism requires a bit of wiring but the idea is you'd wrap `createResource` in maybe a `createQuery` and implement your own conventions around resource cache management. Still working out how this should work best, but the goal is to provide the mechanisms to support resource caches without being responsible for their implementation.

To opt-out being part of the global refetch createResource now takes a `globalRefetch` option that can be set to false. In addition to a new option to disable `refetchResources` there is no an `onHydrated` callback that takes the same arguments as the fetcher. When a resource is restored from the server the fetcher is not called. However, this callback will be. This is useful for populating caches.

### Improvements

#### Better TypeScript Support

Thanks to the tireless efforts of several contributors we now have significantly better types in Solid. This was a huge effort and involved pulling in maintainers of TypeScript to help us work through it. Thank you @trusktr for spearheading the effort.

#### Better SourceMaps

Work has been done to improve sourcemaps by updating `babel-plugin-dom-expressions` to better preserve identifiers from the JSX. Thanks to @LXSMNSYC for exploring and implementing this.

### Breaking Changes/Deprecations

#### `startTransition` no longer takes callback as a second argument

Instead it returns a promise you can await. This works better for chaining sequences of actions.

```js
const [start, isPending] = useTransition();

start(() => doSomething()).then(() => allDone());
```

#### Resource fetcher info object replaces `getPrev`

To streamline API for refetch we are slightly updating the `createResource`:

```js
const [data] = createResource(sourceSignal, (source, { value, refetching }) => {});
```

For those using existing 2nd argument:

```js
const [data] = createResource(sourceSignal, (source, getPrev) => {
  const value = getPrev();
});

// becomes
const [data] = createResource(sourceSignal, (source, { value }) => {});
```

#### Deprecating Legacy Streaming APIs

`pipeToNodeWritable` and `pipeToWritable` are deprecated. They will still work for now with basic usage but some of the more advanced options didn't map over to the new APIs directly and have been removed. Move to using `renderToStream`.

### Bug Fixes

- Fixed browser extensions modifying the head breaking hydration.
- Fixed reinserting `<html>` on hydration from document.
- Fixed over-executing on multi-select with `createSelector`.
- Fixed event delegation conflicting with document event listeners.
- Fixed self owning source infinite recursion.
- Fixed faulty treesplitting for hydration in client only render.
- Fixed return type of `preload` on lazy components to always be a promise.
- Fixed compile error with leading white space after opening tags when generating ssr.

## 1.2.0 - 2021-10-25

### New Features

#### Custom Renderers

This release adds support custom renderers through a new "universal" transform. Solid now provides a sub module `solid-js/universal` that exports a `createRenderer` method that allows you to create your own runtimes. This will enable things like native mobile and desktop, canvas and webgl, or even rendering to the terminal. This is still new so very much looking for feedback.

#### Spreads Added to Solid's `html`

It's been a long time coming but Solid's Tagged Template Literals now support element and component spreads using htm inspired syntax.

```js
html`<div ...${props} />`;
```

### Fixes

#### Dynamic Spreads now work on Components

Previously spreads on components would only track property changes on bound objects and not when the whole object changed. This now works:

```js
<MyComponent {...getStuff()} />
```

#### ClassList properly merges multiple classnames in the key

It is common in libraries like Tailwind to apply multiple classes at the same time. There was an issue where true and false resolutions were cancelling each other out. This would only set `text-sm`.

```js
<div
  classList={{
    "px-2.5 py-1.5 text-xs": false,
    "px-3 py-2 text-sm": false,
    "px-4 py-2 text-sm": true,
    "px-4 py-2 text-base": false,
    "px-6 py-3 text-base": false
  }}
/>
```

#### Consistent handling of HTMLEntities

Things like `&nbsp;` used to render differently depending if in elements or components(or fragments). This has been made consistent across all three.

#### Various improvements to Types and Transitions

A lot of bugs from the last minor release were around Transitions that have been addressed. And as always Types have been gradually improving.

## 1.1.0 - 2021-08-09

Expanding Solid's concurrency to include scheduling. Bug fixes around Types and around reactive execution order guarantees.

### New Features

#### `createUniqueId`

A universal id generator that works across server/browser.

```js
const id = createUniqueId();
```

> **Note** on the server this only works under hydratable components

#### `from`

A simple helper to make it easier to interopt with external producers like RxJS observables or with Svelte Stores. This basically turns any subscribable (object with a `subscribe` method) into a Signal and manages subscription and disposal.

```js
const signal = from(obsv$);
```

It can also take a custom producer function where the function is passed a setter function returns a unsubscribe function:

```js
const clock = from(set => {
  const t = setInterval(() => set(1), 1000);
  return () => clearInterval(t);
});
```

> Note: Signals created by `from` have equality checks turned off to interface better with external streams and sources.

#### `enableScheduling` (experimental)

By default Solid's concurrent rendering/Transitions doesn't schedule work differently and just runs synchronously. Its purpose is to smooth out IO situations like Navigation. However now you can opt into interruptible scheduling similar to React's behavior by calling this once at your programs entry. I've yet to see a realworld scenario where this makes a big difference but now we can do cool demos too and start testing it.

#### `startTransition`

Works like its counterpart in `useTransition`, this useful when you don't need pending state.

```js
import { createSignal, startTransition } from "solid-js";

function App() {
  const [signal, setSignal] = createSignal("Howdy");
  function clickHandler(e) {
    startTransition(() => setSignal("Holla"));
  }

  /* ...stuff */
}
```

## 1.0.0 - 2021-06-27

### Breaking Changes

### setSignal now supports function form

While that in itself is a great new feature as you can do:

```js
const [count, setCount] = createSignal(0);

setCount(c => c + 1);
```

This promotes immutable patterns, let's you access the previous value without it being tracked, and makes Signals consistent with State.

It means that when functions are stored in signals you need to use this form to remove ambiguity

```js
const [count, setCount] = createSignal(ComponentA);

// Do this:
setCount(() => ComponentB);

// Don't do this as it will call the function immediately:
setCount(ComponentB);
```

#### `createState` moved and renamed

`createState` has been renamed to `createStore` and moved to `solid-js/store`. Also moved to `solid-js/store`: `createMutable`, `produce`, `reconcile`

#### SSR Entry points

`renderToString` and `renderToStringAsync` now only return their stringified markup. To insert scripts you need to call `generateHydrationScript` or use the new `<HydrationScript>` component.

`renderToNodeStream` and `renderToWebStream` have been replaced with `pipeToNodeWritable` and `pipeToWritable`, respectively.

#### Options Objects

Most non-essential arguments on reactive primitives are now living on an options object. This was done to homogenize the API and make it easier to make future additions while remaining backwards compatible.

#### on

No longer uses rest parameters for multiple dependencies. Instead pass an array. This facilitates new option to defer execution until dependencies change.

#### Actions renamed to Directives

To remove future confusion with other uses of actions the `JSX.Actions` interace is now the `JSX.Directives` interface.

## 0.26.0 - 2021-04-09

This release is about finalizing some API changes on the road to 1.0. This one has one breaking change and not much else.

#### Signals no longer always notify by default

Solid's original behavior has been to always notify on signal change even if the value hasn't changed. The idea was to simulate stream behavior. However, this has some downsides:

1. Inconsistent with State.. I made the decision to make state equality check by default, it is weird signals and memo's do not.
2. More likely to hit infinite loops. Equality check naturally stops infinite loops in some cases. While infinite loops aren't good and code that produces them suspect, it is nice to keep things clean.
3. It is consistent with other modern reactive libraries like MobX and Vue.

The API has not changed. You can opt out of the default behavior by passing in your own comparator or false to the 2nd parameter of `createSignal` and the 3rd parameter of `createMemo`.

My hope this is the last release before I start making 1.0 RC's. This one has big enough impact I want to get this out first. I imagine the remaining changes will be just syntax.

## 0.25.0 - 2021-03-28

This release is about refining the APIs as we approach the our release candidate for 1.0.

### Breaking Changes

#### Resource API

Minor difference to allow the first argument to be optional and support more features in the future. New full signature is:

```ts
export function createResource<T, U>(
  fn: U | false | (() => U | false),
  fetcher: (k: U, getPrev: () => T | undefined) => T | Promise<T>,
  options?: { initialValue?: T }
): ResourceReturn<T>;
```

3rd argument is now an options object instead of just the initial value. This breaking. But this also allows the first argument to be optional for the non-tracking case. Need a promise that only loads once? Don't have need to re-use the fetcher. Do this:

```js
const [data] = createResource(async () => (await fetch(`https://someapi.com/info`)).json());
```

#### on/onCapture

These are an escape hatch for unusual events. Previously these were custom attributes but now they are namespaced like:

```jsx
<div on:someUnusualEvent={e => console.log(e.target)} />
```

#### change `main` field to be node

Now that we are supporting SSR for legacy(non-ESM) systems I need to use the main field to indicate a node env. We will be using the "browser" field for the client build in Solid. This straight up breaks Jest which doesn't respect that. I've created `solid-jest` to handle this.

https://github.com/solidjs/solid-jest

### New Features

#### Namespace Types

Types added for Namespace attributes. You probably won't need most of these because they are for more advanced usage. However to use them you need to extend the JSX Namespace:

```ts
declare module "solid-js" {
  namespace JSX {
    interface Directives {
      // use:____
    }
    interface ExplicitProperties {
      // prop:____
    }
    interface ExplicitAttributes {
      // attr:____
    }
    interface CustomEvents {
      // on:____
    }
    interface CustomCaptureEvents {
      // oncapture:____
    }
  }
}
```

#### Lazy component preload

Lazy components now have a preload function so you can pre-emptively load them.

```js
const LazyComp = lazy(() => import("./some-comp"));

// load ahead of time
LazyComp.preload();
```

#### Error Boundary reset

Error boundaries now have the ability to reset themselves and try again. It is the second argument to the fallback.

```js
<ErrorBoundary
  fallback={(err, reset) => {
    if (count++ < 3) return reset();
    return "Failure";
  }}
>
  <Component />
</ErrorBoundary>
```

## 0.24.0 - 2021-02-03

This release is the start of the rework of the SSR solution. Consolidating them under a single method. Unfortunately this one comes with several breaking changes.

### Breaking Changes

#### Removed `solid-js/dom`

It's been a few versions deprecated. It's gone.

#### Updated Resource API

Changed to more resemble SWR and React Query. Needed to remove `createResourceState`so now need to use a getter over `createResource` to get same effect. See updated documentation.

#### Change SSR render call signatures

They now return results objects that include the generated hydration script. No more need to generate it separately. Also comes autowrapped in the `script` tag now.

#### `assignProps` to `mergeProps`

While you use them the same way mostly it no longer has `Object.assign` semantics and always returns a new object. This is important as in many cases we need to upgrade to a Proxy.

#### Renamed `getContextOwner` to `getOwner`

Removes confusion around context and consistent with new helper `runWithOwner`.

#### Solid Element no longer uses State for props

This reduces the size of the library especially for those not using state. It also should slightly increase performance as no need for deep nesting of proxies. It also makes things behave more consistently avoided unintended deep wrapping.

### Non-breaking Changes

#### New non-reactive Async SSR

I have now combined sync/streaming/async SSR into the same compiler output. To do so I have developed a new non-reactive Async SSR approach. After realizing how fast Solid renders, it occurred to me on the server we could do a much simpler approach if we were willing to re-render all content in Suspense boundaries. While that is some wasted work, compared to including the reactive system it's a killing.

#### Increase SSR Performance

Through reusing static strings in the template we reduce repeated creation costs. This small improvement can make 5-8% improvements where you have many rows.

#### Event Delegation

Solid is now being more strict on what events it delegates. Limiting to standard pointer/touch/mouse/keyboard events. Custom events will no longer be delegated automatically. This increases compatibility for Web Component users who don't compose their events. Non-delegated events will still work and binding array syntax with them.

#### State getters no longer memos

Automatic memos put some constraints on the disposal system that get in the way of making the approach flexible to hold all manner of reactive primitives. Some previous limitations included not being able to have nested getters. You can still manually create a memo and put it in a getter but the default will not be memoized.

### New Features

#### `children` helper

Resolves children and returns a memo. This makes it much easier to deal with children. Using same mechanism `<Switch>` can now have dynamic children like `<For>` inside.

#### "solid" Export Conidition

This is the way to package the JSX components to be compiled to work on server or client. By putting the "solid" condition the source JSX will be prioritized over normal browser builds.

### Bug Fixes

- Top level primitive values not working with `reconcile`
- Fix Dynamic Components to handle SVG
- Rename potentially conflicting properties for event delegtion
- Fixed State spreads to not loose reactiviy. Added support for dynamically created properties to track in spreads and helpers
- TypeScript, always TypeScript

## 0.23.0 - 2020-12-05

This release is mostly bug fixes. Breaking change for TS users. JSX types no longer pollutes global namespace. This means you need to update your projects to import it.

For users TS 4.1 or above add to your tsconfig to have JSX types in all your TSX files:

```js
"compilerOptions" {
  "jsx": "preserve",
  "jsxImportSource": "solid-js",
}
```

Or mixing and matching? You can set JSX types per file using the pragma at the top of each file:

```js
/* @jsxImportSource solid-js */
```

You can now import `JSX` types directly from Solid as neccessary:

```js
import { JSX } from "solid-js";
```

## 0.22.0 - 2020-11-14

### Unified Exports (Deprecation `solid-js/dom`)

Solid now has streamlined exports for isomorphic development. This means from now on using `solid-js/web` instead of `solid-js/dom`. Based on compiler options it will swap out the appropriate packages for web. You should only ever import `solid-js`, `solid-js/h`, `solid-js/html`, and `solid-js/web` directly in your code.

`solid-js/web` now exports an `isServer` field which indicates whether the code is executed for server rendering. This is constant in the respective packages meaning it can allow for powerful treeshaking/dead code elimination in final bundles even when used directly in end user code or 3rd party libraries.

### Dev Mode

Aliasing `solid-js` to `solid-js/dev` in your bundler links in a Dev mode of Solid. It's still a WIP process but it introduces some new APIs. First signals and state (and resources) have the ability to set a name for debug purposes as an options argument.

We also export a `serializeGraph` method which will serialize all the signals below the executing context in the reactive graph.

Finally there is a new `globalThis._$afterUpdate` hook that can be assigned that will be called after every render that can be used for tracking purposes.

This is just the start but it is my intention to develop these features to allow for better HMR and DevTools.

> Note: If the libraries are not being pulled into your bundle and are treated as external you may need to alias `solid-js` to `solid-js/dev` in your bundler in order to use dev mode.

### Self contained HyperScript/Lit Modules

We now ship the respective DOM expressions code. This makes it much easier to use directly from a CDN like Skypack. You literally can develop with Solid in the old school write it in notepad before npm was a thing sort of way.

```html
<html>
  <body>
    <script type="module">
      import { createSignal, onCleanup } from "https://cdn.skypack.dev/solid-js";
      import { render } from "https://cdn.skypack.dev/solid-js/web";
      import html from "https://cdn.skypack.dev/solid-js/html";

      const App = () => {
        const [count, setCount] = createSignal(0),
          timer = setInterval(() => setCount(count() + 1), 1000);
        onCleanup(() => clearInterval(timer));
        return html`<div>${count}</div>`;
      };
      render(App, document.body);
    </script>
  </body>
</html>
```

Save this in a text file called "site.html" and double click it and instant Solid in your browser.

### renderToWebStream

New `renderToWebStream` for synchronous SSR mode. This allows us to stream from things like Cloudflare Workers.

### createMutable

New mutable state primitive. Useful for interopt with other libraries. We can use this potentially for things like Vue/MobX compat. Or when we need to interact with libraries that can't be aware of Solid's reactive system, yet we want to capture updates. It supports getters and setters.

Use with caution as it can promote difficult to reason about code, anti-patterns, and unexpected performance cliffs. Keep in mind Vue and MobX care less about these inefficient patterns since they have a VDOM safety net. We do not. For advanced users only.

```js
const user = createMutable({
  firstName: "John",
  lastName: "Smith",
  get fullName() {
    return `${this.firstName} ${this.lastName}`;
  },
  set fullName(value) {
    const parts = value.split(" ");
    batch(() => {
      this.firstName = parts[0];
      this.lastName = parts[1];
    });
  }
});
console.log(user.fullName); // John Smith
user.fullName = "Jake Murray";
console.log(user.firstName); // Jake
```

### State Getter/Setters are now Wrapped

Getters are now wrapped in `createMemo` and setters in `batch`. However, this introduces a new limitation that they can only be top level to have this behavior.

### State compatible with Prop Helpers

You can now use state with `assignProps` and `splitProps` helpers.

### Removed DOM SSR

No longer supporting hydratable DOM SSR in patched(ie... JSDOM) node environments. Use the standard SSR methods instead. Can still run Solid in JSDOM for things like Jest, but can't be used for isomorphic development.

## 0.21.0 - 2020-10-17

### Attribute and Prop changes

We will now default to using Attributes where possible to be consistent. Solid is aiming to generally reflect the case insensitiveness of HTML. Custom Elements remain the one place that defaults to property setters on Dynamic elements.

While TypeScript 4.2 is yet to be released, we are introduce `attr`, `prop`, `use` and `style` namespace directives. To allow more expressiveness in binding syntax.

### Other Changes

- New `on` and `onMount` helpers
- More performant SSR escaping
- Lazy eval SSR Component props (fix SSR Context API)
- Add support for SSR with Solid Styled Components
- Fix Lit Dom Expressions style in Template tags
- Fix JSX Types

## 0.20.0 - 2020-09-24

### Re-scheduling Reactivity.

This release makes large changes to the Reactive System. Key changes are deferring `createEffect` to be after rendering and introducing `createComputed` do reactive graph updates like loading async data.

### Concurrency

In addition the reactive model brings updates to Suspense and Transitions. Solid now has true concurrent rendering at a granular level. This mechanism does differ from React as it currently only supports a single future.

### Removed APIs

`afterEffects`, `createDependentEffect`, and `suspend` have been removed as they no longer make sense with the new reactive system timing.

## 0.19.0 - 2020-08-23

API Changes to support better SSR

### Breaking Changes:

#### Set State

Mutable form is no longer a default. It was strangely inconsistent as you could accidentally mutate in immutable forms. No indicator why it should behave differently and work. Increased the size of `state` for everyone and added performance overhead with additional proxy wrapping. Also it was based on returning undefined meaning function forms could never return undefined to blank a vlue. Solid has changed it into a state setter modifier `produce` after ImmerJS naming.

```js
// top level
setState(produce(s => (s.name = "John")));

// nested
setState(
  "user",
  produce(s => (s.name = "John"))
);
```

#### Prop APIs

After writing `setDefaults`, `cloneProps`, and about to introduce `mergeProps` it became clear we can do this all with a single `assignProps` helper. So the former has been removed and now we have:

```js
// default props
props = assignProps({}, { name: "Smith" }, props);

// clone props
newProps = assignProps({}, props);

// merge props
assignProps(props, otherProps);
```

It follows the same pattern as ES `Object.assign` adding properties to the first argument and returning it. Except this method copies property descriptors without accessing them to preserve reactivity.

#### `freeze` & `sample` have been renamed

These APIs never had the most obvious naming, borrowing from SRP and digital circuit concepts rather than common english. They are now `batch` and `untrack` respectively which better reflect their purpose. These are now deprecated and will be removed in next minor version.

#### Resource API

For better automatic hydration support it is prudent to change resource signatures to take functions that return promises rather than promises themselves. This factory function has a lot advantages. This allows the library to decide whether to execute it or not. In certain cases we can choose skipping creating the promise altogether. It also leaves the door open for things like retry.

We use this mechanism to wire up streamed data from the server and automatic data hydration for resources rendered into the page in async SSR.

#### SSR Improvements

New experimental support for Suspense aware synchronous, asynchronous, and streaming SSR with hydration, progressive hydration, and automatic isomorphic data serialization. Completely removed what was there before with a simple static generator and more examples, so all existing projects using `solid-ssr` package will break with this release. This is a much better foundation, and I hope to build better things on top.

### New

#### State Getters

For convenience of passing derived values or external reactive expressions through Solid's state initializer you can now add `getter`'s.

```jsx
const [state, setState] = createState({
  firstName: "Jon",
  lastName: "Snow",
  get greeting() {
    return `You know nothing ${state.firstName} ${state.lastName}`;
  }
});

return <div>{state.greeting}</div>;
```

#### Control Flow

Dynamic allows swapping Component dynamically.

```jsx
// element tag name
const [comp, setComp] = createSignal("h1");

<Dynamic component={comp()} {...otherProps} />;

// Component
setComp(MyComp);
```

ErrorBoundary catches uncaught downstream errors and shows a fallback.

```jsx
<ErrorBoundary fallback={<div>Something went terribly wrong</div>}>
  <MyComp />
</ErrorBoundary>
```

#### Portals render in the Head

You can now render portals in the head with no additional div element.

#### Multi-version detection

Common hard to track issue with Solid is when multiple versions of the library are running on the same page. It breaks reactivity, and is sometimes difficult to notice. Solid now detects if a version has already been loaded at runtime and complains.

### Bug Fixes & Updates

Arguably a new feature but Solid now detects computation owners with pending dependency changes when trying to resolve nested computations. In so it will resolve those dependencies first. This fixes a long time issue with conditional processing with not directly related reactive atoms.

Improved TypeScript Types.

## 0.18.0 - 2020-05-01

A lot of bug fixes, and introduction of string based SSR.
Breaking Changes:

- Removal of `forwardRef`. Value and function handled by just `ref`.
- Change to how TypeScript is managed. Brought all JSX types inside the repo, and improved Component typing.
- Changed default renderer in `solid-ssr` to string renderer.

## 0.17.0 - 2020-03-24

A lot of consolidation in preparation for release candidate

- Big refactor of core reactive system and render list reconciler
  - Significantly smaller reducing core by atleast 3kb minified
- Better handling of nested reactive nodes in Fragments
- Update SSR mechanisms, added progressive event hydration, created repo for SSR environment (`solid-ssr`)
- `@once` compiler hint to statically bind values
- Better wrapping hueristics for booleans and ternaries in JSX

Breaking Changes

- Removed `transform` prop from control flow. Idiomatic approach is to make a HOC for transformations of this nature.
- Removed selectWhen/selectEach control flow transforms.
- Changed event system
  - `on____` prop to stop differentiating on case. Super confusing.Instead will try to delegate unless unable. Made TypeScript all CamelCase (although technically both forms behave identically)
  - Removed `model` event delegation approach. Instead to create bound event use array: `onClick={[handler, row.id]}`. Inspired by Inferno's `linkEvent` helper.
  - Renamed `events` prop to `on` prop
  - Added `onCapture` prop for capture events

## 0.16.0 - 2020-01-14

Big changes to experimental features:

- New resource API `createResource` and `createResourceState` to replace `loadResource`. These are built to prioritize read capabilities and simplify implementation.
- Support for Async SSR `renderToString` now returns a promise. Uses Suspense to know when it is done.
- Progressive Hydration with code splitting support. Ability to track events and replay as hydration completes to reduce "uncanny valley". Components can be lazily loaded even during hydration. **No support for async data on hydration yet**, so render it from server and load into state synchronously.
- New error boundary api with `onError`. If an error occurs in context or child context the nearest handler/s will be called.
- Deprecating the `force` `setState` modifier as it is confusing.

## 0.15.0 - 2019-12-16

A lot fixes and new features:

- Suspense improvements: `SuspenseList`, `useTransition`, trigger on read. Update API, and added `reload` and retry capability. Removed need for `awaitSuspense` by making `Show` and `Switch` control flows `Suspense` aware.
- Deprecate `selectWhen` and `selectEach`.
- Untrack all Components. No more fear of nesting Components in JSX expressions. Top level in a Component will always be inert now.
- Support for safe boolean and logical operators. This allows for the same optimization as the `Show` control flow for simple inline JSX conditionals like `<div>{state.count > 5 && <MyComp />}</div>`.
- Support for non-curried operator forms. All operators now support an accessor first form as well as the functional curried form. Ex `map(() => state.list, item => item)`
- Fix issues with spreading over `children` props.
- Better Type Definitions.

## 0.14.0 - 2019-11-16

v0.14.0 brings changes to the render runtime and `setState` API

- Adds diffing to batched computations to improve update performance
- Supports support for mutable(TypeScript safe) `setState` API inspired by Immer. Function setters in Solid now pass a mutable version of state. Modifying will schedule updates. This form must not return a value. It can still be used immutably simply by returning the new value.
- Changes how `force` and `reconcile` helpers work. They can now be used on nested paths.
- Removes support for multi-path `setState`.

## 0.13.0 - 2019-10-27

v0.13.0 contains large changes to the reactive system and compiler.

The main update is to simplify reactivity by removing computation recycling. While this was a useful feature to avoid unnecessary computation nodes, Solid now uses batching as a different approach to get similar results. Most templating libraries can offer breakneck update speeds without fine grained updates. The real cost of these top down approaches is the need to redo structural reconciliation. The current approach is that different computations will be created for each:

- Dynamic insert expression (any expression between tags)
- Spread operator
- JSX template entry point(Top level tag, Fragment, or Component Children)

To aid in performance simple text inserts the `textContent` binding is now optimized so they can be batched.

In addition there are some improvements to template cloning and SVG handing in SSR.

## 0.12.0 - 2019-10-18

v0.12.0 contains a breaking change to the reactive rendering system

- Removal of explicit dynamic binding, bindings will default to reactive unless impossible to be so (literal, function declaration, simple variable)
- SVG Camelcase attribute Support
- Prettier now supported!

## 0.11.0 - 2019-09-27

v0.11.0 continues to add updates to the reactive system as well as some new features:

- Fix reactivity resolution ordering on downstream conditionals
- Add basic (non-namespaced) SVG support
- Add experimental Server Side Rendering and Client Side Hydration capabilities
- Add Suspense aware control flow transformation (`awaitSuspense`)
- Allow state objects to track functions
- More TypeScript definition improvments and fixes

## 0.10.0 - 2019-08-11

v0.10.0 makes significant changes to the reactive system. Key updates:

- Fixed synchronicity on all hooks/control flows.
- Adds the ability to use comparators on `createMemo`.
- Fixes bugs with nested control flows.
- Fixes bugs with Suspense.
- Update Suspense `delayMs` to `maxDuration` to match React. (Usage of `maxDuration` still experimental)

## 0.9.0 - 2019-07-20

v0.9.0 makes signifigant changes to underlying reconciler.

- New Control Flow
- Removes Custom Directives
- New Functional Operators

## 0.8.0 - 2019-06-14

v0.8.0 brings further improvements in reducing bundle size and optimizations in reactivity. New Features:

- Universal loadResource API
- afterEffects hook
- Switch Control Flow

## 0.7.0 - 2019-05-25

v0.7.0 brings further improvements in tree shaking, Context API including Provide control flow, and suspense helpers for loading Async Components and Data.

This is a breaking change as in order to support this version, Solid has forked S.js the underlying library and now ships with it built in. This means Solid will no longer be compatible other S.js libraries. It is a turning point but enables the powerful new features.

## 0.6.0 - 2019-05-07

v0.6.0 brings a Tree Shakeable runtime. This means when Solid used with JSX the compiler can intelligently only include the code that is being used.

This is a breaking change in that:

- No longer need to import 'r' and selectWhen and selectEach directives have been moved to solid-js from solid-js/dom. You should not need to import from 'solid-js/dom' directly anymore as your compiled code will do it automatically.
- HyperScript and Lit imports have been made the default import now.. ex:

```js
import html from "solid-js/html";
```

- Tidied up the compiled template code. This should make it much nicer to debug when not minified.

## 0.5.0 - 2019-04-14

- Add support for multiple renderers (JSX, Tagged Template Literals, HyperScript). Added direct imports or 'solid-js/dom' alternatives 'solid-js/html' and 'solid-js/h'.
- Reorganized dependencies work.

## 0.4.2 - 2019-03-18

- Add fallbacks for control flow
- Add new Portal Control Flow - This allows nodes to be rendered outside of the component tree with support for satelite ShadowRoots.
- Add new Suspend Control Flow - This renders content to a isolated document and display fallback content in its place until ready. Good for nested Async Data Fetching.
- Default node placeholders to comments (improved text interpolation)
- Added events binding for irregular event names

## 0.4.0 - 2019-02-16

- Rename API to create\_\_ to be semantically correct
- Added implicit event delegation

## 0.3.8 - 2019-01-31

- Add support for HyperScript

## 0.3.7 - 2019-01-16

- Improved data reconciler performance
- Added data reconciler options

## 0.3.4 - 2019-01-04

- Added optional comparator for signals.
- Removed redundant type checks and extra function calls.
- Changed S.js to a dependency instead of a peer dependency.

## 0.3.2 - 2018-12-30

- Separated useSignal getter/setters for clearer more consistent API

## 0.3.1 - 2018-12-29

- Remove operators from core package since are auxilliary with new API.
- Updated JSX Dom Expressions to use new control flow JSX and JSX Fragment support.

## 0.3.0 - 2018-12-25

- New setState API inspired by Falcor paths to handle ranges.
- Reduction in API to remove State object functions and change to explicit methods.
- Expose reconcile method to do deep differences against immutable data sources (previously automatically done for selectors).
- Removed 'from' operators as limited usefulness with new patterns.

## 0.2.0 - 2018-11-13

- Large simplifications to remove inconsistency around wrapping and unwrapping values. State values are always wrapped get, and fully unwrapped on set.
- Updated binding syntax. Dynamic expressions are now bound with an inner parenthesis `{( )}`js
- Removed Immutable State object. May attempt something similar in the future but at this time it wasn't worth the effort trying to attempt both. There are better approaches to Proxy Immutable data structures.

在 macOS (Apple) 系统上安装 CMake，主要有三种最常用的方法。对于大多数开发者来说，**推荐使用 Homebrew**，因为它最简单且易于管理更新。

---

### 方法 1：使用 Homebrew 安装（推荐）

如果你已经安装了 [Homebrew](https://brew.sh/)（macOS 的包管理器），这是最快的方法。

1. 打开**终端 (Terminal)**。
2. 输入以下命令并回车：
```bash
brew install cmake

```


3. 等待安装完成后，验证是否成功：
```bash
cmake --version

```



---

### 方法 2：通过官网下载安装包 (GUI)

如果你更喜欢传统的安装方式，或者不需要经常通过命令行更新，可以去官网下载。

1. 访问 [CMake 官网下载页面](https://cmake.org/download/)。
2. 在 **Binary distributions** 下找到 macOS 栏目，下载 `.dmg` 文件（例如 `cmake-3.x.x-macos-universal.dmg`）。
3. 打开下载的 `.dmg` 文件，将 **CMake.app** 拖入 **Applications (应用程序)** 文件夹。
4. **配置命令行工具（重要）：**
安装好后，为了能在终端直接使用 `cmake` 命令，你需要运行 CMake 应用程序，然后在菜单栏选择：
`Tools` -> `How to Install For Command Line Use`
通常最简单的方法是在终端运行以下命令来创建软连接：
```bash
sudo "/Applications/CMake.app/Contents/bin/cmake-gui" --install

```



---

### 方法 3：使用 MacPorts 安装

如果你使用的是 MacPorts 而不是 Homebrew：

1. 打开终端。
2. 输入以下命令：
```bash
sudo port install cmake

```



---

### 为什么需要安装 CMake？

CMake 是一个开源、跨平台的编译配置工具，它并不直接编译代码，而是通过 `CMakeLists.txt` 文件生成对应平台的工程文件（如 macOS 上的 Xcode 项目或 Unix 的 Makefiles）。

### 常见问题排查

* **权限问题：** 如果在执行 `brew` 或 `sudo` 命令时提示权限不足，请确保你拥有管理员权限，并按提示输入系统密码（输入时不会显示字符）。
* **路径未找到：** 如果安装后终端提示 `command not found: cmake`，请尝试重启终端，或者检查你的 `PATH` 环境变量。对于 Homebrew 用户，通常路径是自动配置好的。

**你需要我帮你编写一个基础的 `CMakeLists.txt` 模板来测试安装是否成功吗？**