import React, {useState} from "react";

const Alert = (props) => {
  const [hidden, setHidden] = useState(false);
  return (
    <div class="alert alert-info" hidden={hidden}>
      <button type="button" class="close" data-dismiss="alert" onClick={(e) => {setHidden(true)}}>
        <i class="fa fa-times"></i>
      </button>
      {props.children}
    </div>
  );
};

export default Alert;
