create table chronicle_transactions
(
    "hash"          text not null constraint chronicle_transactions_pk primary key,
    "from"          text,
    "to"            text,
    "chain"         text,
    "status"        text,
    "timestamp"     timestamp
);

create table aa_transactions
(
    "hash"          text not null constraint aa_transactions_pk primary key,
    "chain"         text,
    "aaType"        text,
    "status"        text,
    "timestamp"     timestamp
);