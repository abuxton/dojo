# Lab 07: Explore the lessons in MongoDB Compass

## Goal

Use the local Compass desktop application to browse the collections created by
the Node.js lessons without copying data outside the local container.

## Start and seed the lab

```sh
npm run db:up
npm run lesson:01
npm run lesson:02
npm run lesson:03
npm run lab:07:compass
```

The launcher opens MongoDB Compass with:

```text
mongodb://127.0.0.1:27017/mongodb_learning
```

If the app does not open, install the local tools first:

```sh
brew bundle --file Brewfile
```

## Explore

1. Save the connection as **MongoDB Learning Lab**.
2. Open `lesson_01_students` and use the Documents tab to reproduce the
   filter `{ "completedLessons": { "$gte": 2 } }`.
3. Open `lesson_02_books`, inspect the `category_1_price_1` index, and use
   Explain Plan to compare the indexed query from Lab 02.
4. Open `lesson_03_orders` and build the aggregation from Lab 03 with the
   Aggregations tab: `$unwind`, `$group`, `$project`, then `$sort`.

Compass is a local desktop client, not a Docker service. Its saved connection
is stored by Compass on the workstation; this repository only provides the
safe local connection URI and launcher.
