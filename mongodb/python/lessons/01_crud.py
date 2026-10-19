from pprint import pprint

from mongodb_learning.db import learning_database


with learning_database() as database:
    students = database.lesson_08_python_students
    students.delete_many({})

    inserted = students.insert_many(
        [
            {"name": "Ada", "track": "database-basics", "completedLessons": 1},
            {"name": "Grace", "track": "database-basics", "completedLessons": 2},
            {"name": "Linus", "track": "querying", "completedLessons": 1},
        ]
    )
    print(f"Inserted {len(inserted.inserted_ids)} documents.")

    found = students.find_one({"name": "Ada"})
    print("find_one({'name': 'Ada'}):")
    pprint(found)

    updated = students.update_one(
        {"name": "Ada"},
        {"$inc": {"completedLessons": 1}, "$set": {"track": "querying"}},
    )
    print(f"Updated {updated.modified_count} document.")

    removed = students.delete_one({"name": "Linus"})
    print(f"Deleted {removed.deleted_count} document.")

    print("Remaining students:")
    pprint(
        list(
            students.find(
                {}, {"_id": 0, "name": 1, "track": 1, "completedLessons": 1}
            ).sort("name", 1)
        )
    )
