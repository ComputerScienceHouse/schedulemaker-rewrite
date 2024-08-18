import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'

const CourseCart = () => {
  return (
    <>
      <div className="visible-xs visible-sm vert-spacer-static-md"></div>
      <div className="panel panel-default course-cart">
        <div className="panel-heading">
          <h2 className="panel-title clearfix">
            <div className="row form-horizontal hidden-sm hidden-xs">
              <div className="col-md-8">
                <div className="pull-left">
                  <FontAwesomeIcon icon={icon({ name: "shopping-cart" })} className="course-cart-logo" />
                </div>
                <h2 className="panel-title control-label pull-left">Course Cart</h2>
              </div>
              <div className="col-md-4">
                <button
                  type="button"
                  className="btn btn-danger pull-right"
                  disabled="disabled"
                >
                  <FontAwesomeIcon icon={icon({ name: "minus" })} />
                  <FontAwesomeIcon icon={icon({ name: "shopping-cart" })} /> All
                </button>
              </div>
            </div>
          </h2>
          <h2 className="panel-title visible-sm visible-xs">
            Course Cart{" "}
            <button
              type="button"
              className="btn btn-xs btn-primary hidden-md hidden-lg pull-right"
            >
              <FontAwesomeIcon icon={icon({ name: "angle-down" })} style={null} />
            </button>
          </h2>
        </div>
        <div
          className="panel-body course-cart-window hidden-xs hidden-sm"
          style={null}
        >
          <div className="animate-show-hide">
            <div className="alert" style={null}>
              Add courses to your cart and make a schedule with them. They will
              show up here.
            </div>
          </div>
        </div>
        <div className="panel-footer">
          <button type="button" className="btn btn-primary btn-block">
            Show Matching Schedules <FontAwesomeIcon icon={icon({ name: "chevron-right" })} />
          </button>
        </div>
      </div>
      <div className="visible-xs visible-sm vert-spacer-sm"></div>
    </>
  );
};

export default CourseCart;
