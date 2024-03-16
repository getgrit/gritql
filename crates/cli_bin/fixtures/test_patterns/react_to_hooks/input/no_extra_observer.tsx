import { Component } from "react";
import PropTypes from "prop-types";
import { observer } from "mobx-react";

@observer
class Link extends Component {
  render() {
    return <a href={href}>Link Text</a>;
  }
}

export default Link;
