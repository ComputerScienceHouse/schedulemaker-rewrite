-- -------------------------------------------------------------------------
-- Section Times Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @decsrip Table for storing the times and locations that a section meets.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS times (
    id          SERIAL PRIMARY KEY,
    section     INT NOT NULL,
    day         SMALLINT NOT NULL,
    start_time  SMALLINT NOT NULL,
    end_time    SMALLINT NOT NULL,
    building    VARCHAR(5) NOT NULL DEFAULT "00",
    room        VARCHAR(10) NOT NULL DEFAULT "0000"
);

-- UNSIGNED CONSTRAINTS ----------------------------------------------------
ALTER TABLE times
    ADD CONSTRAINT CH_times_id_pos
    CHECK (id >= 0);

ALTER TABLE times
    ADD CONSTRAINT CH_times_section_pos
    CHECK (section >= 0);

ALTER TABLE times
    ADD CONSTRAINT CH_times_day_pos
    CHECK (day >= 0);

ALTER TABLE times
    ADD CONSTRAINT CH_times_start_pos
    CHECK (start_time >= 0);

ALTER TABLE times
    ADD CONSTRAINT CH_times_end_pos
    CHECK (end_time >= 0);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE times
    ADD CONSTRAINT FK_times_section
    FOREIGN KEY (section)
    REFERENCES sections(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

ALTER TABLE times
    ADD CONSTRAINT FK_times_building
    FOREIGN KEY (building)
    REFERENCES buildings(number)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

