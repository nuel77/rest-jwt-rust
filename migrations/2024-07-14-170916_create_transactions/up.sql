CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  from_user INTEGER REFERENCES users(id) NOT NULL,
  to_user INTEGER REFERENCES users(id) NOT NULL,
  amount INTEGER NOT NULL
);
