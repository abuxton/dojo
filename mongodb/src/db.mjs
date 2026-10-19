import { MongoClient } from "mongodb";

const defaultUri = "mongodb://127.0.0.1:27017";
const defaultDatabase = "mongodb_learning";
const defaultAppName = "mongodb-learning-lab";
const defaultServerSelectionTimeoutMs = 5_000;

export function getLearningDatabaseConfig() {
  const uri = process.env.MONGODB_URI ?? defaultUri;
  const databaseName = process.env.MONGODB_DB ?? defaultDatabase;
  const appName = process.env.MONGODB_APP_NAME ?? defaultAppName;
  const serverSelectionTimeoutMS = Number(
    process.env.MONGODB_SERVER_SELECTION_TIMEOUT_MS ??
      defaultServerSelectionTimeoutMs,
  );

  if (!databaseName.startsWith("mongodb_learning")) {
    throw new Error(
      'For safety, lessons may only use a database whose name starts with "mongodb_learning".',
    );
  }

  if (!Number.isSafeInteger(serverSelectionTimeoutMS) || serverSelectionTimeoutMS <= 0) {
    throw new Error("MONGODB_SERVER_SELECTION_TIMEOUT_MS must be a positive integer.");
  }

  return {
    uri,
    databaseName,
    clientOptions: { appName, serverSelectionTimeoutMS },
  };
}

export async function withLearningDatabase(work) {
  const { uri, databaseName, clientOptions } = getLearningDatabaseConfig();
  const client = new MongoClient(uri, clientOptions);

  try {
    await client.connect();
    return await work(client.db(databaseName));
  } finally {
    await client.close();
  }
}
