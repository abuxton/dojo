from mongodb_learning.db import get_learning_database_config, learning_database


config = get_learning_database_config()

with learning_database() as database:
    ping = database.command({"ping": 1})

    print(
        {
            "database": config.database_name,
            "appName": config.app_name,
            "serverSelectionTimeoutMS": config.server_selection_timeout_ms,
            "ping": ping["ok"],
        }
    )
