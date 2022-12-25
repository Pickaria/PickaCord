package fr.pickaria.bot.extensions

import com.kotlindiscord.kord.extensions.extensions.Extension
import com.kotlindiscord.kord.extensions.extensions.ephemeralSlashCommand
import com.kotlindiscord.kord.extensions.types.respond

class IpExtension: Extension() {
    override val name = "ip"

    override suspend fun setup() {
        ephemeralSlashCommand {
            name = "ip"
            description = "Obtenir l'adresse du serveur."

            action {
                respond {
                    content = "Le serveur est accessible à l'adresse `play.pickaria.fr`."
                }
            }
        }
    }
}