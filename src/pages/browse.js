import Alert from "../components/alert";
import Reset from "../components/reset";
import TermSelect from "../components/termselect";
import SubPage from "./subpage";

function Browse() {
  return (
    <SubPage>
      <Alert>
        Once you've found some courses you like, simply add them to your cart, 
        they will be included in your possible schedules. Also, check out the{" "}
        <a ui-sref="help" href="/help">
          help
        </a>{" "}
        for more info.
      </Alert>
      <TermSelect title="Browse Courses">
        <div class="panel-body">
            <div id="browse-contents" class="list-group">
                <div class="list-group-item">
                    <div class="browse-heading">
                        <button class="btn pull-left btn-default">
                            <i class="fa fa-plus" ng-class="school.ui.buttonClass"></i>
                        </button>
                        <h4 class="list-group-item-heading ng-binding">CAD</h4>
                        <p class="list-group-item-text ng-binding">College of Art and Design</p>
                    </div>
                </div>
                <div class="list-group-item active">
                    <div class="browse-heading">
                        <button class="btn pull-left btn-default">
                            <i class="fa fa-minus" ng-class="school.ui.buttonClass"></i>
                        </button>
                        <h4 class="list-group-item-heading">CAD</h4>
                        <p class="list-group-item-text">College of Art and Design</p>
                    </div>
                    <div class="browse-heading">
                        <div class="browse-sublist">
                            <div class="list-group">
                                <div class="list-group-item">
                                    <div class="browse-heading">
                                        <button class="btn pull-left btn-default">
                                            <i class="fa fa-plus" ng-class="school.ui.buttonClass"></i>
                                        </button>
                                        <h4 class="list-group-item-heading">CAD</h4>
                                        <p class="list-group-item-text">College of Art and Design</p>
                                    </div>
                                </div>
                            </div>
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

export default Browse;