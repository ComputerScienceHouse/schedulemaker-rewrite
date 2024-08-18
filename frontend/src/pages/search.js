import Reset from "../components/reset";
import TermSelect from "../components/termSelect";
import SubPage from "./subpage";

function Search() {
  return (
    <SubPage>
      <TermSelect title="Search Courses">
        <div className="panel-body">
          <div className="row">
            <div className="col-md-6">
              <div className="form-group">
                <label
                  className="control-label col-sm-4"
                  htmlFor="search.params.college"
                >
                  College:
                </label>
                <div className="col-sm-8">
                  <select
                    id="search.params.college"
                    name="college"
                    className="mousetrap form-control"
                    defaultValue=""
                  >
                    <option
                      label="Any College"
                      value="string:any"
                    >
                      Any College
                    </option>
                    <option
                      label="CAD - College of Art and Design"
                      value="string:29"
                    >
                      CAD - College of Art and Design
                    </option>
                    <option
                      label="CAST - College of Applied Science &amp; Technology"
                      value="string:5"
                    >
                      CAST - College of Applied Science &amp; Technology
                    </option>
                    <option
                      label="CET - College of Engineering Technology"
                      value="string:30"
                    >
                      CET - College of Engineering Technology
                    </option>
                    <option
                      label="CHST - College of Health Sciences and Technology"
                      value="string:20"
                    >
                      CHST - College of Health Sciences and Technology
                    </option>
                    <option
                      label="CIAS - College of Imaging Arts and Sciences"
                      value="string:10"
                    >
                      CIAS - College of Imaging Arts and Sciences
                    </option>
                    <option
                      label="CLA - College of Liberal Arts"
                      value="string:4"
                    >
                      CLA - College of Liberal Arts
                    </option>
                    <option label="COS - College of Science" value="string:7">
                      COS - College of Science
                    </option>
                    <option label="FNADM" value="string:26">
                      FNADM
                    </option>
                    <option
                      label="GCCIS - Golisano College of Computing and Information Sciences"
                      value="string:12"
                    >
                      GCCIS - Golisano College of Computing and Information
                      Sciences
                    </option>
                    <option
                      label="GIS - Golisano Institute For Sustainability"
                      value="string:13"
                    >
                      GIS - Golisano Institute For Sustainability
                    </option>
                    <option
                      label="INTSD - Interdisciplinary Studies "
                      value="string:25"
                    >
                      INTSD - Interdisciplinary Studies{" "}
                    </option>
                    <option
                      label="KGCOE - Kate Gleason College Of Engineering"
                      value="string:3"
                    >
                      KGCOE - Kate Gleason College Of Engineering
                    </option>
                    <option
                      label="NTID - National Technical Institute for the Deaf"
                      value="string:6"
                    >
                      NTID - National Technical Institute for the Deaf
                    </option>
                    <option label="RITHC" value="string:27">
                      RITHC
                    </option>
                    <option
                      label="SCB - Saunders College of Business"
                      value="string:2"
                    >
                      SCB - Saunders College of Business
                    </option>
                    <option label="STUAF - Student Affairs" value="string:23">
                      STUAF - Student Affairs
                    </option>
                  </select>
                </div>
              </div>
            </div>
            <div className="col-md-6">
              <div className="form-group">
                <label
                  className="control-label col-sm-4"
                  htmlFor="search.params.department"
                >
                  Department:
                </label>
                <div className="col-sm-8">
                  <select
                    id="search.params.department"
                    name="department"
                    className="mousetrap form-control"
                    defaultValue=""
                  >
                    <option
                      label="Any Department"
                      value="string:any"
                    >
                      Any Department
                    </option>
                  </select>
                </div>
              </div>
            </div>
          </div>
          <div className="row">
            <div className="col-md-6">
              <div className="form-group">
                <label
                  className="control-label col-sm-4"
                  htmlFor="search.params.credits"
                >
                  Credit Hours:
                </label>
                <div className="col-sm-8">
                  <input
                    type="text"
                    maxLength="2"
                    size="3"
                    id="search.params.credits"
                    name="credits"
                    className="mousetrap form-control"
                  />
                </div>
              </div>
            </div>
            <div className="col-md-6">
              <div className="form-group">
                <label className="control-label col-sm-4" htmlFor="search.params.level">
                  Level:
                </label>
                <div className="col-sm-8">
                  <select
                    id="search.params.level"
                    name="level"
                    className="form-control"
                    defaultValue=""
                  >
                    <option value="any">Any Level</option>
                    <option value="beg">Introductory (0 - 300)</option>
                    <option value="int">Intermediate (300 - 600)</option>
                    <option value="grad">Graduate (&gt;600)</option>
                  </select>
                </div>
              </div>
            </div>
          </div>
          <div className="row">
            <div className="col-md-6">
              <div className="form-group">
                <label className="control-label col-sm-4" htmlFor="search.params.title">
                  Title:
                </label>
                <div className="col-sm-8">
                  <input
                    type="text"
                    id="search.params.title"
                    name="title"
                    className="mousetrap form-control"
                  />
                </div>
              </div>
            </div>
            <div className="col-md-6">
              <div className="form-group">
                <label
                  className="control-label col-sm-4"
                  htmlFor="search.params.professor"
                >
                  Professor:
                </label>
                <div className="col-sm-8">
                  <input
                    type="text"
                    id="search.params.professor"
                    name="professor"
                    className="mousetrap form-control"
                  />
                </div>
              </div>
            </div>
          </div>
          <div className="form-group">
            <label
              className="control-label col-sm-4"
              htmlFor="search.params.description"
            >
              Keywords:
            </label>
            <div className="col-sm-8">
              <input
                type="text"
                id="search.params.description"
                name="description"
                className="mousetrap form-control"
                placeholder="(comma delmited)"
              />
            </div>
          </div>
          <div className="form-group">
            <label className="control-label col-md-4">Days:</label>
            <div className="col-md-8">
              <div
                dow-select-fields="search.params.days"
              >
                <div className="btn-group btn-group-dow">
                  <button
                    type="button"
                    className="btn btn-default btn-dow"
                  >
                    Su
                  </button>
                  <button
                    type="button"

                    className="btn btn-default btn-dow"

                  >
                    Mo
                  </button>
                  <button
                    type="button"

                    className="btn btn-default btn-dow"

                  >
                    Tu
                  </button>
                  <button
                    type="button"

                    className="btn btn-default btn-dow"

                  >
                    We
                  </button>
                  <button
                    type="button"

                    className="btn btn-default btn-dow "

                  >
                    Th
                  </button>
                  <button
                    type="button"

                    className="btn btn-default btn-dow"

                  >
                    Fr
                  </button>
                  <button
                    type="button"

                    className="btn btn-default btn-dow "

                  >
                    Sa
                  </button>
                </div>
              </div>
            </div>
          </div>
          <div className="form-group">
            <label className="control-label col-md-4">Times:</label>
            <div className="col-md-8">
              <div className="btn-group">
                <button
                  type="button"

                  className="btn btn-default"
                >
                  Morning
                </button>
                <button
                  type="button"

                  className="btn btn-default"
                >
                  Afternoon
                </button>
                <button
                  type="button"

                  className="btn btn-default"
                >
                  Night
                </button>
              </div>
            </div>
          </div>
          <div className="form-group">
            <label className="control-label col-sm-4" htmlFor="online">
              Course Options:
            </label>
            <div className="col-sm-8">
              <div className="row">
                <div className="col-sm-4">
                  <button
                    type="button"

                    className="btn btn-default btn-block btn-success"
                  >
                    Online{" "}
                    <i
                      className="fa fa-square-o fa-check-square"
                    ></i>
                  </button>
                </div>
                <div className="vert-spacer-static-md visible-xs"></div>
                <div className="col-sm-4">
                  <button
                    type="button"

                    className="btn btn-default btn-block btn-success"
                  >
                    Honors{" "}
                    <i
                      className="fa fa-square-o fa-check-square"
                    ></i>
                  </button>
                </div>
                <div className="vert-spacer-static-md visible-xs"></div>
                <div className="col-sm-4">
                  <button
                    type="button"

                    className="btn btn-default btn-block"
                  >
                    Off Campus{" "}
                    <i
                      className="fa fa-square-o"
                    ></i>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
      </TermSelect>
      <div className="btn-group">
        <Reset />
      </div>
    </SubPage>
  );
}

export default Search;
