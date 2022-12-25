package fr.pickaria.bot.extensions

import com.kotlindiscord.kord.extensions.commands.Arguments
import com.kotlindiscord.kord.extensions.commands.converters.impl.defaultingString
import com.kotlindiscord.kord.extensions.extensions.Extension
import com.kotlindiscord.kord.extensions.extensions.publicSlashCommand
import com.kotlindiscord.kord.extensions.types.respond

class PingExtension: Extension() {
    override val name = "ping"

    override suspend fun setup() {
        publicSlashCommand(::PingArguments) {
            name = "hello"
            description = "A command that will say hello to you!"

            action {
                respond { content = arguments.content }
            }
        }
    }

    inner class PingArguments : Arguments() {
        val content by defaultingString {
            name = "Content"
            defaultValue = "Pong!"
            description = "Message that will be responded."
        }
    }
}