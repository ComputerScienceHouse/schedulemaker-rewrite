import React from "react";

const Alert = ({ isDefaultShown = false, timeout = 250, type, message }) => {
  return (
    <div class="alert alert-info">
      <button type="button" class="close" data-dismiss="alert">
        <i class="fa fa-times"></i>
      </button>
      Use a comma to separate courses to see which course fits your schedule
      better. Add courses from the Browse or Search page to your schedule as
      well so you can easily create schedule combinations from anywhere! Also,
      check out the{" "}
      <a ui-sref="help" href="/help">
        help
      </a>{" "}
      page for new keyboard shortcuts.
    </div>
  );
};

export default Alert;
