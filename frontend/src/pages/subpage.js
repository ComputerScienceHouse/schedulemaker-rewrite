import CourseCart from "../components/coursecart";
const SubPage = (props) => {
    return (
        <div id="container">
            <div ui-view="true" autoscroll="false" style={null}>
                <div className="container">
                    <div className="row">
                        <div className="col-md-8 clearfix">
                            {props.children}
                        </div>
                        <div className="col-md-4 pinned-track">
                            <CourseCart />
                        </div>
                    </div>
                </div>
            </div>
            <div className="vert-spacer-static-md"></div>
        </div>
    );
}

export default SubPage;
