-- -------------------------------------------------------------------------
-- Buildings Table
--
-- @author  Sam Cordry (samc@csh.rit.edu)
-- @descrip This table holds building codes and links them to numbers and
--          to the name of the of the building
-- -------------------------------------------------------------------------

-- TABLE CREATION ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS buildings (
    number        VARCHAR(5) PRIMARY KEY,
    code          VARCHAR(5) UNIQUE NOT NULL,
    name          VARCHAR(100) NOT NULL,
    off_campus    BOOLEAN NOT NULL DEFAULT TRUE
);

