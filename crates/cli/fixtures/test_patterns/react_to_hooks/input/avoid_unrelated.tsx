import { Component } from "react";
import PropTypes from "prop-types";

class Other {
  render() {
    const { href } = this.props;

    return <a href={href}>Link Text</a>;
  }
}

class Link extends Component {
  render() {
    const { href } = this.props;

    return <a href={href}>Link Text</a>;
  }
}

export default Link;
