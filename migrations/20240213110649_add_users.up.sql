-- Create users table
CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    address text NOT NULL,
    email text NOT NULL
);

-- Add user_id to orders table with foreign key constraint
ALTER TABLE orders
ADD user_id uuid;

ALTER TABLE orders
ADD CONSTRAINT fk_user_id
FOREIGN KEY (user_id) REFERENCES users(id);

