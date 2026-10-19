const learningDb = db.getSiblingDB("mongodb_learning");

print(`Connected to database: ${learningDb.getName()}`);
print("Collections:");
printjson(learningDb.getCollectionNames());

print("Students who completed at least two lessons:");
printjson(
  learningDb.lesson_01_students
    .find(
      { completedLessons: { $gte: 2 } },
      { _id: 0, name: 1, track: 1, completedLessons: 1 },
    )
    .sort({ name: 1 })
    .toArray(),
);
