# Simplemoji 😁
An application where you can have all the emojis with easy and quick access

<p align="center">
  <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/simplemoji/ci.yml?label=ci">
  <img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/SergioRibera/simplemoji/release.yml">
  <img alt="AUR version" src="https://img.shields.io/aur/version/simplemoji?link=https%3A%2F%2Faur.archlinux.org%2Fpackages%2Fsimplemoji">
  <img alt="GitHub release (with filter)" src="https://img.shields.io/github/v/release/SergioRibera/simplemoji?link=https%3A%2F%2Fgithub.com%2FSergioRibera%2FSimplemoji%2Freleases">
</p>

<p align="center">
  <img src="https://github.com/user-attachments/assets/5aed54e0-e71d-4b3b-ad20-a544dedd59f1" />
</p>

> [!NOTE]
> The shortcut with which you launch the application must be configured on your own according to the WindowManager you have

# Features
- 🔎 Searchbar
- 👋 Tone emoji selector
- 👁️ Preview block
- 🗺️ Keyboard Navigation (Tab/Shift Tab/Arrows/Space/Enter)
- 📊 Fuzzy Search Support
- 🔣 Customizable Emoji Font
- ©️ Copy to clipboard
- 🎨 Custom UI Colors
- 🔘 Customize corner radius
- ⚡ Blazing Fast
- 🕟 Recents
- 🐧 Linux (X11/Wayland)

# 🎨 Custom UI Colors
<p align="center">
  <img alt="Simplemoji Theme Preview" src="https://github.com/user-attachments/assets/6217bc36-3b5a-4b42-96c1-c225d49498fd"/>
  <img alt="Simplemoji Debug Keys Preview" src="https://github.com/user-attachments/assets/aec32063-5150-4dea-acc0-5083ad40f788" />
</p>

For this we use the arguments of the application, for example
```sh
simplemoji -m '#000' -b '#DEA584'
```

```sh
simplemoji -m '#d485ad' --background-color '#262626'
```

```sh
simplemoji --primary-color '#c9cbd1' --background-color '#f2ecbc'
```


# 💽 Installation
Requirements:
 - Install [Noto Color Emoji](https://fonts.google.com/noto/specimen/Noto+Color+Emoji) font on your system

Options:
- Download from [releases](https://github.com/SergioRibera/Simplemoji/releases)
- If you use ArchLinux, just install from [Aur](https://aur.archlinux.org/packages/simplemoji)

# 🙇 Usage
```
Fast Application for look your amazing emojis write in Rust

Usage: simplemoji [OPTIONS]

Options:
  -t, --tone <TONE>
          The skin tone to apply to emojis [possible values: default, light, medium-light, medium, medium-dark, dark]
  -d, --debug
          Show debug keys and additional internal information
  -f, --font <FONT>
          The font used to render emojis
  -r, --corner-radius <CORNER_RADIUS>
          The corner radius (in pixels) for emojis when they are in focus
  -s, --show-search
          Display the search bar in the UI
      --show-recent
          Show the "recent emojis" section
      --recent-rows <RECENT_ROWS>
          The number of rows dedicated to recent emojis [default: 1]
      --recent-type <RECENT_TYPE>
          The strategy used to manage the recent emojis list [possible values: most-used, pop-push, mixed]
      --static-recents <STATIC_RECENTS>
          The number of static recents that always appear in the list [default: 4]
  -z, --fuzzing-search
          Enable fuzzy search algorithms
  -p, --show-preview
          Show an emoji preview when selecting
  -o, --close-on-copy
          Automatically close the picker after copying an emoji
  -x, --no-close
          Prevent the application from closing when it loses focus
  -b, --background-color <BACKGROUND_COLOR>
          The background color of the UI, in hexadecimal format
  -m, --primary-color <PRIMARY_COLOR>
          The primary accent color of the UI, in hexadecimal format
  -c, --copy-command <COPY_COMMAND>
          The command that will be executed to copy an emoji
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```
