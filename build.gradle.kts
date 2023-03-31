import com.google.protobuf.gradle.*
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

allprojects {
    repositories {
        google()
        mavenCentral()
    }
}

plugins {
    kotlin("multiplatform") version "1.8.0"
    id("com.android.library") version "7.2.2"
    id("org.mozilla.rust-android-gradle.rust-android") version "0.9.0"
    id("com.google.protobuf") version "0.8.18"
}

val kotlinVersion: String by extra
val pbandkVersion: String by extra
val protobufVersion: String by extra
val stremioCoreAndroidProfile: String by extra
val protosProject = project(":protobuf-codegen")
val protosPath = file("generated")

protosPath.mkdirs()

buildscript {
    extra["kotlinVersion"] = "1.7.20"
    extra["pbandkVersion"] = "0.14.2"
    extra["protobufVersion"] = "3.21.0"
    extra["stremioCoreAndroidProfile"] = "release"

    val kotlinVersion: String by extra

    repositories {
        google()
        mavenCentral()
        maven("https://plugins.gradle.org/m2/")
    }

    dependencies {
        classpath("com.android.tools.build:gradle:7.2.2")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:${kotlinVersion}")
        classpath("org.mozilla.rust-android-gradle:plugin:0.9.0")
        classpath("com.google.protobuf:protobuf-gradle-plugin:0.8.18")
    }
}

kotlin {
    ios()
    android()

    sourceSets {
        val commonMain by getting {
            kotlin.srcDir(protosPath)
            dependencies {
                implementation("pro.streem.pbandk:pbandk-runtime:${pbandkVersion}")
            }
        }
        val commonTest by getting {
            dependencies {
                implementation(kotlin("test"))
            }
        }
        val androidMain by getting {
            dependencies {
                implementation("org.jetbrains.kotlin:kotlin-reflect:${kotlinVersion}")
            }
        }
        val androidTest by getting
    }
}

android {
    compileSdk = 33
    ndkVersion = "21.0.6113669"

    defaultConfig {
        minSdk = 21
        targetSdk = 33
    }

    sourceSets {
        getByName("main") {
            manifest.srcFile("src/androidMain/AndroidManifest.xml")
        }
    }
}

protosProject.tasks
    .matching { it.name == "generateProto" }
    .all {
        this as GenerateProtoTask

        val compileTasks = tasks.matching { it is KotlinCompile }
        compileTasks.forEach { compileTask ->
            compileTask.dependsOn(this)
        }

        outputs.upToDateWhen {
            false
        }

        doLast {
            outputSourceDirectorySet.srcDirs.forEach { generatedDirectory ->
                protosPath.mkdirs()

                val targetDirectory = File(protosPath, generatedDirectory.name)
                targetDirectory.deleteRecursively()

                require(generatedDirectory.renameTo(targetDirectory)) {
                    "Failed to move Generated protobuf files from '${generatedDirectory.absolutePath}' " +
                            "to destination directory '${targetDirectory.absolutePath}'"
                }
            }
        }
    }

cargo {
    module = "./"
    libname = "stremio_core_android"
    targets = listOf("arm", "arm64", "x86", "x86_64")
    verbose = true
    profile = if (rootProject.extra.has("stremioCoreAndroidProfile")) {
        rootProject.extra.get("stremioCoreAndroidProfile") as String
    } else {
        "debug"
    }
}

tasks.whenTaskAdded {
    if (name == "javaPreCompileDebug" || name == "javaPreCompileRelease") {
        dependsOn("cargoBuild")
    }
}
