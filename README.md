# XDG Desktop File Query

Can't find that one .desktop file? Wondering why that one shortcut you deleted
is still showing up in your searches? This simple tool allows you to quickly 
search for a string in the contents of all the desktop files that can be
found through your `$XDG_DATA_DIRS`

# Usage
```
desktop-file-query [QUERY]
```

# Example
```
$ desktop-file-query "zen browser"
/run/current-system/sw/share/applications/chromium-browser.desktop
/run/current-system/sw/share/applications/gtk3-icon-browser.desktop
/run/current-system/sw/share/applications/nixos-manual.desktop
/run/current-system/sw/share/applications/org.gnome.BrowserConnector.desktop
/run/current-system/sw/share/applications/org.gnome.Epiphany.desktop
/run/current-system/sw/share/applications/zen.desktop```
```

# Running as a nix flake
If you have Nix installed and flakes enabled, this package can be
downloaded, built and run effortlessly with one command:
```
nix run github:jorikvanveen/desktop-file-query -- "browser"
```
