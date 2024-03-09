import { Component } from "react";
import PropTypes from "prop-types";

class Link extends Component {
  static propTypes = {
    href: PropTypes.string.isRequired,
  };

  render() {
    const { href } = this.props;

    return <a href={href}>Link Text</a>;
  }
}

export default Link;
