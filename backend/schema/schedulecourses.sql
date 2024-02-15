-- -------------------------------------------------------------------------
-- Saved Schedule Sections Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for linked sections with saved schedules.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schedulecourses (
    id          SERIAL PRIMARY KEY,
    schedule    INT NOT NULL,
    section     INT NOT NULL
);

-- UNSIGNED CONSTRAINTS ----------------------------------------------------
ALTER TABLE schedulecourses
    ADD CONSTRAINT CH_sc_id_pos
    CHECK (id >= 0);

ALTER TABLE schedulecourses
    ADD CONSTRAINT CH_sc_schedule_pos
    CHECK (schedule >= 0);

ALTER TABLE schedulecourses
    ADD CONSTRAINT CH_sc_section_pos
    CHECK (section >= 0);

-- FOREIGN KEYS ------------------------------------------------------------
ALTER TABLE schedulecourses
    ADD CONSTRAINT FK_sc_schedule
    FOREIGN KEY (schedule)
    REFERENCES schedules(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

ALTER TABLE schedulecourses
    ADD CONSTRAINT FK_sc_section
    FOREIGN KEY (section)
    REFERENCES schedules(id)
    ON UPDATE CASCADE
    ON DELETE CASCADE;

