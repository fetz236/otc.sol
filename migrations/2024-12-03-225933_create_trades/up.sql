-- Your SQL goes here
CREATE TABLE trades (
    id SERIAL PRIMARY KEY,
    creator_id INT NOT NULL REFERENCES users(id),
    amount BIGINT NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    status VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
