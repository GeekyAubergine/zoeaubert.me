---
slug: leveraging-your-type-system
date: 2023-06-23T18:00
title: Leveraging Your Type System
description: When programming it is easy to find yourself in unexpected or invalid states. Using your compiler and type system, you can make these invalid states irrepresentable and prevent you from entering them and stopping bugs at the compiler rather than detecting them in production.
tags: ['Programming']
hero: 'https://cdn.geekyaubergine.com/2023/83bbe743c32.png'
heroAlt: 'Screen shot of 3 lines of code. The first line reads "export type Ok<T> = { ok: true; value: T }", the second line reads "export type Err<E> = { ok: false; error: E }" and the third line reads "export type Result<T, E> = Ok<T> | Err<E>"'
heroWidth: 630
heroHeight: 340
---

Early last year, I decided to give Rust a go, and as many people can attest, I won't shut up about it. Fear not; this is not a post about Rust. One of the many features Rust provides is an excellent type system which results in a high level of safety. Since then, I have tried to replicate the level of safety in other languages.

## Making Invalid States Unrepresentable

During my experiments with Rust, I came across [NoBoilerplate](https://www.youtube.com/@NoBoilerplate)'s excellent Rust videos. In many of the videos, they discuss the concept of "Making Invalid States Unrepresentable". This concept is relatively widespread, and after playing around with it, I think I finally have my own interpretation of it:

> If you represent all possible states, which includes all error states, you make it impossible to enter an unexpected [invalid] state

I've also taken "state" to mean the state of anything from your entire application to a single object/class/struct.

The power of this approach is that if you use it correctly. You can get the compiler/static analyser (compiler from here on) to do a lot of work for you and prevent you from making mistakes.

I'll be using TypeScript, but this should apply to almost any language. [View example code](https://github.com/GeekyAubergine/leveraging-your-type-system-examples).

## Invoice Example

```ts
type BadInvoice = {
    uuid: string;
    amount: number;
    state: 'pending' | 'paid' | 'cancelled'
    createdAt: string;
    paidAt: string | null;
    paymentMethod: string | null;
    cancelledAt: string | null;
    cancelledReason: string | null;
}

const validState: BadInvoice = {
    uuid: '123',
    amount: 100,
    state: 'paid',
    createdAt: '2020-01-01',
    paidAt: '2020-01-03',
    paymentMethod: null,
    cancelledAt: null,
    cancelledReason: null,
}

const invalidState: BadInvoice = {
    uuid: '123',
    amount: 100,
    state: 'pending',
    createdAt: '2020-01-01',
    paidAt: '2020-01-03',
    paymentMethod: null,
    cancelledAt: null,
    cancelledReason: null,
}
```

In this example, we're considering an Invoice to be our state. I've included both a valid and invalid state, and the difference is subtle. How would you protect against this happening? If adding a `payInvoice` function, I probably would've written something like this:

```ts
function payBadInvoice(invoice: BadInvoice, payment_method: string): BadInvoice {
    if (invoice.state !== 'pending') {
        throw new Error('Invoice is not pending');
    }

    if (invoice.cancelledAt !== null || invoice.cancelledReason !== null) {
        throw new Error('Invoice is canceled');
    }

    if (invoice.paidAt !== null || invoice.paymentMethod !== null) {
        throw new Error('Invoice is already paid');
    }

    const newInvoice: BadInvoice = {
        ...invoice,
        state: 'paid',
        paymentMethod: payment_method,
    }

    return newInvoice;
}
```

There are several things wrong with this approach:

1. We've checked that we are in a `pending` state
2. We can't trust the data actually matches the expected state for a `pending` invoice, so we have to check that it's not mistakenly a `paid` or `cancelled`
3. We return a new updated invoice with no more guarantees than the invoice that we were given

Issues 2 & 3 are particularly problematic as it is prone to errors. For example, if you check `paidAt` but forget to check `paymentMethod`, you might end up in a position where you incorrectly overwrite the `paymentMethod` when you should've defended against it. 

So how can we improve upon this? We need to represent each state an Invoice can be in and what data each state needs to be valid.

```ts
type GoodInvoicePending = {
  uuid: string;
  amount: number;
  state: "pending";
  createdAt: string;
};

type GoodInvoicePaid = {
  uuid: string;
  amount: number;
  state: "paid";
  createdAt: string;
  paidAt: string;
  paymentMethod: string;
};

type GoodInvoiceCancelled = {
  uuid: string;
  amount: number;
  state: "cancelled";
  createdAt: string;
  cancelledAt: string;
  cancelledReason: string;
};
```

We now have a representation/type for each valid state of an Invoice. We have made it impossible for a pending invoice to have `cancelledAt` set on it, or to have a paid invoice with a no `paymentMethod`. So how can we use this to our advantage?

```ts
function payGoodInvoiceGeneric(
  invoice: GoodInvoice,
  paymentMethod: string
): GoodInvoice {
  if (invoice.state !== "pending") {
    throw new Error("Invoice is not pending");
  }

  const newInvoice: GoodInvoice = {
    ...invoice,
    state: "paid",
    paymentMethod,
    paidAt: new Date().toISOString(),
  };

  return newInvoice;
}
```

Gone are all the checks for `cancelledAt` and the like, as the compiler guarantees that it _cannot_ exist on an invoice in the pending state. It is impossible to make a mistake assuming you've defined your states correctly. But we can go further.

```ts
function payGoodInvoice(
  invoice: GoodInvoicePending,
  paymentMethod: string
): GoodInvoicePaid {
  const newInvoice: GoodInvoicePaid = {
    ...invoice,
    state: "paid",
    paymentMethod,
    paidAt: new Date().toISOString(),
  };

  return newInvoice;
}
```

And just like that, we've made it impossible to pass a non-pending invoice to this function, you've transferred responsibility to ensure that an invoice is pending to the caller, and we've removed an exception/error (more on this later).

If we compare this to our original function, we've come a long way. We have added a lot of certainty to what we know about the data being given to and returned from our function and prevented mistakes. Not only does this prevent bugs, but it removes a log of cognitive load from you. You don't have to remember to check all the possible errors with the state, as it simply cannot be.

## Error handling

We can only discuss representing all states if we include discussing error states. Traditionally we handle errors/exceptions (error from here on) with a `try/catch` statement. There are several issues with this approach:

1. How do you know what functions might throw an error?
2. How do you remember to `try/catch` these error-able functions?
3. How do we recover from these errors?
4. In some languages, the error in `catch (e)` is untyped or typed as `any`

The first problem is often dealt with using some kind of comment, such as `@throws`. This is a reasonable first attempt, but it only helps if you notice or remember it.

The second problem is rarely dealt with unless it is a core part of the language, such as Java's `throws X` syntax. Several languages return tuples with `(err, value)` in an attempt to remind you to check if there's an error before accessing the value. Java's (and others) approach is the only way to get the compiler to force you to handle the error.

The third problem is more complicated to address and will depend on your situation. But if you don't want to throw the error up the stack, you must find a way to return a "good" and "bad" value.

The fourth problem is egregious. Being untyped and relying on you to dance `e instanceof Error` is beyond problematic. If this is built into your language (looking at you, JavaScript and TypeScript), there's little you can do to resolve this other than do the dance.

This is a collection of rather sorry issues. Errors occur. It's the nature of life. A file might be corrupt, an API request might fail, or any number of other things might happen. So how do we address this?

```ts
export type Ok<T> = {
  ok: true;
  value: T;
};

export type Err<E> = {
  ok: false;
  error: E;
};

export type Result<T, E> = Ok<T> | Err<E>;
```

Introducting `Result`. Don't worry if you're not familiar with [generics](https://www.typescriptlang.org/docs/handbook/2/generics.html), you don't need to understand them for this. `Result` is a type that represents the return value of a function. It either worked and is `Ok` or errored and is an `Err`. 

Why is this powerful? Let's look at an example.

```ts
enum FileReadingError {
  FILE_NOT_FOUND,
  FILE_NOT_READABLE,
  // ...
}

function readFile(path: string): Result<string, FileReadingError> {
  // ...
  
  return {
    ok: true,
    value: "file content",
  };
}
```

Looking at the `readFile` function, we immediately know 3 things, it can error, what it will return if it works, and what the error will be if it fails. This also resolves all of the issues highlighted previously. Let's say we wanted to print the contents of the file.

```ts
function printFile(path: string) {
  const result = readFile(path);

  if (!result.ok) {
    switch (result.error) {
      case FileReadingError.FILE_NOT_FOUND:
        console.log("File not found");
        return;
      case FileReadingError.FILE_NOT_READABLE:
        console.log("File not readable");
        return;
      default:
        return;
    }
  }

  console.log(result.value);
}
```

This function reads the file, and checks to see if the result is `Ok`. If it wasn't, it knows what the error type is and can respond correctly, or if it was, it can continue and print the contents.

Why is this powerful? Because the compiler, as before, can step in and ensure you handle the `Result` and let you know the error type. If you forget to check `result.ok`, you cannot access its value, and you'll get a compiler error like this:

```
Property 'value' does not exist on type 'Result<string, FileReadingError>'.
  Property 'value' does not exist on type 'Err<FileReadingError>'.
```

You'll get a similar error if you try to access the error without checking it's errored. It is now impossible to get this wrong and forget a `try/catch` somewhere, and having the error type is immensely useful. Not only does it save you from the `instanceof` dance, you know at compile time what errors you might get and can handle all of them appropriately.

### Api with Result

The only complication with this is when it comes to integrating this approach with non-result-friendly code. But we can get around that.

```ts
type ApiErrorWithStatus = {
  type: "api_error_with_status";
  status: number;
};

type UnknownApiError = {
  type: "unknown_api_error";
  message: string;
};

type ApiError = ApiErrorWithStatus | UnknownApiError;

async function api<D>(url: string): Promise<Result<D, ApiError>> {
  try {
    const response = await fetch(url);

    if (!response.ok) {
      return {
        ok: false,
        error: {
          type: "api_error_with_status",
          status: response.status,
        },
      };
    }

    const data = await response.json();

    return {
      ok: true,
      value: data,
    };
  } catch (error) {
    return {
      ok: false,
      error: {
        type: "unknown_api_error",
        message: error.message,
      },
    };
  }
}

type Trade = {
  instrument: string;
};

type Instrument = {
  name: string;
  price: number;
};

type TradeWithPrice = Trade & {
  price: number;
};

type FetchTradeInstrumentNotFoundError = {
  type: "fetch_trade_instrument_not_found_error";
  instrument: string;
};

type FetchTradeError = ApiError | FetchTradeInstrumentNotFoundError;

async function fetchTradesWithPrices(): Promise<
  Result<TradeWithPrice[], FetchTradeError>
> {
  const trades = await api<Trade[]>("/api/trades");

  if (!trades.ok) {
    return trades;
  }

  const instrumentNames = trades.value.map((trade) => trade.instrument);

  const prices = await api<Instrument[]>(
    `/api/prices?instruments=${instrumentNames.join(",")}`
  );

  if (!prices.ok) {
    return prices;
  }

  const tradesWithPrices: TradeWithPrice[] = [];

  for (const trade of trades.value) {
    const instrument = prices.value.find(
      (instrument) => instrument.name === trade.instrument
    );

    if (!instrument) {
      return Err({
        type: "fetch_trade_instrument_not_found_error",
        instrument: trade.instrument,
      });
    }

    tradesWithPrices.push({
      ...trade,
      price: instrument.price,
    });
  }

  return {
    ok: true,
    value: tradesWithPrices,
  };
}
```

This is a fairly complex example, but hopefully, it demonstrates what is possible. We have wrapped `fetch` in an `api` function that turns it into a promised `Result`. We then use it in `fetchTradesAndPrices` to call the API and either return trades prices or return errors early. Again with complete type safety and assurance that we've remembered to handle any and all errors. We've also combined the API errors with our own errors so that any call of `fetchTradesAndPrices` will have to consider missing price data and potential API errors. You could also transform all the errors into a generic `API_REQUEST_FAILED` if that's all you need to display to users.

## Null

Most languages now have the functionality to either enable strict-null-checking or have an `Option` type. I recommend using the language's built-in variants as they might have some advantages with compiler optimisation. If not, you can create an `Option` type similar to `Result` that forces you to check for null.

```ts
export type Some<T> = {
  some: true;
  value: T;
};

export type None = {
  some: false;
};

export type Option<T> = Some<T> | None;

export function Some<T>(value: T): Some<T> {
  return {
    some: true,
    value,
  };
}

export const None: None = {
  some: false,
};
```

This is similar to `Result` in the way you have to check that `x.some` is true before you can access the inner value, preventing you from accessing a null value. This eliminates the entire category of `NullPointerExcetion` errors.

## Integrating with a database

One question you might have hanging around from our Invoice example is how to make it work with a database, as you almost certainly don't want to store each state on a separate table. So how do we keep the data consistent when building the states?

```ts
type InvoicePending = {
  uuid: string;
  amount: number;
  state: "pending";
  createdAt: string;
};

type InvoicePaid = {
  uuid: string;
  amount: number;
  state: "paid";
  createdAt: string;
  paidAt: string;
  paymentMethod: string;
};

type InvoiceCancelled = {
  uuid: string;
  amount: number;
  state: "cancelled";
  createdAt: string;
  cancelledAt: string;
  cancelledReason: string;
};

type Invoice = InvoicePending | InvoicePaid | InvoiceCancelled;

type DBInvoice = {
  uuid: string;
  amount: number;
  state: "pending" | "paid" | "cancelled";
  createdAt: string;
  paidAt: string | null;
  paymentMethod: string | null;
  cancelledAt: string | null;
  cancelledReason: string | null;
};

enum DBInvoiceParsingError {
  UNKNOWN_STATE = "UNKNOWN_STATE",
  PENDING_INVOICE_HAS_PAID_AT = "PENDING_INVOICE_HAS_PAID_AT",
  PENDING_INVOICE_HAS_PAYMENT_METHOD = "PENDING_INVOICE_HAS_PAYMENT_METHOD",
  PENDING_INVOICE_HAS_CANCELLED_AT = "PENDING_INVOICE_HAS_CANCELLED_AT",
  PENDING_INVOICE_HAS_CANCELLED_REASON = "PENDING_INVOICE_HAS_CANCELLED_REASON",
  PAID_INVOICE_MISSING_PAID_AT = "PAID_INVOICE_MISSING_PAID_AT",
  PAID_INVOICE_MISSING_PAYMENT_METHOD = "PAID_INVOICE_MISSING_PAYMENT_METHOD",
  PAID_INVOICE_HAS_CANCELLED_AT = "PAID_INVOICE_HAS_CANCELLED_AT",
  PAID_INVOICE_HAS_CANCELLED_REASON = "PAID_INVOICE_HAS_CANCELLED_REASON",
  CANCELLED_INVOICE_MISSING_CANCELLED_AT = "CANCELLED_INVOICE_MISSING_CANCELLED_AT",
  CANCELLED_INVOICE_MISSING_CANCELLED_REASON = "CANCELLED_INVOICE_MISSING_CANCELLED_REASON",
  CANCELLED_INVOICE_HAS_PAID_AT = "CANCELLED_INVOICE_HAS_PAID_AT",
  CANCELLED_INVOICE_HAS_PAYMENT_METHOD = "CANCELLED_INVOICE_HAS_PAYMENT_METHOD",
}

function dbInvoiceToInvoice(
  dbInvoice: DBInvoice
): Result<Invoice, DBInvoiceParsingError> {
  switch (dbInvoice.state) {
    case "pending": {
      if (dbInvoice.paidAt !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PENDING_INVOICE_HAS_PAID_AT,
        };
      }
      if (dbInvoice.paymentMethod !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PENDING_INVOICE_HAS_PAYMENT_METHOD,
        };
      }
      if (dbInvoice.cancelledAt !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PENDING_INVOICE_HAS_CANCELLED_AT,
        };
      }
      if (dbInvoice.cancelledReason !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PENDING_INVOICE_HAS_CANCELLED_REASON,
        };
      }

      return {
        ok: true,
        value: {
          uuid: dbInvoice.uuid,
          amount: dbInvoice.amount,
          state: "pending",
          createdAt: dbInvoice.createdAt,
        },
      };
    }
    case "paid": {
      if (dbInvoice.paidAt === null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PAID_INVOICE_MISSING_PAID_AT,
        };
      }
      if (dbInvoice.paymentMethod === null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PAID_INVOICE_MISSING_PAYMENT_METHOD,
        };
      }
      if (dbInvoice.cancelledAt !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PAID_INVOICE_HAS_CANCELLED_AT,
        };
      }
      if (dbInvoice.cancelledReason !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.PAID_INVOICE_HAS_CANCELLED_REASON,
        };
      }

      return {
        ok: true,
        value: {
          uuid: dbInvoice.uuid,
          amount: dbInvoice.amount,
          state: "paid",
          createdAt: dbInvoice.createdAt,
          paidAt: dbInvoice.paidAt,
          paymentMethod: dbInvoice.paymentMethod,
        },
      };
    }
    case "cancelled": {
      if (dbInvoice.cancelledAt === null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.CANCELLED_INVOICE_MISSING_CANCELLED_AT,
        };
      }
      if (dbInvoice.cancelledReason === null) {
        return {
          ok: false,
          error:
            DBInvoiceParsingError.CANCELLED_INVOICE_MISSING_CANCELLED_REASON,
        };
      }
      if (dbInvoice.paidAt !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.CANCELLED_INVOICE_HAS_PAID_AT,
        };
      }
      if (dbInvoice.paymentMethod !== null) {
        return {
          ok: false,
          error: DBInvoiceParsingError.CANCELLED_INVOICE_HAS_PAYMENT_METHOD,
        };
      }
      return {
        ok: true,
        value: {
          uuid: dbInvoice.uuid,
          amount: dbInvoice.amount,
          state: "cancelled",
          createdAt: dbInvoice.createdAt,
          cancelledAt: dbInvoice.cancelledAt,
          cancelledReason: dbInvoice.cancelledReason,
        },
      };
    }
    default:
      return {
        ok: false,
        error: DBInvoiceParsingError.UNKNOWN_STATE,
      };
  }
}
```

Ok, this looks like a lot, and you might be thinking, "ah ha, see, you have having to check for possible invalid states!". Yes, we have to check for invalid states when importing non-validated states into our validated states code, but this is the _only_ time we'll need to do it. If we're going to do these checks, then I believe this is the best place for it, as this will likely be in or adjacent to your database/repository code, so you will be thinking about all the possible states and data you need to be aware of. There's also a less verbose way to write this. I'll leave that as an exercise to the reader.

This function provides a single point for a non-validated state to become valid with helpful error handling. You might be thinking, "but what if it's in an in-between state, it should coerce it to a valid state". I know this is tempting, but you must avoid this thinking. If the "in-between" state was valid, you would have represented it in your initial definition. It should not be possible for the data in the database to not represent one of these valid states. If it does not, you should treat the data as corrupted and go down that recovery route. 

There are many better options available to you than compromising the safety of your states. One such choice could be to present the user with a "Something went wrong" view and have the system raise an urgent ticket to manually review the account and correct the error. As cumbersome and slow as this might seem, and as tempting to say, "but it can self recover from this error", remember, your code could not have inserted this invalid state into the database. Therefore it should not have come out invalid. If it is invalid, it is either corrupted or something has tampered with the data. In either case, the computer can not be expected to make the correct rectification by itself and might, in turn, make the problem worse and harder to track and fix when you inevitably have to intervene.

> If you are unable to load a valid state from a data, you should treat it as though the integrity of the data has been lost and should not try to automatically recover from it

## Combining Valid States

While this invoice example is fine in isolation, combining it with other valid states becomes even more powerful.

```ts
type AccountSignup = {
  uuid: string;
  state: "signup";
  email: string;
};

type AccountTrial = {
  uuid: string;
  state: "trial";
  email: string;
  password: string;
  trialStartAt: string;
  trialEndsAt: string;
};

type AccountActive = {
  uuid: string;
  state: "active";
  email: string;
  password: string;
  paymentMethod: string;
  currentInvoice: InvoicePending | InvoicePaid;
};

type AccountCancelled = {
  uuid: string;
  state: "cancelled";
  email: string;
  password: string;
  currentInvoice: InvoiceCancelled;
  cancelledAt: string;
  cancelledReason: string;
};

type Account = AccountSignup | AccountTrial | AccountActive | AccountCancelled;
```

With this, you have defined some core business logic, embedded it as valid states, and the compiler will now hold you to these guarantees. 

1. To become a trial account after signup, it must:
	1. Have set a password
	2. Have a trial start and end date
2. To become active, the account must:
	1. Have a payment method set
	2. A pending or paid invoice
	3. Not be on trial
3. To cancel an account, you must:
	1. Delete the payment method
	2. Have a cancellation date and reason

You no longer have to consider, "oh, did I remember to force an invoice cancelled when cancelling the account?" or "when I cancelled the invoice, did I remember to create a new pending one to replace it and save it?". These concerns are now the concern of the compiler, and the compiler won't forget.

> The compiler is smarter than you or me. The more information you give it, the fewer bugs you'll have and the less you have to think about

## Another example

Before we move on to other concepts, it might be worth doing something non-finance-related to show you how it can be useful in different contexts.

```ts
const REPORT_TYPE = {
  STRUCTUAL: "STRUCTUAL",
  GROUND: "GROUND",
  RIVER: "RIVER",
} as const;
type REPORT_TYPE = (typeof REPORT_TYPE)[keyof typeof REPORT_TYPE];

type PersonSurveyor = {
  type: "surveyor";
  uuid: string;
  name: string;
};

type PersonReviewer = {
  type: "reviewer";
  uuid: string;
  name: string;
};

type PersonManager = {
  type: "manager";
  uuid: string;
  name: string;
};

type Person = PersonSurveyor | PersonReviewer | PersonManager;

type UnassignedUnconfirmedReport = {
  type: "unassigned_unconfirmed";
  reportType: REPORT_TYPE;
  uuid: string;
};

type AssignedUnconfirmedReport = {
  type: "assigned_unconfirmed";
  reportType: REPORT_TYPE;
  uuid: string;
  assignedTo: PersonSurveyor;
  assignedAt: string;
  assignedBy: PersonManager;
};

type AssignedConfirmedReport = {
  type: "assigned_confirmed";
  reportType: REPORT_TYPE;
  uuid: string;
  assignedTo: PersonSurveyor;
  assignedAt: string;
  assignedBy: PersonManager;
  date: string;
  confirmedAt: string;
  confirmedBy: PersonManager;
};

type InProgressReport = {
  type: "in_progress";
  reportType: REPORT_TYPE;
  uuid: string;
  assignedTo: PersonSurveyor;
  assignedAt: string;
  assignedBy: PersonManager;
  date: string;
  confirmedAt: string;
  confirmedBy: PersonManager;
  startedAt: string;
};

type AwaitingReviewReport = {
  type: "awaiting_review";
  reportType: REPORT_TYPE;
  uuid: string;
  assignedTo: PersonSurveyor;
  assignedAt: string;
  assignedBy: PersonManager;
  date: string;
  confirmedAt: string;
  confirmedBy: PersonManager;
  startedAt: string;
  endedAt: string;
};

type InReviewReport = {
  type: "in_review";
  reportType: REPORT_TYPE;
  uuid: string;
  assignedTo: PersonSurveyor;
  assignedAt: string;
  assignedBy: PersonManager;
  date: string;
  confirmedAt: string;
  confirmedBy: PersonManager;
  startedAt: string;
  endedAt: string;
  reviewStartedAt: string;
  reviewer: PersonReviewer;
};

type CompletedReport = {
  type: "completed";
  reportType: REPORT_TYPE;
  uuid: string;
  assignedTo: PersonSurveyor;
  assignedAt: string;
  assignedBy: PersonManager;
  date: string;
  confirmedAt: string;
  confirmedBy: PersonManager;
  startedAt: string;
  endedAt: string;
  reviewStartedAt: string;
  reviewer: PersonReviewer;
  reviewEndedAt: string;
  completedAt: string;
};

type Report =
  | UnassignedUnconfirmedReport
  | AssignedUnconfirmedReport
  | AssignedConfirmedReport
  | InProgressReport
  | AwaitingReviewReport
  | InReviewReport
  | CompletedReport;
```

Though this example is a little more complex, again, we've encoded a significant amount of business logic into our valid states, ensuring that we follow our own rules.

1. A report starts out unassigned and unconfirmed
2. A report must next be assigned to someone
3. The report time must be confirmed before it can be started, and it must be confirmed by a manager
4. A report must be finished and awaiting review before it can be reviewed
5. A reviewer is the only person who can review a report
6. A reviewer must complete a review before a report can be marked complete

These are some core business rules that previously might've been either a single large `progressToNextState` function that did a ludicrous amount of state validation and checking before progressing or a series of smaller functions littered all over the place. This trap is just waiting for you to forget to implement some business rule either when you first implement it or much later when changing something.

If you later decide that the surveyor can confirm the date themselves, you only need to change it in the state definition, and the compiler will tell you what else you need to change to reflect this change. 

One limitation is that we cannot protect against moving between the states out of order. The easiest way around this is to have an internal rule (or better yet, a linter if possible and/or protected creators) preventing you from manually transitioning from one state to another, instead requiring you to use a series for functions that go from one state to the other.  

```ts
function assignReport(
  report: UnassignedUnconfirmedReport,
  assignedTo: PersonSurveyor,
  assignedAt: string,
  assignedBy: PersonManager
): AssignedUnconfirmedReport {
  return {
    ...report,
    type: "assigned_unconfirmed",
    assignedTo,
    assignedAt,
    assignedBy,
  };
}
```

This not only has the advantage of forcing states to progress in a specific order but also means that in this single function, we could produce any necessary events we want to dispatch and not have to remember to do it anytime we modify a report.

## Exhaust

You may have noticed that I have a `default` case that returns an error in several of my switch statements. This is fine, but we can do better.

```ts
export function exhaust(
  _value: never,
  message = "Failed exhaustive check"
): never {
  throw new Error(message);
}
```

If we use this as the default case for our switch statements, the compiler will not let us forget the variant and will force us to handle it. Yes, this function technically throws an error, and I've previously discussed why this is bad. This is an exception to that rule. While, in theory, it could throw the error, our type system prevents it, and if that code were to be reached, it would hint at a very unlikely compiler bug.

```ts
type ReportA = {
  type: "A";
  a: string;
};

type ReportB = {
  type: "B";
  b: string;
};

type ReportC = {
  type: "C";
  c: string;
};

type Report = ReportA | ReportB | ReportC;

function processReport(report: Report) {
  switch (report.type) {
    case "A":
      console.log(report.a);
      break;
    case "B":
      console.log(report.b);
      break;
    default:
      exhaust(report);
  }
}
```

This example creates a compiler error of:

```Argument of type 'ReportC' is not assignable to parameter of type 'never'```

It won't be happy until we write a case for `ReportC`. This might seem trivial in this case but consider larger applications. Take the example from before with the reports. There are likely multiple places where you'll be doing different things depending on the state/type of the report. If you add a new state/type to your report, you might forget somewhere it's used and end up with an invalid state in that code. Instead, using `exhaust` will raise an error everywhere you use the state/type and prevent you from forgetting to handle this new case. This could be anything from rendering styling to saving to the database.

## Conclusion

Hopefully, you can now see how you can use your type system to your advantage. Why tolerate bugs caused by forgetfulness when you can have the compiler prevent them from ever getting past your editor?

In some languages, you might have to use a static analyser as part of your continuous integration rather than just the compiler, but that should not be a reason to not use this technique. 

This approach is a bit verbose compared to what you might currently be writing, but I'd argue that the upfront cost is worth the lifetime of bug prevention you will gain. I recommend you play around with this technique and try converting some of your existing code. You might be surprised how many bugs it'll catch and tell you about.