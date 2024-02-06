-- -------------------------------------------------------------------------
-- School Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip This table holds school codes and links them to numbers and to
--          the name of the school.
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS schools (
    `id`        INT UNSIGNED PRIMARY KEY,
    `code`      VARCHAR(5),
    `title`     VARCHAR(30)
);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE `schools`
    ADD CONSTRAINT UQ_schools_code
    UNIQUE (`code`);

-- NOTNULL CONSTRAINT ------------------------------------------------------
ALTER TABLE `schools`
    ADD CONSTRAINT NN_schools_all
    NOT NULL (`id`, `code`, `title`);

