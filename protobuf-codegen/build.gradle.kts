import com.google.protobuf.gradle.*
import org.jetbrains.kotlin.gradle.tasks.KotlinCompile

plugins {
    kotlin("jvm")
    id("com.google.protobuf") version "0.8.18"
}

dependencies {
    implementation("pro.streem.pbandk:pbandk-runtime:0.14.2")
}

protobuf {
    generatedFilesBaseDir = "$projectDir/generated"

    protoc {
        artifact = "com.google.protobuf:protoc:3.21.0"
    }

    plugins {
        id("pbandk") {
            artifact = "pro.streem.pbandk:protoc-gen-pbandk-jvm:0.14.2:jvm8@jar"
        }
    }

    generateProtoTasks {
        ofSourceSet("main").forEach { task ->
            task.builtins {
                remove("java")
            }
            task.plugins {
                id("pbandk")
            }
        }
    }
}

tasks.withType<KotlinCompile>().configureEach {
    kotlinOptions {
        freeCompilerArgs += "-Xopt-in=kotlin.RequiresOptIn"
    }
}
