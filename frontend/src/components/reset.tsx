import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'
import React from "react";

const Reset = () => {
    return (
        <button type="button" className="btn btn-lg btn-danger pull-left btn-xs-block dropdown-toggle" data-toggle="dropdown" aria-expanded="false">
            <FontAwesomeIcon icon={icon({ name: "times" })} /> Reset... <span className="caret"></span>
        </button>
    );
}

export default Reset;
