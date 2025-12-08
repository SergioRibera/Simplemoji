
## New Version [1.2.3]

### 🚀 Features

- Add auto insert into input using IME

### 🐛 Bug Fixes

- Some results not show on search

### ⚙️ Miscellaneous Tasks

- Fix get version from github aur workflow

## New Version [1.2.2]

### 🐛 Bug Fixes

- Window size extra large
- Prevent show recent line on search

### 🚜 Refactor

- Remove unnecesary commented code

### ⚙️ Miscellaneous Tasks

- Release simplemoji version {{version}}

## New Version [1.2.1]

### ⚙️ Miscellaneous Tasks

- Release simplemoji version {{version}}
- Disable recents by default
- Fix aur publish

### Build

- Update slint

## New Version [1.2.0]

### 🚀 Features

- Last fixes and update doc for new recents feature
- Customize corner radius on emojis

### 🐛 Bug Fixes

- Windows build
- Aur publish
- Solve the hide window failed
- Tone from args was not applied in the tabs content
- Move name by codes position
- Remove padding right on preview

### ⚙️ Miscellaneous Tasks

- Release simplemoji version {{version}}

### Feat

- Added recents settings and UI

### Fix

- Fixed clippy error
- Activities and objects tab were switched

### Build

- Update nix build required

## New Version [1.1.1]

### 🐛 Bug Fixes

- *(nix)* Export correct packages
- *(nix)* Export simplemoji as an app
- *(bundle)* Remove msi as it is empty

### ⚡ Performance

- Remove skia software

### ⚙️ Miscellaneous Tasks

- Release simplemoji version {{version}}
- Add changelog file
- Update some deps and improvement gitignore
- Integrate cargo dist to release workflow
- Bump clap from 4.5.41 to 4.5.45
- Bump display-info from 0.5.4 to 0.5.5
- Bump emojis from 0.7.0 to 0.7.2

### Build

- Update cargo dist requirements

## New Version [1.1.0]

### 🚀 Features

- Improvement compile and bundle nixos system
- Windows supports

### 🐛 Bug Fixes

- Focus windows
- Follow mouse position on window creation
- Update deps

### ⚡ Performance

- Improvement performance

### ⚙️ Miscellaneous Tasks

- Release simplemoji version {{version}}
- Update linux release pipeline
- Bump emojis from 0.6.4 to 0.7.0

### Build

- Improvement compile times
- Update deps

## New Version [1.0.0]

### 🚀 Features

- Fuzzing alternative to search
- Enable fuzzing search from args
- Debug key pressed on screen
- Press enter or space to call action into emoji selection
- Full keyboard navigation support correct works 🥳
- Keyboard navigation support
- Emoji preview change with tab selection
- Change tabs tone on change global tone
- Tab and sift-tab simple based navigation
- Close app on unfocus window and scape press
- Add start event to move window to mouse position
- Improvement ui looks
- Show search conditional from args
- Add parametter to use custom font
- Some ui improvements
- Search input
- Combobox mask
- Set custom colors to globals
- Custom combobox component to select emoji tone
- Implement search and restore tab after search
- Simple search and select tone works
- Implement emoji component preview
- Handle copy emojis on click
- Handle emojis from rust
- Use skia to render beautiful emojis
- Ui to view emoji by groups

### 🐛 Bug Fixes

- Clippy suggestion
- Remove no_close request from keyboard actions
- Initial tone from args
- No-close option works fine
- Window width fit to content
- Nix build package
- Nixos modules output

### 🚜 Refactor

- Some improvements into a rust code
- Remove old iced files

### 📚 Documentation

- Update readme.md
- Update readme.md
- Update readme.md
- Update readme.md

### ⚙️ Miscellaneous Tasks

- Release simplemoji version 1.0.0
- Check nix build on pr
- Update to custom build
- Prepare environment to slint
- Use direnv
- Bump device_query from 2.1.0 to 3.0.1
- Bump clap from 4.5.35 to 4.5.37
- Update deps
- Remove double v on tag

### Build

- Nix build
- Force to link some deps to binary to run in devshell

## New Version [0.2.1]

### 🚀 Features

- Focus search on focus windows again

### 🐛 Bug Fixes

- Solve aur deploy issues
- Solve clippy issues

### 🚜 Refactor

- Remove lazy_static dep
- Remove cranix from nix flake

### 📚 Documentation

- Update readme with images
- Update readme

### ⚙️ Miscellaneous Tasks

- Release simplemoji version 0.2.1
- Remove arm arch
- Install deps to clippy check
- Update some deps versions
- Update flake
- Remove default nixpkgs
- Update flake
- Add app name env var
- Manually trigger
- Fix aur generation

### Build

- Standardizing rust versioning
- Update rust stable version

## New Version [0.2.0]

### 🚀 Features

- Auto close on copy
- Add options to custom theme
- Add color parser util
- Option to control close
- Move initial position to spawn window stage
- Upgrade and patch deps
- Add new nix modules
- Generated nix package
- Toolchain from toolchain file
- Add first flake
- Update deps
- Calcule starting position on screen
- Add preview component
- Add hoverable component
- Improve performance
- Add cli args

### 🐛 Bug Fixes

- Add padding
- Style to new theme system
- Tab size buttons
- Remove custom font by default in settings
- Nix build and devshell
- Add result to gitignore
- Add noto font to nix
- Remove unnecesary patch
- Clippy warnings
- Format
- Remove unnecesary mut
- Remove unnecesary import
- Add custom command to copy
- Nix develop
- Nix run
- Not ignore cargo lock
- Arboard dep
- Remove unsafe static mut
- Remove unnecesary font file
- Add arboard to fix copy issues
- Badge ci name
- Release process
- Height when search
- Format and clippy
- Fix positioning window based on mouse position

### 🚜 Refactor

- Remove custom hoverable component
- Restruct files
- Remove unnused file

### 📚 Documentation

- Add new args
- First documentation

### 🎨 Styling

- Fix format and clippy suggestion

### ⚙️ Miscellaneous Tasks

- Release simplemoji version 0.2.0
- Use external action to make deploy to aur
- Use cargo-dist to deploy releases
- Update and fix build
- Update iced to 0.12
- Bump device_query from 2.0.0 to 2.1.0
- Bump log from 0.4.20 to 0.4.21
- Bump arboard from 3.3.0 to 3.4.0
- Bump env_logger from 0.11.1 to 0.11.2
- Bump clap from 4.4.11 to 4.5.7
- Add dependabot checks
- Update rust version
- Release simplemoji version 0.1.6
- Release simplemoji version 0.1.5
- Release simplemoji version 0.1.4
- Automate aur publish
- Release simplemoji version 0.1.3
- Release simplemoji version 0.1.2
- Not run ci when publish new version and fix build
- Release simplemoji version 0.1.1
- Release simplemoji version
- Fix bad workflows path ignore
- Enable manually trigger ci
- Separe update logic to module
- Wip concept
- Init proyect

### Build

- Ad license to packages
- Fix git url
- Manually run


