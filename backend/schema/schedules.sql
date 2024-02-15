-- -------------------------------------------------------------------------
-- Saved Schedule Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing saved schedule records.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schedules (
    id              SERIAL PRIMARY KEY,
    last_accessed   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    start_day       SMALLINT NOT NULL DEFAULT 0,
    end_day         SMALLINT NOT NULL DEFAULT 6,
    start_time      SMALLINT NOT NULL DEFAULT 8,
    end_time        SMALLINT NOT NULL DEFAULT 22,
    term            SMALLINT NOT NULL,
    image           BOOLEAN NOT NULL DEFAULT FALSE
);

-- UNSIGNED CONSTRAINTS ---------------------------------------------------
ALTER TABLE schedules
    ADD CONSTRAINT CH_sch_id_pos
    CHECK (id >= 0);

ALTER TABLE schedules
    ADD CONSTRAINT CH_sch_sday_pos
    CHECK (start_day >= 0);

ALTER TABLE schedules
    ADD CONSTRAINT CH_sch_eday_pos
    CHECK (end_day >= 0);

ALTER TABLE schedules
    ADD CONSTRAINT CH_sch_stime_pos
    CHECK (start_time >= 0);

ALTER TABLE schedules
    ADD CONSTRAINT CH_sch_etime_pos
    CHECK (end_time >= 0);

ALTER TABLE schedules
    ADD CONSTRAINT CH_sch_term_pos
    CHECK (term >= 0);

-- FOREIGN KEY ------------------------------------------------------------
ALTER TABLE schedules
    ADD CONSTRAINT FK_schedules_term
    FOREIGN KEY (term)
    REFERENCES academicterms(term)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

