import React from "react";

const Alert = (props) => {
  return (
    <div class="alert alert-info">
      <button type="button" class="close" data-dismiss="alert">
        <i class="fa fa-times"></i>
      </button>
      {props.children}
    </div>
  );
};

export default Alert;
