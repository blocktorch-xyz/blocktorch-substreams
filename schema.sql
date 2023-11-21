create table transactions
(
    "hash"          text not null constraint transactions_pk primary key,
    "from"          text,
    "to"            text,
    "chain"         text,
    "status"        text,
    "timestamp"     timestamp
);