# Guide: Embedding Custom Icon Fonts on Android

**Problem:** Slint uses `font-family: "Segoe Fluent Icons"` for all Fluent icon
codepoints (`\u{E80F}`, `\u{E721}`, `\u{E713}`, …).
On Android this font does not exist — all icons render as □ boxes.

---

## Solution — `SLINT_DEFAULT_FONT` + `include_bytes!`

Fontique (Slint's font manager) reads the `SLINT_DEFAULT_FONT` environment
variable **once**, lazily, the first time the collection is accessed.
If you set it **before** `slint::android::init()` the font is registered under
its real TTF family name and becomes available to all `font-family:` references
in `.slint` files.

### Step 1 — Copy the font into `src/fonts/`

```
cp C:\Windows\Fonts\SegoeIcons.ttf  src/fonts/SegoeFluentIcons.ttf
```

`SegoeIcons.ttf` is the file name on disk; the TTF family name inside the file
is **"Segoe Fluent Icons"** — which is what Slint's components reference.

### Step 2 — `android_main` in `src/lib.rs`

```rust
slint::include_modules!();

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    // 1. Font bytes are compiled into the .so at build time.
    const ICON_FONT: &[u8] = include_bytes!("fonts/SegoeFluentIcons.ttf");

    // 2. Write to app-private storage (APK assets are NOT filesystem paths).
    if let Some(dir) = app.internal_data_path() {
        let path = dir.join("SegoeFluentIcons.ttf");
        if std::fs::write(&path, ICON_FONT).is_ok() {
            // 3. Set env var BEFORE init — fontique reads it once, lazily.
            std::env::set_var("SLINT_DEFAULT_FONT", &path);
        }
    }

    slint::android::init(app).unwrap();
    AppWindow::new().unwrap().run().unwrap();
}
```

### Step 3 — `build.rs` (no embed_resources needed)

```rust
slint_build::compile_with_config(
    "ui/app.slint",
    slint_build::CompilerConfiguration::new().with_library_paths(libs),
)
.unwrap();
```

No `embed_resources` override required — the font is handled at runtime.

---

## Why other approaches fail

| Approach | Why it fails |
|---|---|
| `EmbedResourcesKind::EmbedForSoftwareRenderer` | Android uses the **Skia GL renderer**, not the software renderer. Runtime error: *"The current renderer cannot load fonts built with the EmbedForSoftwareRenderer option."* |
| `EmbedResourcesKind::EmbedFiles` | Embeds the Windows path (`C:\Windows\Fonts\SegoeIcons.ttf`) as a compile-time absolute path. That path does not exist on Android at runtime. |
| `@image-url("fonts/SegoeIcons.ttf")` in a global | Embeds raw bytes as an **image resource**, not as a loadable font. Slint does not register it with fontique. |
| `SLINT_DEFAULT_FONT` to APK asset path | APK assets are **not** accessible as filesystem paths — only via `AssetManager`. Must extract to app-private storage first. |

---

## How it works internally

`i_slint_common::sharedfontique::COLLECTION` is a `LazyLock` initialised the
first time any text is rendered.  During init it reads `SLINT_DEFAULT_FONT` and
calls `fontique::Collection::register_fonts(bytes, None)` which registers the
font under its **actual TTF family name**.

This means:
- `font-family: "Segoe Fluent Icons"` resolves correctly → icons render ✓
- Normal text (no explicit font-family) uses fallback → Android system fonts ✓

---

## NDK / SDK version notes (Android emulator, API 36.1)

NDK **r27c** supports API levels 21–35.
Android Studio may install **android-36.1** which `ndk-build` cannot parse
(decimal, not an integer).

**Fix:** copy the platform directory so both `android-35` and `android-30`
exist:

```
cd %ANDROID_HOME%\platforms
xcopy android-36.1 android-35 /E /I
xcopy android-36.1 android-30 /E /I
```

`ndk-build`'s `default_target_platform()` always requests `min(highest, 30)`,
so `android-30` must exist.

---

## Build command

```bash
export ANDROID_HOME="C:\Users\<user>\AppData\Local\Android\Sdk"
export ANDROID_NDK_ROOT="$ANDROID_HOME\ndk\android-ndk-r27c"
cargo apk build --lib
adb install -r target/debug/apk/android-demo.apk
adb shell am start -n com.example.androiddemo/android.app.NativeActivity
```
