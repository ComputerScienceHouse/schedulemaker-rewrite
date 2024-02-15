-- -------------------------------------------------------------------------
-- Scrape Log Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip Table for storing the status of past dump processing cron jobs.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS scrapelog (
    id                  SERIAL PRIMARY KEY,
    time_started        INT NOT NULL,
    time_ended          INT NOT NULL,
    terms_added         SMALLINT NOT NULL,
    courses_added       INT NOT NULL,
    courses_updated     INT NOT NULL,
    sections_added      INT NOT NULL,
    failures            INT NOT NULL
);

-- UNSIGNED CONSTRAINTS ---------------------------------------------------
ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_id_pos
    CHECK (id >= 0);

ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_timestart_pos
    CHECK (time_started >= 0);

ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_timeend_pos
    CHECK (time_ended >= 0);

ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_terms_pos
    CHECK (terms_added >= 0);

ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_courseadd_pos
    CHECK (courses_added >= 0);

ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_courseupdate_pos
    CHECK (courses_updated >= 0);

ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_sectionadd_pos
    CHECK (sections_added >= 0);

ALTER TABLE scrapelog
    ADD CONSTRAINT CH_sl_falure_pos
    CHECK (failures >= 0);

