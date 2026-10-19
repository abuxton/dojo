"""Connection helpers shared by the synchronous PyMongo lessons."""

from contextlib import contextmanager
from dataclasses import dataclass
from os import environ
from typing import Iterator

from pymongo import MongoClient
from pymongo.database import Database

DEFAULT_URI = "mongodb://127.0.0.1:27017"
DEFAULT_DATABASE = "mongodb_learning"
DEFAULT_APP_NAME = "mongodb-learning-lab-python"
DEFAULT_SERVER_SELECTION_TIMEOUT_MS = 5_000


@dataclass(frozen=True)
class LearningDatabaseConfig:
    uri: str
    database_name: str
    app_name: str
    server_selection_timeout_ms: int


def get_learning_database_config() -> LearningDatabaseConfig:
    uri = environ.get("MONGODB_URI", DEFAULT_URI)
    database_name = environ.get("MONGODB_DB", DEFAULT_DATABASE)
    app_name = environ.get("MONGODB_APP_NAME", DEFAULT_APP_NAME)

    try:
        server_selection_timeout_ms = int(
            environ.get(
                "MONGODB_SERVER_SELECTION_TIMEOUT_MS",
                DEFAULT_SERVER_SELECTION_TIMEOUT_MS,
            )
        )
    except ValueError as error:
        raise ValueError(
            "MONGODB_SERVER_SELECTION_TIMEOUT_MS must be a positive integer."
        ) from error

    if not database_name.startswith("mongodb_learning"):
        raise ValueError(
            'For safety, lessons may only use a database whose name starts with '
            '"mongodb_learning".'
        )
    if server_selection_timeout_ms <= 0:
        raise ValueError(
            "MONGODB_SERVER_SELECTION_TIMEOUT_MS must be a positive integer."
        )

    return LearningDatabaseConfig(
        uri=uri,
        database_name=database_name,
        app_name=app_name,
        server_selection_timeout_ms=server_selection_timeout_ms,
    )


@contextmanager
def learning_database() -> Iterator[Database]:
    config = get_learning_database_config()
    client = MongoClient(
        config.uri,
        appname=config.app_name,
        serverSelectionTimeoutMS=config.server_selection_timeout_ms,
    )

    try:
        yield client[config.database_name]
    finally:
        client.close()
