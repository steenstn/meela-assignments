-- Add your stuff here
CREATE TABLE answers
(
    id         UUID PRIMARY KEY NOT NULL,
    person_id  UUID             NOT NULL,
    question   VARCHAR(255)     NOT NULL,
    answer     VARCHAR(255)     NOT NULL,
    created_at DATETIME         NOT NULL
);

CREATE INDEX idx_person_id ON answers (person_id);