const CourseCart = ({}) => {
  return (
    <>
      <div class="visible-xs visible-sm vert-spacer-static-md"></div>
      <div class="panel panel-default course-cart">
        <div class="panel-heading">
          <h2 class="panel-title clearfix">
            <div class="row form-horizontal hidden-sm hidden-xs">
              <div class="col-md-8">
                <div class="pull-left">
                  <i class="fa fa-shopping-cart course-cart-logo"></i>
                </div>
                <h2 class="panel-title control-label pull-left">Course Cart</h2>
              </div>
              <div class="col-md-4">
                <button
                  type="button"
                  class="btn btn-danger pull-right"
                  disabled="disabled"
                >
                  <i class="fa fa-minus"></i>
                  <i class="fa fa-shopping-cart"></i> All
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
              <i class="fa fa-angle-down" style={null}></i>
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
            Show Matching Schedules <i class="fa fa-chevron-right"></i>
          </button>
        </div>
      </div>
      <div class="visible-xs visible-sm vert-spacer-sm"></div>
    </>
  );
};

export default CourseCart;
