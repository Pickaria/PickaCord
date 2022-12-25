package fr.pickaria.bot

import com.kotlindiscord.kord.extensions.ExtensibleBot
import com.kotlindiscord.kord.extensions.utils.env
import fr.pickaria.bot.extensions.IpExtension

private val TOKEN = env("TOKEN")

suspend fun main() {
    val bot = ExtensibleBot(TOKEN) {
        extensions {
            // add(::PingExtension)
            add(::IpExtension)
        }

        presence {
            playing("Pickaria version alpha")
        }
    }

    bot.start()
}