import { FontAwesomeIcon } from "@fortawesome/react-fontawesome";
import { icon } from '@fortawesome/fontawesome-svg-core/import.macro'

const Reset = () => {
    return (
        <button type="button" class="btn btn-lg btn-danger pull-left btn-xs-block dropdown-toggle" data-toggle="dropdown" aria-expanded="false">
            <FontAwesomeIcon icon={icon({name: "times"})}/> Reset... <span class="caret"></span>
        </button>
    );
}

export default Reset;