# Oxide Android 0.7.9

This update gives the app its own identity, `re.al.oxidelauncher`, and fixes the
settings page glitching while it is scrolled.

Version: `versionCode 44`, `versionName 0.7.9`.

> **This installs as a new app.** Android treats a different application ID as a
> different application, so 0.7.9 cannot update 0.7.8 in place. Both can be
> installed at once, but 0.7.9 starts with default settings and an empty game
> folder: worlds, accounts and settings saved by 0.7.8 stay in 0.7.8's own
> storage. Copy anything you want to keep across before uninstalling 0.7.8.
> From 0.7.9 onward updates install over each other normally, as long as the
> signing key stays the same.

## Application ID

The ID was `dev.horizon.android`, the upstream fork's. It is now
`re.al.oxidelauncher`, which is what Android shows under the app's own details
and what an update has to match. The document provider's authority
(`re.al.oxidelauncher.documents`) is built from it at run time, so the file
picker keeps working, and the release audit now checks for the new ID.

Only the application ID changed. The internal Java package (the classes the
native library calls into, such as `dev.horizon.android.HorizonActivity`) is
unchanged, because the JNI symbols are named after it.

## Settings page while scrolling

Rows were drawn whenever any part of them was inside the list, and nothing cut
them to it, so the top row painted over the page title and the bottom row
painted over the status footer. Two rows partially outside were also still
tappable where nothing was drawn.

- Rows are now cut to the list exactly at its top and bottom edge, using the
  same clipping the game's own scrolling screens use. Nothing spills over the
  title or the footer.
- A row that only partly shows answers taps only on the part that shows.
- On narrow screens, the list's scroll range was 16 dp short of the space its
  rows really take, so the last rows could not be reached and the scrollbar's
  thumb stopped short of the end. The range now matches the rows that are drawn.
- Moving the selection with a controller or keyboard, or a page's own focus
  handling, used to stop a row short of the footer and never reach the end of
  the list. It now ends flush with the footer.

## Notes

- Wallpapers (GIF, APNG, MP4), the bottom-docked launcher, the dither fix for
  cutout textures and the footer tap fix are all as in 0.7.8.
- Original game resources still download from Mojang on first run.
- Signed with the same generated development key as 0.7.7 and 0.7.8 when that
  key is available to the build.
