import Alert from "../components/alert";
import SubPage from "./subpage";

function Generate() {
  return (
    <SubPage>
      <Alert type="info" message="This is info" />
      <div className="panel panel-default form-horizontal">
        <div className="panel-heading">
          <div className="row form-horizontal">
            <div class="col-sm-4 col-xs-6">
              <h2 class="panel-title control-label pull-left">
                Select Courses
              </h2>
            </div>
            <div class="col-sm-8 col-xs-6">
              <div class="row">
                <label class="col-sm-6 control-label hidden-xs" for="term">
                  Term:
                </label>
                <div class="col-sm-6">
                  <select className="form-control"></select>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div className="panel-body"></div>
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
      </div>
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
    </SubPage>
  );
}

export default Generate;
