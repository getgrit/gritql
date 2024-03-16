import { Component } from "react";
import PropTypes from "prop-types";

class Foo extends Component {
  componentDidMount() {
    console.log("mounted");
  }

  componentWillUnmount() {
    console.log("unmounted");
  }

  render() {
    return <p>Foo</p>;
  }
}

export default Foo;
