plugins {
    application

    kotlin("jvm") version "1.7.20"
}

repositories {
    mavenCentral()

    maven("https://oss.sonatype.org/content/repositories/snapshots")
    maven("https://maven.kotlindiscord.com/repository/maven-public/")
}

dependencies {
    implementation(libs.kotlin.stdlib)
    implementation(libs.kord.extensions)
    implementation(libs.logback)
    implementation(libs.logging)
}

application {
    mainClass.set("fr.pickaria.bot.MainKt")
}