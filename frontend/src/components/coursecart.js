import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'

const CourseCart = () => {
  return (
    <>
      <div class="visible-xs visible-sm vert-spacer-static-md"></div>
      <div class="panel panel-default course-cart">
        <div class="panel-heading">
          <h2 class="panel-title clearfix">
            <div class="row form-horizontal hidden-sm hidden-xs">
              <div class="col-md-8">
                <div class="pull-left">
                  <FontAwesomeIcon icon={icon({name: "shopping-cart"})} class="course-cart-logo"/>
                </div>
                <h2 class="panel-title control-label pull-left">Course Cart</h2>
              </div>
              <div class="col-md-4">
                <button
                  type="button"
                  class="btn btn-danger pull-right"
                  disabled="disabled"
                >
                  <FontAwesomeIcon icon={icon({name: "minus"})}/>
                  <FontAwesomeIcon icon={icon({name: "shopping-cart"})}/> All
                </button>
              </div>
            </div>
          </h2>
          <h2 class="panel-title visible-sm visible-xs">
            Course Cart{" "}
            <button
              type="button"
              class="btn btn-xs btn-primary hidden-md hidden-lg pull-right"
            >
              <FontAwesomeIcon icon={icon({name: "angle-down"})} style={null}/>
            </button>
          </h2>
        </div>
        <div
          class="panel-body course-cart-window hidden-xs hidden-sm"
          style={null}
        >
          <div class="animate-show-hide">
            <div class="alert" style={null}>
              Add courses to your cart and make a schedule with them. They will
              show up here.
            </div>
          </div>
        </div>
        <div class="panel-footer">
          <button type="button" class="btn btn-primary btn-block">
            Show Matching Schedules <FontAwesomeIcon icon={icon({name: "chevron-right"})}/>
          </button>
        </div>
      </div>
      <div class="visible-xs visible-sm vert-spacer-sm"></div>
    </>
  );
};

export default CourseCart;
