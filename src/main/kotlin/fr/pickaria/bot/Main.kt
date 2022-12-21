package fr.pickaria.bot

import com.kotlindiscord.kord.extensions.ExtensibleBot
import com.kotlindiscord.kord.extensions.utils.env
import dev.kord.common.entity.Snowflake
import fr.pickaria.bot.extensions.PingExtension

private val TOKEN = env("TOKEN")

suspend fun main() {
    val bot = ExtensibleBot(TOKEN) {
        extensions {
            add(::PingExtension)
        }
    }

    bot.start()
}