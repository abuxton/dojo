from pprint import pprint

from mongodb_learning.db import learning_database


with learning_database() as database:
    books = database.lesson_08_python_books
    books.delete_many({})

    books.insert_many(
        [
            {
                "title": "MongoDB Basics",
                "category": "database",
                "price": 20,
                "inStock": True,
            },
            {
                "title": "Aggregation Guide",
                "category": "database",
                "price": 35,
                "inStock": True,
            },
            {
                "title": "Distributed Systems",
                "category": "architecture",
                "price": 40,
                "inStock": False,
            },
            {
                "title": "Data Modeling",
                "category": "database",
                "price": 28,
                "inStock": True,
            },
        ]
    )

    index_name = books.create_index([("category", 1), ("price", 1)])
    print(f"Created or confirmed index: {index_name}")

    affordable_books = list(
        books.find(
            {"category": "database", "price": {"$lte": 30}, "inStock": True},
            {"_id": 0, "title": 1, "price": 1},
        ).sort("price", 1)
    )
    print("Affordable database books:")
    pprint(affordable_books)

    restocked = books.update_many(
        {"inStock": False},
        {"$set": {"inStock": True}},
    )
    print(f"Restocked {restocked.modified_count} book.")
