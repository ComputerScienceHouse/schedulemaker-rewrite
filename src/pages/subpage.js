import CourseCart from "../components/coursecart";
const SubPage = (props) => {
    return (
        <div id="container">
            <div ui-view autoscroll="false" style={null}>
                <div class="container">
                    <div class="row">
                        <div class="col-md-8 clearfix">
                            {props.children}
                        </div>
                        <div class="col-md-4 pinned-track">
                            <CourseCart />
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}

export default SubPage;