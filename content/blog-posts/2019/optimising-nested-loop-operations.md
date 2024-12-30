---
slug: optimising-nested-database-operations
title: Optimising Nested Database Operations
date: 2019-02-23
description: A discussion in reducing nested database calls
tags: ["Programming"]
---

Nested loops - love them or hate them - are often a necessary evil within many
programming problems; whether it’s to traverse n-dimensional matrices or
iterating over the children of objects, they provide the cleanest solutions.
We’ll discuss the latter example and how it is sometimes possible to unravel
some of the loops to reduce the complexity of the problem many times over.

Imagine you've been given a task; given an array of many objects of class Alpha,
list all children Delta, along with related data from Bravo and Charlie which
also are children of Alpha. While this sounds like a relatively simple task at
first glance, you then remember that Delta does not directly belong to Alpha,
but instead to Charlie, which in turn belong to Bravo, and Bravo’s belong to
Alpha. The problem has now become much harder requiring traversal through three
layers of children to access Delta.

An important thing to consider is whether or not the Alpha objects already have
these children loaded into memory, or they need to retrieve from the database,
this will greatly affect the required approach. Both the [database](#using-databases)
and [preloaded](#using-preloaded-data) approaches can be found below.

We’ll use javascript in these examples but the techniques discussed will be
applicable to any language - I first implemented this solution in PHP.
Additionally, we’ll assume that `alphas` is an array of Alpha that has been
given as an argument to this function, and we’ll assume all operations are
synchronous.

One final thing to consider before we access solutions; it may be entirely
unnecessary for you to implement any of these solutions if there is not already,
or soon going to be, a significant performance problem within your program as
you’ll likely introduce further complexities while also reducing readability
which is something we should all strive to avoid.

> Premature optimization is the root of all evil
>
> <cite>Donald Knuth</cite>

## Using databases

As we will be loading data from the database, it is worth considering the
expensive nature database queries in terms of execution time and reducing the
load on the database.

Let’s start with a simplistic solution that, while not ideal, is readable and
completes the task.

```js
const output = [];

for (const alpha in alphas) {
  const bravos = database.findAllBravosByAlphaId(alpha.id);
  for (const bravo in bravos) {
    const charlies = database.findAllCharliesByBravoId(bravo.id);
    for (const charlie in charlies) {
      const deltas = database.findAllDeltasByCharlieId(charlie.id);
      for (const delta in deltas) {
        output.push({
          w: alpha.w,
          x: bravo.x,
          y: charlie.y,
          z: delta.z,
        });
      }
    }
  }
}

return output;
```

There are many issues with this solution, not only is the quadruple nested loop
is rather unforgiving when it comes to complexity growth - `O(n^4)`, but this
also results in a significant number of database queries. For example, if we
assume that for every Alpha there are 10 Bravos, for every Bravo 20 Charlies,
and for every Charlie, there are 5 Deltas; then the number of database queries
are `1000n`, where n is the number of Alphas. This is clearly rather
problematic, thankfully though we can slowly begin to unravel the inner loops,
lets first do this by pulling out the database queries which are the most
expensive part.

```js
const output = [];

for (const alpha in alphas) {
  const bravos = database.findAllBravosByAlphaId(alpha.id);
  const bravoIds = bravos.map((b) => b.id);

  const charlies = database.findAllCharliesByBravoIds(bravoIds);
  const charlieIds = charlies.map((c) => c.id);

  const deltas = database.findAllDeltasByCharlieIds(charlieIds);

  for (const bravo in bravos) {
    for (const charlie in charlies) {
      for (const delta in deltas) {
        output.push({
          w: alpha.w,
          x: bravo.x,
          y: charlie.y,
          z: delta.z,
        });
      }
    }
  }
}

return output;
```

While this solution leaves a lot to be desired it is a vast improvement over the
previous one. The crux of the solution lies on lines 5, 7, 8 and 10. On line 5
we retrieve produce a simple array that contains all the Bravos we will need,
which we can then use on 7 to find all Charlies which belong to any of these
Bravos. A similar approach is repeated on lines 8 and 10 to retrieve all Deltas.
We’ve now reduced our required number of database queries to `3n`; this is a
significant improvement already and should improve the performance of this
program many times over, but there is still the annoying presence of the
quadruple nested loop, these too can be extracted.

```js
const output = []

for (const alpha in alphas) {
  const bravos = database.findAllBravosByAlphaId(alpha.id)
  const bravoIds = bravos.map(b => b.id)
  const bravoIdMap = bravos.reduce((acc, b) => { ...acc, [b.id]: b }, {})

  const charlies = database.findAllCharliesByBravoIds(bravoIds)
  const charlieIds = charlies.map(c => c.id)
  const charlieIdMap = charlies.reduce((acc, c) => { ...acc, [c.id]: c }, {})

  const deltas = database.findAllDeltasByCharlieIds(charlieIds)

  for (const delta in deltas) {
    const charlie = charlies.find(c => c.id === delta.charlieId)
    const bravo = bravos.find(b => b.id === charlie.bravoId)
    output.push({
      w: alpha.w,
      x: bravo.x,
      y: charlie.y,
      z: delta.z,
    })
  }
}

return output
```

This solution looks to be a large improvement as we have now removed two of the
nested loops; although upon closer inspection we can see this isn’t true as it
relies upon an array `.find` on lines 15 and 16, which sadly re-introduces a
loop. We’ve now reduced the complexity from `O(n^4)` to `O(n^3)` which is a good
start, though it can be improved yet further by doing more pre-computation.

```js
const output = []

for (const alpha in alphas) {
  const bravos = database.findAllBravosByAlphaId(alpha.id)
  const bravoIds = bravos.map(b => b.id)
  const bravoIdMap = bravos.reduce((acc, b) => { ...acc, [b.id]: b }, {})

  const charlies = database.findAllCharliesByBravoIds(bravoIds)
  const charlieIds = charlies.map(c => c.id)
  const charlieIdMap = charlies.reduce((acc, c) => { ...acc, [c.id]: c }, {})

  const deltas = database.findAllDeltasByCharlieIds(charlieIds)

  for (const delta in deltas) {
    const charlie = charlieIdMap[delta.charlieId]
    const bravo = bravoIdMap[charlie.bravoId]
    output.push({
      w: alpha.w,
      x: bravo.x,
      y: charlie.y,
      z: delta.z,
    })
  }
}

return output
```

The pre-computation here is on lines 6 and 10. By precomputing an object with
key-value pair mappings - similar to a hashmap - and using them in the inner
loop the complexity has been reduced yet again to `O(n^2)` as a object/hashmap
get operation is traditionally an `O(n)` operation. There is one final step we
could take to improve performance, though it is at the cost of both memory
usage - though should still be lower than that of the [preloaded](#using-preloaded-data)
approach - and a significant reduction in the readability of the solution.

```js

const output = []

const alphaIds = alphas.map(a => a.id)
const alphaIdMap = alphas.reduce((acc, a) => { ...acc, [a.id]: a}, {})

const bravos = database.findAllBravosByAlphaIds(alphas)
const bravoIds = bravos.map(b => b.id)
const bravoIdMap = bravos.reduce((acc, b) => { ...acc, [b.id]: b }, {})

const charlies = database.findAllCharliesByBravoIds(bravoIds)
const charlieIds = charlies.map(c => c.id)
const charlieIdMap = charlies.reduce((acc, c) => { ...acc, [c.id]: c }, {})

const deltas = database.findAllDeltasByCharlieIds(charlieIds)

for (const delta in deltas) {
  const charlie = charlieIdMap[delta.charlieId]
  const bravo = bravoIdMap[charlie.bravoId]
  const alpha = alphaIdMap[bravo.alphaId]
  output.push({
    w: alpha.w,
    x: bravo.x,
    y: charlie.y,
    z: delta.z,
  })
}

return output
```

Similar to before, this solution produces another pre-computed map, though this
time of Alphas which allows for the removal of the outer loop. In doing so we
have reduced the complexity of this problem to `O(n)`. This demonstrates that
under the right situation it is entirely possible to remove many - if not all -
nested loops from a program while also reducing the number of database queries
required. It is worth noting though that when implementing these techniques,
pre-existing infrastructure may result in the inability to remove all loops,
such as in a case where it is not possible to retrieve all Bravos for all Alphas
as the current queries rely upon being given a single Alpha, and implementing
such an approach may lead to many other headaches and potential inefficiencies.

## Using preloaded data

Similarly to before, a naive approach to this problem might look something like
this:

```js
const output = [];

for (const alpha in alphas) {
  for (const bravo in alpha.bravos) {
    for (const charlie in bravo.charlies) {
      for (const delta in charlie.deltas) {
        output.push({
          w: alpha.w,
          x: bravo.x,
          y: charlie.y,
          z: delta.z,
        });
      }
    }
  }
}

return output;
```

Following a similar technique to which we used for the database approach, we can
extract the outer loops to use maps again.

```js
const output = []

const alphaIdMap = alphas.reduce((acc, a) => { ...acc, [a.id]: a}, {})

const bravos = alphas.reduce((acc, a) => acc.concat(a), [])
const bravoIdMap = bravos.reduce((acc, b) => { ...acc, [b.id]: b }, {})

const charlies = bravos.reduce((acc, b) => acc.concat(b), [])
const charlieIdMap = charlies.reduce((acc, c) => { ...acc, [c.id]: c }, {})

const deltas = charlies.reduce((acc, c) => acc.concat(c), [])

for (const delta in deltas) {
  const charlie = charlieIdMap[delta.charlieId]
  const bravo = bravoIdMap[charlie.bravoId]
  const alpha = alphaIdMap[bravo.alphaId]
  output.push({
    w: alpha.w,
    x: bravo.x,
    y: charlie.y,
    z: delta.z,
  })
}
```

While this solution again achieves `O(n)` complexity, it is very important to
note that this approach will significantly increase the memory footprint of the
program, as we are effectively doubling the memory required to store all
entities other than Deltas. Similarly to before if this memory increase turns
out to be a limiting factor it is reasonable to re-introduce loops to meet the
requirements, though if you’re doing this re-implement from the outside in as
this will have a much greater impact.
