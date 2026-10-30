from pprint import pprint

from mongodb_learning.db import learning_database


with learning_database() as database:
    orders = database.lesson_08_python_orders
    orders.delete_many({})

    orders.insert_many(
        [
            {
                "customer": "Ada",
                "items": [
                    {"product": "Notebook", "quantity": 2, "unitPrice": 5},
                    {"product": "Pen", "quantity": 3, "unitPrice": 2},
                ],
            },
            {
                "customer": "Grace",
                "items": [
                    {"product": "Notebook", "quantity": 1, "unitPrice": 5},
                    {"product": "Marker", "quantity": 2, "unitPrice": 4},
                ],
            },
        ]
    )

    sales_by_product = list(
        orders.aggregate(
            [
                {"$unwind": "$items"},
                {
                    "$group": {
                        "_id": "$items.product",
                        "unitsSold": {"$sum": "$items.quantity"},
                        "revenue": {
                            "$sum": {
                                "$multiply": ["$items.quantity", "$items.unitPrice"]
                            }
                        },
                    }
                },
                {
                    "$project": {
                        "_id": 0,
                        "product": "$_id",
                        "unitsSold": 1,
                        "revenue": 1,
                    }
                },
                {"$sort": {"revenue": -1}},
            ]
        )
    )

    print("Sales by product:")
    pprint(sales_by_product)
