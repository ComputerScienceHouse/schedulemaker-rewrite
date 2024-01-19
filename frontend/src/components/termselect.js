const TermSelect = (props) => {
  return (
    <div className="panel panel-default form-horizontal">
      <div className="panel-heading">
        <div className="row form-horizontal">
          <div class="col-sm-4 col-xs-6">
            <h2 class="panel-title control-label pull-left">{props.title}</h2>
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
      {props.children}
    </div>
  );
};

export default TermSelect;
