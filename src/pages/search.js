import Reset from "../components/reset";
import TermSelect from "../components/termselect";
import SubPage from "./subpage";

function Search() {
  return (
    <SubPage>
      <TermSelect title="Search Courses">
        <div class="panel-body">
          <div class="row">
            <div class="col-md-6">
              <div class="form-group">
                <label
                  class="control-label col-sm-4"
                  for="search.params.college"
                >
                  College:
                </label>
                <div class="col-sm-8">
                  <select
                    id="search.params.college"
                    name="college"
                    class="mousetrap form-control"
                  >
                    <option
                      label="Any College"
                      value="string:any"
                      selected="selected"
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
            <div class="col-md-6">
              <div class="form-group">
                <label
                  class="control-label col-sm-4"
                  for="search.params.department"
                >
                  Department:
                </label>
                <div class="col-sm-8">
                  <select
                    id="search.params.department"
                    name="department"
                    class="mousetrap form-control"
                  >
                    <option
                      label="Any Department"
                      value="string:any"
                      selected="selected"
                    >
                      Any Department
                    </option>
                  </select>
                </div>
              </div>
            </div>
          </div>
          <div class="row">
            <div class="col-md-6">
              <div class="form-group">
                <label
                  class="control-label col-sm-4"
                  for="search.params.credits"
                >
                  Credit Hours:
                </label>
                <div class="col-sm-8">
                  <input
                    type="text"
                    maxlength="2"
                    size="3"
                    id="search.params.credits"
                    name="credits"
                    class="mousetrap form-control"
                  />
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="form-group">
                <label class="control-label col-sm-4" for="search.params.level">
                  Level:
                </label>
                <div class="col-sm-8">
                  <select
                    id="search.params.level"
                    name="level"
                    class="form-control"
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
          <div class="row">
            <div class="col-md-6">
              <div class="form-group">
                <label class="control-label col-sm-4" for="search.params.title">
                  Title:
                </label>
                <div class="col-sm-8">
                  <input
                    type="text"
                    id="search.params.title"
                    name="title"
                    class="mousetrap form-control"
                  />
                </div>
              </div>
            </div>
            <div class="col-md-6">
              <div class="form-group">
                <label
                  class="control-label col-sm-4"
                  for="search.params.professor"
                >
                  Professor:
                </label>
                <div class="col-sm-8">
                  <input
                    type="text"
                    id="search.params.professor"
                    name="professor"
                    class="mousetrap form-control"
                  />
                </div>
              </div>
            </div>
          </div>
          <div class="form-group">
            <label
              class="control-label col-sm-4"
              for="search.params.description"
            >
              Keywords:
            </label>
            <div class="col-sm-8">
              <input
                type="text"
                id="search.params.description"
                name="description"
                class="mousetrap form-control"
                placeholder="(comma delmited)"
              />
            </div>
          </div>
          <div class="form-group">
            <label class="control-label col-md-4">Days:</label>
            <div class="col-md-8">
              <div
                dow-select-fields="search.params.days"
              >
                <div class="btn-group btn-group-dow">
                  <button
                    type="button"
                    class="btn btn-default btn-dow"
                  >
                    Su
                  </button>
                  <button
                    type="button"

                    class="btn btn-default btn-dow"

                  >
                    Mo
                  </button>
                  <button
                    type="button"

                    class="btn btn-default btn-dow"

                  >
                    Tu
                  </button>
                  <button
                    type="button"

                    class="btn btn-default btn-dow"

                  >
                    We
                  </button>
                  <button
                    type="button"

                    class="btn btn-default btn-dow "

                  >
                    Th
                  </button>
                  <button
                    type="button"

                    class="btn btn-default btn-dow"

                  >
                    Fr
                  </button>
                  <button
                    type="button"

                    class="btn btn-default btn-dow "

                  >
                    Sa
                  </button>
                </div>
              </div>
            </div>
          </div>
          <div class="form-group">
            <label class="control-label col-md-4">Times:</label>
            <div class="col-md-8">
              <div class="btn-group">
                <button
                  type="button"

                  class="btn btn-default"
                >
                  Morning
                </button>
                <button
                  type="button"

                  class="btn btn-default"
                >
                  Afternoon
                </button>
                <button
                  type="button"

                  class="btn btn-default"
                >
                  Night
                </button>
              </div>
            </div>
          </div>
          <div class="form-group">
            <label class="control-label col-sm-4" for="online">
              Course Options:
            </label>
            <div class="col-sm-8">
              <div class="row">
                <div class="col-sm-4">
                  <button
                    type="button"

                    class="btn btn-default btn-block btn-success"
                  >
                    Online{" "}
                    <i
                      class="fa fa-square-o fa-check-square"
                    ></i>
                  </button>
                </div>
                <div class="vert-spacer-static-md visible-xs"></div>
                <div class="col-sm-4">
                  <button
                    type="button"

                    class="btn btn-default btn-block btn-success"
                  >
                    Honors{" "}
                    <i
                      class="fa fa-square-o fa-check-square"
                    ></i>
                  </button>
                </div>
                <div class="vert-spacer-static-md visible-xs"></div>
                <div class="col-sm-4">
                  <button
                    type="button"

                    class="btn btn-default btn-block"
                  >
                    Off Campus{" "}
                    <i
                      class="fa fa-square-o"
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
