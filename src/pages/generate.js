import Alert from "../components/alert";
import Reset from "../components/reset";
import TermSelect from "../components/termselect";
import SubPage from "./subpage";

function Generate() {
  return (
    <SubPage>
      <Alert>
        Use a comma to separate courses to see which course fits your schedule
        better. Add courses from the Browse or Search page to your schedule as
        well so you can easily create schedule combinations from anywhere! Also,
        check out the{" "}
        <a ui-sref="help" href="/help">
          help
        </a>{" "}
        page for new keyboard shortcuts.
      </Alert>
      <TermSelect title="Generate Schedules">
        <div className="panel-body">
          <div id="scheduleCourses">
            <div
              dynamic-items="state.courses"
              colors="ui.colors"
              use-class="scheduleCourse"
              helpers="courses_helpers"
            >
              <div class="scheduleCourse repeat-item no-repeat-item-animation">
                <div class="row margin-bottom-sm">
                  <div class="col-md-8">
                    <div
                      class="form-group"
                      ng-class="{'has-error':item.sections[0].isError == true}"
                    >
                      <div class="col-sm-12 col-xs-12">
                        <div class="input-group">
                          <input
                            autocapitalize="off"
                            autocorrect="off"
                            spellcheck="off"
                            autocomplete="off"
                            id="courses1"
                            class="form-control searchField mousetrap"
                            type="text"
                            name="courses1"
                            placeholder="DEPT-CRS-SECT, DEPT-CRS-SECT..."
                          />{" "}
                          <span class="input-group-btn">
                            <button
                              title="Shortcut: Esc"
                              type="button"
                              class="btn btn-default"
                            >
                              <i class="fa fa-spin fa-refresh"></i>{" "}
                              <i class="fa fa-times"></i>
                            </button>
                          </span>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="col-md-4">
                    <div class="form-group course-result hidden-xs hidden-sm">
                      <div class="col-xs-12">
                        <button
                          title="Shortcut: Ctrl + Alt + Down"
                          type="button"
                          class="btn btn-primary btn-block"
                          disabled="disabled"
                        >
                          <i class="fa"></i> Please enter a course
                        </button>
                        <button
                          title="Shortcut: Ctrl + Alt + Down"
                          type="button"
                          class="btn btn-primary btn-block"
                        >
                          <i
                            class="fa fa-angle-down"
                          ></i>{" "}
                          Show 37 Results
                        </button>
                      </div>
                    </div>
                    <div class="course-error alert alert-danger alert-sm">
                      Chom
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="panel-footer">
          <input
            type="hidden"
            value="1"
            name="courseCount"
            id="courseCount"
            autocomplete="off"
          />
          <div class="row">
            <div class="col-md-4 col-xs-6">
              <button type="button" class="btn-default btn btn-block">
                <i class="fa fa-square-o"></i> Ignore full
              </button>
            </div>
            <div class="col-md-4 col-md-offset-4 col-xs-6">
              <button
                class="btn btn-primary btn-block"
                type="button"
                title="Shortcut: Enter"
              >
                <i class="fa fa-plus"></i> Add Course
              </button>
            </div>
          </div>
        </div>
      </TermSelect>
      <div>&nbsp;</div>
      <div>
        <div class="panel panel-default panel-control-overlap">
          <div class="panel-heading form-horizontal">
            <div class="form-horizontal row">
              <div class="col-xs-12">
                <h2 class="panel-title">Non-Course Schedule Items</h2>
              </div>
            </div>
          </div>
          <div class="panel-body"></div>
          <div class="panel-footer">
            <div class="row">
              <div class="col-md-4 col-md-offset-8">
                <button type="button" class="btn btn-block btn-primary">
                  <i class="fa fa-plus"></i> Add Item
                </button>
              </div>
            </div>
          </div>
        </div>
        <div class="panel panel-default panel-control-overlap">
          <div class="panel-heading form-horizontal">
            <div class="form-horizontal row">
              <div class="col-xs-12">
                <h2 class="panel-title">Times You Don't Want Classes</h2>
              </div>
            </div>
          </div>
          <div class="panel-body" style={null}></div>
          <div class="panel-footer">
            <div class="row">
              <div class="col-md-4 col-md-offset-8">
                <button type="button" class="btn btn-block btn-primary">
                  <i class="fa fa-plus"></i> Add Item
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="btn-group">
        <Reset />
      </div>
    </SubPage>
  );
}

export default Generate;
