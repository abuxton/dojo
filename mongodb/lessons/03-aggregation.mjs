import { withLearningDatabase } from "../src/db.mjs";

await withLearningDatabase(async (db) => {
  const orders = db.collection("lesson_03_orders");
  await orders.deleteMany({});

  await orders.insertMany([
    {
      customer: "Ada",
      items: [
        { product: "Notebook", quantity: 2, unitPrice: 5 },
        { product: "Pen", quantity: 3, unitPrice: 2 },
      ],
    },
    {
      customer: "Grace",
      items: [
        { product: "Notebook", quantity: 1, unitPrice: 5 },
        { product: "Marker", quantity: 2, unitPrice: 4 },
      ],
    },
  ]);

  const salesByProduct = await orders
    .aggregate([
      { $unwind: "$items" },
      {
        $group: {
          _id: "$items.product",
          unitsSold: { $sum: "$items.quantity" },
          revenue: {
            $sum: { $multiply: ["$items.quantity", "$items.unitPrice"] },
          },
        },
      },
      { $project: { _id: 0, product: "$_id", unitsSold: 1, revenue: 1 } },
      { $sort: { revenue: -1 } },
    ])
    .toArray();

  console.log("Sales by product:");
  console.table(salesByProduct);
});
