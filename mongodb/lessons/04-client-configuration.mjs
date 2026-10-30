import { getLearningDatabaseConfig, withLearningDatabase } from "../src/db.mjs";

const { databaseName, clientOptions } = getLearningDatabaseConfig();

await withLearningDatabase(async (db) => {
  const ping = await db.command({ ping: 1 });

  console.table([
    { setting: "database", value: databaseName },
    { setting: "appName", value: clientOptions.appName },
    {
      setting: "serverSelectionTimeoutMS",
      value: clientOptions.serverSelectionTimeoutMS,
    },
    { setting: "ping", value: ping.ok },
  ]);
});
