\c lt

CREATE TABLE main.records (
  user_name VARCHAR(255),
  posted_at TIMESTAMP WITH TIME ZONE,
  token bytea,
  message TEXT,
  PRIMARY KEY (user_name, posted_at)
);

CREATE TABLE main.transactions(
  id SERIAL NOT NULL,
  action_summary TEXT, -- Expected JSON
  signiture bytea, -- for action_summary
  public_key bytea,
  PRIMARY KEY (id)
);
