CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  user_id TEXT NOT NULL,
  name TEXT NOT NULL,
  date Text NOT NULL,
  location TEXT NOT NULL
)
