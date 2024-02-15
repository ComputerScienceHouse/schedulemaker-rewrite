-- -------------------------------------------------------------------------
-- Saved Non-Course Schedule Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing non-course items for saved scheduled and linking
--          them to saved schedules.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schedulenoncourses (
    id          SERIAL PRIMARY KEY,
    schedule    INT NOT NULL,
    title       VARCHAR(30) NOT NULL,
    day         SMALLINT NOT NULL,
    start       SMALLINT NOT NULL,
    end         SMALLINT NOT NULL
);

-- UNSIGNED CONSTRAINT -----------------------------------------------------
ALTER TABLE schedulenoncourses
    ADD CONSTRAINT CH_snc_id_pos
    CHECK (id >= 0);

ALTER TABLE schedulenoncourses
    ADD CONSTRAINT CH_snc_schedule_pos
    CHECK (schedule >= 0);

ALTER TABLE schedulenoncourses
    ADD CONSTRAINT CH_snc_day_pos
    CHECK (day >= 0);

ALTER TABLE schedulenoncourses
    ADD CONSTRAINT CH_snc_start_pos
    CHECK (start_time >= 0);

ALTER TABLE schedulenoncourses
    ADD CONSTRAINT CH_snc_end_pos
    CHECK (end_time >= 0);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE schedulenoncourses
    ADD CONSTRAINT FK_snc_schedule
    FOREIGN KEY (schedule)
    REFERENCES schedules(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

