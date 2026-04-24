-- 1. Create a table for users


create table users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);


-- 2.  a table for balances

create table balances( 
    user_id UUID NOT NULL REFERENCES users(id),
    symbol TEXT NOT NULL,
    amount NUMERIC(32,16) NOT NULL DEFAULT 0,
    free NUMERIC(32,16) NOT NULL DEFAULT 0,
    locked NUMERIC(32,16) NOT NULL DEFAULT 0,
    PRIMARY KEY(user_id, symbol) ,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now()
);
-- 3. A simple table for trades
create table trades (
    id UUID PRIMARY KEY,
    symbol TEXT NOT NULL,
    price NUMERIC(32,16) NOT NULL,
    qty NUMERIC(32,16) NOT NULL,
-- MAKER is the one who is in the orderbook it can be buyer or seller
    maker_order_id UUID NOT NULL,
    -- Take is the one who come and fill up the order form the orderbook 
    taker_order_id UUID NOT NULL,
    created_at TIMESTAMP NOT NULL
)