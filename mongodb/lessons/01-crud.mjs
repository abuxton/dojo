import { withLearningDatabase } from "../src/db.mjs";

await withLearningDatabase(async (db) => {
  const students = db.collection("lesson_01_students");

  // This collection belongs only to this repeatable lesson.
  await students.deleteMany({});

  const inserted = await students.insertMany([
    { name: "Ada", track: "database-basics", completedLessons: 1 },
    { name: "Grace", track: "database-basics", completedLessons: 2 },
    { name: "Linus", track: "querying", completedLessons: 1 },
  ]);
  console.log(`Inserted ${inserted.insertedCount} documents.`);

  const found = await students.findOne({ name: "Ada" });
  console.log("findOne({ name: 'Ada' }):", found);

  const updated = await students.updateOne(
    { name: "Ada" },
    { $inc: { completedLessons: 1 }, $set: { track: "querying" } },
  );
  console.log(`Updated ${updated.modifiedCount} document.`);

  const removed = await students.deleteOne({ name: "Linus" });
  console.log(`Deleted ${removed.deletedCount} document.`);

  console.table(
    await students.find({}, { projection: { _id: 0 } }).sort({ name: 1 }).toArray(),
  );
});
