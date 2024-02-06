-- -------------------------------------------------------------------------
-- Buildings Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip This table holds building codes and links them to numbers and
--          to the name of the of the building
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS buildings (
    `number`        VARCHAR(5) PRIMARY KEY,
    `code`          VARCHAR(5),
    `name`          VARCHAR(100),
    `off_campus`    BOOLEAN DEFAULT TRUE
);

-- UNIQUE CONSTRAINT -------------------------------------------------------
ALTER TABLE `buildings`
    ADD CONSTRAINT UQ_buildings_code
    UNIQUE (`code`);

-- NOTNULL CONSTRAINT ------------------------------------------------------
ALTER TABLE buildings
    ADD CONSTRAINT NN_buildings_all
    UNIQUE (`number`, `code`, `name`, `off_campus`);

