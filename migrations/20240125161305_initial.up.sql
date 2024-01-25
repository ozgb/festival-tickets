CREATE TABLE ticket_types (
    id varchar NOT NULL PRIMARY KEY,
    display varchar
);

CREATE TABLE tickets_purchased (
    id uuid DEFAULT gen_random_uuid(),
    ticket_type varchar NOT NULL,
    reserved_until timestamp NOT NULL,
    purchased_at timestamp,

    CONSTRAINT fk_ticket_type
        FOREIGN KEY (ticket_type)
            REFERENCES ticket_types(id)
);
