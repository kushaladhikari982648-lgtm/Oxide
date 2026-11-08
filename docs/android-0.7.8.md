# Oxide Android 0.7.8

This update fixes foliage and entity textures turning see-through at a distance,
fixes the launcher's footer controls stealing taps from page buttons, and points
the launcher's GitHub button at this repository.

Version: `versionCode 43`, `versionName 0.7.8`. Installs over 0.7.7 only with the
same signing key.

## Textures: cutout mips keep their coverage

Grass, leaves, flowers, kelp and similar cutout textures are drawn with an alpha
test: a texel is either drawn or it is not. Their small mip levels are averages
of the full-size texture, and thin detail — blades one or two texels wide —
averages to nearly the same alpha everywhere. At that point the alpha test passes
the whole quad or none of it, so distant foliage and thin edges turned into gaps
or vanished, and partly transparent texels on entity skins read as see-through.

Cutout mip levels are now dithered: each level below the base takes the base
level's mean alpha through a 4×4 ordered (Bayer) dither, so its texels are in or
out in about the same proportion the full-size texture has. Coverage stays
constant as the level shrinks instead of collapsing to a uniform average. Nothing
about the base level, colour, sampling or filtering changes.

- On by default (**Graphics → Dither Cutout Mips**), because the unfixed path is
  visibly wrong on every device.
- Only cutout textures at 64 px or wider are affected; solid textures and small
  sprites are untouched.
- Toggling it triggers a resource reload, because sprite mips are built when the
  atlas loads.

## Launcher: the footer no longer steals taps

The status strip holds the status text, the repository link and the rule above
the dock. Page content now ends above that strip instead of above the dock, so a
page's own bottom row of buttons can no longer be covered by the repository link
or lose taps to it. The link also moved down to sit level with the status text
rather than in a row of its own.

Page content loses 24 dp of height at the bottom to make room. Nothing else in
the dock, the navigation bar or the Play button moves.

## Launcher: GitHub button

The GitHub button opened the upstream fork this project descends from. It now
opens this repository, so it shows the build the player is running.

## Controls

The touch layout editor already covers what this update needed, and is reachable
in game from **Options → Controls → Customize layout...** on Android:

- Drag any control to a new place, per GUI scale; Reset returns the defaults.
- Add extra buttons from the palette and bind each to any key or action.
- Move or resize the navigation pad and the individual action buttons.
- The layout's own Settings control does what a launcher-side floating button
  would: it opens the settings screen from anywhere in game, and it can be
  dragged wherever the player wants it.

The floating in-game settings button is therefore the draggable `Settings`
control, not a separate overlay widget. No launcher-level floating button was
added, because two buttons opening settings would be worse than one that can be
placed freely.

## Notes

- Wallpapers (GIF, APNG, MP4) and the bottom-docked launcher layout are unchanged
  from 0.7.7.
- Original game resources still download from Mojang on first run.
- Signed with the same generated development key as 0.7.7 when that key is
  available to the build; otherwise reinstalling requires removing the old app
  first.
