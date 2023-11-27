create table oracle_transactions
(
    "hash"          text not null constraint oracle_transactions_pk primary key,
    "from"          text,
    "to"            text,
    "chain"         text,
    "status"        text,
    "timestamp"     timestamp
);