\c lt

CREATE TABLE main.records (
  user_name VARCHAR(255),
  posted_at TIMESTAMP WITH TIME ZONE,
  token bytea,
  message TEXT,
  PRIMARY KEY (user_name, posted_at)
);
