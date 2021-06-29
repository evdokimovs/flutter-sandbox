# Flutter sandbox

Using [flutter_webrtc] library, create application for video/audio calls between two users.

Application should work on Android platform.

[Application design on Figma](https://www.figma.com/file/E9BZb5DuoTpHgTjriqMwip/Untitled)
[Application prototype](https://www.figma.com/proto/E9BZb5DuoTpHgTjriqMwip/Untitled)




## Features of the application:

1. Show video of partner participant;
2. Play audio of partner participant;
3. Frontal camera video publishing;
4. User's audio publishing;
5. Preview of the publishing video;
6. Video publishing disabling/enabling;
7. Audio publishing disabling/enabling.




## Implementation requirements:

1. Code documentation;
2. Unit tests;
3. [E2E tests][3].




## [WebRTC signaling][1] server

For [signaling][1] between clients you can use server hosted at `ws://example.com/<your username>`.
Messages sent to this server, will be broadcasted to the all _other_ clients connected to the same
server without changes. Format of messaging between clients you can choose by yourself.


### Working with server example

1. Alice initiates WebSocket connection with `ws://example.com/foobar`
2. Bob initiates WebSocket connection with `ws://example.com/foobar`
3. Alice sends message with text 'Hello Bob'
4. Bob receives message with text 'Hello Bob'
5. Bob sends message with text 'Hello Alice'
6. Alice receives message with text 'Hello Alice'




## [ICE] servers

As [ICE] servers you can use `stun:stun.stunprotocol.org:3478` and `stun:stun.l.google.com:19302`.




## Releasing

To release your application you can run `make release` command.

Or you can do it manually:

```
$ git tag -d latest

$ git tag latest

$ git push latest
```

CI will build your application and create release on GitHub with built APK automatically.




## Needed assets

All needed for this application assets you'll find in the `assets` directory.




## Final design of application

Final design of application can differ from the provided one. This design is needed only
for better explanation of expected application.



## Useful links

- [Basics of WebRTC](https://www.html5rocks.com/en/tutorials/webrtc/basics/)
- [Basic WebRTC glossary](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Protocols)
- [Basics of WebRTC signaling](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Connectivity)
- [WebRTC glossary](https://webrtcglossary.com/)
- [Flutter WebRTC usage examples](https://github.com/flutter-webrtc/flutter-webrtc/tree/master/example)
- [Flutter WebRTC demo](https://github.com/flutter-webrtc/flutter-webrtc-demo)
- [Dead simple WebRTC example written with JS](https://shanetully.com/2014/09/a-dead-simple-webrtc-example/)




## Repository requirements


### Files

Repository __must NOT contain__ (so have them [Git-ignored] to avoid accidents):
- __configuration__ files __of__ developer's __local toolchain__ (unless this configuration is suitable for all project developers);
- __compilation/build results/artifacts__ of source code;
- any __caches or temporary files__;
- __configuration__ files __for running__ application (except examples or Dockerized development environment configurations which are the same for all project developers).

__For keeping an empty directory__ in a repository __use the `.gitkeep` file__ inside that directory.

#### Naming

__Start directory with `.`__ if it contains some __temporary files which do not require direct manipulations__ and are going to be omitted by tools (caches, temp files, etc.). This is a quite common practice (see `.git/`, `.idea/`, `.gradle/`, etc.).  
Also, __all temporary cache files__ must be __placed inside a `.cache/`__ top-level directory of the repository, unless this is impossible for somewhat reasons.

__To emphasize toolchain directories__ (ones which do not contain project sources itself, but rather contain files of a project toolchain) their __name may be started with `_`__, which will make them to "bubble-up" in a repository source tree, so will allow easily to distinguish them from actual project sources (both for humans and tools).

#### Layout example

```bash
tree -a -v --dirsfirst .
```

```tree
.
├── .cache/
│   └── cargo/
├── .git/
├── .idea/
├── _build/
│   └── artifacts/
│       ├── .dockerignore
│       ├── .gitignore
│       └── Dockerfile
├── _dev/
│   └── config.toml
├── src/
│   └── main.rs
├── .editorconfig
├── .gitignore
├── CHANGELOG.md
├── CONTRIBUTING.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE.md
└── README.md
```

### Branches and tags

Every repository contains the following branches:

- `master` - __mainline version__ of the project. Any new project release is usually created from this branch. Developing directly in this branch is forbidden. It __accepts new changes via PRs (pull requests)__.

Any other possible branches and tags can be created and used by developers as they need.

#### Branch naming

[Git] branch name __must__ meet the following requirements:
- consist of __English words__;  
    👍 `fix-tests-failure`  
    🚫 `fix-defectum-probat`
- use __only dashes to separate words__;  
    👍 `fix-tests-failure`  
    🚫 `fix_tests_failure`
- use __[imperative mood][2] for verbs__;  
    👍 `fix-tests-failure`  
    🚫 `fixes-tests-failure`
- __start with the issue number__ when branch is related to some issue (but __DO NOT use PR (pull request) numbers__);  
    👍 `23-fix-tests-failure`  
    🚫 `fix-tests-failure`
- __reflect the meaning of branch changes__, not the initial problem.    
    👍 `23-fix-tests-failure`  
    🚫 `23-problem-with-failing-tests`


### Commits

Every __commit message must contain a short description__ of its changes that meet the following requirements:
- be __on English__ (no other language is allowed);
- __start with a capital letter__;
- has __no punctuation symbols at the end__ (like `;`, `:` or `.`);
- use __[imperative mood][2] for verbs__ (as if you are commanding someone: `Fix`, `Add`, `Change` instead of `Fixes`, `Added`, `Changing`);
- use __marked list for multiple changes__, prepended by __one summary line__ and __one blank line__, where each __list item__ must:
    - __start with a lowercase letter__;
    - has __no punctuation symbols at the end__.

##### 👍 Single-line commit message example

```
Update Employee salary algorithm
```

##### 👍 Multiple-line commit message example

```
Implement employees salary and ajax queries

- update Employee salary algorithm
- remove unused files from public/images/ dir
- implement ajax queries for /settings page
```

##### 🚫 Wrong commit message examples

- Summary line starts with a lowercase letter:

    ```
    update Employee salary algorithm
    ```

- Verb is not in the [imperative mood][2]:

    ```
    Updates Employee salary algorithm
    ```

- Unnecessary punctuation is present:

    ```
    Update Employee salary algorithm.
    ```

    ```
    Implement employees salary and ajax queries:
    
    - update Employee salary algorithm;
    - remove unused files from public/images/ dir.
    ```

- Missing blank line between the summary line and the marked list:

    ```
    Implement employees salary and ajax queries
    - update Employee salary algorithm
    - remove unused files from public/images/ dir
    ```

- Marked list is indented:

    ```
    Implement employees salary and ajax queries
    
      - update Employee salary algorithm
      - remove unused files from public/images/ dir
    ```

- Marked list items start with a capital letter:

    ```
    Implement employees salary and ajax queries
    
    - Update Employee salary algorithm
    - Remove unused files from public/images/ dir
    ```


### Pushing

Developer __must push all his changes__ to the remote __at the end of his working day__. This both prevents from accidental work losses and helps a lead to track developer's progress.





## Code style

All Dart source code must follow [Effective Dart] official recommendations.

Any rules described here are in priority if they have conflicts with
Effective Dart recommendations.


### `switch` statement

If __`case` section__ consists of __one line exactly__, then it's allowed to
__place `break` statement on the same line__.

#### 👍 Switch statement example

```dart
switch (fruit) {
  case 'apple':
    print('delish'); break;
  case 'durian':
    print('stinky');
    break;
  case 'peach':
    print('sweet');
    print('fuzzy');
    break;  
}
```

#### 🚫 `break` statements placed on the same line in multi-line `case` section:

```dart
switch (fruit) {
  case 'peach':
    print('sweet');
    print('fuzzy'); break; 
}
```


### Collections

__Trailing comma__ of last item is __mandatory for multi-line__ collection
declaration.  
__No trailing comma__ required __for single-line__ collection declaration.

#### 👍 Collections examples

- Multi-line collection:

    ```dart
    Map<int, String> map = {
      0: 'zero',
      1: 'one',
    };
    args.addAll([
      "--mode",
      "release",
      "--checked",
    ]);
    ```

- Single-line collection:

    ```dart
    Map<int, String> map = {0: 'zero', 1: 'one'};
    args.addAll(["--mode", "release", "--checked"]);
    ```

#### 🚫 Wrong collections examples

- Multi-line collection without trailing comma:

    ```dart
    Map<int, String> map = {
      0: 'zero',
      1: 'one'
    };
    args.addAll([
      "--mode",
      "release",
      "--checked"
    ]);
    ```

- Single-line collection with trailing comma:

    ```dart
    Map<int, String> map = {0: 'zero', 1: 'one',};
    args.addAll(["--mode", "release", "--checked",]);
    ```


### dartfmt

__Using `dartfmt` is forbidden__ at the moment, as far as this utility is not 
clever enough to consider some important formatting moments which leads to bad
code readability.





[1]: https://webrtcglossary.com/signaling/
[2]: https://en.wikipedia.org/wiki/Imperative_mood
[3]: https://www.browserstack.com/guide/end-to-end-testing

[Medea]: https://github.com/instrumentisto/medea
[flutter_webrtc]: https://pub.dev/packages/flutter_webrtc
[ICE]: https://webrtcglossary.com/ice/
[Effective Dart]: https://www.dartlang.org/guides/language/effective-dart
[Git-ignored]: https://git-scm.com/docs/gitignore
[Git]: https://git-scm.com
