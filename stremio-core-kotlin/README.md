# stremio-core-kotlin

[![](https://jitpack.io/v/Stremio/stremio-core-kotlin.svg)](https://jitpack.io/#Stremio/stremio-core-kotlin)

## Setup

Prerequisite:
- Android NDK `27.2.12479018`
  - The Linkers bin path needs to be added to your `$PATH` env. variable

    E.g.: `$ANDROID_HOME/ndk/$NDK_VERSION/toolchains/llvm/prebuilt/linux-x86_64/bin` (where ANDROID_HOME is installation folder for Android studio and `NDK_VERSION` is an env. variable set to `27.2.12479018`)

  - **Updating:** Needs to be updated in `.cargo/config.toml` for the 
    `stremio-core-kotlin` build, the [`.github/workflows/release.yml`](.github/workflows/release.yml) and [`build.gradle.kts`](build.gradle.kts)

## Using the kotlin library

### Gradle

#### Add the JitPack repository to your root build.gradle

```gradle
allprojects {
    repositories {
        ...
        maven { url 'https://jitpack.io' }
    }
}
```

#### Add the stremio-core-kotlin dependency

```gradle
dependencies {
    implementation 'com.github.Stremio:stremio-core-kotlin:1.11.2'
}
```
