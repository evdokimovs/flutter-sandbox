Flutter sandbox
===============

Using [flutter_webrtc] library create an application for audio/video calls between two users.

Application should work on Android platform.

[Design](https://www.figma.com/file/E9BZb5DuoTpHgTjriqMwip/Untitled)
| [Prototype](https://www.figma.com/proto/E9BZb5DuoTpHgTjriqMwip/Untitled)




## Required features

1. Display video and play audio of a partner participant.
2. Publish video from the frontal camera of a device.
3. Publishing user's audio.
4. Preview of the published video.
5. Ability to enable/disabled published video/audio.




## Implementation requirements

1. Code should be documented with [dartdoc] and the generated documentation should be available on GitHub Pages.
2. Code should be covered with unit tests.
3. [E2E (end-to-end) tests][3] should cover all the required features.




## [WebRTC signaling][1] server

For [signaling][1] between clients you can use simple WebSocket server provided within this project. Messages sent to this server, will be broadcast to the all _other_ clients connected to the same server without any changes. It's up to you to define the format of messages.


### Deploying on [Heroku]

1. Create account on [Heroku] (if you don't have one).
2. Copy [Heroku] API key from the [account page][4].
3. Go to [Actions Secrets][5] settings in your GitHub repository.
4. Add the following repository keys:
    - `HEROKU_API_KEY` - API key which you copied at step 2;
    - `HEROKU_EMAIL` - email with which you registered on [Heroku].
5. Go to ['Deploy signaling server to Heroku'][6] GitHub workflow.
6. Run workflow on `master` branch.

Now your instance of a signaling server can be accessed at `wss://flutter-sandbox-{{ YOUR GITHUB USERNAME }}.herokuapp.com`.


### Example of interaction with server

1. Alice initiates a WebSocket connection on `wss://flutter-sandbox-ferris.herokuapp.com` endpoint.
2. Bob initiates a WebSocket connection on `wss://flutter-sandbox-ferris.herokuapp.com` endpoint.
3. Alice sends message with text `Hello Bob`.
4. Bob receives message with text `Hello Bob`.
5. Bob sends message with text `Hello Alice`.
6. Alice receives message with text `Hello Alice`.




## [ICE] servers

Use `stun:stun.stunprotocol.org:3478` and `stun:stun.l.google.com:19302` as [ICE] servers in your application.




## Releasing

To release your application run `make release` command.

Or you can do it manually:

```bash
$ git tag -d latest
$ git tag latest
$ git push origin latest --force 
```

CI will build your application and create a release on GitHub with `.apk` built automatically.




## Required assets

All the assets required for this application are located in the `assets/` directory.




## Final design of application

Final design of the implemented application may vary from the provided one. The provided design aims only to explain the expected result better.




## Useful links

- [Basics of WebRTC](https://www.html5rocks.com/en/tutorials/webrtc/basics)
- [Basic WebRTC glossary](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Protocols)
- [Basics of WebRTC signaling](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Connectivity)
- [WebRTC glossary](https://webrtcglossary.com)
- [Flutter WebRTC usage examples](https://github.com/flutter-webrtc/flutter-webrtc/tree/master/example)
- [Flutter WebRTC demo](https://github.com/flutter-webrtc/flutter-webrtc-demo)
- [Dead simple WebRTC example written with JS](https://shanetully.com/2014/09/a-dead-simple-webrtc-example)




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

All Dart source code must follow [Effective Dart] official recommendations. For code formatting [dartfmt] must be used (and verified on CI).





[1]: https://webrtcglossary.com/signaling/
[2]: https://en.wikipedia.org/wiki/Imperative_mood
[3]: https://www.browserstack.com/guide/end-to-end-testing
[4]: https://dashboard.heroku.com/account
[5]: /../../settings/secrets/actions
[6]: /../../actions/workflows/deploy-server.yml

[dartdoc]: https://dart.dev/tools/dartdoc
[dartfmt]: https://dart.dev/tools/dart-format
[Effective Dart]: https://www.dartlang.org/guides/language/effective-dart
[flutter_webrtc]: https://pub.dev/packages/flutter_webrtc
[Git]: https://git-scm.com
[Git-ignored]: https://git-scm.com/docs/gitignore
[Heroku]: https://www.heroku.com
[ICE]: https://webrtcglossary.com/ice
[Medea]: https://github.com/instrumentisto/medea
