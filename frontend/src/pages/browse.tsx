import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import Alert from "../components/alert";
import Reset from "../components/reset";
import TermSelect from "../components/termSelect";
import SubPage from "./subpage";
import { Link } from 'react-router-dom'
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'
import React from "react";

function Browse() {
    return (
        <SubPage>
            <Alert>
                Once you've found some courses you like, simply add them to your cart,
                they will be included in your possible schedules. Also, check out the{" "}
                <Link ui-sref="help" to="/help">
                    help
                </Link>{" "}
                for more info.
            </Alert>
            <TermSelect title="Browse Courses">
                <div className="panel-body">
                    <div id="browse-contents" className="list-group">
                        <div className="list-group-item">
                            <div className="browse-heading">
                                <button className="btn pull-left btn-default">
                                    <FontAwesomeIcon icon={icon({ name: "plus" })} />
                                </button>
                                <h4 className="list-group-item-heading ng-binding">CAD</h4>
                                <p className="list-group-item-text ng-binding">College of Art and Design</p>
                            </div>
                        </div>
                        <div className="list-group-item active">
                            <div className="browse-heading">
                                <button className="btn pull-left btn-default">
                                    <FontAwesomeIcon icon={icon({ name: "minus" })} />
                                </button>
                                <h4 className="list-group-item-heading">CAD</h4>
                                <p className="list-group-item-text">College of Art and Design</p>
                            </div>
                            <div className="browse-heading">
                                <div className="browse-sublist">
                                    <div className="list-group">
                                        <div className="list-group-item">
                                            <div className="browse-heading">
                                                <button className="btn pull-left btn-default">
                                                    <FontAwesomeIcon icon={icon({ name: "plus" })} />
                                                </button>
                                                <h4 className="list-group-item-heading">CAD</h4>
                                                <p className="list-group-item-text">College of Art and Design</p>
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
