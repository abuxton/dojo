import { withLearningDatabase } from "../src/db.mjs";

await withLearningDatabase(async (db) => {
  const books = db.collection("lesson_02_books");
  await books.deleteMany({});

  await books.insertMany([
    { title: "MongoDB Basics", category: "database", price: 20, inStock: true },
    { title: "Aggregation Guide", category: "database", price: 35, inStock: true },
    { title: "Distributed Systems", category: "architecture", price: 40, inStock: false },
    { title: "Data Modeling", category: "database", price: 28, inStock: true },
  ]);

  const indexName = await books.createIndex({ category: 1, price: 1 });
  console.log(`Created or confirmed index: ${indexName}`);

  const affordableDatabaseBooks = await books
    .find(
      { category: "database", price: { $lte: 30 }, inStock: true },
      { projection: { _id: 0, title: 1, price: 1 } },
    )
    .sort({ price: 1 })
    .toArray();
  console.log("Affordable database books:");
  console.table(affordableDatabaseBooks);

  const restocked = await books.updateMany(
    { inStock: false },
    { $set: { inStock: true, restockedAt: new Date() } },
  );
  console.log(`Restocked ${restocked.modifiedCount} book.`);
});
