import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'
import React, { useState } from "react";

const Alert = (props: any) => {
  const [hidden, setHidden] = useState(false);
  return (
    <div className="alert alert-info" hidden={hidden}>
      <button type="button" className="close" data-dismiss="alert" onClick={(e) => { setHidden(true) }}>
        <FontAwesomeIcon icon={icon({ name: "times" })} />
      </button>
      {props.children}
    </div>
  );
};

export default Alert;
