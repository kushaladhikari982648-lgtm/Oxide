# Oxide Android 0.8.0

This update adds a second launcher style, switches off rendering work that the
screen cannot see, and lets the launcher's own pages be reached from either
look.

Version: `versionCode 45`, `versionName 0.8.0`. This installs over 0.7.9 in
place: same application ID, same signing key, and a higher `versionCode`.

## Two launcher styles

The launcher now has a **style** setting, and the switch is in two places so it
can be reached from either look:

- On the default look, **Settings → Launcher style**.
- On the new look, the **Default UI** button in the header.

`Default UI` returns to the stone layout this launcher has always had: the
bottom dock, Play at the bottom left, the opacity slider at the bottom right and
the page tabs along the bottom edge. Nothing about it changes, and it stays the
default for a fresh install.

The new look is the Material-style layout that players coming from Zalith
Launcher already know:

- A dark background (`#1b1b1f`) with a **left navigation rail** carrying Play,
  Controls, Packs, Shaders, Files and Settings, the current page marked with a
  sage pill.
- A **floating card** with a rounded corner for the page, and a header carrying
  the version, the account chip and the style button.
- A **filled Play button** in the card's foot, with the UI-opacity slider beside
  it, dragged the same way the default look's slider is.
- Material plates and a thin slider instead of the stone bevel.

Both styles draw the **same pages**, the same lists, the same dialogs and the
same status line. The page body is moved into the card and clipped to it, so the
setting changes the chrome and not what anything does: worlds, packs, settings
and the account row are in the same place, reachable the same way, and a player
can switch mid-task without losing their place. On a screen too small to hold a
rail and a card, the launcher keeps the default layout rather than crowding one.

The rail's labels are the pages' own names: Play, Controls, Packs, Shaders,
Files, Settings. The in-game controls are untouched by this: the draggable
settings button and the layout editor are the same widgets they were.

## Leaf and block-entity culling (off by default)

Two rendering options are added, both **off** unless a world's
`.local/horizon-client/options` turns them on:

- `leafCull` — skips the faces between two `leaves`-tagged blocks, the way
  vanilla does for solid blocks. With the `cutout_leaves` resource-pack flag the
  faces between leaves are skipped only when `leafCull` is on, so packs that rely
  on seeing individual leaf faces keep their look by default.
- `blockEntityCulling` — skips block entities that no visible face can reach,
  with a fixed budget per frame. It is enabled only when every resource pack is
  vanilla, because a pack can change a block entity's model or the block that
  hosts it.

Both are opt-in for a reason: they change what is drawn, and a world that looks
wrong with them on has to be able to go back without editing the install. A
change to either option rebuilds the affected sections.

## Not in this update

The broken, transparent entity textures on 32-bit (armeabi-v7a) devices are
**not** fixed here. The cause is not known yet: the atlas layers, the mip chain,
the GL unpack alignment and the atlas patch uploads have each been checked and
ruled out, and the remaining lead is the `lowp` float precision the GLES shader
compiler is given on that path. A fix will not be claimed until the cause is
shown, and it will ship as its own update.
